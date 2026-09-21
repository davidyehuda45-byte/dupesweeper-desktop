use std::path::PathBuf;
use std::time::SystemTime;

use super::template_fingerprints::{TemplateCategory, TemplateFingerprint};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionStrategy {
    KeepOldest,
    KeepNewest,
    KeepShortestPath,
    KeepLongestPath,
}

impl SelectionStrategy {
    pub fn name(&self) -> &'static str {
        match self {
            SelectionStrategy::KeepOldest => "Simpan Terlama (Oldest)",
            SelectionStrategy::KeepNewest => "Simpan Terbaru (Newest)",
            SelectionStrategy::KeepShortestPath => "Simpan Path Terpendek (Shortest Path)",
            SelectionStrategy::KeepLongestPath => "Simpan Path Terpanjang (Longest Path)",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileItem {
    pub path: PathBuf,
    pub size: u64,
    pub created: Option<SystemTime>,
    pub modified: Option<SystemTime>,
    pub is_selected: bool,
    pub is_recommended_keep: bool,
    pub is_sensitive: bool,
    pub template_match: Option<TemplateFingerprint>,
}

impl FileItem {
    pub fn new(
        path: PathBuf,
        size: u64,
        created: Option<SystemTime>,
        modified: Option<SystemTime>,
        is_sensitive: bool,
        template_match: Option<TemplateFingerprint>,
    ) -> Self {
        Self {
            path,
            size,
            created,
            modified,
            is_selected: false,
            is_recommended_keep: false,
            is_sensitive,
            template_match,
        }
    }

    pub fn file_name(&self) -> String {
        self.path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| self.path.to_string_lossy().to_string())
    }

    pub fn parent_dir_str(&self) -> String {
        self.path
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    /// Checks whether this file is considered protected from automatic selection.
    /// Both sensitive files and functional framework template files are protected.
    pub fn is_auto_select_protected(&self) -> bool {
        if self.is_sensitive {
            return true;
        }
        if let Some(ref tm) = self.template_match {
            if tm.category == TemplateCategory::Functional {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone)]
pub struct DuplicateGroup {
    pub hash: String,
    pub file_size: u64,
    pub files: Vec<FileItem>,
}

impl DuplicateGroup {
    pub fn new(hash: String, file_size: u64, files: Vec<FileItem>) -> Self {
        let mut group = Self {
            hash,
            file_size,
            files,
        };
        group.apply_strategy(SelectionStrategy::KeepOldest);
        group
    }

    pub fn total_wasted_bytes(&self) -> u64 {
        if self.files.len() <= 1 {
            0
        } else {
            (self.files.len() as u64 - 1) * self.file_size
        }
    }

    pub fn selected_wasted_bytes(&self) -> u64 {
        let count = self.files.iter().filter(|f| f.is_selected).count() as u64;
        count * self.file_size
    }

    pub fn selected_count(&self) -> usize {
        self.files.iter().filter(|f| f.is_selected).count()
    }

    pub fn has_sensitive_selected(&self) -> bool {
        self.files.iter().any(|f| f.is_selected && f.is_sensitive)
    }

    pub fn apply_strategy(&mut self, strategy: SelectionStrategy) {
        if self.files.is_empty() {
            return;
        }

        let keep_idx = match strategy {
            SelectionStrategy::KeepOldest => {
                let mut best_idx = 0;
                let mut best_time = self.files[0].modified.unwrap_or(SystemTime::UNIX_EPOCH);
                for (i, file) in self.files.iter().enumerate().skip(1) {
                    let t = file.modified.unwrap_or(SystemTime::UNIX_EPOCH);
                    if t < best_time {
                        best_time = t;
                        best_idx = i;
                    }
                }
                best_idx
            }
            SelectionStrategy::KeepNewest => {
                let mut best_idx = 0;
                let mut best_time = self.files[0].modified.unwrap_or(SystemTime::UNIX_EPOCH);
                for (i, file) in self.files.iter().enumerate().skip(1) {
                    let t = file.modified.unwrap_or(SystemTime::UNIX_EPOCH);
                    if t > best_time {
                        best_time = t;
                        best_idx = i;
                    }
                }
                best_idx
            }
            SelectionStrategy::KeepShortestPath => {
                let mut best_idx = 0;
                let mut best_len = self.files[0].path.as_os_str().len();
                for (i, file) in self.files.iter().enumerate().skip(1) {
                    let l = file.path.as_os_str().len();
                    if l < best_len {
                        best_len = l;
                        best_idx = i;
                    }
                }
                best_idx
            }
            SelectionStrategy::KeepLongestPath => {
                let mut best_idx = 0;
                let mut best_len = self.files[0].path.as_os_str().len();
                for (i, file) in self.files.iter().enumerate().skip(1) {
                    let l = file.path.as_os_str().len();
                    if l > best_len {
                        best_len = l;
                        best_idx = i;
                    }
                }
                best_idx
            }
        };

        for (i, file) in self.files.iter_mut().enumerate() {
            if i == keep_idx {
                file.is_recommended_keep = true;
                file.is_selected = false;
            } else {
                file.is_recommended_keep = false;
                // Sensitive files and functional framework files are NEVER auto-selected!
                // Documentation templates ARE allowed to be selected for deletion according to strategy.
                file.is_selected = !file.is_auto_select_protected();
            }
        }
    }

    pub fn select_all_duplicates(&mut self) {
        for file in self.files.iter_mut() {
            if !file.is_recommended_keep && !file.is_auto_select_protected() {
                file.is_selected = true;
            }
        }
    }

    pub fn deselect_all(&mut self) {
        for file in self.files.iter_mut() {
            file.is_selected = false;
        }
    }
}
