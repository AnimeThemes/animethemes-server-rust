use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn connect() -> DatabaseConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        let db_host = env::var("DB_HOST").expect("DB_HOST is required");

        let db_port = env::var("DB_PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(3306);

        let db_username = env::var("DB_USERNAME").expect("DB_USERNAME is required");

        let db_password = env::var("DB_PASSWORD").ok().filter(|p| !p.is_empty());

        let db_database = env::var("DB_DATABASE").expect("DB_DATABASE is required");

        let auth = match db_password {
            Some(password) => format!("{db_username}:{password}"),
            None => db_username,
        };

        format!("mysql://{auth}@{db_host}:{db_port}/{db_database}")
    });

    Database::connect(database_url).await.unwrap()
}
