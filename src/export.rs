//! CSV / JSON export of scan results, for audit or backup before deletion.

use serde::Serialize;
use std::io;
use std::path::Path;

use crate::cleanup::CategoryScanResult;
use crate::scanner::DuplicateGroup;
use crate::ui::theme::format_system_time;

fn csv_escape(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

fn write_csv(path: &Path, header: &[&str], rows: Vec<Vec<String>>) -> io::Result<()> {
    let mut out = String::new();
    out.push_str(&header.iter().map(|h| csv_escape(h)).collect::<Vec<_>>().join(","));
    out.push_str("\r\n");
    for row in rows {
        out.push_str(&row.iter().map(|c| csv_escape(c)).collect::<Vec<_>>().join(","));
        out.push_str("\r\n");
    }
    std::fs::write(path, out)
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> io::Result<()> {
    let json = serde_json::to_string_pretty(value)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    std::fs::write(path, json)
}

// ---------------------------------------------------------------------------
// Duplicate Finder results
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct DuplicateFileRow {
    group_hash: String,
    path: String,
    size_bytes: u64,
    modified: String,
    is_recommended_keep: bool,
    is_selected_for_deletion: bool,
}

#[derive(Serialize)]
struct DuplicateExportRoot {
    generated_at: String,
    total_groups: usize,
    total_wasted_bytes: u64,
    files: Vec<DuplicateFileRow>,
}

fn duplicate_rows(groups: &[DuplicateGroup]) -> Vec<DuplicateFileRow> {
    groups
        .iter()
        .flat_map(|g| {
            let hash = g.hash.clone();
            g.files.iter().map(move |f| DuplicateFileRow {
                group_hash: hash.clone(),
                path: f.path.to_string_lossy().to_string(),
                size_bytes: f.size,
                modified: format_system_time(f.modified),
                is_recommended_keep: f.is_recommended_keep,
                is_selected_for_deletion: f.is_selected,
            })
        })
        .collect()
}

pub fn export_duplicates_csv(groups: &[DuplicateGroup], path: &Path) -> io::Result<()> {
    let rows = duplicate_rows(groups)
        .into_iter()
        .map(|r| {
            vec![
                r.group_hash,
                r.path,
                r.size_bytes.to_string(),
                r.modified,
                r.is_recommended_keep.to_string(),
                r.is_selected_for_deletion.to_string(),
            ]
        })
        .collect();

    write_csv(
        path,
        &[
            "group_hash",
            "path",
            "size_bytes",
            "modified",
            "is_recommended_keep",
            "is_selected_for_deletion",
        ],
        rows,
    )
}

pub fn export_duplicates_json(groups: &[DuplicateGroup], path: &Path) -> io::Result<()> {
    let total_wasted_bytes: u64 = groups.iter().map(|g| g.total_wasted_bytes()).sum();
    let root = DuplicateExportRoot {
        generated_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        total_groups: groups.len(),
        total_wasted_bytes,
        files: duplicate_rows(groups),
    };
    write_json(path, &root)
}

// ---------------------------------------------------------------------------
// General Cleanup results
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct CleanupItemRow {
    category: String,
    path: String,
    size_bytes: u64,
    is_category_enabled: bool,
}

#[derive(Serialize)]
struct CleanupExportRoot {
    generated_at: String,
    total_items: usize,
    total_bytes: u64,
    items: Vec<CleanupItemRow>,
}

fn cleanup_rows(categories: &[CategoryScanResult]) -> Vec<CleanupItemRow> {
    categories
        .iter()
        .flat_map(|c| {
            let title = c.title.to_string();
            let enabled = c.is_enabled;
            c.items.iter().map(move |item| CleanupItemRow {
                category: title.clone(),
                path: item.path.to_string_lossy().to_string(),
                size_bytes: item.size,
                is_category_enabled: enabled,
            })
        })
        .collect()
}

pub fn export_cleanup_csv(categories: &[CategoryScanResult], path: &Path) -> io::Result<()> {
    let rows = cleanup_rows(categories)
        .into_iter()
        .map(|r| {
            vec![
                r.category,
                r.path,
                r.size_bytes.to_string(),
                r.is_category_enabled.to_string(),
            ]
        })
        .collect();

    write_csv(
        path,
        &["category", "path", "size_bytes", "is_category_enabled"],
        rows,
    )
}

pub fn export_cleanup_json(categories: &[CategoryScanResult], path: &Path) -> io::Result<()> {
    let items = cleanup_rows(categories);
    let total_bytes = categories.iter().map(|c| c.total_bytes).sum();
    let root = CleanupExportRoot {
        generated_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        total_items: items.len(),
        total_bytes,
        items,
    };
    write_json(path, &root)
}
