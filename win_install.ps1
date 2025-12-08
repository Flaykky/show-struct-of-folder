# ============================================
# build-and-install.ps1
# Complete Build and Installation Script for Windows
# ============================================

param(
    [switch]$UserInstall = $false,
    [switch]$SkipBuild = $false,
    [switch]$DevMode = $false
)

# ============================================
# Configuration
# ============================================

$ErrorActionPreference = "Stop"

# Colors
function Write-ColorOutput($ForegroundColor, $message) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    Write-Output $message
    $host.UI.RawUI.ForegroundColor = $fc
}

function Write-Header($message) {
    Write-Output ""
    Write-ColorOutput Cyan "═══════════════════════════════════════"
    Write-ColorOutput Cyan "  $message"
    Write-ColorOutput Cyan "═══════════════════════════════════════"
    Write-Output ""
}

function Write-Success($message) {
    Write-ColorOutput Green "✓ $message"
}

function Write-Info($message) {
    Write-ColorOutput Cyan "→ $message"
}

function Write-Warning($message) {
    Write-ColorOutput Yellow "⚠ $message"
}

function Write-Error($message) {
    Write-ColorOutput Red "✗ $message"
}

# ============================================
# Main Script
# ============================================

Write-Header "SSP - Build & Install Script"

# Check admin rights
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (!$UserInstall -and !$isAdmin) {
    Write-Warning "Not running as Administrator!"
    Write-Info "Will install for current user only."
    Write-Output ""
    Write-Info "For system-wide installation:"
    Write-Output "  1. Run PowerShell as Administrator"
    Write-Output "  2. Execute: .\build-and-install.ps1"
    Write-Output ""
    $response = Read-Host "Continue with user installation? (Y/n)"
    if ($response -eq 'n' -or $response -eq 'N') {
        Write-Info "Installation cancelled"
        exit 0
    }
    $UserInstall = $true
}

# Determine installation directory
if ($UserInstall) {
    $installDir = "$env:USERPROFILE\.local\bin"
    $pathScope = "User"
    Write-Info "Installation type: User"
} else {
    $installDir = "C:\Program Files\ssp"
    $pathScope = "Machine"
    Write-Info "Installation type: System-wide"
}

Write-Info "Install directory: $installDir"
Write-Output ""

# ============================================
# Step 1: Check Prerequisites
# ============================================

Write-Header "Step 1: Checking Prerequisites"

# Check Rust/Cargo
Write-Info "Checking for Rust/Cargo..."
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "Rust/Cargo is not installed!"
    Write-Output ""
    Write-Warning "Please install Rust:"
    Write-Output "  1. Visit: https://rustup.rs/"
    Write-Output "  2. Download and run rustup-init.exe"
    Write-Output "  3. Restart terminal and run this script again"
    exit 1
}
Write-Success "Rust/Cargo found"

# Get versions
$rustVersion = (rustc --version).Split(' ')[1]
$cargoVersion = (cargo --version).Split(' ')[1]
Write-Success "Rust version: $rustVersion"
Write-Success "Cargo version: $cargoVersion"

# Check if Cargo.toml exists
if (!(Test-Path "Cargo.toml")) {
    Write-Error "Cargo.toml not found!"
    Write-Warning "Please run this script from the project root directory"
    exit 1
}
Write-Success "Project files found"

# ============================================
# Step 2: Build Project
# ============================================

if (!$SkipBuild) {
    Write-Header "Step 2: Building Project"
    
    # Clean previous builds (optional)
    if (Test-Path "target") {
        $clean = Read-Host "Clean previous builds? (y/N)"
        if ($clean -eq 'y' -or $clean -eq 'Y') {
            Write-Info "Cleaning..."
            cargo clean
            Write-Success "Cleaned"
        }
    }
    
    # Build
    Write-Info "Building SSP in release mode..."
    Write-Output ""
    
    if ($DevMode) {
        cargo build
        $buildPath = "target\debug\ssp.exe"
    } else {
        cargo build --release
        $buildPath = "target\release\ssp.exe"
    }
    
    if ($LASTEXITCODE -ne 0) {
        Write-Output ""
        Write-Error "Build failed!"
        exit 1
    }
    
    Write-Output ""
    Write-Success "Build completed successfully"
    
    # Check binary size
    $binarySize = (Get-Item $buildPath).Length / 1MB
    Write-Info "Binary size: $([math]::Round($binarySize, 2)) MB"
} else {
    Write-Header "Step 2: Skipping Build"
    Write-Warning "Using existing binary"
    
    if ($DevMode) {
        $buildPath = "target\debug\ssp.exe"
    } else {
        $buildPath = "target\release\ssp.exe"
    }
    
    if (!(Test-Path $buildPath)) {
        Write-Error "Binary not found at: $buildPath"
        Write-Warning "Run without -SkipBuild flag to build first"
        exit 1
    }
}

# ============================================
# Step 3: Install Binary
# ============================================

Write-Header "Step 3: Installing Binary"

# Create installation directory
if (!(Test-Path $installDir)) {
    Write-Info "Creating installation directory..."
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
    Write-Success "Directory created"
} else {
    Write-Info "Installation directory exists"
}

# Backup existing installation
if (Test-Path "$installDir\ssp.exe") {
    Write-Warning "Existing installation found"
    $backup = "$installDir\ssp.exe.backup"
    Copy-Item "$installDir\ssp.exe" -Destination $backup -Force
    Write-Info "Backed up to: ssp.exe.backup"
}

# Copy binary
Write-Info "Installing binary..."
try {
    Copy-Item $buildPath -Destination "$installDir\ssp.exe" -Force
    Write-Success "Binary installed to: $installDir\ssp.exe"
} catch {
    Write-Error "Failed to copy binary: $_"
    exit 1
}

# ============================================
# Step 4: Update PATH
# ============================================

Write-Header "Step 4: Updating PATH"

$currentPath = [Environment]::GetEnvironmentVariable("Path", $pathScope)

if ($currentPath -like "*$installDir*") {
    Write-Success "Directory already in PATH"
} else {
    Write-Info "Adding to PATH..."
    
    try {
        # Remove any trailing semicolons
        $currentPath = $currentPath.TrimEnd(';')
        $newPath = "$currentPath;$installDir"
        
        [Environment]::SetEnvironmentVariable("Path", $newPath, $pathScope)
        Write-Success "Added to PATH ($pathScope)"
        
        # Update current session
        $env:Path = "$installDir;$env:Path"
        Write-Success "Updated current session"
        
    } catch {
        Write-Error "Failed to update PATH: $_"
        Write-Output ""
        Write-Warning "Please add manually:"
        Write-Output "  1. Search 'Environment Variables' in Windows"
        Write-Output "  2. Edit PATH variable"
        Write-Output "  3. Add: $installDir"
    }
}

# ============================================
# Step 5: Verify Installation
# ============================================

Write-Header "Step 5: Verifying Installation"

# Test if command is available
$sspCommand = Get-Command ssp -ErrorAction SilentlyContinue

if ($sspCommand) {
    Write-Success "SSP command is available"
    Write-Info "Location: $($sspCommand.Source)"
    
    # Test run
    Write-Output ""
    Write-Info "Testing SSP..."
    Write-Output ""
    & ssp --help | Select-Object -First 5
    Write-Output ""
    Write-Success "Installation verified successfully!"
} else {
    Write-Warning "SSP command not immediately available"
    Write-Info "This is normal - PATH changes require terminal restart"
}

# ============================================
# Step 6: Create Helper Scripts
# ============================================

Write-Header "Step 6: Creating Helper Scripts"

# Create uninstall script
$uninstallScript = @"
# SSP Uninstall Script
# Generated by build-and-install.ps1

Write-Host "Uninstalling SSP..." -ForegroundColor Cyan
Write-Host ""

if (Test-Path "$installDir\ssp.exe") {
    Remove-Item "$installDir\ssp.exe" -Force
    Write-Host "✓ Removed binary" -ForegroundColor Green
} else {
    Write-Host "⚠ Binary not found" -ForegroundColor Yellow
}

# Remove backup if exists
if (Test-Path "$installDir\ssp.exe.backup") {
    Remove-Item "$installDir\ssp.exe.backup" -Force
    Write-Host "✓ Removed backup" -ForegroundColor Green
}

# Remove directory if empty
if (Test-Path "$installDir") {
    `$items = Get-ChildItem "$installDir" -Force
    if (`$items.Count -eq 0) {
        Remove-Item "$installDir" -Force -Recurse
        Write-Host "✓ Removed directory" -ForegroundColor Green
    }
}

# Remove from PATH
`$currentPath = [Environment]::GetEnvironmentVariable("Path", "$pathScope")
if (`$currentPath -like "*$installDir*") {
    `$pathArray = `$currentPath.Split(';') | Where-Object { `$_ -ne "$installDir" -and `$_ -ne "" }
    `$newPath = `$pathArray -join ';'
    [Environment]::SetEnvironmentVariable("Path", `$newPath, "$pathScope")
    Write-Host "✓ Removed from PATH" -ForegroundColor Green
}

Write-Host ""
Write-Host "╔═══════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║  Uninstallation Complete!             ║" -ForegroundColor Green
Write-Host "╚═══════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""
Write-Host "Please restart your terminal." -ForegroundColor Yellow
"@

Set-Content -Path "uninstall.ps1" -Value $uninstallScript -Encoding UTF8
Write-Success "Created uninstall.ps1"

# Create quick rebuild script
$rebuildScript = @"
# Quick Rebuild Script
Write-Host "Rebuilding SSP..." -ForegroundColor Cyan
cargo build --release
if (`$LASTEXITCODE -eq 0) {
    Write-Host "✓ Build successful" -ForegroundColor Green
    Copy-Item "target\release\ssp.exe" -Destination "$installDir\ssp.exe" -Force
    Write-Host "✓ Updated installation" -ForegroundColor Green
} else {
    Write-Host "✗ Build failed" -ForegroundColor Red
}
"@

Set-Content -Path "rebuild.ps1" -Value $rebuildScript -Encoding UTF8
Write-Success "Created rebuild.ps1"

# ============================================
# Completion
# ============================================

Write-Header "Installation Complete!"

Write-Output "SSP has been successfully installed!"
Write-Output ""
Write-ColorOutput Green "Quick Start:"
Write-Output "  ssp                  - Show current directory structure"
Write-Output "  ssp -l               - Show with line counts"
Write-Output "  ssp -a               - Analyze code"
Write-Output "  ssp -sc -a           - Show code and analysis"
Write-Output "  ssp --help           - Show all options"
Write-Output ""
Write-ColorOutput Cyan "Helper Scripts Created:"
Write-Output "  uninstall.ps1        - Remove SSP from system"
Write-Output "  rebuild.ps1          - Quick rebuild and update"
Write-Output ""
Write-ColorOutput Cyan "Documentation:"
Write-Output "  README.md            - Full documentation"
Write-Output "  https://github.com/Flaykky/show-struct-of-folder"
Write-Output ""

if (!$sspCommand) {
    Write-ColorOutput Yellow "⚠ IMPORTANT: Restart your terminal to use SSP!"
    Write-Output ""
    Write-Output "After restarting, test with: ssp --help"
}

Write-Output ""
Write-ColorOutput Green "╔═══════════════════════════════════════╗"
Write-ColorOutput Green "║  Thank you for installing SSP! 🚀     ║"
Write-ColorOutput Green "╚═══════════════════════════════════════╝"
Write-Output ""
