use std::any::{Any, TypeId};

use axum::{
    async_trait,
    extract::{FromRequest, Request},
};
use st_commander_derive::ApiResponse;
use tokio::sync::{mpsc, oneshot};

use crate::{
    data_format::DataFormat,
    response::ApiError,
    router::CommanderState,
    rpc::{RpcCommand, RpcRequest},
};

use super::{RpcCall, RpcReply};

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
        let input = if TypeId::of::<C::Input>() == TypeId::of::<()>() {
            // I wonder if this is safe... Technically `()` is a zero-sized type, so it
            // should be safe to transmute it to itself. Technically `TypeId` is not
            // guaranteed to be unique, though it's absolutely incredibly unlikely to
            // not be so it's probably fine. Maybe.
            unsafe { std::mem::transmute_copy(&()) }
        } else {
            data_format
                .deserialize::<RpcCall<C>>(body)
                .map_err(DispatchError::BadInput)?
                .input
        };

        let output = self.call(input).await?;

        Ok(data_format
            .serialize(&RpcReply::<C> {
                id: call_id,
                output,
            })
            .map_err(DispatchError::BadOutput)?)
    }
}

#[derive(Debug, thiserror::Error, ApiResponse)]
pub enum CallError {
    /// The command was not handled by any system.
    #[response(status = INTERNAL_SERVER_ERROR)]
    #[error("command was not executed")]
    Unhandled,
}

#[derive(Debug, thiserror::Error, ApiResponse)]
pub enum DispatchError {
    #[response(status = BAD_REQUEST)]
    #[error("failed to deserialize input: {0}")]
    BadInput(Box<dyn std::error::Error + Send + Sync>),

    #[response(transparent)]
    #[error(transparent)]
    Call(#[from] CallError),

    #[response(status = INTERNAL_SERVER_ERROR)]
    #[error("failed to serialize output: {0}")]
    BadOutput(Box<dyn std::error::Error + Send + Sync>),
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
