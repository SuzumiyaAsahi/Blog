use crate::{
    errors::CustomError,
    models::{article::Article, user::Admin},
    AppState,
};
use actix_web::{web, HttpResponse};
use std::sync::Arc;

pub async fn edit_article(
    _: Admin,
    state: web::Data<Arc<AppState>>,
    article: web::Json<Article>,
) -> Result<HttpResponse, CustomError> {
    let db_pool = &state.db_pool;
    let id = match article.id {
        Some(id) => id,
        None => return Err(CustomError::BadRequest("请提供要修改的文章ID".into())),
    };
    sqlx::query!(
        "UPDATE articles SET title = $1, content = $2 WHERE id = $3",
        article.title,
        article.content,
        id as i32,
    )
    .execute(db_pool)
    .await?;
    Ok(HttpResponse::Ok().json("修改文章成功！".to_string()))
}
