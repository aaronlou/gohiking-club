use axum::{extract::State, http::StatusCode, Json};

use crate::models::user::{
    AuthResponse, LoginByPasswordRequest, RegisterByPasswordRequest, UserResponse,
};
use crate::api::auth_extractor::AuthenticatedUser;
use crate::AppState;

/// Register with username + email + password.
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterByPasswordRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    if req.username.is_empty() || req.email.is_empty() || req.password.len() < 6 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Username, email, and password (min 6 chars) required".into(),
        ));
    }

    // Check email uniqueness
    let existing = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM users WHERE email = $1",
    )
    .bind(&req.email)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if existing > 0 {
        return Err((StatusCode::CONFLICT, "Email already registered".into()));
    }

    let password_hash = state
        .auth_service
        .hash_password(&req.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = sqlx::query_as::<_, crate::models::user::User>(
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(&req.username)
    .bind(&req.email)
    .bind(&password_hash)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate") {
            (StatusCode::CONFLICT, "Username already taken".into())
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    })?;

    let token = state
        .auth_service
        .create_token(user.id, &user.email)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            avatar_url: user.avatar_url,
            bio: user.bio,
            photo_count: 0,
            created_at: user.created_at,
        },
    }))
}

/// Login with email + password.
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginByPasswordRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let user = sqlx::query_as::<_, crate::models::user::User>(
        "SELECT * FROM users WHERE email = $1",
    )
    .bind(&req.email)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        (StatusCode::UNAUTHORIZED, "Invalid email or password".into())
    })?;

    if !state.auth_service.verify_password(&req.password, &user.password_hash) {
        return Err((StatusCode::UNAUTHORIZED, "Invalid email or password".into()));
    }

    let token = state
        .auth_service
        .create_token(user.id, &user.email)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Count approved photos
    let photo_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM photos WHERE user_id = $1 AND status = 'approved'",
    )
    .bind(user.id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            avatar_url: user.avatar_url,
            bio: user.bio,
            photo_count: photo_count.0,
            created_at: user.created_at,
        },
    }))
}

/// Get current user profile from JWT.
pub async fn me(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let user = sqlx::query_as::<_, crate::models::user::User>(
        "SELECT * FROM users WHERE id = $1",
    )
    .bind(auth_user.id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, "User not found".into()))?;

    let photo_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM photos WHERE user_id = $1 AND status = 'approved'",
    )
    .bind(user.id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        avatar_url: user.avatar_url,
        bio: user.bio,
        photo_count: photo_count.0,
        created_at: user.created_at,
    }))
}
