<div align="center">

# ⚡ DupeSweeper - Duplicate File Finder & Cleaner
### Version 6.0.0 — Cross-Platform Desktop Support (Windows, Linux, macOS)

**Fast, Lightweight & Safe Multi-Platform Duplicate File Finder & System Cleaner**

[![Bahasa Indonesia](https://img.shields.io/badge/Language-🇮🇩%20Bahasa%20Indonesia-lightgrey?style=flat-square)](../../README.md)
[![English](https://img.shields.io/badge/Language-🇺🇸%20English-blue?style=flat-square)](README.en.md)
[![简体中文](https://img.shields.io/badge/Language-🇨🇳%20简体中文-lightgrey?style=flat-square)](README.zh.md)
[![हिन्दी](https://img.shields.io/badge/Language-🇮🇳%20हिन्दी-lightgrey?style=flat-square)](README.hi.md)

<p align="center">
  <b>🌍 Select Language / Pilih Bahasa:</b><br>
  <a href="../../README.md">🇮🇩 <b>Bahasa Indonesia</b></a> •
  <a href="README.en.md">🇺🇸 <b>English</b></a> •
  <a href="README.zh.md">🇨🇳 <b>简体中文</b></a> •
  <a href="README.hi.md">🇮🇳 <b>हिन्दी</b></a>
</p>

</div>

---

**DupeSweeper** is a modern, ultra-fast, 100% offline desktop utility engineered for **Windows, Linux, and macOS**. It unifies blazing-fast duplicate file detection with comprehensive system cleanup into a single, cohesive desktop experience:
1. **🔍 Duplicate Finder Mode:** Detects and cleans duplicate files based on cryptographic **content hashing** using **BLAKE3**.
2. **🧹 General Cleanup Mode:** Scans and purges system junk & popular application caches (temporary files, browser caches, thumbnail databases, old logs, outdated Downloads installers, developer tools & consumer application caches).

The application compiles into a **standalone binary under 5 MB** with zero runtime dependencies, no complicated installers, zero adware, and **0 bytes of data transmitted to the cloud**.

---

## 🚀 Direct Download & Run (No Installation & No Git Clone Required)

DupeSweeper is distributed as a **portable standalone** desktop executable. You **do not** need to clone this repository, nor do you need to install Rust or any build tools. Simply download the binary for your platform and launch it directly:

| Operating System | Desktop Binary (Click to Download) | Size | How to Run |
|---|---|:---:|---|
| **🪟 Windows (x64)** | [⬇️ **dupesweeper-windows-x86_64.exe**](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/dupesweeper-windows-x86_64.exe) | ~5.0 MB | Download & **double-click** the `.exe`. Starts immediately! |
| **🐧 Linux (x64)** | [⬇️ **dupesweeper-linux-x86_64**](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/dupesweeper-linux-x86_64) | ~11.3 MB | `chmod +x dupesweeper-linux-x86_64` then `./dupesweeper-linux-x86_64` |
| **🍎 macOS (Universal)** | [⬇️ **dupesweeper-macos-universal**](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/dupesweeper-macos-universal) | ~4.5 MB | `chmod +x dupesweeper-macos-universal` then `./dupesweeper-macos-universal` |

> 💡 **Full Releases Archive:** Access version history, changelogs, and complete source archives on [GitHub Releases](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases).

---

### 🛡️ Windows SmartScreen & Antivirus Security Note

When downloading and launching `dupesweeper-windows-x86_64.exe` for the first time, Windows Defender / SmartScreen may present a blue prompt:
> **"Windows protected your PC"**  
> *Microsoft Defender SmartScreen prevented an unrecognized app from starting.*

#### 🤔 Why does this prompt appear?
1. **Free Open-Source Software:** DupeSweeper is 100% free and open-source. Microsoft SmartScreen flags `.exe` files that are not signed with a paid commercial *EV Code Signing Certificate* (which costs \$300–\$500 annually).
2. **New Release Reputation:** SmartScreen utilizes an accumulative download reputation algorithm. Every new version release is treated as "unfamiliar" until enough users have downloaded it.

#### 🚀 How to Run the App (One-Time Only):
1. In the blue SmartScreen window, click the text link **"More info"**.
2. Click the **"Run anyway"** button that appears in the bottom right corner.
3. DupeSweeper will launch instantly!

> 🔒 **100% Security & Privacy Guarantee:**
> - **100% Offline (Zero Telemetry):** The application makes no internet connections, collects no analytics, and does not track user activity.
> - **No Administrator Privileges Required:** Runs strictly under standard user privileges (`asInvoker`), never requesting UAC administrator escalation.
> - **Transparent & Open-Source:** The complete source code is auditable in this repository. All binaries are built reproducibly via GitHub Actions CI.

---

## 🌐 New in v6.0.0: Full Cross-Platform Desktop Support

### 1. Platform Abstraction Layer (`src/platform/`)
- **Single Source Codebase:** Core deduplication engine and immediate mode GUI are 100% portable. OS differences are cleanly encapsulated inside `src/platform/`:
  - `src/platform/windows.rs`: Win32 Shell32 Recycle Bin query & purge, `%TEMP%` / `%LOCALAPPDATA%` resolvers, Explorer file reveal.
  - `src/platform/linux.rs`: Freedesktop.org Trash Specification (`~/.local/share/Trash`), standard `/tmp`, `/var/tmp`, `~/.cache` resolvers, and `xdg-open`.
  - `src/platform/macos.rs`: Finder Trash (`~/.Trash`), `~/Library/Caches`, QuickLook thumbnail cache, and `open -R`.
- **Crate `dirs` Integration:** Replaces hardcoded environment variables with standard cross-platform paths (`dirs::cache_dir()`, `dirs::config_dir()`, `dirs::download_dir()`, `dirs::home_dir()`).

### 2. Multi-Platform System Cleanup Matrix

| Category | Windows | Linux | macOS |
|---|---|---|---|
| **Temp Files** | `%TEMP%`, `C:\Windows\Temp` | `/tmp`, `/var/tmp` | `/private/var/folders`, `/tmp`, `~/Library/Caches` |
| **Chrome Cache** | `%LOCALAPPDATA%\Google\Chrome\User Data\Default\Cache` | `~/.cache/google-chrome/Default/Cache` | `~/Library/Caches/Google/Chrome/Default/Cache` |
| **Firefox Cache** | `%APPDATA%\Mozilla\Firefox\Profiles\*\cache2` | `~/.cache/mozilla/firefox/*/cache2` | `~/Library/Caches/Firefox/Profiles/*/cache2` |
| **Thumbnail Cache** | `thumbcache_*.db` (Explorer) | `~/.cache/thumbnails` | `~/Library/Caches/com.apple.QuickLook.thumbnailcache` |
| **Recycle Bin / Trash** | Windows Recycle Bin (Win32 Shell32) | `~/.local/share/Trash` (Freedesktop) | `~/.Trash` (Finder) |
| **Old Installers in Downloads** | `*.exe`, `*.msi` | `*.deb`, `*.rpm`, `*.appimage`, `*.tar.gz` | `*.dmg`, `*.pkg` |
| **npm cache** | `%APPDATA%\npm-cache` | `~/.npm` | `~/.npm` |
| **pip cache** | `%LOCALAPPDATA%\pip\Cache` | `~/.cache/pip` | `~/Library/Caches/pip` |
| **VS Code cache** | `%APPDATA%\Code\Cache` | `~/.config/Code/Cache` | `~/Library/Application Support/Code/Cache` |
| **Cargo registry cache** | `~/.cargo/registry/cache` | `~/.cargo/registry/cache` | `~/.cargo/registry/cache` |
| **Common Consumer Apps** | Discord, Spotify | Discord, Spotify | Discord, Spotify |

---

## 🧹 General System Cleanup Mode

Accessible directly from the header navigation bar: **"🔍 Find Duplicate Files"** vs **"🧹 Clean System Junk"**.

### Junk File Categories:
1. **📁 Temporary Files:** Leftover installer or application crash files (*.tmp, *.temp). *(Default: Checked / Safe)*
2. **🌐 Browser Cache (Chrome, Edge, Firefox):** Pure web cache files. **Strict Security Principle: NEVER touches Cookies, Passwords, Browsing History, or Bookmarks!** *(Default: Checked / Safe)*
3. **📜 Log Files & Crash Dumps (> 30 Days):** Outdated system diagnostic logs (*.log) and dump files (*.dmp). *(Default: Checked / Safe)*
4. **🖼️ Thumbnail Cache:** Operating system image preview caches. Regenerated automatically by the file manager as needed. *(Default: Checked / Safe)*
5. **🗑️ Recycle Bin / Trash:** Measures total capacity across all drives with one-click purge confirmation. *(Default: Checked / Safe)*
6. **💿 Old Installers in Downloads (> 60 Days):** Outdated setup files. **Safety Principle: Unchecked by default** to avoid removing files you still need. *(Default: Unchecked / Manual Review)*
7. **🛠️ Developer Tools Cache (npm, pip, VS Code, Cargo):** Global package manager caches. Safely redownloaded on demand. *(Default: Checked / Safe)*
8. **🎧 Consumer App Cache (Discord, Spotify):** Local media caches. Refreshed automatically during app usage. *(Default: Checked / Safe)*

---

## 🎯 Template Fingerprint Detection (Duplicate Mode)

DupeSweeper includes an intelligent fingerprinting system to identify boilerplate template files generated by web frameworks (Laravel, Next.js, Vite React, Vite Vue):

- **🗑️ Category C — Documentation Templates (Safe to Clean):** Boilerplate markdown/starter files (`README.md`, `.gitignore`, `welcome.blade.php`, `vite.svg`, `react.svg`). Tagged with `🗑️ Default Template ([Framework])` and eligible for auto-suggest selection.
- **📋 Category D — Functional Templates (Protected from Auto-Selection):** Unmodified functional framework runtime files (`0001_01_01_000000_create_users_table.php`, `DatabaseSeeder.php`, `routes/web.php`, `src/app/page.tsx`, `vite.config.js`). Tagged with `📋 Stock [Framework] (Unmodified)` and **shielded from auto-suggested deletion** across all selection presets.

---

## ⚡ High-Performance Architecture

- **Background Bulk Deletion (`DeleteWorker`):** File removal executes in a dedicated background worker via crossbeam channels. The UI stays 100% responsive with an immediate "Cancel" button.
- **Virtualized Viewport (`show_rows`):** Renders only visible rows within the screen viewport. Guarantees silky-smooth **60 FPS scrolling even with 100,000+ files**.
- **Asynchronous Lazy Thumbnail Decoding:** Image previews decode on a background thread pool without lagging the main UI loop.

---

## 🚀 Building from Source

```bash
# Clone the repository
git clone https://github.com/davidyehuda45-byte/dupesweeper-desktop.git
cd dupesweeper-desktop

# Run with Cargo:
cargo run --release

# Build release binary:
cargo build --release
```

All 29 automated unit & integration tests pass with 100% success:
```bash
cargo test
```
