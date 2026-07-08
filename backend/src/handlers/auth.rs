use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::State, Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    cache,
    errors::{AppError, AppResult},
    middleware::{
        auth::AuthUser,
        jwt::{generate_access_token, generate_refresh_token, verify_token},
    },
    models::user::{User, UserResponse},
    state::AppState,
};

// ─── Request / Response DTOs ──────────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, message = "Password minimal 8 karakter"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,

    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub success: bool,
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserResponse,
}

// ─── Handler: Register ────────────────────────────────────────────────────────

pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> AppResult<Json<serde_json::Value>> {
    // Validasi input
    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Cek email sudah ada
    let exists: Option<bool> = sqlx::query_scalar(
        "SELECT true FROM users WHERE email = $1"
    )
    .bind(&body.email)
    .fetch_optional(&state.db)
    .await?;

    if exists.is_some() {
        return Err(AppError::Conflict("Email sudah terdaftar".into()));
    }

    // Hash password dengan Argon2
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| AppError::Argon2(e.to_string()))?
        .to_string();

    // Insert user
    let user: User = sqlx::query_as(
        r#"
        INSERT INTO users (id, email, name, password_hash, role, is_active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 'user', true, $5, $5)
        RETURNING *
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(&body.email)
    .bind(&body.name)
    .bind(&password_hash)
    .bind(Utc::now())
    .fetch_one(&state.db)
    .await?;

    tracing::info!("User baru terdaftar: {}", user.email);

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Registrasi berhasil",
        "user": UserResponse::from(user)
    })))
}

// ─── Handler: Login ───────────────────────────────────────────────────────────

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Cari user by email
    let user: Option<User> = sqlx::query_as(
        "SELECT * FROM users WHERE email = $1 AND is_active = true"
    )
    .bind(&body.email)
    .fetch_optional(&state.db)
    .await?;

    let user = user.ok_or_else(|| {
        AppError::Unauthorized("Email atau password salah".into())
    })?;

    // Verifikasi password
    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|e| AppError::Argon2(e.to_string()))?;

    Argon2::default()
        .verify_password(body.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized("Email atau password salah".into()))?;

    // Generate tokens
    let (access_token, _access_jti) = generate_access_token(
        user.id,
        &user.email,
        &user.role,
        &state.config.jwt_access_secret,
        state.config.jwt_access_expires_in,
    )?;

    let (refresh_token, refresh_jti) = generate_refresh_token(
        user.id,
        &user.email,
        &user.role,
        &state.config.jwt_refresh_secret,
        state.config.jwt_refresh_expires_in,
    )?;

    // Simpan refresh token JTI di Redis
    let redis_key = cache::refresh_token_key(&user.id.to_string());
    cache::set_ex(
        &state.redis,
        &redis_key,
        &refresh_jti,
        state.config.jwt_refresh_expires_in,
    )
    .await?;

    tracing::info!("User login: {}", user.email);

    Ok(Json(AuthResponse {
        success: true,
        access_token,
        refresh_token,
        user: UserResponse::from(user),
    }))
}

// ─── Handler: Refresh Token ───────────────────────────────────────────────────

pub async fn refresh(
    State(state): State<AppState>,
    Json(body): Json<RefreshRequest>,
) -> AppResult<Json<serde_json::Value>> {
    // Verify refresh token
    let claims = verify_token(&body.refresh_token, &state.config.jwt_refresh_secret)
        .map_err(|_| AppError::Unauthorized("Refresh token tidak valid atau expired".into()))?;

    if claims.token_type != "refresh" {
        return Err(AppError::Unauthorized("Bukan refresh token".into()));
    }

    // Cek JTI di Redis (pastikan masih valid / belum di-revoke)
    let redis_key = cache::refresh_token_key(&claims.sub);
    let stored_jti = cache::get(&state.redis, &redis_key).await?;

    match stored_jti {
        Some(jti) if jti == claims.jti => {} // valid
        _ => return Err(AppError::Unauthorized("Refresh token sudah tidak valid".into())),
    }

    // Generate access token baru
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Internal("ID tidak valid".into()))?;

    let (new_access_token, _) = generate_access_token(
        user_id,
        &claims.email,
        &claims.role,
        &state.config.jwt_access_secret,
        state.config.jwt_access_expires_in,
    )?;

    Ok(Json(serde_json::json!({
        "success": true,
        "access_token": new_access_token
    })))
}

// ─── Handler: Logout ──────────────────────────────────────────────────────────

pub async fn logout(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
) -> AppResult<Json<serde_json::Value>> {
    // Blacklist access token di Redis (sampai expired)
    let remaining_ttl = claims.exp - Utc::now().timestamp();
    if remaining_ttl > 0 {
        let key = cache::blacklist_key(&claims.jti);
        cache::set_ex(&state.redis, &key, "1", remaining_ttl).await?;
    }

    // Hapus refresh token dari Redis
    let redis_key = cache::refresh_token_key(&claims.sub);
    cache::del(&state.redis, &redis_key).await?;

    tracing::info!("User logout: {}", claims.email);

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Logout berhasil"
    })))
}
