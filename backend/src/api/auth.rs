use axum::{extract::State, http::StatusCode, Json};

use crate::models::user::{
    AuthResponse, LoginByPasswordRequest, RegisterByPasswordRequest, UserResponse,
};
use crate::api::auth_extractor::AuthenticatedUser;
use crate::repositories::user_repository::UserRepository;
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

    let user_repo = UserRepository::new(&state.pool);

    // Check email uniqueness
    if user_repo.exists_by_email(&req.email).await.map_err(internal_error)? {
        return Err((StatusCode::CONFLICT, "Email already registered".into()));
    }

    let password_hash = state
        .auth_service
        .hash_password(&req.password)
        .map_err(internal_error)?;

    let user = user_repo
        .create(&req.username, &req.email, &password_hash)
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
        .map_err(internal_error)?;

    Ok(Json(AuthResponse {
        token,
        user: user.into(),
    }))
}

/// Login with email + password.
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginByPasswordRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let user_repo = UserRepository::new(&state.pool);

    let user = user_repo
        .find_by_email(&req.email)
        .await
        .map_err(internal_error)?
        .ok_or_else(|| {
            (StatusCode::UNAUTHORIZED, "Invalid email or password".into())
        })?;

    if !state.auth_service.verify_password(&req.password, &user.password_hash) {
        return Err((StatusCode::UNAUTHORIZED, "Invalid email or password".into()));
    }

    let token = state
        .auth_service
        .create_token(user.id, &user.email)
        .map_err(internal_error)?;

    let photo_count = user_repo.get_photo_count(user.id).await.unwrap_or(0);

    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            avatar_url: user.avatar_url,
            bio: user.bio,
            photo_count,
            created_at: user.created_at,
        },
    }))
}

/// Get current user profile from JWT.
pub async fn me(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let user_repo = UserRepository::new(&state.pool);

    let user = user_repo
        .find_by_id(auth_user.id)
        .await
        .map_err(internal_error)?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "User not found".into()))?;

    let photo_count = user_repo.get_photo_count(user.id).await.unwrap_or(0);

    Ok(Json(UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        avatar_url: user.avatar_url,
        bio: user.bio,
        photo_count,
        created_at: user.created_at,
    }))
}

fn internal_error(e: anyhow::Error) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}
