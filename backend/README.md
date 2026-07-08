# Backend — Axum + JWT + PostgreSQL + Redis

## Tech Stack
- **Framework**: Axum 0.7 (async Rust)
- **Database**: PostgreSQL 16 + SQLx
- **Cache**: Redis 7
- **Auth**: JWT (access + refresh token)
- **Password**: Argon2

## Endpoints

| Method | Path | Auth | Keterangan |
|--------|------|------|-----------|
| GET | `/health` | - | Health check |
| POST | `/api/v1/auth/register` | - | Registrasi |
| POST | `/api/v1/auth/login` | - | Login |
| POST | `/api/v1/auth/refresh` | - | Refresh access token |
| POST | `/api/v1/auth/logout` | Bearer | Logout |
| GET | `/api/v1/users/me` | Bearer | Profil sendiri |

## Setup Development

### 1. Jalankan PostgreSQL + Redis via Docker
```bash
docker-compose up -d
```

### 2. Copy dan isi .env
```bash
cp .env.example .env
# Edit sesuai kebutuhan
```

### 3. Install sqlx-cli (untuk migrations manual)
```bash
cargo install sqlx-cli --no-default-features --features postgres
```

### 4. Jalankan server
```bash
cargo run
```

Server akan otomatis menjalankan migrations saat start.

## JWT Flow

```
Login → access_token (15 menit) + refresh_token (7 hari)
Request → Bearer {access_token} di header Authorization
Expired → POST /auth/refresh dengan { refresh_token }
Logout → Blacklist access_token + hapus refresh_token di Redis
```

## Build Production

```bash
# Build binary
cargo build --release

# Atau Docker
docker build -t backend:latest .
docker run -p 8080:8080 --env-file .env backend:latest
```

## Contoh Request (curl)

### Register
```bash
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"name":"Bagas","email":"bagas@example.com","password":"password123"}'
```

### Login
```bash
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"bagas@example.com","password":"password123"}'
```

### Get Profile
```bash
curl http://localhost:8080/api/v1/users/me \
  -H "Authorization: Bearer {access_token}"
```
