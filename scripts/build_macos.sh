#!/bin/bash

echo "========================================"
echo "Building MetaSort for macOS"
echo "========================================"
echo

echo "Building release executable..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo
echo "✅ Build successful!"
echo
echo "Creating macOS launchers..."

# Create a simple command file that runs directly
cat > MetaSort.command << 'EOF'
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
EOF

# Create a launcher that opens in new terminal window
cat > Run_MetaSort.command << 'EOF'
#!/bin/bash

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Open a new terminal window and run MetaSort
osascript << EOSCRIPT
tell application "Terminal"
    do script "cd '$SCRIPT_DIR' && ./target/release/MetaSort"
    activate
end tell
EOSCRIPT
EOF

# Make both executable
chmod +x MetaSort.command
chmod +x Run_MetaSort.command

echo
echo "🎉 MetaSort launchers created successfully!"
echo
echo "For non-technical users:"
echo "1. Double-click 'Run_MetaSort.command' (opens in new terminal)"
echo "2. Or double-click 'MetaSort.command' (runs in current terminal)"
echo
echo "Both options will:"
echo "✅ Check if ExifTool is installed"
echo "✅ Start MetaSort automatically"
echo "✅ Show clear error messages if something is wrong"
echo "✅ Keep the window open if there are errors"
echo
echo "The launchers are:"
echo "- Run_MetaSort.command (recommended for non-technical users)"
echo "- MetaSort.command (for advanced users)"
echo 