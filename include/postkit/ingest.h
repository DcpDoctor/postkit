#pragma once

#include <cstdint>
#include <filesystem>
#include <string>
#include <vector>

namespace postkit
{

/// Camera raw format identifiers.
enum class CameraFormat
{
  ARRIRAW,
  RED_R3D,
  Sony_RAW,
  Canon_RAW,
  Blackmagic_BRAW,
  ProRes,
  DNxHR,
  Unknown
};

/// Ingest options for camera media.
struct IngestOptions
{
  std::filesystem::path source;       // camera card/media directory
  std::filesystem::path output_dir;   // destination for transcoded media
  std::string output_format = "dpx";  // "dpx", "tiff", "exr", "prores"
  std::string colour_space = "ACES";  // "ACES", "Rec.709", "P3", "LogC"
  uint32_t debayer_quality = 3;       // 1=fast, 3=high quality
  bool apply_lut = false;
  std::filesystem::path lut_path;
  int gpu_device = -1;
};

/// Detected camera clip metadata.
struct ClipInfo
{
  std::filesystem::path path;
  CameraFormat format = CameraFormat::Unknown;
  uint32_t width = 0;
  uint32_t height = 0;
  double frame_rate = 0.0;
  uint32_t frame_count = 0;
  std::string codec;
  std::string colour_space;
  std::string camera_model;
  std::string reel_name;
};

/// Detect camera format from directory/file.
CameraFormat detect_format(const std::filesystem::path& source);

/// Scan a camera card and return clip info.
std::vector<ClipInfo> scan_media(const std::filesystem::path& source);

/// Ingest/transcode camera media to standardized intermediate.
int ingest(const IngestOptions& opts);

} // namespace postkit
