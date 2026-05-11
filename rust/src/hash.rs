use sha1::Sha1;
use sha2::Digest;
use sha2::Sha256;
use std::io::Read;
use std::path::Path;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;

/// Hash algorithm selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    Sha1,
    Sha256,
}

/// Result of hashing a file.
#[derive(Debug, Clone)]
pub struct HashResult {
    /// Lowercase hex-encoded digest.
    pub hex: String,
    /// Standard base64-encoded digest.
    pub base64: String,
}

const BUF_SIZE: usize = 65536;

/// Compute SHA-1 or SHA-256 hash of a file, returning both hex and base64.
pub fn hash_file(path: &Path, algorithm: HashAlgorithm) -> std::io::Result<HashResult> {
    let mut file = std::fs::File::open(path)?;
    let mut buf = [0u8; BUF_SIZE];

    match algorithm {
        HashAlgorithm::Sha1 => {
            let mut hasher = Sha1::new();
            loop {
                let n = file.read(&mut buf)?;
                if n == 0 {
                    break;
                }
                hasher.update(&buf[..n]);
            }
            let digest = hasher.finalize();
            Ok(HashResult {
                hex: hex_encode(&digest),
                base64: BASE64.encode(digest),
            })
        }
        HashAlgorithm::Sha256 => {
            let mut hasher = Sha256::new();
            loop {
                let n = file.read(&mut buf)?;
                if n == 0 {
                    break;
                }
                hasher.update(&buf[..n]);
            }
            let digest = hasher.finalize();
            Ok(HashResult {
                hex: hex_encode(&digest),
                base64: BASE64.encode(digest),
            })
        }
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn sha1_known_value() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");
        std::fs::write(&path, b"hello world").unwrap();

        let result = hash_file(&path, HashAlgorithm::Sha1).unwrap();
        // SHA-1 of "hello world" = 2aae6c35c94fcfb415dbe95f408b9ce91ee846ed
        assert_eq!(result.hex, "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");
        assert!(!result.base64.is_empty());
    }

    #[test]
    fn sha256_known_value() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");
        std::fs::write(&path, b"hello world").unwrap();

        let result = hash_file(&path, HashAlgorithm::Sha256).unwrap();
        // SHA-256 of "hello world" = b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
        assert_eq!(
            result.hex,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn nonexistent_file() {
        let result = hash_file(Path::new("/nonexistent/file.bin"), HashAlgorithm::Sha1);
        assert!(result.is_err());
    }
}
