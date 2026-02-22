use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: u32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}
impl<T> ApiResponse<T> {
    pub fn ok<M: AsRef<str>>(message: M, data: Option<T>) -> Self {
        Self {
            code: 0,
            message: String::from(message.as_ref()),
            data,
        }
    }
    pub fn err<M: AsRef<str>>(message: M) -> Self {
        Self {
            code: 1,
            message: String::from(message.as_ref()),
            data: None,
        }
    }
}
impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}