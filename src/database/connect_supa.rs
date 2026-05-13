use sqlx::{PgPool, postgres::{PgConnectOptions, PgPoolOptions}};
use std::str::FromStr;
use dotenv::dotenv;
use std::env;

pub async fn connect_db() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not found");

    println!("{:?}", db_url);

    let options = PgConnectOptions::from_str(&db_url)?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    Ok(pool)
}