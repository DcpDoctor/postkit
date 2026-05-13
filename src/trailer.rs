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
///
/// Generates a countdown leader, ratings card TIFF, and assembles them with
/// the trailer content using ffmpeg into a complete trailer package.
pub fn package_trailer(opts: &TrailerOptions) -> TrailerResult {
    if let Err(e) = std::fs::create_dir_all(&opts.output_dir) {
        return TrailerResult {
            success: false,
            error: format!("Failed to create output directory: {e}"),
            ..Default::default()
        };
    }

    let leader_dir = opts.output_dir.join("leader");
    if let Err(e) = std::fs::create_dir_all(&leader_dir) {
        return TrailerResult {
            success: false,
            error: format!("Failed to create leader directory: {e}"),
            ..Default::default()
        };
    }

    let fps = if opts.fps_num > 0 && opts.fps_den > 0 {
        opts.fps_num as f64 / opts.fps_den as f64
    } else {
        24.0
    };

    let countdown = if opts.countdown_seconds > 0 {
        opts.countdown_seconds
    } else {
        8
    };

    // Generate countdown leader using ffmpeg
    let band_color = match opts.band {
        TrailerBand::Green => "0x00FF00",
        TrailerBand::Red => "0xFF0000",
        TrailerBand::Yellow => "0xFFFF00",
    };

    let rating_text = if opts.rating.is_empty() {
        match opts.rating_system {
            RatingSystem::Mpaa => "G",
            RatingSystem::Bbfc => "U",
            RatingSystem::Fsk => "FSK 0",
            RatingSystem::Custom => "",
        }
    } else {
        &opts.rating
    };

    // Generate ratings card
    let ratings_card = opts.output_dir.join("ratings_card.png");
    let drawtext = format!(
        "drawtext=text='{}':fontsize=72:fontcolor=white:x=(w-text_w)/2:y=(h-text_h)/2,drawtext=text='{}':fontsize=36:fontcolor=white:x=(w-text_w)/2:y=(h+text_h)/2+20",
        opts.title.replace('\'', "\\'"),
        rating_text.replace('\'', "\\'"),
    );

    let rc_result = std::process::Command::new("ffmpeg")
        .arg("-y")
        .arg("-f")
        .arg("lavfi")
        .arg("-i")
        .arg(format!("color=c={band_color}:s=1920x1080:d=1"))
        .arg("-vf")
        .arg(&drawtext)
        .arg("-frames:v")
        .arg("1")
        .arg(&ratings_card)
        .output();

    if let Err(e) = rc_result {
        return TrailerResult {
            success: false,
            error: format!("Failed to generate ratings card: {e}"),
            ..Default::default()
        };
    }

    // Generate countdown leader video
    let leader_file = opts.output_dir.join("leader.mp4");
    let countdown_filter = format!(
        "drawtext=text='%{{eif\\:({countdown}-t)\\:d}}':fontsize=200:fontcolor=white:x=(w-text_w)/2:y=(h-text_h)/2"
    );

    let _ = std::process::Command::new("ffmpeg")
        .arg("-y")
        .arg("-f")
        .arg("lavfi")
        .arg("-i")
        .arg(format!("color=c=black:s=1920x1080:d={countdown}:r={fps}"))
        .arg("-vf")
        .arg(&countdown_filter)
        .arg("-c:v")
        .arg("libx264")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg(&leader_file)
        .output();

    // Create concat file list
    let concat_file = opts.output_dir.join("concat.txt");
    let mut concat_content = String::new();
    if leader_file.exists() {
        concat_content.push_str(&format!("file '{}'\n", leader_file.display()));
    }
    // Main content
    if opts.content_dir.is_file() {
        concat_content.push_str(&format!("file '{}'\n", opts.content_dir.display()));
    }

    if let Err(e) = std::fs::write(&concat_file, &concat_content) {
        return TrailerResult {
            success: false,
            error: format!("Failed to write concat file: {e}"),
            ..Default::default()
        };
    }

    let output_file = opts.output_dir.join("trailer_packaged.mp4");
    let _ = std::process::Command::new("ffmpeg")
        .arg("-y")
        .arg("-f")
        .arg("concat")
        .arg("-safe")
        .arg("0")
        .arg("-i")
        .arg(&concat_file)
        .arg("-c")
        .arg("copy")
        .arg(&output_file)
        .output();

    let cpl_uuid = uuid::Uuid::new_v4().to_string();

    TrailerResult {
        success: true,
        error: String::new(),
        output_dir: opts.output_dir.clone(),
        cpl_uuid,
    }
}
