use std::{collections::BTreeMap, fs::File, io::Write};

use to_snake_case::ToSnakeCase;
use utoipa::openapi::{KnownFormat, Object, Ref, RefOr, Schema, SchemaFormat, SchemaType};

use crate::INDENT;

#[derive(Default)]
pub struct StructModel {
    pub name: String,
    pub fields: FieldsModel,
}

impl StructModel {
    pub fn write_to_file(&self, file: &mut File) -> std::io::Result<()> {
        println!("writing {}", self.name);

        writeln!(file, r#"use serde::Deserialize;"#)?;
        writeln!(file)?;
        writeln!(file, r#"#[derive(Debug, Deserialize)]"#)?;
        writeln!(file, r#"pub struct {} {{"#, self.name)?;

        for (field_name, field_ty) in &self.fields {
            write!(file, "{}", INDENT)?;
            write!(file, "pub {}: ", field_name.to_snake_case())?;
            field_ty.write_to_file(file)?;
            writeln!(file, ",")?;
        }

        writeln!(file, r#"}}"#)?;

        Ok(())
    }
}

#[derive(Default)]
pub struct FieldsModel(BTreeMap<String, TypeModel>);

impl FieldsModel {
    pub fn add_properties_from(
        &mut self,
        Object {
            title: _,
            description: _,
            enum_values: _,
            required,
            properties,
            ..
        }: Object,
    ) -> anyhow::Result<()> {
        for (prop_name, prop_schema) in properties {
            let output_ty = match prop_schema {
                RefOr::Ref(Ref { ref_location, .. }) => {
                    ref_location.split('/').last().unwrap().to_owned()
                }

                RefOr::T(Schema::Object(Object {
                    schema_type: SchemaType::String,
                    ..
                })) => "String".into(),

                RefOr::T(Schema::Object(Object {
                    schema_type: SchemaType::Integer,
                    format,
                    minimum,
                    ..
                })) => match (format, minimum) {
                    (Some(SchemaFormat::KnownFormat(KnownFormat::Int64)), Some(min))
                        if min >= 0.0 =>
                    {
                        "u64".into()
                    }

                    (Some(SchemaFormat::KnownFormat(KnownFormat::Int64)), _) => "i64".into(),

                    (Some(SchemaFormat::KnownFormat(KnownFormat::Int32)), Some(min))
                        if min >= 0.0 =>
                    {
                        "u32".into()
                    }

                    (Some(SchemaFormat::KnownFormat(KnownFormat::Int32)), _) => "i32".into(),

                    (None, Some(min)) if min >= 0.0 => "usize".into(),

                    _ => anyhow::bail!("unsupported integer format"),
                },

                RefOr::T(schema) => {
                    eprintln!("ignoring property: {:#?}", schema);
                    continue;
                }
            };

            let output_ty = TypeModel {
                ty: output_ty,
                required: required.contains(&prop_name),
            };

            self.0.insert(prop_name, output_ty);
        }

        Ok(())
    }

    pub fn merge(&mut self, other: &Self) -> anyhow::Result<()> {
        for (field_name, field_ty) in &other.0 {
            if let Some(existing_field) = self.0.get(field_name) {
                if existing_field.ty != field_ty.ty {
                    anyhow::bail!(
                        "field type mismatch: {} != {}",
                        existing_field.ty,
                        field_ty.ty
                    );
                }
            }

            self.0.insert(field_name.clone(), field_ty.clone());
        }

        Ok(())
    }
}

impl AsRef<BTreeMap<String, TypeModel>> for FieldsModel {
    fn as_ref(&self) -> &BTreeMap<String, TypeModel> {
        &self.0
    }
}

impl AsMut<BTreeMap<String, TypeModel>> for FieldsModel {
    fn as_mut(&mut self) -> &mut BTreeMap<String, TypeModel> {
        &mut self.0
    }
}

impl TryFrom<Object> for FieldsModel {
    type Error = anyhow::Error;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        let mut fields = Self::default();

        fields.add_properties_from(object)?;

        Ok(fields)
    }
}

impl<'a> IntoIterator for &'a FieldsModel {
    type Item = (&'a String, &'a TypeModel);
    type IntoIter = std::collections::btree_map::Iter<'a, String, TypeModel>;

    fn into_iter(self) -> std::collections::btree_map::Iter<'a, String, TypeModel> {
        self.0.iter()
    }
}

#[derive(Clone)]
pub struct TypeModel {
    pub ty: String,
    pub required: bool,
}

impl TypeModel {
    pub fn write_to_file(&self, file: &mut File) -> std::io::Result<()> {
        if self.required {
            write!(file, "{}", self.ty)?;
        } else {
            write!(file, "Option<{}>", self.ty)?;
        }

        Ok(())
    }
}
