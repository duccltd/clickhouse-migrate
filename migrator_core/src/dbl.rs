use chrono::{DateTime, Local};
use std::fmt::Formatter;

use regex::Regex;
use std::path::PathBuf;
use crate::util::{write_file, calculate_hash, create_path};
use crate::result::Result;
use std::hash::{Hash};

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

    pub fn write(&self, location: PathBuf) -> Result<()> {
        let location = create_path(location, &self.to_display());

        write_file(location, self.sql.as_bytes())?;

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