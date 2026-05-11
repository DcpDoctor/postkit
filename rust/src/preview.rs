use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Frame-accurate media player/preview options for DCP and IMF content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackOptions {
    pub input: PathBuf,
    pub cpl_uuid: String,
    pub start_frame: u32,
    /// 0 = play to end
    pub end_frame: u32,
    pub loop_playback: bool,
    pub decode_to_display: bool,
    pub display_colourspace: String,
    pub gpu_device: i32,
}

impl Default for PlaybackOptions {
    fn default() -> Self {
        Self {
            input: PathBuf::new(),
            cpl_uuid: String::new(),
            start_frame: 0,
            end_frame: 0,
            loop_playback: false,
            decode_to_display: true,
            display_colourspace: "sRGB".to_string(),
            gpu_device: -1,
        }
    }
}

/// Frame metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FrameInfo {
    pub frame_number: u32,
    pub width: u32,
    pub height: u32,
    pub bitrate_kbps: u32,
    pub codec: String,
}

/// Extract a single frame as image (thumbnail/QC).
pub fn extract_frame(_input: &Path, _frame: u32, _output_image: &Path) -> i32 {
    tracing::warn!("extract_frame: not yet implemented");
    0
}

/// Get frame metadata without full decode.
pub fn get_frame_info(_input: &Path, _frame: u32) -> FrameInfo {
    tracing::warn!("get_frame_info: not yet implemented");
    FrameInfo::default()
}

/// Start playback (blocking).
pub fn play(_opts: &PlaybackOptions) -> i32 {
    tracing::warn!("play: not yet implemented");
    0
}

/// Render all frames to image sequence.
pub fn render_to_sequence(_input: &Path, _output_dir: &Path, _format: Option<&str>) -> i32 {
    tracing::warn!("render_to_sequence: not yet implemented");
    0
}
