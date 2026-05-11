use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// A version entry in the OV/VF management system.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VersionEntry {
    pub uuid: String,
    pub title: String,
    /// "OV" or "VF"
    pub version_type: String,
    /// ISO 3166-1 alpha-2 (e.g. "US", "GB", "FR")
    pub territory: String,
    /// RFC 5646
    pub language: String,
    /// "SMPTE" or "Interop"
    pub standard: String,
    pub dcp_path: PathBuf,
    /// For VFs: the referenced OV UUID
    pub ov_uuid: String,
    pub created_date: String,
    /// "draft", "released", "archived"
    pub status: String,
    /// Theater names
    pub kdm_recipients: Vec<String>,
}

/// Territory distribution info.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TerritoryInfo {
    /// "US", "GB", etc.
    pub code: String,
    /// "United States", "United Kingdom"
    pub name: String,
    pub version_count: u32,
    pub languages: Vec<String>,
}

/// Dashboard database options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardOptions {
    pub database_path: PathBuf,
    pub http_port: u32,
    pub bind_address: String,
}

impl Default for DashboardOptions {
    fn default() -> Self {
        Self {
            database_path: PathBuf::new(),
            http_port: 9090,
            bind_address: "127.0.0.1".to_string(),
        }
    }
}

/// Initialize the version management database.
pub fn init_database(_db_path: &Path) -> i32 {
    tracing::warn!("init_database: not yet implemented");
    0
}

/// Register a new DCP version (OV or VF).
pub fn register_version(_entry: &VersionEntry) -> i32 {
    tracing::warn!("register_version: not yet implemented");
    0
}

/// List all versions, optionally filtered.
pub fn list_versions(_territory: Option<&str>, _status: Option<&str>) -> Vec<VersionEntry> {
    tracing::warn!("list_versions: not yet implemented");
    Vec::new()
}

/// List territories with version counts.
pub fn list_territories() -> Vec<TerritoryInfo> {
    tracing::warn!("list_territories: not yet implemented");
    Vec::new()
}

/// Update version status (draft → released → archived).
pub fn update_status(_uuid: &str, _new_status: &str) -> i32 {
    tracing::warn!("update_status: not yet implemented");
    0
}

/// Generate a distribution matrix (territory × version grid).
pub fn export_distribution_matrix(_output_csv: &Path) -> i32 {
    tracing::warn!("export_distribution_matrix: not yet implemented");
    0
}

/// Start the web dashboard (blocking — for standalone use).
pub fn serve_dashboard(_opts: &DashboardOptions) -> i32 {
    tracing::warn!("serve_dashboard: not yet implemented");
    0
}
