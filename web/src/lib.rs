#![feature(fs_try_exists)]
pub mod app;
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

#[cfg(feature = "ssr")]
pub mod server {
    use std::sync::RwLock;

    #[derive(Debug)]
    pub struct WebState {
        pub config: RwLock<Config>,
    }

    impl WebState {
        pub fn new() -> Result<Self, anyhow::Error> {
            Ok(Self {
                config: RwLock::new(Config::read()?)
            })
        }
    }

    use std::{path::Path, fs};

    use serde::{Serialize, Deserialize};

    use crate::{CONFIG_DIRECTORY, CONFIG_FILE_NAME};

    #[derive(Debug, Deserialize, Serialize, Default)]
    pub struct Config {
        pub setup_complete: bool,
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

const CONFIG_DIRECTORY: &str = "/etc/codecheck";
const CONFIG_FILE_NAME: &str = "config.toml";
