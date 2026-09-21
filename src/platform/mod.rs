use std::path::{Path, PathBuf};

pub mod linux;
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
use windows as imp;

#[cfg(target_os = "linux")]
use linux as imp;

#[cfg(target_os = "macos")]
use macos as imp;

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
use linux as imp;

/// Returns temporary file directories for the current OS.
pub fn get_temp_dirs() -> Vec<PathBuf> {
    imp::get_temp_dirs()
}

/// Returns browser cache directories (Chrome, Chromium, Edge) for the current OS.
pub fn get_chrome_cache_dirs() -> Vec<PathBuf> {
    imp::get_chrome_cache_dirs()
}

/// Returns Firefox profile cache directories for the current OS.
pub fn get_firefox_cache_dirs() -> Vec<PathBuf> {
    imp::get_firefox_cache_dirs()
}

/// Returns log and crash dump directories for the current OS.
pub fn get_old_logs_dirs() -> Vec<PathBuf> {
    imp::get_old_logs_dirs()
}

/// Returns cached thumbnail files for the current OS.
pub fn get_thumbnail_cache_files() -> Vec<PathBuf> {
    imp::get_thumbnail_cache_files()
}

/// Returns download directory paths and installer file extensions for the current OS.
pub fn get_downloads_installer_patterns() -> (Vec<PathBuf>, &'static [&'static str]) {
    imp::get_downloads_installer_patterns()
}

/// Returns developer tools cache directories (npm, pip, VS Code, Cargo) for the current OS.
pub fn get_dev_tools_cache_dirs() -> Vec<PathBuf> {
    imp::get_dev_tools_cache_dirs()
}

/// Returns consumer apps cache directories (Discord, Spotify) for the current OS.
pub fn get_consumer_apps_cache_dirs() -> Vec<PathBuf> {
    imp::get_consumer_apps_cache_dirs()
}

/// Queries the Recycle Bin / Trash for total bytes and total item count across the current OS.
pub fn query_recycle_bin() -> (u64, u64) {
    imp::query_recycle_bin()
}

/// Empties the Recycle Bin / Trash on the current OS.
pub fn empty_recycle_bin() -> Result<(), String> {
    imp::empty_recycle_bin()
}

/// Opens the operating system's native file manager and selects or reveals the given path.
pub fn open_in_file_manager(path: &Path) {
    imp::open_in_file_manager(path);
}

/// Opens a file using the operating system's default application.
pub fn open_file(path: &Path) {
    imp::open_file(path);
}
