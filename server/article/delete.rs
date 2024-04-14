
use crate::{errors::CustomError, models::user::Admin, AppState};
use actix_web::{web, HttpResponse};
use std::sync::Arc;

pub async fn delete_article(
    _: Admin,
    state: web::Data<Arc<AppState>>,
    id: web::Path<(u32,)>,
) -> Result<HttpResponse, CustomError> {
    let db_pool = &state.db_pool;
    sqlx::query!("DELETE FROM articles where id = $1", id.0 as i32)
        .execute(db_pool)
        .await?;

    Ok(HttpResponse::Ok().json("删除文章成功!".to_string()))
}
