# Tiny Clippy

![Tiny Clippy Preview](tiny-clippy-preview.gif)

A lightweight, cross-platform implementation of the classic Microsoft Office Assistant built with Rust and egui.

## Features
* **Native Performance:** Compiled Rust binary with minimal memory footprint.
* **Cross-Platform:** Native support for Linux (x86_64/ARM64), macOS (Apple Silicon), and Windows.
* **No Dependencies:** Single-binary execution with no external runtime requirements (except standard graphics libs on Linux).

---

## Installation

Download the binary for your architecture from the [Releases](https://github.com/salva-imm/tiny-clippy/releases) page.

### Linux (AMD64 / ARM64)
```bash
chmod +x tiny-clippy-linux-amd64
./tiny-clippy-linux-amd64
```

### macOS (Apple Silicon)
To bypass the "App is damaged" or "Unidentified Developer" gatekeeper warning:
```bash
# Remove quarantine attribute
xattr -cr tiny-clippy-macos-arm64

# Grant execution permission
chmod +x tiny-clippy-macos-arm64

# Run
./tiny-clippy-macos-arm64
```
*Note: If macOS still prevents execution, Right-Click the file and select Open.*

### Windows (AMD64)
Download `tiny-clippy-windows-amd64.exe` and execute.

---

## Building from Source

Ensure you have a working Rust toolchain installed.

```bash
git clone [https://github.com/salva-imm/tiny-clippy](https://github.com/salva-imm/tiny-clippy)
cd tiny-clippy
cargo build --release
```

**Linux Build Dependencies:**
Standard egui requirements apply: `libwayland-dev`, `libx11-dev`, and `libxkbcommon-dev`.

---
