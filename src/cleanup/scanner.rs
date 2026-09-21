use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use walkdir::WalkDir;

use super::categories::{
    CategoryScanResult, CleanupCategoryId, CleanupItem, CLEANUP_CATEGORIES,
};
use super::recycle_bin;

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
        let mut roots = Vec::new();

        if let Ok(temp) = env::var("TEMP") {
            roots.push(PathBuf::from(temp));
        }
        if let Ok(local) = env::var("LOCALAPPDATA") {
            roots.push(PathBuf::from(local).join("Temp"));
        }
        roots.push(PathBuf::from(r"C:\Windows\Temp"));

        for root in roots {
            Self::collect_files_recursive(&root, res, None, None);
        }
    }

    /// 3.2 Browser Cache (Chrome, Edge, Firefox) - NEVER touches Cookies, Login Data, or History!
    fn scan_browser_cache(res: &mut CategoryScanResult) {
        let mut cache_dirs = Vec::new();

        if let Ok(local) = env::var("LOCALAPPDATA") {
            let local_path = PathBuf::from(local);
            // Chrome cache folders
            let chrome_default = local_path.join(r"Google\Chrome\User Data\Default");
            cache_dirs.push(chrome_default.join("Cache"));
            cache_dirs.push(chrome_default.join("Code Cache"));
            cache_dirs.push(chrome_default.join("GPUCache"));
            cache_dirs.push(chrome_default.join(r"Service Worker\CacheStorage"));
            cache_dirs.push(chrome_default.join(r"Service Worker\ScriptCache"));

            // Edge cache folders
            let edge_default = local_path.join(r"Microsoft\Edge\User Data\Default");
            cache_dirs.push(edge_default.join("Cache"));
            cache_dirs.push(edge_default.join("Code Cache"));
            cache_dirs.push(edge_default.join("GPUCache"));
            cache_dirs.push(edge_default.join(r"Service Worker\CacheStorage"));
            cache_dirs.push(edge_default.join(r"Service Worker\ScriptCache"));
        }

        if let Ok(appdata) = env::var("APPDATA") {
            let ff_profiles = PathBuf::from(appdata).join(r"Mozilla\Firefox\Profiles");
            if ff_profiles.is_dir() {
                if let Ok(entries) = fs::read_dir(&ff_profiles) {
                    for entry in entries.flatten() {
                        let cache2 = entry.path().join("cache2");
                        if cache2.is_dir() {
                            cache_dirs.push(cache2);
                        }
                    }
                }
            }
        }

        for dir in cache_dirs {
            Self::collect_files_recursive(&dir, res, None, None);
        }
    }

    /// 3.3 Log Files & Crash Dumps older than threshold (default: 30 days)
    fn scan_old_logs(res: &mut CategoryScanResult, days_threshold: u64) {
        let age_limit = Duration::from_secs(days_threshold * 86400);
        let mut log_roots = Vec::new();

        if let Ok(local) = env::var("LOCALAPPDATA") {
            let local_path = PathBuf::from(local);
            log_roots.push(local_path.join(r"Microsoft\Windows\WER"));
            log_roots.push(local_path.join("CrashDumps"));
        }

        for root in log_roots {
            Self::collect_files_recursive(&root, res, Some(age_limit), None);
        }

        // Also search for *.log and *.dmp in TEMP
        if let Ok(temp) = env::var("TEMP") {
            let filter_ext = |path: &Path| -> bool {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let ext_lower = ext.to_lowercase();
                    ext_lower == "log" || ext_lower == "dmp"
                } else {
                    false
                }
            };
            Self::collect_files_recursive(
                &PathBuf::from(temp),
                res,
                Some(age_limit),
                Some(&filter_ext),
            );
        }
    }

    /// 3.4 Thumbnail Cache
    fn scan_thumbnail_cache(res: &mut CategoryScanResult) {
        if let Ok(local) = env::var("LOCALAPPDATA") {
            let explorer_dir = PathBuf::from(local).join(r"Microsoft\Windows\Explorer");
            if explorer_dir.is_dir() {
                if let Ok(entries) = fs::read_dir(&explorer_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            let name_lower = name.to_lowercase();
                            if name_lower.starts_with("thumbcache_") && name_lower.ends_with(".db")
                            {
                                if let Ok(meta) = entry.metadata() {
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
                    }
                }
            }
        }
    }

    /// 3.5 Recycle Bin (Native Shell32 API)
    fn scan_recycle_bin(res: &mut CategoryScanResult) {
        let (bytes, items) = recycle_bin::query_recycle_bin();
        if bytes > 0 || items > 0 {
            // Represent Recycle Bin as a single synthetic item for UI display
            res.items.push(CleanupItem {
                path: PathBuf::from("Recycle Bin (Semua Drive)"),
                size: bytes,
                modified: Some(SystemTime::now()),
            });
        }
    }

    /// 3.6 Installer/Setup Files in Downloads older than threshold (default: 60 days)
    fn scan_downloads_installers(res: &mut CategoryScanResult, days_threshold: u64) {
        let age_limit = Duration::from_secs(days_threshold * 86400);

        let mut downloads_dir = None;
        if let Ok(profile) = env::var("USERPROFILE") {
            let p = PathBuf::from(profile).join("Downloads");
            if p.is_dir() {
                downloads_dir = Some(p);
            }
        }

        if let Some(dir) = downloads_dir {
            let filter_ext = |path: &Path| -> bool {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let ext_lower = ext.to_lowercase();
                    ext_lower == "exe" || ext_lower == "msi"
                } else {
                    false
                }
            };

            Self::collect_files_recursive(&dir, res, Some(age_limit), Some(&filter_ext));
        }
    }

    /// 3.7 Developer Tools Cache
    fn scan_dev_tools_cache(res: &mut CategoryScanResult) {
        let mut dev_dirs = Vec::new();

        if let Ok(appdata) = env::var("APPDATA") {
            let appdata_path = PathBuf::from(appdata);
            dev_dirs.push(appdata_path.join("npm-cache"));
            dev_dirs.push(appdata_path.join(r"Code\Cache"));
            dev_dirs.push(appdata_path.join(r"Code\CachedData"));
            dev_dirs.push(appdata_path.join(r"Code\logs"));
        }

        if let Ok(local) = env::var("LOCALAPPDATA") {
            dev_dirs.push(PathBuf::from(local).join(r"pip\Cache"));
        }

        if let Ok(profile) = env::var("USERPROFILE") {
            dev_dirs.push(PathBuf::from(profile).join(r".cargo\registry\cache"));
        }

        for dir in dev_dirs {
            Self::collect_files_recursive(&dir, res, None, None);
        }
    }

    /// 3.8 Consumer Apps Cache (Discord, Spotify)
    fn scan_consumer_apps_cache(res: &mut CategoryScanResult) {
        let mut app_dirs = Vec::new();

        if let Ok(appdata) = env::var("APPDATA") {
            let appdata_path = PathBuf::from(appdata);
            let discord = appdata_path.join("discord");
            app_dirs.push(discord.join("Cache"));
            app_dirs.push(discord.join("Code Cache"));
            app_dirs.push(discord.join("GPUCache"));
            app_dirs.push(appdata_path.join(r"Spotify\Storage"));
        }

        if let Ok(local) = env::var("LOCALAPPDATA") {
            app_dirs.push(PathBuf::from(local).join(r"Spotify\Storage"));
        }

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

