use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub const PARTIAL_CHUNK_SIZE: usize = 4096;
pub const STREAM_BUFFER_SIZE: usize = 65536;

/// Computes the BLAKE3 hash of the first 4KB of a file.
pub fn compute_partial_hash(path: &Path) -> std::io::Result<blake3::Hash> {
    let mut file = File::open(path)?;
    let mut buffer = [0u8; PARTIAL_CHUNK_SIZE];
    let bytes_read = file.read(&mut buffer)?;
    Ok(blake3::hash(&buffer[..bytes_read]))
}

/// Computes the complete streaming BLAKE3 hash of a file.
/// Checks `cancel_flag` periodically during large reads.
pub fn compute_full_hash(
    path: &Path,
    cancel_flag: Option<&Arc<AtomicBool>>,
    on_bytes_read: Option<&dyn Fn(u64)>,
) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(STREAM_BUFFER_SIZE, file);
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0u8; STREAM_BUFFER_SIZE];

    loop {
        if let Some(cancel) = cancel_flag {
            if cancel.load(Ordering::Relaxed) {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Interrupted,
                    "Scan cancelled by user",
                ));
            }
        }

        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);

        if let Some(cb) = on_bytes_read {
            cb(n as u64);
        }
    }

    Ok(hasher.finalize().to_hex().to_string())
}

