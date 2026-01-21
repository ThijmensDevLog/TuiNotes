use std::fs;
use std::path::{Path, PathBuf};

pub fn list_md_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                files.push(path);
            }
        }
    }

    files.sort();
    files
}

pub fn load_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

pub fn save_file(path: &Path, content: &str) {
    let _ = fs::write(path, content);
}
