use envconfig::Envconfig;
use serde::Deserialize;

#[derive(Envconfig, Debug, Deserialize)]
pub struct Config {
    // Maps the "PORT" environment variable to this field.
    #[envconfig(from = "PORT", default = "8080")]
    pub port: u16,

    // Maps the "LOG_LEVEL" variable; default is "info".
    #[envconfig(from = "LOG_LEVEL", default = "info")]
    pub log_level: String,
}

