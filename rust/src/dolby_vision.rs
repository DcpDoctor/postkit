use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Dolby Vision profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DolbyVisionProfile {
    /// MEL (Minimum Enhancement Layer) — single-layer PQ
    Profile5,
    /// HLG backward compatible
    Profile8,
    /// SDR backward compatible (most common for cinema)
    Profile81,
    Unknown,
}

/// HDR metadata type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HdrType {
    Sdr,
    /// Static PQ metadata (SMPTE ST 2086 + CTA 861.3)
    Hdr10,
    /// Dynamic PQ metadata (Samsung)
    Hdr10Plus,
    /// Dolby Vision RPU
    DolbyVision,
    /// Hybrid Log-Gamma
    Hlg,
    /// Academy Color Encoding System
    Aces,
}

/// Static HDR10 metadata (mastering display + content light level).
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Hdr10Metadata {
    // Mastering display colour volume (SMPTE ST 2086)
    pub display_primaries_gx: u16,
    pub display_primaries_gy: u16,
    pub display_primaries_bx: u16,
    pub display_primaries_by: u16,
    pub display_primaries_rx: u16,
    pub display_primaries_ry: u16,
    pub white_point_x: u16,
    pub white_point_y: u16,
    /// cd/m² × 10000
    pub max_luminance: u32,
    /// cd/m² × 10000
    pub min_luminance: u32,
    // Content light level (CTA 861.3)
    /// MaxCLL
    pub max_cll: u16,
    /// MaxFALL
    pub max_fall: u16,
}

/// Dolby Vision RPU (Reference Processing Unit) options.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DolbyVisionOptions {
    /// Source video/image sequence
    pub input: PathBuf,
    /// .bin RPU file or XML metadata
    pub rpu_file: PathBuf,
    pub profile: DolbyVisionProfile,
    pub output: PathBuf,
    /// Embed RPU in output MXF
    pub embed_rpu: bool,
}

impl Default for DolbyVisionProfile {
    fn default() -> Self {
        Self::Profile81
    }
}

/// HDR metadata injection options.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HdrMetadataOptions {
    pub input: PathBuf,
    pub hdr_type: HdrType,
    pub hdr10: Hdr10Metadata,
    /// Dolby Vision metadata XML
    pub dolby_vision_xml: PathBuf,
    pub output: PathBuf,
}

impl Default for HdrType {
    fn default() -> Self {
        Self::Sdr
    }
}

/// Inject HDR10 static metadata into MXF/CPL.
pub fn inject_hdr10_metadata(_opts: &HdrMetadataOptions) -> i32 {
    tracing::warn!("inject_hdr10_metadata: not yet implemented");
    0
}

/// Inject Dolby Vision RPU into MXF track.
pub fn inject_dolby_vision(_opts: &DolbyVisionOptions) -> i32 {
    tracing::warn!("inject_dolby_vision: not yet implemented");
    0
}

/// Read HDR metadata from an existing MXF/CPL.
pub fn detect_hdr_type(_input: &Path) -> HdrType {
    tracing::warn!("detect_hdr_type: not yet implemented");
    HdrType::Sdr
}

/// Read HDR10 static metadata from MXF/CPL.
pub fn read_hdr10_metadata(_input: &Path) -> Hdr10Metadata {
    tracing::warn!("read_hdr10_metadata: not yet implemented");
    Hdr10Metadata::default()
}

/// Convert between HDR formats (e.g. HDR10 → HLG tone map).
pub fn convert_hdr(_input: &Path, _target_type: HdrType, _output: &Path) -> i32 {
    tracing::warn!("convert_hdr: not yet implemented");
    0
}
