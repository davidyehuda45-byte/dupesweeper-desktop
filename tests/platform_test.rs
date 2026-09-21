use dupesweeper::platform;

#[test]
fn test_platform_temp_dirs_valid() {
    let temp_dirs = platform::get_temp_dirs();
    assert!(!temp_dirs.is_empty(), "get_temp_dirs() must return at least 1 path");
    println!("Platform temp dirs: {:?}", temp_dirs);
}

#[test]
fn test_platform_downloads_installer_patterns() {
    let (dirs, exts) = platform::get_downloads_installer_patterns();
    assert!(!exts.is_empty(), "Installer file extensions must not be empty");

    #[cfg(target_os = "windows")]
    {
        assert!(exts.contains(&"exe"), "Windows must include .exe installer");
        assert!(exts.contains(&"msi"), "Windows must include .msi installer");
    }

    #[cfg(target_os = "linux")]
    {
        assert!(exts.contains(&"deb") || exts.contains(&"appimage"), "Linux must include .deb or .appimage");
    }

    #[cfg(target_os = "macos")]
    {
        assert!(exts.contains(&"dmg") || exts.contains(&"pkg"), "macOS must include .dmg or .pkg");
    }

    println!("Downloads dirs: {:?}, installer extensions: {:?}", dirs, exts);
}

#[test]
fn test_platform_dev_tools_cache_dirs() {
    let dev_dirs = platform::get_dev_tools_cache_dirs();
    // Verify does not panic and returns valid list
    println!("Platform dev tools cache dirs: {:?}", dev_dirs);
}

#[test]
fn test_platform_consumer_apps_cache_dirs() {
    let app_dirs = platform::get_consumer_apps_cache_dirs();
    println!("Platform consumer apps cache dirs: {:?}", app_dirs);
}

#[test]
fn test_platform_recycle_bin_query() {
    let (bytes, items) = platform::query_recycle_bin();
    println!("Platform recycle bin query: {} bytes, {} items", bytes, items);
}

#[test]
fn test_linux_module_resolvers_do_not_panic() {
    // Test pure-Rust Linux path resolution logic
    let linux_temps = platform::linux::get_temp_dirs();
    assert!(linux_temps.iter().any(|p| p.to_string_lossy().contains("tmp")));

    let (linux_dl, linux_exts) = platform::linux::get_downloads_installer_patterns();
    assert!(linux_exts.contains(&"deb"));
    assert!(linux_exts.contains(&"rpm"));
    assert!(linux_exts.contains(&"appimage"));
    println!("Linux DL: {:?}, exts: {:?}", linux_dl, linux_exts);

    let linux_chrome = platform::linux::get_chrome_cache_dirs();
    let linux_dev = platform::linux::get_dev_tools_cache_dirs();
    let linux_apps = platform::linux::get_consumer_apps_cache_dirs();
    println!("Linux Chrome: {:?}, Dev: {:?}, Apps: {:?}", linux_chrome, linux_dev, linux_apps);
}

#[test]
fn test_macos_module_resolvers_do_not_panic() {
    // Test pure-Rust macOS path resolution logic
    let mac_temps = platform::macos::get_temp_dirs();
    println!("macOS temps: {:?}", mac_temps);

    let (mac_dl, mac_exts) = platform::macos::get_downloads_installer_patterns();
    assert!(mac_exts.contains(&"dmg"));
    assert!(mac_exts.contains(&"pkg"));
    println!("macOS DL: {:?}, exts: {:?}", mac_dl, mac_exts);

    let mac_chrome = platform::macos::get_chrome_cache_dirs();
    let mac_dev = platform::macos::get_dev_tools_cache_dirs();
    let mac_apps = platform::macos::get_consumer_apps_cache_dirs();
    println!("macOS Chrome: {:?}, Dev: {:?}, Apps: {:?}", mac_chrome, mac_dev, mac_apps);
}

