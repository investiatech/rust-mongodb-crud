use std::env;

pub struct Config {
    pub mongodb_uri: String,
    pub mongodb_database: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            mongodb_uri: env::var("MONGODB_URI")?,
            mongodb_database: env::var("MONGODB_DATABASE")?,
        })
    }
}