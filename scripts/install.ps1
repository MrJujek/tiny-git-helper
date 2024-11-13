#!/usr/bin/env pwsh

function error {
    param (
        [string]$message
    )
    Write-Host $message -ForegroundColor Red
    exit 1
}

function info {
    param (
        [string]$message,
        [string]$color = "Gray"
    )
    Write-Host $message -ForegroundColor $color
}

function info_bold {
    param (
        [string]$message,
        [string]$color = "White"
    )
    Write-Host $message -ForegroundColor $color
}

function success {
    param (
        [string]$message,
        [string]$color = "Green"
    )
    Write-Host $message -ForegroundColor $color
}

if (-not ((Get-CimInstance Win32_ComputerSystem)).SystemType -match "x64-based") {
    error "Tiny-git-helper is currently not available on 32-bit or ARM system."
}

$MinBuild = 17763;
$MinBuildName = "Windows 10 1809"
$WinVer = [System.Environment]::OSVersion.Version
if ($WinVer.Major -lt 10 -or ($WinVer.Major -eq 10 -and $WinVer.Build -lt $MinBuild)) {
  error "Tiny-git-helper requires version ${MinBuildName} or newer."
}

info "This is an info message"
info_bold "This is an info message in bold"
success "This is a success message"
error "This is an error message"
