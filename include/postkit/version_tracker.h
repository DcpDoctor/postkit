#pragma once

#include <cstdint>
#include <filesystem>
#include <string>
#include <vector>

namespace postkit
{

/// Delivery record — tracks which version went where.
struct DeliveryRecord
{
  std::string package_uuid;            // CPL UUID or IMP UUID
  std::string title;
  std::string version;                 // e.g. "OV", "VF_v2", "Trailer_v3"
  std::string destination;             // Venue, platform, or distributor
  std::string delivery_method;         // "drive", "s3", "aspera", "ftp"
  std::string timestamp;               // ISO 8601 delivery time
  std::string operator_name;
  std::string notes;
  bool verified = false;               // Post-delivery hash verification passed
};

/// Query filters for version history.
struct VersionQuery
{
  std::string package_uuid;            // Filter by package
  std::string title;                   // Filter by title (substring)
  std::string destination;             // Filter by destination
  std::string after;                   // ISO 8601 date lower bound
  std::string before;                  // ISO 8601 date upper bound
};

/// Version tracker database interface.
struct VersionTracker
{
  std::filesystem::path db_path;       // SQLite database location

  /// Initialize or open the version database.
  bool open(std::filesystem::path const& path);

  /// Record a delivery.
  bool record(DeliveryRecord const& record);

  /// Query delivery history.
  std::vector<DeliveryRecord> query(VersionQuery const& filter = {});

  /// Get all versions of a specific package.
  std::vector<DeliveryRecord> versions_of(std::string const& package_uuid);

  /// Get delivery history for a destination.
  std::vector<DeliveryRecord> deliveries_to(std::string const& destination);

  /// Export delivery history as JSON.
  std::string export_json(VersionQuery const& filter = {});

  /// Export delivery history as CSV.
  std::string export_csv(VersionQuery const& filter = {});
};

} // namespace postkit
