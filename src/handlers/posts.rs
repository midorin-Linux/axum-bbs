use crate::{
    AppState,
    models::post::{Post, CreatePostRequest}
};

use anyhow::{Context, Result};
use axum::{
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    middleware as axum_middleware,
    response::{Html, IntoResponse, Json, Response},
    routing::{get, post, put, delete},
    Router,
};

pub async fn get_posts(State(state): State<AppState>) -> Result<Json<Vec<Post>>, StatusCode> {
    let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts")
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(posts))
}

pub async fn create_post(State(state): State<AppState>, Json(payload): Json<CreatePostRequest>) -> Result<Json<Post>, StatusCode> {
    let create_post = sqlx::query_as::<_, Post>(
        "INSERT INTO posts (title, content, created_at) VALUES ($1, $2, datetime('now')) RETURNING *"
    )
        .bind(&payload.title)
        .bind(&payload.content)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(create_post))
}