cd /d "%~dp0\.."
@echo off
echo ========================================
echo MetaSort Windows Installation Helper
echo ========================================
echo.

echo Checking if Rust is installed...
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Rust is not installed.
    echo.
    echo Please install Rust from: https://rustup.rs/
    echo After installation, restart this script.
    pause
    exit /b 1
) else (
    echo ✅ Rust is installed.
)

echo.
echo Checking if exiftool is installed...
exiftool -ver >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ ExifTool is not installed.
    echo.
    echo Installing ExifTool using winget...
    winget install ExifTool.ExifTool
    if %errorlevel% neq 0 (
        echo.
        echo Winget installation failed. Trying Chocolatey...
        choco install exiftool
        if %errorlevel% neq 0 (
            echo.
            echo ❌ Automatic installation failed.
            echo.
            echo Please install ExifTool manually:
            echo 1. Download from https://exiftool.org/
            echo 2. Extract to C:\exiftool
            echo 3. Add C:\exiftool to your PATH environment variable
            echo 4. Restart your command prompt
            pause
            exit /b 1
        )
    )
    echo.
    echo ✅ ExifTool installed successfully!
    echo Please restart your command prompt for PATH changes to take effect.
) else (
    echo ✅ ExifTool is installed.
)

echo.
echo Building MetaSort...
cargo build --release
if %errorlevel% neq 0 (
    echo ❌ Build failed. Please check the error messages above.
    pause
    exit /b 1
)

echo.
echo ✅ MetaSort built successfully!
echo.
echo You can now run MetaSort with:
echo cargo run --release
echo.
pause 
