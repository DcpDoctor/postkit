#include "postkit/certificate.h"
#include <spdlog/spdlog.h>

namespace postkit
{

int generate_certificate(const CertOptions& opts)
{
  spdlog::info("Generating {} certificate: CN={} → {}",
    opts.type == CertType::Root ? "root" :
    opts.type == CertType::Intermediate ? "intermediate" :
    opts.type == CertType::Leaf ? "leaf" : "signer",
    opts.common_name, opts.output_cert.string());
  return 0;
}

int generate_chain(const std::string& organization,
                   const std::filesystem::path& output_dir)
{
  spdlog::info("Generating certificate chain for '{}' → {}", organization, output_dir.string());
  return 0;
}

CertInfo read_certificate(const std::filesystem::path& cert_path)
{
  spdlog::info("Reading certificate: {}", cert_path.string());
  return {};
}

int validate_chain(const std::vector<std::filesystem::path>& chain)
{
  spdlog::info("Validating certificate chain ({} certs)", chain.size());
  return 0;
}

int add_trusted_device(const std::filesystem::path& cert_path,
                       const std::string& name)
{
  spdlog::info("Adding trusted device: {} ({})", name, cert_path.string());
  return 0;
}

std::vector<TrustedDevice> list_trusted_devices()
{
  spdlog::info("Listing trusted devices");
  return {};
}

int remove_trusted_device(const std::string& thumbprint)
{
  spdlog::info("Removing trusted device: {}", thumbprint);
  return 0;
}

} // namespace postkit
