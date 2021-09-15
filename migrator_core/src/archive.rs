use std::collections::HashMap;
use crate::dbl::MigrationFile;
use crate::result::Result;
use std::path::PathBuf;
use crate::reader;
use crate::util;
use std::collections::hash_map::Entry;
use tracing::*;

pub type MigrationArchive = HashMap<String, Vec<MigrationFile>>;

pub static MIGRATION_CACHE_DIR: &str = "/version_cache";

pub struct LocalVersionArchive {
    path: Option<PathBuf>,
    dir: MigrationArchive,
}

impl From<Vec<MigrationFile>> for LocalVersionArchive {
    fn from(files: Vec<MigrationFile>) -> Self {
        let dir = parse_migrations_to_archive(files);

        LocalVersionArchive {
            dir,
            path: None
        }
    }
}

impl From<PathBuf> for LocalVersionArchive {
    fn from(path: PathBuf) -> Self {
        let new_path = util::join_path(path, MIGRATION_CACHE_DIR);

        let old_versions = match reader::fetch_migration_versions(new_path.clone()) {
            Ok(e) => e,
            Err(_e) => vec![]
        };

        let dir = parse_migrations_to_archive(old_versions);

        LocalVersionArchive {
            dir,
            path: Some(new_path),
        }
    }
}

fn parse_migrations_to_archive(files: Vec<MigrationFile>) -> MigrationArchive {
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

impl LocalVersionArchive {
    pub fn new(archive: MigrationArchive) -> LocalVersionArchive {
        LocalVersionArchive {
            dir: archive,
            path: None
        }
    }

    pub fn default() -> LocalVersionArchive {
        LocalVersionArchive {
            dir: MigrationArchive::new(),
            path: None
        }
    }

    pub fn get_migration_files(&self, migration: &str) -> Option<Vec<MigrationFile>> {
        let versioned_files = self.dir.get(migration)?;

        Some(versioned_files.clone())
    }

    pub fn reset_migration_dir(&self) -> Result<()> {
        Ok(())
    }

    pub fn add_migration_version(&mut self, migration: MigrationFile) -> Result<()> {
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

    pub fn _rollback_version(&self, _migration: &str, _version: i32) -> Result<()> {
        unimplemented!("rollback not available yet");
    }

    fn _remove_versioned_file(&self, _migration: &str, _version: i32) -> Result<()> {
        unimplemented!("remove versioning not available yet");
    }
}