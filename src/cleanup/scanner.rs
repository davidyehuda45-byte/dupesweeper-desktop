use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use walkdir::WalkDir;

use super::categories::{
    CategoryScanResult, CleanupCategoryId, CleanupItem, CLEANUP_CATEGORIES,
};

pub struct CleanupScanner;

impl CleanupScanner {
    /// Scans all defined cleanup categories and populates their items and sizes.
    pub fn scan_all<F>(progress_callback: Option<F>) -> Vec<CategoryScanResult>
    where
        F: Fn(&str, f32),
    {
        let total = CLEANUP_CATEGORIES.len();
        let mut results = Vec::with_capacity(total);

        for (idx, def) in CLEANUP_CATEGORIES.iter().enumerate() {
            if let Some(ref cb) = progress_callback {
                let ratio = idx as f32 / total as f32;
                cb(def.title, ratio);
            }

            let mut res = CategoryScanResult::new(def);

            match def.id {
                CleanupCategoryId::TempFiles => {
                    Self::scan_temp_files(&mut res);
                }
                CleanupCategoryId::BrowserCache => {
                    Self::scan_browser_cache(&mut res);
                }
                CleanupCategoryId::OldLogs => {
                    Self::scan_old_logs(&mut res, 30);
                }
                CleanupCategoryId::ThumbnailCache => {
                    Self::scan_thumbnail_cache(&mut res);
                }
                CleanupCategoryId::RecycleBin => {
                    Self::scan_recycle_bin(&mut res);
                }
                CleanupCategoryId::DownloadsInstallers => {
                    Self::scan_downloads_installers(&mut res, 60);
                }
                CleanupCategoryId::DevToolsCache => {
                    Self::scan_dev_tools_cache(&mut res);
                }
                CleanupCategoryId::ConsumerAppsCache => {
                    Self::scan_consumer_apps_cache(&mut res);
                }
            }

            res.total_bytes = res.items.iter().map(|i| i.size).sum();
            results.push(res);
        }

        if let Some(ref cb) = progress_callback {
            cb("Analisis Selesai", 1.0);
        }

        results
    }

    /// 3.1 Temporary Files
    fn scan_temp_files(res: &mut CategoryScanResult) {
        let roots = crate::platform::get_temp_dirs();
        for root in roots {
            Self::collect_files_recursive(&root, res, None, None);
        }
    }

    /// 3.2 Browser Cache (Chrome, Edge, Firefox) - NEVER touches Cookies, Login Data, or History!
    fn scan_browser_cache(res: &mut CategoryScanResult) {
        let chrome_dirs = crate::platform::get_chrome_cache_dirs();
        for dir in chrome_dirs {
            Self::collect_files_recursive(&dir, res, None, None);
        }

        let ff_dirs = crate::platform::get_firefox_cache_dirs();
        for dir in ff_dirs {
            Self::collect_files_recursive(&dir, res, None, None);
        }
    }

    /// 3.3 Log Files & Crash Dumps older than threshold (default: 30 days)
    fn scan_old_logs(res: &mut CategoryScanResult, days_threshold: u64) {
        let age_limit = Duration::from_secs(days_threshold * 86400);
        let log_roots = crate::platform::get_old_logs_dirs();

        for root in log_roots {
            Self::collect_files_recursive(&root, res, Some(age_limit), None);
        }

        // Also search for *.log and *.dmp in temp dirs
        let temp_dirs = crate::platform::get_temp_dirs();
        let filter_ext = |path: &Path| -> bool {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext_lower = ext.to_lowercase();
                ext_lower == "log" || ext_lower == "dmp"
            } else {
                false
            }
        };

        for temp in temp_dirs {
            Self::collect_files_recursive(
                &temp,
                res,
                Some(age_limit),
                Some(&filter_ext),
            );
        }
    }

    /// 3.4 Thumbnail Cache
    fn scan_thumbnail_cache(res: &mut CategoryScanResult) {
        let files = crate::platform::get_thumbnail_cache_files();
        for path in files {
            if let Ok(meta) = fs::metadata(&path) {
                if meta.is_file() {
                    res.items.push(CleanupItem {
                        path,
                        size: meta.len(),
                        modified: meta.modified().ok(),
                    });
                }
            }
        }
    }

    /// 3.5 Recycle Bin / Trash
    fn scan_recycle_bin(res: &mut CategoryScanResult) {
        let (bytes, items) = crate::platform::query_recycle_bin();
        if bytes > 0 || items > 0 {
            // Represent Recycle Bin / Trash as a single synthetic item for UI display
            res.items.push(CleanupItem {
                path: PathBuf::from("Recycle Bin / Trash (Semua Drive)"),
                size: bytes,
                modified: Some(SystemTime::now()),
            });
        }
    }

    /// 3.6 Installer/Setup Files in Downloads older than threshold (default: 60 days)
    fn scan_downloads_installers(res: &mut CategoryScanResult, days_threshold: u64) {
        let age_limit = Duration::from_secs(days_threshold * 86400);
        let (dirs, exts) = crate::platform::get_downloads_installer_patterns();

        let filter_ext = move |path: &Path| -> bool {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext_lower = ext.to_lowercase();
                exts.iter().any(|&e| e.eq_ignore_ascii_case(&ext_lower))
            } else {
                false
            }
        };

        for dir in dirs {
            Self::collect_files_recursive(&dir, res, Some(age_limit), Some(&filter_ext));
        }
    }

    /// 3.7 Developer Tools Cache
    fn scan_dev_tools_cache(res: &mut CategoryScanResult) {
        let dev_dirs = crate::platform::get_dev_tools_cache_dirs();
        for dir in dev_dirs {
            Self::collect_files_recursive(&dir, res, None, None);
        }
    }

    /// 3.8 Consumer Apps Cache (Discord, Spotify)
    fn scan_consumer_apps_cache(res: &mut CategoryScanResult) {
        let app_dirs = crate::platform::get_consumer_apps_cache_dirs();
        for dir in app_dirs {
            Self::collect_files_recursive(&dir, res, None, None);
        }
    }

    /// Helper to walk directory recursively and collect matching files.
    pub fn collect_files_recursive(
        dir: &Path,
        res: &mut CategoryScanResult,
        min_age: Option<Duration>,
        filter: Option<&dyn Fn(&Path) -> bool>,
    ) {
        if !dir.exists() || !dir.is_dir() {
            return;
        }

        let now = SystemTime::now();

        for entry in WalkDir::new(dir)
            .min_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();

            if let Some(f) = filter {
                if !f(path) {
                    continue;
                }
            }

            let meta = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            let modified = meta.modified().ok();

            if let Some(limit) = min_age {
                let is_old_enough = if let Some(m) = modified {
                    if let Ok(age) = now.duration_since(m) {
                        age >= limit
                    } else {
                        false
                    }
                } else {
                    false
                };

                if !is_old_enough {
                    continue;
                }
            }

            res.items.push(CleanupItem {
                path: path.to_path_buf(),
                size: meta.len(),
                modified,
            });
        }
    }
}

