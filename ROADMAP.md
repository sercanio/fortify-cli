# Project Roadmap: fortify-cli

## Phase 1: Foundation & Setup
- [ ] Initialize Rust project (`cargo init`)
- [ ] Set up basic configuration (linting with `clippy`, formatting with `rustfmt`)
- [ ] Add core dependencies:
    - `clap` (CLI parsing)
    - `rand` (Cryptographic randomness)
    - `uuid` (GUID/UUID generation)
    - `base64` & `hex` (Secret encoding)
    - `dialoguer` or `inquire` (Interactive prompts)

## Phase 2: Core Logic Implementation
- [ ] **Password Module:**
    - [ ] Implement logic for custom length
    - [ ] Implement toggles for character sets: Lowercase, Uppercase, Numbers, Symbols
    - [ ] (Optional) Implement "pronounceable" password logic
- [ ] **GUID Module:**
    - [ ] Support UUID v4 (Random)
    - [ ] Support UUID v7 (Time-ordered)
- [ ] **Secret Module:**
    - [ ] Generate random bytes of N length
    - [ ] Encode as Hex or Base64

## Phase 3: Interactive Experience (The UI)
- [ ] **Main Menu:** Implement selection for "Password", "GUID", or "Secret"
- [ ] **Wizards:**
    - *Password Wizard:* Prompts for length, symbols, ambiguous characters
    - *Secret Wizard:* Prompts for length, encoding format
- [ ] **Output:** Ensure clear, colored output to stdout

## Phase 4: System Integration & Polish
- [ ] **Clipboard Support:** Integrate `arboard` to auto-copy results
- [ ] **Password Strength:** Integrate `zxcvbn-rs` for strength estimation
- [ ] **Configuration:** Implement saving default preferences

## Phase 5: Testing & Distribution
- [ ] Write Unit Tests for generation logic
- [ ] Write Integration Tests for CLI arguments
- [ ] Create `README.md` with usage examples
