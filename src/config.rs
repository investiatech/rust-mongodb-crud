use std::env;

pub struct Config {
    pub mongodb_uri: String,
    pub mongodb_database: String,
    pub server_address: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            mongodb_uri: env::var("MONGODB_URI")?,
            mongodb_database: env::var("MONGODB_DATABASE")?,
            server_address: env::var("SERVER_ADDRESS")
                .unwrap_or_else(|_| "0.0.0.0:8000".to_string()),
        })
    }
}
