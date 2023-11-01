use actix_web::web;
use async_trait::async_trait;
use anyhow::Result;
use thiserror::Error;
use mongodb::Database;
use db::models::UserId;

#[cfg(feature = "saml_auth")]
pub mod saml;

#[cfg(feature = "basic_auth")]
pub mod basic;

#[derive(Debug, Error)]
pub enum AuthError {

}

#[async_trait]
pub trait AuthProvider {
    async fn logout(&self) -> Result<(), AuthError>;

    fn register_endpoints(&self, cfg: &mut web::ServiceConfig);
}

pub async fn validate_session(db: &Database, token: &str) -> Result<UserId, AuthError> {
    todo!()
}
