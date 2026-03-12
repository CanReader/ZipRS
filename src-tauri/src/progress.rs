use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Clone, Serialize)]
pub struct ProgressPayload {
    pub current: u64,
    pub total: u64,
    pub message: String,
}

#[derive(Clone)]
pub struct ProgressSender {
    app_handle: AppHandle,
}

impl ProgressSender {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }

    pub fn send_progress(&self, current: u64, total: u64, message: String) {
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
