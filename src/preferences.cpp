#include "postkit/preferences.h"

#include <algorithm>
#include <cstdlib>

namespace postkit
{

uint32_t prefs_version(std::string const& json)
{
  auto pos = json.find("\"version\"");
  if (pos == std::string::npos) return 0;
  pos = json.find(':', pos);
  if (pos == std::string::npos) return 0;
  pos++;
  while (pos < json.size() && (json[pos] == ' ' || json[pos] == '\t')) pos++;
  try { return static_cast<uint32_t>(std::stoul(json.substr(pos))); }
  catch (...) { return 0; }
}

std::string prefs_set_version(std::string const& json, uint32_t version)
{
  auto pos = json.find("\"version\"");
  if (pos != std::string::npos)
  {
    auto colon = json.find(':', pos);
    if (colon == std::string::npos) return json;
    colon++;
    while (colon < json.size() && (json[colon] == ' ' || json[colon] == '\t')) colon++;
    auto end = colon;
    while (end < json.size() && json[end] >= '0' && json[end] <= '9') end++;
    return json.substr(0, colon) + std::to_string(version) + json.substr(end);
  }

  auto brace = json.find('{');
  if (brace == std::string::npos) return json;
  return json.substr(0, brace + 1) +
    "\n  \"version\": " + std::to_string(version) + "," +
    json.substr(brace + 1);
}

std::string migrate_preferences(std::string const& json,
                                std::vector<PrefsMigration> const& migrations)
{
  uint32_t current = prefs_version(json);
  std::string result = json;

  auto sorted = migrations;
  std::sort(sorted.begin(), sorted.end(),
            [](auto const& a, auto const& b) { return a.version < b.version; });

  for (auto const& m : sorted)
  {
    if (m.version > current && m.apply)
    {
      result = m.apply(result);
      current = m.version;
    }
  }

  result = prefs_set_version(result, current);
  return result;
}

std::string json_insert_if_missing(std::string const& json,
                                   std::string const& key,
                                   std::string const& value)
{
  std::string search = "\"" + key + "\"";
  if (json.find(search) != std::string::npos) return json;

  auto last_brace = json.rfind('}');
  if (last_brace == std::string::npos) return json;

  auto insert_pos = last_brace;
  while (insert_pos > 0 && (json[insert_pos - 1] == ' ' || json[insert_pos - 1] == '\n' ||
                            json[insert_pos - 1] == '\r' || json[insert_pos - 1] == '\t'))
    insert_pos--;

  std::string prefix;
  if (insert_pos > 0 && json[insert_pos - 1] != '{' && json[insert_pos - 1] != ',')
    prefix = ",";

  std::string insertion = prefix + "\n  \"" + key + "\": " + value;
  return json.substr(0, insert_pos) + insertion + "\n" + json.substr(last_brace);
}

std::filesystem::path config_dir(std::string const& app_name)
{
#ifdef _WIN32
  const char* appdata = std::getenv("APPDATA");
  if (appdata)
    return std::filesystem::path(appdata) / app_name;
  return app_name;
#elif defined(__APPLE__)
  const char* home = std::getenv("HOME");
  if (home)
    return std::filesystem::path(home) / "Library" / "Application Support" / app_name;
  return app_name;
#else
  const char* xdg = std::getenv("XDG_CONFIG_HOME");
  if (xdg)
    return std::filesystem::path(xdg) / app_name;
  const char* home = std::getenv("HOME");
  if (home)
    return std::filesystem::path(home) / ".config" / app_name;
  return app_name;
#endif
}

} // namespace postkit
