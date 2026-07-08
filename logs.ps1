# logs.ps1 — Lihat log service
# Contoh: .\logs.ps1           (semua)
#         .\logs.ps1 backend   (backend saja)
param(
    [string]$Service = ""
)

if ($Service) {
    docker-compose logs -f $Service
} else {
    docker-compose logs -f
}
