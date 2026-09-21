use std::path::Path;

/// Specific sensitive file names (exact match, case-insensitive)
pub const EXACT_SENSITIVE_FILE_NAMES: &[&str] = &[
    ".env",
    "secrets.json",
    "credentials.json",
    "appsettings.json",
    "id_rsa",
    "id_dsa",
    "id_ecdsa",
    "id_ed25519",
    ".npmrc",
    ".netrc",
];

/// Sensitive file extensions (case-insensitive)
pub const SENSITIVE_EXTENSIONS: &[&str] = &[
    "pem",
    "key",
    "pfx",
    "p12",
    "crt",
    "keystore",
    "jks",
];

/// Checks whether a given file path matches the Protected Sensitive Files list.
pub fn is_sensitive_file(path: &Path) -> bool {
    let file_name = match path.file_name().and_then(|n| n.to_str()) {
        Some(name) => name.to_lowercase(),
        None => return false,
    };

    // 1. Exact file name match
    if EXACT_SENSITIVE_FILE_NAMES.iter().any(|&name| name.eq_ignore_ascii_case(&file_name)) {
        return true;
    }

    // 2. Pattern .env.* (e.g. .env.local, .env.development, .env.production, .env.staging, etc.)
    if file_name.starts_with(".env.") {
        return true;
    }

    // 3. Pattern appsettings.*.json (e.g. appsettings.Development.json)
    if file_name.starts_with("appsettings.") && file_name.ends_with(".json") {
        return true;
    }

    // 4. Sensitive file extensions (.pem, .key, .pfx, .p12, .crt, .keystore, .jks)
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext_lower = ext.to_lowercase();
        if SENSITIVE_EXTENSIONS.iter().any(|&s_ext| s_ext == ext_lower) {
            return true;
        }
    }

    // 5. Cloud/Service Credentials or credential directories (.aws, .gcp, .ssh, .gnupg)
    let path_str = path.to_string_lossy().to_lowercase();
    if path_str.contains(".ssh/") || path_str.contains(".ssh\\")
        || path_str.contains(".gnupg/") || path_str.contains(".gnupg\\")
        || path_str.contains(".aws/credentials") || path_str.contains(".aws\\credentials")
        || (path_str.contains(".gcp/") && file_name.ends_with(".json"))
        || (path_str.contains(".gcp\\") && file_name.ends_with(".json"))
    {
        return true;
    }

    false
}

