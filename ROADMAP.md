# Project Roadmap: fortify-cli

## Phase 1: Foundation & Setup
- [x] Initialize Rust project (`cargo init`)
- [x] Set up basic configuration (linting with `clippy`, formatting with `rustfmt`)
- [x] Add core dependencies:
    - `clap` (CLI parsing)
    - `rand` (Cryptographic randomness)
    - `uuid` (GUID/UUID generation)
    - `base64` & `hex` (Secret encoding)
    - `dialoguer` or `inquire` (Interactive prompts)

## Phase 2: Core Logic Implementation
- [x] **Password Module:**
    - [x] Implement logic for custom length
    - [x] Implement toggles for character sets: Lowercase, Uppercase, Numbers, Symbols
    - [ ] (Optional) Implement "pronounceable" password logic
- [x] **GUID Module:**
    - [x] Support UUID v4 (Random)
    - [x] Support UUID v7 (Time-ordered)
- [x] **Secret Module:**
    - [x] Generate random bytes of N length
    - [x] Encode as Hex or Base64

## Phase 3: Interactive Experience (The UI)
- [x] **Main Menu:** Implement selection for "Password", "GUID", or "Secret"
- [x] **Wizards:**
    - *Password Wizard:* Prompts for length, symbols, ambiguous characters
    - *Secret Wizard:* Prompts for length, encoding format
- [x] **Output:** Ensure clear, colored output to stdout

## Phase 4: System Integration & Polish
- [ ] **Clipboard Support:** Integrate `arboard` to auto-copy results
    - *Partially implemented but unreliable on Linux.*
- [x] **Password Strength:** Integrate `zxcvbn-rs` for strength estimation
- [ ] **Configuration:** Implement saving default preferences

## Phase 5: Testing & Distribution
- [x] Write Unit Tests for generation logic
- [x] Write Integration Tests for CLI arguments
- [x] Create `README.md` with usage examples

---

## Phase 6: Clipboard Reliability Refactor (CURRENT PRIORITY)

**Goal:** Ensure 100% reliable clipboard operations on Linux (X11 & Wayland) and graceful fallback for Headless/SSH sessions.

### 6.1 Architecture & Modularization
- [ ] **Create `src/clipboard.rs` module:**
    - Abstract all clipboard logic away from `main.rs`.
    - Define a `ClipboardManager` struct/trait to handle state and backend selection.

### 6.2 Linux X11 Persistence
- [ ] **Implement Fork/Daemon Strategy:**
    - On X11, after copying, fork a background process that holds the clipboard lock.
    - The background process should run for a short duration (e.g., 45s) or until the selection is lost.
    - Prevents data loss when the main CLI process exits.
- [ ] **External Tool Fallback:**
    - Detect `xclip` or `xsel` and utilize them if `arboard` fails or for better persistence handling if needed.

### 6.3 Linux Wayland Support
- [ ] **Native Protocol Check:**
    - Verify `arboard`'s `wayland-data-control` feature is functioning.
- [ ] **External Tool Fallback:**
    - Support `wl-copy` as a fallback if the native library fails to connect to the wayland socket.

### 6.4 Headless & SSH Support (OSC 52)
- [ ] **Detect TTY/SSH:**
    - Identify if the session is headless (no `DISPLAY` or `WAYLAND_DISPLAY`).
- [ ] **Implement OSC 52:**
    - Use ANSI escape sequences (OSC 52) to write directly to the user's terminal clipboard.
    - This allows copying even from a remote SSH session to the local machine.

### 6.5 Testing & Verification
- [ ] **Mocking:**
    - Create a `MockClipboard` for unit testing the logic without side effects.
- [ ] **Integration Tests:**
    - Add tests that simulate clipboard failure to ensure no panic occurs.
    - (Optional) Use `xvfb` in CI to test actual X11 copying.

### 6.6 User Feedback
- [ ] **Verbose/Debug Mode:**
    - Provide clear feedback: "Copied to clipboard (OSC 52)" or "Copied to clipboard (X11 persistence enabled)".
- [ ] **Error Handling:**
    - "Failed to copy to clipboard: [Reason]. Printed to stdout only."