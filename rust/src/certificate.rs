use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Certificate type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CertType {
    /// Self-signed root CA
    Root,
    /// Intermediate CA
    Intermediate,
    /// End-entity (screen/projector)
    Leaf,
    /// Content signer
    Signer,
}

/// Certificate generation options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertOptions {
    pub cert_type: CertType,
    pub common_name: String,
    pub organization: String,
    pub organizational_unit: String,
    pub country: String,
    /// RSA key size
    pub key_bits: u32,
    /// Validity in days (default 10 years)
    pub validity_days: u32,
    pub output_cert: PathBuf,
    pub output_key: PathBuf,
    /// For non-root certs: issuer cert/key
    pub issuer_cert: PathBuf,
    pub issuer_key: PathBuf,
}

impl Default for CertOptions {
    fn default() -> Self {
        Self {
            cert_type: CertType::Signer,
            common_name: String::new(),
            organization: String::new(),
            organizational_unit: String::new(),
            country: "US".to_string(),
            key_bits: 2048,
            validity_days: 3650,
            output_cert: PathBuf::new(),
            output_key: PathBuf::new(),
            issuer_cert: PathBuf::new(),
            issuer_key: PathBuf::new(),
        }
    }
}

/// Certificate info (parsed from PEM/DER).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CertInfo {
    pub subject_cn: String,
    pub issuer_cn: String,
    pub serial: String,
    pub not_before: String,
    pub not_after: String,
    pub key_bits: u32,
    pub is_ca: bool,
    pub is_expired: bool,
    pub thumbprint_sha1: String,
}

/// A trusted device entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedDevice {
    pub name: String,
    pub thumbprint: String,
    pub certificate_path: PathBuf,
}

/// Generate a new X.509 certificate + private key.
pub fn generate_certificate(_opts: &CertOptions) -> i32 {
    tracing::warn!("generate_certificate: not yet implemented");
    0
}

/// Generate a self-signed certificate chain (root → intermediate → signer).
pub fn generate_chain(_organization: &str, _output_dir: &Path) -> i32 {
    tracing::warn!("generate_chain: not yet implemented");
    0
}

/// Read certificate info from PEM file.
pub fn read_certificate(_cert_path: &Path) -> CertInfo {
    tracing::warn!("read_certificate: not yet implemented");
    CertInfo::default()
}

/// Validate a certificate chain.
pub fn validate_chain(_chain: &[PathBuf]) -> i32 {
    tracing::warn!("validate_chain: not yet implemented");
    0
}

/// Add a trusted device.
pub fn add_trusted_device(_cert_path: &Path, _name: &str) -> i32 {
    tracing::warn!("add_trusted_device: not yet implemented");
    0
}

/// List all trusted devices.
pub fn list_trusted_devices() -> Vec<TrustedDevice> {
    tracing::warn!("list_trusted_devices: not yet implemented");
    Vec::new()
}

/// Remove a trusted device by thumbprint.
pub fn remove_trusted_device(_thumbprint: &str) -> i32 {
    tracing::warn!("remove_trusted_device: not yet implemented");
    0
}
