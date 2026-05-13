use serde::{Deserialize, Serialize};
use std::path::Path;

/// EBU R128 loudness measurement result.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoudnessResult {
    /// Integrated loudness in LUFS.
    pub integrated_lufs: f64,
    /// Loudness range in LU.
    pub range_lu: f64,
    /// True peak in dBTP.
    pub true_peak_dbtp: f64,
    /// Short-term loudness max in LUFS.
    pub short_term_max_lufs: f64,
    /// Whether measurement succeeded.
    pub success: bool,
    pub error: String,
}

/// Measure audio loudness per EBU R128 using ffmpeg.
pub fn measure_loudness(input: &Path) -> LoudnessResult {
    let output = match std::process::Command::new("ffmpeg")
        .args([
            "-i",
            &input.to_string_lossy(),
            "-af",
            "loudnorm=print_format=json",
            "-f",
            "null",
            "-",
        ])
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            return LoudnessResult {
                success: false,
                error: format!("Failed to run ffmpeg: {e}"),
                ..Default::default()
            };
        }
    };

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Parse loudnorm JSON output from ffmpeg stderr
    if let Some(json_start) = stderr.rfind('{')
        && let Some(json_end) = stderr[json_start..].find('}')
    {
        let json_str = &stderr[json_start..json_start + json_end + 1];
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(json_str) {
            return LoudnessResult {
                integrated_lufs: val["input_i"]
                    .as_str()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0),
                range_lu: val["input_lra"]
                    .as_str()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0),
                true_peak_dbtp: val["input_tp"]
                    .as_str()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0),
                short_term_max_lufs: 0.0,
                success: true,
                error: String::new(),
            };
        }
    }

    LoudnessResult {
        success: false,
        error: "Failed to parse loudnorm output from ffmpeg".to_string(),
        ..Default::default()
    }
}
