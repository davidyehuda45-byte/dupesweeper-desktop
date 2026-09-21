<div align="center">

# ⚡ DupeSweeper - 重复文件查找与系统清理工具
### 版本 6.0.0 — 全面支持跨平台桌面（Windows、Linux、macOS）

**极速、轻量、安全的多平台重复文件查找与系统垃圾清理工具**

[![Bahasa Indonesia](https://img.shields.io/badge/Language-🇮🇩%20Bahasa%20Indonesia-lightgrey?style=flat-square)](../../README.md)
[![English](https://img.shields.io/badge/Language-🇺🇸%20English-lightgrey?style=flat-square)](README.en.md)
[![简体中文](https://img.shields.io/badge/Language-🇨🇳%20简体中文-blue?style=flat-square)](README.zh.md)
[![हिन्दी](https://img.shields.io/badge/Language-🇮🇳%20हिन्दी-lightgrey?style=flat-square)](README.hi.md)

<p align="center">
  <b>🌍 选择语言 / Select Language:</b><br>
  <a href="../../README.md">🇮🇩 <b>Bahasa Indonesia</b></a> •
  <a href="README.en.md">🇺🇸 <b>English</b></a> •
  <a href="README.zh.md">🇨🇳 <b>简体中文</b></a> •
  <a href="README.hi.md">🇮🇳 <b>हिन्दी</b></a>
</p>

</div>

---

**DupeSweeper** 是一款现代化、运行极快且 100% 离线运行的桌面工具，全面支持 **Windows、Linux 和 macOS**。DupeSweeper 将极速的重复文件检测与系统垃圾清理功能无缝整合到单个简洁的应用中：
1. **🔍 重复文件查找模式：** 基于密码学 **BLAKE3 内容哈希** 精确比对文件内容，绝不仅按文件名判断。
2. **🧹 系统深度清理模式：** 扫描并清理系统临时文件、各浏览器缓存、缩略图数据库、旧日志、Downloads 目录中残留的陈旧安装包，以及开发工具与日常软件缓存。

本软件编译为**体积小于 5 MB 的独立可执行文件**，无任何外部运行时依赖、无需复杂安装、无广告骚扰，且**不会向云端发送任何字节的数据**。

---

## 🚀 直接下载与运行（免安装、免 Git Clone）

DupeSweeper 为**绿色便携版独立应用**。您**无需**克隆 GitHub 仓库，也**无需**安装 Rust 编译器。只需下载对应系统的单文件即可双击启动：

| 操作系统 | 桌面应用程序（点击直接下载） | 文件体积 | 运行方法 |
|---|---|:---:|---|
| **🪟 Windows (x64)** | [⬇️ **dupesweeper-windows-x86_64.exe**](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/dupesweeper-windows-x86_64.exe) | ~5.0 MB | 下载后**双击** `.exe` 即可瞬间启动！ |
| **🐧 Linux (x64)** | [⬇️ **dupesweeper-linux-x86_64**](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/dupesweeper-linux-x86_64) | ~11.3 MB | 终端执行 `chmod +x dupesweeper-linux-x86_64` 并运行 `./dupesweeper-linux-x86_64` |
| **🍎 macOS (Universal)** | [⬇️ **dupesweeper-macos-universal**](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases/download/v6.0.0/dupesweeper-macos-universal) | ~4.5 MB | 终端执行 `chmod +x dupesweeper-macos-universal` 并运行 `./dupesweeper-macos-universal` |

> 💡 **完整版本历史：** 可在 [GitHub Releases](https://github.com/davidyehuda45-byte/dupesweeper-desktop/releases) 查看所有发布历史、更新日志及源代码归档。

---

### 🛡️ Windows SmartScreen 与杀毒软件说明

首次在 Windows 上下载并运行 `dupesweeper-windows-x86_64.exe` 时，Windows Defender / SmartScreen 可能会弹出蓝色提示框：
> **"Windows 已保护你的电脑"**  
> *Microsoft Defender SmartScreen 阻止了无法识别的应用启动。*

#### 🤔 为什么会出现该提示？
1. **独立免费开源项目：** DupeSweeper 是一款 100% 免费开源的软件。微软 SmartScreen 会默认拦截未购买昂贵商业企业代码签名证书（每年需 \$300–\$500）的 `.exe` 可执行文件。
2. **新版本下载信誉机制：** SmartScreen 依靠下载量累积声誉。每次发布新版本时，文件在达到足够的下载基数之前会被标记为“未知”。

#### 🚀 启动方法（仅首次需要）：
1. 在 SmartScreen 蓝色弹窗中，点击下方的 **"更多信息" (More info)** 文本链接。
2. 点击右下角出现的 **"仍要运行" (Run anyway)** 按钮。
3. DupeSweeper 软件将立即顺畅启动！

> 🔒 **100% 安全与隐私保证：**
> - **100% 纯离线（零遥测）：** 软件不会连接任何互联网服务器，不收集任何用户隐私或使用分析。
> - **无需管理员权限：** 以标准普通用户权限（`asInvoker`）安全运行，绝不会弹出 UAC 提权请求。
> - **透明开源：** 全部源代码公开透明，所有发布版本均由 GitHub Actions CI 自动化构建，安全可复现。

---

## 🌐 v6.0.0 新特性：全平台桌面支持

### 1. 跨平台底层抽象架构 (`src/platform/`)
- **单一代码库：** 核心查重引擎与 immediate-mode 界面 100% 可移植。系统专属操作封装在 `src/platform/` 中：
  - `src/platform/windows.rs`: Win32 Shell32 回收站状态查询与清空、`%TEMP%` / `%LOCALAPPDATA%` 路径解析、文件资源管理器定位。
  - `src/platform/linux.rs`: Freedesktop.org 规范垃圾桶 (`~/.local/share/Trash`)、标准 `/tmp`、`/var/tmp`、`~/.cache` 解析及 `xdg-open` 支持。
  - `src/platform/macos.rs`: Finder 废纸篓 (`~/.Trash`)、`~/Library/Caches`、QuickLook 缩略图缓存及 `open -R` 定位。
- **集成 `dirs` Crate：** 全面替代硬编码环境变量，采用跨平台标准的系统路径解析。

### 2. 多平台系统清理支持矩阵

| 分类 | Windows | Linux | macOS |
|---|---|---|---|
| **临时文件** | `%TEMP%`, `C:\Windows\Temp` | `/tmp`, `/var/tmp` | `/private/var/folders`, `/tmp`, `~/Library/Caches` |
| **Chrome 浏览器缓存** | `%LOCALAPPDATA%\Google\Chrome\User Data\Default\Cache` | `~/.cache/google-chrome/Default/Cache` | `~/Library/Caches/Google/Chrome/Default/Cache` |
| **Firefox 浏览器缓存** | `%APPDATA%\Mozilla\Firefox\Profiles\*\cache2` | `~/.cache/mozilla/firefox/*/cache2` | `~/Library/Caches/Firefox/Profiles/*/cache2` |
| **缩略图缓存** | `thumbcache_*.db` (资源管理器) | `~/.cache/thumbnails` | `~/Library/Caches/com.apple.QuickLook.thumbnailcache` |
| **回收站 / 废纸篓** | Windows 回收站 (Win32 Shell32) | `~/.local/share/Trash` (Freedesktop) | `~/.Trash` (Finder) |
| **Downloads 旧安装包** | `*.exe`, `*.msi` | `*.deb`, `*.rpm`, `*.appimage`, `*.tar.gz` | `*.dmg`, `*.pkg` |
| **npm 缓存** | `%APPDATA%\npm-cache` | `~/.npm` | `~/.npm` |
| **pip 缓存** | `%LOCALAPPDATA%\pip\Cache` | `~/.cache/pip` | `~/Library/Caches/pip` |
| **VS Code 缓存** | `%APPDATA%\Code\Cache` | `~/.config/Code/Cache` | `~/Library/Application Support/Code/Cache` |
| **Cargo 注册表缓存** | `~/.cargo/registry/cache` | `~/.cargo/registry/cache` | `~/.cargo/registry/cache` |
| **日常应用缓存** | Discord, Spotify | Discord, Spotify | Discord, Spotify |

---

## 🧹 系统深度清理模式

可在界面顶部导航栏一键切换：**"🔍 查找重复文件"** 与 **"🧹 清理系统垃圾"**。

### 清理类别概览：
1. **📁 临时文件：** 安装器或程序崩溃遗留的无用临时文件（*.tmp, *.temp）。*(默认：勾选 / 安全)*
2. **🌐 浏览器缓存 (Chrome, Edge, Firefox)：** 仅清理网页资源缓存。**严格安全原则：绝不触碰 Cookies、登录凭据、浏览历史记录与书签！** *(默认：勾选 / 安全)*
3. **📜 日志与崩溃转储 (> 30 天)：** 超过 30 天的系统错误日志 (*.log) 与 dump 文件 (*.dmp)。*(默认：勾选 / 安全)*
4. **🖼️ 缩略图数据库缓存：** 图片预览缓存，系统需要时会自动重新生成。*(默认：勾选 / 安全)*
5. **🗑️ 回收站 / 废纸篓：** 一键检测全盘已占用的废纸篓容量，确认后安全清空。*(默认：勾选 / 安全)*
6. **💿 Downloads 目录旧安装包 (> 60 天)：** 超过 60 天的陈旧安装文件。**安全策略：默认不勾选**，避免误删您仍需要的软件包。*(默认：未勾选 / 需手动确认)*
7. **🛠️ 开发者工具缓存 (npm, pip, VS Code, Cargo)：** 包管理器全局缓存，必要时可自动重新下载。*(默认：勾选 / 安全)*
8. **🎧 常用软件本地缓存 (Discord, Spotify)：** 媒体本地临时缓存，随应用使用自动刷新。*(默认：勾选 / 安全)*

---

## 🎯 模板指纹智能识别 (重复文件模式)

DupeSweeper 搭载智能模板指纹算法，能精准辨识常见 Web 框架（Laravel、Next.js、Vite React、Vite Vue）生成的初始代码模板：

- **🗑️ C 类 — 文档与示例模板（安全删除）：** 初始模板说明文档 (`README.md`, `.gitignore`, `welcome.blade.php`, `vite.svg` 等)。标记为 `🗑️ 默认模板 ([框架名称])`，纳入自动推荐删除。
- **📋 D 类 — 功能性运行时模板（智能保护）：** 框架未经修改的核心代码或数据迁移文件 (`0001_01_01_000000_create_users_table.php`, `DatabaseSeeder.php`, `page.tsx`, `vite.config.js` 等)。标记为 `📋 框架原生 (未修改)`，**并在所有预设策略下受到绝对保护，默认绝不勾选**。

---

## ⚡ 卓越性能架构

- **后台异步批量删除 (`DeleteWorker`)：** 文件删除在独立的后台工作线程中进行，UI 界面保持绝对流畅，并支持通过原子信号随时安全“取消”。
- **虚拟化视口渲染 (`show_rows`)：** 仅绘制当前屏幕可见的数据行。即使面对 **100,000+ 个文件**，滚动依然稳定在 **60 FPS**！
- **异步延迟缩略图解码：** 图片缩略图解码在后台工作线程中异步执行，不占用主 UI 线程。

---

## 🚀 从源代码构建

```bash
# 克隆仓库
git clone https://github.com/davidyehuda45-byte/dupesweeper-desktop.git
cd dupesweeper-desktop

# 运行应用：
cargo run --release

# 编译独立二进制程序：
cargo build --release
```

所有 29 项单元与集成测试均 100% 通过：
```bash
cargo test
```
