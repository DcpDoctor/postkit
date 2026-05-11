use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// JPEG 2000 encoding options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodeOptions {
    /// Source image sequence directory (DPX/TIFF/EXR/PNG/BMP)
    pub input_dir: PathBuf,
    /// Output J2K codestream directory
    pub output_dir: PathBuf,
    /// Target bitrate in Mbps (e.g. 250.0 for DCI 2K)
    pub bitrate_mbps: f64,
    /// Resolution: "2K" or "4K"
    pub resolution: String,
    /// Frame rate numerator
    pub fps_num: u32,
    /// Frame rate denominator
    pub fps_den: u32,
    /// Number of quality layers
    pub num_layers: u32,
    /// Progression order: "CPRL", "LRCP", "RLCP"
    pub progression: String,
    /// Number of decomposition levels
    pub num_resolutions: u32,
    /// Code block size (usually 32 or 64)
    pub codeblock_size: u32,
    /// Path to external grok compressor binary (grk_compress)
    pub compressor_path: PathBuf,
    /// GPU device index (-1 for CPU)
    pub gpu_device: i32,
    /// Number of parallel encoding threads
    pub num_threads: u32,
}

impl Default for EncodeOptions {
    fn default() -> Self {
        Self {
            input_dir: PathBuf::new(),
            output_dir: PathBuf::new(),
            bitrate_mbps: 250.0,
            resolution: "2K".to_string(),
            fps_num: 24,
            fps_den: 1,
            num_layers: 1,
            progression: "CPRL".to_string(),
            num_resolutions: 6,
            codeblock_size: 32,
            compressor_path: PathBuf::new(),
            gpu_device: -1,
            num_threads: 0, // auto-detect
        }
    }
}

/// Result of encoding operation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EncodeResult {
    pub success: bool,
    pub error: String,
    pub frames_encoded: u64,
    pub output_dir: PathBuf,
}

/// Image format detected from file extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Dpx,
    Tiff,
    Exr,
    Png,
    Bmp,
    Unknown,
}

/// Detect image format from file extension.
pub fn detect_image_format(path: &Path) -> ImageFormat {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .as_deref()
    {
        Some("dpx") => ImageFormat::Dpx,
        Some("tif" | "tiff") => ImageFormat::Tiff,
        Some("exr") => ImageFormat::Exr,
        Some("png") => ImageFormat::Png,
        Some("bmp") => ImageFormat::Bmp,
        _ => ImageFormat::Unknown,
    }
}

/// Find source image files in a directory, sorted by name.
pub fn find_source_frames(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut frames: Vec<PathBuf> = std::fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file() && detect_image_format(p) != ImageFormat::Unknown)
        .collect();
    frames.sort();
    Ok(frames)
}

/// Encode a sequence of images to JPEG 2000 using an external compressor.
///
/// This spawns the compressor binary (e.g. `grk_compress`) for each frame.
/// For GPU-accelerated encoding, set `opts.gpu_device` to the device index.
pub fn encode(opts: &EncodeOptions) -> EncodeResult {
    let compressor = if opts.compressor_path.as_os_str().is_empty() {
        // Try to find grk_compress in PATH
        which_compressor()
    } else {
        Some(opts.compressor_path.clone())
    };

    let Some(compressor) = compressor else {
        return EncodeResult {
            success: false,
            error: "grk_compress not found in PATH and no compressor_path specified".to_string(),
            ..Default::default()
        };
    };

    let frames = match find_source_frames(&opts.input_dir) {
        Ok(f) => f,
        Err(e) => {
            return EncodeResult {
                success: false,
                error: format!("Failed to read input directory: {e}"),
                ..Default::default()
            };
        }
    };

    if frames.is_empty() {
        return EncodeResult {
            success: false,
            error: "No source image files found in input directory".to_string(),
            ..Default::default()
        };
    }

    if let Err(e) = std::fs::create_dir_all(&opts.output_dir) {
        return EncodeResult {
            success: false,
            error: format!("Failed to create output directory: {e}"),
            ..Default::default()
        };
    }

    let mut encoded = 0u64;
    for frame in &frames {
        let stem = frame.file_stem().unwrap_or_default();
        let output = opts
            .output_dir
            .join(format!("{}.j2c", stem.to_string_lossy()));

        let mut cmd = std::process::Command::new(&compressor);
        cmd.arg("-i")
            .arg(frame)
            .arg("-o")
            .arg(&output)
            .arg("-r")
            .arg(format!("{}", opts.bitrate_mbps));

        if opts.gpu_device >= 0 {
            cmd.arg("-G").arg(format!("{}", opts.gpu_device));
        }
        if opts.num_threads > 0 {
            cmd.arg("-t").arg(format!("{}", opts.num_threads));
        }

        match cmd.output() {
            Ok(out) if out.status.success() => {
                encoded += 1;
            }
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                tracing::error!("Encode failed for {:?}: {}", frame, stderr);
                return EncodeResult {
                    success: false,
                    error: format!("Encode failed at frame {}: {}", encoded, stderr),
                    frames_encoded: encoded,
                    output_dir: opts.output_dir.clone(),
                };
            }
            Err(e) => {
                return EncodeResult {
                    success: false,
                    error: format!("Failed to spawn compressor: {e}"),
                    frames_encoded: encoded,
                    output_dir: opts.output_dir.clone(),
                };
            }
        }
    }

    EncodeResult {
        success: true,
        error: String::new(),
        frames_encoded: encoded,
        output_dir: opts.output_dir.clone(),
    }
}

/// Try to find `grk_compress` in PATH.
fn which_compressor() -> Option<PathBuf> {
    std::process::Command::new("which")
        .arg("grk_compress")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| {
            let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if s.is_empty() {
                None
            } else {
                Some(PathBuf::from(s))
            }
        })
}
