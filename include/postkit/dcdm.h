#pragma once

#include <cstdint>
#include <filesystem>
#include <string>
#include <vector>

namespace postkit
{

/// DCDM colour encoding.
enum class DcdmColourEncoding
{
  XYZ_12bit,      // X'Y'Z' 12-bit (SMPTE 428-1)
  XYZ_16bit       // X'Y'Z' 16-bit
};

/// DCDM creation options.
struct DcdmOptions
{
  std::filesystem::path input_dir;     // Source image sequence (DPX/TIFF/EXR)
  std::filesystem::path output_dir;    // Output DCDM TIFF sequence
  DcdmColourEncoding encoding = DcdmColourEncoding::XYZ_12bit;
  uint32_t width = 4096;
  uint32_t height = 2160;
  uint32_t fps_num = 24;
  uint32_t fps_den = 1;
  std::string colour_space;            // Source colour space for conversion
  std::filesystem::path lut_path;      // Optional 3D LUT for colour transform
};

/// Result of DCDM operation.
struct DcdmResult
{
  bool success = false;
  std::string error;
  uint64_t frames_written = 0;
  std::filesystem::path output_dir;
};

/// Create DCDM (Digital Cinema Distribution Master) from source images.
DcdmResult create_dcdm(DcdmOptions const& opts);

/// Convert DCDM back to viewable format (e.g. for review).
DcdmResult export_dcdm(std::filesystem::path const& dcdm_dir,
                       std::filesystem::path const& output_dir,
                       std::string const& target_colour_space = "rec709");

} // namespace postkit
