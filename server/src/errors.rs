use actix_web::{body::MessageBody, error, http::StatusCode, HttpResponse};
use std::fmt;

#[derive(Debug, Clone)]
pub enum CustomError {
    NotFound(String),
    InternalServerError(String),
    BadRequest(String),
    AuthFailed(String),
}

impl error::ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::AuthFailed(_) => StatusCode::UNAUTHORIZED,
        }
    }
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::new(self.status_code()).set_body(
            match self {
                Self::NotFound(e) => e,
                Self::InternalServerError(e) => e,
                Self::BadRequest(e) => e,
                Self::AuthFailed(e) => e,
            }
            .clone()
            .boxed(),
        )
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::InternalServerError(e) => write!(f, "{e}"),
            CustomError::NotFound(e) => write!(f, "{e}"),
            CustomError::BadRequest(e) => write!(f, "{e}"),
            CustomError::AuthFailed(e) => write!(f, "{e}"),
        }
    }
}

impl From<sqlx::Error> for CustomError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::NotFound("找不到对应的数据".into()),
            _ => Self::InternalServerError("服务器发生内部错误，请联系网站管理员".into()),
        }
    }
}
