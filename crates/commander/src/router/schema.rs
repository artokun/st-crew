use axum::{response::Html, Extension, Json};
use utoipa::openapi::{
    Components, ComponentsBuilder, Info, InfoBuilder, LicenseBuilder, OpenApi, OpenApiBuilder,
    Paths, PathsBuilder,
};
use utoipa_redoc::Redoc;

use crate::response::ApiError;

use super::CommanderState;

pub struct CommanderSchemaBuilder {
    pub info: InfoBuilder,
    pub paths: PathsBuilder,
    pub components: ComponentsBuilder,
}

impl Default for CommanderSchemaBuilder {
    fn default() -> Self {
        Self {
            info: InfoBuilder::new(),
            paths: PathsBuilder::new(),
            components: ComponentsBuilder::new().schema_from::<ApiError>(),
        }
    }
}

impl CommanderSchemaBuilder {
    pub fn build(self) -> CommanderSchema {
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

pub async fn get_schema(Extension(state): Extension<CommanderState>) -> Json<OpenApi> {
    let schema = &state.into_inner().schema;

    let openapi = OpenApiBuilder::new()
        .info(schema.info.clone())
        .paths(schema.paths.clone())
        .components(Some(schema.components.clone()))
        // .tags(Some([TagBuilder::new()
        //     .name("todo")
        //     .description(Some("Todo items management API"))
        //     .build()]))
        .build();

    Json(openapi)
}

pub async fn get_redoc() -> Html<String> {
    Html(Redoc::new("/schema.json").to_html())
}
