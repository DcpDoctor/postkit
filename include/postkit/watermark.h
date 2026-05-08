#pragma once

#include <cstdint>
#include <filesystem>
#include <string>
#include <vector>

namespace postkit
{

/// Watermark technology backend.
enum class WatermarkBackend
{
  NexGuard,       // NexGuard (Nagra) forensic watermarking
  Civolution,     // Civolution/Kantar watermarking
  Internal        // Lightweight internal spatial watermark
};

/// Forensic watermark options.
struct WatermarkOptions
{
  WatermarkBackend backend = WatermarkBackend::Internal;
  std::string operator_id;             // Unique operator/recipient identifier
  std::string session_id;              // Session or transaction identifier
  uint32_t strength = 50;              // Embedding strength (0-100)
  bool per_frame = true;               // Per-frame vs per-segment embedding
  std::filesystem::path input_dir;     // Input image sequence
  std::filesystem::path output_dir;    // Watermarked output
  std::filesystem::path license_file;  // NexGuard/Civolution license
};

/// Result of watermark embedding.
struct WatermarkResult
{
  bool success = false;
  std::string error;
  uint64_t frames_processed = 0;
  std::string payload_hash;            // Hash of embedded payload for audit
};

/// Embed forensic watermark into image sequence.
WatermarkResult embed_watermark(WatermarkOptions const& opts);

/// Detect/extract forensic watermark from image sequence.
WatermarkResult detect_watermark(std::filesystem::path const& input_dir,
                                 WatermarkBackend backend,
                                 std::filesystem::path const& license_file = {});

} // namespace postkit
