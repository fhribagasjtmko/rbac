use axum::{extract::State, Json};
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    middleware::auth::AuthUser,
    models::user::{User, UserResponse},
    state::AppState,
};

// GET /api/v1/users/me
pub async fn get_me(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
) -> AppResult<Json<serde_json::Value>> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Internal("ID tidak valid".into()))?;

    let user: Option<User> = sqlx::query_as(
        "SELECT * FROM users WHERE id = $1 AND is_active = true"
    )
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?;

    let user = user.ok_or_else(|| AppError::NotFound("User tidak ditemukan".into()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "user": UserResponse::from(user)
    })))
}
