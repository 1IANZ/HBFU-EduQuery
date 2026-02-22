use crate::app::response::ApiResponse;
use crate::jwxt::error::SessionError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type ApiResult<T> = Result<T, ApiError>;
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,
    #[error("Method Not Allowed")]
    MethodNotAllowed,
    #[error("学号或密码错误")]
    InvalidCredentials,
    #[error("教务系统连接失败")]
    RemoteSystemUnavailable,
    #[error("请求超时")]
    RequestTimeout,
    #[error("数据解析失败")]
    DataParseError,
    #[error("请求参数无效: {0}")]
    BadRequest(String),
    #[error("未知内部错误")]
    InternalError,
    #[error("{0}")]
    Custom(String),
}
impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        use ApiError::*;
        match self {
            InvalidCredentials => StatusCode::UNAUTHORIZED,
            RemoteSystemUnavailable => StatusCode::BAD_GATEWAY,
            RequestTimeout => StatusCode::GATEWAY_TIMEOUT,
            DataParseError | InternalError | Custom(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BadRequest(_) => StatusCode::BAD_REQUEST,
            NotFound => StatusCode::NOT_FOUND,
            MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
        }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let body = axum::Json(ApiResponse::<()>::err(self.to_string()));
        (status_code, body).into_response()
    }
}
impl From<ApiError> for Response {
    fn from(err: ApiError) -> Self {
        err.into_response()
    }
}
impl From<SessionError> for ApiError {
    fn from(err: SessionError) -> Self {
        match err {
            SessionError::HttpError(_) => ApiError::RemoteSystemUnavailable,
            SessionError::RegexError(_) | SessionError::PatternNotFound => ApiError::DataParseError,
            SessionError::Custom(msg) => ApiError::BadRequest(msg),
            SessionError::AuthFailed(_) => ApiError::InvalidCredentials,
        }
    }
}
