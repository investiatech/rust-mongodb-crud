use crate::item::dto::UpdateItemRequest;
use crate::item::{
    dto::{CreateItemRequest, ItemResponse},
    model::Item,
    repository::ItemRepository,
    service::ItemService,
};
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use mongodb::bson::oid::ObjectId;

fn to_response(item: Item) -> Result<ItemResponse, StatusCode> {
    let id = item
        .id
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    Ok(ItemResponse {
        id,
        name: item.name,
        description: item.description,
    })
}

pub async fn create_item(
    State(state): State<AppState>,
    Json(request): Json<CreateItemRequest>,
) -> Result<(StatusCode, Json<ItemResponse>), StatusCode> {
    let service = ItemService::new(ItemRepository::new(&state.db));

    let item = service
        .create(request)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(to_response(item)?)))
}

pub async fn get_items(state: State<AppState>) -> Result<Json<Vec<ItemResponse>>, StatusCode> {
    let service = ItemService::new(ItemRepository::new(&state.db));

    let items = service
        .find_all()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = items
        .into_iter()
        .map(to_response)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Json(response))
}

pub async fn get_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ItemResponse>, StatusCode> {
    let id = ObjectId::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let service = ItemService::new(ItemRepository::new(&state.db));

    let item = service
        .find_by_id(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(to_response(item)?))
}

pub async fn update_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<UpdateItemRequest>,
) -> Result<Json<ItemResponse>, StatusCode> {
    let id = ObjectId::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let service = ItemService::new(ItemRepository::new(&state.db));

    let updated_item = service
        .update(id, request)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !updated_item {
        return Err(StatusCode::NOT_FOUND);
    }

    let item = service
        .find_by_id(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(to_response(item)?))
}

pub async fn delete_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let id = ObjectId::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let service = ItemService::new(ItemRepository::new(&state.db));

    let deleted_item = service
        .delete(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if deleted_item {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
