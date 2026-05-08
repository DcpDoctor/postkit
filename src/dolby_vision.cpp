#include "postkit/dolby_vision.h"
#include <spdlog/spdlog.h>

namespace postkit
{

int inject_hdr10_metadata(const HdrMetadataOptions& opts)
{
  spdlog::info("Injecting HDR10 metadata into {}", opts.input.string());
  return 0;
}

int inject_dolby_vision(const DolbyVisionOptions& opts)
{
  spdlog::info("Injecting Dolby Vision RPU ({}) into {}",
    opts.rpu_file.string(), opts.input.string());
  return 0;
}

HdrType detect_hdr_type(const std::filesystem::path& input)
{
  spdlog::info("Detecting HDR type: {}", input.string());
  return HdrType::SDR;
}

Hdr10Metadata read_hdr10_metadata(const std::filesystem::path& input)
{
  spdlog::info("Reading HDR10 metadata: {}", input.string());
  return {};
}

int convert_hdr(const std::filesystem::path& input,
                HdrType target_type,
                const std::filesystem::path& output)
{
  spdlog::info("Converting HDR → {} : {}", static_cast<int>(target_type), output.string());
  return 0;
}

} // namespace postkit
