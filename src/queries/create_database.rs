use ::anyhow::anyhow;
use ::anyhow::Context;
use ::anyhow::Result;
use ::sea_orm::query::ConnectionTrait;
use ::sea_orm::query::Statement;
use ::sea_orm::DatabaseConnection;
use ::sea_orm::DbBackend;
use ::std::fmt::Display;

use crate::queries::is_alphanumeric_underscore_hyphen;
use crate::queries::query_set_database;

/// Runs a query which will create a new database with the given name.
///
/// After the Database is created, it will then run another query to
/// switch the `DatabaseConnection` to use that new database.
pub async fn query_create_database<S>(db_connection: &DatabaseConnection, name: &S) -> Result<()>
where
    S: Display,
{
    let db_backend = db_connection.get_database_backend();
    let db_name = name.to_string();
    let create_db_statement = create_database_statement(db_backend, &db_name)?;

    db_connection
        .execute(create_db_statement)
        .await
        .with_context(|| format!("Trying to create new database with name '{}'", db_name))?;

    query_set_database(db_connection, &db_name).await?;

    Ok(())
}

fn create_database_statement(db_backend: DbBackend, db_name: &str) -> Result<Statement> {
    if !is_alphanumeric_underscore_hyphen(db_name) {
        return Err(anyhow!(
            "Given database name is empty or contains non-alphanumeric characters '{}'",
            db_name
        ));
    }

    let statement = match db_backend {
        DbBackend::Postgres => {
            let raw_sql = format!(r#"CREATE DATABASE "{}""#, db_name);
            let statement = Statement::from_string(db_backend, raw_sql);

            statement
        }
        _ => {
            unimplemented!("Unsupported db backend used")
        }
    };

    Ok(statement)
}
