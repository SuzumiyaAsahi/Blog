use crate::{
    errors::CustomError,
    models::article::{Article, ArticlePreview},
    AppState,
};
use actix_web::{
    web::{self, Path},
    HttpResponse,
};
use std::sync::Arc;

pub async fn get_articles_preview(
    state: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, CustomError> {
    let db_pool = &state.db_pool;
    let articles = sqlx::query!("SELECT id, title, date FROM articles")
        .fetch_all(db_pool)
        .await?
        .iter()
        .map(|i| ArticlePreview {
            id: Some(i.id),
            title: i.title.clone(),
            date: i.date,
        })
        .collect::<Vec<ArticlePreview>>();
    Ok(HttpResponse::Ok().json(articles))
}

pub async fn get_article(
    state: web::Data<Arc<AppState>>,
    id: Path<(u32,)>,
) -> Result<HttpResponse, CustomError> {
    let db_pool = &state.db_pool;
    let article = sqlx::query_as!(
        Article,
        "SELECT id, title, content, date FROM articles WHERE id = $1",
        id.0 as i32,
    )
    .fetch_one(db_pool)
    .await?;
    Ok(HttpResponse::Ok().json(article))
}
