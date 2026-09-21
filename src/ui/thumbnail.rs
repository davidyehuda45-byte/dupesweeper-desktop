use crossbeam_channel::{unbounded, Receiver, Sender};
use eframe::egui;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::thread;

pub struct ThumbnailCache {
    cache: HashMap<PathBuf, Option<egui::TextureHandle>>,
    pending: HashSet<PathBuf>,
    request_tx: Sender<(PathBuf, u32)>,
    result_rx: Receiver<(PathBuf, Option<egui::ColorImage>)>,
}

impl Default for ThumbnailCache {
    fn default() -> Self {
        Self::new()
    }
}

impl ThumbnailCache {
    pub fn new() -> Self {
        let (request_tx, request_rx): (Sender<(PathBuf, u32)>, Receiver<(PathBuf, u32)>) = unbounded();
        let (result_tx, result_rx): (
            Sender<(PathBuf, Option<egui::ColorImage>)>,
            Receiver<(PathBuf, Option<egui::ColorImage>)>,
        ) = unbounded();

        // Spawn 2 background worker threads for thumbnail decoding
        for _ in 0..2 {
            let rx = request_rx.clone();
            let tx = result_tx.clone();
            thread::spawn(move || {
                while let Ok((path, max_size)) = rx.recv() {
                    let image_result = Self::decode_thumbnail(&path, max_size);
                    let _ = tx.send((path, image_result));
                }
            });
        }

        Self {
            cache: HashMap::new(),
            pending: HashSet::new(),
            request_tx,
            result_rx,
        }
    }

    fn decode_thumbnail(path: &Path, max_size: u32) -> Option<egui::ColorImage> {
        let img = image::open(path).ok()?;
        let thumb = img.thumbnail(max_size, max_size);
        let rgba = thumb.to_rgba8();
        let size = [rgba.width() as usize, rgba.height() as usize];
        let pixels = rgba.into_raw();
        Some(egui::ColorImage::from_rgba_unmultiplied(size, &pixels))
    }

    pub fn is_image_extension(path: &Path) -> bool {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            matches!(
                ext.to_lowercase().as_str(),
                "jpg" | "jpeg" | "png" | "webp" | "bmp" | "gif" | "ico"
            )
        } else {
            false
        }
    }

    /// Drains any finished background decodes and uploads them to the GPU texture manager.
    pub fn update(&mut self, ctx: &egui::Context) {
        let mut new_loaded = false;
        while let Ok((path, maybe_image)) = self.result_rx.try_recv() {
            self.pending.remove(&path);
            let texture = maybe_image.map(|color_image| {
                ctx.load_texture(
                    format!("thumb_{}", path.display()),
                    color_image,
                    egui::TextureOptions::LINEAR,
                )
            });
            self.cache.insert(path, texture);
            new_loaded = true;
        }

        if new_loaded {
            ctx.request_repaint();
        }
    }

    /// Retrieves an existing texture handle or schedules non-blocking background decode.
    pub fn get_or_load(
        &mut self,
        _ctx: &egui::Context,
        path: &Path,
        max_size: u32,
    ) -> Option<egui::TextureHandle> {
        if let Some(cached) = self.cache.get(path) {
            return cached.clone();
        }

        if self.pending.contains(path) {
            return None;
        }

        if !Self::is_image_extension(path) {
            self.cache.insert(path.to_path_buf(), None);
            return None;
        }

        // Schedule background decode
        self.pending.insert(path.to_path_buf());
        let _ = self.request_tx.send((path.to_path_buf(), max_size));
        None
    }

    pub fn clear(&mut self) {
        self.cache.clear();
        self.pending.clear();
    }
}
