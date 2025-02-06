use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct TrinoSettings {
    pub endpoint: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct S3Settings {
    pub endpoint: String,
    pub region: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub(crate) struct Settings {
    pub trino: TrinoSettings,
    pub s3: S3Settings,
}

impl Settings {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("/etc/redata/redata.yaml").required(false))
            .add_source(File::with_name("redata.json").required(false))
            // Add in settings from the environment (with a prefix of REDATA)
            .add_source(Environment::with_prefix("redata"))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}
