use serde::Deserialize;
use std::path::PathBuf;

/// Preferences schema migration.
/// Each migration upgrades from `(version - 1)` to `version`.
pub struct PrefsMigration {
    /// Target version after this migration.
    pub version: u32,
    /// Human-readable description, e.g. "Add gpu_device field".
    pub description: String,
    /// Mutate the raw JSON string to apply the migration.
    pub apply: Box<dyn Fn(&str) -> String>,
}

/// Read the `"version"` field from a JSON preferences string.
/// Returns 0 if the field is missing (pre-versioning config).
pub fn prefs_version(json: &str) -> u32 {
    #[derive(Deserialize)]
    struct V {
        #[serde(default)]
        version: u32,
    }
    serde_json::from_str::<V>(json)
        .map(|v| v.version)
        .unwrap_or(0)
}

/// Set the `"version"` field in a JSON preferences string.
pub fn prefs_set_version(json: &str, version: u32) -> String {
    let mut val: serde_json::Value = serde_json::from_str(json).unwrap_or_default();
    if let Some(obj) = val.as_object_mut() {
        obj.insert("version".to_string(), serde_json::Value::from(version));
    }
    serde_json::to_string_pretty(&val).unwrap_or_else(|_| json.to_string())
}

/// Run all applicable migrations on a JSON preferences string.
/// Applies migrations where `migration.version > current_version`,
/// in ascending order. Returns the migrated JSON (with updated version).
pub fn migrate_preferences(json: &str, migrations: &[PrefsMigration]) -> String {
    let current = prefs_version(json);
    let mut result = json.to_string();

    let mut sorted: Vec<&PrefsMigration> = migrations.iter().collect();
    sorted.sort_by_key(|m| m.version);

    let mut latest = current;
    for m in sorted {
        if m.version > current {
            result = (m.apply)(&result);
            latest = m.version;
        }
    }

    prefs_set_version(&result, latest)
}

/// Insert a key-value pair into a JSON object string if the key doesn't exist.
/// `value` should be a valid JSON literal (quoted string, number, bool, etc.).
pub fn json_insert_if_missing(json: &str, key: &str, value: &str) -> String {
    let mut val: serde_json::Value = match serde_json::from_str(json) {
        Ok(v) => v,
        Err(_) => return json.to_string(),
    };
    if let Some(obj) = val.as_object_mut() {
        if !obj.contains_key(key) {
            let parsed: serde_json::Value =
                serde_json::from_str(value).unwrap_or(serde_json::Value::Null);
            obj.insert(key.to_string(), parsed);
        }
    }
    serde_json::to_string_pretty(&val).unwrap_or_else(|_| json.to_string())
}

/// Get the platform-specific config directory for an app.
///
/// - Linux: `$XDG_CONFIG_HOME/<app>` or `~/.config/<app>`
/// - macOS: `~/Library/Application Support/<app>`
/// - Windows: `%APPDATA%/<app>`
pub fn config_dir(app_name: &str) -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(app_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_from_json() {
        assert_eq!(prefs_version(r#"{"version": 3, "foo": "bar"}"#), 3);
        assert_eq!(prefs_version(r#"{"foo": "bar"}"#), 0);
        assert_eq!(prefs_version("invalid"), 0);
    }

    #[test]
    fn set_version() {
        let json = r#"{"version": 1, "name": "test"}"#;
        let result = prefs_set_version(json, 5);
        assert_eq!(prefs_version(&result), 5);
        assert!(result.contains("\"name\""));
    }

    #[test]
    fn set_version_missing_field() {
        let json = r#"{"name": "test"}"#;
        let result = prefs_set_version(json, 2);
        assert_eq!(prefs_version(&result), 2);
    }

    #[test]
    fn migrate() {
        let json = r#"{"version": 1, "name": "test"}"#;
        let migrations = vec![
            PrefsMigration {
                version: 2,
                description: "Add colour field".to_string(),
                apply: Box::new(|j| {
                    crate::preferences::json_insert_if_missing(j, "colour", "\"rec709\"")
                }),
            },
            PrefsMigration {
                version: 3,
                description: "Add gpu field".to_string(),
                apply: Box::new(|j| crate::preferences::json_insert_if_missing(j, "gpu", "0")),
            },
        ];
        let result = migrate_preferences(json, &migrations);
        assert_eq!(prefs_version(&result), 3);
        assert!(result.contains("\"colour\""));
        assert!(result.contains("\"gpu\""));
    }

    #[test]
    fn insert_if_missing_adds() {
        let json = r#"{"name": "test"}"#;
        let result = json_insert_if_missing(json, "fps", "24");
        assert!(result.contains("\"fps\""));
        assert!(result.contains("24"));
    }

    #[test]
    fn insert_if_missing_no_overwrite() {
        let json = r#"{"name": "test", "fps": 30}"#;
        let result = json_insert_if_missing(json, "fps", "24");
        assert!(result.contains("30"));
    }

    #[test]
    fn config_dir_nonempty() {
        let dir = config_dir("postkit-test");
        assert!(dir.to_string_lossy().contains("postkit-test"));
    }
}
