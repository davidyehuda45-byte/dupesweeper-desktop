use crossbeam_channel::{unbounded, Receiver, Sender};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use super::logger::AuditLogger;
use super::{ActionKind, ActionReport};
use crate::scanner::FileItem;

#[derive(Debug, Clone)]
pub enum DeleteProgressEvent {
    Started {
        total: usize,
    },
    Progress {
        current: usize,
        total: usize,
        current_file: String,
        bytes_freed: u64,
    },
    Finished {
        report: ActionReport,
    },
    Cancelled {
        partial_report: ActionReport,
    },
}

pub struct DeleteWorker;

impl DeleteWorker {
    /// Spawns deletion on a background thread and returns a non-blocking receiver.
    pub fn start(
        action: ActionKind,
        files_to_remove: Vec<FileItem>,
        cancel_flag: Arc<AtomicBool>,
    ) -> Receiver<DeleteProgressEvent> {
        let (tx, rx) = unbounded();
        thread::spawn(move || {
            Self::run_delete(action, files_to_remove, cancel_flag, tx);
        });
        rx
    }

    /// Internal loop executed on background thread.
    pub fn run_delete(
        action: ActionKind,
        files_to_remove: Vec<FileItem>,
        cancel_flag: Arc<AtomicBool>,
        tx: Sender<DeleteProgressEvent>,
    ) {
        let total = files_to_remove.len();
        let _ = tx.send(DeleteProgressEvent::Started { total });

        let logger = AuditLogger::new().ok();
        let mut successful = 0;
        let mut failed = 0;
        let mut bytes_freed = 0;
        let mut error_details = Vec::new();
        let mut is_cancelled = false;

        for (idx, item) in files_to_remove.iter().enumerate() {
            if cancel_flag.load(Ordering::Relaxed) {
                is_cancelled = true;
                break;
            }

            let path = &item.path;
            let file_name = item.file_name();

            // Notify UI of current item progress every item or every batch
            let _ = tx.send(DeleteProgressEvent::Progress {
                current: idx + 1,
                total,
                current_file: file_name,
                bytes_freed,
            });

            if !path.exists() {
                continue;
            }

            let result: Result<(), String> = match &action {
                ActionKind::RecycleBin => {
                    trash::delete(path).map_err(|e| format!("Recycle bin error: {}", e))
                }
                ActionKind::Quarantine(target_dir) => {
                    Self::move_to_quarantine(path, target_dir)
                }
                ActionKind::PermanentDelete => {
                    fs::remove_file(path).map_err(|e| format!("Delete error: {}", e))
                }
            };

            match result {
                Ok(_) => {
                    successful += 1;
                    bytes_freed += item.size;
                    if let Some(ref l) = logger {
                        l.log_action(action.display_name(), path, true, "");
                    }
                }
                Err(err) => {
                    failed += 1;
                    if let Some(ref l) = logger {
                        l.log_action(action.display_name(), path, false, &err);
                    }
                    error_details.push((path.clone(), err));
                }
            }
        }

        let log_file_path = if let Some(ref l) = logger {
            l.log_summary(total, successful, failed, bytes_freed);
            l.log_path.clone()
        } else {
            PathBuf::from("logs/dupesweeper_audit.txt")
        };

        let report = ActionReport {
            total_processed: if is_cancelled { successful + failed } else { total },
            successful,
            failed,
            bytes_freed,
            log_path: log_file_path,
            error_details,
        };

        if is_cancelled {
            let _ = tx.send(DeleteProgressEvent::Cancelled {
                partial_report: report,
            });
        } else {
            let _ = tx.send(DeleteProgressEvent::Finished { report });
        }
    }

    fn move_to_quarantine(source: &Path, quarantine_dir: &Path) -> Result<(), String> {
        if !quarantine_dir.exists() {
            fs::create_dir_all(quarantine_dir)
                .map_err(|e| format!("Cannot create quarantine dir: {}", e))?;
        }

        let file_name = source
            .file_name()
            .ok_or_else(|| "Invalid file name".to_string())?;

        let mut dest_path = quarantine_dir.join(file_name);
        let mut counter = 1;
        let stem = source.file_stem().unwrap_or_default().to_string_lossy();
        let ext = source.extension().and_then(|e| e.to_str()).unwrap_or("");

        while dest_path.exists() {
            let new_name = if ext.is_empty() {
                format!("{}_{}", stem, counter)
            } else {
                format!("{}_{}.{}", stem, counter, ext)
            };
            dest_path = quarantine_dir.join(new_name);
            counter += 1;
        }

        if fs::rename(source, &dest_path).is_err() {
            fs::copy(source, &dest_path)
                .map_err(|e| format!("Copy to quarantine failed: {}", e))?;
            fs::remove_file(source)
                .map_err(|e| format!("Failed removing original after copy: {}", e))?;
        }

        Ok(())
    }
}

