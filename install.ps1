# Zed Coding Tracker - Windows Installation Script
# Run this script from the project root directory

Write-Host ""
Write-Host "╔════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  Zed Coding Tracker Extension Installer    ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# Check if Rust is installed
Write-Host "[1/5] Checking Rust installation..." -ForegroundColor Yellow
$rustVersion = cargo --version 2>$null
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Rust not found! Please install from https://rustup.rs/" -ForegroundColor Red
    exit 1
}
Write-Host " Rust found: $rustVersion" -ForegroundColor Green

# Check if in correct directory
Write-Host ""
Write-Host "[2/5] Checking project structure..." -ForegroundColor Yellow
if (-not (Test-Path ".\Cargo.toml") -or -not (Test-Path ".\src\lib.rs")) {
    Write-Host "❌ Not in project root directory!" -ForegroundColor Red
    Write-Host "   Please run this script from zed-coding-tracker folder" -ForegroundColor Red
    exit 1
}
Write-Host " Project structure valid" -ForegroundColor Green

# Build extension
Write-Host ""
Write-Host "[3/5] Building extension (this may take a few minutes)..." -ForegroundColor Yellow
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed! Check errors above." -ForegroundColor Red
    Write-Host "   Common issue: Missing Visual Studio Build Tools" -ForegroundColor Yellow
    Write-Host "   Download from: https://visualstudio.microsoft.com/downloads/" -ForegroundColor Yellow
    exit 1
}
Write-Host " Build successful" -ForegroundColor Green

# Check if DLL exists
if (-not (Test-Path ".\target\release\zed_coding_tracker.dll")) {
    Write-Host "❌ DLL not found after build!" -ForegroundColor Red
    exit 1
}

# Create extension directory
Write-Host ""
Write-Host "[4/5] Installing to Zed extensions folder..." -ForegroundColor Yellow
$ExtPath = "$env:APPDATA\Zed\extensions\coding-tracker"

try {
    # Create directory
    New-Item -ItemType Directory -Force -Path $ExtPath | Out-Null
    New-Item -ItemType Directory -Force -Path "$ExtPath\src" | Out-Null
    New-Item -ItemType Directory -Force -Path "$ExtPath\target\release" | Out-Null
    
    # Copy files
    Copy-Item -Path ".\Cargo.toml" -Destination $ExtPath -Force
    Copy-Item -Path ".\extension.toml" -Destination $ExtPath -Force
    Copy-Item -Path ".\src\lib.rs" -Destination "$ExtPath\src\" -Force
    Copy-Item -Path ".\target\release\zed_coding_tracker.dll" -Destination "$ExtPath\target\release\" -Force
    
    Write-Host " Files copied successfully" -ForegroundColor Green
}
catch {
    Write-Host "❌ Failed to copy files: $_" -ForegroundColor Red
    exit 1
}

# Verify installation
Write-Host ""
Write-Host "[5/5] Verifying installation..." -ForegroundColor Yellow
$requiredFiles = @(
    "$ExtPath\extension.toml",
    "$ExtPath\Cargo.toml",
    "$ExtPath\src\lib.rs",
    "$ExtPath\target\release\zed_coding_tracker.dll"
)

$allFilesExist = $true
foreach ($file in $requiredFiles) {
    if (-not (Test-Path $file)) {
        Write-Host "❌ Missing: $file" -ForegroundColor Red
        $allFilesExist = $false
    }
}

if (-not $allFilesExist) {
    Write-Host "❌ Installation incomplete!" -ForegroundColor Red
    exit 1
}

Write-Host " All files verified" -ForegroundColor Green

# Success message
Write-Host ""
Write-Host "╔════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║         Installation Complete!             ║" -ForegroundColor Green
Write-Host "╚════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""
Write-Host " Extension installed to:" -ForegroundColor Cyan
Write-Host "   $ExtPath" -ForegroundColor White
Write-Host ""
Write-Host " Data will be saved to:" -ForegroundColor Cyan
Write-Host "   $env:APPDATA\Zed\coding-tracker-data.txt" -ForegroundColor White
Write-Host ""
Write-Host " Next steps:" -ForegroundColor Yellow
Write-Host "   1. Close Zed completely (check Task Manager)" -ForegroundColor White
Write-Host "   2. Restart Zed" -ForegroundColor White
Write-Host "   3. Press Ctrl+Shift+I to open Developer Console" -ForegroundColor White
Write-Host "   4. Look for [Tracker] messages" -ForegroundColor White
Write-Host ""
Write-Host " Tip: Extension will start tracking automatically!" -ForegroundColor Cyan
Write-Host ""

# Ask if user wants to open extension folder
$response = Read-Host "Open extension folder? (y/n)"
if ($response -eq 'y' -or $response -eq 'Y') {
    explorer $ExtPath
}

Write-Host ""
Write-Host "Happy coding! " -ForegroundColor Green
Write-Host ""