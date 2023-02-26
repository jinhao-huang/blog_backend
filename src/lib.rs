use axum::{
    routing::get,
    Router,
};

pub async fn run() {
    let app = Router::new().route("/", get(|| async { "Hi, friend!" }));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}