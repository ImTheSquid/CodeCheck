#![feature(fs_try_exists)]
#![feature(adt_const_params)]
#![feature(iterator_try_collect)]
#![deny(clippy::unwrap_used)]
pub mod admin;
pub mod app;
pub mod home;
pub mod login;
pub mod setup;

use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;
      use leptos::*;

      console_error_panic_hook::set_once();

      leptos::mount_to_body(App);
    }
}
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum RoleRequirementCondition {
    ExactOrGreater,
    Exact
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct RoleRequirement {
    role: db::Role,
    condition: RoleRequirementCondition,
}

impl RoleRequirement {
    pub fn new(role: db::Role, condition: RoleRequirementCondition) -> Self {
        Self {
            role,
            condition,
        }
    }

    pub fn includes_role(&self, role: db::Role) -> bool {
        match self.condition {
            RoleRequirementCondition::ExactOrGreater => role >= self.role,
            RoleRequirementCondition::Exact => role == self.role,
        }
    }
}

impl Default for RoleRequirement {
    fn default() -> Self {
        Self {
            role: db::Role::Assistant,
            condition: RoleRequirementCondition::ExactOrGreater,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct HumanReadableUser {
    pub id: String,
    pub name: String,
    pub username: String,
}

#[cfg(feature = "ssr")]
#[derive(Debug)]
pub struct AuthedUser<const R: db::Role> {
    pub id: db::UserId,
    pub token: String,
}

#[cfg(feature = "ssr")]
impl<const R: db::Role> actix_web::FromRequest for AuthedUser<R> {
    type Error = auth::AuthError;
    type Future = futures_util::future::LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let cookie = match req.cookie("session") {
            None => return Box::pin(async move { Err(auth::AuthError::InvalidSession) }),
            Some(c) => c,
        };

        let data = req
            .app_data::<actix_web::web::Data<server::WebState>>()
            .unwrap()
            .clone();

        Box::pin(async move {
            let validation_result = match auth::validate_token(&data.database, cookie.value()).await
            {
                Ok(validated_user) => validated_user,
                Err(e) => return Err(e),
            };

            Ok(AuthedUser {
                id: validation_result.id,
                token: cookie.value().to_string(),
            })
        })
    }
}

#[cfg(feature = "ssr")]
pub mod server_prelude {
    pub use crate::server::WebState;
    pub use crate::AuthedUser;
    pub use actix_web::web::Data;
    pub use actix_web::HttpRequest;
    pub use goldleaf::{AutoCollection, CollectionIdentity};
    pub use leptos_actix::{extract, ResponseOptions};
    pub use mongodb::bson::{doc, from_document, oid::ObjectId};
    pub use db::Role;
}

#[cfg(feature = "ssr")]
pub mod server {
    use mongodb::Database;
    use std::{env, sync::RwLock};

    const CONFIG_DIRECTORY: &str = "/etc/codecheck";
    const CONFIG_FILE_NAME: &str = "config.toml";

    #[derive(Debug)]
    pub struct WebState {
        pub config: RwLock<Config>,
        pub database: Database,
    }

    impl WebState {
        pub async fn new() -> Result<Self, anyhow::Error> {
            Ok(Self {
                config: RwLock::new(Config::read()?),
                database: db::connect(
                    &env::var("CODECHECK_MONGO_URI")
                        .expect("`CODECHECK_MONGO_URI` variable required!"),
                    "codecheck",
                )
                .await?,
            })
        }
    }

    use std::{fs, path::Path};

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Default)]
    pub struct Config {
        pub setup_complete: bool,
        pub vocareum_key: Option<String>,
    }

    impl Config {
        pub fn read() -> Result<Self, anyhow::Error> {
            // Ensure path exists
            let config_path = Path::new(CONFIG_DIRECTORY).join(Path::new(CONFIG_FILE_NAME));
            if !fs::try_exists(&config_path)? {
                fs::create_dir_all(Path::new(CONFIG_DIRECTORY))?;
                fs::write(&config_path, toml::to_string(&Config::default())?)?;
                return Ok(Config::default());
            }

            Ok(toml::from_str(&fs::read_to_string(&config_path)?)?)
        }

        pub fn write(&self) -> Result<(), anyhow::Error> {
            let config_path = Path::new(CONFIG_DIRECTORY).join(Path::new(CONFIG_FILE_NAME));
            fs::write(config_path, toml::to_string(self)?)?;

            Ok(())
        }
    }
}
