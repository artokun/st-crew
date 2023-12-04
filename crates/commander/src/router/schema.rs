use axum::{http::StatusCode, Extension};
use utoipa::openapi::{
    tag::TagBuilder, Components, ComponentsBuilder, InfoBuilder, LicenseBuilder, OpenApi,
    OpenApiBuilder,
};

use crate::response::{ApiResponse, ApiResult};

use super::CommanderState;

#[derive(Default)]
pub struct CommanderSchemaBuilder {
    pub components: ComponentsBuilder,
}

impl CommanderSchemaBuilder {
    pub fn build(self) -> CommanderSchema {
        CommanderSchema {
            components: self.components.build(),
        }
    }
}

pub struct CommanderSchema {
    pub components: Components,
}

pub async fn get_schema(Extension(state): Extension<CommanderState>) -> ApiResult<OpenApi> {
    let schema = &state.into_inner().schema;

    let openapi = OpenApiBuilder::new()
        .info(
            InfoBuilder::new()
                .title("st_commander")
                .version("0.1.0")
                .description(Some(""))
                .license(Some(LicenseBuilder::new().name("").build())),
        )
        .paths(
            utoipa::openapi::path::PathsBuilder::new(), // .path(
                                                        //     todo::__path_list_todos::path(),
                                                        //     todo::__path_list_todos::path_item(Some("todo")),
                                                        // )
                                                        // .path(
                                                        //     todo::__path_search_todos::path(),
                                                        //     todo::__path_search_todos::path_item(Some("todo")),
                                                        // )
                                                        // .path(
                                                        //     todo::__path_create_todo::path(),
                                                        //     todo::__path_create_todo::path_item(Some("todo")),
                                                        // )
                                                        // .path(
                                                        //     todo::__path_mark_done::path(),
                                                        //     todo::__path_mark_done::path_item(Some("todo")),
                                                        // )
                                                        // .path(
                                                        //     todo::__path_delete_todo::path(),
                                                        //     todo::__path_delete_todo::path_item(Some("todo")),
                                                        // ),
        )
        .components(Some(schema.components.clone()))
        .tags(Some([TagBuilder::new()
            .name("todo")
            .description(Some("Todo items management API"))
            .build()]))
        .build();

    Ok(ApiResponse::new(StatusCode::OK).with_body(openapi))
}
