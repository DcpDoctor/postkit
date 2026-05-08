#pragma once

#include <cstdint>
#include <filesystem>
#include <string>
#include <vector>

namespace postkit
{

/// A version entry in the OV/VF management system.
struct VersionEntry
{
  std::string uuid;
  std::string title;
  std::string version_type;  // "OV", "VF"
  std::string territory;     // ISO 3166-1 alpha-2 (e.g., "US", "GB", "FR")
  std::string language;      // RFC 5646
  std::string standard;      // "SMPTE" or "Interop"
  std::filesystem::path dcp_path;
  std::string ov_uuid;       // for VFs: the referenced OV UUID
  std::string created_date;
  std::string status;        // "draft", "released", "archived"
  std::vector<std::string> kdm_recipients;  // theater names
};

/// Territory distribution info.
struct TerritoryInfo
{
  std::string code;    // "US", "GB", etc.
  std::string name;    // "United States", "United Kingdom"
  uint32_t version_count = 0;
  std::vector<std::string> languages;
};

/// Dashboard database options.
struct DashboardOptions
{
  std::filesystem::path database_path;  // SQLite DB for version tracking
  uint32_t http_port = 9090;
  std::string bind_address = "127.0.0.1";
};

/// Initialize the version management database.
int init_database(const std::filesystem::path& db_path);

/// Register a new DCP version (OV or VF).
int register_version(const VersionEntry& entry);

/// List all versions, optionally filtered.
std::vector<VersionEntry> list_versions(const std::string& territory = "",
                                        const std::string& status = "");

/// List territories with version counts.
std::vector<TerritoryInfo> list_territories();

/// Update version status (draft → released → archived).
int update_status(const std::string& uuid, const std::string& new_status);

/// Generate a distribution matrix (territory × version grid).
int export_distribution_matrix(const std::filesystem::path& output_csv);

/// Start the web dashboard (blocking — for standalone use).
int serve_dashboard(const DashboardOptions& opts);

} // namespace postkit
