#![feature(fs_try_exists)]
#![feature(adt_const_params)]
pub mod app;
pub mod setup;
pub mod login;
pub mod admin;

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

#[cfg(feature = "ssr")]
pub struct AuthedUser<const R: db::Role> {
    pub id: db::UserId,
    pub token: String,
}

#[cfg(feature = "ssr")]
impl<const R: db::Role> actix_web::FromRequest for AuthedUser<R> {
    type Error = auth::AuthError;
    type Future = futures_util::future::LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let cookie = match req.cookie("sessionToken") {
            None => return Box::pin(async move { Err(auth::AuthError::InvalidSession) }),
            Some(c) => c,
        };

        let data = req.app_data::<actix_web::web::Data<server::WebState>>().unwrap().clone(); 

        Box::pin(async move {
            let validation_result = match auth::validate_token(&data.database, cookie.value()).await {
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
pub mod server {
    use std::{sync::RwLock, env};
    use mongodb::Database;

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
                database: db::connect(&env::var("CODECHECK_MONGO_URI").expect("`CODECHECK_MONGO_URI` variable required!"), "codecheck").await?, 
            })
        }
    }

    use std::{path::Path, fs};

    use serde::{Serialize, Deserialize};

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