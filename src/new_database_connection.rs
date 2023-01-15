use ::anyhow::Result;
use ::log::LevelFilter;
use ::sea_orm::ConnectOptions;
use ::sea_orm::Database;
use ::sea_orm::DatabaseConnection;
use ::serde::Deserialize;
use ::serde::Serialize;
use ::std::time::Duration;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DatabaseConnectionOptions {
    pub sqlx_logging_level: Option<LevelFilter>,

    pub min_connections: Option<u32>,
    pub max_connections: Option<u32>,

    pub acquire_timeout: Option<Duration>,
    pub connection_timeout: Option<Duration>,
    pub idle_timeout: Option<Duration>,

    pub max_lifetime: Option<Duration>,

    pub sqlcipher_key: Option<String>,
}

impl Default for DatabaseConnectionOptions {
    fn default() -> Self {
        Self {
            sqlx_logging_level: None,

            min_connections: None,
            max_connections: None,

            acquire_timeout: None,
            connection_timeout: None,
            idle_timeout: None,
            max_lifetime: None,

            sqlcipher_key: None,
        }
    }
}

pub async fn new_database_connection(url: String) -> Result<DatabaseConnection> {
    new_database_connection_with_options(url, DatabaseConnectionOptions::default()).await
}

pub async fn new_database_connection_with_options(
    url: String,
    options: DatabaseConnectionOptions,
) -> Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(url);

    if let Some(sqlx_logging_level) = options.sqlx_logging_level {
        opt.sqlx_logging(true)
            .sqlx_logging_level(sqlx_logging_level);
    }

    if let Some(min_connections) = options.min_connections {
        opt.min_connections(min_connections);
    }

    if let Some(max_connections) = options.max_connections {
        opt.max_connections(max_connections);
    }

    if let Some(acquire_timeout) = options.acquire_timeout {
        opt.acquire_timeout(acquire_timeout);
    }

    if let Some(connection_timeout) = options.connection_timeout {
        opt.connect_timeout(connection_timeout);
    }

    if let Some(idle_timeout) = options.idle_timeout {
        opt.idle_timeout(idle_timeout);
    }

    if let Some(max_lifetime) = options.max_lifetime {
        opt.max_lifetime(max_lifetime);
    }

    if let Some(sqlcipher_key) = options.sqlcipher_key {
        opt.sqlcipher_key(sqlcipher_key);
    }

    let db_connection = Database::connect(opt).await?;

    Ok(db_connection)
}
