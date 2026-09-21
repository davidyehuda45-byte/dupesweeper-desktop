use std::path::Path;

/// Complete hardcoded list of directory names that are excluded by default across frameworks.
/// All matching is case-insensitive.
pub const EXCLUDED_DIR_NAMES: &[&str] = &[
    // JavaScript / TypeScript / Web
    "node_modules",
    ".next",
    ".nuxt",
    ".svelte-kit",
    ".angular",
    "dist",
    "build",
    ".turbo",
    ".parcel-cache",
    ".vite",
    "out",
    ".cache",
    "bower_components",

    // PHP (Laravel, Symfony, WordPress)
    "vendor",
    "bootstrap/cache",
    ".phpunit.cache",

    // Flutter / Dart
    ".dart_tool",
    ".pub-cache",
    ".flutter-plugins",
    ".flutter-plugins-dependencies",
    ".symlinks",

    // Android Native / Java / Gradle
    ".gradle",
    "captures",
    ".externalnativebuild",
    ".cxx",

    // iOS / Xcode
    "pods",
    "deriveddata",
    ".build",
    "xcuserdata",

    // Java / Rust
    "target",

    // Python
    "venv",
    ".venv",
    "env",
    "__pycache__",
    ".tox",
    ".pytest_cache",
    ".mypy_cache",
    "site-packages",
    ".eggs",

    // .NET / C#
    "bin",
    "obj",
    ".vs",
    "packages",

    // Ruby
    ".bundle",

    // Universal / Version Control / IDE
    ".git",
    ".svn",
    ".hg",
    ".vscode",
    ".idea",

    // Package manager cache
    ".npm",
    ".yarn",
    ".pnpm-store",
];

/// Relative path substrings that should also trigger directory exclusion.
pub const EXCLUDED_PATH_SUBSTRINGS: &[&str] = &[
    "storage/framework/cache",
    "storage/framework/sessions",
    "storage/framework/views",
    "storage\\framework\\cache",
    "storage\\framework\\sessions",
    "storage\\framework\\views",
    "ios/pods",
    "ios\\pods",
    "ios/.symlinks",
    "ios\\.symlinks",
    "android/.gradle",
    "android\\.gradle",
    "android/app/build",
    "android\\app\\build",
    "vendor/bundle",
    "vendor\\bundle",
];

/// Checks whether a directory entry should be completely excluded from traversal.
pub fn is_excluded_directory(path: &Path) -> bool {
    let dir_name = match path.file_name().and_then(|n| n.to_str()) {
        Some(name) => name.to_lowercase(),
        None => return false,
    };

    // Exact directory name match
    if EXCLUDED_DIR_NAMES.iter().any(|&d| d.eq_ignore_ascii_case(&dir_name)) {
        return true;
    }

    // Python egg-info pattern: *.egg-info
    if dir_name.ends_with(".egg-info") {
        return true;
    }

    // Path pattern matching (e.g. storage/framework/cache)
    let path_str = path.to_string_lossy().to_lowercase();
    for pattern in EXCLUDED_PATH_SUBSTRINGS {
        if path_str.contains(pattern) {
            return true;
        }
    }

    false
}

