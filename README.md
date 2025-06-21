# zfetch

> Fast system information fetcher for Linux written in Rust

A modern, colorful system information tool inspired by pfetch and neofetch, with automatic distribution logo detection and terminal-native colors.

## ✨ Features

- ⚡ **Ultra-fast** and lightweight performance
- 🎨 **Colorful ASCII logos** with automatic distribution detection
- 🌈 **Terminal-native colors** - uses your terminal's color scheme
- 🐧 **50+ Linux distributions** supported with custom logos
- 💻 **Comprehensive system info**: OS, kernel, CPU, GPU, memory, uptime
- 📱 **Multiple output formats**: normal, minimal, JSON
- 🔒 **Memory-safe** Rust implementation
- 🎯 **Smart alignment** - logos and text perfectly aligned
- 🚫 **No external dependencies** for ASCII logos

## 🎯 What's New

### v2.0 Major Updates:
- **🎨 Colorful ASCII Logos**: Each distribution now has its own colorful logo
- **🌈 Terminal Color Integration**: Uses your terminal's native color scheme
- **🐧 Extended Distribution Support**: 50+ distributions including CachyOS, Garuda, EndeavourOS, PopOS, etc.
- **⚖️ Perfect Alignment**: Logos and system information are perfectly aligned
- **🧹 Code Cleanup**: Streamlined codebase, removed unused dependencies
- **🎯 Smart Logo Selection**: Automatic detection from `/etc/os-release`
- **🔄 Color Cycling**: Multi-color logos with intelligent color application

### Supported Distributions:
- **Arch-based**: Arch Linux, Manjaro, EndeavourOS, Artix, Garuda, CachyOS
- **Debian-based**: Debian, Ubuntu, Pop!_OS, Linux Mint, Elementary OS, Kali Linux
- **Red Hat-based**: Fedora, CentOS, AlmaLinux, Rocky Linux
- **Other**: openSUSE, Gentoo, Void Linux, NixOS, Alpine Linux, Solus
- **BSD**: FreeBSD, OpenBSD
- **Mobile**: Android
- **macOS**: Darwin support

## 🚀 Installation

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

## 📖 Usage

```bash
# Full output with colorful logo
zfetch

# Minimal output
zfetch --minimal

# JSON format for scripting
zfetch --json

# Disable colors (for scripts/logs)
zfetch --no-color

# Hide logo (info only)
zfetch --no-logo
```

## ⚙️ Options

| Flag | Short | Description |
|------|-------|-------------|
| `--minimal` | `-m` | Show minimal information only |
| `--no-color` | `-n` | Disable colored output |
| `--no-logo` | `-l` | Hide the ASCII logo |
| `--json` | `-j` | Output in JSON format |
| `--help` | `-h` | Show help message |
| `--version` | `-v` | Show version information |

## 🎨 Color Scheme

zfetch uses your terminal's native color palette:
- **Labels**: Cyan (terminal color 6)
- **Values**: White (terminal color 7)
- **Logos**: Distribution-specific color combinations

## 🔧 Technical Details

- **Language**: Rust 🦀
- **Dependencies**: `sysinfo`, `serde_json`
- **Logo Source**: Based on pfetch and pfetch-rs projects
- **Performance**: Sub-millisecond execution time
- **Memory**: < 5MB RAM usage

## 🤝 Contributing

Contributions are welcome! Please feel free to:
- Add new distribution logos
- Improve system information detection
- Report bugs or suggest features
- Submit pull requests

## 📄 License

GPL-3.0 - see [LICENSE](LICENSE) file for details

## 🙏 Acknowledgments

- [pfetch](https://github.com/dylanaraps/pfetch) - Original ASCII logo inspiration
- [pfetch-rs](https://github.com/Gobidev/pfetch-rs) - Rust implementation reference
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - System information library
