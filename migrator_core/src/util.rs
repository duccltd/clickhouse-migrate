use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::Write;
use crate::result::{IOResult};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn join_path(path: PathBuf, extension: &str) -> PathBuf {
    let pathway = format!("{}{}", path.to_str().unwrap(), extension);

    let joined = Path::new(&pathway);

    if !joined.exists() {
        create_dir(joined);
    }

    joined.canonicalize().unwrap()
}

pub fn create_path(path: PathBuf, extension: &str) -> PathBuf {
    let pathway = format!("{}{}", path.to_str().unwrap(), extension);

    let path = Path::new(&pathway);

    path.to_path_buf()
}

pub fn standardise_path(location: &str) -> PathBuf {
    let location = Path::new(location);

    if !location.exists() {
        create_dir(location);
    }

    // Standardise the location path
    location.canonicalize().unwrap()
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