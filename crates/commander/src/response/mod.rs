use axum::http::StatusCode;
use serde::Serialize;
use utoipa::openapi::{ComponentsBuilder, ResponsesBuilder};

mod error;

pub use error::*;

pub trait ApiResponse: Serialize + Send + Sync + 'static {
    fn status(&self) -> StatusCode;

    fn apply_components(components: ComponentsBuilder) -> ComponentsBuilder;

    fn apply_responses(responses: ResponsesBuilder) -> ResponsesBuilder;
}
