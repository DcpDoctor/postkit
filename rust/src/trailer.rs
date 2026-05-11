use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Rating system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum RatingSystem {
    #[default]
    Mpaa,
    Bbfc,
    Fsk,
    Custom,
}

/// Trailer band colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TrailerBand {
    #[default]
    Green,
    Red,
    Yellow,
}

/// Trailer packaging options.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrailerOptions {
    pub content_dir: PathBuf,
    pub audio_file: PathBuf,
    pub output_dir: PathBuf,
    pub title: String,
    pub rating: String,
    pub rating_system: RatingSystem,
    pub band: TrailerBand,
    pub countdown_seconds: u32,
    pub fps_num: u32,
    pub fps_den: u32,
}

/// Result of trailer packaging.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrailerResult {
    pub success: bool,
    pub error: String,
    pub output_dir: PathBuf,
    pub cpl_uuid: String,
}

/// Package a trailer (ratings card + leader + content).
pub fn package_trailer(_opts: &TrailerOptions) -> TrailerResult {
    tracing::warn!("package_trailer: not yet implemented");
    TrailerResult::default()
}
