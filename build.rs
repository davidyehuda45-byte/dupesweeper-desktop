#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set("FileDescription", "DupeSweeper - Fast Duplicate File Finder & Cleaner");
    res.set("ProductName", "DupeSweeper");
    res.set("OriginalFilename", "dupesweeper.exe");
    res.set("LegalCopyright", "Copyright (c) 2026 David Yehuda Surbakti");
    res.set("CompanyName", "David Yehuda Surbakti");
    res.set("FileVersion", "6.0.0.0");
    res.set("ProductVersion", "6.0.0.0");
    
    if std::path::Path::new("assets/icon.ico").exists() {
        res.set_icon("assets/icon.ico");
    }

    res.set_manifest(r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<assemblyIdentity
    version="6.0.0.0"
    processorArchitecture="*"
    name="DavidYehudaSurbakti.DupeSweeper"
    type="win32"
/>
<description>DupeSweeper - Fast Duplicate File Finder and System Cleaner</description>
<trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
        <requestedPrivileges>
            <requestedExecutionLevel level="asInvoker" uiAccess="false"/>
        </requestedPrivileges>
    </security>
</trustInfo>
<compatibility xmlns="urn:schemas-microsoft-com:compatibility.v1">
    <application>
        <!-- Windows 10 and Windows 11 -->
        <supportedOS Id="{8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a}"/>
        <!-- Windows 8.1 -->
        <supportedOS Id="{1f676c76-80e1-4239-95bb-83d0f6d0da78}"/>
        <!-- Windows 8 -->
        <supportedOS Id="{4a2f28e3-53b9-4441-ba9c-d69d4a4a6e38}"/>
        <!-- Windows 7 -->
        <supportedOS Id="{35138b9a-5d96-4fbd-8e2d-a2440225f93a}"/>
    </application>
</compatibility>
<application xmlns="urn:schemas-microsoft-com:asm.v3">
    <windowsSettings>
        <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true/pm</dpiAware>
        <dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2, PerMonitor</dpiAwareness>
        <longPathAware xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">true</longPathAware>
    </windowsSettings>
</application>
</assembly>
"#);
    
    if let Err(e) = res.compile() {
        eprintln!("Failed to compile Windows resources: {}", e);
    }
}

#[cfg(not(windows))]
fn main() {}
