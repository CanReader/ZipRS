use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ProgressPayload {
    pub current: u64,
    pub total: u64,
    pub message: String,
}

/// Trait for progress reporting, used by all archive operations.
pub trait Progress: Send + Sync {
    fn send_progress(&self, current: u64, total: u64, message: String);
}

// --- GUI implementation (Tauri events) ---

#[derive(Clone)]
pub struct GuiProgress {
    app_handle: tauri::AppHandle,
}

impl GuiProgress {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self { app_handle }
    }
}

impl Progress for GuiProgress {
    fn send_progress(&self, current: u64, total: u64, message: String) {
        use tauri::Emitter;
        let _ = self.app_handle.emit(
            "archive-progress",
            ProgressPayload {
                current,
                total,
                message,
            },
        );
    }
}

// --- CLI implementation (stderr) ---

#[derive(Clone)]
pub struct CliProgress {
    pub verbose: bool,
}

impl CliProgress {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }
}

impl Progress for CliProgress {
    fn send_progress(&self, current: u64, total: u64, message: String) {
        if self.verbose {
            eprint!("\r\x1b[K[{}/{}] {}", current + 1, total, message);
            if current + 1 >= total {
                eprintln!();
            }
        }
    }
}
