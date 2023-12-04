use std::sync::Arc;

pub mod layer;
mod parse_body;
mod response_body;
mod stream_format;

pub use parse_body::*;
pub use response_body::*;
pub use stream_format::*;

#[derive(Clone)]
struct BodyToEncode(Arc<dyn erased_serde::Serialize + Send + Sync>);
