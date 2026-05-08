#pragma once

#include <cstdint>
#include <filesystem>
#include <functional>
#include <string>
#include <vector>

namespace postkit
{

/// Preferences schema migration.
/// Each migration upgrades from (version - 1) to (version).
struct PrefsMigration
{
  uint32_t version;                    // Target version after this migration
  std::string description;             // e.g. "Add gpu_device field"
  /// Mutate the raw JSON string in-place to apply the migration.
  /// The JSON is the full file contents. Add missing keys with defaults.
  std::function<std::string(std::string const&)> apply;
};

/// Read the "version" field from a JSON preferences string.
/// Returns 0 if the field is missing (pre-versioning config).
uint32_t prefs_version(std::string const& json);

/// Set the "version" field in a JSON preferences string.
std::string prefs_set_version(std::string const& json, uint32_t version);

/// Run all applicable migrations on a JSON preferences string.
/// Applies migrations where migration.version > current file version,
/// in ascending order. Returns the migrated JSON (with updated version).
std::string migrate_preferences(std::string const& json,
                                std::vector<PrefsMigration> const& migrations);

/// Insert a key-value pair into a JSON object string if the key doesn't exist.
/// value should be a valid JSON literal (quoted string, number, bool, etc.).
std::string json_insert_if_missing(std::string const& json,
                                   std::string const& key,
                                   std::string const& value);

/// Get the platform-specific config directory for an app.
/// Linux: $XDG_CONFIG_HOME/<app> or ~/.config/<app>
/// macOS: ~/Library/Application Support/<app>
/// Windows: %APPDATA%/<app>
std::filesystem::path config_dir(std::string const& app_name);

} // namespace postkit
