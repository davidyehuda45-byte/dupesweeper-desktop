# ⚡ DupeSweeper - Duplicate File Finder & Cleaner
### Version 5.0.0 — Background Bulk Deletion + UI/UX Overhaul + General Cleanup

**DupeSweeper** adalah aplikasi desktop modern, super cepat, dan 100% offline untuk Windows yang menggabungkan solusi pencarian file duplikat berkecepatan tinggi dengan pembersihan sampah sistem dalam satu aplikasi:
1. **🔍 Mode Duplicate Finder:** Mendeteksi dan membersihkan file duplikat berdasarkan **content hash** asli menggunakan algoritma **BLAKE3**.
2. **🧹 Mode General Cleanup:** Mendeteksi dan membersihkan file sampah sistem & aplikasi umum (file sementara, cache browser, thumbnail, log lama, installer lama di Downloads, dan cache developer/consumer tools).

Aplikasi ini dikompilasi menjadi **single binary `.exe` mandiri berukuran sangat ringkas (< 5 MB)** tanpa dependensi runtime tambahan, tanpa instalasi rumit, tanpa iklan, dan **0 byte data dikirim ke cloud**.

---

## 🚀 Fitur Baru di v5.0.0

### 1. Fix UI Freeze saat Bulk Delete (Asynchronous Background Worker)
- **Eliminasi UI Freeze:** Pemindahan operasi penghapusan file (`trash::delete`, `fs::remove_file`, isolasi karantina) dari thread UI utama ke background worker thread terpisah menggunakan `crossbeam-channel`.
- **Dedicated Progress Screen:** Tampilan antarmuka khusus saat pembersihan berjalan yang menampilkan progress bar real-time, nama file yang sedang diproses, dan akumulasi ruang penyimpanan yang berhasil dibebaskan.
- **Dukungan Pembatalan (Cancellation Support):** Pengguna dapat membatalkan proses pembersihan kapan saja dengan aman via atomic flag (`Arc<AtomicBool>`). Pembersihan berhenti dengan rapi tanpa merusak state aplikasi maupun integritas file sistem.

### 2. UI/UX Overhaul & Modern Design System
- **⚡ Startup Splash Screen:** Layar pembuka elegan selama ~650ms dengan animasi pulsing logo petir (`⚡`), indikator loading ramping, dan transisi mulus ke landing screen.
- **📊 Modern Progress Bar Component (`ModernProgressBar`):** Komponen progress bar modern dengan fill animasi, persentase kontras tinggi, dan detail informasi dinamis.
- **🎉 Empty State Visuals (`EmptyState`):** Tampilan visual yang ramah dan jelas saat hasil scan 0 item (misal: drive bersih dari duplikat).
- **🎨 Elevated Dark Theme & 8px Grid:** Sistem desain konsisten berbasis kelipatan 8px, sudut rounded yang serasi, palet multi-level elevation (`#0E1015`, `#161922`, `#1E232F`, `#282F3E`), dan aksen amber hangat (`#F59E0B`).

---

## 🧹 Fitur Mode General Cleanup

Mode terpisah yang dapat dipilih langsung dari header atas melalui tab navigasi: **"🔍 Cari File Duplikat"** vs **"🧹 Bersihkan Sampah Sistem"**.

### Kategori File Sampah yang Dibersihkan:

1. **📁 Temporary Files (`%TEMP%`, `%LOCALAPPDATA%\Temp`, `C:\Windows\Temp`):**
   - File sementara (*.tmp, *.temp) sisa installer atau crash aplikasi.
   - *Status:* **Aman (Default Tercentang)**.
2. **🌐 Cache Browser (Chrome, Edge, Firefox):**
   - Cache murni (`Cache`, `Code Cache`, `GPUCache`, `CacheStorage`).
   - **Prinsip Keamanan Absolut:** **TIDAK PERNAH menyentuh Cookies, Login Data, Riwayat (History), maupun Bookmarks!**
   - *Status:* **Aman (Default Tercentang)**.
3. **📜 Log Files & Crash Dump Lama (> 30 Hari):**
   - File log error (*.log), crash dump (*.dmp), dan Windows Error Reporting (`%LOCALAPPDATA%\Microsoft\Windows\WER`).
   - *Status:* **Aman (Default Tercentang)**.
4. **🖼️ Thumbnail Cache Windows:**
   - File cache `thumbcache_*.db` dan `Thumbs.db`.
   - Aman dihapus karena Windows akan membuat ulang secara otomatis saat dibutuhkan.
   - *Status:* **Aman (Default Tercentang)**.
5. **🗑️ Isi Recycle Bin:**
   - Membaca kapasitas total seluruh drive via Win32 Shell32 native API dan opsi mengosongkan dengan konfirmasi.
   - *Status:* **Aman (Default Tercentang)**.
6. **💿 Installer & Setup Lama di Downloads (> 60 Hari):**
   - File installer (*.exe, *.msi) di folder Downloads yang sudah berumur lebih dari 60 hari.
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
- **Graceful Error Handling:** File yang sedang dikunci (*locked*) atau sedang dipakai oleh Windows/aplikasi lain otomatis dilewati (*skipped*) tanpa crash/panic.
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

- **Virtualized / Lazy Scrolling (`show_rows`):**
  - Mengatasi kendala lag pada hasil scan berskala masif (puluhan hingga ratusan ribu file).
  - Hanya merender baris yang tepat berada di dalam viewport layar. Scroll tetap **60 FPS halus tanpa lag** pada dataset **100.000+ file**!
- **Collapsed by Default:**
  - Grup duplikat tampil ringkas secara default (*collapsed*). Tersedia tombol **"▶ Buka Semua"** dan **"▼ Tutup Semua"**.
- **Asynchronous Lazy Thumbnail Worker:**
  - Decoding thumbnail gambar berlangsung di background thread terpisah tanpa membebani UI thread.

---

## 📋 Matriks Klasifikasi Kategori

| Kategori | Nama | Contoh | Perilaku Scan | Perilaku Seleksi | Badge UI |
|---|---|---|---|---|---|
| **A** | Excluded Directories | `node_modules`, `vendor`, `target`, `.git`, `.next` | Di-skip total | N/A | N/A |
| **B** | Protected Sensitive Files | `.env`, `*.key`, `*.pem`, `secrets.json`, SSH keys | Tetap di-scan & dideteksi | **Default unchecked**, kebal auto-select | `⚠️ Sensitif` |
| **C** | Template Dokumentasi | `README.md`, `.gitignore`, `welcome.blade.php`, `react.svg` | Di-scan & dicocokkan BLAKE3 | **Auto-suggested** untuk dibersihkan | `🗑️ Template Default ([Framework])` |
| **D** | Template Fungsional | Migrasi Laravel, `DatabaseSeeder.php`, `page.tsx`, `vite.config.js` | Di-scan & dicocokkan BLAKE3 | **Default unchecked**, kebal auto-select | `📋 Bawaan [Framework] (Belum Dimodifikasi)` |
| **Clean** | System & App Junk | Temp files, browser cache, logs, thumbnail, installers | Scan lokasi & pola file | **Safe = ON, Risky = OFF** | `Aman` / `⚠️ Perlu Review` |

---

## 🚀 Cara Menjalankan

### Opsi 1: Menjalankan Binary Langsung (Paling Mudah)
Cukup double-click file executable di root folder:
```text
dupesweeper.exe
```
*(Ukuran hanya ~4.7 MB, langsung jalan tanpa instalasi apapun)*

### Opsi 2: Compile & Run dari Source Code (Rust)
```bash
# Jalankan langsung dalam mode release
cargo run --release

# Atau jalankan test suite (22 automated unit & integration tests)
cargo test
```

---

## 🧪 Hasil Pengujian (Automated Tests)

Test suite mencakup seluruh skenario Duplicate Finder, General Cleanup, dan Background Deletion:
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
19. `test_recycle_bin_query`: Query status Recycle Bin Windows berjalan sukses via Win32 Shell32 API.
20. `test_delete_worker_background_execution`: Verifikasi eksekusi penghapusan di background thread tanpa memblokir UI thread (< 50ms return).
21. `test_delete_worker_cancellation`: Verifikasi fitur pembatalan delete massal secara aman menggunakan atomic flag.
22. `test_cleanup_executor_cancellation`: Verifikasi fitur pembatalan pembersihan sistem general cleanup.

Semua 22 test lulus (**100% OK**).
