use std::any::Any;

use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::StatusCode,
};
use tokio::sync::{mpsc, oneshot};

use crate::{
    data_format::DataFormat,
    response::ApiError,
    router::CommanderState,
    rpc::{RpcCommand, RpcRequest},
};

use super::{RpcCallInput, RpcReply};

#[async_trait]
pub trait RpcDispatch: Send + Sync {
    fn as_any(&self) -> &dyn Any;

    async fn dispatch(
        &self,
        call_id: u64,
        data_format: DataFormat,
        body: &[u8],
    ) -> Result<Vec<u8>, DispatchError>;
}

pub struct RpcDispatcher<C>
where
    C: RpcCommand,
{
    tx: mpsc::UnboundedSender<RpcRequest<C>>,
}

impl<C> Clone for RpcDispatcher<C>
where
    C: RpcCommand,
{
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}

impl<C> RpcDispatcher<C>
where
    C: RpcCommand,
{
    pub(crate) fn new(tx: mpsc::UnboundedSender<RpcRequest<C>>) -> Self {
        Self { tx }
    }

    pub async fn call(&self, input: impl Into<C::Input>) -> Result<C::Output, CallError> {
        let (reply_tx, reply_rx) = oneshot::channel();

        self.tx
            .send(RpcRequest {
                input: input.into(),
                reply_tx,
            })
            .ok();

        reply_rx.await.map_err(|_| CallError::Unhandled)
    }
}

#[async_trait]
impl<C> RpcDispatch for RpcDispatcher<C>
where
    C: RpcCommand,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn dispatch(
        &self,
        call_id: u64,
        data_format: DataFormat,
        body: &[u8],
    ) -> Result<Vec<u8>, DispatchError> {
        let input = data_format
            .deserialize::<RpcCallInput<C>>(body)
            .map_err(DispatchError::DeserializeInput)?
            .input;

        let output = self.call(input).await?;

        Ok(data_format
            .serialize(&RpcReply::<C> {
                id: call_id,
                output,
            })
            .map_err(DispatchError::SerializeOutput)?)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CallError {
    #[error("command was not executed")]
    Unhandled,
}

impl From<CallError> for ApiError {
    fn from(err: CallError) -> Self {
        ApiError::internal_server_error(err)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DispatchError {
    #[error("failed to deserialize input: {0}")]
    DeserializeInput(Box<dyn std::error::Error + Send + Sync>),

    #[error(transparent)]
    Call(#[from] CallError),

    #[error("failed to serialize output: {0}")]
    SerializeOutput(Box<dyn std::error::Error + Send + Sync>),
}

impl From<DispatchError> for ApiError {
    fn from(err: DispatchError) -> Self {
        match err {
            DispatchError::DeserializeInput(err) => ApiError::new(StatusCode::BAD_REQUEST)
                .with_name("bad_input")
                .with_error(err),

            DispatchError::Call(err) => ApiError::from(err),

            DispatchError::SerializeOutput(err) => ApiError::internal_server_error(err),
        }
    }
}

#[axum::async_trait]
impl<C, S> FromRequest<S> for RpcDispatcher<C>
where
    C: RpcCommand,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let commander = req
            .extensions()
            .get::<CommanderState>()
            .cloned()
            .expect("commander missing from axum extensions")
            .into_inner();

        let Some(command) = commander.commands.get(C::NAME) else {
            return Err(ApiError::internal_server_error(
                "rpc command not registered in commander",
            ));
        };

        let Some(rpc_dispatch) = command.as_any().downcast_ref::<RpcDispatcher<C>>() else {
            return Err(ApiError::internal_server_error(
                "rpc dispatch was not of the expected type",
            ));
        };

        Ok(rpc_dispatch.clone())
    }
}
