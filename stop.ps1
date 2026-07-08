# stop.ps1 — Matikan semua service
Write-Host "Mematikan semua service..." -ForegroundColor Yellow
docker-compose down
Write-Host "Semua service berhasil dimatikan." -ForegroundColor Green
