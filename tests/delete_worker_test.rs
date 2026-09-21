use dupesweeper::actions::{ActionKind, DeleteProgressEvent, DeleteWorker};
use dupesweeper::cleanup::categories::{
    CategoryScanResult, CleanupCategoryId, CleanupCategoryDef, CleanupItem, SafetyLevel,
};
use dupesweeper::cleanup::executor::{CleanupProgressEvent, CleanupExecutor};
use dupesweeper::scanner::FileItem;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

fn create_temp_test_dir(prefix: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();
    let unique_id = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    dir.push(format!("dupesweeper_worker_{}_{}", prefix, unique_id));
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn test_delete_worker_background_execution() {
    let dir = create_temp_test_dir("bg_exec");
    let mut files = Vec::new();

    for i in 0..10 {
        let p = dir.join(format!("file_{}.txt", i));
        fs::write(&p, format!("test content {}", i)).unwrap();
        files.push(FileItem {
            path: p,
            size: 14,
            created: Some(SystemTime::now()),
            modified: Some(SystemTime::now()),
            is_selected: true,
            is_recommended_keep: false,
            is_sensitive: false,
            template_match: None,
        });
    }

    let cancel_flag = Arc::new(AtomicBool::new(false));
    let start_instant = Instant::now();

    // Start background deletion
    let rx = DeleteWorker::start(
        ActionKind::PermanentDelete,
        files.clone(),
        Arc::clone(&cancel_flag),
    );

    // Verify it returned immediately (non-blocking, < 50ms)
    let elapsed = start_instant.elapsed();
    assert!(
        elapsed < Duration::from_millis(50),
        "DeleteWorker::start must return immediately without blocking UI thread, took {:?}",
        elapsed
    );

    // Collect all progress events
    let mut started = false;
    let mut progress_count = 0;
    let mut finished = false;

    while let Ok(event) = rx.recv_timeout(Duration::from_secs(5)) {
        match event {
            DeleteProgressEvent::Started { total } => {
                assert_eq!(total, 10);
                started = true;
            }
            DeleteProgressEvent::Progress { current, total, .. } => {
                assert_eq!(total, 10);
                assert!(current >= 1 && current <= 10);
                progress_count += 1;
            }
            DeleteProgressEvent::Finished { report } => {
                assert_eq!(report.successful, 10);
                assert_eq!(report.failed, 0);
                assert_eq!(report.bytes_freed, 140);
                finished = true;
                break;
            }
            DeleteProgressEvent::Cancelled { .. } => {
                panic!("Should not be cancelled");
            }
        }
    }

    assert!(started, "Started event must be received");
    assert!(progress_count > 0, "Progress events must be received");
    assert!(finished, "Finished event must be received");

    // All files should be deleted
    for item in &files {
        assert!(!item.path.exists(), "File {:?} should be deleted", item.path);
    }

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_delete_worker_cancellation() {
    let dir = create_temp_test_dir("cancel");
    let mut files = Vec::new();

    // Create 30 files
    for i in 0..30 {
        let p = dir.join(format!("cancel_file_{}.txt", i));
        fs::write(&p, format!("cancel content {}", i)).unwrap();
        files.push(FileItem {
            path: p,
            size: 16,
            created: Some(SystemTime::now()),
            modified: Some(SystemTime::now()),
            is_selected: true,
            is_recommended_keep: false,
            is_sensitive: false,
            template_match: None,
        });
    }

    let cancel_flag = Arc::new(AtomicBool::new(false));

    // Cancel immediately before / right as it starts
    cancel_flag.store(true, Ordering::Relaxed);

    let rx = DeleteWorker::start(
        ActionKind::PermanentDelete,
        files.clone(),
        Arc::clone(&cancel_flag),
    );

    let mut cancelled = false;
    while let Ok(event) = rx.recv_timeout(Duration::from_secs(5)) {
        match event {
            DeleteProgressEvent::Cancelled { partial_report } => {
                // Must stop early without deleting all files
                assert!(
                    partial_report.successful < 30,
                    "Should have cancelled before all 30 files were deleted"
                );
                cancelled = true;
                break;
            }
            DeleteProgressEvent::Finished { .. } => {
                panic!("Should receive Cancelled instead of Finished");
            }
            _ => {}
        }
    }

    assert!(cancelled, "Must receive DeleteProgressEvent::Cancelled");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_cleanup_executor_cancellation() {
    let dir = create_temp_test_dir("cleanup_cancel");
    let mut items = Vec::new();

    for i in 0..20 {
        let p = dir.join(format!("cleanup_temp_{}.tmp", i));
        fs::write(&p, b"temp data").unwrap();
        items.push(CleanupItem {
            path: p,
            size: 9,
            modified: Some(SystemTime::now()),
        });
    }

    let def = CleanupCategoryDef {
        id: CleanupCategoryId::TempFiles,
        title: "Test Cancel Temp",
        description: "Test",
        icon: "📁",
        safety: SafetyLevel::Safe,
        default_enabled: true,
        warning: None,
    };

    let mut cat_result = CategoryScanResult::new(&def);
    cat_result.items = items;
    cat_result.total_bytes = 180;
    cat_result.is_enabled = true;

    let cancel_flag = Arc::new(AtomicBool::new(true)); // Pre-cancelled

    let rx = CleanupExecutor::start(vec![cat_result], cancel_flag);

    let mut got_event = false;
    while let Ok(event) = rx.recv_timeout(Duration::from_secs(5)) {
        match event {
            CleanupProgressEvent::Cancelled { partial_report } => {
                assert!(partial_report.successful_deleted < 20);
                got_event = true;
                break;
            }
            CleanupProgressEvent::Finished { .. } => {
                panic!("Should not finish when cancelled");
            }
            _ => {}
        }
    }

    assert!(got_event, "Must receive CleanupProgressEvent::Cancelled");
    let _ = fs::remove_dir_all(&dir);
}

