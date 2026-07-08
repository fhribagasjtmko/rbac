## Struktur folder yang dibutuhkan

```
my-project/          
├── backend/         
├── frontend/        
├── docker-compose.yml
├── .env.example
├── setup.ps1        
├── stop.ps1
├── restart.ps1
└── logs.ps1
```
## running
# Start semua
docker-compose up -d

# Matikan semua
.\stop.ps1

# Lihat log backend
.\logs.ps1 backend

# Lihat log frontend
.\logs.ps1 frontend

# Restart backend
.\restart.ps1 backend

# Rebuild ulang
docker-compose up -d --build backend
docker-compose up -d --build frontend
```

## URL

| Service   | URL                        |
|-----------|----------------------------|
| Frontend  | http://localhost:3000      |
| Backend   | http://localhost:8080      |
| Health    | http://localhost:8080/health |
| DB        | localhost:5432             |

---
