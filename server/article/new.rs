use crate::models::user::{Admin};
use crate::{errors::CustomError, models::article::Article, AppState};
use actix_web::{web, HttpResponse};
use std::sync::{Arc};

pub async fn new_article(
    _: Admin,
    state: web::Data<Arc<AppState>>,
    article: web::Json<Article>,
) -> Result<HttpResponse, CustomError> {
    let db_pool = &state.db_pool;

    sqlx::query!(
        "INSERT INTO articles (title, content) VALUES ($1, $2)",
        article.title,
        article.content,
    )
    .execute(db_pool)
    .await?;

    Ok(HttpResponse::Ok().body("新增文章成功！"))
}
