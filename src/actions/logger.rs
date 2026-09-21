use chrono::Local;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct AuditLogger {
    pub log_path: PathBuf,
}

impl AuditLogger {
    pub fn new() -> std::io::Result<Self> {
        let logs_dir = PathBuf::from("logs");
        if !logs_dir.exists() {
            fs::create_dir_all(&logs_dir)?;
        }

        let now = Local::now();
        let file_name = format!("dupesweeper_audit_{}.txt", now.format("%Y%m%d_%H%M%S"));
        let log_path = logs_dir.join(file_name);

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&log_path)?;

        writeln!(file, "=======================================================")?;
        writeln!(file, "DupeSweeper - Session Audit Log")?;
        writeln!(file, "Timestamp: {}", now.format("%Y-%m-%d %H:%M:%S"))?;
        writeln!(file, "=======================================================\n")?;

        Ok(Self { log_path })
    }

    pub fn log_action(&self, action: &str, file_path: &Path, success: bool, note: &str) {
        if let Ok(mut file) = OpenOptions::new().append(true).open(&self.log_path) {
            let time_str = Local::now().format("%H:%M:%S");
            let status_str = if success { "SUCCESS" } else { "FAILED" };
            let _ = writeln!(
                file,
                "[{}] [{}] [{}] {} {}",
                time_str,
                status_str,
                action,
                file_path.display(),
                if note.is_empty() { String::new() } else { format!("({})", note) }
            );
        }
    }

    pub fn log_summary(&self, total_processed: usize, successful: usize, failed: usize, bytes_freed: u64) {
        if let Ok(mut file) = OpenOptions::new().append(true).open(&self.log_path) {
            let _ = writeln!(file, "\n-------------------------------------------------------");
            let _ = writeln!(file, "Execution Summary:");
            let _ = writeln!(file, "Total Files Processed: {}", total_processed);
            let _ = writeln!(file, "Successful: {}", successful);
            let _ = writeln!(file, "Failed: {}", failed);
            let _ = writeln!(
                file,
                "Total Space Freed: {:.2} MB ({} bytes)",
                bytes_freed as f64 / (1024.0 * 1024.0),
                bytes_freed
            );
            let _ = writeln!(file, "-------------------------------------------------------");
        }
    }
}

