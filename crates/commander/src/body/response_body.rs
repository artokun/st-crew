use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use super::BodyToEncode;

pub struct EncodeBody<T>(pub T)
where
    T: Serialize + Send + Sync + 'static;

impl<T> IntoResponse for EncodeBody<T>
where
    T: Serialize + Send + Sync + 'static,
{
    fn into_response(self) -> Response {
        let mut response = StatusCode::NOT_IMPLEMENTED.into_response();

        response
            .extensions_mut()
            .insert(BodyToEncode(Arc::new(self.0)));

        response
    }
}
