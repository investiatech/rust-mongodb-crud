use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use mongodb::bson::doc;
use serde::Serialize;
use crate::state::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub database: &'static str,
}

pub async fn health(state: State<AppState>) -> (StatusCode, Json<HealthResponse>) {
    match state.db.run_command(doc! { "ping": 1 }).await {
        Ok(_) => (StatusCode::OK, Json(HealthResponse {status: "ok", database: "connected"})),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(HealthResponse {status: "error", database: "disconnect"})),
    }
}