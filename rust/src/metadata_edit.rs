use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Editable metadata field.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetadataField {
    pub key: String,
    pub value: String,
    /// "string", "uuid", "datetime", "integer", "rational"
    pub field_type: String,
    pub readonly: bool,
}

/// Metadata for a CPL or OPL.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompositionMetadata {
    pub uuid: String,
    pub title: String,
    pub annotation: String,
    pub issuer: String,
    pub creator: String,
    pub issue_date: String,
    /// "feature", "trailer", "advertisement", etc.
    pub content_kind: String,
    pub rating: String,
    pub custom_fields: Vec<MetadataField>,
}

/// Read metadata from a CPL/OPL XML file.
pub fn read_metadata(_cpl_path: &Path) -> CompositionMetadata {
    tracing::warn!("read_metadata: not yet implemented");
    CompositionMetadata::default()
}

/// Write updated metadata back to CPL/OPL XML (non-destructive — preserves structure).
pub fn write_metadata(_cpl_path: &Path, _meta: &CompositionMetadata) -> i32 {
    tracing::warn!("write_metadata: not yet implemented");
    0
}

/// Batch update a field across multiple CPLs.
pub fn batch_update_field(_cpls: &[PathBuf], _field_key: &str, _new_value: &str) -> i32 {
    tracing::warn!("batch_update_field: not yet implemented");
    0
}

/// List all editable fields in a CPL.
pub fn list_fields(_cpl_path: &Path) -> Vec<MetadataField> {
    tracing::warn!("list_fields: not yet implemented");
    Vec::new()
}
