# How to Install ShadowCrypt

This guide covers everything you need to install and run ShadowCrypt on your system.

---

## Table of Contents

1. [System Requirements](#system-requirements)
2. [Installing Rust](#installing-rust)
3. [Getting the Source Code](#getting-the-source-code)
4. [Building the Game](#building-the-game)
5. [Running the Game](#running-the-game)
6. [Troubleshooting](#troubleshooting)

---

## System Requirements

### Minimum Requirements

- **Operating System:** Windows 10+, macOS 10.15+, or Linux (any modern distribution)
- **Terminal:** A terminal emulator with UTF-8 support
- **Disk Space:** ~100 MB for Rust toolchain + ~10 MB for the game
- **RAM:** 128 MB minimum
- **Rust Version:** 1.75.0 or newer (uses Rust 2024 edition)

### Recommended Terminal Emulators

| OS | Recommended Terminals |
|----|----------------------|
| **Windows** | Windows Terminal, Alacritty, Hyper |
| **macOS** | iTerm2, Terminal.app, Alacritty |
| **Linux** | Alacritty, Kitty, GNOME Terminal, Konsole |

**Note:** The game uses the `crossterm` library for terminal rendering and requires a terminal that supports:
- ANSI escape codes
- 256 colors (or true color)
- UTF-8 character encoding

---

## Installing Rust

ShadowCrypt is written in Rust and requires the Rust toolchain to compile.

### Method 1: Using rustup (Recommended)

The recommended way to install Rust is through `rustup`, the official Rust version manager.

#### On Linux or macOS

Open your terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions. When prompted, choose the default installation (option 1).

After installation, configure your shell:

```bash
source $HOME/.cargo/env
```

Or restart your terminal.

#### On Windows

1. Download the rustup installer from: https://rustup.rs
2. Run the downloaded `rustup-init.exe`
3. Follow the on-screen instructions
4. Choose the default installation when prompted
5. Restart your terminal or command prompt

### Method 2: Using Package Managers

#### Arch Linux
```bash
sudo pacman -S rust
```

#### Fedora
```bash
sudo dnf install rust cargo
```

#### Ubuntu/Debian
```bash
sudo apt install rustc cargo
```

#### macOS (Homebrew)
```bash
brew install rust
```

#### Windows (Chocolatey)
```powershell
choco install rust
```

#### Windows (Scoop)
```powershell
scoop install rust
```

### Verifying Installation

After installation, verify that Rust is properly installed:

```bash
rustc --version
cargo --version
```

You should see output similar to:
```
rustc 1.75.0 (82e1608df 2023-12-21)
cargo 1.75.0 (1d8b05cdd 2023-11-20)
```

### Updating Rust

If you already have Rust installed, make sure it's up to date:

```bash
rustup update
```

---

## Getting the Source Code

### Option 1: Clone with Git

If you have Git installed, clone the repository:

```bash
git clone https://github.com/yourusername/shadowcrypt.git
cd shadowcrypt
```

### Option 2: Download ZIP

1. Navigate to the repository on GitHub
2. Click the green "Code" button
3. Select "Download ZIP"
4. Extract the ZIP file to your desired location
5. Open a terminal in the extracted folder

### Option 3: If You Already Have the Source

If you received the source files directly, navigate to the game directory:

```bash
cd /path/to/roguelike
```

---

## Building the Game

### Debug Build (Faster Compilation)

For testing and development:

```bash
cargo build
```

The executable will be created at:
```
target/debug/shadowcrypt
```

### Release Build (Optimized, Recommended)

For the best performance:

```bash
cargo build --release
```

The optimized executable will be created at:
```
target/release/shadowcrypt
```

**Note:** The release build takes longer to compile but runs significantly faster.

### Build Options

| Command | Description |
|---------|-------------|
| `cargo build` | Debug build |
| `cargo build --release` | Optimized release build |
| `cargo build -j 4` | Build using 4 parallel jobs |

---

## Running the Game

### Running from Cargo

The easiest way to run the game:

```bash
cargo run --release
```

This will build (if needed) and run the game.

### Running the Executable Directly

After building, you can run the executable directly:

#### Linux/macOS
```bash
./target/release/shadowcrypt
```

#### Windows
```cmd
.\target\release\shadowcrypt.exe
```

### Installing System-Wide (Optional)

To install the game to your system's binary path:

```bash
cargo install --path .
```

Then run from anywhere:
```bash
shadowcrypt
```

---

## Troubleshooting

### Common Issues

#### 1. "rustc: command not found"

**Problem:** Rust is not installed or not in your PATH.

**Solution:**
- Ensure Rust is installed (see [Installing Rust](#installing-rust))
- Restart your terminal
- On Linux/macOS, run: `source $HOME/.cargo/env`

#### 2. "error: edition 2024 is not yet stable"

**Problem:** Your Rust version is too old.

**Solution:**
```bash
rustup update
rustup default stable
```

If you need Rust 2024 edition specifically:
```bash
rustup default nightly
```

#### 3. Compilation Errors with crossterm

**Problem:** Missing system dependencies for terminal handling.

**Solution (Linux):**
```bash
# Ubuntu/Debian
sudo apt install build-essential

# Fedora
sudo dnf groupinstall "Development Tools"

# Arch
sudo pacman -S base-devel
```

#### 4. "terminal does not support colors"

**Problem:** Your terminal doesn't support ANSI colors.

**Solution:**
- Use a modern terminal emulator (see [Recommended Terminals](#recommended-terminal-emulators))
- Set the `TERM` environment variable:
  ```bash
  export TERM=xterm-256color
  ```

#### 5. Characters Display Incorrectly

**Problem:** Terminal font doesn't support required characters.

**Solution:**
- Use a font with good Unicode support (e.g., JetBrains Mono, Fira Code, DejaVu Sans Mono)
- Ensure your terminal is set to UTF-8 encoding

#### 6. Screen Size Issues

**Problem:** The game doesn't render correctly or text is cut off.

**Solution:**
- Resize your terminal window to at least 120x50 characters
- Use a smaller font size if needed
- Check that your terminal supports the minimum dimensions

#### 7. Input Not Working

**Problem:** Keyboard input is not recognized.

**Solution:**
- Ensure your terminal is in focus
- Try a different terminal emulator
- On some systems, raw mode may require specific terminal settings

#### 8. Windows-Specific: ANSI Escape Codes Not Working

**Problem:** Colors and special characters don't render on Windows.

**Solution:**
- Use Windows Terminal (recommended)
- Enable virtual terminal processing in CMD:
  ```cmd
  reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1
  ```
- Or run in PowerShell with proper settings

#### 9. Build Takes Too Long

**Problem:** Compilation is very slow.

**Solution:**
- Use more CPU cores: `cargo build -j$(nproc)`
- For development, use debug builds: `cargo build`
- Consider using `sccache` for faster recompilation
- Increase available RAM

#### 10. Game Runs Slowly

**Problem:** Poor performance during gameplay.

**Solution:**
- Use the release build: `cargo build --release`
- Ensure no other heavy applications are running
- Try a faster terminal emulator (Alacritty is very fast)

### Getting Help

If you encounter issues not covered here:

1. **Check the error message** - Rust provides helpful error descriptions
2. **Search online** - Many Rust issues have solutions on Stack Overflow
3. **Open an issue** - Report bugs on the project's issue tracker

### Reporting Bugs

When reporting a bug, include:
- Your operating system and version
- Your Rust version (`rustc --version`)
- Your terminal emulator
- The complete error message
- Steps to reproduce the issue

---

## Quick Start Summary

```bash
# 1. Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. Navigate to game directory
cd /path/to/roguelike

# 3. Build and run
cargo run --release
```

---

## Dependencies

ShadowCrypt uses the following Rust crates:

| Crate | Version | Purpose |
|-------|---------|---------|
| crossterm | 0.28 | Cross-platform terminal manipulation |
| rand | 0.8 | Random number generation |

All dependencies are automatically downloaded and compiled by Cargo.

---

## Uninstalling

### Remove Build Artifacts
```bash
cargo clean
```

### Remove System-Wide Installation
```bash
cargo uninstall shadowcrypt
```

### Remove Rust Entirely (Optional)
```bash
rustup self uninstall
```

---

*Happy adventuring in the depths of ShadowCrypt!*
