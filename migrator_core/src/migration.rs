use chrono::{DateTime, Local};
use std::fmt::Formatter;

use regex::Regex;
use std::path::PathBuf;
use crate::util::{write_file, checksum, create_path};
use crate::result::Result;


lazy_static! {
    static ref REGEX: Regex = Regex::new(r"\b([vV]([0-9])_{1,2})?([A-z0-9]\w+).sql").unwrap();
}

#[derive(Debug, Clone)]
pub struct Migration {
    name: String,
    version: Option<i32>,
    sql: String,
    checksum: u64
}

impl std::fmt::Display for Migration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.version {
            Some(version) => write!(f, "migration {} - version {}", self.name, version),
            None => write!(f, "migration {}", self.name)
        }
    }
}

impl Into<Migration> for PathBuf {
    fn into(self) -> Migration {
        let file_name = String::from(self.file_name()
            .and_then(|name| name.to_str())
            .unwrap());

        let content = std::fs::read_to_string(
            self.as_path()
        ).unwrap_or("".to_string());

        if REGEX.is_match(&file_name) {
            let matches = REGEX.captures(&file_name).unwrap();

            // Check to see if is versioned file
            return if matches.get(1).is_none() && matches.get(2).is_none() {
                Migration::new(&file_name, &content, None)
            } else {
                let name = match matches.get(3) {
                    Some(name) => name.as_str(),
                    None => &file_name
                };
                let version = match matches.get(2) {
                    Some(version) => version.as_str().parse::<i32>().unwrap(),
                    None => return Migration::default(&file_name, &content)
                };

                Migration::new(name, &content, Some(version))
            }
        } else {
            Migration::default(&file_name, &content)
        }
    }
}

impl Migration {
    pub fn new(name: &str, content: &str, version: Option<i32>) -> Migration {
        // Checksum to get file hash
        let checksum = checksum(&[name, content]);

        Migration {
            name: name.to_string(),
            version: Some(version.unwrap_or(0)),
            checksum,
            sql: content.to_string()
        }
    }

    pub fn default(name: &str, content: &str) -> Migration {
        let checksum = checksum(&[name, content]);

        Migration {
            name: name.to_string(),
            version: Some(0),
            checksum,
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
        if self.version.is_some() {
            name = name + &self.version.unwrap().to_string() + "_";
        }

        name = name + &self.name + ".sql";

        name
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn version(&mut self) -> &Option<i32> {
        &mut self.version
    }

    pub fn checksum(&self) -> &u64 {
        &self.checksum
    }

    pub fn sql(&self) -> &String {
        &self.sql
    }

    pub fn bump_version(&mut self) {
        if self.version.is_none() {
            self.version = Some(1);
        } else {
            self.version = Some(self.version.unwrap() + 1);
        }
    }

    pub fn set_version(&mut self, version: u32) {
        self.version = Some(version as i32);
    }

    pub fn to_insert_sql(&self) -> String {
        let timestamp = Local::now();

        let ts_seconds = timestamp.timestamp();
        let ts_nanos = timestamp.timestamp_subsec_nanos();

        let entry = ((ts_seconds as u64) * 1_000_000_000) + (ts_nanos as u64);

        format!(
            "INSERT INTO migration_lock (*) VALUES ({}, '{}', {}, {})",
            entry,
            self.name,
            self.version.unwrap(),
            self.checksum
        )
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionReport {
    ran_migrations: Vec<Migration>,
}

impl ExecutionReport {
    pub fn new(ran_migrations: Vec<Migration>) -> Self {
        ExecutionReport {
            ran_migrations
        }
    }
}

impl std::fmt::Display for ExecutionReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for migration in &self.ran_migrations {
            writeln!(f, "{}", migration);
        }
        writeln!(f, "{} migrations", &self.ran_migrations.len())
    }
}