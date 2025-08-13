---
title: Installation
description: How to install excel-to-json on your system
---

# Installation Guide

excel-to-json can be installed in several ways depending on your platform and preferences. Choose the method that works best for you.

## Prerequisites

### For Building from Source
If you plan to install from source or using Cargo, you'll need:
- **Rust**: Version 1.70 or later ([Install Rust](https://rustup.rs/))
- **Cargo**: Comes with Rust installation

### For Pre-built Binaries
- No prerequisites required!

## Installation Methods

### Method 1: Install with Cargo (Recommended)

The easiest way to install excel-to-json is using Cargo, Rust's package manager:

```bash
cargo install excel-to-json
```

This will:
1. Download the latest version from crates.io
2. Compile it for your platform
3. Install the binary to `~/.cargo/bin/`

Make sure `~/.cargo/bin` is in your PATH:

```bash
# Add to your shell profile (.bashrc, .zshrc, etc.)
export PATH="$HOME/.cargo/bin:$PATH"
```

### Method 2: Download Pre-built Binary

Download the latest release for your platform from the [GitHub Releases](https://github.com/ryan-ajility/excel-to-json/releases) page.

#### macOS
```bash
# Download the binary (replace VERSION with actual version)
curl -L https://github.com/ryan-ajility/excel-to-json/releases/download/VERSION/excel-to-json-macos -o excel-to-json

# Make it executable
chmod +x excel-to-json

# Move to a directory in your PATH
sudo mv excel-to-json /usr/local/bin/
```

#### Linux
```bash
# Download the binary (replace VERSION with actual version)
wget https://github.com/ryan-ajility/excel-to-json/releases/download/VERSION/excel-to-json-linux -O excel-to-json

# Make it executable
chmod +x excel-to-json

# Move to a directory in your PATH
sudo mv excel-to-json /usr/local/bin/
```

#### Windows
1. Download `excel-to-json-windows.exe` from the releases page
2. Rename to `excel-to-json.exe`
3. Add the directory containing the executable to your PATH

### Method 3: Build from Source

Clone the repository and build with Cargo:

```bash
# Clone the repository
git clone https://github.com/ryan-ajility/excel-to-json.git
cd excel-to-json

# Build and install
cargo build --release
cargo install --path .
```

### Method 4: Using Homebrew (macOS)

If you're on macOS and have Homebrew installed:

```bash
# Add the tap (if available)
brew tap ryan-ajility/excel-to-json

# Install
brew install excel-to-json
```

## Verify Installation

After installation, verify that excel-to-json is working:

```bash
# Check version
excel-to-json --version

# Display help
excel-to-json --help
```

You should see output similar to:
```
excel-to-json 1.0.0
A fast Excel to JSON converter written in Rust

USAGE:
    excel-to-json [OPTIONS] <FILE>
...
```

## Platform-Specific Notes

### macOS

On macOS, you might encounter a security warning when running the binary for the first time. To resolve this:

1. Go to **System Preferences → Security & Privacy**
2. Click **Allow Anyway** for excel-to-json
3. Or use this command: `xattr -d com.apple.quarantine excel-to-json`

### Linux

Ensure you have glibc 2.17 or later:
```bash
ldd --version
```

### Windows

- Use PowerShell or Command Prompt to run excel-to-json
- Add `.exe` extension when running: `excel-to-json.exe data.xlsx`

## Docker Installation

You can also run excel-to-json using Docker:

```dockerfile
# Dockerfile
FROM rust:1.70 as builder
WORKDIR /app
RUN cargo install excel-to-json

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/excel-to-json /usr/local/bin/
ENTRYPOINT ["excel-to-json"]
```

Build and run:
```bash
docker build -t excel-to-json .
docker run -v $(pwd):/data excel-to-json /data/file.xlsx
```

## Updating excel-to-json

### Using Cargo
```bash
cargo install excel-to-json --force
```

### Using Pre-built Binaries
Download the latest version and replace the existing binary.

## Uninstallation

### Installed with Cargo
```bash
cargo uninstall excel-to-json
```

### Installed Manually
```bash
# Remove from common locations
sudo rm /usr/local/bin/excel-to-json
rm ~/.local/bin/excel-to-json
```

## Troubleshooting

### Command Not Found

If you get "command not found" after installation:

1. Check if the binary exists:
   ```bash
   ls ~/.cargo/bin/excel-to-json
   ```

2. Ensure the directory is in your PATH:
   ```bash
   echo $PATH
   ```

3. Reload your shell configuration:
   ```bash
   source ~/.bashrc  # or ~/.zshrc
   ```

### Permission Denied

If you get "permission denied":
```bash
chmod +x excel-to-json
```

### Library Dependencies

On Linux, if you encounter library errors:
```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install libssl-dev

# RHEL/CentOS/Fedora
sudo yum install openssl-devel
```

## Next Steps

Now that you have excel-to-json installed, check out the [Quick Start Guide](/getting-started/quick-start/) to start converting Excel files to JSON!
