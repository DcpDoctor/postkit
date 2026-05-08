#pragma once

#include <cstdint>
#include <filesystem>
#include <string>
#include <vector>

namespace postkit
{

/// Dolby Vision profile.
enum class DolbyVisionProfile
{
  Profile5,   // MEL (Minimum Enhancement Layer) — single-layer PQ
  Profile8,   // HLG backward compatible
  Profile81,  // SDR backward compatible (most common for cinema)
  Unknown
};

/// HDR metadata type.
enum class HdrType
{
  SDR,
  HDR10,       // Static PQ metadata (SMPTE ST 2086 + CTA 861.3)
  HDR10Plus,   // Dynamic PQ metadata (Samsung)
  DolbyVision, // Dolby Vision RPU
  HLG,         // Hybrid Log-Gamma
  ACES         // Academy Color Encoding System
};

/// Static HDR10 metadata (mastering display + content light level).
struct Hdr10Metadata
{
  // Mastering display colour volume (SMPTE ST 2086)
  uint16_t display_primaries_gx = 0;
  uint16_t display_primaries_gy = 0;
  uint16_t display_primaries_bx = 0;
  uint16_t display_primaries_by = 0;
  uint16_t display_primaries_rx = 0;
  uint16_t display_primaries_ry = 0;
  uint16_t white_point_x = 0;
  uint16_t white_point_y = 0;
  uint32_t max_luminance = 0;     // cd/m² × 10000
  uint32_t min_luminance = 0;     // cd/m² × 10000

  // Content light level (CTA 861.3)
  uint16_t max_cll = 0;   // MaxCLL
  uint16_t max_fall = 0;  // MaxFALL
};

/// Dolby Vision RPU (Reference Processing Unit) options.
struct DolbyVisionOptions
{
  std::filesystem::path input;          // source video/image sequence
  std::filesystem::path rpu_file;       // .bin RPU file or XML metadata
  DolbyVisionProfile profile = DolbyVisionProfile::Profile81;
  std::filesystem::path output;
  bool embed_rpu = true;                // embed RPU in output MXF
};

/// HDR metadata injection options.
struct HdrMetadataOptions
{
  std::filesystem::path input;
  HdrType type = HdrType::SDR;
  Hdr10Metadata hdr10;
  std::filesystem::path dolby_vision_xml;  // Dolby Vision metadata XML
  std::filesystem::path output;
};

/// Inject HDR10 static metadata into MXF/CPL.
int inject_hdr10_metadata(const HdrMetadataOptions& opts);

/// Inject Dolby Vision RPU into MXF track.
int inject_dolby_vision(const DolbyVisionOptions& opts);

/// Read HDR metadata from an existing MXF/CPL.
HdrType detect_hdr_type(const std::filesystem::path& input);
Hdr10Metadata read_hdr10_metadata(const std::filesystem::path& input);

/// Convert between HDR formats (e.g., HDR10 → HLG tone map).
int convert_hdr(const std::filesystem::path& input,
                HdrType target_type,
                const std::filesystem::path& output);

} // namespace postkit
