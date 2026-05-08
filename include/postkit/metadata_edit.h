#pragma once

#include <cstdint>
#include <filesystem>
#include <string>
#include <vector>

namespace postkit
{

/// Editable metadata field.
struct MetadataField
{
  std::string key;
  std::string value;
  std::string type;  // "string", "uuid", "datetime", "integer", "rational"
  bool readonly = false;
};

/// Metadata for a CPL or OPL.
struct CompositionMetadata
{
  std::string uuid;
  std::string title;
  std::string annotation;
  std::string issuer;
  std::string creator;
  std::string issue_date;
  std::string content_kind;       // "feature", "trailer", "advertisement", etc.
  std::string rating;
  std::vector<MetadataField> custom_fields;
};

/// Read metadata from a CPL/OPL XML file.
CompositionMetadata read_metadata(const std::filesystem::path& cpl_path);

/// Write updated metadata back to CPL/OPL XML (non-destructive — preserves structure).
int write_metadata(const std::filesystem::path& cpl_path,
                   const CompositionMetadata& meta);

/// Batch update a field across multiple CPLs.
int batch_update_field(const std::vector<std::filesystem::path>& cpls,
                       const std::string& field_key,
                       const std::string& new_value);

/// List all editable fields in a CPL.
std::vector<MetadataField> list_fields(const std::filesystem::path& cpl_path);

} // namespace postkit
