use std::collections::BTreeSet;

use bevy::utils::tracing;
use schemars::schema::{
    ArrayValidation, NumberValidation, ObjectValidation, StringValidation, SubschemaValidation,
};
use utoipa::{
    openapi::{schema::AdditionalProperties, Deprecated, RefOr, Schema, SchemaType},
    ToSchema,
};

pub trait ToJsonSchema {
    fn to_json_schema() -> schemars::schema::Schema;
}

impl<T> ToJsonSchema for T
where
    T: ToSchema<'static>,
{
    fn to_json_schema() -> schemars::schema::Schema {
        to_json_schema(T::schema().1)
    }
}

fn to_json_schema(schema: RefOr<Schema>) -> schemars::schema::Schema {
    let mut obj = schemars::schema::SchemaObject::default();

    match schema {
        RefOr::Ref(_) => unimplemented!("reference schemas are not supported yet"),

        RefOr::T(Schema::Array(schema)) => {
            obj.instance_type = Some(schemars::schema::SingleOrVec::Single(Box::new(
                schemars::schema::InstanceType::Array,
            )));

            obj.metadata = Some(Box::new(schemars::schema::Metadata {
                id: None,
                title: schema.title,
                description: schema.description,
                default: schema.default,
                deprecated: schema
                    .deprecated
                    .map_or(false, |dep| dep == Deprecated::True),
                read_only: false,
                write_only: false,
                examples: schema
                    .example
                    .clone()
                    .map_or_else(Vec::default, |example| vec![example]),
            }));

            obj.format = None;
            obj.enum_values = None;
            obj.const_value = None;
            obj.subschemas = None;
            obj.number = None;
            obj.string = None;
            obj.array = Some(Box::new(ArrayValidation {
                items: Some(schemars::schema::SingleOrVec::Single(Box::new(
                    to_json_schema(*schema.items),
                ))),
                additional_items: None,
                max_items: schema.max_items.map(|max| max as _),
                min_items: schema.min_items.map(|min| min as _),
                unique_items: Some(schema.unique_items),
                contains: None,
            }));
            obj.object = None;
            obj.reference = None;
        }

        RefOr::T(Schema::Object(schema)) => {
            obj.instance_type = Some(schemars::schema::SingleOrVec::Single(Box::new(
                match schema.schema_type {
                    SchemaType::Object => schemars::schema::InstanceType::Object,
                    SchemaType::Value => {
                        if schema.default == Some(serde_json::Value::Null) {
                            schemars::schema::InstanceType::Null
                        } else {
                            tracing::error!("{:?}", schema);

                            unimplemented!("non-null value schemas are not supported yet")
                        }
                    }
                    SchemaType::String => schemars::schema::InstanceType::String,
                    SchemaType::Integer => schemars::schema::InstanceType::Integer,
                    SchemaType::Number => schemars::schema::InstanceType::Number,
                    SchemaType::Boolean => schemars::schema::InstanceType::Boolean,
                    SchemaType::Array => schemars::schema::InstanceType::Array,
                },
            )));

            obj.metadata = Some(Box::new(schemars::schema::Metadata {
                id: None,
                title: schema.title,
                description: schema.description,
                default: schema.default,
                deprecated: schema
                    .deprecated
                    .map_or(false, |dep| dep == Deprecated::True),
                read_only: schema.write_only.unwrap_or_default(),
                write_only: schema.write_only.unwrap_or_default(),
                examples: schema
                    .example
                    .map_or_else(Vec::default, |example| vec![example]),
            }));

            obj.format = schema
                .format
                .map(|format| serde_json::to_string(&format).expect("failed to serialize format"));

            if let Some(enum_values) = schema.enum_values {
                if enum_values.len() == 1 {
                    obj.enum_values = None;
                    obj.const_value = Some(enum_values[0].clone());
                } else {
                    obj.enum_values = Some(enum_values);
                    obj.const_value = None;
                }
            }

            match schema.schema_type {
                SchemaType::Object => {
                    obj.object = Some(Box::new(ObjectValidation {
                        max_properties: schema.max_properties.map(|max| max as _),
                        min_properties: schema.min_properties.map(|min| min as _),
                        required: BTreeSet::from_iter(schema.required),
                        properties: schema
                            .properties
                            .into_iter()
                            .map(|(key, schema)| (key, to_json_schema(schema)))
                            .collect(),
                        pattern_properties: Default::default(),
                        additional_properties: schema.additional_properties.map(
                            |additional_properties| match *additional_properties {
                                AdditionalProperties::RefOr(schema) => {
                                    Box::new(to_json_schema(schema))
                                }

                                AdditionalProperties::FreeForm(freeform) => {
                                    Box::new(schemars::schema::Schema::Bool(freeform))
                                }
                            },
                        ),
                        property_names: None,
                    }));
                }

                SchemaType::Value => {}

                SchemaType::String => {
                    obj.string = Some(Box::new(StringValidation {
                        max_length: schema.max_length.map(|max| max as _),
                        min_length: schema.min_length.map(|min| min as _),
                        pattern: schema.pattern,
                    }));
                }

                SchemaType::Integer | SchemaType::Number => {
                    obj.number = Some(Box::new(NumberValidation {
                        multiple_of: schema.multiple_of,
                        maximum: schema.maximum,
                        exclusive_maximum: schema.exclusive_maximum,
                        minimum: schema.minimum,
                        exclusive_minimum: schema.exclusive_minimum,
                    }));
                }

                SchemaType::Boolean => {}

                SchemaType::Array => unreachable!("array schemas are handled separately"),
            }

            obj.subschemas = None;
            obj.reference = None;
        }

        RefOr::T(Schema::OneOf(schema)) => {
            obj.metadata = Some(Box::new(schemars::schema::Metadata {
                id: None,
                title: schema.title,
                description: schema.description,
                default: schema.default,
                deprecated: false,
                read_only: false,
                write_only: false,
                examples: schema
                    .example
                    .map_or_else(Vec::default, |example| vec![example]),
            }));

            assert_eq!(
                schema.discriminator, None,
                "discriminators are not supported in JSON Schema"
            );

            obj.subschemas = Some(Box::new(SubschemaValidation {
                all_of: None,
                any_of: None,
                one_of: Some(schema.items.into_iter().map(to_json_schema).collect()),
                not: None,
                if_schema: None,
                then_schema: None,
                else_schema: None,
            }));
        }

        RefOr::T(Schema::AllOf(schema)) => {
            obj.metadata = Some(Box::new(schemars::schema::Metadata {
                id: None,
                title: schema.title,
                description: schema.description,
                default: schema.default,
                deprecated: false,
                read_only: false,
                write_only: false,
                examples: schema
                    .example
                    .map_or_else(Vec::default, |example| vec![example]),
            }));

            assert_eq!(
                schema.discriminator, None,
                "discriminators are not supported in JSON Schema"
            );

            obj.subschemas = Some(Box::new(SubschemaValidation {
                all_of: Some(schema.items.into_iter().map(to_json_schema).collect()),
                any_of: None,
                one_of: None,
                not: None,
                if_schema: None,
                then_schema: None,
                else_schema: None,
            }));
        }

        RefOr::T(Schema::AnyOf(schema)) => {
            obj.metadata = Some(Box::new(schemars::schema::Metadata {
                id: None,
                title: None,
                description: schema.description,
                default: schema.default,
                deprecated: false,
                read_only: false,
                write_only: false,
                examples: schema
                    .example
                    .map_or_else(Vec::default, |example| vec![example]),
            }));

            assert_eq!(
                schema.discriminator, None,
                "discriminators are not supported in JSON Schema"
            );

            obj.subschemas = Some(Box::new(SubschemaValidation {
                all_of: None,
                any_of: Some(schema.items.into_iter().map(to_json_schema).collect()),
                one_of: None,
                not: None,
                if_schema: None,
                then_schema: None,
                else_schema: None,
            }));
        }

        _ => unimplemented!("unsupported schema type"),
    }

    schemars::schema::Schema::Object(obj)
}
