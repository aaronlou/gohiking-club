use axum::{extract::State, http::StatusCode, Json};

use crate::models::user::{
    AuthResponse, LoginByPasswordRequest, RegisterByPasswordRequest, UserResponse,
};
use crate::api::auth_extractor::AuthenticatedUser;
use crate::repositories::user_repository::UserRepository;
use crate::AppState;

/// Register with username + password only.
/// Email is auto-generated internally to satisfy DB constraints.
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterByPasswordRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let username = req.username.trim();
    if username.is_empty() || req.password.len() < 6 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Username and password (min 6 chars) required".into(),
        ));
    }

    let user_repo = UserRepository::new(&state.pool);

    // Check username uniqueness
    if user_repo.exists_by_username(username).await.map_err(internal_error)? {
        return Err((StatusCode::CONFLICT, "用户名已被使用".into()));
    }

    let password_hash = state
        .auth_service
        .hash_password(&req.password)
        .map_err(internal_error)?;

    // Auto-generate email to satisfy DB NOT NULL constraint
    let email = format!("{}@user.gohiking", username);

    let user = user_repo
        .create(username, &email, &password_hash)
        .await
        .map_err(|e| {
            if e.to_string().contains("duplicate") {
                (StatusCode::CONFLICT, "用户名或邮箱已被使用".into())
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

/// Login with username + password.
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginByPasswordRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let user_repo = UserRepository::new(&state.pool);

    let user = user_repo
        .find_by_username(&req.username)
        .await
        .map_err(internal_error)?
        .ok_or_else(|| {
            (StatusCode::UNAUTHORIZED, "用户名或密码错误".into())
        })?;

    if !state.auth_service.verify_password(&req.password, &user.password_hash) {
        return Err((StatusCode::UNAUTHORIZED, "用户名或密码错误".into()));
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
        .ok_or_else(|| (StatusCode::NOT_FOUND, "用户不存在".into()))?;

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
