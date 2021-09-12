
use std::collections::HashMap;
use crate::migrator::Migration;
use crate::result::Result;
use std::path::PathBuf;
use crate::reader;
use crate::util;
use std::collections::hash_map::Entry;
use tracing::*;

pub type MigrationArchive = HashMap<String, Vec<Migration>>;

pub static MIGRATION_CACHE_DIR: &str = "/version_cache";

pub struct Archive {
    path: Option<PathBuf>,
    dir: MigrationArchive,
}

impl From<Vec<Migration>> for Archive {
    fn from(files: Vec<Migration>) -> Self {
        let dir = parse_migrations_to_archive(files);

        Archive {
            dir,
            path: None
        }
    }
}

impl From<PathBuf> for Archive {
    fn from(path: PathBuf) -> Self {
        let new_path = util::join_path(path, MIGRATION_CACHE_DIR);

        let old_versions = match reader::fetch_migration_versions(new_path.clone()) {
            Ok(e) => e,
            Err(_e) => vec![]
        };

        let dir = parse_migrations_to_archive(old_versions);

        Archive {
            dir,
            path: Some(new_path),
        }
    }
}

fn parse_migrations_to_archive(files: Vec<Migration>) -> MigrationArchive {
    let mut dir = MigrationArchive::new();
    for migration in files.clone() {
        let file_name = migration.name().clone();

        let entry = dir.get_mut(&file_name);
        if let Some(entry) = entry {
            entry.push(migration);
        } else {
            dir.insert(file_name, vec![migration]);
        }
    }

    dir
}

impl Archive {
    pub fn new(archive: MigrationArchive) -> Archive {
        Archive {
            dir: archive,
            path: None
        }
    }

    pub fn default() -> Archive {
        Archive {
            dir: MigrationArchive::new(),
            path: None
        }
    }

    pub fn get_migration_files(&self, migration: &str) -> Option<Vec<Migration>> {
        let versioned_files = self.dir.get(migration)?;

        Some(versioned_files.clone())
    }

    pub fn reset_migration_dir(&self) -> Result<()> {
        Ok(())
    }

    pub fn add_migration_version(&mut self, migration: Migration) -> Result<()> {
        let migration_name = migration.name().clone();

        if self.path.is_some() {
            migration.write(util::join_path(self.path.clone().unwrap(), &("/".to_string() + &migration_name)))?;
        }

        match self.dir.entry(migration_name) {
            Entry::Occupied(mut e) => {
                e.get_mut().push(migration);
            },
            Entry::Vacant(e) => {
                e.insert(vec![migration]);
            }
        };

        Ok(())
    }

    pub fn get_latest_version(&self, migration: &str) -> Option<Migration> {
        let mut latest_migration: Option<Migration> = None;
        if let Some(versions) =  self.get_migration_files(migration) {
            let mut latest_version: Option<i32> = None;
            for mut version in versions {
                if version.version().is_none() {
                    continue
                }
                let curr_version = version.version().unwrap();
                if latest_version.is_none() {
                    latest_version = Some(curr_version);
                    latest_migration = Some(version);
                } else if curr_version > latest_version.unwrap() {
                    latest_version = Some(curr_version);
                    latest_migration = Some(version);
                }
            }
        }
        latest_migration
    }

    pub fn _rollback_version(&self, _migration: &str, _version: i32) -> Result<()> {
        unimplemented!("rollback not available yet");
    }

    fn _remove_versioned_file(&self, _migration: &str, _version: i32) -> Result<()> {
        unimplemented!("remove versioning not available yet");
    }
}