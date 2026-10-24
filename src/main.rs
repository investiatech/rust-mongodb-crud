#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    crud::run()
        .await
        .expect("Error running server");
}
