use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[repr(C)]
pub struct SHQUERYRBINFO {
    pub cb_size: u32,
    pub i64_size: i64,
    pub i64_num_items: i64,
}

#[link(name = "shell32")]
extern "system" {
    pub fn SHQueryRecycleBinW(
        psz_root_path: *const u16,
        p_sh_query_rb_info: *mut SHQUERYRBINFO,
    ) -> i32;

    pub fn SHEmptyRecycleBinW(
        hwnd: *mut std::ffi::c_void,
        psz_root_path: *const u16,
        dw_flags: u32,
    ) -> i32;
}

pub const SHERB_NOCONFIRMATION: u32 = 0x00000001;
pub const SHERB_NOPROGRESSUI: u32 = 0x00000002;
pub const SHERB_NOSOUND: u32 = 0x00000004;

pub fn get_temp_dirs() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Ok(temp) = env::var("TEMP") {
        roots.push(PathBuf::from(temp));
    }
    if let Ok(local) = env::var("LOCALAPPDATA") {
        roots.push(PathBuf::from(local).join("Temp"));
    } else if let Some(local) = dirs::data_local_dir() {
        roots.push(local.join("Temp"));
    }
    roots.push(PathBuf::from(r"C:\Windows\Temp"));
    roots
}

pub fn get_chrome_cache_dirs() -> Vec<PathBuf> {
    let mut dirs_list = Vec::new();
    let local = env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .ok()
        .or_else(dirs::data_local_dir);

    if let Some(local_path) = local {
        let chrome_default = local_path.join(r"Google\Chrome\User Data\Default");
        dirs_list.push(chrome_default.join("Cache"));
        dirs_list.push(chrome_default.join("Code Cache"));
        dirs_list.push(chrome_default.join("GPUCache"));
        dirs_list.push(chrome_default.join(r"Service Worker\CacheStorage"));
        dirs_list.push(chrome_default.join(r"Service Worker\ScriptCache"));

        let edge_default = local_path.join(r"Microsoft\Edge\User Data\Default");
        dirs_list.push(edge_default.join("Cache"));
        dirs_list.push(edge_default.join("Code Cache"));
        dirs_list.push(edge_default.join("GPUCache"));
        dirs_list.push(edge_default.join(r"Service Worker\CacheStorage"));
        dirs_list.push(edge_default.join(r"Service Worker\ScriptCache"));
    }
    dirs_list
}

pub fn get_firefox_cache_dirs() -> Vec<PathBuf> {
    let mut dirs_list = Vec::new();
    let appdata = env::var("APPDATA")
        .map(PathBuf::from)
        .ok()
        .or_else(dirs::config_dir);

    if let Some(appdata_path) = appdata {
        let ff_profiles = appdata_path.join(r"Mozilla\Firefox\Profiles");
        if ff_profiles.is_dir() {
            if let Ok(entries) = fs::read_dir(&ff_profiles) {
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
    let local = env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .ok()
        .or_else(dirs::data_local_dir);

    if let Some(local_path) = local {
        log_roots.push(local_path.join(r"Microsoft\Windows\WER"));
        log_roots.push(local_path.join("CrashDumps"));
    }
    log_roots
}

pub fn get_thumbnail_cache_files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    let local = env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .ok()
        .or_else(dirs::data_local_dir);

    if let Some(local_path) = local {
        let explorer_dir = local_path.join(r"Microsoft\Windows\Explorer");
        if explorer_dir.is_dir() {
            if let Ok(entries) = fs::read_dir(&explorer_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        let name_lower = name.to_lowercase();
                        if name_lower.starts_with("thumbcache_") && name_lower.ends_with(".db") {
                            files.push(path);
                        }
                    }
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
    } else if let Ok(profile) = env::var("USERPROFILE") {
        let p = PathBuf::from(profile).join("Downloads");
        if p.is_dir() {
            dirs_list.push(p);
        }
    }
    (dirs_list, &["exe", "msi"])
}

pub fn get_dev_tools_cache_dirs() -> Vec<PathBuf> {
    let mut dev_dirs = Vec::new();
    let appdata = env::var("APPDATA")
        .map(PathBuf::from)
        .ok()
        .or_else(dirs::config_dir);
    if let Some(appdata_path) = appdata {
        dev_dirs.push(appdata_path.join("npm-cache"));
        dev_dirs.push(appdata_path.join(r"Code\Cache"));
        dev_dirs.push(appdata_path.join(r"Code\CachedData"));
        dev_dirs.push(appdata_path.join(r"Code\logs"));
    }

    let local = env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .ok()
        .or_else(dirs::data_local_dir);
    if let Some(local_path) = local {
        dev_dirs.push(local_path.join(r"pip\Cache"));
    }

    if let Some(home) = dirs::home_dir() {
        dev_dirs.push(home.join(r".cargo\registry\cache"));
    } else if let Ok(profile) = env::var("USERPROFILE") {
        dev_dirs.push(PathBuf::from(profile).join(r".cargo\registry\cache"));
    }

    dev_dirs
}

pub fn get_consumer_apps_cache_dirs() -> Vec<PathBuf> {
    let mut app_dirs = Vec::new();
    let appdata = env::var("APPDATA")
        .map(PathBuf::from)
        .ok()
        .or_else(dirs::config_dir);
    if let Some(appdata_path) = appdata {
        let discord = appdata_path.join("discord");
        app_dirs.push(discord.join("Cache"));
        app_dirs.push(discord.join("Code Cache"));
        app_dirs.push(discord.join("GPUCache"));
        app_dirs.push(appdata_path.join(r"Spotify\Storage"));
    }

    let local = env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .ok()
        .or_else(dirs::data_local_dir);
    if let Some(local_path) = local {
        app_dirs.push(local_path.join(r"Spotify\Storage"));
    }

    app_dirs
}

pub fn query_recycle_bin() -> (u64, u64) {
    let mut info = SHQUERYRBINFO {
        cb_size: std::mem::size_of::<SHQUERYRBINFO>() as u32,
        i64_size: 0,
        i64_num_items: 0,
    };

    unsafe {
        // Passing NULL queries all drives at once in a single call according to Microsoft Win32 API docs
        let res = SHQueryRecycleBinW(std::ptr::null(), &mut info);
        if res == 0 {
            return (
                if info.i64_size > 0 { info.i64_size as u64 } else { 0 },
                if info.i64_num_items > 0 { info.i64_num_items as u64 } else { 0 },
            );
        }
    }

    (0, 0)
}

pub fn empty_recycle_bin() -> Result<(), String> {
    let flags = SHERB_NOCONFIRMATION | SHERB_NOPROGRESSUI | SHERB_NOSOUND;
    unsafe {
        let res = SHEmptyRecycleBinW(std::ptr::null_mut(), std::ptr::null(), flags);
        if res == 0 {
            Ok(())
        } else {
            Err(format!("SHEmptyRecycleBinW error code: {:#x}", res))
        }
    }
}

pub fn open_in_file_manager(path: &Path) {
    let _ = Command::new("explorer")
        .arg(format!("/select,{}", path.display()))
        .spawn();
}

pub fn open_file(path: &Path) {
    let _ = Command::new("cmd")
        .args(["/c", "start", "", &path.to_string_lossy()])
        .spawn();
}
