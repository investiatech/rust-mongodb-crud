use crate::item::model::Item;
use mongodb::bson::oid::ObjectId;
use mongodb::{Collection, Database, bson::doc};

pub struct ItemRepository {
    collection: Collection<Item>,
}

impl ItemRepository {
    pub fn new(database: &Database) -> Self {
        Self {
            collection: database.collection("Items"),
        }
    }

    pub async fn create(&self, item: Item) -> mongodb::error::Result<Item> {
        let result = self.collection.insert_one(&item).await?;

        Ok(Item {
            id: result.inserted_id.as_object_id(),
            name: item.name,
            description: item.description,
        })
    }

    pub async fn find_all(&self) -> mongodb::error::Result<Vec<Item>> {
        let mut cursor = self.collection.find(doc! {}).await?;
        let mut items = Vec::new();

        while cursor.advance().await? {
            let item: Item = cursor.deserialize_current()?;
            items.push(item);
        }

        Ok(items)
    }

    pub async fn find_by_id(&self, id: ObjectId) -> mongodb::error::Result<Option<Item>> {
        self.collection.find_one(doc! {"_id": id}).await
    }

    pub async fn update(&self, id: ObjectId, item: Item) -> mongodb::error::Result<Option<Item>> {
        self.collection
            .find_one_and_update(
                doc! {"_id": id},
                doc! {
                    "$set": {
                    "name": item.name,
                        "description": item.description,
                        }
                },
            )
            .return_document(mongodb::options::ReturnDocument::After)
            .await
    }

    pub async fn delete(&self, id: ObjectId) -> mongodb::error::Result<bool> {
        let result = self.collection.delete_one(doc! {"_id": id}).await?;
        Ok(result.deleted_count > 0)
    }
}
