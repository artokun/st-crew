use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::body::EncodeBody;

mod error;

pub use error::*;

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

pub struct ApiResponse<T> {
    pub status: StatusCode,
    pub body: T,
}

impl ApiResponse<()> {
    #[must_use]
    pub const fn new(status: StatusCode) -> Self {
        Self { status, body: () }
    }

    pub const fn with_body<T>(self, body: T) -> ApiResponse<T> {
        ApiResponse {
            status: self.status,
            body,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize + Send + Sync + 'static,
{
    fn into_response(self) -> Response {
        (self.status, EncodeBody(self.body)).into_response()
    }
}
