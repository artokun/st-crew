use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
};
use mime::MimeIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFormat {
    Json,
    MsgPack { named: bool },
    Form,
}

impl DataFormat {
    pub fn deserialize<T>(&self, data: &[u8]) -> Result<T, Box<dyn std::error::Error + Send + Sync>>
    where
        T: serde::de::DeserializeOwned,
    {
        Ok(match self {
            DataFormat::Json => serde_json::from_slice(data)?,
            DataFormat::MsgPack { .. } => rmp_serde::from_slice(data)?,
            DataFormat::Form => serde_urlencoded::from_bytes(data)?,
        })
    }

    pub fn serialize<T>(
        &self,
        data: &T,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>
    where
        T: serde::Serialize,
    {
        Ok(match self {
            DataFormat::Json => serde_json::to_vec(data)?,

            DataFormat::MsgPack { named } => {
                if *named {
                    rmp_serde::to_vec_named(data)?
                } else {
                    rmp_serde::to_vec(data)?
                }
            }

            DataFormat::Form => serde_urlencoded::to_string(data)?.into_bytes(),
        })
    }

    pub fn parse_header(header: &str) -> Option<Self> {
        let mut best_quality = (0.0, None);

        for mime in MimeIter::new(header).filter_map(Result::ok) {
            let format = match mime.essence_str() {
                "*/*" | "application/*" | "application/json" => DataFormat::Json,

                "application/msgpack" => {
                    let named = mime
                        .get_param("named")
                        .and_then(|quality| quality.as_str().parse::<bool>().ok())
                        .unwrap_or(true);

                    DataFormat::MsgPack { named }
                }

                "application/x-www-form-urlencoded" => DataFormat::Form,
                _ => continue,
            };

            let Some(quality) = mime
                .get_param("q")
                .and_then(|quality| quality.as_str().parse::<f32>().ok())
            else {
                return Some(format);
            };

            if quality > best_quality.0 {
                best_quality = (quality, Some(format));
            }
        }

        best_quality.1
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for DataFormat
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(body_format) = parts
            .headers
            .get_all(&header::ACCEPT)
            .into_iter()
            .filter_map(|header| header.to_str().ok())
            .find_map(DataFormat::parse_header)
        {
            Ok(body_format)
        } else {
            Err(StatusCode::NOT_ACCEPTABLE)
        }
    }
}
