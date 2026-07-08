use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    errors::{AppError, AppResult},
    middleware::auth::AdminUser,
    models::user::{User, UserResponse},
    state::AppState,
};

// ─── DTOs ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateRoleRequest {
    #[validate(length(min = 1))]
    pub role: String,
}

// ─── GET /api/v1/admin/users ──────────────────────────────────────────────────
// List semua user (admin only)

pub async fn list_users(
    State(state): State<AppState>,
    AdminUser(_claims): AdminUser,
) -> AppResult<Json<serde_json::Value>> {
    let users: Vec<User> = sqlx::query_as(
        "SELECT * FROM users ORDER BY created_at DESC"
    )
    .fetch_all(&state.db)
    .await?;

    let responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();

    Ok(Json(serde_json::json!({
        "success": true,
        "total": responses.len(),
        "users": responses
    })))
}

// ─── GET /api/v1/admin/users/:id ─────────────────────────────────────────────
// Detail satu user (admin only)

pub async fn get_user(
    State(state): State<AppState>,
    AdminUser(_claims): AdminUser,
    Path(user_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let user: Option<User> = sqlx::query_as(
        "SELECT * FROM users WHERE id = $1"
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

// ─── PATCH /api/v1/admin/users/:id/role ──────────────────────────────────────
// Update role user (admin only)

pub async fn update_role(
    State(state): State<AppState>,
    AdminUser(claims): AdminUser,
    Path(user_id): Path<Uuid>,
    Json(body): Json<UpdateRoleRequest>,
) -> AppResult<Json<serde_json::Value>> {
    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Validasi nilai role
    let valid_roles = ["user", "admin", "moderator"];
    if !valid_roles.contains(&body.role.as_str()) {
        return Err(AppError::Validation(
            format!("Role tidak valid. Pilihan: {}", valid_roles.join(", "))
        ));
    }

    // Admin tidak bisa ubah role dirinya sendiri
    if claims.sub == user_id.to_string() {
        return Err(AppError::Forbidden("Tidak bisa mengubah role diri sendiri".into()));
    }

    let user: Option<User> = sqlx::query_as(
        r#"
        UPDATE users
        SET role = $1, updated_at = $2
        WHERE id = $3
        RETURNING *
        "#
    )
    .bind(&body.role)
    .bind(Utc::now())
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?;

    let user = user.ok_or_else(|| AppError::NotFound("User tidak ditemukan".into()))?;

    tracing::info!(
        "Admin {} mengubah role user {} menjadi {}",
        claims.email, user.email, body.role
    );

    Ok(Json(serde_json::json!({
        "success": true,
        "message": format!("Role berhasil diubah menjadi '{}'", body.role),
        "user": UserResponse::from(user)
    })))
}

// ─── PATCH /api/v1/admin/users/:id/deactivate ────────────────────────────────
// Nonaktifkan user (admin only)

pub async fn deactivate_user(
    State(state): State<AppState>,
    AdminUser(claims): AdminUser,
    Path(user_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    // Admin tidak bisa nonaktifkan dirinya sendiri
    if claims.sub == user_id.to_string() {
        return Err(AppError::Forbidden("Tidak bisa menonaktifkan akun sendiri".into()));
    }

    let user: Option<User> = sqlx::query_as(
        r#"
        UPDATE users
        SET is_active = false, updated_at = $1
        WHERE id = $2
        RETURNING *
        "#
    )
    .bind(Utc::now())
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?;

    let user = user.ok_or_else(|| AppError::NotFound("User tidak ditemukan".into()))?;

    tracing::warn!(
        "Admin {} menonaktifkan user {}",
        claims.email, user.email
    );

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "User berhasil dinonaktifkan",
        "user": UserResponse::from(user)
    })))
}

// ─── PATCH /api/v1/admin/users/:id/activate ──────────────────────────────────
// Aktifkan kembali user (admin only)

pub async fn activate_user(
    State(state): State<AppState>,
    AdminUser(claims): AdminUser,
    Path(user_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let user: Option<User> = sqlx::query_as(
        r#"
        UPDATE users
        SET is_active = true, updated_at = $1
        WHERE id = $2
        RETURNING *
        "#
    )
    .bind(Utc::now())
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?;

    let user = user.ok_or_else(|| AppError::NotFound("User tidak ditemukan".into()))?;

    tracing::info!(
        "Admin {} mengaktifkan kembali user {}",
        claims.email, user.email
    );

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "User berhasil diaktifkan kembali",
        "user": UserResponse::from(user)
    })))
}

// ─── DELETE /api/v1/admin/users/:id ──────────────────────────────────────────
// Hard delete user (admin only) — gunakan dengan hati-hati!

pub async fn delete_user(
    State(state): State<AppState>,
    AdminUser(claims): AdminUser,
    Path(user_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    if claims.sub == user_id.to_string() {
        return Err(AppError::Forbidden("Tidak bisa menghapus akun sendiri".into()));
    }

    let result = sqlx::query(
        "DELETE FROM users WHERE id = $1"
    )
    .bind(user_id)
    .execute(&state.db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("User tidak ditemukan".into()));
    }

    tracing::warn!("Admin {} menghapus user {}", claims.email, user_id);

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "User berhasil dihapus"
    })))
}
