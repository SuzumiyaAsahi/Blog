use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ArticlePreview {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub date: Option<chrono::NaiveDate>,
}
