param(
    [string]$Address = "0.0.0.0",
    [int]$Port = 8080
)

$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
Set-Location $repoRoot

function Stop-ProcessById {
    param(
        [int]$ProcessId,
        [string]$Reason
    )

    if ($ProcessId -le 0 -or $ProcessId -eq $PID) {
        return
    }

    $process = Get-CimInstance Win32_Process -Filter "ProcessId = $ProcessId" -ErrorAction SilentlyContinue
    if (-not $process) {
        return
    }

    Write-Host "Stopping $($process.Name) process $ProcessId ($Reason)."
    Stop-Process -Id $ProcessId -Force -ErrorAction SilentlyContinue
    Wait-Process -Id $ProcessId -Timeout 5 -ErrorAction SilentlyContinue
}

Get-CimInstance Win32_Process -ErrorAction SilentlyContinue |
    Where-Object { $_.Name -ieq "dx.exe" -and $_.CommandLine -match "\bserve\b" } |
    ForEach-Object {
        Stop-ProcessById -ProcessId $_.ProcessId -Reason "existing Dioxus server"
    }

Get-NetTCPConnection -LocalPort $Port -State Listen -ErrorAction SilentlyContinue |
    Select-Object -ExpandProperty OwningProcess -Unique |
    ForEach-Object {
        Stop-ProcessById -ProcessId $_ -Reason "port $Port listener"
    }

$wifiAddress = Get-NetIPConfiguration |
    Where-Object { $_.IPv4DefaultGateway -and $_.NetAdapter.Status -eq "Up" } |
    Select-Object -ExpandProperty IPv4Address -First 1 |
    Select-Object -ExpandProperty IPAddress

Write-Host "Starting web app for laptop and phone testing on the same Wi-Fi."
Write-Host "Laptop: http://localhost:$Port"
if ($wifiAddress) {
    Write-Host "Phone:  http://$wifiAddress`:$Port"
} else {
    Write-Host "Phone:  use this laptop's Wi-Fi IPv4 address with port $Port."
}
Write-Host ""

if (-not (Get-Command npm -ErrorAction SilentlyContinue)) {
    throw "npm is required for Tailwind CSS. Run Scripts\Common\InstallDependencies.ps1 first."
}

Write-Host "Building Tailwind CSS..."
npm run tailwind:build

dx serve --platform web --addr $Address --port $Port
