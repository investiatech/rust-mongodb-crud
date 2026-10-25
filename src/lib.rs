pub mod config;
pub mod database;
pub mod health;
pub mod item;
pub mod state;

use crate::state::AppState;
use axum::Router;
use config::Config;
use database::connection;

fn app_router(state: AppState) -> Router {
    Router::new()
        .merge(item::routes::router())
        .merge(health::routes::router())
        .with_state(state)
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    let db = connection::connect(&config.mongodb_uri, &config.mongodb_database).await?;
    let state = AppState { db };

    let app: Router = app_router(state);
    let listener = tokio::net::TcpListener::bind(config.server_address).await?;

    println!("Listening on http://0.0.0.0:8000");
    axum::serve(listener, app).await?;

    Ok(())
}
