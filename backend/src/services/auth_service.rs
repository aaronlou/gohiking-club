use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::AuthConfig;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // user_id
    email: String,
    exp: i64,
    iat: i64,
}

pub struct AuthService {
    jwt_secret: String,
    jwt_expires_in_secs: u64,
}

impl AuthService {
    pub fn new(config: &AuthConfig) -> Self {
        Self {
            jwt_secret: config.jwt_secret.clone(),
            jwt_expires_in_secs: config.jwt_expires_in_secs,
        }
    }

    pub fn hash_password(&self, password: &str) -> anyhow::Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {e}"))?;
        Ok(password_hash.to_string())
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> bool {
        let parsed_hash = match PasswordHash::new(hash) {
            Ok(h) => h,
            Err(_) => return false,
        };
        let argon2 = Argon2::default();
        argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok()
    }

    pub fn create_token(&self, user_id: Uuid, email: &str) -> anyhow::Result<String> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.jwt_expires_in_secs as i64);

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| anyhow::anyhow!("Failed to create token: {e}"))?;

        Ok(token)
    }

    pub fn verify_token(&self, token: &str) -> anyhow::Result<Uuid> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| anyhow::anyhow!("Invalid token: {e}"))?;

        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|e| anyhow::anyhow!("Invalid user_id in token: {e}"))?;

        Ok(user_id)
    }
}
