use std::collections::BTreeMap;

use axum::{response::Html, Extension, Json};
use utoipa::openapi::{
    AllOfBuilder, Components, ComponentsBuilder, Discriminator, Info, InfoBuilder, ObjectBuilder,
    OneOfBuilder, OpenApi, OpenApiBuilder, Paths, PathsBuilder, Ref, RefOr, Response, SchemaType,
};
use utoipa_redoc::Redoc;

use crate::response::ApiError;

use super::CommanderState;

pub struct CommanderSchemaBuilder {
    pub info: InfoBuilder,
    pub paths: PathsBuilder,
    pub components: ComponentsBuilder,

    pub commands: Vec<CommandSchema>,
}

pub struct CommandSchema {
    pub id: &'static str,

    pub name: &'static str,
    pub title: Option<String>,
    pub description: Option<String>,

    pub input: Option<Ref>,
    pub responses: BTreeMap<String, RefOr<Response>>,
}

impl Default for CommanderSchemaBuilder {
    fn default() -> Self {
        Self {
            info: InfoBuilder::new(),
            paths: PathsBuilder::new(),
            components: ComponentsBuilder::new().schema_from::<ApiError>(),

            commands: Vec::with_capacity(16),
        }
    }
}

impl CommanderSchemaBuilder {
    pub fn build(mut self) -> CommanderSchema {
        let mut rpc_calls = OneOfBuilder::new();

        let mut rpc_call_discriminator = Discriminator::new("command");

        for command in self.commands {
            let command_entry_ref = {
                let command_schema = ObjectBuilder::new()
                    .property(
                        "command",
                        ObjectBuilder::new()
                            .schema_type(SchemaType::String)
                            .enum_values(Some(vec![command.id])),
                    )
                    .required("command");

                self.components = self.components.schema(
                    command.name,
                    if let Some(input) = command.input {
                        command_schema.property("input", input).required("input")
                    } else {
                        command_schema
                    },
                );

                Ref::from_schema_name(command.name)
            };

            rpc_call_discriminator.mapping.insert(
                command.id.to_string(),
                command_entry_ref.ref_location.clone(),
            );

            rpc_calls = rpc_calls.item(command_entry_ref);

            let mut response_schema = OneOfBuilder::new()
                .title(command.title)
                .description(command.description);

            for (_status, response) in command.responses {
                // let status_code = status
                //     .parse::<u16>()
                //     .expect("command responses must be valid status codes");

                match response {
                    RefOr::Ref { .. } => {}
                    RefOr::T(response) => {
                        let content = response
                            .content
                            .into_values()
                            .next()
                            .expect("command responses must have at least one content type");

                        response_schema = response_schema.item(content.schema);
                    }
                }
            }

            self.components = self
                .components
                .schema(&format!("{}Result", command.name), response_schema);
        }

        let rpc_call_schema = AllOfBuilder::new()
            .item(
                ObjectBuilder::new()
                    .property(
                        "id",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(utoipa::openapi::SchemaType::Integer)
                            .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                                utoipa::openapi::KnownFormat::Int64,
                            )))
                            .minimum(Some(0f64)),
                    )
                    .required("id"),
            )
            .item(rpc_calls.discriminator(Some(rpc_call_discriminator)));

        self.components = self.components.schema("Command", rpc_call_schema);

        CommanderSchema {
            info: self.info.build(),
            paths: self.paths.build(),
            components: self.components.build(),
        }
    }
}

pub struct CommanderSchema {
    pub info: Info,
    pub paths: Paths,
    pub components: Components,
}

impl CommanderSchema {
    pub fn build(&self) -> OpenApi {
        OpenApiBuilder::new()
            .info(self.info.clone())
            .paths(self.paths.clone())
            .components(Some(self.components.clone()))
            .build()
    }
}

pub async fn get_openapi_schema(Extension(state): Extension<CommanderState>) -> Json<OpenApi> {
    Json(state.into_inner().schema.build())
}

// pub async fn get_json_schema(
//     Extension(state): Extension<CommanderState>,
// ) -> Json<schemars::schema::RootSchema> {
//     Json(state.into_inner().schema.jsonschema.build())
// }

pub async fn get_redoc() -> Html<String> {
    Html(Redoc::new("/openapi.json").to_html())
}
