# MetaSort Windows Installation Helper (PowerShell)
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "MetaSort Windows Installation Helper" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check if Rust is installed
Write-Host "Checking if Rust is installed..." -ForegroundColor Yellow
try {
    $rustVersion = rustc --version 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Rust is installed: $rustVersion" -ForegroundColor Green
    } else {
        throw "Rust not found"
    }
} catch {
    Write-Host "❌ Rust is not installed." -ForegroundColor Red
    Write-Host ""
    Write-Host "Please install Rust from: https://rustup.rs/" -ForegroundColor Yellow
    Write-Host "After installation, restart this script." -ForegroundColor Yellow
    Read-Host "Press Enter to exit"
    exit 1
}

Write-Host ""
Write-Host "Checking if ExifTool is installed..." -ForegroundColor Yellow
try {
    $exifVersion = exiftool -ver 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ ExifTool is installed: $exifVersion" -ForegroundColor Green
    } else {
        throw "ExifTool not found"
    }
} catch {
    Write-Host "❌ ExifTool is not installed." -ForegroundColor Red
    Write-Host ""
    Write-Host "Attempting to install ExifTool..." -ForegroundColor Yellow
    
    # Try winget first
    Write-Host "Trying winget installation..." -ForegroundColor Yellow
    try {
        winget install ExifTool.ExifTool
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ ExifTool installed successfully via winget!" -ForegroundColor Green
        } else {
            throw "Winget installation failed"
        }
    } catch {
        Write-Host "Winget installation failed. Trying Chocolatey..." -ForegroundColor Yellow
        try {
            choco install exiftool -y
            if ($LASTEXITCODE -eq 0) {
                Write-Host "✅ ExifTool installed successfully via Chocolatey!" -ForegroundColor Green
            } else {
                throw "Chocolatey installation failed"
            }
        } catch {
            Write-Host "❌ Automatic installation failed." -ForegroundColor Red
            Write-Host ""
            Write-Host "Please install ExifTool manually:" -ForegroundColor Yellow
            Write-Host "1. Download from https://exiftool.org/" -ForegroundColor White
            Write-Host "2. Extract to C:\exiftool" -ForegroundColor White
            Write-Host "3. Add C:\exiftool to your PATH environment variable" -ForegroundColor White
            Write-Host "4. Restart your PowerShell session" -ForegroundColor White
            Read-Host "Press Enter to exit"
            exit 1
        }
    }
    
    Write-Host ""
    Write-Host "⚠️  Please restart your PowerShell session for PATH changes to take effect." -ForegroundColor Yellow
    Write-Host "Then run this script again to continue." -ForegroundColor Yellow
    Read-Host "Press Enter to exit"
    exit 0
}

Write-Host ""
Write-Host "Building MetaSort..." -ForegroundColor Yellow
try {
    cargo build --release
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ MetaSort built successfully!" -ForegroundColor Green
    } else {
        throw "Build failed"
    }
} catch {
    Write-Host "❌ Build failed. Please check the error messages above." -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
}

Write-Host ""
Write-Host "🎉 Installation complete!" -ForegroundColor Green
Write-Host ""
Write-Host "You can now run MetaSort with:" -ForegroundColor Cyan
Write-Host "cargo run --release" -ForegroundColor White
Write-Host ""
Read-Host "Press Enter to exit" 