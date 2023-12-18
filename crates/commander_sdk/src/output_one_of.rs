use std::{fs::File, io::Write};

use to_snake_case::ToSnakeCase;

use crate::{
    output_struct::{FieldsModel, TypeModel},
    INDENT,
};

#[derive(Default)]
pub struct OneOfModel {
    pub name: String,
    pub items: Vec<OneOfItem>,
}

pub struct OneOfItem {
    pub name: String,
    pub ty: OneOfItemType,
}

pub enum OneOfItemType {
    Ref(String),
    Type(TypeModel),
    Struct(FieldsModel),
    OneOf(OneOfModel),
}

impl OneOfModel {
    pub fn write_to_file(&self, file: &mut File) -> std::io::Result<()> {
        println!("writing {}", self.name);

        writeln!(file, r#"use serde::Deserialize;"#)?;
        writeln!(file)?;
        writeln!(file, r#"#[derive(Debug, Deserialize)]"#)?;
        writeln!(file, r#"pub enum {} {{"#, self.name)?;

        for item in &self.items {
            write!(file, "{}", INDENT)?;
            write!(file, "{}", item.name)?;

            match &item.ty {
                OneOfItemType::Ref(ref_name) => {
                    write!(file, "({})", ref_name)?;
                }

                OneOfItemType::Type(ty_model) => {
                    write!(file, "(")?;
                    ty_model.write_to_file(file)?;
                    write!(file, ")")?;
                }

                OneOfItemType::Struct(fields_model) => {
                    writeln!(file, " {{")?;

                    for (field_name, field_ty) in fields_model {
                        write!(file, "{}", INDENT)?;
                        write!(file, "{}", INDENT)?;
                        write!(file, "{}: ", field_name.to_snake_case())?;
                        field_ty.write_to_file(file)?;
                        writeln!(file, ",")?;
                    }

                    write!(file, "{}", INDENT)?;
                    write!(file, "}}")?;
                }

                OneOfItemType::OneOf(_) => todo!(),
            }

            // write!(file, "pub {}: ", field_name)?;
            // field_ty.write_to_file(file)?;
            writeln!(file, ",")?;
        }

        writeln!(file, r#"}}"#)?;

        Ok(())
    }
}
