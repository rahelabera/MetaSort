# Cross-Platform Changes for MetaSort v1.0.0

## Overview
MetaSort has been updated to support both macOS and Windows platforms, with Linux support planned for future versions.

## Changes Made

### 1. New Platform Module (`src/platform.rs`)
- **Cross-platform exiftool command detection**: Uses `exiftool.exe` on Windows, `exiftool` on other platforms
- **ExifTool availability checking**: Verifies if exiftool is installed and accessible
- **Platform-specific installation instructions**: Provides tailored setup instructions for each OS
- **Utility functions**: Path handling and platform detection helpers

### 2. Updated Main Application (`src/main.rs`)
- **ExifTool validation**: Checks for exiftool availability at startup
- **User-friendly error messages**: Provides clear installation instructions if exiftool is missing
- **Cross-platform branding**: Updated ASCII art to mention cross-platform support

### 3. Updated Core Modules
- **`src/metadata_embed.rs`**: Now uses platform-specific exiftool command
- **`src/sort_to_folders.rs`**: Updated to use platform-specific exiftool command
- **Removed unused imports**: Cleaned up compilation warnings

### 4. Windows Installation Scripts
- **`install_windows.ps1`**: PowerShell script with colored output and comprehensive error handling
- **`install_windows.bat`**: Batch script for basic installation
- **Automatic dependency management**: Attempts to install exiftool via winget or Chocolatey
- **User guidance**: Clear instructions for manual installation if automatic methods fail

### 5. Updated Documentation (`README.md`)
- **Windows installation section**: Comprehensive setup instructions
- **Multiple installation methods**: winget, Chocolatey, and manual installation options
- **Quick start scripts**: Documentation for the new installation helpers
- **Cross-platform branding**: Updated descriptions to mention Windows support

### 6. Updated Package Configuration (`Cargo.toml`)
- **Updated description**: Now mentions cross-platform support
- **Added comments**: Notes about cross-platform compatibility

## Platform-Specific Features

### Windows
- **ExifTool command**: `exiftool.exe`
- **Installation methods**: winget, Chocolatey, manual PATH setup
- **Path handling**: Backslash separators
- **Installation scripts**: PowerShell and batch file helpers

### macOS
- **ExifTool command**: `exiftool`
- **Installation method**: Homebrew (`brew install exiftool`)
- **Path handling**: Forward slash separators

### Linux (Future)
- **ExifTool command**: `exiftool`
- **Installation methods**: apt, yum, dnf package managers
- **Path handling**: Forward slash separators

## User Experience Improvements

### Before (macOS only)
- Users had to manually install exiftool via Homebrew
- No validation of exiftool availability
- Platform-specific hardcoded commands
- Limited installation guidance

### After (Cross-platform)
- **Automatic validation**: ExifTool availability checked at startup
- **Clear error messages**: Platform-specific installation instructions
- **Installation helpers**: Automated setup scripts for Windows
- **Multiple installation options**: Various methods to install dependencies
- **Better user guidance**: Step-by-step instructions for each platform

## Testing Recommendations

### Windows Testing
1. Test on Windows 10/11 with PowerShell
2. Verify winget installation method
3. Verify Chocolatey installation method
4. Test manual PATH setup
5. Verify exiftool.exe command detection

### macOS Testing
1. Verify existing Homebrew installation still works
2. Test exiftool availability checking
3. Ensure no regression in functionality

### Cross-Platform Testing
1. Verify path handling differences
2. Test file operations on different platforms
3. Ensure consistent behavior across platforms

## Future Enhancements

### Planned Features
- **Linux support**: Full Linux compatibility
- **GUI interface**: Cross-platform GUI using Tauri or similar
- **Docker support**: Containerized version for consistent environments
- **CI/CD**: Automated testing across all platforms

### Potential Improvements
- **Native exiftool integration**: Embed exiftool binary or use Rust libraries
- **Platform-specific optimizations**: Tailored performance improvements
- **Advanced installation**: More sophisticated dependency management

## Migration Notes

### For Existing Users
- **No breaking changes**: All existing functionality preserved
- **Enhanced experience**: Better error messages and validation
- **Optional features**: New installation scripts are optional

### For Developers
- **New module**: `src/platform.rs` contains cross-platform utilities
- **Updated imports**: Some modules now use platform-specific functions
- **Testing**: Consider testing on multiple platforms

## Conclusion

MetaSort is now a truly cross-platform application that provides a consistent experience across macOS and Windows. The changes maintain backward compatibility while adding robust platform detection, improved error handling, and user-friendly installation processes.

The addition of Windows support significantly expands MetaSort's user base and makes it accessible to a wider audience of users who need to organize their Google Photos takeout data. 