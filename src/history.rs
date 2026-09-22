//! Cleanup session history with best-effort Undo for Recycle Bin actions.

use chrono::Local;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A reference to a single item sitting in the OS trash, captured right after
/// a delete so it can later be located again for restore.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrashRef {
    pub id: String,
    pub name: String,
    pub original_parent: String,
    pub time_deleted: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HistoryActionKind {
    RecycleBin,
    Quarantine,
    PermanentDelete,
}

impl HistoryActionKind {
    pub fn label(&self) -> &'static str {
        match self {
            HistoryActionKind::RecycleBin => "Recycle Bin",
            HistoryActionKind::Quarantine => "Karantina",
            HistoryActionKind::PermanentDelete => "Hapus Permanen",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// Millis since epoch; also serves as a unique id.
    pub id: u64,
    pub timestamp_label: String,
    pub source: String,
    pub action: HistoryActionKind,
    pub item_count: usize,
    pub total_bytes: u64,
    pub trash_refs: Vec<TrashRef>,
    pub restored: bool,
}

impl HistoryEntry {
    pub fn new(
        source: impl Into<String>,
        action: HistoryActionKind,
        item_count: usize,
        total_bytes: u64,
        trash_refs: Vec<TrashRef>,
    ) -> Self {
        let now = Local::now();
        Self {
            id: now.timestamp_millis().max(0) as u64,
            timestamp_label: now.format("%d %b %Y, %H:%M:%S").to_string(),
            source: source.into(),
            action,
            item_count,
            total_bytes,
            trash_refs,
            restored: false,
        }
    }

    pub fn is_restorable(&self) -> bool {
        self.action == HistoryActionKind::RecycleBin
            && !self.restored
            && !self.trash_refs.is_empty()
            && is_restore_supported()
    }
}

pub struct HistoryStore;

impl HistoryStore {
    const MAX_ENTRIES: usize = 200;

    fn file_path() -> Option<PathBuf> {
        let mut dir = dirs::data_dir()?;
        dir.push("DupeSweeper");
        std::fs::create_dir_all(&dir).ok()?;
        dir.push("history.json");
        Some(dir)
    }

    pub fn load() -> Vec<HistoryEntry> {
        let Some(path) = Self::file_path() else {
            return Vec::new();
        };
        let Ok(content) = std::fs::read_to_string(&path) else {
            return Vec::new();
        };
        serde_json::from_str(&content).unwrap_or_default()
    }

    fn save(entries: &[HistoryEntry]) {
        if let Some(path) = Self::file_path() {
            if let Ok(json) = serde_json::to_string_pretty(entries) {
                let _ = std::fs::write(path, json);
            }
        }
    }

    pub fn add_entry(entry: HistoryEntry) {
        let mut entries = Self::load();
        entries.insert(0, entry);
        entries.truncate(Self::MAX_ENTRIES);
        Self::save(&entries);
    }

    pub fn mark_restored(id: u64) {
        let mut entries = Self::load();
        if let Some(e) = entries.iter_mut().find(|e| e.id == id) {
            e.restored = true;
        }
        Self::save(&entries);
    }

    pub fn clear_all() {
        Self::save(&[]);
    }
}

/// Whether Recycle Bin introspection (list/restore) is supported on this OS.
/// The underlying `trash` crate only exposes `os_limited` on Windows and
/// Freedesktop-Trash-compliant Unix (i.e. not macOS).
pub const fn is_restore_supported() -> bool {
    cfg!(any(windows, all(unix, not(target_os = "macos"))))
}

#[cfg(any(windows, all(unix, not(target_os = "macos"))))]
pub fn capture_trash_refs(paths: &[PathBuf]) -> Vec<TrashRef> {
    use std::collections::HashSet;

    if paths.is_empty() {
        return Vec::new();
    }

    let wanted: HashSet<&std::path::Path> = paths.iter().map(|p| p.as_path()).collect();
    let items = match trash::os_limited::list() {
        Ok(items) => items,
        Err(_) => return Vec::new(),
    };

    let mut matched: Vec<_> = items
        .into_iter()
        .filter(|item| wanted.contains(item.original_path().as_path()))
        .collect();

    // Keep only the most-recently-trashed entry per original path (in case a
    // path was deleted, restored, and deleted again previously).
    matched.sort_by(|a, b| b.time_deleted.cmp(&a.time_deleted));
    let mut seen = HashSet::new();
    matched.retain(|item| seen.insert(item.original_path()));

    matched
        .into_iter()
        .map(|item| TrashRef {
            id: item.id.to_string_lossy().to_string(),
            name: item.name.to_string_lossy().to_string(),
            original_parent: item.original_parent.to_string_lossy().to_string(),
            time_deleted: item.time_deleted,
        })
        .collect()
}

#[cfg(not(any(windows, all(unix, not(target_os = "macos")))))]
pub fn capture_trash_refs(_paths: &[PathBuf]) -> Vec<TrashRef> {
    Vec::new()
}

#[cfg(any(windows, all(unix, not(target_os = "macos"))))]
pub fn restore_refs(refs: &[TrashRef]) -> Result<(), String> {
    use std::ffi::OsString;

    let items: Vec<trash::TrashItem> = refs
        .iter()
        .map(|r| trash::TrashItem {
            id: OsString::from(&r.id),
            name: OsString::from(&r.name),
            original_parent: PathBuf::from(&r.original_parent),
            time_deleted: r.time_deleted,
        })
        .collect();

    trash::os_limited::restore_all(items).map_err(|e| e.to_string())
}

#[cfg(not(any(windows, all(unix, not(target_os = "macos")))))]
pub fn restore_refs(_refs: &[TrashRef]) -> Result<(), String> {
    Err("Undo tidak didukung di platform ini (macOS)".to_string())
}
