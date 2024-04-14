use crate::{
    errors::CustomError,
    models::{comment::Comment, user::GithubUserInfo},
    AppState,
};
use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use std::sync::Arc;

pub async fn get_comments_for_article(
    article_id: Path<(u32,)>,
    state: Data<Arc<AppState>>,
) -> Result<HttpResponse, CustomError> {
    let db_pool = &state.db_pool;

    let article_id = article_id.0;

    let commnets = sqlx::query!(
        "SELECT comments.user_id, comments.content, 
       comments.date, users.name, 
       users.avatar_url
       FROM comments JOIN users on users.id = comments.user_id
       WHERE comments.article = $1",
        article_id as i32
    )
    .fetch_all(db_pool)
    .await?
    .iter()
    .map(|i| Comment {
        id: None,
        user: Some(GithubUserInfo {
            id: i.user_id,
            login: i.name.as_ref().unwrap().clone(),
            avatar_url: i.avatar_url.as_ref().unwrap().clone(),
        }),
        content: i.content.clone(),
        date: Some(i.date),
        article: None,
    })
    .collect::<Vec<Comment>>();

    Ok(HttpResponse::Ok().json(commnets))
}
