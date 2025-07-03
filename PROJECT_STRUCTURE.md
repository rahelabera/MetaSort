# 📁 MetaSort Project Structure

## 🎯 Quick Start Files (Root Directory)
- **`Run_MetaSort.command`** - Double-click to open MetaSort in new terminal (recommended for non-technical users)
- **`MetaSort.command`** - Double-click to run MetaSort in current terminal
- **`README.md`** - Main documentation and installation guide

## 📂 Organized Directories

### `/scripts/` - Build and Installation Scripts
- **`build_macos.sh`** - Builds MetaSort and creates macOS launchers
- **`build_windows.bat`** - Builds MetaSort for Windows
- **`build_all.sh`** - Universal build script (detects platform)
- **`install_windows.ps1`** - PowerShell installation helper for Windows
- **`install_windows.bat`** - Batch installation helper for Windows

### `/docs/` - Documentation
- **`SIMPLE_INSTALL.md`** - Step-by-step guide for non-technical users
- **`CROSS_PLATFORM_CHANGES.md`** - Technical details of cross-platform implementation

### `/src/` - Source Code
- **`main.rs`** - Main application entry point
- **`platform.rs`** - Cross-platform compatibility layer
- **`ui.rs`** - User interface and progress bars
- **`media_cleaning.rs`** - File cleaning and organization
- **`metadata_extraction.rs`** - Metadata extraction from JSON
- **`metadata_embed.rs`** - Embedding metadata into files
- **`sort_to_folders.rs`** - File sorting and folder creation
- **`csv_report.rs`** - CSV report generation
- **`html_report.rs`** - HTML report generation
- **`filename_date_guess.rs`** - Date extraction from filenames
- **`utils.rs`** - Utility functions

### `/assets/` - Resources
- **`logo.png`** - MetaSort logo
- **`upi.png`** - UPI QR code for donations

### `/target/` - Build Output
- Compiled executables and build artifacts

## 🚀 How to Use

### For Non-Technical Users:
1. **Double-click `Run_MetaSort.command`** - Opens MetaSort in a new terminal window
2. Follow the prompts to organize your photos

### For Developers:
1. **Build**: `./scripts/build_macos.sh` (macOS) or `./scripts/build_windows.bat` (Windows)
2. **Run**: `cargo run --release`
3. **Install dependencies**: See `docs/SIMPLE_INSTALL.md`

## 🎯 Key Features
- ✅ **Cross-platform** - Works on macOS and Windows
- ✅ **User-friendly** - Multiple launcher options
- ✅ **Clean organization** - Well-structured codebase
- ✅ **Easy installation** - Automated scripts for both platforms
- ✅ **Comprehensive docs** - Separate guides for different user types 