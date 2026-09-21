#[cfg(target_os = "windows")]
mod win32 {
    #[repr(C)]
    pub struct SHQUERYRBINFO {
        pub cb_size: u32,
        pub i64_size: i64,
        pub i64_num_items: i64,
    }

    #[link(name = "shell32")]
    extern "system" {
        pub fn SHQueryRecycleBinW(
            psz_root_path: *const u16,
            p_sh_query_rb_info: *mut SHQUERYRBINFO,
        ) -> i32;

        pub fn SHEmptyRecycleBinW(
            hwnd: *mut std::ffi::c_void,
            psz_root_path: *const u16,
            dw_flags: u32,
        ) -> i32;
    }

    pub const SHERB_NOCONFIRMATION: u32 = 0x00000001;
    pub const SHERB_NOPROGRESSUI: u32 = 0x00000002;
    pub const SHERB_NOSOUND: u32 = 0x00000004;
}

/// Queries the total size and item count in the Recycle Bin across common drives.
pub fn query_recycle_bin() -> (u64, u64) {
    #[cfg(target_os = "windows")]
    {
        use win32::*;

        let mut total_bytes: u64 = 0;
        let mut total_items: u64 = 0;

        // Query available drive letters C: through Z:
        for drive_letter in b'C'..=b'Z' {
            let root = format!("{}:\\\0", drive_letter as char);
            let wide: Vec<u16> = root.encode_utf16().collect();

            let mut info = SHQUERYRBINFO {
                cb_size: std::mem::size_of::<SHQUERYRBINFO>() as u32,
                i64_size: 0,
                i64_num_items: 0,
            };

            unsafe {
                let res = SHQueryRecycleBinW(wide.as_ptr(), &mut info);
                if res == 0 {
                    if info.i64_size > 0 {
                        total_bytes += info.i64_size as u64;
                    }
                    if info.i64_num_items > 0 {
                        total_items += info.i64_num_items as u64;
                    }
                }
            }
        }

        (total_bytes, total_items)
    }

    #[cfg(not(target_os = "windows"))]
    {
        (0, 0)
    }
}

/// Empties the Recycle Bin on all drives.
pub fn empty_recycle_bin() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use win32::*;

        unsafe {
            let flags = SHERB_NOCONFIRMATION | SHERB_NOPROGRESSUI | SHERB_NOSOUND;
            let res = SHEmptyRecycleBinW(std::ptr::null_mut(), std::ptr::null(), flags);
            if res == 0 {
                Ok(())
            } else {
                Err(format!("SHEmptyRecycleBinW error code: {:#x}", res))
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        Ok(())
    }
}

