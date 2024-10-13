use crate::queries::is_alphanumeric_underscore_hyphen;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use sea_orm::query::ConnectionTrait;
use sea_orm::query::Statement;
use sea_orm::DatabaseConnection;
use sea_orm::DbBackend;
use std::fmt::Display;

/// Runs a query which will create a new database with the given name.
pub async fn query_create_database_from_template<S, T>(
    db_connection: &DatabaseConnection,
    name: &S,
    template_name: &T,
) -> Result<()>
where
    S: Display,
    T: Display,
{
    let db_backend = db_connection.get_database_backend();
    let db_name = name.to_string();
    let db_template_name = template_name.to_string();
    let create_db_statement =
        create_database_statement_from_template(db_backend, &db_name, &db_template_name)?;

    db_connection
        .execute(create_db_statement)
        .await
        .with_context(|| format!("Trying to create new database with name '{}'", db_name))?;

    Ok(())
}

fn create_database_statement_from_template(
    db_backend: DbBackend,
    db_name: &str,
    db_template_name: &str,
) -> Result<Statement> {
    if !is_alphanumeric_underscore_hyphen(db_name) {
        return Err(anyhow!(
            "Given database name is empty or contains non-alphanumeric characters '{}'",
            db_name
        ));
    }

    let statement = match db_backend {
        DbBackend::Postgres => {
            let raw_sql = format!(r#"CREATE DATABASE "{db_name}" TEMPLATE "{db_template_name}""#);
            let statement = Statement::from_string(db_backend, raw_sql);

            statement
        }
        _ => {
            unimplemented!("Unsupported db backend used")
        }
    };

    Ok(statement)
}

#[cfg(test)]
mod query_create_database_from_template {
    use crate::new_database_connection;
    use crate::queries::query_create_random_database;
    use crate::queries::query_create_random_database_from_template;

    const POSTGRES_LOCAL_DB_URL: &'static str = &"postgres://user:password@localhost:5432/tea-orm";

    #[tokio::test]
    async fn it_should_clone_db() {
        let db_connection = new_database_connection(POSTGRES_LOCAL_DB_URL.to_string())
            .await
            .unwrap();
        let db_name = query_create_random_database(&db_connection).await.unwrap();
        let result = query_create_random_database_from_template(&db_connection, &db_name).await;

        assert!(result.is_ok());
    }
}
