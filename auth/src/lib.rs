use async_trait::async_trait;
use anyhow::Result;
use thiserror::Error;

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

    fn register_endpoints(&self);
}

pub async fn validate_session() {
    todo!()
}
