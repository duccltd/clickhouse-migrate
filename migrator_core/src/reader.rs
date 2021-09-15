use walkdir::{WalkDir, DirEntry as WalkDirEntry};
use std::path::{PathBuf};

use crate::error::{ErrorType};
use crate::dbl::MigrationFile;
use crate::util;
use crate::result::Result;
use std::{fs};

pub fn find_migration_files(path: PathBuf) -> Result<Vec<MigrationFile>> {
    let file_paths = fs::read_dir(path)?
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(
            move | entry | match entry.file_name().to_str() {
                Some(file_name) if file_name.ends_with(".sql") => true,
                Some(_file_name) => false,
                None => false
            }
        )
        .map(|e| PathBuf::into(e.path()))
        .collect();

    Ok(file_paths)
}


pub fn fetch_migration_versions(path: PathBuf) -> Result<Vec<MigrationFile>> {
    if !path.exists() {
        util::create_dir(path.as_path())?;
    } else if path.is_file() {
        return Err(
            ErrorType::VersionCacheInvalidType(String::from("version_cache is a file instead of a folder"))
        );
    }

    let migrations = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(WalkDirEntry::into_path)
        .map(PathBuf::into)
        .collect();

    Ok(migrations)
}