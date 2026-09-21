use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::SystemTime;
use walkdir::{DirEntry, WalkDir};

use super::exclude_list;

#[derive(Debug, Clone)]
pub struct RawFileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub created: Option<SystemTime>,
    pub modified: Option<SystemTime>,
}

#[derive(Debug, Clone)]
pub struct WalkerConfig {
    pub roots: Vec<PathBuf>,
    pub exclude_dirs: Vec<PathBuf>,
    pub exclude_extensions: Vec<String>,
    pub min_size: u64,
    pub max_size: Option<u64>,
    pub include_hidden: bool,
    pub scan_all_folders: bool, // When false, default exclude list is strictly applied
}

impl Default for WalkerConfig {
    fn default() -> Self {
        Self {
            roots: Vec::new(),
            exclude_dirs: Vec::new(),
            exclude_extensions: Vec::new(),
            min_size: 1, // Ignore 0-byte files by default
            max_size: None,
            include_hidden: false,
            scan_all_folders: false,
        }
    }
}

/// Checks whether a directory or file should be skipped based on system patterns.
pub fn is_system_or_ignored(entry: &DirEntry, include_hidden: bool) -> bool {
    let file_name = entry.file_name().to_string_lossy();
    let lower_name = file_name.to_lowercase();

    // System folders that shouldn't be scanned
    if lower_name == "$recycle.bin"
        || lower_name == "system volume information"
        || lower_name == "windows"
        || lower_name == "recovery"
        || lower_name == "appdata"
        || lower_name == "pagefile.sys"
        || lower_name == "hiberfil.sys"
        || lower_name == "dumpstack.log"
    {
        return true;
    }

    if !include_hidden && file_name.starts_with('.') {
        return true;
    }

    false
}

/// Recursively scans roots and groups files by their exact byte size.
/// Skips excluded directories and returns (size_map, total_files_scanned, folders_skipped).
pub fn walk_and_group_by_size(
    config: &WalkerConfig,
    cancel_flag: &Arc<AtomicBool>,
    on_file_found: &dyn Fn(&Path, usize),
) -> (HashMap<u64, Vec<RawFileInfo>>, usize, usize) {
    let mut size_map: HashMap<u64, Vec<RawFileInfo>> = HashMap::new();
    let mut total_files_scanned = 0;
    let folders_skipped = Arc::new(AtomicUsize::new(0));

    let normalized_excludes: Arc<Vec<PathBuf>> = Arc::new(
        config
            .exclude_dirs
            .iter()
            .map(|p| p.canonicalize().unwrap_or_else(|_| p.clone()))
            .collect(),
    );

    let clean_exts: Vec<String> = config
        .exclude_extensions
        .iter()
        .map(|ext| ext.trim_start_matches('.').to_lowercase())
        .collect();

    for root in &config.roots {
        if !root.exists() {
            continue;
        }

        let skipped_ref = Arc::clone(&folders_skipped);
        let excludes_ref = Arc::clone(&normalized_excludes);
        let scan_all = config.scan_all_folders;

        let walker = WalkDir::new(root)
            .follow_links(false)
            .into_iter()
            .filter_entry(move |entry| {
                if is_system_or_ignored(entry, config.include_hidden) {
                    return false;
                }

                // Check directory exclusions if entry is a directory and not the scan root itself
                if entry.file_type().is_dir() && entry.depth() > 0 {
                    let entry_path = entry.path();

                    // Kategori A: Framework & build directory exclusions
                    if !scan_all && exclude_list::is_excluded_directory(entry_path) {
                        skipped_ref.fetch_add(1, Ordering::Relaxed);
                        return false;
                    }

                    // User-specified exclude directories
                    for ex in excludes_ref.iter() {
                        if entry_path.starts_with(ex) {
                            skipped_ref.fetch_add(1, Ordering::Relaxed);
                            return false;
                        }
                    }
                }

                true
            });

        for entry_res in walker {
            if cancel_flag.load(Ordering::Relaxed) {
                return (HashMap::new(), total_files_scanned, folders_skipped.load(Ordering::Relaxed));
            }

            let entry = match entry_res {
                Ok(e) => e,
                Err(_) => continue, // Skip unreadable or permission-denied entries
            };

            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path().to_path_buf();

            // Check file extension exclusion
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if clean_exts.iter().any(|e| e.eq_ignore_ascii_case(ext)) {
                    continue;
                }
            }

            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            let size = metadata.len();
            if size < config.min_size {
                continue;
            }
            if let Some(max) = config.max_size {
                if size > max {
                    continue;
                }
            }

            total_files_scanned += 1;
            on_file_found(&path, total_files_scanned);

            let created = metadata.created().ok();
            let modified = metadata.modified().ok();

            let info = RawFileInfo {
                path,
                size,
                created,
                modified,
            };

            size_map.entry(size).or_default().push(info);
        }
    }

    // Retain only sizes that have 2 or more files (potential duplicates)
    size_map.retain(|_, files| files.len() >= 2);

    let skipped = folders_skipped.load(Ordering::Relaxed);
    (size_map, total_files_scanned, skipped)
}
