use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, header},
};

use crate::{
    cache,
    errors::{AppError, AppResult},
    middleware::jwt::{verify_token, Claims},
    state::AppState,
};

// ─── Extractor: AuthUser ──────────────────────────────────────────────────────
// Gunakan ini di handler yang butuh auth:
//   async fn handler(auth: AuthUser, ...) -> ...

#[derive(Debug, Clone)]
pub struct AuthUser(pub Claims);

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> AppResult<Self> {
        // 1. Ambil header Authorization
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Authorization header tidak ada".into()))?;

        // 2. Strip "Bearer "
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("Format token salah, gunakan Bearer".into()))?;

        // 3. Verify JWT
        let claims = verify_token(token, &state.config.jwt_access_secret)?;

        // 4. Pastikan token type = "access"
        if claims.token_type != "access" {
            return Err(AppError::Unauthorized("Bukan access token".into()));
        }

        // 5. Cek blacklist Redis
        let blacklist_key = cache::blacklist_key(&claims.jti);
        if cache::exists(&state.redis, &blacklist_key).await? {
            return Err(AppError::Unauthorized("Token sudah di-logout".into()));
        }

        Ok(AuthUser(claims))
    }
}

// ─── Extractor: AdminUser ─────────────────────────────────────────────────────
// Hanya untuk role "admin"

#[derive(Debug, Clone)]
pub struct AdminUser(pub Claims);

#[async_trait]
impl FromRequestParts<AppState> for AdminUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> AppResult<Self> {
        let AuthUser(claims) = AuthUser::from_request_parts(parts, state).await?;

        if claims.role != "admin" {
            return Err(AppError::Forbidden("Hanya admin yang bisa mengakses ini".into()));
        }

        Ok(AdminUser(claims))
    }
}
