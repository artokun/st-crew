use serde::de::DeserializeOwned;
use utoipa::ToSchema;

mod call;
mod dispatch;
mod receiver;
mod request;

pub use call::*;
pub use dispatch::*;
pub use receiver::*;
pub use request::*;

use crate::response::ApiResponse;

#[axum::async_trait]
pub trait RpcCommand: ToSchema<'static> + DeserializeOwned + Send + Sync + 'static {
    const NAME: &'static str;

    type Input: ToSchema<'static> + DeserializeOwned + Send + Sync + 'static;
    type Output: ApiResponse;
}

/// If you see it in type errors its most likely because the third argument to your handler isn't
/// [`Rpc`].
///
/// You normally shouldn't have to use this trait directly.
pub trait RpcEndpoint {
    type Command: RpcCommand;
}

mod macros {
    /// Implements `RpcEndpoint` for functions with `Rpc<Command>` somewhere in their signature.
    macro_rules! impl_rpc_endpoint_for_fn {
        ( $($ty:ident),* $(,)? ) => {
            impl<M, T0, C, $($ty,)*> RpcEndpoint for (M, T0, RpcDispatcher<C>, $($ty,)*)
            where
                C: RpcCommand
            {
                type Command = C;
            }
        };
    }

    pub(super) use impl_rpc_endpoint_for_fn;
}

macros::impl_rpc_endpoint_for_fn!();
macros::impl_rpc_endpoint_for_fn!(T1);
macros::impl_rpc_endpoint_for_fn!(T1, T2);
macros::impl_rpc_endpoint_for_fn!(T1, T2, T3);
macros::impl_rpc_endpoint_for_fn!(T1, T2, T3, T4);
macros::impl_rpc_endpoint_for_fn!(T1, T2, T3, T4, T5);
macros::impl_rpc_endpoint_for_fn!(T1, T2, T3, T4, T5, T6);
macros::impl_rpc_endpoint_for_fn!(T1, T2, T3, T4, T5, T6, T7);
macros::impl_rpc_endpoint_for_fn!(T1, T2, T3, T4, T5, T6, T7, T8);
macros::impl_rpc_endpoint_for_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
macros::impl_rpc_endpoint_for_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
macros::impl_rpc_endpoint_for_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
macros::impl_rpc_endpoint_for_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
macros::impl_rpc_endpoint_for_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
macros::impl_rpc_endpoint_for_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
macros::impl_rpc_endpoint_for_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
macros::impl_rpc_endpoint_for_fn!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
);

// pub trait RpcCommandSchema {
//     fn input_schema() -> RootSchema;

//     fn output_schema() -> RootSchema;
// }

// impl<C> RpcCommandSchema for C
// where
//     C: RpcCommand,
// {
//     fn input_schema() -> RootSchema {
//         SchemaGenerator::default().into_root_schema_for::<C>()
//     }

//     fn output_schema() -> RootSchema {
//         SchemaGenerator::default().into_root_schema_for::<C::Output>()
//     }
// }
