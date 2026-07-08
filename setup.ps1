# ============================================================
#  setup.ps1 - One-click setup MyApp di Windows Docker Desktop
#  Jalankan: .\setup.ps1
# ============================================================

$ErrorActionPreference = "Stop"

function Info  { param($msg) Write-Host "[INFO]  $msg" -ForegroundColor Cyan }
function Ok    { param($msg) Write-Host "[OK]    $msg" -ForegroundColor Green }
function Warn  { param($msg) Write-Host "[WARN]  $msg" -ForegroundColor Yellow }
function Fail  { param($msg) Write-Host "[ERROR] $msg" -ForegroundColor Red; exit 1 }

Write-Host ""
Write-Host "=========================================" -ForegroundColor Magenta
Write-Host "   MyApp - Docker Setup (Windows)        " -ForegroundColor Magenta
Write-Host "=========================================" -ForegroundColor Magenta
Write-Host ""

# 1. Cek Docker Desktop
Info "Mengecek Docker Desktop..."
try {
    docker info 2>&1 | Out-Null
    Ok "Docker Desktop berjalan"
} catch {
    Fail "Docker Desktop tidak berjalan! Buka Docker Desktop dulu lalu jalankan ulang script ini."
}

# 2. Cek struktur folder
Info "Mengecek struktur folder..."
if (-not (Test-Path "./backend")) { Fail "Folder 'backend' tidak ditemukan!" }
if (-not (Test-Path "./frontend")) { Fail "Folder 'frontend' tidak ditemukan!" }
if (-not (Test-Path "./backend/Dockerfile")) { Fail "backend/Dockerfile tidak ditemukan!" }
if (-not (Test-Path "./frontend/Dockerfile")) { Fail "frontend/Dockerfile tidak ditemukan!" }
Ok "Struktur folder OK"

# 3. Buat .env kalau belum ada
Info "Mengecek file .env..."
if (-not (Test-Path "./.env")) {
    if (Test-Path "./.env.example") {
        Copy-Item ".env.example" ".env"
        Warn ".env dibuat dari .env.example - silakan edit JWT secret sebelum production!"
    } else {
        $envContent = "DB_USER=postgres`nDB_PASSWORD=password123`nDB_NAME=myapp_db`nJWT_ACCESS_SECRET=dev_access_secret_ganti_di_production`nJWT_REFRESH_SECRET=dev_refresh_secret_ganti_di_production"
        Set-Content -Path ".env" -Value $envContent -Encoding utf8
        Warn ".env dibuat otomatis dengan nilai default (development only)"
    }
} else {
    Ok ".env sudah ada"
}

# 4. Cek next.config.js punya output standalone
Info "Mengecek next.config.js..."
$nextConfigPath = "./frontend/next.config.js"
if (Test-Path $nextConfigPath) {
    $nextConfig = Get-Content $nextConfigPath -Raw
    if ($nextConfig -notmatch "standalone") {
        Warn "Menambahkan output standalone ke next.config.js..."
        $newConfig = "/** @type {import('next').NextConfig} */`nconst nextConfig = {`n  output: 'standalone',`n  async rewrites() {`n    return [`n      {`n        source: '/api/v1/:path*',`n        destination: `"`${process.env.NEXT_PUBLIC_API_URL}/api/v1/:path*`",`n      },`n    ]`n  },`n}`n`nmodule.exports = nextConfig`n"
        Set-Content -Path $nextConfigPath -Value $newConfig -Encoding utf8
        Ok "next.config.js diupdate"
    } else {
        Ok "next.config.js sudah OK"
    }
} else {
    Warn "next.config.js tidak ditemukan, dilewati"
}

# 5. Stop container lama
Info "Membersihkan container lama..."
docker-compose down --remove-orphans 2>&1 | Out-Null
Ok "Container lama dihapus"

# 6. Build semua image
Write-Host ""
Info "Build Docker images (pertama kali butuh beberapa menit)..."
Write-Host "  Backend Rust  : ~3-5 menit" -ForegroundColor Gray
Write-Host "  Frontend Next : ~2-3 menit" -ForegroundColor Gray
Write-Host ""

docker-compose build --no-cache
if ($LASTEXITCODE -ne 0) { Fail "Build gagal! Cek error di atas." }
Ok "Semua image berhasil dibuild"

# 7. Jalankan semua service
Write-Host ""
Info "Menjalankan semua service..."
docker-compose up -d
if ($LASTEXITCODE -ne 0) { Fail "Gagal menjalankan container!" }

# 8. Tunggu backend siap
Info "Menunggu backend siap (max 60 detik)..."
$maxWait = 60
$waited  = 0
$ready   = $false
do {
    Start-Sleep -Seconds 3
    $waited += 3
    try {
        $resp = Invoke-WebRequest -Uri "http://localhost:8080/health" -UseBasicParsing -TimeoutSec 2 -ErrorAction SilentlyContinue
        if ($resp.StatusCode -eq 200) { $ready = $true; break }
    } catch {}
    Write-Host "  Menunggu... ($waited/$maxWait detik)" -ForegroundColor Gray
} while ($waited -lt $maxWait)

if (-not $ready) {
    Warn "Backend belum merespons. Cek log: docker logs myapp_backend"
} else {
    Ok "Backend siap!"
}

# 9. Tampilkan status
Write-Host ""
Write-Host "=========================================" -ForegroundColor Green
Write-Host "   SEMUA SERVICE BERJALAN!               " -ForegroundColor Green
Write-Host "=========================================" -ForegroundColor Green
Write-Host ""
Write-Host "  Frontend -> http://localhost:3000" -ForegroundColor White
Write-Host "  Backend  -> http://localhost:8080" -ForegroundColor White
Write-Host "  Health   -> http://localhost:8080/health" -ForegroundColor White
Write-Host ""
Write-Host "  docker-compose logs -f backend    lihat log backend" -ForegroundColor Gray
Write-Host "  docker-compose logs -f frontend   lihat log frontend" -ForegroundColor Gray
Write-Host "  docker-compose down               matikan semua" -ForegroundColor Gray
Write-Host ""

Start-Process "http://localhost:3000"
