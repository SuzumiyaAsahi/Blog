use gloo::net::http::Request;
use serde::Deserialize;

pub async fn fetch<T: for<'a> Deserialize<'a>>(url: String) -> Result<T, String> {
    let resp = Request::get(&url).send().await;
    match resp {
        Ok(r) if r.status().to_string().starts_with('2') => match r.json::<T>().await {
            Ok(r) => Ok(r),
            Err(e) => Err(format!("无法解析响应: {e}")),
        },
        Ok(r) => Err(format!(
            "{} ({} {})",
            r.text().await.unwrap(),
            r.status(),
            r.status_text()
        )),
        Err(e) => Err(format!("无法发送请求: {e}")),
    }
}
