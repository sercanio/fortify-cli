use arboard::Clipboard;
use std::env;

/// Result of a clipboard operation
pub enum ClipboardResult {
    Success,
    Unavailable,
    Error(String),
}

/// Main entry point for copying text to clipboard
pub fn copy_text(text: &str) -> ClipboardResult {
    // 1. Try to detect if we are in a headless environment
    if is_headless() {
        // Headless fallback not implemented
        return ClipboardResult::Unavailable;
    }

    // 2. Platform specific handling
    #[cfg(target_os = "linux")]
    {
        copy_linux(text)
    }
    #[cfg(not(target_os = "linux"))]
    {
        copy_direct(text)
    }
}

fn copy_direct(text: &str) -> ClipboardResult {
    match Clipboard::new() {
        Ok(mut clipboard) => {
            if let Err(e) = clipboard.set_text(text) {
                ClipboardResult::Error(e.to_string())
            } else {
                ClipboardResult::Success
            }
        },
        Err(e) => {
            // Often "Clipboard not available"
            ClipboardResult::Error(e.to_string())
        }
    }
}

#[cfg(target_os = "linux")]
fn copy_linux(text: &str) -> ClipboardResult {
    copy_direct(text)
}

fn is_headless() -> bool {
    // Simple heuristic: check for DISPLAY or WAYLAND_DISPLAY variables on Linux
    #[cfg(target_os = "linux")]
    {
        env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err()
    }
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}