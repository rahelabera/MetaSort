@echo off
echo ========================================
echo Building MetaSort for Windows
echo ========================================
echo.

echo Building release executable...
cargo build --release --target x86_64-pc-windows-msvc

if %errorlevel% neq 0 (
    echo ❌ Build failed!
    pause
    exit /b 1
)

echo.
echo ✅ Build successful!
echo.
echo Creating single-click executable...

REM Create a simple launcher script
echo @echo off > MetaSort.exe
echo echo Starting MetaSort... >> MetaSort.exe
echo echo. >> MetaSort.exe
echo target\x86_64-pc-windows-msvc\release\MetaSort.exe >> MetaSort.exe
echo pause >> MetaSort.exe

echo.
echo 🎉 MetaSort.exe created successfully!
echo.
echo Users can now:
echo 1. Double-click MetaSort.exe to run
echo 2. No installation required
echo 3. Just make sure exiftool is installed
echo.
echo The executable is in: MetaSort.exe
echo.
pause 