use std::{backtrace::Backtrace, borrow::Cow, fmt::Display, sync::Arc};

use axum::{
    extract::rejection::{BytesRejection, FailedToBufferBody},
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use bevy::log;
use serde::Serialize;
use tokio::task::JoinError;
use utoipa::ToSchema;

use crate::body::EncodeBody;

pub struct ApiErrorBuilder<const WITH_NAME: bool> {
    status: StatusCode,

    /// A human readable name which should be unique for this *class* of error.
    name: Option<&'static str>,
}

impl ApiErrorBuilder<false> {
    #[must_use]
    pub const fn with_name(self, name: &'static str) -> ApiErrorBuilder<true> {
        ApiErrorBuilder {
            status: self.status,
            name: Some(name),
        }
    }
}

impl ApiErrorBuilder<true> {
    #[must_use]
    pub fn with_error<E>(self, err: E) -> ApiError
    where
        E: Display,
    {
        let mut message = format!("{err}");

        // Capitalize the first letter of the message.
        if let Some(first_char) = message.get_mut(..1) {
            first_char.make_ascii_uppercase();
        }

        // Add a period to the end of the message if it doesn't already have punctuation.
        if !message.ends_with(|c: char| c.is_ascii_punctuation()) {
            message.push('.');
        }

        self.with_message(message)
    }

    #[must_use]
    pub fn with_message(self, message: impl Into<Arc<str>>) -> ApiError {
        ApiError {
            status: self.status,

            // This unwrap is safe since `WithName` is only accessible if `name` is `Some`.
            error: Cow::Borrowed(
                self.name
                    .expect("name must exist before with_message can be called"),
            ),

            message: message.into(),

            context: (),

            headers: None,
            backtrace: None,
        }
    }
}

#[derive(ToSchema, Serialize)]
pub struct ApiError<Context = ()> {
    #[serde(skip)]
    pub status: StatusCode,

    #[schema(example = "not_found")]
    pub error: Cow<'static, str>,

    #[schema(example = "The requested resource was not found.")]
    pub message: Arc<str>,

    #[serde(skip)]
    pub context: Context,

    #[serde(skip)]
    headers: Option<HeaderMap<HeaderValue>>,

    #[serde(skip)]
    backtrace: Option<Backtrace>,
}

impl std::fmt::Debug for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiError")
            .field("status", &self.status)
            .field("error", &self.error)
            .field("message", &self.message)
            .field("context", &serde_json::to_string(&self.context).ok())
            .field("headers", &self.headers)
            .field("backtrace", &self.backtrace)
            .finish()
    }
}

impl ApiError {
    #[must_use]
    #[allow(clippy::new_ret_no_self)]
    pub const fn new(status: StatusCode) -> ApiErrorBuilder<false> {
        ApiErrorBuilder { status, name: None }
    }

    #[must_use]
    pub fn not_found(name: &'static str) -> Self {
        Self::new(StatusCode::NOT_FOUND)
            .with_name("not_found")
            .with_message(format!("The requested {name} was not found."))
    }

    #[must_use]
    pub fn not_implemented() -> Self {
        Self::new(StatusCode::NOT_IMPLEMENTED)
            .with_name("not_implemeneted")
            .with_message("This endpoint is not yet implemented.")
    }
    #[must_use]
    pub fn forbidden() -> Self {
        Self::new(StatusCode::FORBIDDEN)
            .with_name("invalid_permissions")
            .with_message("You do not have the required permissions to access this resource.")
    }

    #[must_use]
    pub fn bad_gateway<E>(err: E) -> Self
    where
        E: Display,
    {
        #[cfg(debug_assertions)]
        return Self::new(StatusCode::BAD_GATEWAY)
            .with_name("bad_gateway")
            .with_error(err);

        #[cfg(not(debug_assertions))]
        return Self::new(StatusCode::BAD_GATEWAY)
            .with_name("bad_gateway")
            .with_message(
            "Something went wrong. Please try again. If the issue persists, please let us know!",
        );
    }

    /// Should only be used for legitimate, unrecoverable internal server errors.
    #[must_use]
    pub fn internal_server_error<E>(err: E) -> Self
    where
        E: Display,
    {
        #[cfg(debug_assertions)]
        return Self::new(StatusCode::INTERNAL_SERVER_ERROR)
            .with_name("internal_server_error")
            .with_error(err)
            .with_backtrace();

        #[cfg(not(debug_assertions))]
        return Self::new(StatusCode::INTERNAL_SERVER_ERROR)
            .with_name("internal_server_error")
            .with_message("Something went wrong. Please try again. If the issue persists, please let us know!")
            .with_backtrace();
    }
}

impl ApiError {
    #[must_use]
    pub fn with_backtrace(self) -> Self {
        Self {
            status: self.status,

            error: self.error,
            message: self.message,

            context: self.context,

            headers: self.headers,

            backtrace: Some(Backtrace::capture()),
        }
    }
}

impl ApiError {
    #[must_use]
    pub fn with_context<C>(self, context: C) -> ApiError<C> {
        ApiError::<C> {
            status: self.status,

            error: self.error,
            message: self.message,

            context,

            headers: self.headers,

            backtrace: self.backtrace,
        }
    }

    #[must_use]
    pub fn with_headers<I: IntoIterator<Item = (Option<HeaderName>, HeaderValue)>>(
        mut self,
        iter: I,
    ) -> Self {
        self.headers.get_or_insert_with(HeaderMap::new).extend(iter);

        self
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.error)?;
        f.write_str(": ")?;
        f.write_str(&self.message)
    }
}

impl std::error::Error for ApiError {}

impl IntoResponse for ApiError {
    fn into_response(mut self) -> Response {
        #[derive(Serialize)]
        pub struct ApiErrorResponse<Context = ()> {
            pub error: Cow<'static, str>,
            pub message: Arc<str>,

            pub context: Context,
        }

        assert!(
            self.status.is_client_error() || self.status.is_server_error(),
            "expected api error status"
        );

        if self.status.is_server_error() {
            log::error!(status = self.status.as_u16(), "{}", self);

            if let Some(backtrace) = &self.backtrace {
                log::error!("{}", backtrace);
            }
        }

        let headers = self.headers.take().unwrap_or_default();

        let mut response = (
            self.status,
            EncodeBody(ApiErrorResponse {
                error: self.error,
                message: self.message,

                context: self.context,
            }),
        )
            .into_response();

        response.headers_mut().extend(headers);

        response
    }
}

impl From<JoinError> for ApiError {
    fn from(err: JoinError) -> Self {
        Self::internal_server_error(err)
    }
}

impl From<BytesRejection> for ApiError {
    fn from(err: BytesRejection) -> Self {
        match err {
            BytesRejection::FailedToBufferBody(FailedToBufferBody::LengthLimitError(err)) => {
                Self::new(err.status())
                    .with_name("body_too_large")
                    .with_error(&err)
            }

            BytesRejection::FailedToBufferBody(FailedToBufferBody::UnknownBodyError(err)) => {
                Self::new(err.status())
                    .with_name("budy_buffer_failed")
                    .with_error(&err)
            }

            _ => Self::internal_server_error(err),
        }
    }
}

impl From<serde_path_to_error::Error<serde_json::Error>> for ApiError {
    fn from(err: serde_path_to_error::Error<serde_json::Error>) -> Self {
        match err.inner().classify() {
            serde_json::error::Category::Io
            | serde_json::error::Category::Syntax
            | serde_json::error::Category::Eof => Self::new(StatusCode::BAD_REQUEST)
                .with_name("invalid_json")
                .with_error(&err),

            serde_json::error::Category::Data => Self::new(StatusCode::UNPROCESSABLE_ENTITY)
                .with_name("invalid_body")
                .with_error(&err),
        }
    }
}

impl From<serde_path_to_error::Error<rmp_serde::decode::Error>> for ApiError {
    fn from(err: serde_path_to_error::Error<rmp_serde::decode::Error>) -> Self {
        match err.inner() {
            err @ (rmp_serde::decode::Error::InvalidMarkerRead(_)
            | rmp_serde::decode::Error::InvalidDataRead(_)
            | rmp_serde::decode::Error::Utf8Error(_)
            | rmp_serde::decode::Error::DepthLimitExceeded) => Self::new(StatusCode::BAD_REQUEST)
                .with_name("invalid_msgpack")
                .with_error(err),

            err @ (rmp_serde::decode::Error::TypeMismatch(_)
            | rmp_serde::decode::Error::OutOfRange
            | rmp_serde::decode::Error::LengthMismatch(_)
            | rmp_serde::decode::Error::Uncategorized(_)
            | rmp_serde::decode::Error::Syntax(_)) => Self::new(StatusCode::UNPROCESSABLE_ENTITY)
                .with_name("invalid_body")
                .with_error(err),
        }
    }
}

// impl From<jsonwebtoken::errors::Error> for ApiError {
//     fn from(err: jsonwebtoken::errors::Error) -> Self {
//         match err.kind() {
//             jsonwebtoken::errors::ErrorKind::InvalidToken => Self::new(StatusCode::BAD_REQUEST)
//                 .with_name("invalid_token")
//                 .with_message("The given token was invalid."),

//             jsonwebtoken::errors::ErrorKind::InvalidSignature => {
//                 Self::new(StatusCode::UNAUTHORIZED)
//                     .with_name("invalid_token")
//                     .with_message("The given token signature was invalid.")
//             }

//             jsonwebtoken::errors::ErrorKind::InvalidEcdsaKey => {
//                 Self::new(StatusCode::INTERNAL_SERVER_ERROR)
//                     .with_name("invalid_ecsda_key")
//                     .with_message("The given ECDSA key was invalid.")
//             }

//             jsonwebtoken::errors::ErrorKind::InvalidRsaKey(err) => {
//                 Self::new(StatusCode::INTERNAL_SERVER_ERROR)
//                     .with_name("invalid_rsa_key")
//                     .with_error(err)
//             }

//             jsonwebtoken::errors::ErrorKind::RsaFailedSigning => {
//                 Self::new(StatusCode::INTERNAL_SERVER_ERROR)
//                     .with_name("token_signing_failed")
//                     .with_message("Failed to sign with the given RSA key.")
//             }

//             jsonwebtoken::errors::ErrorKind::InvalidAlgorithmName
//             | jsonwebtoken::errors::ErrorKind::InvalidAlgorithm => {
//                 Self::new(StatusCode::INTERNAL_SERVER_ERROR)
//                     .with_name("token_algorithm_invalid")
//                     .with_message("The given algorithm was invalid.")
//             }

//             jsonwebtoken::errors::ErrorKind::InvalidKeyFormat => {
//                 Self::new(StatusCode::INTERNAL_SERVER_ERROR)
//                     .with_name("invalid_key_format")
//                     .with_message("The given key format was invalid.")
//             }

//             jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
//                 Self::new(StatusCode::UNAUTHORIZED)
//                     .with_name("token_expired")
//                     .with_message("The given token has expired.")
//             }

//             jsonwebtoken::errors::ErrorKind::InvalidIssuer => Self::new(StatusCode::FORBIDDEN)
//                 .with_name("issuer_invalid")
//                 .with_message("The given token issuer was invalid."),

//             jsonwebtoken::errors::ErrorKind::InvalidAudience => Self::new(StatusCode::FORBIDDEN)
//                 .with_name("invalid_audience")
//                 .with_message("The given token audience was invalid."),

//             jsonwebtoken::errors::ErrorKind::InvalidSubject => Self::new(StatusCode::FORBIDDEN)
//                 .with_name("invalid_subject")
//                 .with_message("The given token subject was invalid."),

//             jsonwebtoken::errors::ErrorKind::ImmatureSignature => {
//                 Self::new(StatusCode::UNAUTHORIZED)
//                     .with_name("immature_token")
//                     .with_message("The given token is not yet valid.")
//             }

//             jsonwebtoken::errors::ErrorKind::MissingAlgorithm => {
//                 Self::new(StatusCode::UNAUTHORIZED)
//                     .with_name("missing_token_algorithm")
//                     .with_message("The given token was missing an algorithm.")
//             }

//             jsonwebtoken::errors::ErrorKind::MissingRequiredClaim(_) => {
//                 Self::new(StatusCode::FORBIDDEN)
//                     .with_name("missing_claim")
//                     .with_message("The given token was missing a required claim.")
//             }

//             jsonwebtoken::errors::ErrorKind::Base64(err) => Self::new(StatusCode::UNAUTHORIZED)
//                 .with_name("malformed_token")
//                 .with_error(err),

//             jsonwebtoken::errors::ErrorKind::Json(err) => Self::new(StatusCode::UNAUTHORIZED)
//                 .with_name("malformed_token")
//                 .with_error(err),

//             jsonwebtoken::errors::ErrorKind::Utf8(err) => Self::new(StatusCode::UNAUTHORIZED)
//                 .with_name("malformed_token")
//                 .with_error(err),

//             jsonwebtoken::errors::ErrorKind::Crypto(err) => Self::new(StatusCode::UNAUTHORIZED)
//                 .with_name("invalid_token")
//                 .with_error(err),

//             _ => Self::new(StatusCode::INTERNAL_SERVER_ERROR)
//                 .with_name("invalid_token")
//                 .with_message("An unknown error occurred."),
//         }
//     }
// }
