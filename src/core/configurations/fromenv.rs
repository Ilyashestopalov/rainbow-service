use std::env;
use crate::core::configurations::errors::{ConfigurationError};

pub struct DatabaseConfiguration {
    pub driver: String,
    pub host: String,
    pub port: String,
    pub name: String,
    pub user: String,
    pub password: String
}

pub struct Configuration {
    pub db: DatabaseConfiguration,
}

impl Configuration {
    pub fn init() -> Result<Configuration, ConfigurationError> {
        let cfg = Configuration {
            db: {
                DatabaseConfiguration {
                    driver: match env::var("DB_DRIVER") {
                        Ok(val) => val,
                        Err(e) => {
                            return Err(ConfigurationError::MissingVar(
                                format!("couldn't interpret DB_DRIVER: {}", e)
                            ))
                        }
                    },
                    host: match env::var("DB_HOST") {
                        Ok(val) => val,
                        Err(e) => {
                            return Err(ConfigurationError::MissingVar(
                                format!("couldn't interpret DB_HOST: {}", e)
                            ))
                        }
                    },
                    port: match env::var("DB_PORT") {
                        Ok(val) => val,
                        Err(e) => {
                            return Err(ConfigurationError::MissingVar(
                                format!("couldn't interpret DB_PORT: {}", e)
                            ))
                        }
                    },
                    name: match env::var("DB_NAME") {
                        Ok(val) => val,
                        Err(e) => {
                            return Err(ConfigurationError::MissingVar(
                                format!("couldn't interpret DB_NAME: {}", e)
                            ))
                        }
                    },
                    user: match env::var("DB_USER") {
                        Ok(val) => val,
                        Err(e) => {
                            return Err(ConfigurationError::MissingVar(
                                format!("couldn't interpret DB_USER: {}", e)
                            ))
                        }
                    },
                    password: match env::var("DB_PASSWORD") {
                        Ok(val) => val,
                        Err(e) => {
                            return Err(ConfigurationError::MissingVar(
                                format!("couldn't interpret DB_PASSWORD: {}", e)
                            ))
                        }
                    }
                }
            }
        }; 
        Ok(cfg)
    }
}
