use std::{fmt, marker::PhantomData, sync::Arc};

use axum::{
    body::HttpBody,
    http::{header::ACCEPT, Request, Response, StatusCode},
};
use mime::{Mime, MimeIter};
use tower_http::validate_request::ValidateRequest;

pub struct AcceptHeaders<const LEN: usize, ResBody> {
    header_values: Arc<[Mime; LEN]>,
    _ty: PhantomData<fn() -> ResBody>,
}

impl<const LEN: usize, ResBody> AcceptHeaders<LEN, ResBody> {
    /// Create a new `AcceptHeaders`.
    ///
    /// # Panics
    ///
    /// Panics if `header_values` are not in the form: `type/subtype`, such as `application/json`
    pub fn new(header_values: [Mime; LEN]) -> Self
    where
        ResBody: HttpBody + Default,
    {
        Self {
            header_values: Arc::new(header_values),
            _ty: PhantomData,
        }
    }
}

impl<const LEN: usize, ResBody> Clone for AcceptHeaders<LEN, ResBody> {
    fn clone(&self) -> Self {
        Self {
            header_values: Arc::clone(&self.header_values),
            _ty: PhantomData,
        }
    }
}

impl<const LEN: usize, ResBody> fmt::Debug for AcceptHeaders<LEN, ResBody> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AcceptHeaders")
            .field("header_values", &self.header_values)
            .finish()
    }
}

impl<const LEN: usize, B, ResBody> ValidateRequest<B> for AcceptHeaders<LEN, ResBody>
where
    ResBody: HttpBody + Default,
{
    type ResponseBody = ResBody;

    fn validate(&mut self, request: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {
        if request
            .headers()
            .get_all(ACCEPT)
            .into_iter()
            .filter_map(|header| header.to_str().ok())
            .any(|h| {
                MimeIter::new(h)
                    .map(|mim| {
                        if let Ok(mim) = mim {
                            self.header_values.iter().any(|header_value| {
                                let typ = header_value.type_();
                                let subtype = header_value.subtype();

                                match (mim.type_(), mim.subtype()) {
                                    (t, s) if t == typ && s == subtype => true,
                                    (t, mime::STAR) if t == typ => true,
                                    (mime::STAR, mime::STAR) => true,
                                    _ => false,
                                }
                            })
                        } else {
                            false
                        }
                    })
                    .reduce(|acc, mim| acc || mim)
                    .unwrap_or(false)
            })
        {
            return Ok(());
        }

        let mut response = Response::new(ResBody::default());
        *response.status_mut() = StatusCode::NOT_ACCEPTABLE;
        Err(response)
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{header::ACCEPT, Request, Response, StatusCode},
    };
    use mime::Mime;
    use tower::{BoxError, Service, ServiceBuilder, ServiceExt};
    use tower_http::validate_request::ValidateRequestHeaderLayer;

    use super::AcceptHeaders;

    const ACCEPTABLE_MIME_TYPES: [Mime; 2] = [
        mime::APPLICATION_JSON,
        mime::APPLICATION_WWW_FORM_URLENCODED,
    ];

    const ACCEPTABLE_HEADERS: [&str; 2] = ["application/json", "application/x-www-form-urlencoded"];

    async fn make_request<const LEN: usize>(headers: [&str; LEN]) -> StatusCode {
        let mut service = ServiceBuilder::new()
            .layer(ValidateRequestHeaderLayer::custom(AcceptHeaders::new(
                ACCEPTABLE_MIME_TYPES,
            )))
            .service_fn(echo);

        service
            .ready()
            .await
            .unwrap()
            .call(dummy_request(headers))
            .await
            .unwrap()
            .status()
    }

    fn dummy_request<const LEN: usize>(headers: [&str; LEN]) -> Request<Body> {
        let mut req = Request::get("/");

        for header in headers {
            req = req.header(ACCEPT, header);
        }

        req.body(Body::empty()).unwrap()
    }

    #[tokio::test]
    async fn valid_accept_header() {
        for acceptable in &ACCEPTABLE_HEADERS {
            assert_eq!(
                make_request([acceptable]).await,
                StatusCode::OK,
                "{acceptable} should be accepted",
            );
        }
    }

    #[tokio::test]
    async fn valid_accept_header_accept_all_json() {
        assert_eq!(
            make_request(["application/*"]).await,
            StatusCode::OK,
            "application/* should be accepted"
        );
    }

    #[tokio::test]
    async fn valid_accept_header_accept_all() {
        assert_eq!(
            make_request(["*/*"]).await,
            StatusCode::OK,
            "*/* should be accepted"
        );
    }

    #[tokio::test]
    async fn invalid_accept_header() {
        assert_eq!(
            make_request(["invalid"]).await,
            StatusCode::NOT_ACCEPTABLE,
            "invalid mime type should not be acceptable"
        );
    }

    #[tokio::test]
    async fn not_accepted_accept_header_subtype() {
        assert_eq!(
            make_request(["application/strings"]).await,
            StatusCode::NOT_ACCEPTABLE,
            "application/strings should not be acceptable"
        );
    }

    #[tokio::test]
    async fn not_accepted_accept_header() {
        assert_eq!(
            make_request(["text/strings"]).await,
            StatusCode::NOT_ACCEPTABLE,
            "text/strings should not be acceptable"
        );
    }

    #[tokio::test]
    async fn accepted_multiple_header_value() {
        for acceptable in &ACCEPTABLE_HEADERS {
            assert_eq!(
                make_request(["text/strings", &format!("invalid, {acceptable}")]).await,
                StatusCode::OK,
                "multiple accept headers with {acceptable} should be accepted",
            );
        }
    }

    #[tokio::test]
    async fn accepted_inner_header_value() {
        for acceptable in &ACCEPTABLE_HEADERS {
            assert_eq!(
                make_request([&format!("text/strings, invalid, {acceptable}")]).await,
                StatusCode::OK,
                "accept headers with inner {acceptable} should be accepted"
            );
        }
    }

    #[tokio::test]
    async fn accepted_header_with_quotes_valid() {
        assert_eq!(
            make_request([
                "foo/bar; parisien=\"baguette, text/html, jambon, fromage\", application/*"
            ])
            .await,
            StatusCode::OK
        );
    }

    #[tokio::test]
    async fn accepted_header_with_quotes_invalid() {
        assert_eq!(
            make_request(["foo/bar; parisien=\"baguette, text/html, jambon, fromage\""]).await,
            StatusCode::NOT_ACCEPTABLE
        );
    }

    async fn echo(req: Request<Body>) -> Result<Response<Body>, BoxError> {
        Ok(Response::new(req.into_body()))
    }
}
