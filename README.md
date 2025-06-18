# zfetch

> Fast system information fetcher for Linux written in Rust

## Features

- ⚡ Fast and lightweight
- 🎨 Colorful output with customizable logo
- � Shows system info: OS, kernel, CPU, GPU, memory
- 📱 Multiple output formats (normal, minimal, JSON)
- 🔒 Memory-safe Rust implementation

## Installation

### Pre-built Packages

**Debian/Ubuntu (.deb)**
```bash
wget https://github.com/zcurq/zfetch/releases/latest/download/zfetch_VERSION_amd64.deb
sudo dpkg -i zfetch_VERSION_amd64.deb
```

**Universal Linux (AppImage)**
```bash
wget https://github.com/zcurq/zfetch/releases/latest/download/zfetch-VERSION-x86_64.AppImage
chmod +x zfetch-VERSION-x86_64.AppImage
./zfetch-VERSION-x86_64.AppImage
```

### From Source

```bash
git clone https://github.com/zcurq/zfetch.git
cd zfetch
cargo build --release
sudo cp target/release/zfetch /usr/local/bin/
```

## Usage

```bash
# Full output with logo
zfetch

# Minimal output
zfetch --minimal

# JSON format
zfetch --json

# No colors
zfetch --no-color

# Hide logo
zfetch --no-logo
```

## Options

| Flag | Description |
|------|-------------|
| `-m, --minimal` | Minimal information display |
| `-n, --no-color` | Disable colored output |
| `-l, --no-logo` | Hide the logo |
| `-j, --json` | JSON format output |
| `-h, --help` | Show help |
| `-v, --version` | Show version |

## License

GPL-3.0
