# restart.ps1 — Restart service tertentu
# Contoh: .\restart.ps1 backend
param(
    [string]$Service = "backend"
)

Write-Host "Restart $Service..." -ForegroundColor Yellow
docker-compose restart $Service
Write-Host "$Service berhasil direstart." -ForegroundColor Green
docker-compose logs --tail=20 $Service
