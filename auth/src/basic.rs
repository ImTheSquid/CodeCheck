use crate::AuthError;
use crate::ValidatedUser;
use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use chrono::Utc;
use db::models::{Session, User};
use goldleaf::{AutoCollection, CollectionIdentity};
use mongodb::{bson::doc, Database};
use rand::{distributions::Alphanumeric, rngs::StdRng, Rng, SeedableRng};
use std::time::Duration;
use thiserror::Error;

pub async fn enroll_root(
    db: &Database,
    username: String,
    password: String,
) -> Result<(), AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    let password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AuthError::PasswordError(e.to_string()))?
        .to_string();

    db.auto_collection::<User>()
        .insert_one(
            User {
                id: None,
                username,
                name: "Admin User".to_string(),
                role: db::Role::Admin,
                password,
                sessions: Vec::new(),
                email: None,
            },
            None,
        )
        .await?;

    Ok(())
}

/// Authenticates a user
pub async fn authenticate(
    db: &Database,
    username: &str,
    password: &str,
) -> Result<String, AuthError> {
    let user = db
        .auto_collection::<User>()
        .find_one(
            doc! {
                "username": username
            },
            None,
        )
        .await?;

    let mut user = match user {
        Some(user) => user,
        None => return Err(AuthError::UserDoesNotExist.into()),
    };

    if let Err(e) = Argon2::default().verify_password(
        password.as_bytes(),
        &PasswordHash::new(&user.password).map_err(|e| AuthError::PasswordError(e.to_string()))?,
    ) {
        return Err(match e {
            argon2::password_hash::Error::Password => AuthError::InvalidPassword,
            _ => AuthError::PasswordError(e.to_string()),
        }
        .into());
    }

    // Create a session
    let token = StdRng::from_entropy()
        .sample_iter(Alphanumeric)
        .take(64)
        .map(char::from)
        .collect::<String>();

    user.sessions.push(Session {
        token: token.clone(),
        timestamp: Utc::now(),
        last_use: Utc::now(),
    });

    user.save(db).await?;

    Ok(token)
}
