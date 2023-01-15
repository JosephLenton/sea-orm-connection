use ::anyhow::Result;
use ::names::Generator;
use ::sea_orm::DatabaseConnection;

const TEST_DATABASE_BASENAME: &'static str = "test-database";

use crate::queries::query_create_database;

/// Runs a query which will create a new database with a randomly generated name.
///
/// After the Database is created, it will then run another query to
/// switch the `DatabaseConnection` to use that new database.
pub async fn query_create_random_database(db_connection: &DatabaseConnection) -> Result<String> {
    let db_name = random_db_name();

    query_create_database(&db_connection, &db_name).await?;

    Ok(db_name)
}

fn random_db_name() -> String {
    let mut generator = Generator::default();
    let db_name = generator
        .next()
        .expect("Expect generating a name should always succeed");

    format!("{}--{}", TEST_DATABASE_BASENAME, db_name,)
}
