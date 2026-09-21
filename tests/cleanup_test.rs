use dupesweeper::cleanup::categories::{
    CategoryScanResult, CleanupCategoryId, CleanupCategoryDef, CleanupItem, SafetyLevel,
    CLEANUP_CATEGORIES,
};
use dupesweeper::ui::icons::IconKind;
use dupesweeper::cleanup::executor::CleanupExecutor;
use dupesweeper::cleanup::recycle_bin::query_recycle_bin;
use dupesweeper::cleanup::scanner::CleanupScanner;
use std::fs::{self, OpenOptions};
use std::path::PathBuf;
use std::time::SystemTime;

#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt;

fn create_temp_cleanup_dir(prefix: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();
    let unique_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    dir.push(format!("dupesweeper_cleanup_{}_{}", prefix, unique_id));
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// 8.5 Test 1: Simulating locked file does not crash and is recorded as skipped
#[test]
fn test_locked_file_skipped_gracefully() {
    let dir = create_temp_cleanup_dir("locked_file");
    let normal_file = dir.join("normal.tmp");
    let locked_sub = dir.join("locked_folder");
    fs::create_dir_all(&locked_sub).unwrap();
    let locked_file = locked_sub.join("locked.tmp");

    fs::write(&normal_file, b"normal temporary content 12345").unwrap();
    fs::write(&locked_file, b"locked file content cannot delete").unwrap();

    // Lock the file by opening it in exclusive mode (Windows) or read-only directory (Unix)
    let mut options = OpenOptions::new();
    options.read(true).write(true);
    #[cfg(windows)]
    options.share_mode(0);
    let _lock_handle = options.open(&locked_file).unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&locked_sub).unwrap().permissions();
        perms.set_mode(0o555);
        fs::set_permissions(&locked_sub, perms).unwrap();
    }

    let def = CleanupCategoryDef {
        id: CleanupCategoryId::TempFiles,
        title: "Test Temp Files",
        description: "Test",
        icon: IconKind::Folder,
        safety: SafetyLevel::Safe,
        default_enabled: true,
        warning: None,
    };

    let mut cat_result = CategoryScanResult::new(&def);
    cat_result.items.push(CleanupItem {
        path: normal_file.clone(),
        size: 30,
        modified: Some(SystemTime::now()),
    });
    cat_result.items.push(CleanupItem {
        path: locked_file.clone(),
        size: 33,
        modified: Some(SystemTime::now()),
    });
    cat_result.total_bytes = 63;
    cat_result.is_enabled = true;

    // Execute cleanup
    let report = CleanupExecutor::execute(&[cat_result]);

    // Verify:
    // 1. Did not crash or panic
    // 2. Normal file was successfully deleted
    assert!(!normal_file.exists(), "Normal file must be deleted");
    assert_eq!(report.successful_deleted, 1, "1 file must be successfully deleted");

    // 3. Locked file was skipped gracefully and remains
    assert_eq!(report.skipped_locked, 1, "1 locked file must be recorded as skipped");
    assert!(locked_file.exists(), "Locked file must still exist");

    // Release lock handle / restore permissions
    drop(_lock_handle);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(meta) = fs::metadata(&locked_sub) {
            let mut perms = meta.permissions();
            perms.set_mode(0o755);
            let _ = fs::set_permissions(&locked_sub, perms);
        }
    }
    let _ = fs::remove_dir_all(&dir);
}

/// 8.5 Test 2: Verify installer category defaults to unchecked (Safety = NeedsReview)
#[test]
fn test_installer_category_default_unchecked() {
    let installer_def = CLEANUP_CATEGORIES
        .iter()
        .find(|c| c.id == CleanupCategoryId::DownloadsInstallers)
        .expect("DownloadsInstallers category must exist");

    assert_eq!(
        installer_def.safety,
        SafetyLevel::NeedsReview,
        "DownloadsInstallers must have SafetyLevel::NeedsReview"
    );
    assert_eq!(
        installer_def.default_enabled, false,
        "DownloadsInstallers must default to unchecked (false)"
    );

    let result = CategoryScanResult::new(installer_def);
    assert_eq!(
        result.is_enabled, false,
        "CategoryScanResult for DownloadsInstallers must have is_enabled = false"
    );
}

/// 8.5 Test 3: Verify safe categories default to checked (Safety = Safe)
#[test]
fn test_safe_categories_default_checked() {
    let safe_category_ids = [
        CleanupCategoryId::TempFiles,
        CleanupCategoryId::BrowserCache,
        CleanupCategoryId::OldLogs,
        CleanupCategoryId::ThumbnailCache,
        CleanupCategoryId::RecycleBin,
        CleanupCategoryId::DevToolsCache,
        CleanupCategoryId::ConsumerAppsCache,
    ];

    for cat_id in safe_category_ids {
        let def = CLEANUP_CATEGORIES
            .iter()
            .find(|c| c.id == cat_id)
            .unwrap_or_else(|| panic!("Category {:?} must exist in CLEANUP_CATEGORIES", cat_id));

        assert_eq!(
            def.safety,
            SafetyLevel::Safe,
            "Category {:?} must have SafetyLevel::Safe",
            cat_id
        );
        assert_eq!(
            def.default_enabled, true,
            "Category {:?} must default to checked (true)",
            cat_id
        );

        let result = CategoryScanResult::new(def);
        assert_eq!(
            result.is_enabled, true,
            "CategoryScanResult for {:?} must be enabled by default",
            cat_id
        );
    }
}

/// 8.5 Test 4: Verify estimated size matches actual freed bytes after deletion
#[test]
fn test_size_calculation_matches_actual_deletion() {
    let dir = create_temp_cleanup_dir("size_calc_match");
    let file1 = dir.join("cache_1.tmp");
    let file2 = dir.join("cache_2.tmp");
    let file3 = dir.join("cache_3.tmp");

    let data1 = vec![0xAAu8; 15000]; // 15,000 bytes
    let data2 = vec![0xBBu8; 25000]; // 25,000 bytes
    let data3 = vec![0xCCu8; 10000]; // 10,000 bytes
    let expected_total_bytes = 15000 + 25000 + 10000;

    fs::write(&file1, &data1).unwrap();
    fs::write(&file2, &data2).unwrap();
    fs::write(&file3, &data3).unwrap();

    let def = CleanupCategoryDef {
        id: CleanupCategoryId::TempFiles,
        title: "Test Size Match",
        description: "Test",
        icon: IconKind::Folder,
        safety: SafetyLevel::Safe,
        default_enabled: true,
        warning: None,
    };

    let mut cat_result = CategoryScanResult::new(&def);
    CleanupScanner::collect_files_recursive(&dir, &mut cat_result, None, None);
    cat_result.total_bytes = cat_result.items.iter().map(|i| i.size).sum();

    // Verify dry-run size calculation
    assert_eq!(cat_result.items.len(), 3, "Must find exactly 3 files");
    assert_eq!(
        cat_result.total_bytes, expected_total_bytes as u64,
        "Estimated size must match total bytes of created test files"
    );

    // Execute deletion
    let report = CleanupExecutor::execute(&[cat_result]);

    // Verify actual freed bytes equals estimated size
    assert_eq!(report.successful_deleted, 3);
    assert_eq!(report.skipped_locked, 0);
    assert_eq!(
        report.bytes_freed, expected_total_bytes as u64,
        "Actual freed bytes must exactly match the pre-calculated estimated size"
    );

    // Files should no longer exist
    assert!(!file1.exists());
    assert!(!file2.exists());
    assert!(!file3.exists());

    let _ = fs::remove_dir_all(&dir);
}

/// Extra test: Windows Shell32 Recycle Bin query does not crash
#[test]
fn test_recycle_bin_query() {
    let (bytes, items) = query_recycle_bin();
    // Verify valid return values without panic
    println!("Recycle bin query: {} bytes, {} items", bytes, items);
}
