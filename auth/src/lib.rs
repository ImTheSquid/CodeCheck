use anyhow::Result;
use chrono::{DateTime, Utc};
use thiserror::Error;
use db::{UserId, Role};
use std::time::Duration;

#[cfg(all(feature = "basic_auth", feature = "server"))]
pub mod basic;

#[cfg(feature = "server")]
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
    #[error(transparent)]
    Mongo(#[from] mongodb::error::Error),
}

pub const TOKEN_LIFETIME: Duration = Duration::from_secs(60 * 60 * 24 * 30);

// Validates a token and returns the necessary information for getting a user
#[cfg(feature = "server")]
pub async fn validate_token(db: &mongodb::Database, token: &str) -> Result<ValidatedUser, AuthError> {
    use goldleaf::{AutoCollection, CollectionIdentity};
    use db::models::User;
    use mongodb::bson::doc;
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

#[cfg(feature = "server")]
pub async fn logout(db: &mongodb::Database, token: &str) -> Result<(), AuthError> {
    use goldleaf::{AutoCollection, CollectionIdentity};
    use db::models::User;
    use mongodb::bson::doc;
    let user = db.auto_collection::<User>()
        .find_one(doc! {
            "sessions.token": token
        }, None).await?;

    match user {
        None => Ok(()),
        Some(mut user) => {
            user.sessions.retain(|s| s.token != token);

            user.save(db).await?;

            Ok(())
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct ValidatedUser {
    pub id: UserId,
    pub role: Role,
}
