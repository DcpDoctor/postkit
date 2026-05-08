#pragma once

#include <cstdint>
#include <filesystem>
#include <string>

namespace postkit
{

/// Frame-accurate media player/preview for DCP and IMF content.
struct PlaybackOptions
{
  std::filesystem::path input;
  std::string cpl_uuid;
  uint32_t start_frame = 0;
  uint32_t end_frame = 0;  // 0 = play to end
  bool loop = false;
  bool decode_to_display = true;
  std::string display_colourspace = "sRGB";
  int gpu_device = -1;
};

struct FrameInfo
{
  uint32_t frame_number = 0;
  uint32_t width = 0;
  uint32_t height = 0;
  uint32_t bitrate_kbps = 0;
  std::string codec;
};

/// Extract a single frame as image (thumbnail/QC).
int extract_frame(const std::filesystem::path& input, uint32_t frame,
                  const std::filesystem::path& output_image);

/// Get frame metadata without full decode.
FrameInfo get_frame_info(const std::filesystem::path& input, uint32_t frame);

/// Start playback (blocking).
int play(const PlaybackOptions& opts);

/// Render all frames to image sequence.
int render_to_sequence(const std::filesystem::path& input,
                       const std::filesystem::path& output_dir,
                       const std::string& format = "png");

} // namespace postkit
