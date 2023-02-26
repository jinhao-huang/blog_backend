use dotenvy::{dotenv, var};

#[tokio::main]
async fn main() {
    dotenv().unwrap();
    let database_url = var("DATABASE_URL").unwrap();
    blog_backend::run(database_url).await;
}
