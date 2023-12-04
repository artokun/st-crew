use axum::response::sse::Event;
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use futures_util::{Future, Stream};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use utoipa::ToSchema;

use crate::response::ApiError;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    ToSchema,
    Serialize,
    Deserialize,
    Default,
)]
#[serde(rename_all = "lowercase")]
pub enum SseStreamFormat {
    #[default]
    Json,
    MsgPack,
}

impl SseStreamFormat {
    fn serialize<E: Serialize>(self, data: &E) -> Result<String, ApiError> {
        match self {
            Self::Json => serde_json::to_string(data).map_err(ApiError::internal_server_error),
            Self::MsgPack => rmp_serde::encode::to_vec_named(data)
                .map(|a| STANDARD_NO_PAD.encode(a))
                .map_err(ApiError::internal_server_error),
        }
    }
}

pub trait SeeStreamEvent: Serialize {
    fn event(&self) -> &'static str {
        "message"
    }
}

pub fn sse_stream_channel<E, Fut>(
    name: &'static str,
    format: SseStreamFormat,
    func: impl FnOnce(SseChannel<E>) -> Fut,
) -> impl Stream<Item = Result<Event, ApiError>>
where
    E: SeeStreamEvent,
    Fut: Future<Output = Result<(), ApiError>>,
{
    let (tx, mut rx) = mpsc::channel::<E>(1);

    let stream = async_stream::stream! {
        let _guard = StreamCounterGuard::new(name);

        let fut = func(SseChannel { tx });

        tokio::pin!(fut);

        loop {
            tokio::select! {
                result = &mut fut => {
                    if let Err(err) = result {
                        yield Err(err);
                    }

                    break;
                },

                message = rx.recv() => {
                    let Some(message) = message else {
                        break;
                    };

                    match format.serialize(&message) {
                        Ok(data) => {
                            yield Ok(Event::default()
                                .event(message.event())
                                .data(data));
                        }

                        Err(err) => {
                            yield Err(err);
                        }
                    }
                }
            }
        }
    };

    stream
}

pub struct SseChannel<E> {
    tx: mpsc::Sender<E>,
}

impl<E> SseChannel<E> {
    pub async fn send(&self, value: impl Into<E>) -> Result<(), mpsc::error::SendError<()>> {
        self.tx
            .send(value.into())
            .await
            .map_err(|_err| mpsc::error::SendError(()))
    }
}

struct StreamCounterGuard {
    name: &'static str,
}

impl StreamCounterGuard {
    fn new(name: &'static str) -> Self {
        // increment_gauge!("server_sent_event_streams", 1.0_f64, "name" => name);

        Self { name }
    }
}

impl Drop for StreamCounterGuard {
    fn drop(&mut self) {
        // decrement_gauge!("server_sent_event_streams", 1.0_f64, "name" => self.name);
    }
}
