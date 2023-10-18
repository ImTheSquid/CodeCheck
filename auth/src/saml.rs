use actix_web::web;
use async_trait::async_trait;

use crate::{AuthProvider, AuthError};

pub struct SamlAuthProvider {

}

#[async_trait]
impl AuthProvider for SamlAuthProvider {
    async fn logout(&self) -> Result<(), AuthError> {
        todo!()
    }

    fn register_endpoints(&self, cfg: &mut web::ServiceConfig) {
        todo!()
    }
}