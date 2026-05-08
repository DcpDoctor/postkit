#include "postkit/conform.h"
#include <spdlog/spdlog.h>

namespace postkit
{

Timeline parse_timeline(const std::filesystem::path& file)
{
  spdlog::info("Parsing timeline: {}", file.string());
  return {};
}

TimelineFormat detect_timeline_format(const std::filesystem::path& file)
{
  auto ext = file.extension().string();
  if (ext == ".edl") return TimelineFormat::EDL_CMX3600;
  if (ext == ".aaf") return TimelineFormat::AAF;
  if (ext == ".xml" || ext == ".fcpxml") return TimelineFormat::XML_FCPX;
  if (ext == ".otio") return TimelineFormat::OTIO;
  return TimelineFormat::Unknown;
}

int conform(const ConformOptions& opts)
{
  spdlog::info("Conforming {} → {}", opts.timeline_file.string(), opts.output_dir.string());
  return 0;
}

std::vector<std::string> find_missing_reels(const Timeline& timeline,
                                            const std::filesystem::path& media_dir)
{
  spdlog::info("Checking {} events against {}", timeline.events.size(), media_dir.string());
  return {};
}

} // namespace postkit
