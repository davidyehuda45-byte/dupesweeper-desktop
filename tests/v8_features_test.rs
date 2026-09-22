//! Tests for the v8 feature set: export, history/undo logic, auto-scan
//! settings, and the folder size analyzer.

use dupesweeper::analyzer::{AnalyzerProgress, FolderAnalyzer};
use dupesweeper::cleanup::categories::{CategoryScanResult, CleanupCategoryId, CleanupItem, SafetyLevel};
use dupesweeper::export;
use dupesweeper::history::{HistoryActionKind, HistoryEntry, TrashRef};
use dupesweeper::scanner::{DuplicateGroup, FileItem};
use dupesweeper::settings::{AppSettings, AutoScanInterval};
use dupesweeper::ui::icons::IconKind;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

fn temp_subdir(prefix: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();
    let unique_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    dir.push(format!("dupesweeper_v8_{}_{}", prefix, unique_id));
    fs::create_dir_all(&dir).unwrap();
    dir
}

// ---------------------------------------------------------------------------
// Export
// ---------------------------------------------------------------------------

fn sample_duplicate_groups() -> Vec<DuplicateGroup> {
    let f1 = FileItem::new(PathBuf::from("/tmp/a.txt"), 100, None, None, false, None);
    let f2 = FileItem::new(PathBuf::from("/tmp/b.txt"), 100, None, None, false, None);
    vec![DuplicateGroup::new("hash123".to_string(), 100, vec![f1, f2])]
}

#[test]
fn test_export_duplicates_csv_and_json_roundtrip() {
    let dir = temp_subdir("export_dup");
    let groups = sample_duplicate_groups();

    let csv_path = dir.join("out.csv");
    export::export_duplicates_csv(&groups, &csv_path).unwrap();
    let csv_content = fs::read_to_string(&csv_path).unwrap();
    assert!(csv_content.contains("group_hash"));
    assert!(csv_content.contains("hash123"));
    assert!(csv_content.contains("a.txt") || csv_content.contains("a.txt".replace('/', "\\").as_str()));
    // header + 2 file rows
    assert_eq!(csv_content.trim_end().lines().count(), 3);

    let json_path = dir.join("out.json");
    export::export_duplicates_json(&groups, &json_path).unwrap();
    let json_content = fs::read_to_string(&json_path).unwrap();
    assert!(json_content.contains("\"total_groups\": 1"));
    assert!(json_content.contains("hash123"));

    fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_export_cleanup_csv_and_json_roundtrip() {
    let dir = temp_subdir("export_cleanup");
    let def = dupesweeper::cleanup::categories::CleanupCategoryDef {
        id: CleanupCategoryId::TempFiles,
        title: "Temp",
        description: "desc",
        icon: IconKind::Folder,
        safety: SafetyLevel::Safe,
        default_enabled: true,
        warning: None,
    };
    let mut cat = CategoryScanResult::new(&def);
    cat.items.push(CleanupItem {
        path: PathBuf::from("/tmp/junk.log"),
        size: 500,
        modified: None,
    });
    cat.total_bytes = 500;
    let categories = vec![cat];

    let csv_path = dir.join("cleanup.csv");
    export::export_cleanup_csv(&categories, &csv_path).unwrap();
    let csv_content = fs::read_to_string(&csv_path).unwrap();
    assert!(csv_content.contains("junk.log"));
    assert!(csv_content.contains("500"));

    let json_path = dir.join("cleanup.json");
    export::export_cleanup_json(&categories, &json_path).unwrap();
    let json_content = fs::read_to_string(&json_path).unwrap();
    assert!(json_content.contains("\"total_bytes\": 500"));

    fs::remove_dir_all(&dir).ok();
}

// ---------------------------------------------------------------------------
// History / Undo logic (pure, no real Recycle Bin access)
// ---------------------------------------------------------------------------

#[test]
fn test_history_entry_is_restorable_only_for_recycle_bin_with_refs() {
    let refs = vec![TrashRef {
        id: "id1".to_string(),
        name: "file.txt".to_string(),
        original_parent: "/tmp".to_string(),
        time_deleted: 0,
    }];

    let restorable = HistoryEntry::new("Src", HistoryActionKind::RecycleBin, 1, 100, refs.clone());
    assert_eq!(
        restorable.is_restorable(),
        dupesweeper::history::is_restore_supported(),
        "Restorable only when the platform supports trash introspection"
    );

    let no_refs = HistoryEntry::new("Src", HistoryActionKind::RecycleBin, 1, 100, Vec::new());
    assert!(!no_refs.is_restorable(), "No trash refs captured => not restorable");

    let permanent = HistoryEntry::new("Src", HistoryActionKind::PermanentDelete, 1, 100, refs.clone());
    assert!(!permanent.is_restorable(), "Permanent delete can never be undone");

    let quarantine = HistoryEntry::new("Src", HistoryActionKind::Quarantine, 1, 100, refs.clone());
    assert!(!quarantine.is_restorable(), "Quarantine undo is not wired to trash refs");

    let mut already_restored = HistoryEntry::new("Src", HistoryActionKind::RecycleBin, 1, 100, refs);
    already_restored.restored = true;
    assert!(!already_restored.is_restorable(), "Already-restored entries can't be undone again");
}

#[test]
fn test_history_action_kind_labels_are_distinct() {
    let labels = [
        HistoryActionKind::RecycleBin.label(),
        HistoryActionKind::Quarantine.label(),
        HistoryActionKind::PermanentDelete.label(),
    ];
    assert_eq!(labels.len(), 3);
    assert_ne!(labels[0], labels[1]);
    assert_ne!(labels[1], labels[2]);
}

// ---------------------------------------------------------------------------
// Settings
// ---------------------------------------------------------------------------

#[test]
fn test_auto_scan_interval_seconds_mapping() {
    assert_eq!(AutoScanInterval::Off.as_secs(), None);
    assert_eq!(AutoScanInterval::Min30.as_secs(), Some(30 * 60));
    assert_eq!(AutoScanInterval::Hour1.as_secs(), Some(60 * 60));
    assert_eq!(AutoScanInterval::Hour6.as_secs(), Some(6 * 60 * 60));
    assert_eq!(AutoScanInterval::Hour24.as_secs(), Some(24 * 60 * 60));
}

#[test]
fn test_app_settings_serde_roundtrip_in_memory() {
    let mut settings = AppSettings::default();
    settings.auto_scan_interval = AutoScanInterval::Hour6;
    settings.last_scan_roots = vec![PathBuf::from("/some/folder")];

    let json = serde_json::to_string(&settings).unwrap();
    let restored: AppSettings = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.auto_scan_interval, AutoScanInterval::Hour6);
    assert_eq!(restored.last_scan_roots, vec![PathBuf::from("/some/folder")]);
}

// ---------------------------------------------------------------------------
// Folder size analyzer
// ---------------------------------------------------------------------------

#[test]
fn test_folder_analyzer_computes_child_sizes_and_recurses_into_subfolders() {
    let dir = temp_subdir("analyzer");
    fs::write(dir.join("top_level.bin"), vec![0u8; 1000]).unwrap();

    let sub = dir.join("subfolder");
    fs::create_dir_all(&sub).unwrap();
    fs::write(sub.join("a.bin"), vec![0u8; 2000]).unwrap();
    fs::write(sub.join("b.bin"), vec![0u8; 3000]).unwrap();

    let cancel = Arc::new(AtomicBool::new(false));
    let rx = FolderAnalyzer::start(dir.clone(), cancel);

    let mut entries;
    loop {
        match rx.recv().unwrap() {
            AnalyzerProgress::Finished { entries: e, total_size, .. } => {
                entries = e;
                assert_eq!(total_size, 1000 + 2000 + 3000);
                break;
            }
            AnalyzerProgress::Error(e) => panic!("Analyzer error: {}", e),
            _ => continue,
        }
    }

    entries.sort_by(|a, b| a.name.cmp(&b.name));
    let file_entry = entries.iter().find(|e| e.name == "top_level.bin").unwrap();
    assert!(!file_entry.is_dir);
    assert_eq!(file_entry.size, 1000);

    let dir_entry = entries.iter().find(|e| e.name == "subfolder").unwrap();
    assert!(dir_entry.is_dir);
    assert_eq!(dir_entry.size, 5000, "Subfolder size must aggregate its files recursively");
    assert_eq!(dir_entry.file_count, 2);

    fs::remove_dir_all(&dir).ok();
}
