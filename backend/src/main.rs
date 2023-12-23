use backend::App;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().unwrap();
    App::default().serve().await.unwrap();
}
