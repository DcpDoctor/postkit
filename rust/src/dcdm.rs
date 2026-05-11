use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// DCDM colour encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DcdmColourEncoding {
    /// X'Y'Z' 12-bit (SMPTE 428-1)
    Xyz12Bit,
    /// X'Y'Z' 16-bit
    Xyz16Bit,
}

/// DCDM creation options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcdmOptions {
    /// Source image sequence (DPX/TIFF/EXR)
    pub input_dir: PathBuf,
    /// Output DCDM TIFF sequence
    pub output_dir: PathBuf,
    pub encoding: DcdmColourEncoding,
    pub width: u32,
    pub height: u32,
    pub fps_num: u32,
    pub fps_den: u32,
    /// Source colour space for conversion
    pub colour_space: String,
    /// Optional 3D LUT for colour transform
    pub lut_path: PathBuf,
}

impl Default for DcdmOptions {
    fn default() -> Self {
        Self {
            input_dir: PathBuf::new(),
            output_dir: PathBuf::new(),
            encoding: DcdmColourEncoding::Xyz12Bit,
            width: 4096,
            height: 2160,
            fps_num: 24,
            fps_den: 1,
            colour_space: String::new(),
            lut_path: PathBuf::new(),
        }
    }
}

/// Result of DCDM operation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DcdmResult {
    pub success: bool,
    pub error: String,
    pub frames_written: u64,
    pub output_dir: PathBuf,
}

/// Create DCDM (Digital Cinema Distribution Master) from source images.
pub fn create_dcdm(_opts: &DcdmOptions) -> DcdmResult {
    tracing::warn!("create_dcdm: not yet implemented");
    DcdmResult::default()
}

/// Convert DCDM back to viewable format (e.g. for review).
pub fn export_dcdm(
    _dcdm_dir: &Path,
    _output_dir: &Path,
    _target_colour_space: Option<&str>,
) -> DcdmResult {
    tracing::warn!("export_dcdm: not yet implemented");
    DcdmResult::default()
}
