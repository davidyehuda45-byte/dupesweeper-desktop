<div align="center">

# ⚡ DupeSweeper - Duplicate File Finder & Cleaner
### Version 11.0.0 — Export, Riwayat & Undo, Auto-Scan, Analisis Disk

**Fast, Lightweight & Safe Multi-Platform Duplicate File Finder & System Cleaner**

[![Bahasa Indonesia](https://img.shields.io/badge/Language-🇮🇩%20Bahasa%20Indonesia-blue?style=flat-square)](README.md)
[![English](https://img.shields.io/badge/Language-🇺🇸%20English-lightgrey?style=flat-square)](docs/translations/README.en.md)
[![简体中文](https://img.shields.io/badge/Language-🇨🇳%20简体中文-lightgrey?style=flat-square)](docs/translations/README.zh.md)
[![हिन्दी](https://img.shields.io/badge/Language-🇮🇳%20हिन्दी-lightgrey?style=flat-square)](docs/translations/README.hi.md)

<p align="center">
  <b>🌍 Pilih Bahasa / Select Language:</b><br>
  <a href="README.md">🇮🇩 <b>Bahasa Indonesia</b></a> •
  <a href="docs/translations/README.en.md">🇺🇸 <b>English</b></a> •
  <a href="docs/translations/README.zh.md">🇨🇳 <b>简体中文</b></a> •
  <a href="docs/translations/README.hi.md">🇮🇳 <b>हिन्दी</b></a>
</p>

</div>

---

**DupeSweeper** adalah aplikasi desktop modern, super cepat, dan 100% offline yang mendukung **Windows, Linux, dan macOS**. DupeSweeper menggabungkan pencarian file duplikat berkecepatan tinggi dengan pembersihan sampah sistem dalam satu aplikasi yang ringkas:
1. **🔍 Mode Duplicate Finder:** Mendeteksi dan membersihkan file duplikat berdasarkan **content hash** asli menggunakan algoritma kriptografi **BLAKE3**.
2. **🧹 Mode General Cleanup:** Mendeteksi dan membersihkan file sampah sistem & aplikasi umum (file sementara, cache browser, thumbnail, log lama, installer lama di Downloads, dan cache developer/consumer tools).

Aplikasi ini dikompilasi menjadi **single binary mandiri berukuran sangat ringkas (< 5 MB)** tanpa dependensi runtime tambahan, tanpa instalasi rumit, tanpa iklan, dan **0 byte data dikirim ke cloud**.

---

## 🚀 Download & Jalankan Langsung (Tanpa Install & Tanpa Clone)

DupeSweeper adalah aplikasi desktop **portable standalone**. Pengguna **tidak perlu** meng-clone git repository, tidak perlu meng-install Rust atau compiler apa pun. Cukup unduh file aplikasinya dan jalankan langsung:

| Sistem Operasi | File Aplikasi Desktop (Klik untuk Unduh) | Ukuran | Cara Menjalankan |
|---|---|:---:|---|
| **🪟 Windows (Installer)** | [⬇️ **DupeSweeper-Setup.exe**](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/DupeSweeper-Setup.exe) | ~5.0 MB | **Disarankan.** Unduh & pasang. Otomatis membuat shortcut **"DupeSweeper"** di Desktop & Start Menu (nama bersih tanpa `.exe`). |
| **🪟 Windows (Portable)** | [⬇️ **DupeSweeper.exe**](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/DupeSweeper.exe) | ~5.0 MB | Versi mandiri tanpa instalasi. Cukup unduh & klik 2x untuk membuka langsung. |
| **🐧 Linux (x64)** | [⬇️ **dupesweeper-linux-x86_64**](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/dupesweeper-linux-x86_64) | ~11.3 MB | `chmod +x dupesweeper-linux-x86_64` lalu `./dupesweeper-linux-x86_64` |
| **🍎 macOS (Universal)** | [⬇️ **dupesweeper-macos-universal**](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/dupesweeper-macos-universal) | ~4.5 MB | `chmod +x dupesweeper-macos-universal` lalu `./dupesweeper-macos-universal` |

> 💡 **Halaman Rilis Lengkap:** Seluruh riwayat versi, changelog, dan source code arsip dapat diakses melalui [GitHub Releases](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases).

---

### 🛡️ Catatan Keamanan Windows SmartScreen & Antivirus

Saat Anda pertama kali mengunduh dan menjalankan file `DupeSweeper-Setup.exe` atau `DupeSweeper.exe`, Windows Defender / SmartScreen mungkin menampilkan jendela peringatan bertuliskan:
> **"Windows protected your PC / Windows melindungi PC Anda"**  
> *Microsoft Defender SmartScreen prevented an unrecognized app from starting.*

#### 🤔 Kenapa peringatan ini muncul?
1. **Software Open-Source Bebas Biaya:** DupeSweeper adalah aplikasi 100% gratis dan open-source. Microsoft SmartScreen secara default menandai file `.exe` yang tidak ditandatangani dengan *EV Code Signing Certificate* berbayar (yang berbiaya \$300–\$500/tahun).
2. **File Rilis Baru:** SmartScreen bekerja berdasarkan sistem reputasi akumulatif. Setiap rilis versi baru akan dianggap "belum dikenal" sampai reputasi unduhan meningkat di server Microsoft.

#### 🚀 Cara Menjalankannya (Hanya Sekali di Awal):
1. Pada jendela biru SmartScreen, klik teks **"More info"** *(Info selengkapnya)*.
2. Klik tombol **"Run anyway"** *(Tetap jalankan)* yang muncul di pojok kanan bawah.
3. DupeSweeper akan langsung terbuka seketika!

> 🔒 **Jaminan Keamanan & Privasi 100%:**
> - **100% Offline (Zero Telemetry):** Aplikasi tidak terhubung ke internet sama sekali, tidak mengirim data analitik, dan tidak melacak pengguna.
> - **No Administrator Privilege Required:** Berjalan dengan izin pengguna biasa (*asInvoker*), tidak pernah meminta hak akses Administrator (UAC).
> - **Transparan & Open-Source:** Seluruh source code dapat diaudit secara bebas di repositori GitHub ini. Binary Windows dan multi-platform dibangun langsung secara otomatis dan reproducible lewat GitHub Actions CI.

---

## 🌐 Fitur Baru di v6.0.0: Dukungan Cross-Platform Penuh

### 1. Arsitektur Abstraksi Platform (`src/platform/`)
- **Single Source Codebase:** Logika inti duplikasi dan GUI immediate mode 100% portabel. Perbedaan sistem operasi diisolasi secara rapi di dalam layer `src/platform/`:
  - `src/platform/windows.rs`: Win32 Shell32 Recycle Bin query & empty, Windows %TEMP% / %LOCALAPPDATA% path resolvers, Windows Explorer selection.
  - `src/platform/linux.rs`: Freedesktop.org Trash Specification (`~/.local/share/Trash`), standard `/tmp`, `/var/tmp`, `~/.cache` resolvers, dan `xdg-open`.
  - `src/platform/macos.rs`: Finder Trash (`~/.Trash`), `~/Library/Caches`, `~/Library/Application Support`, QuickLook thumbnail cache, dan `open -R`.
- **Crate `dirs` Integration:** Menggantikan semua pembacaan environment variable hardcoded Windows dengan resolver path standar cross-platform (`dirs::cache_dir()`, `dirs::config_dir()`, `dirs::download_dir()`, `dirs::home_dir()`).

### 2. Matriks Path General Cleanup per Platform

| Kategori | Windows | Linux | macOS |
|---|---|---|---|
| **Temp Files** | `%TEMP%`, `C:\Windows\Temp` | `/tmp`, `/var/tmp` | `/private/var/folders`, `/tmp`, `~/Library/Caches` |
| **Cache Browser Chrome** | `%LOCALAPPDATA%\Google\Chrome\User Data\Default\Cache` | `~/.cache/google-chrome/Default/Cache` | `~/Library/Caches/Google/Chrome/Default/Cache` |
| **Cache Browser Firefox** | `%APPDATA%\Mozilla\Firefox\Profiles\*\cache2` | `~/.cache/mozilla/firefox/*/cache2` | `~/Library/Caches/Firefox/Profiles/*/cache2` |
| **Thumbnail Cache** | `thumbcache_*.db` (Explorer) | `~/.cache/thumbnails` | `~/Library/Caches/com.apple.QuickLook.thumbnailcache` |
| **Recycle Bin / Trash** | Windows Recycle Bin (Win32 Shell32) | `~/.local/share/Trash` (Freedesktop) | `~/.Trash` (Finder) |
| **Installer di Downloads** | `*.exe`, `*.msi` | `*.deb`, `*.rpm`, `*.appimage`, `*.tar.gz` | `*.dmg`, `*.pkg` |
| **npm cache** | `%APPDATA%\npm-cache` | `~/.npm` | `~/.npm` |
| **pip cache** | `%LOCALAPPDATA%\pip\Cache` | `~/.cache/pip` | `~/Library/Caches/pip` |
| **VS Code cache** | `%APPDATA%\Code\Cache` | `~/.config/Code/Cache` | `~/Library/Application Support/Code/Cache` |
| **Cargo registry cache** | `~/.cargo/registry/cache` | `~/.cargo/registry/cache` | `~/.cargo/registry/cache` |
| **Aplikasi Umum** | Discord, Spotify | Discord, Spotify | Discord, Spotify |

### 3. CI/CD Multi-Platform Otomatis
- Disediakan file konfigurasi GitHub Actions workflow: `.github/workflows/ci.yml`.
- Otomatis melakukan `cargo test` dan `cargo build --release` serentak di `windows-latest`, `ubuntu-latest`, dan `macos-latest` setiap push/pull request.

---

## 🧹 Fitur Mode General Cleanup

Mode terpisah yang dapat dipilih langsung dari header atas melalui tab navigasi: **"🔍 Cari File Duplikat"** vs **"🧹 Bersihkan Sampah Sistem"**.

### Kategori File Sampah yang Dibersihkan:

1. **📁 Temporary Files:**
   - File sementara (*.tmp, *.temp) sisa installer atau crash aplikasi.
   - *Status:* **Aman (Default Tercentang)**.
2. **🌐 Cache Browser (Chrome, Edge, Firefox):**
   - Cache murni (`Cache`, `Code Cache`, `GPUCache`, `CacheStorage`).
   - **Prinsip Keamanan Absolut:** **TIDAK PERNAH menyentuh Cookies, Login Data, Riwayat (History), maupun Bookmarks!**
   - *Status:* **Aman (Default Tercentang)**.
3. **📜 Log Files & Crash Dump Lama (> 30 Hari):**
   - File log error (*.log), crash dump (*.dmp), dan laporan error sistem lama.
   - *Status:* **Aman (Default Tercentang)**.
4. **🖼️ Thumbnail Cache:**
   - Database cache pratinjau thumbnail file manager. Sistem operasi akan meregenerasi otomatis saat dibutuhkan.
   - *Status:* **Aman (Default Tercentang)**.
5. **🗑️ Isi Recycle Bin / Trash:**
   - Membaca kapasitas total seluruh drive dan opsi mengosongkan dengan konfirmasi.
   - *Status:* **Aman (Default Tercentang)**.
6. **💿 Installer & Setup Lama di Downloads (> 60 Hari):**
   - File installer (*.exe, *.msi, *.deb, *.rpm, *.AppImage, *.dmg, *.pkg) di folder Downloads yang sudah berumur lebih dari 60 hari.
   - **Prinsip Keamanan:** **Default TIDAK TERCENTANG (OFF)** untuk mencegah file yang masih dibutuhkan terhapus tanpa sengaja.
   - *Status:* **Perlu Review (Manual Select)**.
7. **🛠️ Cache Developer Tools (npm, pip, VS Code, Cargo):**
   - Global package manager cache (`npm-cache`, pip cache, VS Code cache/logs, Cargo registry cache).
   - Aman dibersihkan karena tools akan men-download ulang otomatis jika diperlukan.
   - *Status:* **Aman (Default Tercentang)**.
8. **🎧 Cache Aplikasi Umum (Discord, Spotify):**
   - Cache lokal penyimpanan Discord & Spotify. Aplikasi akan me-refresh cache saat digunakan kembali.
   - *Status:* **Aman (Default Tercentang)**.

### Fitur Keamanan General Cleanup:
- **Graceful Error Handling:** File yang sedang dikunci (*locked*) atau sedang dipakai oleh aplikasi lain otomatis dilewati (*skipped*) tanpa crash/panic.
- **Dry-Run Size Calculation:** Menghitung estimasi kapasitas yang bisa dibebaskan sebelum pengguna menekan tombol bersihkan.
- **Detail Drawer:** Setiap kartu kategori dapat di-*expand* untuk melihat daftar file spesifik di dalamnya.
- **Audit Logging:** Seluruh aktivitas pembersihan dicatat ke file log audit (`logs/dupesweeper_audit_*.txt`).

---

## 🎯 Fitur Template Fingerprint Detection (Mode Duplicate)

DupeSweeper dilengkapi sistem cerdas untuk membedakan file starter template bawaan framework (Laravel, Next.js, Vite React, Vite Vue) yang identik di banyak project:

### 🗑️ Kategori C — Template Dokumentasi (Aman Dihapus)
- File dokumentasi / metadata bawaan starter kit (`README.md`, `.gitignore`, `welcome.blade.php`, `vite.svg`, `react.svg`, dll).
- Diberi badge khusus: **`🗑️ Template Default ([Framework])`** (warna muted teal).
- **Perilaku:** Diikutsertakan dalam auto-suggest pembersihan.

### 📋 Kategori D — Template Fungsional (Terproteksi dari Auto-Selection)
- File kode fungsional / runtime default framework yang belum dimodifikasi (`0001_01_01_000000_create_users_table.php`, `DatabaseSeeder.php`, `routes/web.php`, `src/app/page.tsx`, `vite.config.js`, dll).
- Diberi badge khusus: **`📋 Bawaan [Framework] (Belum Dimodifikasi)`** (warna soft blue).
- **Perilaku:** **Kebal dari auto-suggest pembersihan** (default tidak tercentang di bawah semua strategi seleksi).

---

## ⚡ Fitur Performa Ekstrem

- **Background Bulk Deletion (`DeleteWorker`):**
  - Penghapusan file dilakukan di background thread terpisah menggunakan non-blocking crossbeam channel. Bebas dari freeze/not responding.
  - Dilengkapi tombol **"Batalkan"** yang menghentikan proses pembersihan kapan saja dengan aman via atomic flag.
- **Virtualized / Lazy Scrolling (`show_rows`):**
  - Hanya merender baris yang tepat berada di dalam viewport layar. Scroll tetap **60 FPS halus tanpa lag** pada dataset **100.000+ file**!
- **Collapsed by Default:**
  - Grup duplikat tampil ringkas secara default (*collapsed*). Tersedia tombol **"▶ Buka Semua"** dan **"▼ Tutup Semua"**.
- **Asynchronous Lazy Thumbnail Worker:**
  - Decoding thumbnail gambar berlangsung di background thread terpisah tanpa membebani UI thread.

---

## 📋 Matriks Klasifikasi Kategori

| Kategori | Nama | Contoh | Perilaku Scan | Perilaku Seleksi | Badge UI |
|---|---|---|---|---|---|
| **A** | Excluded Directories | `node_modules`, `vendor`, `target`, `.git`, `.next`, `DerivedData`, `.cache` | Di-skip total | N/A | N/A |
| **B** | Protected Sensitive Files | `.env`, `*.key`, `*.pem`, `secrets.json`, SSH keys | Tetap di-scan & dideteksi | **Default unchecked**, kebal auto-select | `⚠️ Sensitif` |
| **C** | Template Dokumentasi | `README.md`, `.gitignore`, `welcome.blade.php`, `react.svg` | Di-scan & dicocokkan BLAKE3 | **Auto-suggested** untuk dibersihkan | `🗑️ Template Default ([Framework])` |
| **D** | Template Fungsional | Migrasi Laravel, `DatabaseSeeder.php`, `page.tsx`, `vite.config.js` | Di-scan & dicocokkan BLAKE3 | **Default unchecked**, kebal auto-select | `📋 Bawaan [Framework] (Belum Dimodifikasi)` |
| **Clean** | System & App Junk | Temp files, browser cache, logs, thumbnail, installers | Scan lokasi & pola file | **Safe = ON, Risky = OFF** | `Aman` / `⚠️ Perlu Review` |

---

## 🚀 Cara Menjalankan & Build per Platform

### 🪟 Windows
```powershell
# Jalankan langsung dari source:
cargo run --release

# Build standalone .exe:
cargo build --release
# File binary siap pakai di target/release/dupesweeper.exe (~4.7 MB)
```

### 🐧 Linux
```bash
# Install dependensi GUI dasar (Ubuntu/Debian):
sudo apt-get install -y libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libasound2-dev

# Jalankan langsung:
cargo run --release

# Build binary standalone:
cargo build --release

# Opsional: Package sebagai .AppImage portabel
cargo install cargo-appimage
cargo appimage
```

### 🍎 macOS
```bash
# Jalankan langsung:
cargo run --release

# Build binary standalone:
cargo build --release

# Opsional: Package sebagai bundle .app / .dmg
cargo install cargo-bundle
cargo bundle --release
```

---

## 🧪 Hasil Pengujian (Automated Tests)

Test suite mencakup seluruh skenario Duplicate Finder, General Cleanup, Background Deletion, dan Cross-Platform Resolution:
1. `test_hasher_partial_and_full`: Verifikasi partial vs full hash integrity.
2. `test_end_to_end_scanner_detection`: Verifikasi scanner multi-folder end-to-end.
3. `test_selection_strategies`: Validasi Keep Oldest, Keep Newest, Keep Shortest Path.
4. `test_action_quarantine_execution`: Pemindahan ke karantina dan pencatatan audit log.
5. `test_zero_byte_files_filtered_by_default`: Filter file 0-byte.
6. `test_three_way_duplicates_across_nested_subfolders`: Multi-duplikat di folder bersarang.
7. `test_excluded_directories_are_skipped`: Direktori node_modules, vendor, dll otomatis dilewati.
8. `test_sensitive_files_default_unchecked`: File sensitif (.env, key) kebal auto-selection.
9. `test_scan_all_toggle_overrides_exclude`: Toggle scan all mem-bypass exclude list.
10. `test_large_dataset_virtualized_performance`: Simulasi 50.000 grup (150.000 file) terbukti memproses flat row mapping dalam waktu **< 50ms** tanpa frame drop.
11. `test_documentation_template_match_is_auto_selected`: Template dokumentasi (Kategori C) terdeteksi dan diikutsertakan dalam auto-suggest pembersihan.
12. `test_functional_template_match_is_protected_from_auto_selection`: Template fungsional (Kategori D) kebal dari auto-suggest di bawah seluruh strategi & tombol Pilih Semua.
13. `test_end_to_end_template_fingerprint_scanner`: Pengujian end-to-end scanner mendeteksi Kategori C dan D secara bersamaan.
14. `test_modified_framework_file_no_longer_matches_template`: File template yang dimodifikasi developer otomatis tidak lagi cocok dengan fingerprint.
15. `test_locked_file_skipped_gracefully`: File yang terkunci (*locked*) oleh proses lain dilewati tanpa crash dan dicatat pada audit log.
16. `test_installer_category_default_unchecked`: Kategori installer Downloads default tidak tercentang (SafetyLevel::NeedsReview).
17. `test_safe_categories_default_checked`: Kategori aman (temp, browser cache, logs, thumbnail, recycle bin, dev tools, apps) default tercentang (SafetyLevel::Safe).
18. `test_size_calculation_matches_actual_deletion`: Estimasi ukuran pra-pembersihan persis sama dengan ukuran yang dibebaskan setelah pembersihan.
19. `test_recycle_bin_query`: Query status Recycle Bin / Trash berjalan sukses via native API.
20. `test_delete_worker_background_execution`: Verifikasi eksekusi penghapusan di background thread tanpa memblokir UI thread (< 50ms return).
21. `test_delete_worker_cancellation`: Verifikasi fitur pembatalan delete massal secara aman menggunakan atomic flag.
22. `test_cleanup_executor_cancellation`: Verifikasi fitur pembatalan pembersihan sistem general cleanup.
23. `test_platform_temp_dirs_valid`: Verifikasi ketersediaan direktori temporary lintas platform.
24. `test_platform_downloads_installer_patterns`: Verifikasi pattern installer (.exe/.msi di Windows, .deb/.rpm/.appimage di Linux, .dmg/.pkg di macOS).
25. `test_platform_dev_tools_cache_dirs`: Verifikasi resolver direktori cache developer tools (npm, pip, VS Code, Cargo).
26. `test_platform_consumer_apps_cache_dirs`: Verifikasi resolver direktori cache Discord dan Spotify.
27. `test_platform_recycle_bin_query`: Verifikasi query ukuran dan item Recycle Bin / Trash.
28. `test_linux_module_resolvers_do_not_panic`: Verifikasi path resolver Linux berjalan tanpa panic.
29. `test_macos_module_resolvers_do_not_panic`: Verifikasi path resolver macOS berjalan tanpa panic.

Semua 29 test lulus (**100% OK**).
