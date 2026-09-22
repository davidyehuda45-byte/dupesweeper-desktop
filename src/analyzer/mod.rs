//! Folder size analyzer: computes the size of every immediate child of a
//! directory (files as-is, sub-folders aggregated recursively), so the user
//! can drill down to find what is eating disk space.

use crossbeam_channel::{unbounded, Receiver, Sender};
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct SizeEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub file_count: usize,
}

#[derive(Debug, Clone)]
pub enum AnalyzerProgress {
    Scanning { scanned_count: usize },
    Finished {
        entries: Vec<SizeEntry>,
        total_size: u64,
        elapsed: Duration,
    },
    Cancelled,
    Error(String),
}

pub struct FolderAnalyzer;

impl FolderAnalyzer {
    /// Spawns a background scan of `root`'s immediate children.
    pub fn start(root: PathBuf, cancel_flag: Arc<AtomicBool>) -> Receiver<AnalyzerProgress> {
        let (tx, rx) = unbounded();
        thread::spawn(move || {
            Self::run(root, cancel_flag, tx);
        });
        rx
    }

    fn run(root: PathBuf, cancel_flag: Arc<AtomicBool>, tx: Sender<AnalyzerProgress>) {
        let start = Instant::now();

        if !root.is_dir() {
            let _ = tx.send(AnalyzerProgress::Error("Folder tidak ditemukan".to_string()));
            return;
        }

        let children: Vec<PathBuf> = match fs::read_dir(&root) {
            Ok(rd) => rd.filter_map(|e| e.ok()).map(|e| e.path()).collect(),
            Err(e) => {
                let _ = tx.send(AnalyzerProgress::Error(format!("Tidak bisa membaca folder: {}", e)));
                return;
            }
        };

        let scanned = Arc::new(AtomicUsize::new(0));

        let entries: Vec<SizeEntry> = children
            .into_par_iter()
            .filter_map(|child| {
                if cancel_flag.load(Ordering::Relaxed) {
                    return None;
                }
                let name = child
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| child.to_string_lossy().to_string());

                let metadata = fs::symlink_metadata(&child).ok()?;

                if metadata.is_dir() {
                    let (size, count) = Self::dir_size(&child, &cancel_flag, &scanned, &tx);
                    Some(SizeEntry {
                        name,
                        path: child,
                        is_dir: true,
                        size,
                        file_count: count,
                    })
                } else {
                    let c = scanned.fetch_add(1, Ordering::Relaxed) + 1;
                    if c % 200 == 0 {
                        let _ = tx.send(AnalyzerProgress::Scanning { scanned_count: c });
                    }
                    Some(SizeEntry {
                        name,
                        path: child,
                        is_dir: false,
                        size: metadata.len(),
                        file_count: 1,
                    })
                }
            })
            .collect();

        if cancel_flag.load(Ordering::Relaxed) {
            let _ = tx.send(AnalyzerProgress::Cancelled);
            return;
        }

        let mut entries = entries;
        entries.sort_by(|a, b| b.size.cmp(&a.size));
        let total_size: u64 = entries.iter().map(|e| e.size).sum();

        let _ = tx.send(AnalyzerProgress::Finished {
            entries,
            total_size,
            elapsed: start.elapsed(),
        });
    }

    fn dir_size(
        path: &std::path::Path,
        cancel_flag: &Arc<AtomicBool>,
        scanned: &Arc<AtomicUsize>,
        tx: &Sender<AnalyzerProgress>,
    ) -> (u64, usize) {
        let mut total = 0u64;
        let mut count = 0usize;

        for entry in WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if cancel_flag.load(Ordering::Relaxed) {
                break;
            }
            if entry.file_type().is_file() {
                if let Ok(md) = entry.metadata() {
                    total += md.len();
                    count += 1;
                    let c = scanned.fetch_add(1, Ordering::Relaxed) + 1;
                    if c % 500 == 0 {
                        let _ = tx.send(AnalyzerProgress::Scanning { scanned_count: c });
                    }
                }
            }
        }

        (total, count)
    }
}
