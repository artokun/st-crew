use utoipa::openapi::{OpenApi, PathItem, Ref, RefOr, Schema};

const OPENAPI_SCHEMA: &str = include_str!("../../../schemas/openapi.json");

fn main() {
    let schema =
        serde_json::from_str::<OpenApi>(OPENAPI_SCHEMA).expect("failed to parse openapi schema");

    let paths = schema.paths.paths;
    let components = schema.components.expect("missing components");

    for (component_name, component_schema) in components.schemas {
        match component_schema {
            RefOr::Ref(schema_ref) => {
                println!("ignoring component reference: {:?}", schema_ref);
            }

            RefOr::T(schema) => {
                write_component(&component_name, schema);
            }
        }
    }

    for (path_name, path_item) in paths {
        write_path(&path_name, path_item);
    }
}

fn write_component(name: &str, schema: Schema) {
    println!("writing component: {}", name);
    println!("{:#?}", schema);
}

fn write_path(name: &str, path_item: PathItem) {
    println!("writing path: {}", name);
    println!("{:#?}", path_item);
}
