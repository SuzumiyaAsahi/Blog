use std::sync::Arc;

use crate::{
    errors::CustomError,
    models::{comment::Comment, user::User},
    AppState,
};
use actix_web::{web, HttpResponse};

pub async fn new_comment(
    user: User,
    comment: web::Json<Comment>,
    state: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, CustomError> {
    let db_pool = &state.db_pool;

    let user_id = user.id;
    let article_id = match comment.article {
        Some(id) => id,
        None => return Err(CustomError::BadRequest("请提供要评论的文章 ID".into())),
    };

    if sqlx::query!("SELECT id FROM articles WHERE id = $1", article_id as i32)
        .fetch_optional(db_pool)
        .await?
        .is_none()
    {
        return Err(CustomError::BadRequest("要评论的文章不存在".into()));
    }

    sqlx::query!(
        "INSERT INTO comments (user_id, content, article) VALUES ($1, $2, $3)",
        user_id,
        comment.content,
        article_id as i32
    )
    .execute(db_pool)
    .await?;
    Ok(HttpResponse::Ok().json("新增评论成功！".to_string()))
}
