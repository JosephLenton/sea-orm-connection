use ::anyhow::Context;
use ::anyhow::Result;
use ::sea_orm::query::ConnectionTrait;
use ::sea_orm::query::Statement;
use ::sea_orm::DatabaseConnection;
use ::sea_orm::DbBackend;

pub async fn query_list_databases(db_connection: &DatabaseConnection) -> Result<Vec<String>> {
    let db_backend = db_connection.get_database_backend();
    let list_databases_statement = create_list_databases_statement(db_backend);

    let results = db_connection
        .query_all(list_databases_statement)
        .await
        .with_context(|| format!("Trying to list all databases"))?;

    let database_names: Vec<String> = results
        .into_iter()
        .map(|row_result| {
            let database_name = row_result
                .try_get::<String>("", "database_name")
                .expect("expect `database_name` to be present in SQL Query results");

            database_name
        })
        .collect();

    Ok(database_names)
}

fn create_list_databases_statement(db_backend: DbBackend) -> Statement {
    match db_backend {
        DbBackend::Postgres => Statement::from_string(
            db_backend,
            "SELECT datname AS database_name FROM pg_database
          WHERE datistemplate = false"
                .to_string(),
        ),
        _ => {
            unimplemented!("Unsupported db backend used")
        }
    }
}

#[cfg(test)]
mod query_list_databases {
    use crate::new_database_connection;
    use crate::queries::query_create_random_database;
    use crate::queries::query_list_databases;

    const POSTGRES_LOCAL_DB_URL: &'static str = &"postgres://user:password@localhost:5432/tea-orm";

    #[tokio::test]
    async fn is_should_list_the_current_number_of_dbs_after_creation() {
        let db_connection = new_database_connection(POSTGRES_LOCAL_DB_URL.to_string())
            .await
            .unwrap();
        let dbs = query_list_databases(&db_connection).await.unwrap();
        let new_database_name = query_create_random_database(&db_connection).await.unwrap();
        let new_dbs = query_list_databases(&db_connection).await.unwrap();

        // We expect the number of databases to have increased,
        // since I just made one above.
        //
        // However it's possible another test also created a DB at the same time.
        // Resulting in it going up at more than 1.
        assert_eq!(dbs.contains(&new_database_name), false);
        assert!(dbs.len() < new_dbs.len());
        assert_eq!(new_dbs.contains(&new_database_name), true);
    }
}
