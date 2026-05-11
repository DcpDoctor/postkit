use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Camera raw format identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CameraFormat {
    Arriraw,
    RedR3d,
    SonyRaw,
    CanonRaw,
    BlackmagicBraw,
    ProRes,
    DnxHr,
    #[default]
    Unknown,
}

/// Ingest options for camera media.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestOptions {
    /// Camera card/media directory
    pub source: PathBuf,
    /// Destination for transcoded media
    pub output_dir: PathBuf,
    /// "dpx", "tiff", "exr", "prores"
    pub output_format: String,
    /// "ACES", "Rec.709", "P3", "LogC"
    pub colour_space: String,
    /// 1=fast, 3=high quality
    pub debayer_quality: u32,
    pub apply_lut: bool,
    pub lut_path: PathBuf,
    pub gpu_device: i32,
}

impl Default for IngestOptions {
    fn default() -> Self {
        Self {
            source: PathBuf::new(),
            output_dir: PathBuf::new(),
            output_format: "dpx".to_string(),
            colour_space: "ACES".to_string(),
            debayer_quality: 3,
            apply_lut: false,
            lut_path: PathBuf::new(),
            gpu_device: -1,
        }
    }
}

/// Detected camera clip metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClipInfo {
    pub path: PathBuf,
    pub format: CameraFormat,
    pub width: u32,
    pub height: u32,
    pub frame_rate: f64,
    pub frame_count: u32,
    pub codec: String,
    pub colour_space: String,
    pub camera_model: String,
    pub reel_name: String,
}

/// Detect camera format from directory/file.
pub fn detect_format(_source: &Path) -> CameraFormat {
    tracing::warn!("detect_format: not yet implemented");
    CameraFormat::Unknown
}

/// Scan a camera card and return clip info.
pub fn scan_media(_source: &Path) -> Vec<ClipInfo> {
    tracing::warn!("scan_media: not yet implemented");
    Vec::new()
}

/// Ingest/transcode camera media to standardized intermediate.
pub fn ingest(_opts: &IngestOptions) -> i32 {
    tracing::warn!("ingest: not yet implemented");
    0
}
