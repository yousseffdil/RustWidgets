use anyhow::{Context, Result};
use notify::{Watcher, RecursiveMode, Event};
use std::path::Path;
use std::sync::mpsc::channel;

pub struct FileLoader;

impl FileLoader {
    pub fn load_file(path: &Path) -> Result<String> {
        std::fs::read_to_string(path)
            .context(format!("No se pudo leer el archivo: {:?}", path))
    }

    pub fn watch_file<F>(path: &Path, mut callback: F) -> Result<notify::RecommendedWatcher>
    where
        F: FnMut() + Send + 'static,
    {
        let (tx, rx) = channel();

        let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            if let Ok(_event) = res {
                let _ = tx.send(());
            }
        })?;

        watcher.watch(path, RecursiveMode::NonRecursive)?;

        // Spawn thread para manejar eventos
        std::thread::spawn(move || {
            while rx.recv().is_ok() {
                callback();
            }
        });

        Ok(watcher)
    }
}