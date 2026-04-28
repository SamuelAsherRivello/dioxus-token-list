$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
Set-Location $repoRoot

if (-not (Get-Command npm -ErrorAction SilentlyContinue)) {
    throw "npm is required to build Tailwind CSS. Install Node.js and rerun this script."
}

if (-not (Test-Path (Join-Path $repoRoot "node_modules\.bin\tailwindcss.cmd"))) {
    Write-Host "Installing Tailwind CSS dependencies..."
    npm install
}

Write-Host "Building Tailwind CSS..."
npm run tailwind:build
