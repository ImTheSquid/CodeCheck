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

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Error while checking password: {0}")]
    PasswordError(String),
    #[error("User does not exist")]
    UserDoesNotExist,
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Invalid session")]
    InvalidSession,
}

pub async fn enroll_root(db: &Database, username: String, password: String) -> Result<()> {
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
pub async fn authenticate(db: &Database, username: &str, password: &str) -> Result<String> {
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

const TOKEN_LIFETIME: Duration = Duration::from_secs(60 * 60 * 24 * 30);

// Validates a token and returns the necessary information for getting a user
pub async fn validate_token(db: &Database, token: &str) -> Result<ValidatedUser> {
    let user = db.auto_collection::<User>()
        .find_one(doc! {
            "sessions.token": token
        }, None).await?;

    match user {
        None => Err(AuthError::InvalidSession.into()),
        Some(mut user) => {
            let session = user.sessions.iter_mut().find(|s| s.token == token).unwrap();

            if session.timestamp + TOKEN_LIFETIME < Utc::now() {
                user.sessions.retain(|s| s.token != token);
                user.save(db).await?;
                return Err(AuthError::InvalidSession.into());
            }

            session.last_use = Utc::now();

            user.save(db).await?;

            Ok(ValidatedUser {
                id: user.id.unwrap(),
                role: user.role
            })
        }
    }
}
