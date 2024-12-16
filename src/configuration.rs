use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::PgConnectOptions;
use sqlx::ConnectOptions;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub url: Secret<String>,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn get_connect_options(&self) -> PgConnectOptions {
        // We actually do not want SSL in Fly
        /*let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };*/
        
        let url = self.url.expose_secret().parse().expect("Invalid database URL.");
        let opts = PgConnectOptions::from_url(&url)
            .expect("Failed to parse database URL into ConnectOptions.")
            //.ssl_mode(ssl_mode)
            .log_statements(tracing_log::log::LevelFilter::Trace);

        opts
    }
}

pub enum Environment {
    Development,
    Production,
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "production" =>  Ok(Self::Production),
            "development" => Ok(Self::Development),
            other => Err(format!("{} is not a supported environment.", other)),
        }
    }
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Development => "development",
            Self::Production => "production",
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine current directory");

    let configuration_directory = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "development".into())
        .try_into()
        .expect("Failed to parse APP_ENV");

    let environment_filename = format!(
        "{}.yaml",
        environment.as_str()
    );

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml")
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename)
        ))
        .add_source(config::Environment::default().separator("_"))
        .build()?;

    settings.try_deserialize::<Settings>()
}
