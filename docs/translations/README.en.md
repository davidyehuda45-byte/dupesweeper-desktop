<div align="center">

# DupeSweeper — Duplicate File Finder & Cleaner
### Version 11.0.0 — Export, History & Undo, Auto-Scan, Disk Analyzer

**A Fast, Lightweight, and Safe Multi-Platform Duplicate File Finder & System Cleaner**

[![Bahasa Indonesia](https://img.shields.io/badge/Language-Bahasa%20Indonesia-lightgrey?style=flat-square)](../../README.md)
[![English](https://img.shields.io/badge/Language-English-blue?style=flat-square)](README.en.md)
[![简体中文](https://img.shields.io/badge/Language-简体中文-lightgrey?style=flat-square)](README.zh.md)
[![हिन्दी](https://img.shields.io/badge/Language-हिन्दी-lightgrey?style=flat-square)](README.hi.md)

<p align="center">
  <b>Select Language / Pilih Bahasa:</b><br>
  <a href="../../README.md"><b>Bahasa Indonesia</b></a> &middot;
  <a href="README.en.md"><b>English</b></a> &middot;
  <a href="README.zh.md"><b>简体中文</b></a> &middot;
  <a href="README.hi.md"><b>हिन्दी</b></a>
</p>

</div>

---

**DupeSweeper** is a modern, fast, 100% offline desktop utility built for **Windows, Linux, and macOS**. It unifies high-speed duplicate file detection with comprehensive system cleanup in a single, cohesive application:

1. **Duplicate Finder Mode** — Detects and removes duplicate files based on true content hashing using the **BLAKE3** cryptographic algorithm.
2. **General Cleanup Mode** — Scans and clears system junk and common application caches (temporary files, browser caches, thumbnail databases, old logs, outdated installers in Downloads, and developer/consumer tool caches).
3. **Disk Analyzer Mode** — Browses a folder's contents sorted by size, with drill-down navigation, to help you find what is consuming disk space.

The application compiles into a **standalone binary under 5.5 MB** with no additional runtime dependencies, no complicated installation, no ads, and **no data sent to the cloud**.

---

## Download & Run Directly (No Installation, No Cloning Required)

DupeSweeper is distributed as a **portable standalone** desktop application. You do **not** need to clone this repository or install Rust or any compiler. Simply download the binary for your platform and run it.

| Operating System | Application File | Size | How to Run |
|---|---|:---:|---|
| **Windows (Installer)** | [DupeSweeper-Setup.exe](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/DupeSweeper-Setup.exe) | ~5.0 MB | Recommended. Download and install. Automatically creates a "DupeSweeper" shortcut on the Desktop and Start Menu. |
| **Windows (Portable)** | [DupeSweeper.exe](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/DupeSweeper.exe) | ~5.0 MB | Standalone executable, no installation. Download and run directly. |
| **Linux (x64)** | [dupesweeper-linux-x86_64](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/dupesweeper-linux-x86_64) | ~11.3 MB | `chmod +x dupesweeper-linux-x86_64` then `./dupesweeper-linux-x86_64` |
| **macOS (Universal)** | [dupesweeper-macos-universal](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/dupesweeper-macos-universal) | ~4.5 MB | `chmod +x dupesweeper-macos-universal` then `./dupesweeper-macos-universal` |

> The full release archive, version history, and changelog are available on [GitHub Releases](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases).

---

### Windows SmartScreen & Antivirus Security Note

When first downloading and running `DupeSweeper-Setup.exe` or `DupeSweeper.exe`, Windows Defender / SmartScreen may display a warning window stating:

> "Windows protected your PC"
> Microsoft Defender SmartScreen prevented an unrecognized app from starting.

**Why this warning appears:**
1. **Free, open-source software.** DupeSweeper is 100% free and open-source. SmartScreen flags `.exe` files that are not signed with a paid *EV Code Signing Certificate* (which costs roughly $300–500 per year).
2. **New release.** SmartScreen relies on a cumulative reputation system. Each new version is treated as "unrecognized" until enough users have downloaded it.

**How to run it (one time only):**
1. In the blue SmartScreen window, click "More info".
2. Click the "Run anyway" button that appears.
3. DupeSweeper will open immediately.

> **Security and privacy guarantees:**
> - **100% offline, zero telemetry.** The application never connects to the internet, sends no analytics, and does not track users.
> - **No administrator privileges required.** Runs under standard user permissions (`asInvoker`) and never requests UAC elevation.
> - **Transparent and open-source.** The complete source code is freely auditable in this repository. Windows and multi-platform binaries are built automatically and reproducibly via GitHub Actions CI.

---

## What's New in v11.0.0

1. **Export Reports (CSV/JSON)** — Export duplicate scan results or cleanup analysis to CSV or JSON files for auditing or backup before deletion.
2. **History & Undo** — Every cleanup session is recorded locally. For Recycle Bin actions, DupeSweeper can restore (undo) files to their original location directly from the app.
3. **Scheduled Auto-Scan** — Automatically re-scans the last-used folders on a configurable interval (30 minutes to 24 hours), while the application remains open.
4. **Disk Analyzer** — A new tab for browsing the largest folders and files within a directory, with breadcrumb-based drill-down navigation.

---

## Cross-Platform Support

### Platform Abstraction Layer (`src/platform/`)
- **Single source codebase.** The core deduplication engine and immediate-mode GUI are fully portable. OS differences are cleanly isolated inside `src/platform/`:
  - `src/platform/windows.rs` — Win32 Shell32 Recycle Bin query & purge, `%TEMP%` / `%LOCALAPPDATA%` path resolvers, Windows Explorer integration.
  - `src/platform/linux.rs` — Freedesktop.org Trash Specification (`~/.local/share/Trash`), standard `/tmp`, `/var/tmp`, `~/.cache` resolvers, and `xdg-open`.
  - `src/platform/macos.rs` — Finder Trash (`~/.Trash`), `~/Library/Caches`, `~/Library/Application Support`, QuickLook thumbnail cache, and `open -R`.
- **`dirs` crate integration.** Replaces hardcoded Windows environment variable reads with standard cross-platform path resolvers (`dirs::cache_dir()`, `dirs::config_dir()`, `dirs::download_dir()`, `dirs::home_dir()`).

### Multi-Platform Cleanup Path Matrix

| Category | Windows | Linux | macOS |
|---|---|---|---|
| Temp Files | `%TEMP%`, `C:\Windows\Temp` | `/tmp`, `/var/tmp` | `/private/var/folders`, `/tmp`, `~/Library/Caches` |
| Chrome Cache | `%LOCALAPPDATA%\Google\Chrome\User Data\Default\Cache` | `~/.cache/google-chrome/Default/Cache` | `~/Library/Caches/Google/Chrome/Default/Cache` |
| Firefox Cache | `%APPDATA%\Mozilla\Firefox\Profiles\*\cache2` | `~/.cache/mozilla/firefox/*/cache2` | `~/Library/Caches/Firefox/Profiles/*/cache2` |
| Thumbnail Cache | `thumbcache_*.db` (Explorer) | `~/.cache/thumbnails` | `~/Library/Caches/com.apple.QuickLook.thumbnailcache` |
| Recycle Bin / Trash | Windows Recycle Bin (Win32 Shell32) | `~/.local/share/Trash` (Freedesktop) | `~/.Trash` (Finder) |
| Old Installers in Downloads | `*.exe`, `*.msi` | `*.deb`, `*.rpm`, `*.appimage`, `*.tar.gz` | `*.dmg`, `*.pkg` |
| npm cache | `%APPDATA%\npm-cache` | `~/.npm` | `~/.npm` |
| pip cache | `%LOCALAPPDATA%\pip\Cache` | `~/.cache/pip` | `~/Library/Caches/pip` |
| VS Code cache | `%APPDATA%\Code\Cache` | `~/.config/Code/Cache` | `~/Library/Application Support/Code/Cache` |
| Cargo registry cache | `~/.cargo/registry/cache` | `~/.cargo/registry/cache` | `~/.cargo/registry/cache` |
| Common consumer apps | Discord, Spotify | Discord, Spotify | Discord, Spotify |

### Automated Multi-Platform CI/CD
- A GitHub Actions workflow is provided at `.github/workflows/ci.yml`.
- It automatically runs `cargo test` and `cargo build --release` on `windows-latest`, `ubuntu-latest`, and `macos-latest` for every push and pull request.

---

## General Cleanup Mode Features

A separate mode selectable from the header navigation tabs: "Find Duplicate Files" vs. "Clean System Junk".

### Junk File Categories

1. **Temporary Files** — Leftover temporary files (`*.tmp`, `*.temp`) from installers or crashed applications. Status: safe, checked by default.
2. **Browser Cache (Chrome, Edge, Firefox)** — Pure cache data (`Cache`, `Code Cache`, `GPUCache`, `CacheStorage`). Strict safety principle: never touches cookies, saved logins, browsing history, or bookmarks. Status: safe, checked by default.
3. **Old Log Files & Crash Dumps (older than 30 days)** — Error logs (`*.log`), crash dumps (`*.dmp`), and old system error reports. Status: safe, checked by default.
4. **Thumbnail Cache** — File manager thumbnail preview cache databases. Regenerated automatically by the OS as needed. Status: safe, checked by default.
5. **Recycle Bin / Trash Contents** — Reads total capacity across all drives with a confirmation before emptying. Status: safe, checked by default.
6. **Old Installers in Downloads (older than 60 days)** — Installer files (`*.exe`, `*.msi`, `*.deb`, `*.rpm`, `*.AppImage`, `*.dmg`, `*.pkg`) in the Downloads folder older than 60 days. Unchecked by default to avoid accidentally removing files you still need. Status: requires manual review.
7. **Developer Tools Cache (npm, pip, VS Code, Cargo)** — Global package manager caches. Safe to clean since tools will re-download as needed. Status: safe, checked by default.
8. **Common Application Cache (Discord, Spotify)** — Local storage cache for Discord and Spotify. Refreshed automatically on next use. Status: safe, checked by default.

### General Cleanup Safety Features
- **Graceful error handling** — Locked files or files in use by another application are automatically skipped without crashing.
- **Dry-run size calculation** — Estimates reclaimable space before the user triggers the cleanup.
- **Detail drawer** — Each category card can be expanded to view the specific list of files inside it.
- **Audit logging** — Every cleanup action is recorded to an audit log file (`logs/dupesweeper_audit_*.txt`).

---

## Template Fingerprint Detection (Duplicate Mode)

DupeSweeper includes an intelligent system to distinguish framework boilerplate template files (Laravel, Next.js, Vite React, Vite Vue) that are identical across many projects.

### Category C — Documentation Templates (Safe to Delete)
- Boilerplate documentation/metadata files (`README.md`, `.gitignore`, `welcome.blade.php`, `vite.svg`, `react.svg`, and similar).
- Tagged with the badge "Default Template ([Framework])".
- Behavior: included in auto-suggested cleanup.

### Category D — Functional Templates (Protected from Auto-Selection)
- Unmodified functional/runtime framework files (`0001_01_01_000000_create_users_table.php`, `DatabaseSeeder.php`, `routes/web.php`, `src/app/page.tsx`, `vite.config.js`, and similar).
- Tagged with the badge "Stock [Framework] (Unmodified)".
- Behavior: shielded from auto-suggested cleanup (unchecked by default under every selection strategy).

---

## Performance Features

- **Background bulk deletion (`DeleteWorker`)** — File deletion runs on a dedicated background thread via a non-blocking crossbeam channel, free of UI freezes. Includes a "Cancel" button that safely stops the process at any time via an atomic flag.
- **Virtualized / lazy scrolling (`show_rows`)** — Only renders rows currently inside the viewport. Scrolling stays smooth on datasets of 100,000+ files.
- **Collapsed by default** — Duplicate groups are collapsed by default, with "Expand All" and "Collapse All" controls.
- **Asynchronous lazy thumbnail worker** — Image thumbnail decoding happens on a dedicated background thread without blocking the UI thread.

---

## Category Classification Matrix

| Category | Name | Examples | Scan Behavior | Selection Behavior | UI Badge |
|---|---|---|---|---|---|
| A | Excluded Directories | `node_modules`, `vendor`, `target`, `.git`, `.next`, `DerivedData`, `.cache` | Fully skipped | Not applicable | Not applicable |
| B | Protected Sensitive Files | `.env`, `*.key`, `*.pem`, `secrets.json`, SSH keys | Still scanned & detected | Unchecked by default, immune to auto-select | Sensitive |
| C | Documentation Templates | `README.md`, `.gitignore`, `welcome.blade.php`, `react.svg` | Scanned & matched via BLAKE3 | Auto-suggested for cleanup | Default Template ([Framework]) |
| D | Functional Templates | Laravel migrations, `DatabaseSeeder.php`, `page.tsx`, `vite.config.js` | Scanned & matched via BLAKE3 | Unchecked by default, immune to auto-select | Stock [Framework] (Unmodified) |
| Clean | System & App Junk | Temp files, browser cache, logs, thumbnails, installers | Location & pattern scan | Safe = ON, Risky = OFF | Safe / Needs Review |

---

## Building & Running per Platform

### Windows
```powershell
# Run directly from source:
cargo run --release

# Build a standalone .exe:
cargo build --release
# Ready-to-use binary at target/release/dupesweeper.exe
```

### Linux
```bash
# Install base GUI dependencies (Ubuntu/Debian):
sudo apt-get install -y libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libasound2-dev

# Run directly:
cargo run --release

# Build a standalone binary:
cargo build --release

# Optional: package as a portable .AppImage
cargo install cargo-appimage
cargo appimage
```

### macOS
```bash
# Run directly:
cargo run --release

# Build a standalone binary:
cargo build --release

# Optional: package as a .app / .dmg bundle
cargo install cargo-bundle
cargo bundle --release
```

---

## Test Results (Automated Tests)

The test suite covers Duplicate Finder, General Cleanup, background deletion, cross-platform path resolution, and the v11 feature set (export, history/undo, auto-scan, disk analyzer):

1. `test_hasher_partial_and_full` — Verifies partial vs. full hash integrity.
2. `test_end_to_end_scanner_detection` — Verifies end-to-end multi-folder scanning.
3. `test_selection_strategies` — Validates Keep Oldest, Keep Newest, Keep Shortest Path.
4. `test_action_quarantine_execution` — Quarantine move and audit log recording.
5. `test_zero_byte_files_filtered_by_default` — Filters zero-byte files.
6. `test_three_way_duplicates_across_nested_subfolders` — Multi-way duplicates across nested folders.
7. `test_excluded_directories_are_skipped` — Directories such as `node_modules`, `vendor`, and similar are automatically skipped.
8. `test_sensitive_files_default_unchecked` — Sensitive files (`.env`, keys) are immune to auto-selection.
9. `test_scan_all_toggle_overrides_exclude` — "Scan all" toggle bypasses the exclude list.
10. `test_large_dataset_virtualized_performance` — Simulates 50,000 groups (150,000 files) processing flat row mapping in under 50ms with no frame drops.
11. `test_documentation_template_match_is_auto_selected` — Documentation templates (Category C) are detected and included in auto-suggested cleanup.
12. `test_functional_template_match_is_protected_from_auto_selection` — Functional templates (Category D) are immune to auto-suggestion across every strategy and the Select All action.
13. `test_end_to_end_template_fingerprint_scanner` — End-to-end scanner test detecting Categories C and D simultaneously.
14. `test_modified_framework_file_no_longer_matches_template` — A developer-modified template file no longer matches its fingerprint.
15. `test_locked_file_skipped_gracefully` — A file locked by another process is skipped without crashing and recorded in the audit log.
16. `test_installer_category_default_unchecked` — The Downloads installer category is unchecked by default (`SafetyLevel::NeedsReview`).
17. `test_safe_categories_default_checked` — Safe categories (temp, browser cache, logs, thumbnails, recycle bin, dev tools, apps) are checked by default (`SafetyLevel::Safe`).
18. `test_size_calculation_matches_actual_deletion` — Pre-cleanup size estimate matches the actual space freed after cleanup.
19. `test_recycle_bin_query` — Recycle Bin / Trash status query succeeds via the native API.
20. `test_delete_worker_background_execution` — Verifies deletion executes on a background thread without blocking the UI thread.
21. `test_delete_worker_cancellation` — Verifies safe cancellation of a bulk delete via an atomic flag.
22. `test_cleanup_executor_cancellation` — Verifies safe cancellation of a general cleanup run.
23. `test_platform_temp_dirs_valid` — Verifies availability of temp directories across platforms.
24. `test_platform_downloads_installer_patterns` — Verifies installer patterns (`.exe`/`.msi` on Windows, `.deb`/`.rpm`/`.appimage` on Linux, `.dmg`/`.pkg` on macOS).
25. `test_platform_dev_tools_cache_dirs` — Verifies developer tools cache directory resolvers (npm, pip, VS Code, Cargo).
26. `test_platform_consumer_apps_cache_dirs` — Verifies Discord and Spotify cache directory resolvers.
27. `test_platform_recycle_bin_query` — Verifies Recycle Bin / Trash size and item count queries.
28. `test_linux_module_resolvers_do_not_panic` — Verifies Linux path resolvers run without panicking.
29. `test_macos_module_resolvers_do_not_panic` — Verifies macOS path resolvers run without panicking.
30. `test_installer_info_and_shortcut_creation` — Verifies Windows install info and shortcut creation.
31. `test_export_duplicates_csv_and_json_roundtrip` — Verifies exporting duplicate finder results to CSV and JSON.
32. `test_export_cleanup_csv_and_json_roundtrip` — Verifies exporting general cleanup results to CSV and JSON.
33. `test_history_entry_is_restorable_only_for_recycle_bin_with_refs` — Verifies undo-eligibility logic for cleanup history entries.
34. `test_history_action_kind_labels_are_distinct` — Verifies history action type labels.
35. `test_auto_scan_interval_seconds_mapping` — Verifies auto-scan interval conversion.
36. `test_app_settings_serde_roundtrip_in_memory` — Verifies application settings serialization.
37. `test_folder_analyzer_computes_child_sizes_and_recurses_into_subfolders` — Verifies folder size analyzer computation, including recursive subfolder aggregation.

All 37 tests pass (100%).

```bash
cargo test
```
