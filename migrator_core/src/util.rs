use crate::result::IOResult;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn standardise_path(location: &str) -> IOResult<PathBuf> {
    let location = Path::new(location);

    if !location.exists() {
        create_dir(location)?;
    }

    // Standardise the location path
    Ok(location.canonicalize().unwrap())
}

pub fn write_file(path: PathBuf, data: &[u8]) -> std::io::Result<()> {
    let prefix = path.parent().unwrap();

    std::fs::create_dir_all(prefix)?;

    let mut file = File::create(path)?;

    file.write_all(data)?;

    Ok(())
}

pub fn create_dir(path: &Path) -> IOResult<()> {
    std::fs::create_dir_all(path)?;

    Ok(())
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
