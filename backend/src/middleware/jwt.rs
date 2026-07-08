use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::{AppError, AppResult};

// ─── Claims ──────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,       // user_id
    pub email: String,
    pub role: String,
    pub jti: String,       // JWT ID unik (untuk blacklist)
    pub exp: i64,          // expiry unix timestamp
    pub iat: i64,          // issued at
    pub token_type: String, // "access" | "refresh"
}

// ─── Token generator ─────────────────────────────────────────────────────────

pub fn generate_access_token(
    user_id: Uuid,
    email: &str,
    role: &str,
    secret: &str,
    expires_in: i64,
) -> AppResult<(String, String)> {
    let jti = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        role: role.to_string(),
        jti: jti.clone(),
        exp: now + expires_in,
        iat: now,
        token_type: "access".to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok((token, jti))
}

pub fn generate_refresh_token(
    user_id: Uuid,
    email: &str,
    role: &str,
    secret: &str,
    expires_in: i64,
) -> AppResult<(String, String)> {
    let jti = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        role: role.to_string(),
        jti: jti.clone(),
        exp: now + expires_in,
        iat: now,
        token_type: "refresh".to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok((token, jti))
}

pub fn verify_token(token: &str, secret: &str) -> AppResult<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| AppError::Unauthorized(format!("Token tidak valid: {}", e)))?;

    Ok(token_data.claims)
}
