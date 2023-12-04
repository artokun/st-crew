use axum::{
    body::{Body, Bytes, HttpBody},
    extract::Request,
    http::{header, HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Form, Json,
};
use mime::MimeIter;

use crate::data_format::DataFormat;

use super::BodyToEncode;

/// Transforms the respons into the correct format based on the `Accept` header
pub(crate) async fn transform_response(request: Request, next: Next) -> Response {
    let Some(body_format) = request
        .headers()
        .get_all(&header::ACCEPT)
        .into_iter()
        .filter_map(|header| header.to_str().ok())
        .find_map(|header| {
            let mut best_quality = (0.0, None);

            for mime in MimeIter::new(header).filter_map(Result::ok) {
                let format = match mime.essence_str() {
                    "*/*" | "application/*" | "application/json" => DataFormat::Json,
                    "application/msgpack" => DataFormat::MsgPack,
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
        })
    else {
        // We require the client to send an Accept header, so if it's missing we
        // need to return an error with no body.
        return StatusCode::NOT_ACCEPTABLE.into_response();
    };

    let mut response = next.run(request).await;

    if let Some(BodyToEncode(body)) = response.extensions_mut().remove::<BodyToEncode>() {
        let (mut parts, _) = response.into_parts();

        if parts.status == StatusCode::NOT_IMPLEMENTED {
            parts.status = StatusCode::OK;
        }

        parts.headers.remove(header::CONTENT_LENGTH);

        match body_format {
            DataFormat::Json => (parts, Json(body)).into_response(),

            DataFormat::MsgPack => {
                let bytes = match rmp_serde::encode::to_vec_named(&body) {
                    Ok(res) => res,

                    Err(err) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .header(header::CONTENT_TYPE, "text/plain")
                            .body(Body::new(err.to_string()))
                            .unwrap();
                    }
                };

                let mut res = bytes.into_response();

                res.headers_mut().insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("application/msgpack"),
                );

                res
            }

            DataFormat::Form => (parts, Form(body)).into_response(),
        }
    } else if response.body().size_hint().exact() != Some(0) {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(header::CONTENT_TYPE, "text/plain")
            .body(Bytes::from_static(b"response body was not encoded").into())
            .unwrap();
    } else {
        response
    }
}
