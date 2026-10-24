use mongodb::bson::doc;
use mongodb::{Client, Database};

pub async fn connect(uri: &str, database_name: &str) -> mongodb::error::Result<Database> {
    let client = Client::with_uri_str(uri).await?;
    let database = client.database(database_name);

    database.run_command(doc! {"ping": 1}).await?;

    println!("Connected to database {} ", database_name);

    Ok(database)
}
