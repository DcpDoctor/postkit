#include "postkit/dashboard.h"
#include <spdlog/spdlog.h>

namespace postkit
{

int init_database(const std::filesystem::path& db_path)
{
  spdlog::info("Initializing version database: {}", db_path.string());
  return 0;
}

int register_version(const VersionEntry& entry)
{
  spdlog::info("Registering version: {} ({}) [{}]", entry.title, entry.version_type, entry.territory);
  return 0;
}

std::vector<VersionEntry> list_versions(const std::string& territory,
                                        const std::string& status)
{
  spdlog::info("Listing versions (territory={}, status={})", territory, status);
  return {};
}

std::vector<TerritoryInfo> list_territories()
{
  spdlog::info("Listing territories");
  return {};
}

int update_status(const std::string& uuid, const std::string& new_status)
{
  spdlog::info("Updating {} → {}", uuid, new_status);
  return 0;
}

int export_distribution_matrix(const std::filesystem::path& output_csv)
{
  spdlog::info("Exporting distribution matrix → {}", output_csv.string());
  return 0;
}

int serve_dashboard(const DashboardOptions& opts)
{
  spdlog::info("Starting dashboard on {}:{}", opts.bind_address, opts.http_port);
  return 0;
}

} // namespace postkit
