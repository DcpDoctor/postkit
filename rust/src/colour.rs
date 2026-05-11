use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Colour space identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColourSpace {
    /// Rec. 709 / sRGB
    Rec709,
    /// DCI-P3
    P3,
    /// CIE XYZ (digital cinema)
    Xyz,
    /// Rec. 2020
    Rec2020,
    /// ACES (AP0)
    Aces,
    /// ACEScg (AP1)
    AcesCg,
    /// Alexa LogC
    LogC,
}

/// Colour conversion options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColourConvertOptions {
    pub input: PathBuf,
    pub output: PathBuf,
    pub source_space: ColourSpace,
    pub target_space: ColourSpace,
    /// Optional 3D LUT path for custom transform
    pub lut_path: Option<PathBuf>,
}

/// Convert colour space of an image or sequence using ffmpeg.
pub fn convert_colour(opts: &ColourConvertOptions) -> std::io::Result<()> {
    let mut cmd = std::process::Command::new("ffmpeg");
    cmd.arg("-y").arg("-i").arg(&opts.input);

    // If a custom LUT is provided, use it
    if let Some(ref lut) = opts.lut_path {
        cmd.arg("-vf").arg(format!("lut3d={}", lut.display()));
    } else {
        // Use ffmpeg colorspace filter for standard conversions
        let (colorspace, primaries, trc) = ffmpeg_color_params(opts.target_space);
        let (in_colorspace, in_primaries, in_trc) = ffmpeg_color_params(opts.source_space);

        let filter = format!(
            "colorspace=all={colorspace}:iall={in_colorspace}:iprimaries={in_primaries}:itrc={in_trc}:primaries={primaries}:trc={trc}"
        );
        cmd.arg("-vf").arg(filter);
    }

    cmd.arg(&opts.output);

    let output = cmd.output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(std::io::Error::other(format!(
            "ffmpeg colour conversion failed: {stderr}"
        )));
    }
    Ok(())
}

fn ffmpeg_color_params(cs: ColourSpace) -> (&'static str, &'static str, &'static str) {
    match cs {
        ColourSpace::Rec709 => ("bt709", "bt709", "bt709"),
        ColourSpace::P3 => ("bt709", "smpte431", "bt709"),
        ColourSpace::Xyz => ("bt709", "bt709", "linear"),
        ColourSpace::Rec2020 => ("bt2020ncl", "bt2020", "bt2020-10"),
        ColourSpace::Aces | ColourSpace::AcesCg => ("bt709", "bt709", "linear"),
        ColourSpace::LogC => ("bt709", "bt709", "log"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffmpeg_color_params() {
        let (cs, p, t) = ffmpeg_color_params(ColourSpace::Rec709);
        assert_eq!(cs, "bt709");
        assert_eq!(p, "bt709");
        assert_eq!(t, "bt709");
    }
}
