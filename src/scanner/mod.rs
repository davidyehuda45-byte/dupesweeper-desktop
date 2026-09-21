pub mod exclude_list;
pub mod group;
pub mod hasher;
pub mod protected_list;
pub mod template_fingerprints;
pub mod walker;

use crossbeam_channel::Sender;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

pub use group::{DuplicateGroup, FileItem, SelectionStrategy};
pub use template_fingerprints::{TemplateCategory, TemplateFingerprint};
pub use walker::WalkerConfig;

#[derive(Debug, Clone)]
pub enum ScanProgress {
    Walking {
        files_scanned: usize,
        current_path: String,
    },
    PartialHashing {
        current: usize,
        total: usize,
        current_file: String,
    },
    FullHashing {
        current: usize,
        total: usize,
        current_file: String,
        bytes_hashed: u64,
        total_candidate_bytes: u64,
    },
    Finished {
        groups: Vec<DuplicateGroup>,
        total_files_scanned: usize,
        folders_skipped: usize,
        total_duplicates: usize,
        wasted_bytes: u64,
        elapsed: Duration,
    },
    Cancelled,
    Error(String),
}

pub struct Scanner;

impl Scanner {
    pub fn run(
        config: WalkerConfig,
        cancel_flag: Arc<AtomicBool>,
        progress_tx: Sender<ScanProgress>,
    ) {
        let start_time = Instant::now();

        // -------------------------------------------------------------
        // STAGE 1: Directory Traversal & Size Grouping
        // -------------------------------------------------------------
        let (size_map, total_scanned, folders_skipped) = walker::walk_and_group_by_size(
            &config,
            &cancel_flag,
            &|path: &Path, count: usize| {
                if count % 100 == 0 || count < 100 {
                    let _ = progress_tx.send(ScanProgress::Walking {
                        files_scanned: count,
                        current_path: path.to_string_lossy().to_string(),
                    });
                }
            },
        );

        if cancel_flag.load(Ordering::Relaxed) {
            let _ = progress_tx.send(ScanProgress::Cancelled);
            return;
        }

        let total_candidate_files: usize = size_map.values().map(|v| v.len()).sum();
        if total_candidate_files == 0 {
            let _ = progress_tx.send(ScanProgress::Finished {
                groups: Vec::new(),
                total_files_scanned: total_scanned,
                folders_skipped,
                total_duplicates: 0,
                wasted_bytes: 0,
                elapsed: start_time.elapsed(),
            });
            return;
        }

        // -------------------------------------------------------------
        // STAGE 2: 4KB Partial Hashing
        // -------------------------------------------------------------
        let partial_processed = Arc::new(AtomicUsize::new(0));

        // Flatten candidates
        let all_candidates: Vec<walker::RawFileInfo> =
            size_map.into_values().flatten().collect();

        struct PartialResult {
            info: walker::RawFileInfo,
            partial_hash: blake3::Hash,
            is_full: bool,
        }

        let partial_results: Vec<PartialResult> = all_candidates
            .into_par_iter()
            .filter_map(|info| {
                if cancel_flag.load(Ordering::Relaxed) {
                    return None;
                }

                let hash = match hasher::compute_partial_hash(&info.path) {
                    Ok(h) => h,
                    Err(_) => return None, // Skip unreadable
                };

                let current = partial_processed.fetch_add(1, Ordering::Relaxed) + 1;
                if current % 50 == 0 || current == total_candidate_files {
                    let _ = progress_tx.send(ScanProgress::PartialHashing {
                        current,
                        total: total_candidate_files,
                        current_file: info.path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                    });
                }

                let is_full = info.size <= hasher::PARTIAL_CHUNK_SIZE as u64;

                Some(PartialResult {
                    info,
                    partial_hash: hash,
                    is_full,
                })
            })
            .collect();

        if cancel_flag.load(Ordering::Relaxed) {
            let _ = progress_tx.send(ScanProgress::Cancelled);
            return;
        }

        // Group by (size, partial_hash)
        let mut partial_groups: HashMap<(u64, [u8; 32]), Vec<PartialResult>> = HashMap::new();
        for item in partial_results {
            partial_groups
                .entry((item.info.size, *item.partial_hash.as_bytes()))
                .or_default()
                .push(item);
        }

        // Retain groups with >= 2 files
        partial_groups.retain(|_, v| v.len() >= 2);

        // -------------------------------------------------------------
        // STAGE 3: Full Hashing
        // -------------------------------------------------------------
        let full_candidates: Vec<PartialResult> =
            partial_groups.into_values().flatten().collect();

        let total_full_candidates = full_candidates.len();
        let total_candidate_bytes: u64 = full_candidates.iter().map(|f| f.info.size).sum();

        let full_processed = Arc::new(AtomicUsize::new(0));
        let total_bytes_hashed = Arc::new(AtomicU64::new(0));

        struct FullResult {
            info: walker::RawFileInfo,
            full_hash: String,
        }

        let full_results: Vec<FullResult> = full_candidates
            .into_par_iter()
            .filter_map(|item| {
                if cancel_flag.load(Ordering::Relaxed) {
                    return None;
                }

                let full_hash = if item.is_full {
                    item.partial_hash.to_hex().to_string()
                } else {
                    let bytes_ref = Arc::clone(&total_bytes_hashed);
                    match hasher::compute_full_hash(
                        &item.info.path,
                        Some(&cancel_flag),
                        Some(&|bytes| {
                            bytes_ref.fetch_add(bytes, Ordering::Relaxed);
                        }),
                    ) {
                        Ok(h) => h,
                        Err(_) => return None,
                    }
                };

                let current = full_processed.fetch_add(1, Ordering::Relaxed) + 1;
                if current % 10 == 0 || current == total_full_candidates {
                    let _ = progress_tx.send(ScanProgress::FullHashing {
                        current,
                        total: total_full_candidates,
                        current_file: item.info.path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                        bytes_hashed: total_bytes_hashed.load(Ordering::Relaxed),
                        total_candidate_bytes,
                    });
                }

                Some(FullResult {
                    info: item.info,
                    full_hash,
                })
            })
            .collect();

        if cancel_flag.load(Ordering::Relaxed) {
            let _ = progress_tx.send(ScanProgress::Cancelled);
            return;
        }

        // Group by full hash
        let mut full_groups: HashMap<String, Vec<walker::RawFileInfo>> = HashMap::new();
        for item in full_results {
            full_groups
                .entry(item.full_hash)
                .or_default()
                .push(item.info);
        }

        // Retain groups with >= 2 files (true duplicates)
        full_groups.retain(|_, v| v.len() >= 2);

        // Convert to DuplicateGroup structures
        let mut final_groups: Vec<DuplicateGroup> = full_groups
            .into_iter()
            .map(|(hash, files)| {
                let size = files[0].size;
                let template_match = template_fingerprints::match_template(&hash).cloned();
                let file_items = files
                    .into_iter()
                    .map(|f| {
                        let is_sensitive = protected_list::is_sensitive_file(&f.path);
                        FileItem::new(
                            f.path,
                            f.size,
                            f.created,
                            f.modified,
                            is_sensitive,
                            template_match.clone(),
                        )
                    })
                    .collect();
                DuplicateGroup::new(hash, size, file_items)
            })
            .collect();

        // Sort groups by wasted space descending (largest duplicates first)
        final_groups.sort_by(|a, b| b.total_wasted_bytes().cmp(&a.total_wasted_bytes()));

        let total_duplicates: usize = final_groups.iter().map(|g| g.files.len() - 1).sum();
        let wasted_bytes: u64 = final_groups.iter().map(|g| g.total_wasted_bytes()).sum();

        let _ = progress_tx.send(ScanProgress::Finished {
            groups: final_groups,
            total_files_scanned: total_scanned,
            folders_skipped,
            total_duplicates,
            wasted_bytes,
            elapsed: start_time.elapsed(),
        });
    }
}
