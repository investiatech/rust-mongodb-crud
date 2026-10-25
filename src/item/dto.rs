use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub description: Option<String>,
}
#[derive(Deserialize, Serialize)]
pub struct UpdateItemRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ItemResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}
