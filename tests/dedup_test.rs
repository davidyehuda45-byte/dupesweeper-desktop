use crossbeam_channel::unbounded;
use dupesweeper::actions::{ActionExecutor, ActionKind};
use dupesweeper::scanner::group::{FileItem, SelectionStrategy};
use dupesweeper::scanner::hasher::{compute_full_hash, compute_partial_hash};
use dupesweeper::scanner::{DuplicateGroup, ScanProgress, Scanner, WalkerConfig};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::SystemTime;

fn create_temp_test_dir(prefix: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();
    let unique_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    dir.push(format!("dupesweeper_test_{}_{}", prefix, unique_id));
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn test_hasher_partial_and_full() {
    let dir = create_temp_test_dir("hasher");
    let file1 = dir.join("file1.dat");
    let file2 = dir.join("file2.dat");

    let data1 = vec![0xAB; 8192];
    let mut data2 = vec![0xAB; 8192];

    fs::write(&file1, &data1).unwrap();
    fs::write(&file2, &data2).unwrap();

    let partial1 = compute_partial_hash(&file1).unwrap();
    let partial2 = compute_partial_hash(&file2).unwrap();
    assert_eq!(partial1, partial2);

    let full1 = compute_full_hash(&file1, None, None).unwrap();
    let full2 = compute_full_hash(&file2, None, None).unwrap();
    assert_eq!(full1, full2);

    // Modify after 4KB (partial hash should still match, but full hash must differ)
    data2[5000] = 0xCD;
    fs::write(&file2, &data2).unwrap();

    let partial2_mod = compute_partial_hash(&file2).unwrap();
    assert_eq!(partial1, partial2_mod, "Partial hashes should still match for first 4KB");

    let full2_mod = compute_full_hash(&file2, None, None).unwrap();
    assert_ne!(full1, full2_mod, "Full hashes must differ when byte 5000 is modified");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_end_to_end_scanner_detection() {
    let dir = create_temp_test_dir("scan_e2e");
    let sub1 = dir.join("folder1");
    let sub2 = dir.join("folder2");
    fs::create_dir_all(&sub1).unwrap();
    fs::create_dir_all(&sub2).unwrap();

    // 2 duplicates
    let dupe1 = sub1.join("image.png");
    let dupe2 = sub2.join("image_copy.png");
    let content_dupe = vec![42u8; 10000];
    fs::write(&dupe1, &content_dupe).unwrap();
    fs::write(&dupe2, &content_dupe).unwrap();

    // 1 unique file of same size
    let unique_same_size = sub1.join("other.png");
    let mut content_diff = vec![42u8; 10000];
    content_diff[9999] = 99;
    fs::write(&unique_same_size, &content_diff).unwrap();

    // 1 file of different size
    let unique_small = sub2.join("small.txt");
    fs::write(&unique_small, b"hello world").unwrap();

    let config = WalkerConfig {
        roots: vec![dir.clone()],
        exclude_dirs: Vec::new(),
        exclude_extensions: Vec::new(),
        min_size: 1,
        max_size: None,
        include_hidden: false,
        scan_all_folders: false,
    };

    let cancel = Arc::new(AtomicBool::new(false));
    let (tx, rx) = unbounded();

    Scanner::run(config, cancel, tx);

    let mut found_groups: Vec<DuplicateGroup> = Vec::new();
    while let Ok(msg) = rx.recv() {
        if let ScanProgress::Finished { groups, .. } = msg {
            found_groups = groups;
            break;
        }
    }

    assert_eq!(found_groups.len(), 1, "Must find exactly 1 duplicate group");
    let group = &found_groups[0];
    assert_eq!(group.files.len(), 2, "Duplicate group must contain 2 files");
    assert_eq!(group.file_size, 10000);

    let paths: Vec<PathBuf> = group.files.iter().map(|f| f.path.clone()).collect();
    assert!(paths.contains(&dupe1));
    assert!(paths.contains(&dupe2));
    assert!(!paths.contains(&unique_same_size));
    assert!(!paths.contains(&unique_small));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_selection_strategies() {
    let now = SystemTime::now();
    let earlier = now - std::time::Duration::from_secs(3600);
    let earliest = now - std::time::Duration::from_secs(7200);

    let file_earliest = FileItem::new(
        PathBuf::from("C:/short/a.txt"),
        500,
        Some(earliest),
        Some(earliest),
        false,
        None,
    );
    let file_earlier = FileItem::new(
        PathBuf::from("C:/medium/path/b.txt"),
        500,
        Some(earlier),
        Some(earlier),
        false,
        None,
    );
    let file_now = FileItem::new(
        PathBuf::from("C:/very/long/nested/path/to/folder/c.txt"),
        500,
        Some(now),
        Some(now),
        false,
        None,
    );

    let mut group = DuplicateGroup::new(
        "testhash".to_string(),
        500,
        vec![file_earlier, file_now, file_earliest],
    );

    // Keep Oldest
    group.apply_strategy(SelectionStrategy::KeepOldest);
    let kept = group.files.iter().find(|f| f.is_recommended_keep).unwrap();
    assert_eq!(kept.path, PathBuf::from("C:/short/a.txt"));
    assert_eq!(group.selected_count(), 2);

    // Keep Newest
    group.apply_strategy(SelectionStrategy::KeepNewest);
    let kept_new = group.files.iter().find(|f| f.is_recommended_keep).unwrap();
    assert_eq!(kept_new.path, PathBuf::from("C:/very/long/nested/path/to/folder/c.txt"));
    assert_eq!(group.selected_count(), 2);

    // Keep Shortest Path
    group.apply_strategy(SelectionStrategy::KeepShortestPath);
    let kept_short = group.files.iter().find(|f| f.is_recommended_keep).unwrap();
    assert_eq!(kept_short.path, PathBuf::from("C:/short/a.txt"));
}

#[test]
fn test_action_quarantine_execution() {
    let dir = create_temp_test_dir("quarantine_test");
    let file_to_quarantine = dir.join("trash_candidate.txt");
    fs::write(&file_to_quarantine, b"remove me").unwrap();

    let quarantine_dir = dir.join("quarantine_target");

    let item = FileItem::new(file_to_quarantine.clone(), 9, None, None, false, None);
    let report = ActionExecutor::execute(
        &ActionKind::Quarantine(quarantine_dir.clone()),
        &[item],
    );

    assert_eq!(report.successful, 1);
    assert_eq!(report.failed, 0);
    assert_eq!(report.bytes_freed, 9);
    assert!(!file_to_quarantine.exists(), "Original file should no longer exist at original path");

    let quarantined_file = quarantine_dir.join("trash_candidate.txt");
    assert!(quarantined_file.exists(), "File must exist in quarantine folder");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_zero_byte_files_filtered_by_default() {
    let dir = create_temp_test_dir("zero_byte_test");
    let zero1 = dir.join("zero1.txt");
    let zero2 = dir.join("zero2.txt");
    fs::write(&zero1, b"").unwrap();
    fs::write(&zero2, b"").unwrap();

    let config = WalkerConfig {
        roots: vec![dir.clone()],
        exclude_dirs: Vec::new(),
        exclude_extensions: Vec::new(),
        min_size: 1, // default min_size = 1 byte (filters out 0 bytes)
        max_size: None,
        include_hidden: false,
        scan_all_folders: false,
    };

    let cancel = Arc::new(AtomicBool::new(false));
    let (tx, rx) = unbounded();

    Scanner::run(config, cancel, tx);

    let mut found_groups: Vec<DuplicateGroup> = Vec::new();
    while let Ok(msg) = rx.recv() {
        if let ScanProgress::Finished { groups, .. } = msg {
            found_groups = groups;
            break;
        }
    }

    assert_eq!(found_groups.len(), 0, "0-byte files should be skipped when min_size >= 1");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_three_way_duplicates_across_nested_subfolders() {
    let dir = create_temp_test_dir("three_way_nested");
    let d1 = dir.join("a").join("b");
    let d2 = dir.join("c").join("d");
    let d3 = dir.join("e");
    fs::create_dir_all(&d1).unwrap();
    fs::create_dir_all(&d2).unwrap();
    fs::create_dir_all(&d3).unwrap();

    let f1 = d1.join("doc.pdf");
    let f2 = d2.join("doc_backup.pdf");
    let f3 = d3.join("doc_final.pdf");
    let content = b"Duplicate PDF Document Content Test 1234567890";

    fs::write(&f1, content).unwrap();
    fs::write(&f2, content).unwrap();
    fs::write(&f3, content).unwrap();

    let config = WalkerConfig {
        roots: vec![dir.clone()],
        exclude_dirs: Vec::new(),
        exclude_extensions: Vec::new(),
        min_size: 1,
        max_size: None,
        include_hidden: false,
        scan_all_folders: false,
    };

    let cancel = Arc::new(AtomicBool::new(false));
    let (tx, rx) = unbounded();

    Scanner::run(config, cancel, tx);

    let mut found_groups: Vec<DuplicateGroup> = Vec::new();
    while let Ok(msg) = rx.recv() {
        if let ScanProgress::Finished { groups, .. } = msg {
            found_groups = groups;
            break;
        }
    }

    assert_eq!(found_groups.len(), 1);
    assert_eq!(found_groups[0].files.len(), 3);
    assert_eq!(found_groups[0].total_wasted_bytes(), (content.len() * 2) as u64);

    let _ = fs::remove_dir_all(&dir);
}

// =========================================================================
// PRD v2 Tests: Excluded Directories & Protected Sensitive Files
// =========================================================================

#[test]
fn test_excluded_directories_are_skipped() {
    let dir = create_temp_test_dir("excluded_dirs");
    
    // Create folders that match universal exclude list
    let node_modules = dir.join("node_modules").join("some_pkg");
    let vendor = dir.join("vendor").join("composer");
    let git = dir.join(".git").join("hooks");
    let target = dir.join("target").join("debug");
    let dist = dir.join("dist");
    let src = dir.join("src");

    fs::create_dir_all(&node_modules).unwrap();
    fs::create_dir_all(&vendor).unwrap();
    fs::create_dir_all(&git).unwrap();
    fs::create_dir_all(&target).unwrap();
    fs::create_dir_all(&dist).unwrap();
    fs::create_dir_all(&src).unwrap();

    // Create identical files in excluded folders and one in src
    let content = b"identical duplicate content 12345";
    fs::write(node_modules.join("index.js"), content).unwrap();
    fs::write(vendor.join("autoload.php"), content).unwrap();
    fs::write(git.join("pre-commit"), content).unwrap();
    fs::write(target.join("output.bin"), content).unwrap();
    fs::write(dist.join("bundle.js"), content).unwrap();
    fs::write(src.join("main.js"), content).unwrap();

    let config = WalkerConfig {
        roots: vec![dir.clone()],
        exclude_dirs: Vec::new(),
        exclude_extensions: Vec::new(),
        min_size: 1,
        max_size: None,
        include_hidden: true, // even if hidden is true, .git and excluded dirs must still be skipped
        scan_all_folders: false, // default
    };

    let cancel = Arc::new(AtomicBool::new(false));
    let (tx, rx) = unbounded();

    Scanner::run(config, cancel, tx);

    let mut found_groups: Vec<DuplicateGroup> = Vec::new();
    let mut folders_skipped_count = 0;

    while let Ok(msg) = rx.recv() {
        if let ScanProgress::Finished { groups, folders_skipped, .. } = msg {
            found_groups = groups;
            folders_skipped_count = folders_skipped;
            break;
        }
    }

    // Since all duplicate copies were inside excluded directories, src/main.js is unique!
    assert_eq!(found_groups.len(), 0, "No duplicates should be found because excluded dirs were skipped");
    assert!(folders_skipped_count >= 5, "Should have recorded skipped folders");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_sensitive_files_default_unchecked() {
    let dir = create_temp_test_dir("sensitive_files");
    let proj1 = dir.join("app1");
    let proj2 = dir.join("app2");
    fs::create_dir_all(&proj1).unwrap();
    fs::create_dir_all(&proj2).unwrap();

    // Create duplicate .env files
    let env1 = proj1.join(".env");
    let env2 = proj2.join(".env.production");
    let key1 = proj1.join("server.key");
    let key2 = proj2.join("backup.key");

    let env_content = b"DATABASE_URL=postgres://user:pass@localhost/db";
    let key_content = b"-----BEGIN PRIVATE KEY-----\nMIIEvgIBADANBgkqhkiG9w0BAQEFAASC";

    fs::write(&env1, env_content).unwrap();
    fs::write(&env2, env_content).unwrap();
    fs::write(&key1, key_content).unwrap();
    fs::write(&key2, key_content).unwrap();

    let config = WalkerConfig {
        roots: vec![dir.clone()],
        exclude_dirs: Vec::new(),
        exclude_extensions: Vec::new(),
        min_size: 1,
        max_size: None,
        include_hidden: true,
        scan_all_folders: false,
    };

    let cancel = Arc::new(AtomicBool::new(false));
    let (tx, rx) = unbounded();

    Scanner::run(config, cancel, tx);

    let mut found_groups: Vec<DuplicateGroup> = Vec::new();
    while let Ok(msg) = rx.recv() {
        if let ScanProgress::Finished { groups, .. } = msg {
            found_groups = groups;
            break;
        }
    }

    assert_eq!(found_groups.len(), 2, "Must find both duplicate groups (.env and .key)");

    for mut group in found_groups {
        for file in &group.files {
            assert!(
                file.is_sensitive,
                "File {:?} must be flagged as sensitive",
                file.path
            );
            assert!(
                !file.is_selected,
                "Sensitive file {:?} must default to is_selected = false",
                file.path
            );
        }

        // Test that switching strategies NEVER selects sensitive files
        group.apply_strategy(SelectionStrategy::KeepOldest);
        assert_eq!(group.selected_count(), 0, "No sensitive files should be selected under KeepOldest");

        group.apply_strategy(SelectionStrategy::KeepNewest);
        assert_eq!(group.selected_count(), 0, "No sensitive files should be selected under KeepNewest");

        group.apply_strategy(SelectionStrategy::KeepShortestPath);
        assert_eq!(group.selected_count(), 0, "No sensitive files should be selected under KeepShortestPath");

        // Test select_all_duplicates skips sensitive files
        group.select_all_duplicates();
        assert_eq!(group.selected_count(), 0, "select_all_duplicates must not select sensitive files");
    }

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_scan_all_toggle_overrides_exclude() {
    let dir = create_temp_test_dir("scan_all_toggle");
    let nm = dir.join("node_modules").join("dep");
    let src = dir.join("src");
    fs::create_dir_all(&nm).unwrap();
    fs::create_dir_all(&src).unwrap();

    let content = b"duplicate test file across node_modules";
    fs::write(nm.join("file.txt"), content).unwrap();
    fs::write(src.join("file.txt"), content).unwrap();

    // Case 1: scan_all_folders = false -> excluded
    let config_excluded = WalkerConfig {
        roots: vec![dir.clone()],
        exclude_dirs: Vec::new(),
        exclude_extensions: Vec::new(),
        min_size: 1,
        max_size: None,
        include_hidden: true,
        scan_all_folders: false,
    };
    let cancel1 = Arc::new(AtomicBool::new(false));
    let (tx1, rx1) = unbounded();
    Scanner::run(config_excluded, cancel1, tx1);
    let mut groups1 = Vec::new();
    while let Ok(msg) = rx1.recv() {
        if let ScanProgress::Finished { groups, .. } = msg {
            groups1 = groups;
            break;
        }
    }
    assert_eq!(groups1.len(), 0, "With scan_all = false, node_modules duplicate must be skipped");

    // Case 2: scan_all_folders = true -> scanned
    let config_scan_all = WalkerConfig {
        roots: vec![dir.clone()],
        exclude_dirs: Vec::new(),
        exclude_extensions: Vec::new(),
        min_size: 1,
        max_size: None,
        include_hidden: true,
        scan_all_folders: true, // OVERRIDE!
    };
    let cancel2 = Arc::new(AtomicBool::new(false));
    let (tx2, rx2) = unbounded();
    Scanner::run(config_scan_all, cancel2, tx2);
    let mut groups2 = Vec::new();
    while let Ok(msg) = rx2.recv() {
        if let ScanProgress::Finished { groups, .. } = msg {
            groups2 = groups;
            break;
        }
    }
    assert_eq!(groups2.len(), 1, "With scan_all = true, node_modules duplicate must be detected");
    assert_eq!(groups2[0].files.len(), 2);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_large_dataset_virtualized_performance() {
    use std::time::Instant;

    // Simulate 50,000 duplicate groups (150,000 files)
    let total_groups = 50_000;
    let mut groups: Vec<DuplicateGroup> = Vec::with_capacity(total_groups);

    for i in 0..total_groups {
        let f1 = FileItem::new(
            PathBuf::from(format!("C:/data/drive_d/photos/img_{}.jpg", i)),
            2_500_000,
            None,
            None,
            false,
            None,
        );
        let f2 = FileItem::new(
            PathBuf::from(format!("C:/backup/photos_copy/img_{}.jpg", i)),
            2_500_000,
            None,
            None,
            false,
            None,
        );
        let f3 = FileItem::new(
            PathBuf::from(format!("C:/external_hd/archive/img_{}.jpg", i)),
            2_500_000,
            None,
            None,
            false,
            None,
        );
        groups.push(DuplicateGroup::new(
            format!("blake3_hash_{:08x}", i),
            2_500_000,
            vec![f1, f2, f3],
        ));
    }

    assert_eq!(groups.len(), 50_000);

    // Measure time to construct virtual row mapping with collapsed groups (default)
    let start_collapsed = Instant::now();
    let mut display_rows_collapsed = Vec::with_capacity(groups.len());
    for (g_idx, _group) in groups.iter().enumerate() {
        display_rows_collapsed.push(g_idx);
    }
    let elapsed_collapsed = start_collapsed.elapsed();

    assert_eq!(display_rows_collapsed.len(), 50_000);
    assert!(
        elapsed_collapsed.as_millis() < 50,
        "Initial collapsed row setup for 50,000 groups must be under 50ms, took: {:?}",
        elapsed_collapsed
    );

    // Simulate expanding 500 groups
    let mut expanded_set = std::collections::HashSet::new();
    for i in 0..500 {
        expanded_set.insert(i);
    }

    let start_expanded = Instant::now();
    let mut flat_count = 0;
    for (g_idx, group) in groups.iter().enumerate() {
        flat_count += 1; // header
        if expanded_set.contains(&g_idx) {
            flat_count += group.files.len();
        }
    }
    let elapsed_expanded = start_expanded.elapsed();

    // 50,000 headers + 500 * 3 files = 51,500 rows
    assert_eq!(flat_count, 51_500);
    assert!(
        elapsed_expanded.as_millis() < 50,
        "Flattening with expanded groups must be under 50ms, took: {:?}",
        elapsed_expanded
    );
}

// =========================================================================
// PRD v3 Tests: Template Fingerprint Detection (Kategori C & D)
// =========================================================================

#[test]
fn test_documentation_template_match_is_auto_selected() {
    use dupesweeper::scanner::template_fingerprints::{match_template, TemplateCategory};

    // Laravel README hash
    let laravel_readme_hash = "c35d26f139fc16e2aaaa252e5d9eb98ae120275b594027d5282d5b1be67fee3b";
    let matched = match_template(laravel_readme_hash);
    assert!(matched.is_some(), "Laravel README hash must match template fingerprint");
    let fp = matched.unwrap();
    assert_eq!(fp.category, TemplateCategory::Documentation);
    assert_eq!(fp.framework, "Laravel");

    // Create duplicate group with this template match
    let f1 = FileItem::new(
        PathBuf::from("C:/proj1/README.md"),
        2583,
        None,
        None,
        false,
        Some(fp.clone()),
    );
    let f2 = FileItem::new(
        PathBuf::from("C:/proj2/README.md"),
        2583,
        None,
        None,
        false,
        Some(fp.clone()),
    );

    let group = DuplicateGroup::new(
        laravel_readme_hash.to_string(),
        2583,
        vec![f1, f2],
    );

    // Kategori C (Dokumentasi) IS safe to delete and SHOULD be auto-selected according to strategy
    assert_eq!(group.files[0].is_recommended_keep, true);
    assert_eq!(group.files[1].is_recommended_keep, false);
    assert_eq!(
        group.files[1].is_selected, true,
        "Documentation template duplicates must be auto-selected for deletion"
    );
}

#[test]
fn test_functional_template_match_is_protected_from_auto_selection() {
    use dupesweeper::scanner::template_fingerprints::{match_template, TemplateCategory};

    // Laravel users table migration hash
    let users_migration_hash = "e4b76df2d2048e6a373a8f88001a06930f08de587011a84300386679bd08b082";
    let matched = match_template(users_migration_hash);
    assert!(matched.is_some(), "Users table migration hash must match template fingerprint");
    let fp = matched.unwrap();
    assert_eq!(fp.category, TemplateCategory::Functional);
    assert_eq!(fp.framework, "Laravel");

    let f1 = FileItem::new(
        PathBuf::from("C:/proj1/database/migrations/0001_01_01_000000_create_users_table.php"),
        1100,
        None,
        None,
        false,
        Some(fp.clone()),
    );
    let f2 = FileItem::new(
        PathBuf::from("C:/proj2/database/migrations/0001_01_01_000000_create_users_table.php"),
        1100,
        None,
        None,
        false,
        Some(fp.clone()),
    );

    let mut group = DuplicateGroup::new(
        users_migration_hash.to_string(),
        1100,
        vec![f1, f2],
    );

    // Kategori D (Fungsional) is PROTECTED from auto-selection
    assert_eq!(group.files[0].is_recommended_keep, true);
    assert_eq!(group.files[1].is_recommended_keep, false);
    assert_eq!(
        group.files[1].is_selected, false,
        "Functional framework templates must NOT be auto-selected for deletion"
    );
    assert_eq!(group.selected_count(), 0);

    // Test other strategies
    group.apply_strategy(SelectionStrategy::KeepNewest);
    assert_eq!(group.selected_count(), 0, "KeepNewest must not select functional templates");

    group.apply_strategy(SelectionStrategy::KeepShortestPath);
    assert_eq!(group.selected_count(), 0, "KeepShortestPath must not select functional templates");

    // Test select_all_duplicates ignores functional template protected files
    group.select_all_duplicates();
    assert_eq!(group.selected_count(), 0, "select_all_duplicates must not select functional templates");
}

#[test]
fn test_end_to_end_template_fingerprint_scanner() {
    use dupesweeper::scanner::template_fingerprints::TemplateCategory;

    let dir = create_temp_test_dir("template_e2e");
    let p1 = dir.join("proj1");
    let p2 = dir.join("proj2");
    fs::create_dir_all(&p1).unwrap();
    fs::create_dir_all(&p2).unwrap();

    // Vite React react.svg content (Documentation template)
    // Hash: 679fc68d782e2ec79add7891c0181421808eecff388149ad3cd73e9bf41ab113
    let fixture_react_svg = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/react.svg");
    let react_svg_bytes = fs::read(&fixture_react_svg).expect("Must read fixture react.svg");

    let svg1 = p1.join("react.svg");
    let svg2 = p2.join("react.svg");
    fs::write(&svg1, &react_svg_bytes).unwrap();
    fs::write(&svg2, &react_svg_bytes).unwrap();

    // Laravel routes/web.php content (Functional template)
    let fixture_routes = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/web.php");
    let routes_bytes = fs::read(&fixture_routes).expect("Must read fixture web.php");

    let r1 = p1.join("web.php");
    let r2 = p2.join("web.php");
    fs::write(&r1, &routes_bytes).unwrap();
    fs::write(&r2, &routes_bytes).unwrap();

    let config = WalkerConfig {
        roots: vec![dir.clone()],
        exclude_dirs: Vec::new(),
        exclude_extensions: Vec::new(),
        min_size: 1,
        max_size: None,
        include_hidden: false,
        scan_all_folders: false,
    };

    let cancel = Arc::new(AtomicBool::new(false));
    let (tx, rx) = unbounded();

    Scanner::run(config, cancel, tx);

    let mut found_groups = Vec::new();
    while let Ok(msg) = rx.recv() {
        if let ScanProgress::Finished { groups, .. } = msg {
            found_groups = groups;
            break;
        }
    }

    assert_eq!(found_groups.len(), 2, "Must find 2 duplicate groups");

    for group in &found_groups {
        let tm = group.files[0].template_match.as_ref().expect("Group must have template_match");
        match tm.category {
            TemplateCategory::Documentation => {
                assert_eq!(tm.framework, "Vite + React");
                assert_eq!(group.selected_count(), 1, "Documentation template duplicate must be selected");
            }
            TemplateCategory::Functional => {
                assert_eq!(tm.framework, "Laravel");
                assert_eq!(group.selected_count(), 0, "Functional template duplicate must NOT be selected");
            }
        }
    }

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_modified_framework_file_no_longer_matches_template() {
    use dupesweeper::scanner::template_fingerprints::match_template;

    let fixture_routes = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/web.php");
    let mut routes_bytes = fs::read(&fixture_routes).expect("Must read fixture web.php");

    // Modify by adding a custom route
    routes_bytes.extend_from_slice(b"\nRoute::get('/api/custom', fn() => 'custom');\n");

    let mut hasher = blake3::Hasher::new();
    hasher.update(&routes_bytes);
    let modified_hash = hasher.finalize().to_hex().to_string();

    let matched = match_template(&modified_hash);
    assert!(matched.is_none(), "Modified file must NOT match template fingerprint");
}

