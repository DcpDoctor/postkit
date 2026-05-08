#include "postkit/ingest.h"
#include <spdlog/spdlog.h>

namespace postkit
{

CameraFormat detect_format(const std::filesystem::path& source)
{
  spdlog::info("Detecting camera format: {}", source.string());
  return CameraFormat::Unknown;
}

std::vector<ClipInfo> scan_media(const std::filesystem::path& source)
{
  spdlog::info("Scanning media: {}", source.string());
  return {};
}

int ingest(const IngestOptions& opts)
{
  spdlog::info("Ingesting {} → {}", opts.source.string(), opts.output_dir.string());
  return 0;
}

} // namespace postkit
