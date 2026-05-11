use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// DCDM colour encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DcdmColourEncoding {
    /// X'Y'Z' 12-bit (SMPTE 428-1)
    Xyz12Bit,
    /// X'Y'Z' 16-bit
    Xyz16Bit,
}

/// DCDM creation options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcdmOptions {
    /// Source image sequence (DPX/TIFF/EXR)
    pub input_dir: PathBuf,
    /// Output DCDM TIFF sequence
    pub output_dir: PathBuf,
    pub encoding: DcdmColourEncoding,
    pub width: u32,
    pub height: u32,
    pub fps_num: u32,
    pub fps_den: u32,
    /// Source colour space for conversion
    pub colour_space: String,
    /// Optional 3D LUT for colour transform
    pub lut_path: PathBuf,
}

impl Default for DcdmOptions {
    fn default() -> Self {
        Self {
            input_dir: PathBuf::new(),
            output_dir: PathBuf::new(),
            encoding: DcdmColourEncoding::Xyz12Bit,
            width: 4096,
            height: 2160,
            fps_num: 24,
            fps_den: 1,
            colour_space: String::new(),
            lut_path: PathBuf::new(),
        }
    }
}

/// Result of DCDM operation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DcdmResult {
    pub success: bool,
    pub error: String,
    pub frames_written: u64,
    pub output_dir: PathBuf,
}

/// Create DCDM (Digital Cinema Distribution Master) from source images.
///
/// Uses ffmpeg to convert source images to X'Y'Z' TIFF sequence.
pub fn create_dcdm(opts: &DcdmOptions) -> DcdmResult {
    if let Err(e) = std::fs::create_dir_all(&opts.output_dir) {
        return DcdmResult {
            success: false,
            error: format!("Failed to create output directory: {e}"),
            ..Default::default()
        };
    }

    // Find source frames
    let frames = match crate::encode::find_source_frames(&opts.input_dir) {
        Ok(f) => f,
        Err(e) => {
            return DcdmResult {
                success: false,
                error: format!("Failed to find source frames: {e}"),
                ..Default::default()
            };
        }
    };

    if frames.is_empty() {
        return DcdmResult {
            success: false,
            error: "No source frames found".into(),
            ..Default::default()
        };
    }

    // Build ffmpeg command for batch conversion
    // Use glob pattern or sequence input
    let first = &frames[0];
    let parent = first.parent().unwrap_or(Path::new("."));
    let ext = first.extension().and_then(|e| e.to_str()).unwrap_or("dpx");

    let input_pattern = parent.join(format!("*.{ext}"));
    let output_pattern = opts.output_dir.join("dcdm_%06d.tif");

    let mut cmd = std::process::Command::new("ffmpeg");
    cmd.arg("-y")
        .arg("-pattern_type")
        .arg("glob")
        .arg("-i")
        .arg(input_pattern.to_string_lossy().as_ref())
        .arg("-pix_fmt")
        .arg(match opts.encoding {
            DcdmColourEncoding::Xyz12Bit => "rgb48le",
            DcdmColourEncoding::Xyz16Bit => "rgb48le",
        })
        .arg("-s")
        .arg(format!("{}x{}", opts.width, opts.height));

    // Add LUT if provided
    if !opts.lut_path.as_os_str().is_empty() && opts.lut_path.exists() {
        cmd.arg("-vf")
            .arg(format!("lut3d={}", opts.lut_path.display()));
    }

    cmd.arg(&output_pattern);

    let output = match cmd.output() {
        Ok(o) => o,
        Err(e) => {
            return DcdmResult {
                success: false,
                error: format!("Failed to run ffmpeg: {e}"),
                ..Default::default()
            };
        }
    };

    if !output.status.success() {
        return DcdmResult {
            success: false,
            error: String::from_utf8_lossy(&output.stderr).into_owned(),
            ..Default::default()
        };
    }

    // Count output frames
    let frames_written = std::fs::read_dir(&opts.output_dir)
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| {
                    e.path()
                        .extension()
                        .is_some_and(|ext| ext == "tif" || ext == "tiff")
                })
                .count() as u64
        })
        .unwrap_or(0);

    DcdmResult {
        success: true,
        error: String::new(),
        frames_written,
        output_dir: opts.output_dir.clone(),
    }
}

/// Convert DCDM back to viewable format (e.g. Rec.709 ProRes for review).
pub fn export_dcdm(
    dcdm_dir: &Path,
    output_dir: &Path,
    target_colour_space: Option<&str>,
) -> DcdmResult {
    if let Err(e) = std::fs::create_dir_all(output_dir) {
        return DcdmResult {
            success: false,
            error: format!("Failed to create output directory: {e}"),
            ..Default::default()
        };
    }

    let input_pattern = dcdm_dir.join("*.tif");
    let output_file = output_dir.join("review.mov");

    let colour_filter = match target_colour_space {
        Some("rec709") | Some("Rec709") | None => "colorspace=all=bt709:iall=bt709",
        Some("p3") | Some("P3") => "colorspace=all=bt709:iprimaries=smpte431",
        Some(_) => "colorspace=all=bt709",
    };

    let output = std::process::Command::new("ffmpeg")
        .arg("-y")
        .arg("-pattern_type")
        .arg("glob")
        .arg("-i")
        .arg(input_pattern.to_string_lossy().as_ref())
        .arg("-vf")
        .arg(colour_filter)
        .arg("-c:v")
        .arg("prores_ks")
        .arg("-profile:v")
        .arg("3")
        .arg(&output_file)
        .output();

    match output {
        Ok(o) if o.status.success() => DcdmResult {
            success: true,
            error: String::new(),
            frames_written: 0,
            output_dir: output_dir.to_path_buf(),
        },
        Ok(o) => DcdmResult {
            success: false,
            error: String::from_utf8_lossy(&o.stderr).into_owned(),
            ..Default::default()
        },
        Err(e) => DcdmResult {
            success: false,
            error: format!("Failed to run ffmpeg: {e}"),
            ..Default::default()
        },
    }
}
