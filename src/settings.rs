//! Persisted user preferences, including the in-app auto-scan scheduler.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutoScanInterval {
    Off,
    Min30,
    Hour1,
    Hour6,
    Hour24,
}

impl AutoScanInterval {
    pub const ALL: [AutoScanInterval; 5] = [
        AutoScanInterval::Off,
        AutoScanInterval::Min30,
        AutoScanInterval::Hour1,
        AutoScanInterval::Hour6,
        AutoScanInterval::Hour24,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            AutoScanInterval::Off => "Nonaktif",
            AutoScanInterval::Min30 => "Tiap 30 menit",
            AutoScanInterval::Hour1 => "Tiap 1 jam",
            AutoScanInterval::Hour6 => "Tiap 6 jam",
            AutoScanInterval::Hour24 => "Tiap 24 jam",
        }
    }

    pub fn as_secs(&self) -> Option<u64> {
        match self {
            AutoScanInterval::Off => None,
            AutoScanInterval::Min30 => Some(30 * 60),
            AutoScanInterval::Hour1 => Some(60 * 60),
            AutoScanInterval::Hour6 => Some(6 * 60 * 60),
            AutoScanInterval::Hour24 => Some(24 * 60 * 60),
        }
    }
}

impl Default for AutoScanInterval {
    fn default() -> Self {
        AutoScanInterval::Off
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppSettings {
    pub auto_scan_interval: AutoScanInterval,
    /// Folders remembered from the last scan so auto-scan knows what to re-scan.
    pub last_scan_roots: Vec<PathBuf>,
}

impl AppSettings {
    fn file_path() -> Option<PathBuf> {
        let mut dir = dirs::config_dir()?;
        dir.push("DupeSweeper");
        std::fs::create_dir_all(&dir).ok()?;
        dir.push("settings.json");
        Some(dir)
    }

    pub fn load() -> Self {
        let Some(path) = Self::file_path() else {
            return Self::default();
        };
        let Ok(content) = std::fs::read_to_string(&path) else {
            return Self::default();
        };
        serde_json::from_str(&content).unwrap_or_default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::file_path() {
            if let Ok(json) = serde_json::to_string_pretty(self) {
                let _ = std::fs::write(path, json);
            }
        }
    }
}
