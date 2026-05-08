#include "postkit/preview.h"
#include <spdlog/spdlog.h>

namespace postkit
{

int extract_frame(const std::filesystem::path& input, uint32_t frame,
                  const std::filesystem::path& output_image)
{
  spdlog::info("Extracting frame {} from {} → {}", frame, input.string(), output_image.string());
  return 0;
}

FrameInfo get_frame_info(const std::filesystem::path& input, uint32_t frame)
{
  spdlog::info("Getting frame info: {} frame {}", input.string(), frame);
  return {};
}

int play(const PlaybackOptions& opts)
{
  spdlog::info("Playing {}", opts.input.string());
  return 0;
}

int render_to_sequence(const std::filesystem::path& input,
                       const std::filesystem::path& output_dir,
                       const std::string& format)
{
  spdlog::info("Rendering {} → {} ({})", input.string(), output_dir.string(), format);
  return 0;
}

} // namespace postkit
