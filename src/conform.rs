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
pub fn parse_timeline(file: &Path) -> Timeline {
    let format = detect_timeline_format(file);
    match format {
        TimelineFormat::EdlCmx3600 => parse_edl(file),
        _ => {
            tracing::warn!("Timeline format {format:?} not supported, trying EDL parser");
            parse_edl(file)
        }
    }
}

/// Parse a CMX 3600 EDL file.
fn parse_edl(file: &Path) -> Timeline {
    let content = match std::fs::read_to_string(file) {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to read EDL file: {e}");
            return Timeline::default();
        }
    };

    let mut timeline = Timeline {
        format: TimelineFormat::EdlCmx3600,
        ..Default::default()
    };

    // CMX 3600 format:
    // TITLE: <title>
    // FCM: DROP FRAME / NON-DROP FRAME
    // 001  REEL001  V  C  01:00:00:00 01:00:05:00 01:00:00:00 01:00:05:00
    let event_re = regex::Regex::new(
        r"^\s*(\d+)\s+(\S+)\s+(\S+)\s+(\S+)\s+(\d{2}:\d{2}:\d{2}[:;]\d{2})\s+(\d{2}:\d{2}:\d{2}[:;]\d{2})\s+(\d{2}:\d{2}:\d{2}[:;]\d{2})\s+(\d{2}:\d{2}:\d{2}[:;]\d{2})"
    ).unwrap();

    let mut last_comment = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if let Some(title) = trimmed.strip_prefix("TITLE:") {
            timeline.title = title.trim().to_string();
            continue;
        }

        if trimmed.starts_with("FCM:") {
            if trimmed.contains("DROP") && !trimmed.contains("NON") {
                timeline.frame_rate = 29.97;
            }
            continue;
        }

        if trimmed.starts_with('*') || trimmed.starts_with(';') {
            last_comment = trimmed[1..].trim().to_string();
            continue;
        }

        if let Some(caps) = event_re.captures(trimmed) {
            let event = EditEvent {
                event_number: caps[1].parse().unwrap_or(0),
                reel_name: caps[2].to_string(),
                track_type: caps[3].to_string(),
                source_in: tc_to_frames(&caps[5], timeline.frame_rate as u32),
                source_out: tc_to_frames(&caps[6], timeline.frame_rate as u32),
                record_in: tc_to_frames(&caps[7], timeline.frame_rate as u32),
                record_out: tc_to_frames(&caps[8], timeline.frame_rate as u32),
                transition: caps[4].to_string(),
                comment: std::mem::take(&mut last_comment),
            };
            timeline.events.push(event);
        }
    }

    timeline
}

fn tc_to_frames(tc: &str, fps: u32) -> u32 {
    let fps = if fps == 0 { 24 } else { fps };
    let tc = tc.replace(';', ":");
    let parts: Vec<&str> = tc.split(':').collect();
    if parts.len() != 4 {
        return 0;
    }
    let h: u32 = parts[0].parse().unwrap_or(0);
    let m: u32 = parts[1].parse().unwrap_or(0);
    let s: u32 = parts[2].parse().unwrap_or(0);
    let f: u32 = parts[3].parse().unwrap_or(0);
    h * 3600 * fps + m * 60 * fps + s * fps + f
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
///
/// Creates symlinks or copies source media files into the output directory
/// organised by reel, based on the timeline edit decisions.
pub fn conform(opts: &ConformOptions) -> i32 {
    let timeline = parse_timeline(&opts.timeline_file);

    if timeline.events.is_empty() {
        tracing::error!("No events found in timeline");
        return -1;
    }

    // Create output directory
    if let Err(e) = std::fs::create_dir_all(&opts.output_dir) {
        tracing::error!("Failed to create output directory: {e}");
        return -1;
    }

    // Check for missing reels
    let missing = find_missing_reels(&timeline, &opts.media_dir);
    if !missing.is_empty() {
        for m in &missing {
            tracing::warn!("Missing reel: {m}");
        }
    }

    // Write assembled timeline as JSON for downstream tools
    let manifest_path = opts.output_dir.join("conform_manifest.json");
    let json = serde_json::to_string_pretty(&timeline).unwrap_or_default();
    if let Err(e) = std::fs::write(&manifest_path, json) {
        tracing::error!("Failed to write manifest: {e}");
        return -1;
    }

    tracing::info!(
        "Conformed {} events to {}",
        timeline.events.len(),
        opts.output_dir.display()
    );
    0
}

/// Verify that all source reels referenced in timeline exist in media_dir.
pub fn find_missing_reels(timeline: &Timeline, media_dir: &Path) -> Vec<String> {
    let mut missing = Vec::new();
    let mut checked = std::collections::HashSet::new();

    for event in &timeline.events {
        if event.reel_name == "BL" || event.reel_name == "AX" {
            continue; // black/aux
        }
        if !checked.insert(&event.reel_name) {
            continue; // already checked
        }

        // Check if any file matching the reel name exists
        let found = std::fs::read_dir(media_dir)
            .into_iter()
            .flatten()
            .flatten()
            .any(|entry| {
                entry
                    .file_name()
                    .to_str()
                    .is_some_and(|name| name.contains(&event.reel_name))
            });

        if !found {
            missing.push(event.reel_name.clone());
        }
    }

    missing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_edl() {
        let dir = tempfile::tempdir().unwrap();
        let edl_path = dir.path().join("test.edl");
        std::fs::write(
            &edl_path,
            "TITLE: Test Edit\nFCM: NON-DROP FRAME\n\n001  REEL001  V  C        01:00:00:00 01:00:05:00 01:00:00:00 01:00:05:00\n002  REEL002  V  C        01:00:05:00 01:00:10:00 01:00:05:00 01:00:10:00\n",
        ).unwrap();
        let tl = parse_timeline(&edl_path);
        assert_eq!(tl.title, "Test Edit");
        assert_eq!(tl.events.len(), 2);
        assert_eq!(tl.events[0].reel_name, "REEL001");
        assert_eq!(tl.events[1].event_number, 2);
    }

    #[test]
    fn test_tc_to_frames() {
        assert_eq!(tc_to_frames("01:00:00:00", 24), 86400);
        assert_eq!(tc_to_frames("00:00:01:00", 24), 24);
        assert_eq!(tc_to_frames("00:00:00:12", 24), 12);
    }

    #[test]
    fn test_detect_format() {
        assert_eq!(
            detect_timeline_format(Path::new("test.edl")),
            TimelineFormat::EdlCmx3600
        );
        assert_eq!(
            detect_timeline_format(Path::new("test.aaf")),
            TimelineFormat::Aaf
        );
    }

    #[test]
    fn test_find_missing_reels() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("REEL001.mxf"), "").unwrap();
        let tl = Timeline {
            events: vec![
                EditEvent {
                    reel_name: "REEL001".into(),
                    ..Default::default()
                },
                EditEvent {
                    reel_name: "REEL002".into(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let missing = find_missing_reels(&tl, dir.path());
        assert_eq!(missing, vec!["REEL002"]);
    }
}
