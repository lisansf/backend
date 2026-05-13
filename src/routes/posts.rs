// routes/posts.rs

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use sqlx::PgPool;

use crate::models::posts::{
    CreatePost,
    Posts,
};

pub async fn getpost(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Posts>>, StatusCode> {

    let posts = sqlx::query_as::<_, Posts>(
        "
        SELECT
            id,
            title,
            content,
            tags,
            likes,
            views,
            user_id,
            created_at,
            updated_at
        FROM posts
        "
    )
    .persistent(false)
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(posts))
}

pub async fn sendpost(
    State(pool): State<PgPool>,
    Json(body): Json<CreatePost>,
) -> Result<Json<Posts>, StatusCode> {

    let send = sqlx::query_as::<_, Posts>(
        "
        INSERT INTO posts (
            user_id,
            title,
            content
        )

        VALUES ($1, $2, $3)

        RETURNING
            id,
            title,
            content,
            tags,
            likes,
            views,
            user_id,
            created_at,
            updated_at
        "
    )
    .persistent(false)
    .bind(body.user_id)
    .bind(body.title)
    .bind(body.content)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(send))
}