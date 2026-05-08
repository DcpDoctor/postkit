#pragma once

#include <cstdint>
#include <filesystem>
#include <string>
#include <vector>

namespace postkit
{

/// Accessibility standard to check against.
enum class AccessibilityStandard
{
  CVAA,           // US: 21st Century Communications and Video Accessibility Act
  EAA,            // EU: European Accessibility Act (2025)
  AODA,           // Canada: Accessibility for Ontarians with Disabilities Act
  Ofcom           // UK: Ofcom broadcasting accessibility code
};

/// Accessibility track type.
enum class AccessibilityTrack
{
  AudioDescription,       // AD — visually impaired narration
  HearingImpaired,        // HI — SDH/CC subtitles for deaf/hard of hearing
  SignLanguage,           // SL — sign language video overlay
  OpenCaptions,          // OC — burned-in captions
  ClosedCaptions,        // CC — CEA-608/708 caption stream
  Commentary             // Director/audio commentary
};

/// Single compliance finding.
struct AccessibilityFinding
{
  enum class Severity { Error, Warning, Info };

  Severity severity = Severity::Warning;
  AccessibilityTrack track_type;
  std::string rule_id;                 // e.g. "CVAA-3.1", "EAA-4.2"
  std::string description;
  std::string recommendation;
};

/// Result of accessibility compliance check.
struct AccessibilityResult
{
  bool compliant = false;
  AccessibilityStandard standard;
  std::vector<AccessibilityFinding> findings;
  uint32_t errors = 0;
  uint32_t warnings = 0;
  std::vector<AccessibilityTrack> tracks_present;
  std::vector<AccessibilityTrack> tracks_missing;
};

/// Check accessibility compliance of a DCP or IMP.
AccessibilityResult check_accessibility(std::filesystem::path const& package_dir,
                                        AccessibilityStandard standard = AccessibilityStandard::CVAA);

/// Check accessibility compliance against multiple standards.
std::vector<AccessibilityResult> check_accessibility_multi(
    std::filesystem::path const& package_dir,
    std::vector<AccessibilityStandard> const& standards);

} // namespace postkit
