use axum::{routing::get, Router};
use sea_orm::Database;

pub async fn run(database_url: String) {
    connect_database(database_url).await;

    let app = Router::new().route("/", get(|| async { "Hi, friend!" }));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn connect_database(database_url: String) {
    Database::connect(database_url).await.unwrap();
}
