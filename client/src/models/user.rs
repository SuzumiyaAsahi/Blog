use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub avatat_url: String,
    pub is_admin: bool,
}

/// 用于 OAuth 登录时从路径中提取 query 参数和向后端发送请求
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Login {
    pub code: String,
}
