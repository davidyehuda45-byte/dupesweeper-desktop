use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CleanupCategoryId {
    TempFiles,
    BrowserCache,
    OldLogs,
    ThumbnailCache,
    RecycleBin,
    DownloadsInstallers,
    DevToolsCache,
    ConsumerAppsCache,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyLevel {
    /// Low-risk cache or temporary files that can be automatically suggested (default checked)
    Safe,
    /// Higher-risk files requiring manual review before deletion (default unchecked)
    NeedsReview,
}

#[derive(Debug, Clone)]
pub struct CleanupCategoryDef {
    pub id: CleanupCategoryId,
    pub title: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    pub safety: SafetyLevel,
    pub default_enabled: bool,
    pub warning: Option<&'static str>,
}

pub static CLEANUP_CATEGORIES: &[CleanupCategoryDef] = &[
    CleanupCategoryDef {
        id: CleanupCategoryId::TempFiles,
        title: "Temporary Files (File Sementara)",
        description: "File sementara dari direktori temporary sistem (Windows %TEMP%, Linux /tmp, macOS ~/Library/Caches).",
        icon: "📁",
        safety: SafetyLevel::Safe,
        default_enabled: true,
        warning: None,
    },
    CleanupCategoryDef {
        id: CleanupCategoryId::BrowserCache,
        title: "Cache Browser (Chrome, Edge, Firefox)",
        description: "Cache gambar dan skrip web browser. Tidak menyentuh riwayat (History) atau Cookies.",
        icon: "🌐",
        safety: SafetyLevel::Safe,
        default_enabled: true,
        warning: Some("Tutup browser Anda sebelum membersihkan untuk hasil maksimal."),
    },
    CleanupCategoryDef {
        id: CleanupCategoryId::OldLogs,
        title: "Log Files & Crash Dump Lama (> 30 Hari)",
        description: "File log (*.log), crash dump (*.dmp), dan laporan error sistem lama.",
        icon: "📜",
        safety: SafetyLevel::Safe,
        default_enabled: true,
        warning: None,
    },
    CleanupCategoryDef {
        id: CleanupCategoryId::ThumbnailCache,
        title: "Thumbnail Cache",
        description: "Database cache pratinjau thumbnail file manager (Windows thumbcache, Linux ~/.cache/thumbnails, macOS QuickLook).",
        icon: "🖼️",
        safety: SafetyLevel::Safe,
        default_enabled: true,
        warning: None,
    },
    CleanupCategoryDef {
        id: CleanupCategoryId::RecycleBin,
        title: "Isi Recycle Bin / Trash",
        description: "File yang sebelumnya dihapus ke Recycle Bin / Trash dan masih memakan kapasitas drive Anda.",
        icon: "🗑️",
        safety: SafetyLevel::Safe,
        default_enabled: true,
        warning: Some("File di Recycle Bin / Trash akan dikosongkan secara permanen."),
    },
    CleanupCategoryDef {
        id: CleanupCategoryId::DownloadsInstallers,
        title: "Installer & Setup Lama di Downloads (> 60 Hari)",
        description: "File installer (*.exe, *.msi, *.deb, *.rpm, *.AppImage, *.dmg, *.pkg) di folder Downloads yang sudah berumur lebih dari 60 hari.",
        icon: "💿",
        safety: SafetyLevel::NeedsReview,
        default_enabled: false,
        warning: Some("Pastikan software terkait sudah terinstall dengan benar sebelum menghapus installer ini."),
    },
    CleanupCategoryDef {
        id: CleanupCategoryId::DevToolsCache,
        title: "Cache Developer Tools (npm, pip, VS Code, Cargo)",
        description: "Global package cache & log editor. Aman dibersihkan karena akan di-download ulang otomatis jika diperlukan.",
        icon: "🛠️",
        safety: SafetyLevel::Safe,
        default_enabled: true,
        warning: None,
    },
    CleanupCategoryDef {
        id: CleanupCategoryId::ConsumerAppsCache,
        title: "Cache Aplikasi Umum (Discord, Spotify)",
        description: "Cache media dan storage lokal aplikasi Discord & Spotify. Aplikasi akan re-cache saat dijalankan.",
        icon: "🎧",
        safety: SafetyLevel::Safe,
        default_enabled: true,
        warning: Some("Tutup aplikasi Discord/Spotify sebelum membersihkan agar file tidak terkunci."),
    },
];

#[derive(Debug, Clone)]
pub struct CleanupItem {
    pub path: PathBuf,
    pub size: u64,
    pub modified: Option<SystemTime>,
}

#[derive(Debug, Clone)]
pub struct CategoryScanResult {
    pub id: CleanupCategoryId,
    pub title: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    pub safety: SafetyLevel,
    pub items: Vec<CleanupItem>,
    pub total_bytes: u64,
    pub is_enabled: bool,
    pub is_expanded: bool,
    pub warning: Option<&'static str>,
}

impl CategoryScanResult {
    pub fn new(def: &CleanupCategoryDef) -> Self {
        Self {
            id: def.id,
            title: def.title,
            description: def.description,
            icon: def.icon,
            safety: def.safety,
            items: Vec::new(),
            total_bytes: 0,
            is_enabled: def.default_enabled,
            is_expanded: false,
            warning: def.warning,
        }
    }
}

