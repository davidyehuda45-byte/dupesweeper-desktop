<div align="center">

# DupeSweeper — Duplicate File Finder & Cleaner
### Version 11.0.0 — Export, Riwayat & Undo, Auto-Scan, Analisis Disk

**Duplicate File Finder & System Cleaner Multi-Platform yang Cepat, Ringan, dan Aman**

[![Bahasa Indonesia](https://img.shields.io/badge/Language-Bahasa%20Indonesia-blue?style=flat-square)](README.md)
[![English](https://img.shields.io/badge/Language-English-lightgrey?style=flat-square)](docs/translations/README.en.md)
[![简体中文](https://img.shields.io/badge/Language-简体中文-lightgrey?style=flat-square)](docs/translations/README.zh.md)
[![हिन्दी](https://img.shields.io/badge/Language-हिन्दी-lightgrey?style=flat-square)](docs/translations/README.hi.md)

<p align="center">
  <b>Pilih Bahasa / Select Language:</b><br>
  <a href="README.md"><b>Bahasa Indonesia</b></a> &middot;
  <a href="docs/translations/README.en.md"><b>English</b></a> &middot;
  <a href="docs/translations/README.zh.md"><b>简体中文</b></a> &middot;
  <a href="docs/translations/README.hi.md"><b>हिन्दी</b></a>
</p>

</div>

---

**DupeSweeper** adalah aplikasi desktop modern, cepat, dan 100% offline yang mendukung **Windows, Linux, dan macOS**. DupeSweeper menggabungkan pencarian file duplikat berkecepatan tinggi dengan pembersihan sampah sistem dalam satu aplikasi yang ringkas:

1. **Mode Duplicate Finder** — Mendeteksi dan membersihkan file duplikat berdasarkan *content hash* asli menggunakan algoritma kriptografi **BLAKE3**.
2. **Mode General Cleanup** — Mendeteksi dan membersihkan file sampah sistem & aplikasi umum (file sementara, cache browser, thumbnail, log lama, installer lama di Downloads, dan cache developer/consumer tools).
3. **Mode Analisis Disk** — Menelusuri isi sebuah folder terurut dari yang paling besar, dengan navigasi drill-down, untuk menemukan apa yang memakan ruang penyimpanan.

Aplikasi ini dikompilasi menjadi **single binary mandiri berukuran ringkas (kurang dari 5,5 MB)** tanpa dependensi runtime tambahan, tanpa instalasi rumit, tanpa iklan, dan **tanpa data yang dikirim ke cloud**.

---

## Download & Jalankan Langsung (Tanpa Install & Tanpa Clone)

DupeSweeper adalah aplikasi desktop **portable standalone**. Pengguna **tidak perlu** meng-clone git repository maupun meng-install Rust atau compiler apa pun. Cukup unduh file aplikasinya dan jalankan langsung.

| Sistem Operasi | File Aplikasi Desktop | Ukuran | Cara Menjalankan |
|---|---|:---:|---|
| **Windows (Installer)** | [DupeSweeper-Setup.exe](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/DupeSweeper-Setup.exe) | ~5,0 MB | Disarankan. Unduh dan pasang. Otomatis membuat shortcut "DupeSweeper" di Desktop & Start Menu. |
| **Windows (Portable)** | [DupeSweeper.exe](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/DupeSweeper.exe) | ~5,0 MB | Versi mandiri tanpa instalasi. Unduh lalu jalankan langsung. |
| **Linux (x64)** | [dupesweeper-linux-x86_64](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/dupesweeper-linux-x86_64) | ~11,3 MB | `chmod +x dupesweeper-linux-x86_64` lalu `./dupesweeper-linux-x86_64` |
| **macOS (Universal)** | [dupesweeper-macos-universal](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/dupesweeper-macos-universal) | ~4,5 MB | `chmod +x dupesweeper-macos-universal` lalu `./dupesweeper-macos-universal` |

> Halaman rilis lengkap, riwayat versi, changelog, dan source code arsip dapat diakses melalui [GitHub Releases](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases).

---

### Catatan Keamanan Windows SmartScreen & Antivirus

Saat pertama kali mengunduh dan menjalankan `DupeSweeper-Setup.exe` atau `DupeSweeper.exe`, Windows Defender / SmartScreen dapat menampilkan jendela peringatan bertuliskan:

> "Windows protected your PC" / "Windows melindungi PC Anda"
> Microsoft Defender SmartScreen prevented an unrecognized app from starting.

**Mengapa peringatan ini muncul:**
1. **Software open-source bebas biaya.** DupeSweeper adalah aplikasi 100% gratis dan open-source. SmartScreen secara default menandai file `.exe` yang tidak ditandatangani dengan *EV Code Signing Certificate* berbayar (berbiaya sekitar $300–500 per tahun).
2. **File rilis baru.** SmartScreen bekerja berdasarkan sistem reputasi akumulatif. Setiap rilis versi baru dianggap "belum dikenal" sampai reputasi unduhannya meningkat di server Microsoft.

**Cara menjalankannya (hanya sekali di awal):**
1. Pada jendela biru SmartScreen, klik teks "More info" ("Info selengkapnya").
2. Klik tombol "Run anyway" ("Tetap jalankan") yang muncul di bagian bawah.
3. DupeSweeper akan langsung terbuka.

> **Jaminan keamanan dan privasi:**
> - **100% offline, zero telemetry.** Aplikasi tidak terhubung ke internet, tidak mengirim data analitik, dan tidak melacak pengguna.
> - **Tidak memerlukan hak akses Administrator.** Berjalan dengan izin pengguna biasa (*asInvoker*) dan tidak pernah meminta elevasi UAC.
> - **Transparan dan open-source.** Seluruh source code dapat diaudit secara bebas di repositori ini. Binary Windows dan multi-platform dibangun secara otomatis dan reproducible melalui GitHub Actions CI.

---

## Fitur Baru di v11.0.0

1. **Export Laporan (CSV/JSON)** — Ekspor hasil scan duplikat maupun hasil analisis cleanup ke file CSV atau JSON untuk keperluan audit atau backup sebelum penghapusan.
2. **Riwayat & Undo** — Setiap sesi pembersihan tercatat secara lokal. Untuk aksi Recycle Bin, DupeSweeper dapat mengembalikan (undo) file ke lokasi semula langsung dari dalam aplikasi.
3. **Auto-Scan Terjadwal** — Memindai ulang folder terakhir secara otomatis pada interval yang dapat diatur (30 menit hingga 24 jam), selama aplikasi tetap terbuka.
4. **Analisis Disk** — Tab baru untuk menelusuri ukuran folder dan file terbesar dalam sebuah direktori, dengan navigasi drill-down berbasis breadcrumb.

---

## Dukungan Cross-Platform

### Arsitektur Abstraksi Platform (`src/platform/`)
- **Single source codebase.** Logika inti duplikasi dan GUI immediate mode sepenuhnya portabel. Perbedaan sistem operasi diisolasi secara rapi di dalam layer `src/platform/`:
  - `src/platform/windows.rs` — Win32 Shell32 Recycle Bin query & empty, Windows `%TEMP%` / `%LOCALAPPDATA%` path resolvers, integrasi Windows Explorer.
  - `src/platform/linux.rs` — Freedesktop.org Trash Specification (`~/.local/share/Trash`), resolver `/tmp`, `/var/tmp`, `~/.cache`, dan `xdg-open`.
  - `src/platform/macos.rs` — Finder Trash (`~/.Trash`), `~/Library/Caches`, `~/Library/Application Support`, QuickLook thumbnail cache, dan `open -R`.
- **Integrasi crate `dirs`.** Menggantikan seluruh pembacaan environment variable hardcoded Windows dengan resolver path standar cross-platform (`dirs::cache_dir()`, `dirs::config_dir()`, `dirs::download_dir()`, `dirs::home_dir()`).

### Matriks Path General Cleanup per Platform

| Kategori | Windows | Linux | macOS |
|---|---|---|---|
| Temp Files | `%TEMP%`, `C:\Windows\Temp` | `/tmp`, `/var/tmp` | `/private/var/folders`, `/tmp`, `~/Library/Caches` |
| Cache Browser Chrome | `%LOCALAPPDATA%\Google\Chrome\User Data\Default\Cache` | `~/.cache/google-chrome/Default/Cache` | `~/Library/Caches/Google/Chrome/Default/Cache` |
| Cache Browser Firefox | `%APPDATA%\Mozilla\Firefox\Profiles\*\cache2` | `~/.cache/mozilla/firefox/*/cache2` | `~/Library/Caches/Firefox/Profiles/*/cache2` |
| Thumbnail Cache | `thumbcache_*.db` (Explorer) | `~/.cache/thumbnails` | `~/Library/Caches/com.apple.QuickLook.thumbnailcache` |
| Recycle Bin / Trash | Windows Recycle Bin (Win32 Shell32) | `~/.local/share/Trash` (Freedesktop) | `~/.Trash` (Finder) |
| Installer di Downloads | `*.exe`, `*.msi` | `*.deb`, `*.rpm`, `*.appimage`, `*.tar.gz` | `*.dmg`, `*.pkg` |
| npm cache | `%APPDATA%\npm-cache` | `~/.npm` | `~/.npm` |
| pip cache | `%LOCALAPPDATA%\pip\Cache` | `~/.cache/pip` | `~/Library/Caches/pip` |
| VS Code cache | `%APPDATA%\Code\Cache` | `~/.config/Code/Cache` | `~/Library/Application Support/Code/Cache` |
| Cargo registry cache | `~/.cargo/registry/cache` | `~/.cargo/registry/cache` | `~/.cargo/registry/cache` |
| Aplikasi umum | Discord, Spotify | Discord, Spotify | Discord, Spotify |

### CI/CD Multi-Platform Otomatis
- Disediakan file konfigurasi GitHub Actions workflow: `.github/workflows/ci.yml`.
- Otomatis menjalankan `cargo test` dan `cargo build --release` di `windows-latest`, `ubuntu-latest`, dan `macos-latest` pada setiap push/pull request.

---

## Fitur Mode General Cleanup

Mode terpisah yang dapat dipilih langsung dari header atas melalui tab navigasi: "Cari File Duplikat" vs "Bersihkan Sampah Sistem".

### Kategori File Sampah yang Dibersihkan

1. **Temporary Files** — File sementara (`*.tmp`, `*.temp`) sisa installer atau crash aplikasi. Status: aman, default tercentang.
2. **Cache Browser (Chrome, Edge, Firefox)** — Cache murni (`Cache`, `Code Cache`, `GPUCache`, `CacheStorage`). Prinsip keamanan: tidak pernah menyentuh cookies, login data, riwayat, maupun bookmark. Status: aman, default tercentang.
3. **Log Files & Crash Dump Lama (lebih dari 30 hari)** — File log error (`*.log`), crash dump (`*.dmp`), dan laporan error sistem lama. Status: aman, default tercentang.
4. **Thumbnail Cache** — Database cache pratinjau thumbnail file manager. Sistem operasi meregenerasi otomatis saat dibutuhkan. Status: aman, default tercentang.
5. **Isi Recycle Bin / Trash** — Membaca kapasitas total drive dan opsi mengosongkan dengan konfirmasi. Status: aman, default tercentang.
6. **Installer & Setup Lama di Downloads (lebih dari 60 hari)** — File installer (`*.exe`, `*.msi`, `*.deb`, `*.rpm`, `*.AppImage`, `*.dmg`, `*.pkg`) di folder Downloads yang berumur lebih dari 60 hari. Default tidak tercentang untuk mencegah file yang masih dibutuhkan terhapus tanpa sengaja. Status: perlu review manual.
7. **Cache Developer Tools (npm, pip, VS Code, Cargo)** — Global package manager cache. Aman dibersihkan karena tools akan mengunduh ulang otomatis jika diperlukan. Status: aman, default tercentang.
8. **Cache Aplikasi Umum (Discord, Spotify)** — Cache lokal penyimpanan Discord & Spotify. Aplikasi akan menyegarkan cache saat digunakan kembali. Status: aman, default tercentang.

### Fitur Keamanan General Cleanup
- **Graceful error handling** — File yang sedang terkunci atau dipakai aplikasi lain otomatis dilewati tanpa crash.
- **Dry-run size calculation** — Menghitung estimasi kapasitas yang bisa dibebaskan sebelum pengguna menekan tombol bersihkan.
- **Detail drawer** — Setiap kartu kategori dapat di-*expand* untuk melihat daftar file spesifik.
- **Audit logging** — Seluruh aktivitas pembersihan dicatat ke file log audit (`logs/dupesweeper_audit_*.txt`).

---

## Fitur Template Fingerprint Detection (Mode Duplicate)

DupeSweeper dilengkapi sistem untuk membedakan file starter template bawaan framework (Laravel, Next.js, Vite React, Vite Vue) yang identik di banyak project.

### Kategori C — Template Dokumentasi (Aman Dihapus)
- File dokumentasi / metadata bawaan starter kit (`README.md`, `.gitignore`, `welcome.blade.php`, `vite.svg`, `react.svg`, dan sejenisnya).
- Diberi badge: "Template Default ([Framework])".
- Perilaku: diikutsertakan dalam auto-suggest pembersihan.

### Kategori D — Template Fungsional (Terproteksi dari Auto-Selection)
- File kode fungsional / runtime default framework yang belum dimodifikasi (`0001_01_01_000000_create_users_table.php`, `DatabaseSeeder.php`, `routes/web.php`, `src/app/page.tsx`, `vite.config.js`, dan sejenisnya).
- Diberi badge: "Bawaan [Framework] (Belum Dimodifikasi)".
- Perilaku: kebal dari auto-suggest pembersihan (default tidak tercentang di bawah semua strategi seleksi).

---

## Fitur Performa

- **Background bulk deletion (`DeleteWorker`)** — Penghapusan file berjalan di background thread terpisah menggunakan non-blocking crossbeam channel, bebas dari freeze. Dilengkapi tombol "Batalkan" yang menghentikan proses kapan saja secara aman melalui atomic flag.
- **Virtualized / lazy scrolling (`show_rows`)** — Hanya merender baris yang berada di dalam viewport layar. Scroll tetap halus pada dataset 100.000+ file.
- **Collapsed by default** — Grup duplikat tampil ringkas secara default, dengan tombol "Buka Semua" dan "Tutup Semua".
- **Asynchronous lazy thumbnail worker** — Decoding thumbnail gambar berlangsung di background thread terpisah tanpa membebani UI thread.

---

## Matriks Klasifikasi Kategori

| Kategori | Nama | Contoh | Perilaku Scan | Perilaku Seleksi | Badge UI |
|---|---|---|---|---|---|
| A | Excluded Directories | `node_modules`, `vendor`, `target`, `.git`, `.next`, `DerivedData`, `.cache` | Di-skip total | Tidak berlaku | Tidak berlaku |
| B | Protected Sensitive Files | `.env`, `*.key`, `*.pem`, `secrets.json`, SSH keys | Tetap di-scan & dideteksi | Default unchecked, kebal auto-select | Sensitif |
| C | Template Dokumentasi | `README.md`, `.gitignore`, `welcome.blade.php`, `react.svg` | Di-scan & dicocokkan BLAKE3 | Auto-suggested untuk dibersihkan | Template Default ([Framework]) |
| D | Template Fungsional | Migrasi Laravel, `DatabaseSeeder.php`, `page.tsx`, `vite.config.js` | Di-scan & dicocokkan BLAKE3 | Default unchecked, kebal auto-select | Bawaan [Framework] (Belum Dimodifikasi) |
| Clean | System & App Junk | Temp files, browser cache, logs, thumbnail, installer | Scan lokasi & pola file | Safe = ON, Risky = OFF | Aman / Perlu Review |

---

## Cara Menjalankan & Build per Platform

### Windows
```powershell
# Jalankan langsung dari source:
cargo run --release

# Build standalone .exe:
cargo build --release
# File binary siap pakai di target/release/dupesweeper.exe
```

### Linux
```bash
# Install dependensi GUI dasar (Ubuntu/Debian):
sudo apt-get install -y libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libasound2-dev

# Jalankan langsung:
cargo run --release

# Build binary standalone:
cargo build --release

# Opsional: package sebagai .AppImage portabel
cargo install cargo-appimage
cargo appimage
```

### macOS
```bash
# Jalankan langsung:
cargo run --release

# Build binary standalone:
cargo build --release

# Opsional: package sebagai bundle .app / .dmg
cargo install cargo-bundle
cargo bundle --release
```

---

## Hasil Pengujian (Automated Tests)

Test suite mencakup seluruh skenario Duplicate Finder, General Cleanup, Background Deletion, Cross-Platform Resolution, dan fitur v11 (export, history/undo, auto-scan, analisis disk):

1. `test_hasher_partial_and_full` — Verifikasi partial vs full hash integrity.
2. `test_end_to_end_scanner_detection` — Verifikasi scanner multi-folder end-to-end.
3. `test_selection_strategies` — Validasi Keep Oldest, Keep Newest, Keep Shortest Path.
4. `test_action_quarantine_execution` — Pemindahan ke karantina dan pencatatan audit log.
5. `test_zero_byte_files_filtered_by_default` — Filter file 0-byte.
6. `test_three_way_duplicates_across_nested_subfolders` — Multi-duplikat di folder bersarang.
7. `test_excluded_directories_are_skipped` — Direktori `node_modules`, `vendor`, dan sejenisnya otomatis dilewati.
8. `test_sensitive_files_default_unchecked` — File sensitif (`.env`, key) kebal auto-selection.
9. `test_scan_all_toggle_overrides_exclude` — Toggle scan all mem-bypass exclude list.
10. `test_large_dataset_virtualized_performance` — Simulasi 50.000 grup (150.000 file) terbukti memproses flat row mapping dalam waktu kurang dari 50ms tanpa frame drop.
11. `test_documentation_template_match_is_auto_selected` — Template dokumentasi (Kategori C) terdeteksi dan diikutsertakan dalam auto-suggest pembersihan.
12. `test_functional_template_match_is_protected_from_auto_selection` — Template fungsional (Kategori D) kebal dari auto-suggest di bawah seluruh strategi & tombol Pilih Semua.
13. `test_end_to_end_template_fingerprint_scanner` — Pengujian end-to-end scanner mendeteksi Kategori C dan D secara bersamaan.
14. `test_modified_framework_file_no_longer_matches_template` — File template yang dimodifikasi developer otomatis tidak lagi cocok dengan fingerprint.
15. `test_locked_file_skipped_gracefully` — File yang terkunci oleh proses lain dilewati tanpa crash dan dicatat pada audit log.
16. `test_installer_category_default_unchecked` — Kategori installer Downloads default tidak tercentang (`SafetyLevel::NeedsReview`).
17. `test_safe_categories_default_checked` — Kategori aman (temp, browser cache, logs, thumbnail, recycle bin, dev tools, apps) default tercentang (`SafetyLevel::Safe`).
18. `test_size_calculation_matches_actual_deletion` — Estimasi ukuran pra-pembersihan sama persis dengan ukuran yang dibebaskan setelah pembersihan.
19. `test_recycle_bin_query` — Query status Recycle Bin / Trash berjalan sukses via native API.
20. `test_delete_worker_background_execution` — Verifikasi eksekusi penghapusan di background thread tanpa memblokir UI thread.
21. `test_delete_worker_cancellation` — Verifikasi fitur pembatalan delete massal secara aman menggunakan atomic flag.
22. `test_cleanup_executor_cancellation` — Verifikasi fitur pembatalan pembersihan sistem general cleanup.
23. `test_platform_temp_dirs_valid` — Verifikasi ketersediaan direktori temporary lintas platform.
24. `test_platform_downloads_installer_patterns` — Verifikasi pattern installer (`.exe`/`.msi` di Windows, `.deb`/`.rpm`/`.appimage` di Linux, `.dmg`/`.pkg` di macOS).
25. `test_platform_dev_tools_cache_dirs` — Verifikasi resolver direktori cache developer tools (npm, pip, VS Code, Cargo).
26. `test_platform_consumer_apps_cache_dirs` — Verifikasi resolver direktori cache Discord dan Spotify.
27. `test_platform_recycle_bin_query` — Verifikasi query ukuran dan item Recycle Bin / Trash.
28. `test_linux_module_resolvers_do_not_panic` — Verifikasi path resolver Linux berjalan tanpa panic.
29. `test_macos_module_resolvers_do_not_panic` — Verifikasi path resolver macOS berjalan tanpa panic.
30. `test_installer_info_and_shortcut_creation` — Verifikasi info instalasi dan pembuatan shortcut Windows.
31. `test_export_duplicates_csv_and_json_roundtrip` — Verifikasi ekspor hasil duplicate finder ke CSV dan JSON.
32. `test_export_cleanup_csv_and_json_roundtrip` — Verifikasi ekspor hasil general cleanup ke CSV dan JSON.
33. `test_history_entry_is_restorable_only_for_recycle_bin_with_refs` — Verifikasi logika kelayakan undo pada riwayat pembersihan.
34. `test_history_action_kind_labels_are_distinct` — Verifikasi label jenis aksi riwayat.
35. `test_auto_scan_interval_seconds_mapping` — Verifikasi konversi interval auto-scan.
36. `test_app_settings_serde_roundtrip_in_memory` — Verifikasi serialisasi pengaturan aplikasi.
37. `test_folder_analyzer_computes_child_sizes_and_recurses_into_subfolders` — Verifikasi perhitungan ukuran folder analyzer, termasuk agregasi rekursif subfolder.

Seluruh 37 test lulus (100%).
