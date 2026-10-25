use crate::item::handler::{create_item, delete_item, get_item, get_items, update_item};
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/items", get(get_items).post(create_item))
        .route("/items/{id}", get(get_item).put(update_item).delete(delete_item))
}
