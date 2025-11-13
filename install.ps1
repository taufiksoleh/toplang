# TopLang Installation Script for Windows
# This script automatically downloads and installs the latest toplang binary

$RepoOwner = "taufiksoleh"
$RepoName = "toplang"
$BinaryName = "topc.exe"
$InstallDir = "$env:LOCALAPPDATA\Programs\toplang"

function Write-Info {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Green
}

function Write-Warn {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor Yellow
}

function Write-Error-Custom {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
    exit 1
}

function Get-LatestVersion {
    try {
        $response = Invoke-RestMethod -Uri "https://api.github.com/repos/$RepoOwner/$RepoName/releases/latest"
        return $response.tag_name
    } catch {
        Write-Error-Custom "Failed to fetch the latest version. Please check your internet connection."
    }
}

function Install-TopLang {
    Write-Host ""
    Write-Host "╔════════════════════════════════════════╗"
    Write-Host "║   TopLang Installation Script         ║"
    Write-Host "╔════════════════════════════════════════╗"
    Write-Host ""

    # Detect architecture
    $Arch = "x64"
    if ($env:PROCESSOR_ARCHITECTURE -eq "ARM64") {
        $Arch = "arm64"
    }

    Write-Info "Detected Architecture: $Arch"

    # Get latest version
    Write-Info "Fetching latest release version..."
    $Version = Get-LatestVersion

    if ([string]::IsNullOrEmpty($Version)) {
        Write-Error-Custom "Failed to fetch the latest version."
    }

    Write-Info "Latest version: $Version"

    # Construct download URL
    $AssetName = "topc-windows-$Arch.zip"
    $DownloadUrl = "https://github.com/$RepoOwner/$RepoName/releases/download/$Version/$AssetName"

    Write-Info "Downloading from: $DownloadUrl"

    # Create install directory if it doesn't exist
    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }

    # Download the archive
    $TempDir = [System.IO.Path]::Combine([System.IO.Path]::GetTempPath(), [System.Guid]::NewGuid().ToString())
    New-Item -ItemType Directory -Path $TempDir -Force | Out-Null
    $TempFile = Join-Path $TempDir $AssetName

    try {
        Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempFile
    } catch {
        Remove-Item -Path $TempDir -Recurse -Force -ErrorAction SilentlyContinue
        Write-Error-Custom "Download failed: $_"
    }

    Write-Info "Extracting archive..."

    # Extract the archive
    try {
        Expand-Archive -Path $TempFile -DestinationPath $TempDir -Force
    } catch {
        Remove-Item -Path $TempDir -Recurse -Force -ErrorAction SilentlyContinue
        Write-Error-Custom "Extraction failed: $_"
    }

    # Move the extracted binary to install directory
    $InstallPath = Join-Path $InstallDir $BinaryName
    $ExtractedBinary = Join-Path $TempDir $BinaryName
    Move-Item -Path $ExtractedBinary -Destination $InstallPath -Force

    # Clean up temporary files
    Remove-Item -Path $TempDir -Recurse -Force -ErrorAction SilentlyContinue

    Write-Info "✓ TopLang installed successfully to: $InstallPath"

    # Check if install directory is in PATH
    $PathArray = $env:PATH -split ';'
    if ($PathArray -contains $InstallDir) {
        Write-Info "✓ $InstallDir is in your PATH"
    } else {
        Write-Warn "$InstallDir is not in your PATH"
        Write-Host ""
        Write-Host "To add it to your PATH permanently, run PowerShell as Administrator and execute:"
        Write-Host "    [Environment]::SetEnvironmentVariable('Path', `$env:Path + ';$InstallDir', 'User')"
        Write-Host ""
        Write-Host "Or add it to the current session only:"
        Write-Host "    `$env:Path += ';$InstallDir'"
        Write-Host ""

        # Add to current session
        $env:Path += ";$InstallDir"
        Write-Info "Added to current PowerShell session PATH"
    }

    # Verify installation
    try {
        $VersionOutput = & $InstallPath --version 2>&1
        Write-Info "Installation verified! Run '$BinaryName --help' to get started."
    } catch {
        Write-Warn "Installation completed but verification failed."
    }

    Write-Host ""
    Write-Info "Installation complete!"
    Write-Host ""
}

# Run the installation
Install-TopLang
