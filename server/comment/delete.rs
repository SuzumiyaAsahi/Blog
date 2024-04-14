use std::sync::Arc;

use crate::{
    errors::CustomError,
    models::user::{Admin, User},
    AppState,
};
use actix_web::{web, HttpResponse};

pub async fn delete_comment(
    user: User,
    admin: Option<Admin>,
    comment_id: web::Path<(u32,)>,
    state: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, CustomError> {
    let db_pool = &state.db_pool;

    let comment_id = comment_id.0;
    let user_id = user.id;

    let is_admin = admin.is_some();

    let rows_affected = if is_admin {
        sqlx::query!("DELETE FROM comments WHERE id = $1", comment_id as i32)
            .execute(db_pool)
            .await?
    } else {
        sqlx::query!(
            "DELETE FROM comments WHERE id = $1 AND user_id = $2",
            comment_id as i32,
            user_id
        )
        .execute(db_pool)
        .await?
    }
    // 返回这次sql操作影响的行数
    .rows_affected();

    if rows_affected == 0 {
        Err(CustomError::NotFound(
            "删除评论失败，可能是提供的评论 ID 不正确或你没有权限删除这条评论".into(),
        ))
    } else {
        Ok(HttpResponse::Ok().json("删除评论成功！".to_string()))
    }
}
