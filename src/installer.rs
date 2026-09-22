//! Windows Installer & Shortcut Manager for DupeSweeper
//! Handles creating clean "DupeSweeper" desktop & start menu shortcuts,
//! installation to %LOCALAPPDATA%\Programs\DupeSweeper, and registry registration.

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct InstallInfo {
    pub app_name: &'static str,
    pub publisher: &'static str,
    pub version: &'static str,
    pub install_dir: PathBuf,
    pub exe_path: PathBuf,
    pub desktop_shortcut: PathBuf,
    pub start_menu_shortcut: PathBuf,
}

#[cfg(windows)]
pub mod windows {
    use super::InstallInfo;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    pub fn get_install_info() -> Option<InstallInfo> {
        let local_app_data = dirs::data_local_dir()?; // %LOCALAPPDATA%
        let install_dir = local_app_data.join("Programs").join("DupeSweeper");
        let exe_path = install_dir.join("DupeSweeper.exe");

        let desktop = dirs::desktop_dir()?;
        let desktop_shortcut = desktop.join("DupeSweeper.lnk");

        let app_data = dirs::data_dir()?; // %APPDATA%
        let start_menu = app_data
            .join("Microsoft")
            .join("Windows")
            .join("Start Menu")
            .join("Programs");
        let start_menu_shortcut = start_menu.join("DupeSweeper.lnk");

        Some(InstallInfo {
            app_name: "DupeSweeper",
            publisher: "David Yehuda Surbakti",
            version: "11.0.0",
            install_dir,
            exe_path,
            desktop_shortcut,
            start_menu_shortcut,
        })
    }

    /// Creates a Windows shell shortcut (.lnk) via PowerShell WScript.Shell
    pub fn create_shortcut(target_exe: &Path, shortcut_path: &Path, description: &str) -> bool {
        let target_str = target_exe.to_string_lossy().replace('\'', "''");
        let shortcut_str = shortcut_path.to_string_lossy().replace('\'', "''");
        let workdir_str = target_exe
            .parent()
            .unwrap_or(target_exe)
            .to_string_lossy()
            .replace('\'', "''");
        let desc_str = description.replace('\'', "''");

        let ps_script = format!(
            "$ws = New-Object -ComObject WScript.Shell; \
             $s = $ws.CreateShortcut('{}'); \
             $s.TargetPath = '{}'; \
             $s.WorkingDirectory = '{}'; \
             $s.Description = '{}'; \
             $s.Save()",
            shortcut_str, target_str, workdir_str, desc_str
        );

        let status = Command::new("powershell")
            .args([
                "-NoProfile",
                "-NonInteractive",
                "-WindowStyle",
                "Hidden",
                "-Command",
                &ps_script,
            ])
            .status();

        matches!(status, Ok(s) if s.success())
    }

    /// Registers DupeSweeper in Windows Apps & Features (Settings) under HKCU
    pub fn register_uninstall_entry(info: &InstallInfo) -> bool {
        let reg_key = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall\DupeSweeper";
        let uninstall_cmd = format!("\"{}\" --uninstall", info.exe_path.to_string_lossy());
        let icon_str = format!("\"{}\",0", info.exe_path.to_string_lossy());

        let cmds = [
            format!("reg.exe add \"{}\" /v \"DisplayName\" /t REG_SZ /d \"{}\" /f", reg_key, info.app_name),
            format!("reg.exe add \"{}\" /v \"DisplayIcon\" /t REG_SZ /d \"{}\" /f", reg_key, icon_str),
            format!("reg.exe add \"{}\" /v \"DisplayVersion\" /t REG_SZ /d \"{}\" /f", reg_key, info.version),
            format!("reg.exe add \"{}\" /v \"Publisher\" /t REG_SZ /d \"{}\" /f", reg_key, info.publisher),
            format!("reg.exe add \"{}\" /v \"InstallLocation\" /t REG_SZ /d \"{}\" /f", reg_key, info.install_dir.to_string_lossy()),
            format!("reg.exe add \"{}\" /v \"UninstallString\" /t REG_SZ /d \"{}\" /f", reg_key, uninstall_cmd),
            format!("reg.exe add \"{}\" /v \"NoModify\" /t REG_DWORD /d 1 /f", reg_key),
            format!("reg.exe add \"{}\" /v \"NoRepair\" /t REG_DWORD /d 1 /f", reg_key),
        ];

        for cmd in &cmds {
            let _ = Command::new("cmd").args(["/C", cmd]).status();
        }
        true
    }

    /// Installs current executable into %LOCALAPPDATA%\Programs\DupeSweeper\DupeSweeper.exe
    /// and creates clean "DupeSweeper" shortcuts on Desktop and Start Menu.
    pub fn install_current_exe(create_desktop: bool, create_start_menu: bool) -> Result<PathBuf, String> {
        let current_exe = std::env::current_exe().map_err(|e| e.to_string())?;
        let info = get_install_info().ok_or_else(|| "Gagal menentukan direktori instalasi Windows".to_string())?;

        // Create destination directory
        fs::create_dir_all(&info.install_dir).map_err(|e| e.to_string())?;

        // Copy executable if not already in the target path
        if current_exe != info.exe_path {
            fs::copy(&current_exe, &info.exe_path).map_err(|e| format!("Gagal menyalin file: {}", e))?;
        }

        // Create Start Menu shortcut
        if create_start_menu {
            if let Some(parent) = info.start_menu_shortcut.parent() {
                let _ = fs::create_dir_all(parent);
            }
            create_shortcut(
                &info.exe_path,
                &info.start_menu_shortcut,
                "DupeSweeper - Duplicate File Finder & Cleaner",
            );
        }

        // Create Desktop shortcut
        if create_desktop {
            create_shortcut(
                &info.exe_path,
                &info.desktop_shortcut,
                "DupeSweeper - Duplicate File Finder & Cleaner",
            );
        }

        // Register in Windows Settings Apps
        register_uninstall_entry(&info);

        Ok(info.exe_path)
    }

    /// Removes shortcuts and uninstall registry keys
    pub fn uninstall() -> Result<(), String> {
        if let Some(info) = get_install_info() {
            let _ = fs::remove_file(&info.desktop_shortcut);
            let _ = fs::remove_file(&info.start_menu_shortcut);

            let _ = Command::new("reg")
                .args([
                    "delete",
                    r"HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall\DupeSweeper",
                    "/f",
                ])
                .status();

            // Self-delete install folder after process exits
            let cmd_str = format!("timeout /t 2 & rmdir /s /q \"{}\"", info.install_dir.to_string_lossy());
            let _ = Command::new("cmd").args(["/C", &cmd_str]).spawn();
        }
        Ok(())
    }

    /// Returns true if the currently running executable is located at the installed path
    pub fn is_running_installed() -> bool {
        if let (Ok(current), Some(info)) = (std::env::current_exe(), get_install_info()) {
            if let (Ok(c_can), Ok(i_can)) = (current.canonicalize(), info.exe_path.canonicalize()) {
                return c_can == i_can;
            }
            current == info.exe_path
        } else {
            false
        }
    }

    /// Determines if the current invocation is acting as a Setup Installer
    pub fn is_setup_mode() -> bool {
        if let Ok(current) = std::env::current_exe() {
            let name = current
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_lowercase();
            if name.contains("setup") || name.contains("installer") {
                return !is_running_installed();
            }
        }
        std::env::args().any(|a| a == "--install" || a == "/install")
    }
}

#[cfg(not(windows))]
pub mod non_windows {
    use super::InstallInfo;
    use std::path::PathBuf;

    pub fn get_install_info() -> Option<InstallInfo> {
        None
    }

    pub fn install_current_exe(_desktop: bool, _start_menu: bool) -> Result<PathBuf, String> {
        Ok(std::env::current_exe().unwrap_or_else(|_| PathBuf::from("dupesweeper")))
    }

    pub fn uninstall() -> Result<(), String> {
        Ok(())
    }

    pub fn is_running_installed() -> bool {
        true
    }

    pub fn is_setup_mode() -> bool {
        false
    }
}

#[cfg(windows)]
pub use windows::*;

#[cfg(not(windows))]
pub use non_windows::*;
