#!/bin/bash

# zfetch Build Script
# This script builds .deb and .AppImage packages for zfetch

set -e

VERSION=${1:-"0.1.0"}
ARCH="amd64"

echo "Building zfetch v$VERSION for Linux ($ARCH)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if required tools are installed
check_dependencies() {
    print_status "Checking dependencies..."
    
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo is not installed. Please install Rust first."
        exit 1
    fi
    
    if ! command -v dpkg-deb &> /dev/null; then
        print_error "dpkg-deb is not installed. Install with: sudo apt install dpkg-dev"
        exit 1
    fi
    
    print_status "All dependencies are available."
}

# Build the binary
build_binary() {
    print_status "Building release binary..."
    cargo build --release
    
    if [ ! -f "target/release/zfetch" ]; then
        print_error "Binary build failed!"
        exit 1
    fi
    
    # Strip the binary to reduce size
    strip target/release/zfetch
    print_status "Binary built and stripped successfully."
}

# Create .deb package
create_deb_package() {
    print_status "Creating .deb package..."
    
    # Clean up any existing package directory
    rm -rf package
    
    # Create package structure
    mkdir -p package/DEBIAN
    mkdir -p package/usr/bin
    mkdir -p package/usr/share/doc/zfetch
    mkdir -p package/usr/share/man/man1
    
    # Copy binary
    cp target/release/zfetch package/usr/bin/
    chmod +x package/usr/bin/zfetch
    
    # Create control file
    cat > package/DEBIAN/control << EOF
Package: zfetch
Version: $VERSION
Section: utils
Priority: optional
Architecture: $ARCH
Depends: libc6 (>= 2.27)
Maintainer: zfetch contributors <zfetch@example.com>
Description: Yet another system information fetcher for Linux
 zfetch is a fast and lightweight system information fetcher written in Rust.
 It was created as an alternative to neofetch with a focus on performance,
 stability, and simplicity.
 .
 Features include:
  - Fast execution written in Rust
  - Beautiful colored output
  - Comprehensive system information display
  - Minimal mode and JSON output support
  - Memory-safe implementation
Homepage: https://github.com/yourusername/zfetch
EOF

    # Create copyright file
    cat > package/usr/share/doc/zfetch/copyright << EOF
Format: https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/
Upstream-Name: zfetch
Upstream-Contact: zfetch contributors
Source: https://github.com/yourusername/zfetch

Files: *
Copyright: 2025 zfetch contributors
License: MIT

License: MIT
 Permission is hereby granted, free of charge, to any person obtaining a copy
 of this software and associated documentation files (the "Software"), to deal
 in the Software without restriction, including without limitation the rights
 to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 copies of the Software, and to permit persons to whom the Software is
 furnished to do so, subject to the following conditions:
 .
 The above copyright notice and this permission notice shall be included in all
 copies or substantial portions of the Software.
 .
 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 SOFTWARE.
EOF

    # Create changelog
    cat > package/usr/share/doc/zfetch/changelog << EOF
zfetch ($VERSION) stable; urgency=medium

  * Release $VERSION
  * Initial release of zfetch system information fetcher

 -- zfetch contributors <zfetch@example.com>  $(date -R)
EOF
    gzip -9 package/usr/share/doc/zfetch/changelog

    # Create man page
    cat > package/usr/share/man/man1/zfetch.1 << 'EOF'
.TH ZFETCH 1 "January 2025" "zfetch" "User Commands"
.SH NAME
zfetch \- system information fetcher for Linux
.SH SYNOPSIS
.B zfetch
[\fIOPTIONS\fR]
.SH DESCRIPTION
zfetch is a fast and lightweight system information fetcher written in Rust.
It displays various system information including OS, kernel, CPU, GPU, memory usage, and more.
.SH OPTIONS
.TP
\fB\-m\fR, \fB\-\-minimal\fR
Display minimal information
.TP
\fB\-n\fR, \fB\-\-no\-color\fR
Disable colored output
.TP
\fB\-l\fR, \fB\-\-no\-logo\fR
Don't display the logo
.TP
\fB\-j\fR, \fB\-\-json\fR
Output information in JSON format
.TP
\fB\-h\fR, \fB\-\-help\fR
Show help message
.TP
\fB\-v\fR, \fB\-\-version\fR
Show version information
.SH EXAMPLES
.TP
zfetch
Show full system information with logo
.TP
zfetch \-\-minimal
Show minimal system information
.TP
zfetch \-\-json
Output system information in JSON format
.SH AUTHORS
Written by zfetch contributors.
.SH REPORTING BUGS
Report bugs to: https://github.com/yourusername/zfetch/issues
.SH COPYRIGHT
Copyright © 2025 zfetch contributors. License MIT.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
EOF
    gzip -9 package/usr/share/man/man1/zfetch.1

    # Build the .deb package
    dpkg-deb --build package "zfetch_${VERSION}_${ARCH}.deb"
    
    print_status ".deb package created: zfetch_${VERSION}_${ARCH}.deb"
}

# Create AppImage
create_appimage() {
    print_status "Creating AppImage..."
    
    # Download linuxdeploy if not exists
    if [ ! -f "linuxdeploy" ]; then
        print_status "Downloading linuxdeploy..."
        wget -O linuxdeploy https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
        chmod +x linuxdeploy
    fi
    
    # Clean up any existing AppDir
    rm -rf AppDir
    
    # Create AppDir structure
    mkdir -p AppDir/usr/bin
    mkdir -p AppDir/usr/share/applications
    mkdir -p AppDir/usr/share/icons/hicolor/256x256/apps
    
    # Copy binary
    cp target/release/zfetch AppDir/usr/bin/
    
    # Create desktop file
    cat > AppDir/usr/share/applications/zfetch.desktop << EOF
[Desktop Entry]
Name=zfetch
Comment=System information fetcher for Linux
Exec=zfetch
Icon=zfetch
Type=Application
Categories=System;
Terminal=true
EOF

    # Create a simple icon placeholder (you can replace this with a real icon)
    mkdir -p AppDir/usr/share/icons/hicolor/256x256/apps
    # For now, we'll create a simple text file as placeholder
    echo "zfetch icon placeholder" > AppDir/usr/share/icons/hicolor/256x256/apps/zfetch.png
    
    # Create AppImage
    ./linuxdeploy --appdir AppDir --output appimage
    
    # Rename the AppImage
    if ls zfetch*.AppImage 1> /dev/null 2>&1; then
        mv zfetch*.AppImage "zfetch-${VERSION}-x86_64.AppImage"
        print_status "AppImage created: zfetch-${VERSION}-x86_64.AppImage"
    else
        print_warning "AppImage creation may have failed or file not found with expected pattern"
        ls -la *.AppImage 2>/dev/null || print_warning "No AppImage files found"
    fi
}

# Generate checksums
generate_checksums() {
    print_status "Generating checksums..."
    
    rm -f checksums.txt
    
    if [ -f "zfetch_${VERSION}_${ARCH}.deb" ]; then
        sha256sum "zfetch_${VERSION}_${ARCH}.deb" >> checksums.txt
    fi
    
    if [ -f "zfetch-${VERSION}-x86_64.AppImage" ]; then
        sha256sum "zfetch-${VERSION}-x86_64.AppImage" >> checksums.txt
    fi
    
    if [ -f "checksums.txt" ]; then
        print_status "Checksums generated in checksums.txt"
        cat checksums.txt
    fi
}

# Cleanup function
cleanup() {
    print_status "Cleaning up temporary files..."
    rm -rf package AppDir
}

# Main execution
main() {
    echo "======================================"
    echo "      zfetch Build Script v1.0       "
    echo "======================================"
    echo
    
    check_dependencies
    build_binary
    create_deb_package
    
    # Try to create AppImage (may fail if dependencies are missing)
    if command -v wget &> /dev/null; then
        create_appimage
    else
        print_warning "wget not found, skipping AppImage creation"
    fi
    
    generate_checksums
    cleanup
    
    print_status "Build completed successfully!"
    echo
    echo "Generated files:"
    ls -la *.deb *.AppImage checksums.txt 2>/dev/null || true
}

# Run main function
main "$@"
