# Fortify CLI

A robust CLI tool for generating strong passwords, GUIDs/UUIDs, and cryptographically secure secrets.

## Features

- **Password Generation**: Customizable length, character sets (upper, lower, numbers, symbols), and strength estimation using `zxcvbn`.
- **GUID/UUID**: Generate UUID v4 (Random) and v7 (Time-ordered).
- **Secrets**: Generate random bytes in Hex or Base64 encoding.
- **Interactive Mode**: Easy-to-use wizards if no arguments are provided.
- **Clipboard Integration**: Automatically copies generated values to clipboard (can be disabled).

## Installation

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
fortify-cli password --length 20 --no-symbols
```
Options:
- `--length <N>` (Default: 16)
- `--no-uppercase`
- `--no-lowercase`
- `--no-numbers`
- `--no-symbols`

**Generate a GUID:**
```bash
fortify-cli guid --version v4
```
Options:
- `--version <v4|v7>` (Default: v4)

**Generate a Secret:**
```bash
fortify-cli secret --length 64 --encoding base64
```
Options:
- `--length <N>` (Default: 32)
- `--encoding <hex|base64>` (Default: hex)

**Global Options:**
- `--no-copy`: Disable automatic clipboard copying.

## Testing

Run the test suite:
```bash
cargo test
```
