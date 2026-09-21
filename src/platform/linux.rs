use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

pub fn get_temp_dirs() -> Vec<PathBuf> {
    vec![PathBuf::from("/tmp"), PathBuf::from("/var/tmp")]
}

pub fn get_chrome_cache_dirs() -> Vec<PathBuf> {
    let mut dirs_list = Vec::new();
    if let Some(cache_base) = dirs::cache_dir() {
        // Google Chrome
        let chrome_default = cache_base.join("google-chrome/Default");
        dirs_list.push(chrome_default.join("Cache"));
        dirs_list.push(chrome_default.join("Code Cache"));
        dirs_list.push(chrome_default.join("GPUCache"));
        dirs_list.push(chrome_default.join("Service Worker/CacheStorage"));
        dirs_list.push(chrome_default.join("Service Worker/ScriptCache"));

        // Chromium
        let chromium_default = cache_base.join("chromium/Default");
        dirs_list.push(chromium_default.join("Cache"));
        dirs_list.push(chromium_default.join("Code Cache"));
        dirs_list.push(chromium_default.join("GPUCache"));
        dirs_list.push(chromium_default.join("Service Worker/CacheStorage"));
        dirs_list.push(chromium_default.join("Service Worker/ScriptCache"));

        // Microsoft Edge for Linux
        let edge_default = cache_base.join("microsoft-edge/Default");
        dirs_list.push(edge_default.join("Cache"));
        dirs_list.push(edge_default.join("Code Cache"));
        dirs_list.push(edge_default.join("GPUCache"));
    }
    dirs_list
}

pub fn get_firefox_cache_dirs() -> Vec<PathBuf> {
    let mut dirs_list = Vec::new();
    if let Some(cache_base) = dirs::cache_dir() {
        let ff_cache = cache_base.join("mozilla/firefox");
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
    if let Some(data) = dirs::data_dir() {
        log_roots.push(data.join("xorg"));
    }
    let var_log = PathBuf::from("/var/log");
    if var_log.is_dir() {
        log_roots.push(var_log);
    }
    log_roots
}

pub fn get_thumbnail_cache_files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Some(cache_base) = dirs::cache_dir() {
        let thumbs_dir = cache_base.join("thumbnails");
        if thumbs_dir.is_dir() {
            for entry in WalkDir::new(&thumbs_dir)
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
    (dirs_list, &["deb", "rpm", "appimage", "tar.gz", "tar.xz"])
}

pub fn get_dev_tools_cache_dirs() -> Vec<PathBuf> {
    let mut dev_dirs = Vec::new();
    if let Some(home) = dirs::home_dir() {
        dev_dirs.push(home.join(".npm"));
        dev_dirs.push(home.join(".cargo/registry/cache"));
    }
    if let Some(cache_base) = dirs::cache_dir() {
        dev_dirs.push(cache_base.join("pip"));
    }
    if let Some(config_base) = dirs::config_dir() {
        dev_dirs.push(config_base.join("Code/Cache"));
        dev_dirs.push(config_base.join("Code/CachedData"));
        dev_dirs.push(config_base.join("Code/logs"));
    }
    dev_dirs
}

pub fn get_consumer_apps_cache_dirs() -> Vec<PathBuf> {
    let mut app_dirs = Vec::new();
    if let Some(config_base) = dirs::config_dir() {
        let discord = config_base.join("discord");
        app_dirs.push(discord.join("Cache"));
        app_dirs.push(discord.join("Code Cache"));
        app_dirs.push(discord.join("GPUCache"));
        app_dirs.push(config_base.join("spotify/Storage"));
    }
    if let Some(cache_base) = dirs::cache_dir() {
        app_dirs.push(cache_base.join("spotify/Storage"));
    }
    app_dirs
}

fn get_freedesktop_trash_dir() -> PathBuf {
    if let Some(data) = dirs::data_dir() {
        data.join("Trash")
    } else if let Some(home) = dirs::home_dir() {
        home.join(".local/share/Trash")
    } else {
        PathBuf::from("/tmp")
    }
}

pub fn query_recycle_bin() -> (u64, u64) {
    let trash_files = get_freedesktop_trash_dir().join("files");
    if !trash_files.is_dir() {
        return (0, 0);
    }

    let mut total_bytes = 0u64;
    let mut total_items = 0u64;

    for entry in WalkDir::new(&trash_files)
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
    let trash_dir = get_freedesktop_trash_dir();
    let files_dir = trash_dir.join("files");
    let info_dir = trash_dir.join("info");

    let mut errors = Vec::new();

    if files_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&files_dir) {
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
    }

    if info_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&info_dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                let _ = fs::remove_file(&p);
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
    let target = if path.is_file() {
        path.parent().unwrap_or(path)
    } else {
        path
    };
    let _ = Command::new("xdg-open").arg(target).spawn();
}

pub fn open_file(path: &Path) {
    let _ = Command::new("xdg-open").arg(path).spawn();
}
