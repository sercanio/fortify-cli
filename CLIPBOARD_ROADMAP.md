# Comprehensive Clipboard Reliability Roadmap

## 1. Goal
Achieve 100% reliable clipboard copy functionality on Linux (X11 & Wayland), Windows, and macOS, including support for headless environments (SSH) and "bare" window managers.

## 2. Current Architecture & Issues
- **Library:** `arboard` (v3.6.1) with `wayland-data-control` feature.
- **Problem (X11):** X11 clipboard data is owned by the source window. When `fortify-cli` exits, the window is destroyed, and clipboard data is lost unless a Clipboard Manager is running.
- **Problem (Headless):** Currently fails silently or prints minimal error. No fallback for SSH users (e.g., OSC 52).
- **Status (Wayland):** Theoretical support exists via `wayland-data-control`, but persistence issues may parallel X11 depending on the compositor's implementation of the data control protocol.

## 3. Proposed Solution: The "Worker" Strategy (Pure Rust)
To ensure persistence on Linux without requiring users to install external tools (like `xclip` or `wl-copy`), we will implement a "Daemon/Worker" pattern.

### 3.1 The Logic flow
1.  **Detection:** The CLI detects it is running on Linux.
2.  **Spawn:** Instead of setting clipboard directly and exiting, the main process spawns a detached child process: `fortify-cli internal-clipboard-worker`.
3.  **Data Transfer:** The main process writes the secret content to the child's `stdin` via a pipe.
4.  **Main Exit:** The main process prints "Copied to clipboard" and exits immediately.
5.  **Worker Action:**
    - Reads content from `stdin`.
    - Initializes `arboard`.
    - Sets clipboard content.
    - **Persist:** Sleeps for 45 seconds (or until ownership is lost, if detectable), keeping the "window" alive to serve paste requests.
    - Exits automatically after timeout.

### 3.2 Benefits
- **Reliability:** Works on minimal WMs (i3, bspwm) without a clipboard manager.
- **Zero-Dep:** No need for `xclip` / `xsel` runtime dependencies.
- **Security:** Data is passed via pipe (not argv), hidden from `ps` inspection.

## 4. Implementation Plan

### Phase 1: Modularization (Immediate)
- Extract clipboard logic from `main.rs` to `src/clipboard.rs`.
- Define `ClipboardService` trait/struct.
- **Deliverable:** Clean `main.rs`, isolated clipboard logic.

### Phase 2: The Worker Subcommand
- Add a hidden `clap` subcommand: `internal-clipboard-worker`.
- Implement the "read from stdin and wait" logic in this subcommand.
- **Deliverable:** The app can act as its own clipboard daemon.

### Phase 3: Main Process Orchestration
- Update `ClipboardService` to spawn the worker on Linux.
- Implement fallback: If spawning fails, try direct copy (best effort).
- **Deliverable:** Seamless experience for the user.

### Phase 4: Headless/SSH Support (OSC 52)
- Detect if `$DISPLAY` and `$WAYLAND_DISPLAY` are missing.
- If missing, emit ANSI OSC 52 escape sequence to copy to the terminal emulator's clipboard.
- **Deliverable:** Copying works over SSH.

### Phase 5: Testing
- **Unit Tests:** Mock the worker spawning to ensure logic correctness.
- **Integration:** Test the hidden subcommand behavior.

## 5. Technical Specifications

### New Module: `src/clipboard.rs`

```rust
pub enum ClipboardResult {
    Success,
    Ignored(String), // e.g. "No display"
    Error(String),
}

pub fn copy_to_clipboard(text: String, use_worker: bool) -> ClipboardResult {
    // Platform detection logic
    // Worker spawning logic
}
```

## 6. Verification Steps
1.  **X11 (i3/bare):** Run `fortify-cli password`, close terminal immediately, try to paste in another app.
2.  **Wayland (Sway/Hyprland):** Verify `wayland-data-control` interaction.
3.  **SSH:** Connect to server, run `fortify-cli`, verify local paste (OSC 52).
