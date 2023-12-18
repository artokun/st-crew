use std::{collections::BTreeMap, path::PathBuf};

use anyhow::Context;
use to_snake_case::ToSnakeCase;
use utoipa::openapi::{AllOf, Object, OneOf, Ref, RefOr, Schema};

use crate::{
    output_one_of::{OneOfItem, OneOfItemType, OneOfModel},
    output_struct::{FieldsModel, StructModel},
};

#[derive(Default)]
pub struct GenerateContext {
    pub rust_sdk_root: PathBuf,

    pub components: BTreeMap<String, StructModel>,

    /// These are components that reference other components that have not yet been loaded
    /// in a way that prevents their type from being generated.
    pub delayed_components: BTreeMap<String, Schema>,
}

impl GenerateContext {
    pub fn load_component(&mut self, component_name: String, schema: Schema) -> anyhow::Result<()> {
        if component_name == "Command" {
            // The command enum is currently useless to us, so we can ignore it.
            return Ok(());
        }

        if component_name.ends_with("Command") {
            return self.load_command(component_name, schema);
        }

        let file_name = format!("{}.rs", component_name.to_snake_case());

        let file_location = self
            .rust_sdk_root
            .join("src")
            .join("models")
            .join(file_name);

        // Open the file in write-only mode, overwriting any existing content
        let mut file = std::fs::File::create(&file_location)
            .with_context(|| format!("failed to create file: {}", file_location.display()))?;

        match schema {
            Schema::Object(object) => {
                let component = self.load_object(component_name.clone(), object)?;

                component.write_to_file(&mut file)?;

                self.components.insert(component_name, component);
            }

            Schema::Array(schema) => anyhow::bail!("array: {:#?}", schema),

            Schema::OneOf(one_of) => {
                self.load_one_of(component_name, one_of)?
                    .write_to_file(&mut file)?;
            }

            Schema::AllOf(all_of) => {
                StructModel {
                    name: component_name,
                    fields: self.load_all_of(all_of)?,
                }
                .write_to_file(&mut file)?;
            }

            Schema::AnyOf(schema) => anyhow::bail!("anyof: {:#?}", schema),

            schema => anyhow::bail!("other: {:#?}", schema),
        }

        Ok(())
    }

    fn load_object(&mut self, name: String, object: Object) -> anyhow::Result<StructModel> {
        let mut struct_model = StructModel {
            name,
            ..Default::default()
        };

        struct_model.fields.add_properties_from(object)?;

        Ok(struct_model)
    }

    fn load_one_of(&mut self, name: String, one_of: OneOf) -> anyhow::Result<OneOfModel> {
        let mut one_of_model = OneOfModel {
            name: name.clone(),
            ..Default::default()
        };

        for (i, item) in one_of.items.into_iter().enumerate() {
            match item {
                RefOr::Ref(Ref { ref_location, .. }) => {
                    let ref_name = ref_location.split('/').last().unwrap();

                    one_of_model.items.push(OneOfItem {
                        name: ref_name.to_owned(),
                        ty: OneOfItemType::Ref(ref_name.to_owned()),
                    });
                }

                RefOr::T(schema) => {
                    let item = match schema {
                        Schema::Object(object) => OneOfItem {
                            name: object
                                .title
                                .clone()
                                .unwrap_or_else(|| format!("{}_{}", name, i)),
                            ty: OneOfItemType::Struct(FieldsModel::try_from(object)?),
                        },

                        Schema::Array(schema) => anyhow::bail!("array: {:#?}", schema),

                        Schema::OneOf(one_of) => anyhow::bail!("nested oneof: {:#?}", one_of),

                        // Schema::OneOf(one_of) => OneOfItem {
                        //     name: one_of.title.clone().unwrap_or_else(|| i.to_string()),
                        //     ty: OneOfItemType::OneOf(self.load_one_of(
                        //         format!(
                        //             "{}_{}",
                        //             name,
                        //             one_of.title.clone().unwrap_or_else(|| i.to_string())
                        //         ),
                        //         one_of,
                        //     )?),
                        // },
                        Schema::AllOf(all_of) => OneOfItem {
                            name: all_of
                                .title
                                .clone()
                                .unwrap_or_else(|| format!("{}_{}", name, i)),
                            ty: OneOfItemType::Struct(self.load_all_of(all_of)?),
                        },

                        Schema::AnyOf(schema) => anyhow::bail!("anyof: {:#?}", schema),

                        schema => anyhow::bail!("other: {:#?}", schema),
                    };

                    one_of_model.items.push(item);
                }
            }
        }

        Ok(one_of_model)
    }

    fn load_all_of(&mut self, all_of: AllOf) -> anyhow::Result<FieldsModel> {
        let mut fields_model = FieldsModel::default();

        for item in all_of.items {
            match item {
                RefOr::Ref(Ref { ref_location, .. }) => {
                    let ref_name = ref_location.split('/').last().unwrap();

                    // If the component has already been loaded, add its fields to the current
                    // struct model. Otherwise, add it to the list of delayed components.
                    if let Some(other_struct_model) = self.components.get(ref_name) {
                        fields_model.merge(&other_struct_model.fields)?;
                    } else {
                        panic!("all_of ref: {}", ref_name);
                    }
                }

                RefOr::T(Schema::Object(object)) => {
                    fields_model.add_properties_from(object)?;
                }

                RefOr::T(schema) => {
                    eprintln!("ignoring component: {:#?}", schema);
                    continue;
                }
            }
        }

        Ok(fields_model)
    }

    pub fn load_command(&mut self, name: String, schema: Schema) -> anyhow::Result<()> {
        let file_name = format!("{}.rs", name.to_snake_case());

        let file_location = self
            .rust_sdk_root
            .join("src")
            .join("commands")
            .join(file_name);

        Ok(())
    }
}
