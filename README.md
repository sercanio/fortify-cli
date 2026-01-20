# Fortify CLI

A robust CLI tool for generating strong passwords, GUIDs/UUIDs, and cryptographically secure secrets.

## Features

- **Password Generation**: Customizable length, character sets (upper, lower, numbers, symbols), and strength estimation using `zxcvbn`.
- **GUID/UUID**: Generate UUID v4 (Random) and v7 (Time-ordered).
- **Secrets**: Generate random bytes in Hex or Base64 encoding.
- **Interactive Mode**: Easy-to-use wizards if no arguments are provided.
- **Clipboard Integration**: Automatically copies generated values to clipboard (can be disabled).

## Installation

### Unix (Linux/macOS)
```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/YOUR_USERNAME/fortify-cli/releases/latest/download/fortify-cli-installer.sh | sh
```

### Windows (PowerShell)
```powershell
powershell -c "irm https://github.com/YOUR_USERNAME/fortify-cli/releases/latest/download/fortify-cli-installer.ps1 | iex"
```

### Cargo Binstall
```bash
cargo binstall fortify-cli
```

### From Source
```bash
cargo install --path .
```

## Usage

### Interactive Mode
Simply run the tool without arguments:
```bash
fortify-cli
```

### Command Line Mode

**Generate a Password:**
```bash
# Default (16 chars, all character sets)
fortify-cli password

# Specific length and character sets (e.g., only numbers)
fortify-cli password --length 20 --numbers
```
Options:
- `--length <N>` (Default: 16)
- `--uppercase`
- `--lowercase`
- `--numbers`
- `--symbols`
(If no character sets are specified, all are enabled by default)

**Generate a GUID:**
```bash
fortify-cli guid --v4
```
Options:
- `--v4` (Default)
- `--v7` (Time-ordered)

**Generate a Secret:**
```bash
fortify-cli secret --length 64 --base64
```
Options:
- `--length <N>` (Default: 32)
- `--hex` (Default)
- `--base64`

**Global Options:**
- `--no-copy`: Disable automatic clipboard copying.

## Testing

Run the test suite:
```bash
cargo test
```
