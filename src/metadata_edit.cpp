#include "postkit/metadata_edit.h"
#include <spdlog/spdlog.h>

namespace postkit
{

CompositionMetadata read_metadata(const std::filesystem::path& cpl_path)
{
  spdlog::info("Reading metadata: {}", cpl_path.string());
  return {};
}

int write_metadata(const std::filesystem::path& cpl_path,
                   const CompositionMetadata& meta)
{
  spdlog::info("Writing metadata: {} title={}", cpl_path.string(), meta.title);
  return 0;
}

int batch_update_field(const std::vector<std::filesystem::path>& cpls,
                       const std::string& field_key,
                       const std::string& new_value)
{
  spdlog::info("Batch updating '{}' = '{}' across {} CPLs", field_key, new_value, cpls.size());
  return 0;
}

std::vector<MetadataField> list_fields(const std::filesystem::path& cpl_path)
{
  spdlog::info("Listing fields: {}", cpl_path.string());
  return {};
}

} // namespace postkit
