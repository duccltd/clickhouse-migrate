use chrono::Local;
use std::fmt::Formatter;

use crate::result::Result;
use crate::util::{calculate_hash, write_file};
use std::hash::Hash;
use std::path::PathBuf;
use tracing::*;

#[derive(Debug, Clone, Hash)]
pub struct MigrationFile {
    pub name: String,
    pub sql: String,
    pub rollback: bool
}

impl std::fmt::Display for MigrationFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "migration {} - checksum {}", self.name, self.checksum())
    }
}

impl Into<MigrationFile> for PathBuf {
    fn into(self) -> MigrationFile {
        let file_name = String::from(self.file_name().and_then(|name| name.to_str()).unwrap());

        let content = std::fs::read_to_string(self.as_path()).unwrap_or("".to_string());

        let rollback = file_name.contains(".down");

        MigrationFile {
            name: file_name,
            sql: content,
            rollback
        }
    }
}

impl MigrationFile {
    pub fn create(directory: String, name: String) -> Result<()> {
        let new_name = name.replace(" ", "-");
        let file_name = format!("{}_{}", Local::now().format("%Y%m%d%H%M%S"), &new_name);

        let up_path = PathBuf::from(format!("{}/{}.up.sql", &directory, &file_name));
        let down_path = PathBuf::from(format!("{}/{}.down.sql", &directory, &file_name));

        write_file(up_path, &vec![])?;
        write_file(down_path, &vec![])?;

        info!("Created new migration: {}", file_name);

        Ok(())
    }

    pub fn checksum(&self) -> u64 {
        calculate_hash(self)
    }

    pub fn to_insert_sql(&self) -> String {
        let timestamp = Local::now();

        let ts_seconds = timestamp.timestamp();
        let ts_nanos = timestamp.timestamp_subsec_nanos();

        let entry = ((ts_seconds as u64) * 1_000_000_000) + (ts_nanos as u64);

        format!(
            "INSERT INTO clickhouse_migrations (*) VALUES ({}, '{}', '{}')",
            entry,
            self.name,
            self.checksum()
        )
    }
}
