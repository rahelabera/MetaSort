#!/bin/bash

echo "========================================"
echo "Building MetaSort for All Platforms"
echo "========================================"
echo

# Detect platform
if [[ "$OSTYPE" == "darwin"* ]]; then
    PLATFORM="macos"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    PLATFORM="windows"
else
    PLATFORM="linux"
fi

echo "Detected platform: $PLATFORM"
echo

# Build for current platform
echo "Building for $PLATFORM..."
if [[ "$PLATFORM" == "macos" ]]; then
    ./build_macos.sh
elif [[ "$PLATFORM" == "windows" ]]; then
    ./build_windows.bat
else
    echo "Building Linux executable..."
    cargo build --release
    echo "✅ Linux executable created: target/release/MetaSort"
fi

echo
echo "========================================"
echo "Build Complete!"
echo "========================================"
echo
echo "For distribution:"
echo "1. Copy the executable to users"
echo "2. Ensure exiftool is installed on their system"
echo "3. Users can double-click to run"
echo 