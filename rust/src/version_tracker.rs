use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Delivery record.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeliveryRecord {
    pub package_uuid: String,
    pub title: String,
    pub version: String,
    pub destination: String,
    pub delivery_method: String,
    pub timestamp: String,
    pub verified: bool,
}

/// Query filter for version tracker.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VersionQuery {
    pub package_uuid: Option<String>,
    pub title: Option<String>,
    pub destination: Option<String>,
    pub after: Option<String>,
    pub before: Option<String>,
}

/// Content versioning / delivery history tracker.
pub struct VersionTracker {
    db_path: PathBuf,
    conn: Option<rusqlite::Connection>,
}

impl VersionTracker {
    pub fn new() -> Self {
        Self {
            db_path: PathBuf::new(),
            conn: None,
        }
    }

    /// Open or create a version tracker database.
    pub fn open(&mut self, db_path: &Path) -> bool {
        self.db_path = db_path.to_path_buf();
        match rusqlite::Connection::open(db_path) {
            Ok(conn) => {
                let r = conn.execute_batch(
                    "CREATE TABLE IF NOT EXISTS deliveries (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        package_uuid TEXT NOT NULL,
                        title TEXT NOT NULL DEFAULT '',
                        version TEXT NOT NULL DEFAULT '',
                        destination TEXT NOT NULL DEFAULT '',
                        delivery_method TEXT NOT NULL DEFAULT '',
                        timestamp TEXT NOT NULL DEFAULT '',
                        verified INTEGER NOT NULL DEFAULT 0
                    )",
                );
                if let Err(e) = r {
                    tracing::error!("Failed to create table: {e}");
                    return false;
                }
                self.conn = Some(conn);
                true
            }
            Err(e) => {
                tracing::error!("Failed to open database: {e}");
                false
            }
        }
    }

    fn conn(&self) -> Option<&rusqlite::Connection> {
        self.conn.as_ref()
    }

    /// Record a delivery.
    pub fn record(&self, record: &DeliveryRecord) -> bool {
        let Some(conn) = self.conn() else {
            return false;
        };
        let r = conn.execute(
            "INSERT INTO deliveries (package_uuid, title, version, destination, delivery_method, timestamp, verified)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                record.package_uuid,
                record.title,
                record.version,
                record.destination,
                record.delivery_method,
                record.timestamp,
                record.verified as i32,
            ],
        );
        match r {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("Failed to record delivery: {e}");
                false
            }
        }
    }

    /// Query delivery records with optional filters.
    pub fn query(&self, query: &VersionQuery) -> Vec<DeliveryRecord> {
        let Some(conn) = self.conn() else {
            return Vec::new();
        };

        let mut sql = "SELECT package_uuid, title, version, destination, delivery_method, timestamp, verified FROM deliveries WHERE 1=1".to_string();
        let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
        let mut idx = 1;

        if let Some(ref uuid) = query.package_uuid {
            sql.push_str(&format!(" AND package_uuid = ?{idx}"));
            params.push(Box::new(uuid.clone()));
            idx += 1;
        }
        if let Some(ref title) = query.title {
            sql.push_str(&format!(" AND title LIKE ?{idx}"));
            params.push(Box::new(format!("%{title}%")));
            idx += 1;
        }
        if let Some(ref dest) = query.destination {
            sql.push_str(&format!(" AND destination = ?{idx}"));
            params.push(Box::new(dest.clone()));
            idx += 1;
        }
        if let Some(ref after) = query.after {
            sql.push_str(&format!(" AND timestamp >= ?{idx}"));
            params.push(Box::new(after.clone()));
            idx += 1;
        }
        if let Some(ref before) = query.before {
            sql.push_str(&format!(" AND timestamp <= ?{idx}"));
            params.push(Box::new(before.clone()));
            #[allow(unused_assignments)]
            {
                idx += 1;
            }
        }

        sql.push_str(" ORDER BY timestamp DESC");

        let param_refs: Vec<&dyn rusqlite::types::ToSql> =
            params.iter().map(|p| p.as_ref()).collect();
        let mut stmt = match conn.prepare(&sql) {
            Ok(s) => s,
            Err(e) => {
                tracing::error!("Failed to prepare query: {e}");
                return Vec::new();
            }
        };

        let rows = stmt.query_map(param_refs.as_slice(), |row| {
            Ok(DeliveryRecord {
                package_uuid: row.get(0)?,
                title: row.get(1)?,
                version: row.get(2)?,
                destination: row.get(3)?,
                delivery_method: row.get(4)?,
                timestamp: row.get(5)?,
                verified: row.get::<_, i32>(6)? != 0,
            })
        });

        match rows {
            Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
            Err(e) => {
                tracing::error!("Failed to query deliveries: {e}");
                Vec::new()
            }
        }
    }

    /// List all versions of a specific package.
    pub fn versions_of(&self, package_uuid: &str) -> Vec<DeliveryRecord> {
        self.query(&VersionQuery {
            package_uuid: Some(package_uuid.to_string()),
            ..Default::default()
        })
    }

    /// List all deliveries to a specific destination.
    pub fn deliveries_to(&self, destination: &str) -> Vec<DeliveryRecord> {
        self.query(&VersionQuery {
            destination: Some(destination.to_string()),
            ..Default::default()
        })
    }

    /// Export delivery history as JSON.
    pub fn export_json(&self, output: &Path) -> bool {
        let records = self.query(&VersionQuery::default());
        match serde_json::to_string_pretty(&records) {
            Ok(json) => std::fs::write(output, json).is_ok(),
            Err(e) => {
                tracing::error!("Failed to serialize JSON: {e}");
                false
            }
        }
    }

    /// Export delivery history as CSV.
    pub fn export_csv(&self, output: &Path) -> bool {
        let records = self.query(&VersionQuery::default());
        let mut csv = String::from(
            "package_uuid,title,version,destination,delivery_method,timestamp,verified\n",
        );
        for r in &records {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                r.package_uuid,
                r.title,
                r.version,
                r.destination,
                r.delivery_method,
                r.timestamp,
                r.verified
            ));
        }
        std::fs::write(output, csv).is_ok()
    }
}

impl Default for VersionTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_tracker() {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("tracker.db");

        let mut tracker = VersionTracker::new();
        assert!(tracker.open(&db));

        let record = DeliveryRecord {
            package_uuid: "uuid-1".into(),
            title: "Test Film".into(),
            version: "v1".into(),
            destination: "AMC".into(),
            delivery_method: "satellite".into(),
            timestamp: "2024-01-01T00:00:00Z".into(),
            verified: true,
        };
        assert!(tracker.record(&record));

        let results = tracker.versions_of("uuid-1");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Test Film");

        let amc = tracker.deliveries_to("AMC");
        assert_eq!(amc.len(), 1);
    }

    #[test]
    fn test_export() {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("tracker.db");
        let mut tracker = VersionTracker::new();
        tracker.open(&db);

        tracker.record(&DeliveryRecord {
            package_uuid: "u1".into(),
            title: "Film".into(),
            ..Default::default()
        });

        let json_path = dir.path().join("out.json");
        assert!(tracker.export_json(&json_path));
        let json = std::fs::read_to_string(&json_path).unwrap();
        assert!(json.contains("Film"));

        let csv_path = dir.path().join("out.csv");
        assert!(tracker.export_csv(&csv_path));
        let csv = std::fs::read_to_string(&csv_path).unwrap();
        assert!(csv.contains("Film"));
    }
}
