# 🚀 Simple Installation Guide for Non-Technical Users

## macOS Users

### Step 1: Install Dependencies
Open Terminal and run these commands one by one:

```bash
# Install Homebrew (if you don't have it)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Press 1 when prompted, then restart Terminal

# Install ExifTool
brew install exiftool
```

### Step 2: Download and Build MetaSort
```bash
# Download MetaSort
git clone https://github.com/iamsanmith/MetaSort.git
cd MetaSort_v1.0.0

# Build and create launchers
./scripts/build_macos.sh
```

### Step 3: Use MetaSort
After building, you'll have two easy ways to run MetaSort:

1. **Double-click `Run_MetaSort.command`** (recommended)
   - Opens MetaSort in a new terminal window
   - Perfect for non-technical users

2. **Double-click `MetaSort.command`**
   - Runs MetaSort in the current terminal
   - For advanced users

## Windows Users

### Step 1: Quick Installation
1. Download MetaSort from GitHub
2. Right-click on `scripts/install_windows.bat` and select "Run as administrator"
3. Follow the prompts

### Step 2: Manual Installation (if quick install fails)
1. Install Rust from https://rustup.rs/
2. Install ExifTool using winget: `winget install ExifTool.ExifTool`
3. Open Command Prompt in the MetaSort folder
4. Run: `cargo build --release`
5. Run: `cargo run --release`

## 🎯 What MetaSort Does

MetaSort helps you organize your Google Photos Takeout (or any messy photo folder) by:

- 🧹 Cleaning up file names
- 📅 Sorting photos by date
- 🏷️ Adding missing metadata
- 📊 Creating beautiful reports
- 📁 Organizing everything into neat folders

## 🆘 Troubleshooting

### "ExifTool not found" Error
- **macOS**: Run `brew install exiftool` in Terminal
- **Windows**: Run `winget install ExifTool.ExifTool` in Command Prompt

### "Rust not found" Error
- **macOS**: Run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Windows**: Download and install from https://rustup.rs/

### App won't open on macOS
- Right-click the app → Open
- Or use the `.command` files instead

## 📞 Need Help?

If you're still having trouble:
1. Check that all dependencies are installed
2. Make sure you're running the commands in the correct directory
3. Try the alternative launcher options
4. Check the main README.md for detailed instructions 