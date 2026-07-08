use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::{
    handlers::{admin, auth, user},
    state::AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // ── Health check ──────────────────────────────────────
        .route("/health", get(health_check))

        // ── Auth (public) ─────────────────────────────────────
        .route("/api/v1/auth/register", post(auth::register))
        .route("/api/v1/auth/login",    post(auth::login))
        .route("/api/v1/auth/refresh",  post(auth::refresh))

        // ── Auth (protected) ──────────────────────────────────
        .route("/api/v1/auth/logout",   post(auth::logout))

        // ── User (protected) ──────────────────────────────────
        .route("/api/v1/users/me",      get(user::get_me))

        // ── Admin (admin only) ────────────────────────────────
        .route("/api/v1/admin/users",                         get(admin::list_users))
        .route("/api/v1/admin/users/:id",                     get(admin::get_user))
        .route("/api/v1/admin/users/:id/role",                patch(admin::update_role))
        .route("/api/v1/admin/users/:id/deactivate",          patch(admin::deactivate_user))
        .route("/api/v1/admin/users/:id/activate",            patch(admin::activate_user))
        .route("/api/v1/admin/users/:id",                     delete(admin::delete_user))

        .with_state(state)
}

async fn health_check() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "ok",
        "service": "backend-api",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
