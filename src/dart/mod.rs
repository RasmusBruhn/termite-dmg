//!
//! This module handles generation of Dart code to support a data model, it
//! includes the ability to create a source file, (de)serialization and
//! documentation.
//!
//! For any data model to work the termite dependency must be generated from
//! get_termite_dependency() and be saved as "termite.dart" and
//! "termite-types.dart" at a location where they can be included as "import
//! 'termite.dart';" and "import 'termite-types.dart';"
//!

use std::collections::{HashMap, HashSet};

use indoc::formatdoc;

use crate::data_model;

mod type_array;
mod type_constrained;
mod type_enum;
mod type_struct;
mod type_variant;

/// Obtains the base termite Dart dependencies required for all generated data
/// models, must be saved as "termite.dart" and "termite-types.dart"
pub fn get_termite_dependency() -> (&'static str, &'static str) {
    return (
        include_str!("termite.dart"),
        include_str!("termite-types.dart"),
    );
}

/// Obtains the JSON interface source for reading and writing json objects
pub fn get_json_interface() -> &'static str {
    return include_str!("termite-json.dart");
}

impl data_model::DataModel {
    /// Generates the Dart source code for the entire data model
    ///
    /// # Parameters
    ///
    /// indent: The number of spaces per indentation level
    pub fn get_dart<'a>(&self, indent: usize) -> Result<String, data_model::Error> {
        let header = if let Some(header) = self.headers.get("dart") {
            let value = data_model::expand_macros(
                &data_model::SerializationModel::Value(header.clone()),
                &self.macros,
                &mut HashSet::new(),
            )?;
            if let data_model::SerializationModel::Value(value) = value {
                value
            } else {
                return Err(data_model::Error {
                    location: "".to_string(),
                    error: data_model::ErrorCore::HeaderMacro(header.clone()),
                });
            }
        } else {
            "".to_string()
        };

        let footer = if let Some(footer) = self.footers.get("dart") {
            let value = data_model::expand_macros(
                &data_model::SerializationModel::Value(footer.clone()),
                &self.macros,
                &mut HashSet::new(),
            )?;
            if let data_model::SerializationModel::Value(value) = value {
                value
            } else {
                return Err(data_model::Error {
                    location: "".to_string(),
                    error: data_model::ErrorCore::FooterMacro(footer.clone()),
                });
            }
        } else {
            "".to_string()
        };

        let data_types = self
            .data_types
            .iter()
            .map(|data_type| data_type.get_dart(indent, &self.macros))
            .collect::<Result<Vec<String>, data_model::Error>>()?
            .join("\n\n");

        return Ok(formatdoc!(
            "
            // Generated with the Termite Data Model Generator

            // ignore_for_file: no_leading_underscores_for_local_identifiers

            import 'termite.dart' as termite;
            import 'termite-types.dart';

            {header}

            {data_types}

            {footer}
            "
        ));
    }
}

impl data_model::DataType {
    /// Generates the Dart source code for a the type
    ///
    /// # Parameters
    ///
    /// indent: The number of spaces per indentation level
    ///
    /// macros: The macros defined in the data model used for expanding default values
    pub fn get_dart<'a>(
        &self,
        indent: usize,
        macros: &'a HashMap<String, data_model::SerializationModel>,
    ) -> Result<String, data_model::Error> {
        let description = match &self.description {
            Some(description) => format!("/// {description}\n"),
            None => "".to_string(),
        };

        return Ok(format!(
            "{description}{data}",
            data = self.data.get_dart(&self.name, indent, macros)?
        ));
    }
}

impl data_model::DataTypeData {
    /// Generates the Dart source code for a the type data
    ///
    /// # Parameters
    ///
    /// name: The name of the type
    ///
    /// indent: The number of spaces per indentation level
    ///
    /// macros: The macros defined in the data model used for expanding default values
    fn get_dart<'a>(
        &self,
        name: &str,
        indent: usize,
        macros: &'a HashMap<String, data_model::SerializationModel>,
    ) -> Result<String, data_model::Error> {
        return match &self {
            data_model::DataTypeData::Enum(data) => Ok(data.get_dart(name, indent)),
            data_model::DataTypeData::Struct(data) => data.get_dart(name, indent, macros),
            data_model::DataTypeData::Variant(data) => Ok(data.get_dart(name, indent)),
            data_model::DataTypeData::Array(data) => Ok(data.get_dart(name, indent)),
            data_model::DataTypeData::ConstrainedType(data) => Ok(data.get_dart(name, indent)),
        };
    }
}
