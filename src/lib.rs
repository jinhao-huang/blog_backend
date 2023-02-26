mod database;
use crate::database::posts::Entity as Post;
use axum::{extract::Path, http::StatusCode, routing::get, Extension, Json, Router};
use sea_orm::{prelude::DateTime, Database, DatabaseConnection, EntityTrait};
use serde::Serialize;

#[derive(Serialize)]
pub struct PostInfo {
    title: String,
    date: DateTime,
    content: String,
}

pub async fn run(database_url: String) {
    let database = connect_database(database_url).await;

    let app = Router::new()
        .route("/", get(|| async { "Hi, friend!" }))
        .route("/post/:post_id", get(get_post))
        .layer(Extension(database));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn connect_database(database_url: String) -> DatabaseConnection {
    Database::connect(database_url).await.unwrap()
}

async fn get_post(
    Path(post_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<PostInfo>, StatusCode> {
    let post = Post::find_by_id(post_id).one(&database).await.unwrap();

    if let Some(post) = post {
        Ok(Json(PostInfo {
            title: post.title,
            date: post.date.naive_utc(),
            content: post.content,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
