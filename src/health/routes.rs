use axum::Router;
use axum::routing::get;
use crate::state::AppState;
use crate::health::handler::health;

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health))
}