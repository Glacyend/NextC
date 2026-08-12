Write-Host "----- Building English Documentation..."
mdbook build docs -d .\pages\docs\en

Write-Host "----- Building Chinese Documentation..."
mdbook build docs-zh -d .\pages\docs\zh

Write-Host "----- Documentation Build Complete!"
