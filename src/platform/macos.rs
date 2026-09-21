use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

pub fn get_temp_dirs() -> Vec<PathBuf> {
    let mut roots = vec![
        PathBuf::from("/private/var/folders"),
        PathBuf::from("/tmp"),
    ];
    if let Some(cache_base) = dirs::cache_dir() {
        roots.push(cache_base);
    }
    roots
}

pub fn get_chrome_cache_dirs() -> Vec<PathBuf> {
    let mut dirs_list = Vec::new();
    if let Some(cache_base) = dirs::cache_dir() {
        let chrome_default = cache_base.join("Google/Chrome/Default");
        dirs_list.push(chrome_default.join("Cache"));
        dirs_list.push(chrome_default.join("Code Cache"));
        dirs_list.push(chrome_default.join("GPUCache"));
        dirs_list.push(chrome_default.join("Service Worker/CacheStorage"));
        dirs_list.push(chrome_default.join("Service Worker/ScriptCache"));

        let edge_default = cache_base.join("Microsoft Edge/Default");
        dirs_list.push(edge_default.join("Cache"));
        dirs_list.push(edge_default.join("Code Cache"));
        dirs_list.push(edge_default.join("GPUCache"));
    }
    dirs_list
}

pub fn get_firefox_cache_dirs() -> Vec<PathBuf> {
    let mut dirs_list = Vec::new();
    if let Some(cache_base) = dirs::cache_dir() {
        let ff_cache = cache_base.join("Firefox/Profiles");
        if ff_cache.is_dir() {
            if let Ok(entries) = fs::read_dir(&ff_cache) {
                for entry in entries.flatten() {
                    let cache2 = entry.path().join("cache2");
                    if cache2.is_dir() {
                        dirs_list.push(cache2);
                    }
                }
            }
        }
    }
    dirs_list
}

pub fn get_old_logs_dirs() -> Vec<PathBuf> {
    let mut log_roots = Vec::new();
    if let Some(home) = dirs::home_dir() {
        let logs = home.join("Library/Logs");
        if logs.is_dir() {
            log_roots.push(logs.clone());
            let diag = logs.join("DiagnosticReports");
            if diag.is_dir() {
                log_roots.push(diag);
            }
        }
    }
    log_roots
}

pub fn get_thumbnail_cache_files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Some(cache_base) = dirs::cache_dir() {
        let ql_cache = cache_base.join("com.apple.QuickLook.thumbnailcache");
        if ql_cache.is_dir() {
            for entry in WalkDir::new(&ql_cache)
                .min_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_type().is_file() {
                    files.push(entry.into_path());
                }
            }
        }
    }
    files
}

pub fn get_downloads_installer_patterns() -> (Vec<PathBuf>, &'static [&'static str]) {
    let mut dirs_list = Vec::new();
    if let Some(d) = dirs::download_dir() {
        dirs_list.push(d);
    } else if let Some(home) = dirs::home_dir() {
        let d = home.join("Downloads");
        if d.is_dir() {
            dirs_list.push(d);
        }
    }
    (dirs_list, &["dmg", "pkg"])
}

pub fn get_dev_tools_cache_dirs() -> Vec<PathBuf> {
    let mut dev_dirs = Vec::new();
    if let Some(home) = dirs::home_dir() {
        dev_dirs.push(home.join(".npm"));
        dev_dirs.push(home.join(".cargo/registry/cache"));
        let app_support = home.join("Library/Application Support");
        dev_dirs.push(app_support.join("Code/Cache"));
        dev_dirs.push(app_support.join("Code/CachedData"));
        dev_dirs.push(home.join("Library/Logs/Code"));
    }
    if let Some(cache_base) = dirs::cache_dir() {
        dev_dirs.push(cache_base.join("pip"));
    }
    dev_dirs
}

pub fn get_consumer_apps_cache_dirs() -> Vec<PathBuf> {
    let mut app_dirs = Vec::new();
    if let Some(home) = dirs::home_dir() {
        let app_support = home.join("Library/Application Support");
        let discord = app_support.join("discord");
        app_dirs.push(discord.join("Cache"));
        app_dirs.push(discord.join("Code Cache"));
        app_dirs.push(discord.join("GPUCache"));
        app_dirs.push(app_support.join("Spotify/Storage"));
    }
    if let Some(cache_base) = dirs::cache_dir() {
        app_dirs.push(cache_base.join("com.spotify.client"));
    }
    app_dirs
}

fn get_macos_trash_dir() -> PathBuf {
    if let Some(home) = dirs::home_dir() {
        home.join(".Trash")
    } else {
        PathBuf::from("/tmp")
    }
}

pub fn query_recycle_bin() -> (u64, u64) {
    let trash_dir = get_macos_trash_dir();
    if !trash_dir.is_dir() {
        return (0, 0);
    }

    let mut total_bytes = 0u64;
    let mut total_items = 0u64;

    for entry in WalkDir::new(&trash_dir)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            total_items += 1;
            if let Ok(meta) = entry.metadata() {
                total_bytes += meta.len();
            }
        }
    }

    (total_bytes, total_items)
}

pub fn empty_recycle_bin() -> Result<(), String> {
    let trash_dir = get_macos_trash_dir();
    if !trash_dir.is_dir() {
        return Ok(());
    }

    let mut errors = Vec::new();
    if let Ok(entries) = fs::read_dir(&trash_dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            let res = if p.is_dir() {
                fs::remove_dir_all(&p)
            } else {
                fs::remove_file(&p)
            };
            if let Err(e) = res {
                errors.push(format!("Failed to delete {:?}: {}", p, e));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("; "))
    }
}

pub fn open_in_file_manager(path: &Path) {
    let _ = Command::new("open").arg("-R").arg(path).spawn();
}

pub fn open_file(path: &Path) {
    let _ = Command::new("open").arg(path).spawn();
}
