use crate::{errors::CustomError, AppState};
use actix_web::web::Data;
use actix_web::FromRequest;
use actix_web::HttpRequest;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;

use std::sync::Arc;

// 前端 Github 授权登录后传上来的 code
#[derive(Debug, Clone, Deserialize)]
pub struct Login {
    pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
}

// Github 返回的用户信息，其实是有很多的，不过我们只用一些就行。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GithubUserInfo {
    // Github 用户 ID
    pub id: i32,
    // 用户名 (不是昵称)
    pub login: String,
    // 用户头像的地址
    pub avatar_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub login: String,
    pub avatar_url: String,
    pub is_admin: bool,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
}

#[derive(Debug, Clone)]
pub struct Admin {
    pub id: i32,
}

impl FromRequest for User {
    type Error = CustomError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let db_pool = Arc::clone(req.app_data::<Data<Arc<AppState>>>().unwrap())
            .db_pool
            .clone();

        let access_token = req.cookie("ACCESS_TOKEN");

        let fut = async move {
            let access_token = match access_token {
                Some(c) => c,
                None => return Err(CustomError::AuthFailed("你还没有登录".into())),
            };

            let user_id = match get_user_id(&access_token).await {
                Ok(id) => id,
                Err(e) => {
                    return Err(e);
                }
            };

            if sqlx::query!("SELECT id FROM users WHERE id = $1", user_id)
                .fetch_optional(&db_pool)
                .await
                .unwrap()
                .is_none()
            {
                return Err(CustomError::AuthFailed(
                    "你还没有在本站使用 Github 登陆过，请登录".into(),
                ));
            }

            Ok(Self { id: user_id })
        };

        Box::pin(fut)
    }
}

async fn get_user_id(access_token: &actix_web::cookie::Cookie<'_>) -> Result<i32, CustomError> {
    let client = Client::new();

    let user_info = client
        .get("https://api.github.com/user")
        .bearer_auth(access_token.value())
        .header("User-Agent", "SuzumiyaAsahi")
        .send()
        .await;

    let user_id = match user_info {
        Ok(r) => {
            match r.json::<GithubUserInfo>().await {
                Ok(i) => i.id,
                Err(_) => {
                    // 无法解析，可能是 Github 返回了错误消息
                    return Err(CustomError::BadRequest(
                        "无法获取 Github 用户信息，可能是提供了不正确的 access_token， 请重新登录"
                            .into(),
                    ));
                }
            }
        }
        Err(_) => {
            return Err(CustomError::InternalServerError(
                "无法获取 Github 用户信息，请重试".into(),
            ));
        }
    };
    Ok(user_id)
}

impl FromRequest for Admin {
    type Error = CustomError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let db_pool = Arc::clone(req.app_data::<Data<Arc<AppState>>>().unwrap())
            .db_pool
            .clone();

        let access_token = req.cookie("ACCESS_TOKEN");

        let fut = async move {
            let access_token = match access_token {
                Some(c) => c,
                None => return Err(CustomError::AuthFailed("你还没有登录".into())),
            };

            let user_id = match get_user_id(&access_token).await {
                Ok(id) => id,
                Err(e) => {
                    return Err(e);
                }
            };

            if sqlx::query!("SELECT id FROM users WHERE id = $1", user_id)
                .fetch_optional(&db_pool)
                .await
                .unwrap()
                .is_some()
            {
                if user_id != 108174641 {
                    return Err(CustomError::AuthFailed(
                        "你不是管理员，无权执行该操作".into(),
                    ));
                }
            } else {
                return Err(CustomError::AuthFailed(
                    "你还没有在本站使用 Github 登陆过，请登录".into(),
                ));
            }

            Ok(Self { id: user_id })
        };
        Box::pin(fut)
    }
}
