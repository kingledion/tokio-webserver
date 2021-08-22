use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::fmt;

const CONFIG_FILE_PREFIX: &str = "./config/";

#[derive(Clone, Debug, Deserialize)]
pub enum ENV {
    Local,
    Container,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::Local => write!(f, "Local"),
            ENV::Container => write!(f, "Container"),
        }
    }
}

impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "Local" => ENV::Local,
            "Container" => ENV::Container,
            _ => ENV::Local,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
    // for some reason this can't be read from the toml file?
    // TODO: error catching to allow part of the env variables to be bound; or, optional bindings
    pub hostname: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Repository {
    pub db_host: String,
    pub db_user: String,
    pub db_pass: String,
}


#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub repository: Repository,
    pub env: ENV,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "Local".into());
        let mut c = Config::new();
        c.set("env", env.clone())?;

        c.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))?;

        c.try_into()
    }
}


