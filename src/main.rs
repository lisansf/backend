use axum::{
    routing::{get, post},
    Router,
};

use tokio::net::TcpListener;

// Folder Lokal
mod routes;
mod database;
mod models;

use database::connect_supa::connect_db;

use routes::{
    posts::{getpost, sendpost},
    users::{getusers, senduser},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let pool = connect_db().await?;

    let app = Router::new()
        .route("/register", post(senduser))
        .route("/users", get(getusers))
        .route("/posts", get(getpost).post(sendpost))
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
