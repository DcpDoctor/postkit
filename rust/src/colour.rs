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

/// Convert colour space of an image or sequence.
pub fn convert_colour(_opts: &ColourConvertOptions) -> std::io::Result<()> {
    tracing::warn!("convert_colour: not yet implemented");
    Ok(())
}
