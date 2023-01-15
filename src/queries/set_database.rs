use ::anyhow::anyhow;
use ::anyhow::Context;
use ::anyhow::Result;
use ::sea_orm::query::ConnectionTrait;
use ::sea_orm::query::Statement;
use ::sea_orm::DatabaseConnection;
use ::sea_orm::DbBackend;
use ::std::fmt::Display;

use crate::queries::is_alphanumeric_underscore_hyphen;

pub async fn query_set_database<S>(db_connection: &DatabaseConnection, name: &S) -> Result<()>
where
    S: Display,
{
    let db_backend = db_connection.get_database_backend();
    let db_name = name.to_string();
    let set_db_statement = create_set_database_statement(db_backend, &db_name)?;

    db_connection
        .execute(set_db_statement)
        .await
        .with_context(|| format!("Trying to set to use database '{}'", db_name))?;

    Ok(())
}

fn create_set_database_statement(db_backend: DbBackend, db_name: &str) -> Result<Statement> {
    if !is_alphanumeric_underscore_hyphen(db_name) {
        return Err(anyhow!(
            "Given database name is empty or contains non-alphanumeric characters '{}'",
            db_name
        ));
    }

    let statement = match db_backend {
        DbBackend::Postgres => {
            let raw_sql = format!(r#"SET search_path = "{}""#, db_name);
            let statement = Statement::from_string(db_backend, raw_sql);

            statement
        }
        _ => {
            unimplemented!("Unsupported db backend used")
        }
    };

    Ok(statement)
}
