use anyhow::Result;
use chrono::{DateTime, Utc};
use thiserror::Error;
use db::{UserId, Role};

#[cfg(feature = "saml_auth")]
pub mod saml;

#[cfg(all(feature = "basic_auth", feature = "server"))]
pub mod basic;

#[cfg(feature = "server")]
#[derive(Debug, Error)]
pub enum AuthError {
    #[error(transparent)]
    Mongo(#[from] mongodb::error::Error),
    #[error("Invalid or expired token")]
    InvalidToken,
}

#[cfg(feature = "server")]
pub async fn validate_session(db: &mongodb::Database, token: &str) -> Result<UserId, AuthError> {
    use mongodb::bson::doc;
    use goldleaf::AutoCollection;
    use db::models::User;

    let mut user = db.auto_collection::<User>().find_one(doc! {
        "sessions.token": token,
        "sessions.expiration": Utc::now(),
    }, None).await?.ok_or(AuthError::InvalidToken)?;

    user.sessions.iter_mut().find(|s| s.token == token).unwrap().last_use = Utc::now();

    Ok(user.id.unwrap())
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct ValidatedUser {
    pub id: UserId,
    pub role: Role,
}
