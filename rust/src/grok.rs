use std::ffi::CString;
use std::os::raw::c_char;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

static INITIALIZED: AtomicBool = AtomicBool::new(false);
static CODEC_LOCK: Mutex<()> = Mutex::new(());

/// Initialize grok library. Safe to call multiple times.
pub fn initialize(num_threads: u32) {
    if INITIALIZED.swap(true, Ordering::SeqCst) {
        return;
    }
    unsafe {
        grokj2k_sys::grk_initialize(std::ptr::null(), num_threads, std::ptr::null_mut());
    }
}

/// Deinitialize grok library.
pub fn deinitialize() {
    if !INITIALIZED.swap(false, Ordering::SeqCst) {
        return;
    }
    unsafe {
        grokj2k_sys::grk_deinitialize();
    }
}

/// Compress planar int32 image data to a JPEG 2000 file with XYZ colour transform.
///
/// Uses the core grok API (`grk_image_new` → `grk_apply_xyz_transform` →
/// `grk_compress_init` → `grk_compress`) — no global state, fully concurrent.
///
/// # Arguments
/// * `components` - 3 planar buffers of int32 pixel values, one per channel (R, G, B)
/// * `width` - image width in pixels
/// * `height` - image height in pixels
/// * `precision` - bit depth (e.g. 12)
/// * `output` - output file path (.j2c)
/// * `ratio` - compression ratio (e.g. 1.0 for lossless)
/// * `num_resolutions` - number of DWT decomposition levels + 1
/// * `codeblock_size` - codeblock dimension (e.g. 32)
/// * `progression` - progression order (e.g. "CPRL")
pub fn compress_image_xyz(
    components: [&[i32]; 3],
    width: u32,
    height: u32,
    precision: u8,
    output: &Path,
    ratio: f64,
    num_resolutions: u8,
    codeblock_size: u32,
    progression: &str,
) -> Result<u64, String> {
    let output_str = output.to_str()
        .ok_or_else(|| format!("Invalid output path: {}", output.display()))?;

    let expected_len = (width as usize) * (height as usize);
    for (i, comp) in components.iter().enumerate() {
        if comp.len() < expected_len {
            return Err(format!(
                "Component {} buffer too small: {} < {}",
                i, comp.len(), expected_len
            ));
        }
    }

    unsafe {
        // Create grk_image with 3 components
        let mut cmptparms: [grokj2k_sys::_grk_image_comp; 3] = std::mem::zeroed();
        for c in &mut cmptparms {
            c.dx = 1;
            c.dy = 1;
            c.w = width;
            c.h = height;
            c.stride = width;
            c.prec = precision;
            c.sgnd = false;
        }

        let image = grokj2k_sys::grk_image_new(
            3,
            cmptparms.as_mut_ptr(),
            grokj2k_sys::_GRK_COLOR_SPACE_GRK_CLRSPC_SRGB,
            true,
        );
        if image.is_null() {
            return Err("grk_image_new failed".to_string());
        }

        // Set image dimensions
        (*image).x0 = 0;
        (*image).y0 = 0;
        (*image).x1 = width;
        (*image).y1 = height;

        // Copy pixel data into grk_image component buffers
        for i in 0..3usize {
            let comp = &(*image).comps.add(i).read();
            let stride = comp.stride as usize;
            let dst = comp.data as *mut i32;
            if dst.is_null() {
                grokj2k_sys::grk_object_unref(&mut (*image).obj);
                return Err(format!("Component {} data is null", i));
            }
            // Copy row by row if stride != width
            if stride == width as usize {
                std::ptr::copy_nonoverlapping(
                    components[i].as_ptr(),
                    dst,
                    expected_len,
                );
            } else {
                for row in 0..height as usize {
                    std::ptr::copy_nonoverlapping(
                        components[i].as_ptr().add(row * width as usize),
                        dst.add(row * stride),
                        width as usize,
                    );
                }
            }
        }

        // Apply RGB → XYZ colour transform in-place
        if !grokj2k_sys::grk_apply_xyz_transform(image) {
            grokj2k_sys::grk_object_unref(&mut (*image).obj);
            return Err("grk_apply_xyz_transform failed".to_string());
        }

        // Set up compression parameters
        let mut params: grokj2k_sys::_grk_cparameters = std::mem::zeroed();
        grokj2k_sys::grk_compress_set_default_params(&mut params);
        params.cod_format = grokj2k_sys::_GRK_SUPPORTED_FILE_FMT_GRK_FMT_J2K;
        params.numresolution = num_resolutions;
        params.cblockw_init = codeblock_size;
        params.cblockh_init = codeblock_size;
        params.mct = 0; // No MCT for XYZ data
        params.irreversible = true;

        // Set compression ratio
        if ratio > 1.0 {
            params.layer_rate[0] = ratio as f64;
            params.numlayers = 1;
            params.allocation_by_rate_distortion = true;
        }

        // Set progression order
        let prog = match progression {
            "LRCP" => grokj2k_sys::_GRK_PROG_ORDER_GRK_LRCP,
            "RLCP" => grokj2k_sys::_GRK_PROG_ORDER_GRK_RLCP,
            "RPCL" => grokj2k_sys::_GRK_PROG_ORDER_GRK_RPCL,
            "PCRL" => grokj2k_sys::_GRK_PROG_ORDER_GRK_PCRL,
            "CPRL" => grokj2k_sys::_GRK_PROG_ORDER_GRK_CPRL,
            _ => grokj2k_sys::_GRK_PROG_ORDER_GRK_CPRL,
        };
        params.prog_order = prog;

        // Set up output stream
        let mut stream_params: grokj2k_sys::_grk_stream_params = std::mem::zeroed();
        let c_output = CString::new(output_str).map_err(|e| format!("CString error: {e}"))?;
        let bytes = c_output.as_bytes_with_nul();
        if bytes.len() > stream_params.file.len() {
            grokj2k_sys::grk_object_unref(&mut (*image).obj);
            return Err("Output path too long".to_string());
        }
        for (i, &b) in bytes.iter().enumerate() {
            stream_params.file[i] = b as c_char;
        }

        // Compress
        let codec = grokj2k_sys::grk_compress_init(&mut stream_params, &mut params, image);
        if codec.is_null() {
            grokj2k_sys::grk_object_unref(&mut (*image).obj);
            return Err("grk_compress_init failed".to_string());
        }

        let compressed_bytes = grokj2k_sys::grk_compress(codec, std::ptr::null_mut());
        grokj2k_sys::grk_object_unref(codec);
        grokj2k_sys::grk_object_unref(&mut (*image).obj);

        if compressed_bytes == 0 {
            return Err("grk_compress produced 0 bytes".to_string());
        }

        Ok(compressed_bytes)
    }
}

/// Load a TIFF image and compress it to JPEG 2000 with XYZ transform.
///
/// Convenience wrapper that loads a TIFF via grk_codec_compress with --xyz.
/// Uses the codec API (serialized via mutex) for image loading convenience.
pub fn compress_xyz(
    input: &Path,
    output: &Path,
    ratio: f64,
    num_resolutions: u8,
    codeblock_size: u32,
    progression: &str,
    num_threads: u32,
) -> Result<(), String> {
    let input_str = input.to_str()
        .ok_or_else(|| format!("Invalid input path: {}", input.display()))?;
    let output_str = output.to_str()
        .ok_or_else(|| format!("Invalid output path: {}", output.display()))?;

    let ratio_str = format!("{}", ratio);
    let res_str = format!("{}", num_resolutions);
    let cb_str = format!("{},{}", codeblock_size, codeblock_size);
    let threads_str = format!("{}", num_threads);

    let c_args: Vec<CString> = [
        "grk_compress",
        "-i", input_str,
        "-o", output_str,
        "-r", &ratio_str,
        "-n", &res_str,
        "-b", &cb_str,
        "-p", progression,
        "-H", &threads_str,
        "--xyz",
    ].iter().map(|s| CString::new(*s).unwrap()).collect();

    let c_ptrs: Vec<*const std::os::raw::c_char> = c_args.iter()
        .map(|s| s.as_ptr())
        .collect();

    // Serialize codec calls — grk_codec_compress uses global state
    let _guard = CODEC_LOCK.lock().map_err(|e| format!("Lock error: {e}"))?;

    let ret = unsafe {
        grokj2kcodec_sys::grk_codec_compress(
            c_ptrs.len() as i32,
            c_ptrs.as_ptr() as *mut *const std::os::raw::c_char,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };

    if ret == 0 {
        Ok(())
    } else {
        Err(format!("grk_codec_compress failed (code {})", ret))
    }
}

/// Loaded TIFF frame: planar int32 component buffers + metadata.
pub struct TiffFrame {
    pub components: [Vec<i32>; 3],
    pub width: u32,
    pub height: u32,
    pub precision: u8,
    pub path: PathBuf,
}

/// Load a TIFF file into planar int32 component buffers.
///
/// Supports 8, 12, 16-bit RGB TIFFs. Returns 3 planar buffers (R, G, B).
pub fn load_tiff(path: &Path) -> Result<TiffFrame, String> {
    use tiff::decoder::Decoder;
    use tiff::tags::Tag;
    use std::io::{BufReader, Read, Seek, SeekFrom};

    let file = std::fs::File::open(path)
        .map_err(|e| format!("Cannot open {}: {e}", path.display()))?;
    let mut reader = BufReader::new(file);
    let mut decoder = Decoder::new(&mut reader)
        .map_err(|e| format!("TIFF decode error for {}: {e}", path.display()))?;

    let (width, height) = decoder.dimensions()
        .map_err(|e| format!("TIFF dimensions error: {e}"))?;

    // Read bits per sample
    let bits_per_sample = decoder.get_tag_u32(Tag::BitsPerSample)
        .map_err(|e| format!("Cannot read BitsPerSample: {e}"))? as u8;

    // Read samples per pixel
    let samples_per_pixel = decoder.get_tag_u32(Tag::SamplesPerPixel)
        .unwrap_or(3) as u8;
    if samples_per_pixel < 3 {
        return Err(format!("Need ≥3 samples/pixel, got {}", samples_per_pixel));
    }

    let num_pixels = (width as usize) * (height as usize);

    // For standard bit depths (8, 16), use the tiff crate decoder
    if bits_per_sample == 8 || bits_per_sample == 16 {
        let image = decoder.read_image()
            .map_err(|e| format!("TIFF read error for {}: {e}", path.display()))?;

        let mut r = Vec::with_capacity(num_pixels);
        let mut g = Vec::with_capacity(num_pixels);
        let mut b = Vec::with_capacity(num_pixels);

        match image {
            tiff::decoder::DecodingResult::U8(data) => {
                let ch = samples_per_pixel as usize;
                for i in 0..num_pixels {
                    r.push(data[i * ch] as i32);
                    g.push(data[i * ch + 1] as i32);
                    b.push(data[i * ch + 2] as i32);
                }
            }
            tiff::decoder::DecodingResult::U16(data) => {
                let ch = samples_per_pixel as usize;
                for i in 0..num_pixels {
                    r.push(data[i * ch] as i32);
                    g.push(data[i * ch + 1] as i32);
                    b.push(data[i * ch + 2] as i32);
                }
            }
            _ => return Err("Unsupported TIFF sample format".to_string()),
        }

        return Ok(TiffFrame {
            components: [r, g, b],
            width,
            height,
            precision: bits_per_sample,
            path: path.to_path_buf(),
        });
    }

    // For packed bit depths (e.g. 12-bit), read raw strip data and unpack
    if bits_per_sample != 12 {
        return Err(format!("Unsupported bits/sample: {}", bits_per_sample));
    }

    // Get strip offsets and byte counts
    let strip_offsets = decoder.get_tag_u64_vec(Tag::StripOffsets)
        .map_err(|e| format!("Cannot read StripOffsets: {e}"))?;
    let strip_byte_counts = decoder.get_tag_u64_vec(Tag::StripByteCounts)
        .map_err(|e| format!("Cannot read StripByteCounts: {e}"))?;

    // Read all strip data
    let total_bytes: u64 = strip_byte_counts.iter().sum();
    let mut raw_data = Vec::with_capacity(total_bytes as usize);
    // Need to get inner reader back from decoder
    drop(decoder);
    for (offset, count) in strip_offsets.iter().zip(strip_byte_counts.iter()) {
        reader.seek(SeekFrom::Start(*offset))
            .map_err(|e| format!("Seek error: {e}"))?;
        let mut buf = vec![0u8; *count as usize];
        reader.read_exact(&mut buf)
            .map_err(|e| format!("Read error: {e}"))?;
        raw_data.extend_from_slice(&buf);
    }

    // Unpack 12-bit packed samples (interleaved RGB)
    // Each pair of 12-bit values is stored in 3 bytes: [A₁₁..A₄ | A₃..A₀ B₁₁..B₈ | B₇..B₀]
    let total_samples = num_pixels * samples_per_pixel as usize;
    let mut samples = Vec::with_capacity(total_samples);
    let mut byte_idx = 0usize;
    let mut sample_idx = 0usize;
    while sample_idx < total_samples {
        if byte_idx + 2 >= raw_data.len() { break; }
        if sample_idx + 1 < total_samples {
            // Two 12-bit samples from 3 bytes
            let b0 = raw_data[byte_idx] as u16;
            let b1 = raw_data[byte_idx + 1] as u16;
            let b2 = raw_data[byte_idx + 2] as u16;
            let s0 = (b0 << 4) | (b1 >> 4);
            let s1 = ((b1 & 0x0F) << 8) | b2;
            samples.push(s0 as i32);
            samples.push(s1 as i32);
            byte_idx += 3;
            sample_idx += 2;
        } else {
            // Odd last sample
            let b0 = raw_data[byte_idx] as u16;
            let b1 = raw_data[byte_idx + 1] as u16;
            let s0 = (b0 << 4) | (b1 >> 4);
            samples.push(s0 as i32);
            byte_idx += 2;
            sample_idx += 1;
        }
    }

    // De-interleave into planar buffers
    let ch = samples_per_pixel as usize;
    let mut r = Vec::with_capacity(num_pixels);
    let mut g = Vec::with_capacity(num_pixels);
    let mut b = Vec::with_capacity(num_pixels);
    for i in 0..num_pixels {
        r.push(samples[i * ch]);
        g.push(samples[i * ch + 1]);
        b.push(samples[i * ch + 2]);
    }

    Ok(TiffFrame {
        components: [r, g, b],
        width,
        height,
        precision: bits_per_sample,
        path: path.to_path_buf(),
    })
}

/// Compress a loaded TIFF frame to JPEG 2000 with XYZ transform.
///
/// Uses the core API — fully thread-safe, can be called concurrently.
pub fn compress_frame_xyz(
    frame: &TiffFrame,
    output: &Path,
    ratio: f64,
    num_resolutions: u8,
    codeblock_size: u32,
    progression: &str,
) -> Result<u64, String> {
    compress_image_xyz(
        [&frame.components[0], &frame.components[1], &frame.components[2]],
        frame.width,
        frame.height,
        frame.precision,
        output,
        ratio,
        num_resolutions,
        codeblock_size,
        progression,
    )
}

/// Find the grk_compress binary.
/// Searches: 1) $HOME/bin/grok/bin/ 2) PATH via `which`
pub fn find_grk_compress() -> Option<PathBuf> {
    if let Ok(home) = std::env::var("HOME") {
        let p = PathBuf::from(home).join("bin/grok/bin/grk_compress");
        if p.exists() { return Some(p); }
    }
    // Check PATH
    std::process::Command::new("which")
        .arg("grk_compress")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| PathBuf::from(s.trim()))
}

/// Compress a single TIFF to J2C by spawning a `grk_compress` subprocess.
///
/// Uses `-H 1` (single thread) so the caller can run many in parallel.
pub fn compress_file_subprocess(
    grk_bin: &Path,
    lib_path: &str,
    input: &Path,
    output: &Path,
    ratio: f64,
    num_resolutions: u8,
    codeblock_size: u32,
    progression: &str,
) -> Result<(), String> {
    let status = std::process::Command::new(grk_bin)
        .env("LD_LIBRARY_PATH", lib_path)
        .args([
            "-i", &input.to_string_lossy(),
            "-o", &output.to_string_lossy(),
            "-r", &format!("{}", ratio),
            "--xyz",
            "-n", &format!("{}", num_resolutions),
            "-b", &format!("{},{}", codeblock_size, codeblock_size),
            "-p", progression,
            "-H", "1",
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| format!("Failed to spawn grk_compress: {e}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("grk_compress failed with status {} for {}",
            status, input.display()))
    }
}

/// Get the LD_LIBRARY_PATH for grok libraries.
pub fn grok_lib_path() -> String {
    if let Ok(home) = std::env::var("HOME") {
        let p = format!("{}/bin/grok/lib64", home);
        if std::path::Path::new(&p).exists() {
            return p;
        }
    }
    String::new()
}
