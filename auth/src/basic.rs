use anyhow::Result;
use chrono::Utc;
use db::models::{Session, User};
use mongodb::{Database, bson::doc};
use rand::{rngs::StdRng, SeedableRng, Rng, distributions::Alphanumeric};
use thiserror::Error;
use crate::ValidatedUser;
use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}, PasswordVerifier, PasswordHash};
use goldleaf::{AutoCollection, CollectionIdentity};
use std::time::Duration;
use crate::AuthError;


pub async fn enroll_root(db: &Database, username: String, password: String) -> Result<(), AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    let password = Argon2::default().hash_password(password.as_bytes(), &salt).map_err(|e| AuthError::PasswordError(e.to_string()))?.to_string();

    db.auto_collection::<User>().insert_one(User {
        id: None,
        username,
        role: db::Role::Admin,
        password,
        sessions: Vec::new(),
    }, None).await?;

    Ok(())
}

/// Authenticates a user
pub async fn authenticate(db: &Database, username: &str, password: &str) -> Result<String, AuthError> {
    let user = db.auto_collection::<User>().find_one(doc! {
        "username": username
    }, None).await?;

    let mut user = match user {
        Some(user) => user,
        None => return Err(AuthError::UserDoesNotExist.into()),
    };

    if let Err(e) = Argon2::default().verify_password(password.as_bytes(), &PasswordHash::new(&user.password).map_err(|e| AuthError::PasswordError(e.to_string()))?) {
        return Err(match e {
            argon2::password_hash::Error::Password => AuthError::InvalidPassword,
            _ => AuthError::PasswordError(e.to_string())
        }.into())
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
