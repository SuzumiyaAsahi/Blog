use crate::{
    errors::CustomError,
    models::user::{User, UserInfo},
    AppState,
};
use actix_web::{web, HttpResponse};
use std::sync::Arc;

pub async fn get_user_info(
    user: User,
    state: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, CustomError> {
    let db_pool = &state.db_pool;
    let user_id = user.id;
    let user_info = sqlx::query!("SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(db_pool)
        .await?;

    Ok(HttpResponse::Ok().json(UserInfo {
        id: user_info.id,
        login: user_info.name.map_or(String::new(), |name| name),
        avatar_url: user_info
            .avatar_url
            .map_or(String::new(), |avatar_url| avatar_url),
        is_admin: user_info.id == 108174641,
    }))
}
