use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Timeline edit decision format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimelineFormat {
    EdlCmx3600,
    Aaf,
    /// Final Cut Pro XML
    XmlFcp,
    /// FCP X XML
    XmlFcpx,
    /// DaVinci Resolve XML
    XmlResolve,
    /// OpenTimelineIO
    Otio,
    Unknown,
}

/// A single edit event in a timeline.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditEvent {
    pub event_number: u32,
    pub reel_name: String,
    /// "V", "A1", "A2", etc.
    pub track_type: String,
    /// Source in frame number
    pub source_in: u32,
    pub source_out: u32,
    pub record_in: u32,
    pub record_out: u32,
    /// "CUT", "DISSOLVE"
    pub transition: String,
    pub comment: String,
}

/// Parsed timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    pub title: String,
    pub frame_rate: f64,
    pub format: TimelineFormat,
    pub events: Vec<EditEvent>,
}

impl Default for Timeline {
    fn default() -> Self {
        Self {
            title: String::new(),
            frame_rate: 24.0,
            format: TimelineFormat::Unknown,
            events: Vec::new(),
        }
    }
}

/// Conform options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConformOptions {
    /// EDL/AAF/XML path
    pub timeline_file: PathBuf,
    /// Directory containing source reels
    pub media_dir: PathBuf,
    /// Assembled output
    pub output_dir: PathBuf,
    pub auto_detect_format: bool,
    pub force_format: TimelineFormat,
    pub frame_rate: f64,
}

impl Default for ConformOptions {
    fn default() -> Self {
        Self {
            timeline_file: PathBuf::new(),
            media_dir: PathBuf::new(),
            output_dir: PathBuf::new(),
            auto_detect_format: true,
            force_format: TimelineFormat::Unknown,
            frame_rate: 24.0,
        }
    }
}

/// Parse a timeline file (EDL, AAF, XML).
pub fn parse_timeline(_file: &Path) -> Timeline {
    tracing::warn!("parse_timeline: not yet implemented");
    Timeline::default()
}

/// Detect timeline format from file extension/content.
pub fn detect_timeline_format(file: &Path) -> TimelineFormat {
    match file
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .as_deref()
    {
        Some("edl") => TimelineFormat::EdlCmx3600,
        Some("aaf") => TimelineFormat::Aaf,
        Some("otio") => TimelineFormat::Otio,
        Some("xml" | "fcpxml") => TimelineFormat::XmlFcpx,
        _ => TimelineFormat::Unknown,
    }
}

/// Conform/assemble media from a timeline into reel structure.
pub fn conform(_opts: &ConformOptions) -> i32 {
    tracing::warn!("conform: not yet implemented");
    0
}

/// Verify that all source reels referenced in timeline exist in media_dir.
pub fn find_missing_reels(_timeline: &Timeline, _media_dir: &Path) -> Vec<String> {
    tracing::warn!("find_missing_reels: not yet implemented");
    Vec::new()
}
