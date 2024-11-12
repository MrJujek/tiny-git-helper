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

info "This is an info message"
info_bold "This is an info message in bold"
success "This is a success message"
error "This is an error message"
