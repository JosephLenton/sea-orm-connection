use crate::queries::query_create_database_from_template;
use anyhow::Result;
use names::Generator;
use names::Name;
use sea_orm::DatabaseConnection;
use std::fmt::Display;

const TEST_DATABASE_BASENAME: &'static str = "test-database";

/// Runs a query which will create a new database with a randomly generated name.
/// The name of this new database will be returned.
pub async fn query_create_random_database_from_template<T>(
    db_connection: &DatabaseConnection,
    template_name: &T,
) -> Result<String>
where
    T: Display,
{
    let db_name = random_db_name();

    query_create_database_from_template(&db_connection, &db_name, template_name).await?;

    Ok(db_name)
}

fn random_db_name() -> String {
    let mut generator = Generator::with_naming(Name::Numbered);
    let db_name = generator
        .next()
        .expect("Expect generating a name should always succeed");

    format!("{}--{}", TEST_DATABASE_BASENAME, db_name,)
}
