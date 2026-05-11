use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Forensic watermark backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum WatermarkBackend {
    NexGuard,
    Civolution,
    #[default]
    Internal,
}

/// Watermark options.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WatermarkOptions {
    pub backend: WatermarkBackend,
    pub operator_id: String,
    pub session_id: String,
    pub strength: f32,
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub license_file: PathBuf,
}

/// Watermark operation result.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WatermarkResult {
    pub success: bool,
    pub error: String,
    pub frames_processed: u64,
    pub payload_hash: String,
}

/// Embed forensic watermark into frame sequence.
pub fn embed_watermark(_opts: &WatermarkOptions) -> WatermarkResult {
    tracing::warn!("embed_watermark: not yet implemented");
    WatermarkResult::default()
}

/// Detect forensic watermark in frame sequence.
pub fn detect_watermark(
    _input: &Path,
    _backend: WatermarkBackend,
    _license_file: Option<&Path>,
) -> WatermarkResult {
    tracing::warn!("detect_watermark: not yet implemented");
    WatermarkResult::default()
}
