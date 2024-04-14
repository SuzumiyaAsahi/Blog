use crate::errors::CustomError;
use crate::models::user::AccessToken;
use crate::models::user::{Login};
use crate::{models::user::GithubUserInfo, AppState};
use actix_web::{web, HttpResponse};

use cookie::time::Duration;
use reqwest::{Client};
use std::sync::Arc;
const CLIENT_ID: &str = "b6d90008eb475faeb20e";
const CLIENT_SECRET: &str = "463c4a259bbc01881a7c53607fd8748ee55d33e3";

pub async fn github_login(
    code: web::Json<Login>,
    state: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, CustomError> {
    let code = code.code.as_str();
    let client = Client::new();
    let access_token = client.post(format!(
            "https://github.com/login/oauth/access_token?client_id={CLIENT_ID}&client_secret={CLIENT_SECRET}&code={code}"
            ))
        .header("Accept", "application/json")
        .send()
        .await;

    let access_token = match access_token {
        Ok(r) => match r.json::<AccessToken>().await {
            Ok(r) => r.access_token,
            Err(_) => {
                return Err(CustomError::AuthFailed(
                    "code 是无效的 (可能已经过期), 请重新使用 Github 登录".into(),
                ));
            }
        },
        Err(_) => {
            return Err(CustomError::InternalServerError(
                "无法获取 access_token, 请重试".into(),
            ));
        }
    };

    let user_info = client
        .get("https://api.github.com/user")
        .bearer_auth(access_token.clone())
        .header("User-Agent", "SuzumiyaAsahi")
        .send()
        .await;

    let user_info = match user_info {
        Ok(r) => r.json::<GithubUserInfo>().await.unwrap(),
        Err(_) => {
            return Err(CustomError::InternalServerError(
                "无法获取 Github 用户信息，请重试".into(),
            ));
        }
    };

    let mut cookie = actix_web::cookie::Cookie::new("ACCESS_TOKEN", access_token);
    cookie.set_path("/");
    cookie.set_max_age(Duration::days(7));
    cookie.set_http_only(true);

    // 把用户信息存进数据库
    let db_pool = &state.db_pool;

    sqlx::query!(
        "INSERT INTO users (id , name, avatar_url) VALUES ($1, $2, $3) ON CONFLICT 
                 (id) DO UPDATE SET name = $2, avatar_url = $3",
        user_info.id,
        user_info.login,
        user_info.avatar_url
    )
    .execute(db_pool)
    .await?;

    let mut response = HttpResponse::Ok().json(format!("Hi, {}!", user_info.login));

    // 忽略错误
    let _ = response.add_cookie(&cookie);

    Ok(response)
}
