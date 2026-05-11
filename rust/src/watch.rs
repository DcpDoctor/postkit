use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::mpsc;

/// Watch event type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WatchEventKind {
    Created,
    Modified,
    Removed,
}

/// A file system watch event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchEvent {
    pub kind: WatchEventKind,
    pub paths: Vec<PathBuf>,
}

/// Watch action triggered on events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchAction {
    pub name: String,
    /// Glob pattern to match against event paths
    pub pattern: String,
    /// Command to execute when pattern matches
    pub command: String,
}

/// File system watcher with action pipeline.
pub struct FileWatcher {
    _watcher: RecommendedWatcher,
    rx: mpsc::Receiver<Result<Event, notify::Error>>,
    actions: Vec<WatchAction>,
}

impl FileWatcher {
    /// Create a new watcher on the given directory.
    pub fn new(dir: &Path) -> Result<Self, notify::Error> {
        let (tx, rx) = mpsc::channel();
        let mut watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            Config::default(),
        )?;
        watcher.watch(dir, RecursiveMode::Recursive)?;

        Ok(Self {
            _watcher: watcher,
            rx,
            actions: Vec::new(),
        })
    }

    /// Add an action to execute when a matching event occurs.
    pub fn add_action(&mut self, action: WatchAction) {
        self.actions.push(action);
    }

    /// Poll for the next event (blocking).
    pub fn next_event(&self) -> Option<WatchEvent> {
        match self.rx.recv() {
            Ok(Ok(event)) => {
                let kind = match event.kind {
                    notify::EventKind::Create(_) => WatchEventKind::Created,
                    notify::EventKind::Modify(_) => WatchEventKind::Modified,
                    notify::EventKind::Remove(_) => WatchEventKind::Removed,
                    _ => return None,
                };
                Some(WatchEvent {
                    kind,
                    paths: event.paths,
                })
            }
            _ => None,
        }
    }

    /// Poll with timeout (non-blocking).
    pub fn try_next_event(&self, timeout: std::time::Duration) -> Option<WatchEvent> {
        match self.rx.recv_timeout(timeout) {
            Ok(Ok(event)) => {
                let kind = match event.kind {
                    notify::EventKind::Create(_) => WatchEventKind::Created,
                    notify::EventKind::Modify(_) => WatchEventKind::Modified,
                    notify::EventKind::Remove(_) => WatchEventKind::Removed,
                    _ => return None,
                };
                Some(WatchEvent {
                    kind,
                    paths: event.paths,
                })
            }
            _ => None,
        }
    }
}
