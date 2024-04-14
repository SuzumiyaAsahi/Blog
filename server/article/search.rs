use std::{sync::Arc};

use crate::{
    errors::CustomError,
    models::article::{ArticlePreview},
    AppState,
};
use actix_web::{web, HttpResponse};

pub async fn search_article(
    state: web::Data<Arc<AppState>>,
    keyword: web::Path<(String,)>,
) -> Result<HttpResponse, CustomError> {
    let db_pool = &state.db_pool;
    let result = sqlx::query!(
        "SELECT id, title, date 
        FROM articles 
        WHERE 
        title LIKE $1 OR content LIKE $1",
        format!("%{}%", keyword.0)
    )
    .fetch_all(db_pool)
    .await?
    .iter()
    .map(|i| ArticlePreview {
        id: Some(i.id),
        title: i.title.clone(),
        date: i.date,
    })
    .collect::<Vec<ArticlePreview>>();

    if result.is_empty() {
        return Err(CustomError::NotFound("找不到文章".into()));
    }

    Ok(HttpResponse::Ok().json(result))
}
