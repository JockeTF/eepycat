use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

use walkdir::DirEntry;
use walkdir::WalkDir;

const FILTER: &[&str] = &["c", "rs"];

fn filter(entry: DirEntry) -> Option<(PathBuf, String)> {
    let path = entry.into_path();
    let extension = path
        .extension()
        .map(OsStr::to_string_lossy)
        .map(String::from)?;

    if path.is_file() && FILTER.contains(&extension.as_str()) {
        Some((path, extension))
    } else {
        None
    }
}

pub fn search(path: impl AsRef<Path>) -> Vec<(PathBuf, String)> {
    WalkDir::new(path)
        .into_iter()
        .flatten()
        .filter_map(filter)
        .collect()
}
