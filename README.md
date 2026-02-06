# 📎 Tiny Clippy

![Tiny Clippy Preview](tiny-clippy-preview.gif)

The legendary Microsoft Office Assistant, reborn in **Rust**.
Tiny Clippy is a lightweight, cross-platform desktop companion built with **egui**. No bloat, just pure nostalgia and performance.

## 🚀 Features
* **The Legend Returns:** The classic assistant you know and love (or hate).
* **Native Performance:** Built in Rust for minimal resource usage.
* **Cross-Platform:** Runs natively on Windows, Linux, and macOS (Apple Silicon supported).

---

## 📥 Installation

Download the latest binary for your architecture from the [Releases](https://github.com/salva-imm/tiny-clippy/releases) page.

### 🐧 Linux (AMD64 / ARM64)
```bash
# Grant execution permissions
chmod +x tiny-clippy-linux-amd64

# Run
./tiny-clippy-linux-amd64
```

### 🍎 macOS (Apple Silicon)
To bypass the "App is damaged" or "Unidentified Developer" warning:
```bash
# Remove quarantine attribute
xattr -cr tiny-clippy-macos-arm64

# Grant execution permission
chmod +x tiny-clippy-macos-arm64

# Run
./tiny-clippy-macos-arm64
```
> **Pro Tip:** If Gatekeeper still blocks you, **Right-Click** the file and select **Open**.

### 🪟 Windows (AMD64)
Download `tiny-clippy-windows-amd64.exe` and execute.

---

## 🛠 Building from Source

Ensure you have the Rust toolchain installed.

```bash
# Clone and build
git clone [https://github.com/salva-imm/tiny-clippy](https://github.com/salva-imm/tiny-clippy)
cd tiny-clippy
cargo build --release
```

**Note for Linux:** You may need `libwayland-dev`, `libx11-dev`, and `libxkbcommon-dev` installed to compile the GUI.

---
