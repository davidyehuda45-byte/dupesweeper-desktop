//! Cross-platform Recycle Bin / Trash API delegation.

/// Queries the total size and item count in the Recycle Bin / Trash across the current OS.
pub fn query_recycle_bin() -> (u64, u64) {
    crate::platform::query_recycle_bin()
}

/// Empties the Recycle Bin / Trash on the current OS.
pub fn empty_recycle_bin() -> Result<(), String> {
    crate::platform::empty_recycle_bin()
}
