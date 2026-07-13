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

            // ignore_for_file: no_leading_underscores_for_local_identifiers, non_constant_identifier_names, unnecessary_string_interpolations, camel_case_types, empty_constructor_bodies, camel_case_extensions, unused_import

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

#[cfg(test)]
pub(crate) mod test_utils {
    use std::{path, process};

    pub(crate) fn run_test(name: &str, generate_model: bool, include_json: bool) {
        let test_name = path::Path::new(name).file_name().unwrap().to_str().unwrap();
        let test_path = path::Path::new("tests/dart").join(name);
        let generated_path = test_path.join("generated");

        if !std::fs::exists(&generated_path).unwrap() {
            std::fs::create_dir(&generated_path).unwrap();
        }

        let (termite, termite_types) = super::get_termite_dependency();
        std::fs::write(generated_path.join("termite.dart"), termite).unwrap();
        std::fs::write(generated_path.join("termite-types.dart"), termite_types).unwrap();

        if include_json {
            std::fs::write(
                generated_path.join("termite-json.dart"),
                super::get_json_interface(),
            )
            .unwrap();
        }

        if generate_model {
            let model_path = test_path.join(format!("{}_datamodel.yaml", test_name));
            let yaml_model = std::fs::read_to_string(model_path).unwrap();
            let model = crate::DataModel::import_yaml(&yaml_model).unwrap();
            let dart = model.get_dart(2).unwrap();
            std::fs::write(generated_path.join(format!("{}.dart", test_name)), dart).unwrap();
        }

        let output = if cfg!(target_os = "windows") {
            process::Command::new("cmd")
                .current_dir(&test_path)
                .arg("/C")
                .arg(format!("dart run {}_test.dart", test_name))
                .output()
                .expect("failed to run dart test")
        } else {
            process::Command::new("sh")
                .current_dir(&test_path)
                .arg("-c")
                .arg(format!("dart run {}_test.dart", test_name))
                .output()
                .expect("failed to run dart test")
        };

        if output.status.code().unwrap_or(1) != 0 || !output.stderr.is_empty() {
            panic!(
                "Dart test failed:\nstdout:\n{}\nstderr:\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dart::test_utils::*;

    #[test]
    fn termite() {
        run_test("termite", false, false);
    }

    #[test]
    fn termite_json() {
        run_test("termite_json", false, true);
    }

    #[test]
    fn header() {
        run_test("header", true, false);
    }

    #[test]
    fn footer() {
        run_test("footer", true, false);
    }

    #[test]
    fn namespace() {
        run_test("namespace", true, false);
    }

    #[test]
    fn outline() {
        run_test("outline", true, false);
    }

    #[test]
    fn full_example() {
        run_test("full_example", true, true);
    }
}
