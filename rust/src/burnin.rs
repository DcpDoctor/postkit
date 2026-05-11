use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Burn-in options for subtitle/watermark overlay.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BurninOptions {
    pub input: PathBuf,
    pub output: PathBuf,
    /// Subtitle file path (SRT, VTT, ASS)
    pub subtitle_file: Option<PathBuf>,
    /// Burn-in text (for watermark/slate)
    pub text: Option<String>,
    /// Font size in pixels
    pub font_size: u32,
    /// Font colour as hex (e.g. "FFFFFF")
    pub font_colour: String,
    /// Position: "top", "bottom", "center"
    pub position: String,
}

/// Burn subtitles or text into video frames using ffmpeg.
pub fn burnin(opts: &BurninOptions) -> std::io::Result<()> {
    let mut args = vec!["-i".to_string(), opts.input.to_string_lossy().to_string()];

    if let Some(ref sub) = opts.subtitle_file {
        args.push("-vf".to_string());
        args.push(format!("subtitles={}", sub.to_string_lossy()));
    } else if let Some(ref text) = opts.text {
        let fontsize = if opts.font_size > 0 {
            opts.font_size
        } else {
            24
        };
        let colour = if opts.font_colour.is_empty() {
            "white"
        } else {
            &opts.font_colour
        };
        let y_pos = match opts.position.as_str() {
            "top" => "10",
            "center" => "(h-text_h)/2",
            _ => "h-th-10",
        };
        args.push("-vf".to_string());
        args.push(format!(
            "drawtext=text='{text}':fontsize={fontsize}:fontcolor={colour}:x=(w-text_w)/2:y={y_pos}"
        ));
    }

    args.push("-y".to_string());
    args.push(opts.output.to_string_lossy().to_string());

    let output = std::process::Command::new("ffmpeg").args(&args).output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("ffmpeg burn-in failed: {stderr}"),
        ));
    }

    Ok(())
}
