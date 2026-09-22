<div align="center">

# DupeSweeper — 重复文件查找与系统清理工具
### 版本 11.0.0 — 导出、历史记录与撤销、自动扫描、磁盘分析

**极速、轻量、安全的多平台重复文件查找与系统垃圾清理工具**

[![Bahasa Indonesia](https://img.shields.io/badge/Language-Bahasa%20Indonesia-lightgrey?style=flat-square)](../../README.md)
[![English](https://img.shields.io/badge/Language-English-lightgrey?style=flat-square)](README.en.md)
[![简体中文](https://img.shields.io/badge/Language-简体中文-blue?style=flat-square)](README.zh.md)
[![हिन्दी](https://img.shields.io/badge/Language-हिन्दी-lightgrey?style=flat-square)](README.hi.md)

<p align="center">
  <b>选择语言 / Select Language:</b><br>
  <a href="../../README.md"><b>Bahasa Indonesia</b></a> &middot;
  <a href="README.en.md"><b>English</b></a> &middot;
  <a href="README.zh.md"><b>简体中文</b></a> &middot;
  <a href="README.hi.md"><b>हिन्दी</b></a>
</p>

</div>

---

**DupeSweeper** 是一款现代化、运行迅速且 100% 离线运行的桌面工具，全面支持 **Windows、Linux 和 macOS**。DupeSweeper 将高速的重复文件检测与系统垃圾清理功能整合到单个简洁的应用中：

1. **重复文件查找模式** — 基于密码学 BLAKE3 内容哈希精确比对文件内容，而非仅按文件名判断。
2. **系统深度清理模式** — 扫描并清理系统临时文件、各浏览器缓存、缩略图数据库、旧日志、Downloads 目录中残留的陈旧安装包，以及开发工具与常用软件缓存。
3. **磁盘分析模式** — 按大小排序浏览文件夹内容，支持逐层深入导航，帮助您找出占用磁盘空间的具体来源。

本软件编译为**体积小于 5.5 MB 的独立可执行文件**，无任何外部运行时依赖、无需复杂安装、无广告，且**不会向云端发送任何数据**。

---

## 直接下载与运行（免安装、免 Git Clone）

DupeSweeper 为绿色便携版独立应用。您**无需**克隆 GitHub 仓库，也无需安装 Rust 编译器。只需下载对应系统的单文件即可运行。

| 操作系统 | 应用程序文件 | 文件体积 | 运行方法 |
|---|---|:---:|---|
| **Windows（安装包）** | [DupeSweeper-Setup.exe](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/DupeSweeper-Setup.exe) | ~5.0 MB | 推荐。下载并安装，自动在桌面与开始菜单创建 "DupeSweeper" 快捷方式。 |
| **Windows（便携版）** | [DupeSweeper.exe](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/DupeSweeper.exe) | ~5.0 MB | 独立单文件版，下载后直接运行，无需安装。 |
| **Linux（x64）** | [dupesweeper-linux-x86_64](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/dupesweeper-linux-x86_64) | ~11.3 MB | 执行 `chmod +x dupesweeper-linux-x86_64`，再运行 `./dupesweeper-linux-x86_64` |
| **macOS（Universal）** | [dupesweeper-macos-universal](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/dupesweeper-macos-universal) | ~4.5 MB | 执行 `chmod +x dupesweeper-macos-universal`，再运行 `./dupesweeper-macos-universal` |

> 完整版本历史、更新日志与源代码归档，请前往 [GitHub Releases](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases) 查看。

---

### Windows SmartScreen 与杀毒软件提示说明

首次下载并运行 `DupeSweeper-Setup.exe` 或 `DupeSweeper.exe` 时，Windows Defender / SmartScreen 可能会弹出提示框：

> "Windows 已保护你的电脑"
> Microsoft Defender SmartScreen 阻止了无法识别的应用启动。

**出现该提示的原因：**
1. **免费开源软件。** DupeSweeper 是 100% 免费开源的软件。SmartScreen 会默认拦截未购买商业 EV 代码签名证书（每年费用约 300–500 美元）的 `.exe` 文件。
2. **新版本信誉尚未累积。** SmartScreen 依靠下载量的累积声誉判断风险，每次发布新版本都会先被标记为"未知"，直到有足够多用户下载。

**运行方法（仅首次需要）：**
1. 在 SmartScreen 提示框中，点击"更多信息"（More info）。
2. 点击随后出现的"仍要运行"（Run anyway）按钮。
3. DupeSweeper 即会正常启动。

> **安全与隐私保证：**
> - **100% 离线，零遥测。** 软件不会连接互联网，不收集任何使用数据，也不追踪用户。
> - **无需管理员权限。** 以标准用户权限（`asInvoker`）运行，绝不会请求 UAC 提权。
> - **透明开源。** 完整源代码可在本仓库自由审查，所有平台的二进制文件均通过 GitHub Actions CI 自动化、可复现地构建。

---

## v11.0.0 新特性

1. **导出报告（CSV/JSON）** — 将重复文件扫描结果或清理分析结果导出为 CSV 或 JSON 文件，便于删除前的审计或备份。
2. **历史记录与撤销** — 每次清理会话都会在本地记录。对于移入回收站的操作，DupeSweeper 可在应用内直接将文件还原到原始位置。
3. **自动扫描计划** — 在应用保持打开状态期间，按可配置的时间间隔（30 分钟至 24 小时）自动重新扫描上次使用的文件夹。
4. **磁盘分析** — 全新标签页，用于浏览目录中最大的文件夹与文件，并支持基于面包屑导航的逐层深入。

---

## 跨平台支持

### 跨平台底层抽象架构（`src/platform/`）
- **单一代码库。** 核心查重引擎与即时模式界面完全可移植，系统差异被清晰封装在 `src/platform/` 中：
  - `src/platform/windows.rs` — Win32 Shell32 回收站查询与清空、`%TEMP%` / `%LOCALAPPDATA%` 路径解析、Windows 资源管理器集成。
  - `src/platform/linux.rs` — Freedesktop.org 垃圾桶规范（`~/.local/share/Trash`）、标准 `/tmp`、`/var/tmp`、`~/.cache` 路径解析，以及 `xdg-open` 支持。
  - `src/platform/macos.rs` — Finder 废纸篓（`~/.Trash`）、`~/Library/Caches`、`~/Library/Application Support`、QuickLook 缩略图缓存，以及 `open -R`。
- **集成 `dirs` crate。** 以跨平台标准路径解析器（`dirs::cache_dir()`、`dirs::config_dir()`、`dirs::download_dir()`、`dirs::home_dir()`）取代硬编码的 Windows 环境变量读取。

### 多平台清理路径对照表

| 类别 | Windows | Linux | macOS |
|---|---|---|---|
| 临时文件 | `%TEMP%`, `C:\Windows\Temp` | `/tmp`, `/var/tmp` | `/private/var/folders`, `/tmp`, `~/Library/Caches` |
| Chrome 浏览器缓存 | `%LOCALAPPDATA%\Google\Chrome\User Data\Default\Cache` | `~/.cache/google-chrome/Default/Cache` | `~/Library/Caches/Google/Chrome/Default/Cache` |
| Firefox 浏览器缓存 | `%APPDATA%\Mozilla\Firefox\Profiles\*\cache2` | `~/.cache/mozilla/firefox/*/cache2` | `~/Library/Caches/Firefox/Profiles/*/cache2` |
| 缩略图缓存 | `thumbcache_*.db`（资源管理器） | `~/.cache/thumbnails` | `~/Library/Caches/com.apple.QuickLook.thumbnailcache` |
| 回收站 / 废纸篓 | Windows 回收站（Win32 Shell32） | `~/.local/share/Trash`（Freedesktop） | `~/.Trash`（Finder） |
| Downloads 旧安装包 | `*.exe`, `*.msi` | `*.deb`, `*.rpm`, `*.appimage`, `*.tar.gz` | `*.dmg`, `*.pkg` |
| npm 缓存 | `%APPDATA%\npm-cache` | `~/.npm` | `~/.npm` |
| pip 缓存 | `%LOCALAPPDATA%\pip\Cache` | `~/.cache/pip` | `~/Library/Caches/pip` |
| VS Code 缓存 | `%APPDATA%\Code\Cache` | `~/.config/Code/Cache` | `~/Library/Application Support/Code/Cache` |
| Cargo 注册表缓存 | `~/.cargo/registry/cache` | `~/.cargo/registry/cache` | `~/.cargo/registry/cache` |
| 常用软件缓存 | Discord, Spotify | Discord, Spotify | Discord, Spotify |

### 自动化多平台 CI/CD
- 仓库提供 GitHub Actions 工作流配置文件：`.github/workflows/ci.yml`。
- 每次 push 或 pull request 都会在 `windows-latest`、`ubuntu-latest`、`macos-latest` 上自动执行 `cargo test` 与 `cargo build --release`。

---

## 系统深度清理模式功能

可通过界面顶部导航标签切换的独立模式："查找重复文件" 与 "清理系统垃圾"。

### 清理类别

1. **临时文件** — 安装程序或应用崩溃遗留的临时文件（`*.tmp`、`*.temp`）。状态：安全，默认勾选。
2. **浏览器缓存（Chrome、Edge、Firefox）** — 仅清理纯缓存数据（`Cache`、`Code Cache`、`GPUCache`、`CacheStorage`）。安全原则：绝不触碰 Cookies、登录凭据、浏览历史或书签。状态：安全，默认勾选。
3. **旧日志与崩溃转储（超过 30 天）** — 系统错误日志（`*.log`）与崩溃转储文件（`*.dmp`）。状态：安全，默认勾选。
4. **缩略图缓存** — 文件管理器的缩略图预览缓存数据库，系统会在需要时自动重新生成。状态：安全，默认勾选。
5. **回收站 / 废纸篓内容** — 读取所有磁盘已占用的容量，确认后清空。状态：安全，默认勾选。
6. **Downloads 目录旧安装包（超过 60 天）** — 超过 60 天的安装文件（`*.exe`、`*.msi`、`*.deb`、`*.rpm`、`*.AppImage`、`*.dmg`、`*.pkg`）。默认不勾选，以避免误删仍需要的文件。状态：需人工审核。
7. **开发者工具缓存（npm、pip、VS Code、Cargo）** — 包管理器全局缓存，工具会在需要时自动重新下载。状态：安全，默认勾选。
8. **常用软件缓存（Discord、Spotify）** — Discord 与 Spotify 的本地存储缓存，应用再次使用时会自动刷新。状态：安全，默认勾选。

### 系统清理安全机制
- **优雅的错误处理** — 被其他程序锁定或占用的文件会自动跳过，不会导致程序崩溃。
- **预估计算（Dry-Run）** — 在用户点击清理按钮前，预先计算可释放的空间。
- **详情展开** — 每个分类卡片均可展开，查看其中具体的文件列表。
- **审计日志** — 所有清理操作都会记录到审计日志文件（`logs/dupesweeper_audit_*.txt`）。

---

## 模板指纹识别（重复文件模式）

DupeSweeper 内置智能系统，用于识别在众多项目中重复出现的框架初始模板文件（Laravel、Next.js、Vite React、Vite Vue）。

### C 类 — 文档模板（可安全删除）
- 初始模板自带的文档/元数据文件（`README.md`、`.gitignore`、`welcome.blade.php`、`vite.svg`、`react.svg` 等）。
- 标记为"默认模板（[框架名称]）"徽章。
- 行为：纳入自动推荐清理范围。

### D 类 — 功能性模板（受保护，不参与自动选择）
- 框架未经修改的功能性/运行时文件（`0001_01_01_000000_create_users_table.php`、`DatabaseSeeder.php`、`routes/web.php`、`src/app/page.tsx`、`vite.config.js` 等）。
- 标记为"框架原生（未修改）"徽章。
- 行为：在所有预设选择策略下均受保护，默认不勾选，不参与自动推荐清理。

---

## 性能特性

- **后台批量删除（`DeleteWorker`）** — 文件删除通过非阻塞的 crossbeam 通道在独立后台线程中执行，界面不会卡顿。提供"取消"按钮，可随时通过原子标志安全终止流程。
- **虚拟化 / 延迟滚动渲染（`show_rows`）** — 仅渲染当前处于屏幕可视区域内的行，在 10 万以上文件的数据集上滚动依然流畅。
- **默认折叠显示** — 重复文件组默认以折叠形式显示，并提供"全部展开"与"全部折叠"操作。
- **异步延迟缩略图工作线程** — 图片缩略图解码在独立后台线程中进行，不占用 UI 主线程。

---

## 分类矩阵

| 类别 | 名称 | 示例 | 扫描行为 | 选择行为 | UI 徽章 |
|---|---|---|---|---|---|
| A | 排除目录 | `node_modules`, `vendor`, `target`, `.git`, `.next`, `DerivedData`, `.cache` | 完全跳过 | 不适用 | 不适用 |
| B | 受保护的敏感文件 | `.env`, `*.key`, `*.pem`, `secrets.json`, SSH 密钥 | 仍会扫描并检测 | 默认不勾选，免疫自动选择 | 敏感 |
| C | 文档模板 | `README.md`, `.gitignore`, `welcome.blade.php`, `react.svg` | 扫描并通过 BLAKE3 匹配 | 自动推荐清理 | 默认模板（[框架]） |
| D | 功能性模板 | Laravel 数据迁移、`DatabaseSeeder.php`、`page.tsx`、`vite.config.js` | 扫描并通过 BLAKE3 匹配 | 默认不勾选，免疫自动选择 | 框架原生（未修改） |
| Clean | 系统与应用垃圾 | 临时文件、浏览器缓存、日志、缩略图、安装包 | 按位置与模式扫描 | 安全类默认开启，风险类默认关闭 | 安全 / 需审核 |

---

## 各平台构建与运行方式

### Windows
```powershell
# 从源代码直接运行：
cargo run --release

# 编译独立 .exe：
cargo build --release
# 可执行文件位于 target/release/dupesweeper.exe
```

### Linux
```bash
# 安装基础 GUI 依赖（Ubuntu/Debian）：
sudo apt-get install -y libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libasound2-dev

# 直接运行：
cargo run --release

# 编译独立二进制文件：
cargo build --release

# 可选：打包为便携版 .AppImage
cargo install cargo-appimage
cargo appimage
```

### macOS
```bash
# 直接运行：
cargo run --release

# 编译独立二进制文件：
cargo build --release

# 可选：打包为 .app / .dmg
cargo install cargo-bundle
cargo bundle --release
```

---

## 测试结果（自动化测试）

测试套件覆盖重复文件查找、系统清理、后台删除、跨平台路径解析，以及 v11 新增功能（导出、历史记录与撤销、自动扫描、磁盘分析）：

1. `test_hasher_partial_and_full` — 验证局部哈希与完整哈希的一致性。
2. `test_end_to_end_scanner_detection` — 验证多文件夹端到端扫描。
3. `test_selection_strategies` — 验证保留最旧、保留最新、保留最短路径策略。
4. `test_action_quarantine_execution` — 验证移入隔离区并记录审计日志。
5. `test_zero_byte_files_filtered_by_default` — 验证默认过滤零字节文件。
6. `test_three_way_duplicates_across_nested_subfolders` — 验证嵌套子文件夹中的多重复文件检测。
7. `test_excluded_directories_are_skipped` — 验证 `node_modules`、`vendor` 等目录自动跳过。
8. `test_sensitive_files_default_unchecked` — 验证敏感文件（`.env`、密钥）免疫自动选择。
9. `test_scan_all_toggle_overrides_exclude` — 验证"扫描全部"开关可绕过排除列表。
10. `test_large_dataset_virtualized_performance` — 模拟 5 万个分组（15 万文件），验证扁平化行映射在 50 毫秒内完成且无掉帧。
11. `test_documentation_template_match_is_auto_selected` — 验证文档模板（C 类）被检测并纳入自动推荐清理。
12. `test_functional_template_match_is_protected_from_auto_selection` — 验证功能性模板（D 类）在所有策略与"全选"操作下均受保护。
13. `test_end_to_end_template_fingerprint_scanner` — 验证扫描器可同时检测 C 类与 D 类模板。
14. `test_modified_framework_file_no_longer_matches_template` — 验证开发者修改过的模板文件不再匹配指纹。
15. `test_locked_file_skipped_gracefully` — 验证被其他进程锁定的文件被跳过而不崩溃，并记录到审计日志。
16. `test_installer_category_default_unchecked` — 验证 Downloads 安装包类别默认不勾选（`SafetyLevel::NeedsReview`）。
17. `test_safe_categories_default_checked` — 验证安全类别（临时文件、浏览器缓存、日志、缩略图、回收站、开发工具、应用缓存）默认勾选（`SafetyLevel::Safe`）。
18. `test_size_calculation_matches_actual_deletion` — 验证清理前的预估大小与实际释放空间完全一致。
19. `test_recycle_bin_query` — 验证通过原生 API 成功查询回收站 / 废纸篓状态。
20. `test_delete_worker_background_execution` — 验证删除操作在后台线程执行且不阻塞 UI 线程。
21. `test_delete_worker_cancellation` — 验证通过原子标志安全取消批量删除。
22. `test_cleanup_executor_cancellation` — 验证安全取消系统清理流程。
23. `test_platform_temp_dirs_valid` — 验证跨平台临时目录的可用性。
24. `test_platform_downloads_installer_patterns` — 验证安装包匹配模式（Windows 的 `.exe`/`.msi`，Linux 的 `.deb`/`.rpm`/`.appimage`，macOS 的 `.dmg`/`.pkg`）。
25. `test_platform_dev_tools_cache_dirs` — 验证开发工具缓存目录解析（npm、pip、VS Code、Cargo）。
26. `test_platform_consumer_apps_cache_dirs` — 验证 Discord 与 Spotify 缓存目录解析。
27. `test_platform_recycle_bin_query` — 验证回收站 / 废纸篓大小与项目数量查询。
28. `test_linux_module_resolvers_do_not_panic` — 验证 Linux 路径解析器运行时不会 panic。
29. `test_macos_module_resolvers_do_not_panic` — 验证 macOS 路径解析器运行时不会 panic。
30. `test_installer_info_and_shortcut_creation` — 验证 Windows 安装信息与快捷方式创建。
31. `test_export_duplicates_csv_and_json_roundtrip` — 验证重复文件结果导出为 CSV 与 JSON。
32. `test_export_cleanup_csv_and_json_roundtrip` — 验证系统清理结果导出为 CSV 与 JSON。
33. `test_history_entry_is_restorable_only_for_recycle_bin_with_refs` — 验证清理历史记录的可撤销判定逻辑。
34. `test_history_action_kind_labels_are_distinct` — 验证历史记录操作类型标签。
35. `test_auto_scan_interval_seconds_mapping` — 验证自动扫描时间间隔换算。
36. `test_app_settings_serde_roundtrip_in_memory` — 验证应用设置的序列化与反序列化。
37. `test_folder_analyzer_computes_child_sizes_and_recurses_into_subfolders` — 验证磁盘分析器的大小计算，包括子文件夹的递归汇总。

全部 37 项测试均通过（100%）。

```bash
cargo test
```
