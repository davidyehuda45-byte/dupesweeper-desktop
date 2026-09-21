use crossbeam_channel::{unbounded, Receiver, Sender};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use super::categories::{CategoryScanResult, CleanupCategoryId};
use super::recycle_bin;
use crate::actions::logger::AuditLogger;

#[derive(Debug, Clone)]
pub struct CleanupExecutionReport {
    pub total_categories: usize,
    pub total_files_processed: usize,
    pub successful_deleted: usize,
    pub skipped_locked: usize,
    pub bytes_freed: u64,
    pub log_path: PathBuf,
}

#[derive(Debug, Clone)]
pub enum CleanupProgressEvent {
    Started {
        total: usize,
    },
    Progress {
        current: usize,
        total: usize,
        current_item: String,
        category: String,
        bytes_freed: u64,
    },
    Finished {
        report: CleanupExecutionReport,
    },
    Cancelled {
        partial_report: CleanupExecutionReport,
    },
}

pub struct CleanupExecutor;

impl CleanupExecutor {
    /// Spawns cleanup on a background thread and sends progress events non-blockingly.
    pub fn start(
        categories: Vec<CategoryScanResult>,
        cancel_flag: Arc<AtomicBool>,
    ) -> Receiver<CleanupProgressEvent> {
        let (tx, rx) = unbounded();
        thread::spawn(move || {
            Self::run_cleanup(categories, cancel_flag, tx);
        });
        rx
    }

    /// Background execution loop with cancellation checks.
    pub fn run_cleanup(
        categories: Vec<CategoryScanResult>,
        cancel_flag: Arc<AtomicBool>,
        tx: Sender<CleanupProgressEvent>,
    ) {
        let total_items: usize = categories
            .iter()
            .filter(|c| c.is_enabled)
            .map(|c| c.items.len())
            .sum();

        let _ = tx.send(CleanupProgressEvent::Started { total: total_items });

        let logger = AuditLogger::new().ok();
        let mut total_categories = 0;
        let mut total_files_processed = 0;
        let mut successful_deleted = 0;
        let mut skipped_locked = 0;
        let mut bytes_freed = 0;
        let mut is_cancelled = false;

        for cat in &categories {
            if !cat.is_enabled {
                continue;
            }

            if cancel_flag.load(Ordering::Relaxed) {
                is_cancelled = true;
                break;
            }

            total_categories += 1;

            if cat.id == CleanupCategoryId::RecycleBin {
                // Empty Recycle Bin
                let _ = tx.send(CleanupProgressEvent::Progress {
                    current: total_files_processed + 1,
                    total: total_items,
                    current_item: "Mengosongkan Recycle Bin...".to_string(),
                    category: cat.title.to_string(),
                    bytes_freed,
                });

                if let Some(ref l) = logger {
                    l.log_action("Empty Recycle Bin", Path::new("Recycle Bin"), true, "");
                }
                match recycle_bin::empty_recycle_bin() {
                    Ok(_) => {
                        successful_deleted += cat.items.len();
                        bytes_freed += cat.total_bytes;
                    }
                    Err(err) => {
                        skipped_locked += 1;
                        if let Some(ref l) = logger {
                            l.log_action(
                                "Empty Recycle Bin",
                                Path::new("Recycle Bin"),
                                false,
                                &err,
                            );
                        }
                    }
                }
                continue;
            }

            for item in &cat.items {
                if cancel_flag.load(Ordering::Relaxed) {
                    is_cancelled = true;
                    break;
                }

                let path = &item.path;
                let file_name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| path.to_string_lossy().to_string());

                total_files_processed += 1;

                let _ = tx.send(CleanupProgressEvent::Progress {
                    current: total_files_processed,
                    total: total_items,
                    current_item: file_name,
                    category: cat.title.to_string(),
                    bytes_freed,
                });

                if !path.exists() {
                    continue;
                }

                match fs::remove_file(path) {
                    Ok(_) => {
                        successful_deleted += 1;
                        bytes_freed += item.size;
                        if let Some(ref l) = logger {
                            l.log_action("General Cleanup (Delete)", path, true, cat.title);
                        }
                    }
                    Err(err) => {
                        skipped_locked += 1;
                        if let Some(ref l) = logger {
                            let note = format!("Skipped locked/in-use: {}", err);
                            l.log_action("General Cleanup (Skip)", path, false, &note);
                        }
                    }
                }
            }

            if is_cancelled {
                break;
            }
        }

        let log_file_path = if let Some(ref l) = logger {
            l.log_summary(
                total_files_processed,
                successful_deleted,
                skipped_locked,
                bytes_freed,
            );
            l.log_path.clone()
        } else {
            PathBuf::from("logs/dupesweeper_audit.txt")
        };

        let report = CleanupExecutionReport {
            total_categories,
            total_files_processed,
            successful_deleted,
            skipped_locked,
            bytes_freed,
            log_path: log_file_path,
        };

        if is_cancelled {
            let _ = tx.send(CleanupProgressEvent::Cancelled {
                partial_report: report,
            });
        } else {
            let _ = tx.send(CleanupProgressEvent::Finished { report });
        }
    }

    /// Synchronous execution method retained for backward compatibility / tests.
    pub fn execute(categories: &[CategoryScanResult]) -> CleanupExecutionReport {
        let cancel_flag = Arc::new(AtomicBool::new(false));
        let (tx, rx) = unbounded();
        Self::run_cleanup(categories.to_vec(), cancel_flag, tx);

        while let Ok(event) = rx.recv() {
            match event {
                CleanupProgressEvent::Finished { report }
                | CleanupProgressEvent::Cancelled {
                    partial_report: report,
                } => return report,
                _ => {}
            }
        }

        CleanupExecutionReport {
            total_categories: 0,
            total_files_processed: 0,
            successful_deleted: 0,
            skipped_locked: 0,
            bytes_freed: 0,
            log_path: PathBuf::from("logs/dupesweeper_audit.txt"),
        }
    }
}
