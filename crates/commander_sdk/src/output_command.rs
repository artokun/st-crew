use std::{borrow::Cow, collections::BTreeMap, fs::File, io::Write};

use utoipa::openapi::{KnownFormat, Object, Ref, RefOr, Schema, SchemaFormat, SchemaType};

use crate::{output_struct::TypeModel, INDENT};

/**

#[derive(Serialize)]
pub struct GetServerInfoCommand;

impl RpcCommand for GetServerInfoCommand {
    const NAME: &'static str = "get_server_info";

    type Output = GetServerInfoResult;
}

 */

#[derive(Default)]
pub struct CommandModel {
    pub command_name: String,
    pub input: BTreeMap<String, TypeModel>,
    pub output: String,
}
