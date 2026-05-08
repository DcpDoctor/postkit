#pragma once

#include <cstdint>
#include <filesystem>
#include <string>
#include <vector>

namespace postkit
{

/// Timeline edit decision format.
enum class TimelineFormat
{
  EDL_CMX3600,
  AAF,
  XML_FCP,     // Final Cut Pro XML
  XML_FCPX,    // FCP X XML
  XML_Resolve, // DaVinci Resolve XML
  OTIO,        // OpenTimelineIO
  Unknown
};

/// A single edit event in a timeline.
struct EditEvent
{
  uint32_t event_number = 0;
  std::string reel_name;
  std::string track_type;  // "V", "A1", "A2", etc.
  uint32_t source_in = 0;  // frame number
  uint32_t source_out = 0;
  uint32_t record_in = 0;
  uint32_t record_out = 0;
  std::string transition;  // "CUT", "DISSOLVE"
  std::string comment;
};

/// Parsed timeline.
struct Timeline
{
  std::string title;
  double frame_rate = 24.0;
  TimelineFormat format = TimelineFormat::Unknown;
  std::vector<EditEvent> events;
};

/// Conform options.
struct ConformOptions
{
  std::filesystem::path timeline_file;       // EDL/AAF/XML path
  std::filesystem::path media_dir;           // directory containing source reels
  std::filesystem::path output_dir;          // assembled output
  bool auto_detect_format = true;
  TimelineFormat force_format = TimelineFormat::Unknown;
  double frame_rate = 24.0;
};

/// Parse a timeline file (EDL, AAF, XML).
Timeline parse_timeline(const std::filesystem::path& file);

/// Detect timeline format from file extension/content.
TimelineFormat detect_timeline_format(const std::filesystem::path& file);

/// Conform/assemble media from a timeline into reel structure.
int conform(const ConformOptions& opts);

/// Verify that all source reels referenced in timeline exist in media_dir.
std::vector<std::string> find_missing_reels(const Timeline& timeline,
                                            const std::filesystem::path& media_dir);

} // namespace postkit
