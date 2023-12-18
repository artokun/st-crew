use std::{borrow::Cow, path::Path};

use anyhow::Context;
use context::GenerateContext;
use utoipa::openapi::{OpenApi, PathItem, RefOr};

mod context;
mod output_command;
mod output_one_of;
mod output_struct;

const OPENAPI_SCHEMA: &str = include_str!("../../../schemas/openapi.json");

const INDENT: &str = "    ";

fn main() -> anyhow::Result<()> {
    let schema =
        serde_json::from_str::<OpenApi>(OPENAPI_SCHEMA).expect("failed to parse openapi schema");

    let paths = schema.paths.paths;
    let components = schema.components.expect("missing components");

    let workspace_root = std::env::var("CARGO_MANIFEST_DIR").expect("missing CARGO_MANIFEST_DIR");
    let workspace_root = Path::new(&workspace_root)
        .parent()
        .with_context(|| "failed to get parent")?
        .parent()
        .with_context(|| "failed to get parent")?;

    let rust_sdk_root = workspace_root.join("sdk").join("rust");

    let mut context = GenerateContext {
        rust_sdk_root,
        ..GenerateContext::default()
    };

    for (component_name, component_schema) in components.schemas {
        match component_schema {
            RefOr::Ref(schema_ref) => {
                println!("ignoring component reference: {:?}", schema_ref);
            }

            RefOr::T(schema) => {
                context
                    .load_component(component_name.clone(), schema)
                    .with_context(|| format!("failed to load component: {}", component_name))?;
            }
        }
    }

    for (path_name, path_item) in paths {
        write_path(&path_name, path_item);
    }

    Ok(())
}

fn write_path(name: &str, path_item: PathItem) {
    // println!("writing path: {}", name);
    // println!("{:#?}", path_item);
}
