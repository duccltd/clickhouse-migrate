use chrono::{Local};
use std::fmt::Formatter;

use std::path::PathBuf;
use crate::util::{write_file, calculate_hash};
use crate::result::Result;
use std::hash::{Hash};
use tracing::*;

#[derive(Debug, Clone, Hash)]
pub struct MigrationFile {
    pub name: String,
    pub sql: String
}

impl std::fmt::Display for MigrationFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "migration {} - checksum {}", self.name, self.checksum())
    }
}

impl Into<MigrationFile> for PathBuf {
    fn into(self) -> MigrationFile {
        let file_name = String::from(self.file_name()
            .and_then(|name| name.to_str())
            .unwrap());

        let content = std::fs::read_to_string(
            self.as_path()
        ).unwrap_or("".to_string());

        MigrationFile::default(&file_name, &content)
    }
}

impl MigrationFile {
    pub fn new(name: &str, content: &str) -> MigrationFile {
        MigrationFile {
            name: name.to_string(),
            sql: content.to_string()
        }
    }

    pub fn default(name: &str, content: &str) -> MigrationFile {
        MigrationFile {
            name: name.to_string(),
            sql: content.to_string()
        }
    }

    pub fn create(directory: String, name: String) -> Result<()> {
        let new_name = name.replace(" ", "-");
        let file_name = format!("{}_{}.sql", Local::now().format("%Y%m%d%H%M%S"), &new_name);

        let path = PathBuf::from(format!("{}/{}", &directory, &file_name));

        write_file(path, &vec![])?;

        info!("Created new migration: {}", file_name);

        Ok(())
    }

    pub fn to_display(&self) -> String {
        let mut name: String = String::from("/v");

        name = name + &self.name + ".sql";

        name
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn checksum(&self) -> u64 {
        calculate_hash(self)
    }

    pub fn sql(&self) -> &String {
        &self.sql
    }

    pub fn to_insert_sql(&self) -> String {
        let timestamp = Local::now();

        let ts_seconds = timestamp.timestamp();
        let ts_nanos = timestamp.timestamp_subsec_nanos();

        let entry = ((ts_seconds as u64) * 1_000_000_000) + (ts_nanos as u64);

        format!(
            "INSERT INTO migration_lock (*) VALUES ({}, '{}', '{}')",
            entry,
            self.name,
            self.checksum()
        )
    }
}