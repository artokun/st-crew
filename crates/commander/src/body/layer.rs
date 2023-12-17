use axum::{
    body::{Body, Bytes, HttpBody},
    extract::Request,
    http::{header, HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Form, Json,
};

use crate::data_format::DataFormat;

use super::BodyToEncode;

/// Transforms the respons into the correct format based on the `Accept` header
pub(crate) async fn transform_response(request: Request, next: Next) -> Response {
    let Some(body_format) = request
        .headers()
        .get_all(&header::ACCEPT)
        .into_iter()
        .filter_map(|header| header.to_str().ok())
        .find_map(DataFormat::parse_header)
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

            DataFormat::MsgPack { named } => {
                let result = if named {
                    rmp_serde::to_vec_named(&body)
                } else {
                    rmp_serde::to_vec(&body)
                };

                let bytes = match result {
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

            DataFormat::Cbor { packed } => {
                let result = if packed {
                    serde_cbor::ser::to_vec_packed(&body)
                } else {
                    serde_cbor::ser::to_vec(&body)
                };

                let bytes = match result {
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
                    HeaderValue::from_static("application/cbor"),
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
