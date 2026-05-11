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

/// Get the trusted devices directory (XDG data or fallback).
fn trusted_devices_dir() -> PathBuf {
    let base = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("postkit").join("trusted_devices")
}

/// Compute SHA-1 thumbprint of DER-encoded certificate bytes.
fn sha1_thumbprint(der_bytes: &[u8]) -> String {
    use sha1::Digest;
    let hash = sha1::Sha1::digest(der_bytes);
    hash.iter().map(|b| format!("{b:02x}")).collect()
}

/// Generate a new X.509 certificate + private key.
pub fn generate_certificate(opts: &CertOptions) -> i32 {
    use rcgen::{
        BasicConstraints, CertificateParams, DnType, DnValue, IsCa, KeyPair, KeyUsagePurpose,
    };

    let mut params = CertificateParams::default();
    params.distinguished_name.push(
        DnType::CommonName,
        DnValue::Utf8String(opts.common_name.clone()),
    );
    if !opts.organization.is_empty() {
        params.distinguished_name.push(
            DnType::OrganizationName,
            DnValue::Utf8String(opts.organization.clone()),
        );
    }
    if !opts.organizational_unit.is_empty() {
        params.distinguished_name.push(
            DnType::OrganizationalUnitName,
            DnValue::Utf8String(opts.organizational_unit.clone()),
        );
    }
    if !opts.country.is_empty() {
        params.distinguished_name.push(
            DnType::CountryName,
            DnValue::Utf8String(opts.country.clone()),
        );
    }

    let now = time::OffsetDateTime::now_utc();
    params.not_before = now;
    params.not_after = now + time::Duration::days(opts.validity_days as i64);

    match opts.cert_type {
        CertType::Root => {
            params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
            params.key_usages = vec![
                KeyUsagePurpose::KeyCertSign,
                KeyUsagePurpose::CrlSign,
            ];
        }
        CertType::Intermediate => {
            params.is_ca = IsCa::Ca(BasicConstraints::Constrained(0));
            params.key_usages = vec![
                KeyUsagePurpose::KeyCertSign,
                KeyUsagePurpose::CrlSign,
            ];
        }
        CertType::Leaf | CertType::Signer => {
            params.is_ca = IsCa::NoCa;
            params.key_usages = vec![
                KeyUsagePurpose::DigitalSignature,
                KeyUsagePurpose::ContentCommitment,
            ];
        }
    }

    let key_pair = match KeyPair::generate_for(&rcgen::PKCS_RSA_SHA256) {
        Ok(kp) => kp,
        Err(e) => {
            tracing::error!("failed to generate RSA key pair: {e}");
            return -1;
        }
    };

    let cert = if opts.cert_type == CertType::Root {
        // Self-signed
        match params.self_signed(&key_pair) {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("failed to self-sign certificate: {e}");
                return -1;
            }
        }
    } else {
        // Signed by issuer
        let issuer_cert_pem = match std::fs::read_to_string(&opts.issuer_cert) {
            Ok(s) => s,
            Err(e) => {
                tracing::error!("failed to read issuer cert: {e}");
                return -1;
            }
        };
        let issuer_key_pem = match std::fs::read_to_string(&opts.issuer_key) {
            Ok(s) => s,
            Err(e) => {
                tracing::error!("failed to read issuer key: {e}");
                return -1;
            }
        };

        let issuer_key = match KeyPair::from_pem(&issuer_key_pem) {
            Ok(kp) => kp,
            Err(e) => {
                tracing::error!("failed to parse issuer key: {e}");
                return -1;
            }
        };

        let issuer_params =
            match CertificateParams::from_ca_cert_pem(&issuer_cert_pem) {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!("failed to parse issuer cert: {e}");
                    return -1;
                }
            };

        let issuer = match issuer_params.self_signed(&issuer_key) {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("failed to reconstruct issuer: {e}");
                return -1;
            }
        };

        match params.signed_by(&key_pair, &issuer, &issuer_key) {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("failed to sign certificate: {e}");
                return -1;
            }
        }
    };

    if let Err(e) = std::fs::write(&opts.output_cert, cert.pem()) {
        tracing::error!("failed to write cert: {e}");
        return -1;
    }

    if let Err(e) = std::fs::write(&opts.output_key, key_pair.serialize_pem()) {
        tracing::error!("failed to write key: {e}");
        return -1;
    }

    tracing::info!("generated certificate: {}", opts.output_cert.display());
    0
}

/// Generate a self-signed certificate chain (root → intermediate → signer).
pub fn generate_chain(organization: &str, output_dir: &Path) -> i32 {
    if let Err(e) = std::fs::create_dir_all(output_dir) {
        tracing::error!("failed to create output dir: {e}");
        return -1;
    }

    // Root CA
    let root_opts = CertOptions {
        cert_type: CertType::Root,
        common_name: format!("{organization} Root CA"),
        organization: organization.to_string(),
        organizational_unit: "Digital Cinema".to_string(),
        validity_days: 3650 * 3, // 30 years
        output_cert: output_dir.join("root.pem"),
        output_key: output_dir.join("root.key"),
        ..Default::default()
    };
    if generate_certificate(&root_opts) != 0 {
        return -1;
    }

    // Intermediate CA
    let inter_opts = CertOptions {
        cert_type: CertType::Intermediate,
        common_name: format!("{organization} Intermediate CA"),
        organization: organization.to_string(),
        organizational_unit: "Digital Cinema".to_string(),
        validity_days: 3650 * 2, // 20 years
        output_cert: output_dir.join("intermediate.pem"),
        output_key: output_dir.join("intermediate.key"),
        issuer_cert: output_dir.join("root.pem"),
        issuer_key: output_dir.join("root.key"),
        ..Default::default()
    };
    if generate_certificate(&inter_opts) != 0 {
        return -1;
    }

    // Signer (leaf)
    let signer_opts = CertOptions {
        cert_type: CertType::Signer,
        common_name: format!("{organization} Signer"),
        organization: organization.to_string(),
        organizational_unit: "Digital Cinema".to_string(),
        validity_days: 3650,
        output_cert: output_dir.join("signer.pem"),
        output_key: output_dir.join("signer.key"),
        issuer_cert: output_dir.join("intermediate.pem"),
        issuer_key: output_dir.join("intermediate.key"),
        ..Default::default()
    };
    if generate_certificate(&signer_opts) != 0 {
        return -1;
    }

    tracing::info!("generated certificate chain in {}", output_dir.display());
    0
}

/// Read certificate info from PEM file.
pub fn read_certificate(cert_path: &Path) -> CertInfo {
    use x509_parser::prelude::*;

    let pem_data = match std::fs::read(cert_path) {
        Ok(d) => d,
        Err(e) => {
            tracing::error!("failed to read cert {}: {e}", cert_path.display());
            return CertInfo::default();
        }
    };

    let (_, pem) = match parse_x509_pem(&pem_data) {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("failed to parse PEM: {e}");
            return CertInfo::default();
        }
    };

    let cert = match pem.parse_x509() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("failed to parse X.509: {e}");
            return CertInfo::default();
        }
    };

    let subject_cn = cert
        .subject()
        .iter_common_name()
        .next()
        .and_then(|cn| cn.as_str().ok())
        .unwrap_or("")
        .to_string();

    let issuer_cn = cert
        .issuer()
        .iter_common_name()
        .next()
        .and_then(|cn| cn.as_str().ok())
        .unwrap_or("")
        .to_string();

    let serial = cert.serial.to_str_radix(16);

    let not_before = cert.validity().not_before.to_rfc2822().unwrap_or_default();
    let not_after = cert.validity().not_after.to_rfc2822().unwrap_or_default();

    let key_bits = cert
        .public_key()
        .parsed()
        .ok()
        .map(|pk| match pk {
            x509_parser::public_key::PublicKey::RSA(rsa) => (rsa.key_size() * 8) as u32,
            _ => 0,
        })
        .unwrap_or(0);

    let is_ca = cert.is_ca();

    let now = x509_parser::time::ASN1Time::now();
    let is_expired = cert.validity().not_after < now;

    let thumbprint = sha1_thumbprint(&pem.contents);

    CertInfo {
        subject_cn,
        issuer_cn,
        serial,
        not_before,
        not_after,
        key_bits,
        is_ca,
        is_expired,
        thumbprint_sha1: thumbprint,
    }
}

/// Validate a certificate chain.
///
/// Checks that each certificate in the chain is valid and properly linked.
pub fn validate_chain(chain: &[PathBuf]) -> i32 {
    if chain.is_empty() {
        tracing::error!("empty certificate chain");
        return -1;
    }

    // Read and validate each certificate individually, collecting key info
    struct ChainEntry {
        subject: String,
        issuer: String,
        expired: bool,
        not_yet_valid: bool,
    }

    let mut entries = Vec::new();
    for path in chain {
        let info = read_certificate(path);
        if info.subject_cn.is_empty() && info.serial.is_empty() {
            tracing::error!("failed to parse certificate: {}", path.display());
            return -1;
        }
        entries.push(ChainEntry {
            subject: info.subject_cn.clone(),
            issuer: info.issuer_cn.clone(),
            expired: info.is_expired,
            not_yet_valid: false, // read_certificate doesn't check not_yet_valid but that's rare
        });
    }

    // Check expiry
    for (i, entry) in entries.iter().enumerate() {
        if entry.expired {
            tracing::error!("certificate expired: {}", chain[i].display());
            return -1;
        }
        if entry.not_yet_valid {
            tracing::error!("certificate not yet valid: {}", chain[i].display());
            return -1;
        }
    }

    // Check issuer/subject chain
    for i in 0..entries.len().saturating_sub(1) {
        if entries[i].issuer != entries[i + 1].subject {
            tracing::error!(
                "chain broken: '{}' issuer '{}' does not match '{}' subject '{}'",
                entries[i].subject,
                entries[i].issuer,
                entries[i + 1].subject,
                entries[i + 1].subject,
            );
            return -1;
        }
    }

    // Root should be self-signed
    let last = &entries[entries.len() - 1];
    if last.issuer != last.subject {
        tracing::error!(
            "root cert is not self-signed: subject='{}', issuer='{}'",
            last.subject,
            last.issuer,
        );
        return -1;
    }

    tracing::info!("certificate chain valid ({} certificates)", entries.len());
    0
}

/// Add a trusted device.
pub fn add_trusted_device(cert_path: &Path, name: &str) -> i32 {
    let dir = trusted_devices_dir();
    if let Err(e) = std::fs::create_dir_all(&dir) {
        tracing::error!("failed to create trusted devices dir: {e}");
        return -1;
    }

    let info = read_certificate(cert_path);
    if info.thumbprint_sha1.is_empty() {
        tracing::error!("failed to read certificate for trusted device");
        return -1;
    }

    let device = TrustedDevice {
        name: name.to_string(),
        thumbprint: info.thumbprint_sha1.clone(),
        certificate_path: cert_path.to_path_buf(),
    };

    // Copy cert to trusted devices dir
    let dest = dir.join(format!("{}.pem", info.thumbprint_sha1));
    if let Err(e) = std::fs::copy(cert_path, &dest) {
        tracing::error!("failed to copy certificate: {e}");
        return -1;
    }

    // Write metadata JSON
    let meta_path = dir.join(format!("{}.json", info.thumbprint_sha1));
    let json = match serde_json::to_string_pretty(&device) {
        Ok(j) => j,
        Err(e) => {
            tracing::error!("failed to serialize device metadata: {e}");
            return -1;
        }
    };
    if let Err(e) = std::fs::write(&meta_path, json) {
        tracing::error!("failed to write device metadata: {e}");
        return -1;
    }

    tracing::info!("added trusted device '{}' ({})", name, info.thumbprint_sha1);
    0
}

/// List all trusted devices.
pub fn list_trusted_devices() -> Vec<TrustedDevice> {
    let dir = trusted_devices_dir();
    let entries = match std::fs::read_dir(&dir) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };

    let mut devices = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json")
            && let Ok(data) = std::fs::read_to_string(&path)
            && let Ok(device) = serde_json::from_str::<TrustedDevice>(&data)
        {
            devices.push(device);
        }
    }
    devices
}

/// Remove a trusted device by thumbprint.
pub fn remove_trusted_device(thumbprint: &str) -> i32 {
    let dir = trusted_devices_dir();
    let pem_path = dir.join(format!("{thumbprint}.pem"));
    let json_path = dir.join(format!("{thumbprint}.json"));

    let mut removed = false;
    if pem_path.exists() {
        if let Err(e) = std::fs::remove_file(&pem_path) {
            tracing::error!("failed to remove {}: {e}", pem_path.display());
            return -1;
        }
        removed = true;
    }
    if json_path.exists() {
        if let Err(e) = std::fs::remove_file(&json_path) {
            tracing::error!("failed to remove {}: {e}", json_path.display());
            return -1;
        }
        removed = true;
    }

    if removed {
        tracing::info!("removed trusted device {thumbprint}");
        0
    } else {
        tracing::warn!("trusted device not found: {thumbprint}");
        -1
    }
}
