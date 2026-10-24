use crate::item::dto::UpdateItemRequest;
use crate::item::{dto::CreateItemRequest, model::Item, repository::ItemRepository};
use mongodb::bson::oid::ObjectId;

pub struct ItemService {
    repository: ItemRepository,
}

impl ItemService {
    pub fn new(repository: ItemRepository) -> Self {
        Self { repository }
    }

    pub async fn create(&self, request: CreateItemRequest) -> mongodb::error::Result<Item> {
        let item = Item {
            id: None,
            name: request.name,
            description: request.description,
        };

        self.repository.create(item).await
    }

    pub async fn find_all(&self) -> mongodb::error::Result<Vec<Item>> {
        self.repository.find_all().await
    }

    pub async fn find_by_id(&self, id: ObjectId) -> mongodb::error::Result<Option<Item>> {
        self.repository.find_by_id(id).await
    }

    pub async fn update(
        &self,
        id: ObjectId,
        request: UpdateItemRequest,
    ) -> mongodb::error::Result<bool> {
        let item = Item {
            id: Some(id),
            name: request.name,
            description: request.description,
        };

        self.repository.update(id, item).await
    }

    pub async fn delete(&self, id: ObjectId) -> mongodb::error::Result<bool> {
        self.repository.delete(id).await
    }
}
