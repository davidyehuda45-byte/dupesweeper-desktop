use eframe::egui::{self, Color32, Rounding, Stroke};
use std::path::Path;
use std::time::SystemTime;

// ----------------------------------------------------------------------------
// Spacing & Sizing Grid (8px Base)
// ----------------------------------------------------------------------------
pub const SPACE_XS: f32 = 4.0;
pub const SPACE_SM: f32 = 8.0;
pub const SPACE_MD: f32 = 16.0;
pub const SPACE_LG: f32 = 24.0;
pub const SPACE_XL: f32 = 32.0;

// Corner Radii
pub const RADIUS_SM: f32 = 4.0;
pub const RADIUS_MD: f32 = 8.0;
pub const RADIUS_LG: f32 = 12.0;

// ----------------------------------------------------------------------------
// Multi-Level Dark Elevation Palette
// ----------------------------------------------------------------------------
pub const COLOR_BG_DARK: Color32 = Color32::from_rgb(14, 16, 21);      // Level 0: Main Window Background (#0E1015)
pub const COLOR_PANEL_BG: Color32 = Color32::from_rgb(22, 25, 34);     // Level 1: Top/Bottom Header Panel (#161922)
pub const COLOR_CARD_BG: Color32 = Color32::from_rgb(30, 35, 47);      // Level 2: Component Cards (#1E232F)
pub const COLOR_CARD_HEADER: Color32 = Color32::from_rgb(38, 44, 58);  // Level 3: Card Headers / Highlight (#262C3A)
pub const COLOR_CARD_HOVER: Color32 = Color32::from_rgb(40, 47, 62);   // Interactive Hover (#282F3E)
pub const COLOR_BORDER: Color32 = Color32::from_rgb(51, 60, 78);       // Border Outline (#333C4E)
pub const COLOR_BORDER_SUBTLE: Color32 = Color32::from_rgb(38, 44, 58);

// ----------------------------------------------------------------------------
// Brand & Semantic Colors
// ----------------------------------------------------------------------------
pub const COLOR_BRAND_ACCENT: Color32 = Color32::from_rgb(245, 158, 11);  // Electric Lightning Amber (#F59E0B)
pub const COLOR_BRAND_HOVER: Color32 = Color32::from_rgb(251, 191, 36);   // Hover Amber (#FBBF24)

pub const COLOR_ACCENT_PRIMARY: Color32 = Color32::from_rgb(59, 130, 246); // Action Blue 500 (#3B82F6)
pub const COLOR_ACCENT_HOVER: Color32 = Color32::from_rgb(96, 165, 250);   // Action Blue 400 (#60A5FA)

pub const COLOR_KEEP_BG: Color32 = Color32::from_rgb(22, 101, 52);        // Forest Green (#166534)
pub const COLOR_KEEP_TEXT: Color32 = Color32::from_rgb(187, 247, 208);    // Green 200 (#BBF7D0)

pub const COLOR_DELETE_BG: Color32 = Color32::from_rgb(153, 27, 27);      // Deep Red (#991B1B)
pub const COLOR_DELETE_HOVER: Color32 = Color32::from_rgb(185, 28, 28);   // Red Hover (#B91C1C)
pub const COLOR_DELETE_TEXT: Color32 = Color32::from_rgb(254, 202, 202);  // Red 200 (#FECACA)

pub const COLOR_SENSITIVE_BG: Color32 = Color32::from_rgb(146, 64, 14);   // Dark Amber (#92400E)
pub const COLOR_SENSITIVE_TEXT: Color32 = Color32::from_rgb(254, 240, 138);// Light Amber (#FEF08A)

pub const COLOR_TEXT_PRIMARY: Color32 = Color32::from_rgb(249, 250, 251); // High Contrast White (#F9FAFB)
pub const COLOR_MUTED_TEXT: Color32 = Color32::from_rgb(156, 163, 175);   // Secondary Muted Gray (#9CA3AF)

pub fn setup_custom_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.visuals.dark_mode = true;
    style.visuals.override_text_color = Some(COLOR_TEXT_PRIMARY);
    style.visuals.window_fill = COLOR_BG_DARK;
    style.visuals.panel_fill = COLOR_PANEL_BG;

    // Smooth rounded widgets with 8px radius
    style.visuals.widgets.noninteractive.bg_fill = COLOR_CARD_BG;
    style.visuals.widgets.noninteractive.rounding = Rounding::same(RADIUS_MD);
    style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, COLOR_BORDER);

    style.visuals.widgets.inactive.bg_fill = COLOR_CARD_BG;
    style.visuals.widgets.inactive.rounding = Rounding::same(RADIUS_MD);
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, COLOR_BORDER);

    style.visuals.widgets.hovered.bg_fill = COLOR_CARD_HOVER;
    style.visuals.widgets.hovered.rounding = Rounding::same(RADIUS_MD);
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, COLOR_ACCENT_HOVER);

    style.visuals.widgets.active.bg_fill = COLOR_ACCENT_PRIMARY;
    style.visuals.widgets.active.rounding = Rounding::same(RADIUS_MD);

    ctx.set_style(style);
}

pub fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * KB;
    const GB: f64 = 1024.0 * MB;

    let b = bytes as f64;
    if b >= GB {
        format!("{:.2} GB", b / GB)
    } else if b >= MB {
        format!("{:.2} MB", b / MB)
    } else if b >= KB {
        format!("{:.1} KB", b / KB)
    } else {
        format!("{} B", bytes)
    }
}

pub fn format_system_time(time_opt: Option<SystemTime>) -> String {
    match time_opt {
        Some(t) => {
            let datetime: chrono::DateTime<chrono::Local> = t.into();
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        None => "-".to_string(),
    }
}

pub fn file_extension_category(path: &Path) -> (&'static str, Color32) {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    match ext.as_str() {
        "jpg" | "jpeg" | "png" | "webp" | "gif" | "bmp" | "svg" | "ico" => {
            ("IMG", Color32::from_rgb(147, 51, 234)) // Purple
        }
        "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" => {
            ("VID", Color32::from_rgb(239, 68, 68)) // Red
        }
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" => {
            ("AUD", Color32::from_rgb(234, 88, 12)) // Orange
        }
        "pdf" => ("PDF", Color32::from_rgb(220, 38, 38)), // Crimson
        "doc" | "docx" | "txt" | "rtf" | "odt" => {
            ("DOC", Color32::from_rgb(37, 99, 235)) // Blue
        }
        "xls" | "xlsx" | "csv" => {
            ("SHEET", Color32::from_rgb(22, 163, 74)) // Green
        }
        "zip" | "rar" | "7z" | "tar" | "gz" => {
            ("ARCH", Color32::from_rgb(202, 138, 4)) // Yellow
        }
        "rs" | "js" | "ts" | "py" | "html" | "css" | "json" | "c" | "cpp" => {
            ("CODE", Color32::from_rgb(13, 148, 136)) // Teal
        }
        _ => ("FILE", Color32::from_rgb(107, 114, 128)), // Gray
    }
}
