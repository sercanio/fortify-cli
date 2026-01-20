# Clipboard Reliability Analysis

## 1. Executive Summary
The current clipboard implementation in `fortify-cli` uses the `arboard` crate. While functional for basic use cases, it exhibits significant reliability gaps on Linux platforms, specifically regarding data persistence on X11 and lack of support for native Wayland environments. Windows and macOS support is expected to be reliable.

## 2. Implementation Details
- **Library:** `arboard` (v3.6.1)
- **Method:** `Clipboard::new()` followed by `clipboard.set_text()`.
- **Error Handling:** Gracefully handles initialization failures (e.g., headless environments) by silently ignoring the error.

## 3. Platform-Specific Analysis

### 3.1 Linux (X11)
- **Status:** Partially Reliable (Environment Dependent).
- **The Issue:** The X11 clipboard protocol relies on the application *owning* the selection to provide the data when requested by another application ("paste"). Since `fortify-cli` is a short-lived CLI tool, it exits immediately after copying.
    - If a **Clipboard Manager** (e.g., `clipit`, `klipper`, Gnome/KDE built-in managers) is running, it will detect the copy event and persist the data.
    - If **NO Clipboard Manager** is running (common in minimal window managers like i3, bspwm, or bare X11 sessions), the clipboard content is lost the moment the process terminates.
- **Observed Behavior:** `arboard` detects the quick exit and prints a warning to stderr:
  ```text
  Clipboard was dropped very quickly after writing (3ms)...
  ```

### 3.2 Linux (Wayland)
- **Status:** Unreliable (Currently Unsupported).
- **The Issue:** The `arboard` crate requires the `wayland-data-control` feature to interface with the Wayland clipboard protocol natively.
- **Verification:** Inspection of `Cargo.lock` reveals no `wayland-client` or related dependencies.
- **Impact:** Users on Wayland compositors without XWayland (or if X11 fallback fails) will not have clipboard functionality. The app will silently fail to copy.

### 3.3 Windows & macOS
- **Status:** Reliable.
- **Details:** These OSs manage the clipboard globally. Once text is set, it persists after the application exits.

### 3.4 Headless / SSH
- **Status:** Good.
- **Details:** The application correctly catches the "clipboard unavailable" error and skips the copy operation without crashing or printing ugly stack traces.

## 4. Recommendations

### High Priority
1.  **Enable Wayland Support:**
    Update `Cargo.toml` to include the `wayland-data-control` feature for `arboard`.
    ```toml
    arboard = { version = "3.6.1", features = ["wayland-data-control"] }
    ```

### Medium Priority
2.  **Address X11 Persistence:**
    - **Option A (Daemonize):** Fork a background process to hold the clipboard content for a timeout period (e.g., 30 seconds) or until ownership is lost. This is complex to implement correctly in a cross-platform way.
    - **Option B (User Education):** Update `README.md` to inform Linux users that a Clipboard Manager is required for reliable operation on X11.
    - **Option C (Wait):** Add a flag (e.g., `--wait-paste`) to keep the process alive until the user presses Enter or a timeout occurs (less CLI-friendly).

3.  **Suppress/Handle Warnings:**
    - The stderr warning from `arboard` on X11 is helpful but might be confusing. We could verify if we can suppress it or wrap it with a more user-friendly message explaining *why* it might have failed.
