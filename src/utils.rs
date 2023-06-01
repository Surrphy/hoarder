use std::collections::HashMap;

use pgp::{SignedPublicKey, Deserializable};
use sqlx::PgPool;
use sqlx::Error;

use pgp::{Message, crypto::sym::SymmetricKeyAlgorithm};

use axum::http::StatusCode;
use sqlx::Row;

pub struct AppState {
    pub pool: PgPool,
    pub connected_users: HashMap<String, (User, String)>
}

impl AppState {
    pub fn add_connected_user(&mut self, user: User, api_token: String) {
        self.connected_users.insert(user.user_fingerprint.clone(), (user, api_token));
    }
}

#[derive(Clone)]
pub struct User {
    user_id: i32,
    user_fingerprint: String,
    user_public_key: SignedPublicKey,
    is_admin: bool
}

impl User {
    pub fn get_pub_key(&self) -> &SignedPublicKey {
        &self.user_public_key
    }
}

pub async fn get_user(pool: &PgPool, fingerprint: &str) -> Result<User, StatusCode> {
    let user = match sqlx::query("SELECT * FROM users WHERE user_fingerprint = $1")
        .bind(fingerprint)
        .fetch_one(pool)
        .await {
        Ok(val) => val,
        Err(Error::RowNotFound) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    let public_key = match SignedPublicKey::from_string(user.get("user_public_key")) {
        Ok(val) => val.0,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    Ok(User { user_id: user.get("user_id"), user_fingerprint: user.get("user_fingerprint"), user_public_key: public_key, is_admin: user.get("is_admin") })
}

pub async fn encrypt(s: &str, pub_key: &SignedPublicKey) -> Result<String, StatusCode> {
    let mut rng = rand::thread_rng();

    let secret = Message::new_literal("none", s);
    let secret = match secret.encrypt_to_keys(&mut rng, SymmetricKeyAlgorithm::AES128, &[pub_key]) {
        Ok(val) => val,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    let secret = match secret.to_armored_string(None) {
        Ok(val) => val,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };
    
    Ok(secret)
}
