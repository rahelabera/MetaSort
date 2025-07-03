#!/bin/bash

# Change to the directory where this script is located
cd "$(dirname "$0")"

# Check if exiftool is installed
if ! command -v exiftool &> /dev/null; then
    echo "❌ ExifTool is not installed!"
    echo "Please install it first:"
    echo "brew install exiftool"
    echo ""
    echo "Press Enter to exit..."
    read
    exit 1
fi

# Run MetaSort
echo "🚀 Starting MetaSort..."
echo ""

# Run the executable directly
./target/release/MetaSort

# Keep terminal open if there's an error
if [ $? -ne 0 ]; then
    echo ""
    echo "Press Enter to exit..."
    read
fi
