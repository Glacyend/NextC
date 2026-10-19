#Requires -Version 5.1
$ErrorActionPreference = 'Stop'

$PAGES_DIR = Join-Path $PSScriptRoot 'pages'
$SITE_DIR  = Join-Path $PSScriptRoot 'site'

function Invoke-NativeCommand {
    param(
        [Parameter(Mandatory)][string]$Command,
        [Parameter(Mandatory)][string[]]$Arguments
    )
    & $Command @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "Command '$Command $($Arguments -join ' ')' failed with exit code $LASTEXITCODE"
    }
}

Write-Host '----- Preparing pages directory...'
if (Test-Path -LiteralPath $PAGES_DIR) {
    Remove-Item -LiteralPath $PAGES_DIR -Recurse -Force
}
New-Item -ItemType Directory -Path $PAGES_DIR | Out-Null

Write-Host '----- Copying static site files...'
if (-not (Test-Path -LiteralPath $SITE_DIR)) {
    throw "Site directory not found: $SITE_DIR"
}
# -Force 让 Get-ChildItem 包含隐藏文件，对齐 bash 的 cp -r "$SITE_DIR/."
Get-ChildItem -LiteralPath $SITE_DIR -Force |
    Copy-Item -Destination $PAGES_DIR -Recurse -Force

Write-Host '----- Building English Documentation...'
Invoke-NativeCommand -Command 'mdbook' -Arguments @(
    'build', 'docs',
    '-d', (Join-Path $PAGES_DIR 'docs/en')
)

Write-Host '----- Building Chinese Documentation...'
Invoke-NativeCommand -Command 'mdbook' -Arguments @(
    'build', 'docs-zh',
    '-d', (Join-Path $PAGES_DIR 'docs/zh')
)

Write-Host '----- Documentation Build Complete!'
