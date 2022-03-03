use std::path::{PathBuf};

use crate::dbl::MigrationFile;
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
