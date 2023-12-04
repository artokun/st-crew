use axum::{
    body::Bytes,
    extract::{FromRequest, Request},
    http::{header::CONTENT_TYPE, StatusCode},
    Form, RequestExt,
};
use serde::de::DeserializeOwned;

use crate::response::ApiError;

pub struct ParseBody<T>(pub T);

#[axum::async_trait]
impl<T, S> FromRequest<S> for ParseBody<T>
where
    T: DeserializeOwned + 'static,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        if let Some(content_mime) = req
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<mime::Mime>().ok())
        {
            if content_mime == mime::APPLICATION_JSON {
                let bytes = Bytes::from_request(req, state).await?;

                let deserializer = &mut serde_json::Deserializer::from_slice(&bytes);

                Ok(Self(serde_path_to_error::deserialize::<_, T>(
                    deserializer,
                )?))
            } else if content_mime == mime::APPLICATION_MSGPACK {
                let bytes = Bytes::from_request(req, state).await?;

                let deserializer = &mut rmp_serde::Deserializer::from_read_ref(&bytes);

                Ok(Self(serde_path_to_error::deserialize::<_, T>(
                    deserializer,
                )?))
            } else if content_mime == mime::APPLICATION_WWW_FORM_URLENCODED {
                let Form(payload) = req.extract().await.map_err(|err| {
                    ApiError::new(StatusCode::UNPROCESSABLE_ENTITY)
                        .with_name("invalid_body")
                        .with_error(&err)
                })?;

                Ok(Self(payload))
            } else {
                Err(ApiError::new(StatusCode::UNSUPPORTED_MEDIA_TYPE)
                    .with_name("unsuppoprted_content_type")
                    .with_message("The given content type is not supported."))
            }
        } else {
            Err(ApiError::new(StatusCode::UNSUPPORTED_MEDIA_TYPE)
                .with_name("invalid_content_type")
                .with_message("Content type was missing or invalid."))
        }
    }
}
