#pragma once

#include <cstdint>
#include <filesystem>
#include <string>
#include <vector>

namespace postkit
{

/// Rating system.
enum class RatingSystem
{
  MPAA,           // G, PG, PG-13, R, NC-17
  BBFC,           // U, PG, 12A, 15, 18
  FSK,            // 0, 6, 12, 16, 18
  Custom          // User-provided ratings card image
};

/// Band colour for trailer packaging.
enum class TrailerBand
{
  Green,          // Approved for all audiences
  Red,            // Restricted audiences only
  Yellow          // International / no rating
};

/// Trailer packaging options.
struct TrailerOptions
{
  std::filesystem::path content_dir;   // Main trailer content (J2K or video)
  std::filesystem::path audio_file;    // Audio track
  std::filesystem::path output_dir;    // Output DCP/IMP
  std::string title;                   // Trailer title
  RatingSystem rating_system = RatingSystem::MPAA;
  std::string rating;                  // e.g. "PG-13"
  TrailerBand band = TrailerBand::Green;
  std::filesystem::path custom_card;   // Custom ratings card image (if Custom)
  bool add_countdown = true;           // Add countdown leader
  bool add_tail = true;                // Add tail leader (black + silence)
  uint32_t countdown_seconds = 5;
  uint32_t tail_seconds = 2;
  uint32_t fps_num = 24;
  uint32_t fps_den = 1;
};

/// Result of trailer packaging.
struct TrailerResult
{
  bool success = false;
  std::string error;
  std::filesystem::path output_dir;
  std::string cpl_uuid;
};

/// Package trailer with ratings card, countdown, and tail.
TrailerResult package_trailer(TrailerOptions const& opts);

} // namespace postkit
