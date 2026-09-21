pub mod delete_worker;
pub mod logger;

pub use delete_worker::{DeleteProgressEvent, DeleteWorker};

use std::fs;
use std::path::{Path, PathBuf};
use crate::scanner::FileItem;
use logger::AuditLogger;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionKind {
    RecycleBin,
    Quarantine(PathBuf),
    PermanentDelete,
}

impl ActionKind {
    pub fn display_name(&self) -> &'static str {
        match self {
            ActionKind::RecycleBin => "Pindahkan ke Recycle Bin (Aman - Default)",
            ActionKind::Quarantine(_) => "Pindahkan ke Folder Karantina",
            ActionKind::PermanentDelete => "Hapus Permanen (Perhatian: Tidak bisa dikembalikan!)",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActionReport {
    pub total_processed: usize,
    pub successful: usize,
    pub failed: usize,
    pub bytes_freed: u64,
    pub log_path: PathBuf,
    pub error_details: Vec<(PathBuf, String)>,
}

pub struct ActionExecutor;

impl ActionExecutor {
    pub fn execute(
        action: &ActionKind,
        files_to_remove: &[FileItem],
    ) -> ActionReport {
        let logger = AuditLogger::new().ok();
        let mut successful = 0;
        let mut failed = 0;
        let mut bytes_freed = 0;
        let mut error_details = Vec::new();

        for item in files_to_remove {
            let path = &item.path;
            if !path.exists() {
                continue;
            }

            let result: Result<(), String> = match action {
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
            l.log_summary(files_to_remove.len(), successful, failed, bytes_freed);
            l.log_path.clone()
        } else {
            PathBuf::from("logs/dupesweeper_audit.txt")
        };

        ActionReport {
            total_processed: files_to_remove.len(),
            successful,
            failed,
            bytes_freed,
            log_path: log_file_path,
            error_details,
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

        // Avoid collision in quarantine folder
        while dest_path.exists() {
            let new_name = if ext.is_empty() {
                format!("{}_{}", stem, counter)
            } else {
                format!("{}_{}.{}", stem, counter, ext)
            };
            dest_path = quarantine_dir.join(new_name);
            counter += 1;
        }

        // Try atomic rename first, fallback to copy + delete across volumes
        if fs::rename(source, &dest_path).is_err() {
            fs::copy(source, &dest_path)
                .map_err(|e| format!("Copy to quarantine failed: {}", e))?;
            fs::remove_file(source)
                .map_err(|e| format!("Failed removing original after copy: {}", e))?;
        }

        Ok(())
    }
}

