#pragma once

#include <cstdint>
#include <filesystem>
#include <string>
#include <vector>

namespace postkit
{

/// Certificate type.
enum class CertType
{
  Root,           // Self-signed root CA
  Intermediate,   // Intermediate CA
  Leaf,           // End-entity (screen/projector)
  Signer          // Content signer
};

/// Certificate generation options.
struct CertOptions
{
  CertType type = CertType::Signer;
  std::string common_name;             // e.g. "My Studio Signer"
  std::string organization;
  std::string organizational_unit;
  std::string country = "US";
  uint32_t key_bits = 2048;            // RSA key size
  uint32_t validity_days = 3650;       // 10 years
  std::filesystem::path output_cert;
  std::filesystem::path output_key;
  // For non-root certs: issuer cert/key
  std::filesystem::path issuer_cert;
  std::filesystem::path issuer_key;
};

/// Certificate info (parsed from PEM/DER).
struct CertInfo
{
  std::string subject_cn;
  std::string issuer_cn;
  std::string serial;
  std::string not_before;
  std::string not_after;
  uint32_t key_bits = 0;
  bool is_ca = false;
  bool is_expired = false;
  std::string thumbprint_sha1;
};

/// A trusted device entry.
struct TrustedDevice
{
  std::string name;
  std::string thumbprint;
  std::filesystem::path certificate_path;
};

/// Generate a new X.509 certificate + private key.
int generate_certificate(const CertOptions& opts);

/// Generate a self-signed certificate chain (root → intermediate → signer).
int generate_chain(const std::string& organization,
                   const std::filesystem::path& output_dir);

/// Read certificate info from PEM file.
CertInfo read_certificate(const std::filesystem::path& cert_path);

/// Validate a certificate chain.
int validate_chain(const std::vector<std::filesystem::path>& chain);

/// Manage trusted device list.
int add_trusted_device(const std::filesystem::path& cert_path,
                       const std::string& name);
std::vector<TrustedDevice> list_trusted_devices();
int remove_trusted_device(const std::string& thumbprint);

} // namespace postkit
