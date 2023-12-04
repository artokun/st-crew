use serde::{de::DeserializeOwned, ser::Serialize};
use utoipa::ToSchema;

mod call;
mod dispatch;
mod receiver;
mod request;

pub use call::*;
pub use dispatch::*;
pub use receiver::*;
pub use request::*;

pub trait RpcCommand: ToSchema<'static> + DeserializeOwned + Send + Sync + 'static {
    const NAME: &'static str;

    type Input: ToSchema<'static> + DeserializeOwned + Send + Sync + 'static;
    type Output: ToSchema<'static> + Serialize + Send + Sync + 'static;
}

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
