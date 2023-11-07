use actix_web::web;
use async_trait::async_trait;
use anyhow::Result;
use chrono::{DateTime, Utc};
use thiserror::Error;
use mongodb::{Database, bson::doc};
use db::models::{UserId, User};
use goldleaf::AutoCollection;

#[cfg(feature = "saml_auth")]
pub mod saml;

#[cfg(feature = "basic_auth")]
pub mod basic;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error(transparent)]
    Mongo(#[from] mongodb::error::Error),
    #[error("Invalid or expired token")]
    InvalidToken,
}

#[async_trait]
pub trait AuthProvider {
    async fn logout(&self) -> Result<(), AuthError>;

    fn register_endpoints(&self, cfg: &mut web::ServiceConfig);
}

pub async fn validate_session(db: &Database, token: &str) -> Result<UserId, AuthError> {
    let mut user = db.auto_collection::<User>().find_one(doc! {
        "sessions.token": token,
        "sessions.expiration": Utc::now(),
    }, None).await?.ok_or(AuthError::InvalidToken)?;

    user.sessions.iter_mut().find(|s| s.token == token).unwrap().last_use = Utc::now();

    Ok(user.id.unwrap())
}
