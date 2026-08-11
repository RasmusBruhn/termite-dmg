//!
//! This module handles generation of c++ code to support a data model, it
//! includes the ability to create a header file, (de)serialization and
//! documentation.
//!
//! For any data model to work the termite dependency must be generated from
//! get_termite_dependency() and be saved as "termite.hpp" at a location where
//! it can be included as "#include <termite.hpp>"
//!

use crate::*;
use indoc::formatdoc;
use std::{
    char::ToLowercase,
    collections::{HashMap, HashSet},
};

mod error;
mod type_array;
mod type_constrained;
mod type_enum;
mod type_struct;
mod type_variant;
pub use error::{Error, ErrorCore};

/// Iterator to convert an iterator of chars to snake case converting all
/// uppercase characters to an underscore and the lowercase character
struct ToSnakeCase<'a> {
    /// The characters to convert to snake case
    chars: &'a mut dyn Iterator<Item = char>,
    /// The characters currently being converted to lowercase
    set_lower: Option<ToLowercase>,
}

impl<'a> ToSnakeCase<'a> {
    /// Creates a new ToSnakeCase object
    ///
    /// # Parameters
    ///
    /// chars: The iterator of the characters to convert
    fn new(chars: &'a mut dyn Iterator<Item = char>) -> Self {
        // Make sure the first character is lowercase without an underscore
        let set_lower = if let Some(first_char) = chars.next() {
            Some(first_char.to_lowercase())
        } else {
            None
        };

        return Self { chars, set_lower };
    }
}

impl<'a> Iterator for ToSnakeCase<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        // Set to lower case
        if let Some(set_lower) = &mut self.set_lower {
            // Get the next character
            if let Some(next_char) = set_lower.next() {
                return Some(next_char);
            }

            // Finish up setting to lowercase
            self.set_lower = None;
        }

        // Get next character
        return if let Some(next_char) = self.chars.next() {
            // Set to lowercase if it is uppercase
            if next_char.is_uppercase() {
                self.set_lower = Some(next_char.to_lowercase());
                Some('_')
            } else {
                Some(next_char)
            }
        } else {
            None
        };
    }
}

/// Obtains the base termite c++ dependency required for all generated data
/// models
pub fn get_termite_dependency() -> &'static str {
    return include_str!("termite.hpp");
}

/// Obtains the yaml-cpp interface header and source for reading and writing yaml files
pub fn get_yaml_interface() -> (&'static str, &'static str) {
    return (
        include_str!("termite-yaml.h"),
        include_str!("termite-yaml.cpp"),
    );
}

/// Obtains the nlohmann::json interface header and source for reading and writing json files
pub fn get_json_interface() -> (&'static str, &'static str) {
    return (
        include_str!("termite-json.h"),
        include_str!("termite-json.cpp"),
    );
}

/// Generates the header file
///
/// # Parameters
///
/// data: The data model to generate code for
///
/// name: The name of the header file (used for header guard so should be capslocked)
///
/// indent: The number of spaces to use for indentation
pub fn generate_header(data: &DataModel, name: &str, indent: usize) -> Result<String, Error> {
    // Sort the data types
    let data_types = sort::sort_data_types(&data.data_types)?;

    // Get the namespace
    let namespace = data.namespace.join("::");
    let namespace_begin = if namespace.is_empty() {
        format!("")
    } else {
        format!("namespace {namespace} {{")
    };
    let namespace_end = if namespace.is_empty() {
        format!("")
    } else {
        format!("}} // namespace {namespace}")
    };

    // Get all structs
    let data_type_definitions = data_types
        .iter()
        .map(|(type_name, data_type)| {
            data_type::generate_definition_header(data_type, type_name, indent)
        })
        .collect::<Vec<String>>()
        .join("\n\n");

    // Get all parsers
    let parsers = data_types
        .iter()
        .map(|(type_name, data_type)| {
            data_type::generate_parser_header(data_type, type_name, &data.namespace)
        })
        .collect::<Vec<String>>()
        .join("\n\n");

    // Expand macros in the header and footer
    let empty_string = String::new();
    let header_str = data.headers.get("cpp-header").unwrap_or(&empty_string);
    let header = match expand_macros(
        &SerializationModel::Value(header_str.clone()),
        &data.macros,
        &mut HashSet::new(),
    )? {
        SerializationModel::Value(value) => value,
        _ => {
            return Err(Error {
                location: "".to_string(),
                error: ErrorCore::HeaderMacro(header_str.clone()),
            })
        }
    };
    let footer_str = data.footers.get("cpp-header").unwrap_or(&empty_string);
    let footer = match expand_macros(
        &SerializationModel::Value(footer_str.clone()),
        &data.macros,
        &mut HashSet::new(),
    )? {
        SerializationModel::Value(value) => value,
        _ => {
            return Err(Error {
                location: "".to_string(),
                error: ErrorCore::FooterMacro(footer_str.clone()),
            })
        }
    };

    return Ok(formatdoc!(
        "
        // Generated with the Termite Data Model Generator
        #ifndef {name}_TERMITE_H_INCLUDED
        #define {name}_TERMITE_H_INCLUDED

        #include <iostream>
        #include <sstream>
        #include <optional>
        #include <variant>
        #include <algorithm>
        #include <termite.hpp>

        {header}

        {namespace_begin}

        {data_type_definitions}

        {namespace_end}

        namespace termite {{

        {parsers}

        }} // namespace termite
        
        {footer}
        
        #endif
        ",
    ));
}

/// Generates the source file
///
/// # Parameters
///
/// data: The data model to generate code for
///
/// name: The file location for the associated header file (is used for #include "name")
///
/// indent: The number of spaces to use for indentation
pub fn generate_source(data: &DataModel, name: &str, indent: usize) -> Result<String, Error> {
    // Sort the data types
    let data_types = sort::sort_data_types(&data.data_types)?;

    // Get the namespace
    let namespace = data.namespace.join("::");
    let namespace_begin = if namespace.is_empty() {
        format!("")
    } else {
        format!("namespace {namespace} {{")
    };
    let namespace_end = if namespace.is_empty() {
        format!("")
    } else {
        format!("}} // namespace {namespace}")
    };

    // Get all structs
    let data_type_definitions = data_types
        .iter()
        .map(|(type_name, data_type)| {
            data_type::generate_definition_source(data_type, type_name, &data.macros, indent)
        })
        .collect::<Result<Vec<_>, _>>()?
        .join("\n\n");

    // Get all parsers
    let parsers = data_types
        .iter()
        .map(|(type_name, data_type)| {
            data_type::generate_parser_source(data_type, type_name, indent, &data.namespace)
        })
        .collect::<Vec<String>>()
        .join("\n\n");

    // Expand macros in the header and footer
    let empty_string = String::new();
    let header_str = data.headers.get("cpp-source").unwrap_or(&empty_string);
    let header = match expand_macros(
        &SerializationModel::Value(header_str.clone()),
        &data.macros,
        &mut HashSet::new(),
    )? {
        SerializationModel::Value(value) => value,
        _ => {
            return Err(Error {
                location: "".to_string(),
                error: ErrorCore::HeaderMacro(header_str.clone()),
            })
        }
    };
    let footer_str = data.footers.get("cpp-source").unwrap_or(&empty_string);
    let footer = match expand_macros(
        &SerializationModel::Value(footer_str.clone()),
        &data.macros,
        &mut HashSet::new(),
    )? {
        SerializationModel::Value(value) => value,
        _ => {
            return Err(Error {
                location: "".to_string(),
                error: ErrorCore::FooterMacro(footer_str.clone()),
            })
        }
    };

    return Ok(formatdoc!("
        // Generated with the Termite Data Model Generator
        #include \"{name}.h\"

        {header}

        {namespace_begin}

        namespace {{

        // Code to make printing easier
        template <typename T, typename = void>
        struct has_insertion_operator : std::false_type {{}};
        template <typename T>
        struct has_insertion_operator<T, std::void_t<decltype(std::declval<std::ostream &>() << std::declval<T>())>> : std::true_type {{}};

        template <typename T>
        typename std::enable_if<has_insertion_operator<T>::value, std::ostream &>::type
        operator<<(std::ostream &os, const std::optional<T> &value) {{
        {0:indent$}if (value) {{
        {0:indent$}{0:indent$}return os << *value;
        {0:indent$}}} else {{
        {0:indent$}{0:indent$}return os << \"nullopt\";
        {0:indent$}}}
        }}

        template <typename T>
        typename std::enable_if<has_insertion_operator<T>::value, std::ostream &>::type
        operator<<(std::ostream &os, const std::vector<T> &value) {{
        {0:indent$}os << \"[ \";
        {0:indent$}for (auto value_it = value.cbegin(); value_it != value.cend(); ++value_it) {{
        {0:indent$}{0:indent$}if (value_it != value.cbegin()) {{
        {0:indent$}{0:indent$}{0:indent$}os << \", \";
        {0:indent$}{0:indent$}}}
        {0:indent$}{0:indent$}os << *value_it;
        {0:indent$}}}
        {0:indent$}return os << \" ]\";
        }}

        }} // namespace

        {data_type_definitions}

        {namespace_end}

        namespace termite {{
        
        {parsers}

        }} // namespace termite
        
        {footer}
        ",
        "",
    ));
}

mod data_type {
    use super::*;

    /// Converts the data type to a string for use in the header file
    ///
    /// # Parameters
    ///
    /// data: The data type to generate code for
    ///
    /// name: The name of the data type
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn generate_definition_header(data: &DataType, name: &str, indent: usize) -> String {
        return formatdoc!(
            "
            /**
             * @brief {description}
             * 
             */
            {definition}",
            description = get_description(data),
            definition = data_type_data::generate_definition_header(&data.data, name, indent),
        );
    }

    /// Converts the data type to a string for use in the source file
    ///
    /// # Parameters
    ///
    /// data: The data type to generate code for
    ///
    /// name: The name of the data type
    ///
    /// macros: A map of all macros to expand default values
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn generate_definition_source(
        data: &DataType,
        name: &str,
        macros: &HashMap<String, SerializationModel>,
        indent: usize,
    ) -> Result<String, Error> {
        return Ok(formatdoc!(
            "
            {definition}",
            definition =
                data_type_data::generate_definition_source(&data.data, name, macros, indent)?,
        ));
    }

    /// Gets the header code for the parser for this type allowing it to be read from a file
    ///
    /// # Parameters
    ///
    /// data: The data type to generate code for
    ///
    /// name: The name of the data type
    ///
    /// namespace: The namespace of the type
    pub(super) fn generate_parser_header(
        data: &DataType,
        name: &str,
        namespace: &[String],
    ) -> String {
        return data_type_data::generate_parser_header(&data.data, name, namespace);
    }

    /// Gets the source code for the parser for this type allowing it to be read from a file
    ///
    /// # Parameters
    ///
    /// data: The data type to generate code for
    ///
    /// name: The name of the data type
    ///
    /// indent: The number of spaces to use for indentation
    ///
    /// namespace: The namespace of the type
    pub(super) fn generate_parser_source(
        data: &DataType,
        name: &str,
        indent: usize,
        namespace: &[String],
    ) -> String {
        return data_type_data::generate_parser_source(&data.data, name, indent, namespace);
    }

    /// Generates the description if it is supplied
    ///
    /// # Parameters
    ///
    /// data: The data type to generate code for
    fn get_description(data: &DataType) -> String {
        return match &data.description {
            Some(description) => description.clone(),
            None => "".to_string(),
        };
    }
}

mod data_type_data {
    use super::*;

    /// Converts the data type data to a string for use in the header file
    ///
    /// # Parameters
    ///
    /// data: The data type data to generate code for
    ///
    /// name: The name of the data type
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn generate_definition_header(
        data: &DataTypeData,
        name: &str,
        indent: usize,
    ) -> String {
        return match data {
            DataTypeData::Struct(data) => {
                type_struct::generate_definition_header(data, name, indent)
            }
            DataTypeData::Array(data) => type_array::generate_definition_header(data, name, indent),
            DataTypeData::Variant(data) => {
                type_variant::generate_definition_header(data, name, indent)
            }
            DataTypeData::Enum(data) => type_enum::generate_definition_header(data, name, indent),
            DataTypeData::ConstrainedType(data) => {
                type_constrained::generate_definition_header(data, name, indent)
            }
        };
    }

    /// Converts the data type data to a string for use in the source file
    ///
    /// # Parameters
    ///
    /// data: The data type data to generate code for
    ///
    /// name: The name of the data type
    ///
    /// macros: A map of all macros to expand default values
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn generate_definition_source(
        data: &DataTypeData,
        name: &str,
        macros: &HashMap<String, SerializationModel>,
        indent: usize,
    ) -> Result<String, Error> {
        return match data {
            DataTypeData::Struct(data) => Ok(type_struct::generate_definition_source(
                data, name, macros, indent,
            )?),
            DataTypeData::Array(data) => {
                Ok(type_array::generate_definition_source(data, name, indent))
            }
            DataTypeData::Variant(data) => {
                Ok(type_variant::generate_definition_source(data, name, indent))
            }
            DataTypeData::Enum(data) => {
                Ok(type_enum::generate_definition_source(data, name, indent))
            }
            DataTypeData::ConstrainedType(data) => Ok(
                type_constrained::generate_definition_source(data, name, indent),
            ),
        };
    }

    /// Gets the header code for the parser for this type allowing it to be read from a file
    ///
    /// # Parameters
    ///
    /// data: The data type data to generate code for
    ///
    /// name: The name of the type
    ///
    /// namespace: The namespace of the type
    pub(super) fn generate_parser_header(
        data: &DataTypeData,
        name: &str,
        namespace: &[String],
    ) -> String {
        return match data {
            DataTypeData::Struct(data) => {
                type_struct::generate_parser_header(data, name, namespace)
            }
            DataTypeData::Array(data) => type_array::generate_parser_header(data, name, namespace),
            DataTypeData::Variant(data) => {
                type_variant::generate_parser_header(data, name, namespace)
            }
            DataTypeData::Enum(data) => type_enum::generate_parser_header(data, name, namespace),
            DataTypeData::ConstrainedType(data) => {
                type_constrained::generate_parser_header(data, name, namespace)
            }
        };
    }

    /// Gets the source code for the parser for this type allowing it to be read from a file
    ///
    /// # Parameters
    ///
    /// data: The data type data to generate code for
    ///
    /// name: The name of the type
    ///
    /// indent: The number of spaces to use for indentation
    ///
    /// namespace: The namespace of the type
    pub(super) fn generate_parser_source(
        data: &DataTypeData,
        name: &str,
        indent: usize,
        namespace: &[String],
    ) -> String {
        return match data {
            DataTypeData::Struct(data) => {
                type_struct::generate_parser_source(data, name, indent, namespace)
            }
            DataTypeData::Array(data) => {
                type_array::generate_parser_source(data, name, indent, namespace)
            }
            DataTypeData::Variant(data) => {
                type_variant::generate_parser_source(data, name, indent, namespace)
            }
            DataTypeData::Enum(data) => {
                type_enum::generate_parser_source(data, name, indent, namespace)
            }
            DataTypeData::ConstrainedType(data) => {
                type_constrained::generate_parser_source(data, name, indent, namespace)
            }
        };
    }
}

#[cfg(test)]
pub(crate) mod test_utils {
    use super::*;
    use std::{path, process};

    pub(crate) fn run_test(
        name: &str,
        generate_model: bool,
        include_yaml: bool,
        include_json: bool,
    ) {
        // Get the path to the test directory
        let test_name = path::Path::new(name).file_name().unwrap().to_str().unwrap();
        let test_path = path::Path::new("tests/cpp").join(name);
        let include_yaml = if include_yaml { "true" } else { "false" };
        let include_json = if include_json { "true" } else { "false" };
        let termite_path = (0..(test_path.components().count() + 1))
            .map(|_| "..".to_string())
            .chain(vec!["src".to_string(), "cpp".to_string()].into_iter())
            .collect::<Vec<String>>()
            .join("/");

        // Construct the folder for the generated files
        let generated_path = test_path.join("generated");
        if !std::fs::exists(&generated_path).unwrap() {
            std::fs::create_dir(&generated_path).unwrap();
        }

        // Generate the code
        if generate_model {
            let model_path = test_path.join(format!("{}_datamodel.yaml", test_name));
            let yaml_model = std::fs::read_to_string(model_path).unwrap();
            let model = crate::DataModel::import_yaml(&yaml_model).unwrap();

            // Create the header file
            let header_path = generated_path.join(format!("{}.h", test_name));
            let source_path = generated_path.join(format!("{}.cpp", test_name));
            let header_file = generate_header(&model, &test_name.to_uppercase(), 2).unwrap();
            let source_file = generate_source(&model, test_name, 2).unwrap();
            std::fs::write(header_path, &header_file).unwrap();
            std::fs::write(source_path, &source_file).unwrap();
        }

        // Create the cmake file
        let cmake_path = generated_path.join("CMakeLists.txt");
        let cmake_raw = include_str!("../../tests/cpp/CMakeLists.txt");
        let cmake_file = cmake_raw
            .replace("%%TEST_NAME%%", test_name)
            .replace("%%INCLUDE_YAML%%", include_yaml)
            .replace("%%INCLUDE_JSON%%", include_json)
            .replace("%%TERMITE_PATH%%", &termite_path);
        std::fs::write(cmake_path, cmake_file).unwrap();

        // Compile c++ code
        if cfg!(target_os = "windows") {
            process::Command::new("cmd")
                .current_dir(&test_path)
                .arg("/C")
                .arg("mkdir build")
                .output()
                .expect("failed to compile");
        } else {
            process::Command::new("sh")
                .current_dir(&test_path)
                .arg("-c")
                .arg("mkdir build")
                .output()
                .expect("failed to compile");
        };

        let build_path = test_path.join("build");
        let compile_output = if cfg!(target_os = "windows") {
            process::Command::new("cmd")
                .current_dir(&build_path)
                .arg("/C")
                .arg("cmake ../generated")
                .output()
                .expect("failed to compile")
        } else {
            process::Command::new("sh")
                .current_dir(&build_path)
                .arg("-c")
                .arg("cmake ../generated")
                .output()
                .expect("failed to compile")
        };

        if compile_output.status.code().unwrap_or(1) != 0 || !compile_output.stderr.is_empty() {
            panic!(
                "C++ test failed:\nstdout:\n{}\nstderr:\n{}",
                String::from_utf8_lossy(&compile_output.stdout),
                String::from_utf8_lossy(&compile_output.stderr),
            );
        }

        let compile_output2 = if cfg!(target_os = "windows") {
            process::Command::new("cmd")
                .current_dir(&build_path)
                .arg("/C")
                .arg("cmake --build .")
                .output()
                .expect("failed to compile")
        } else {
            process::Command::new("sh")
                .current_dir(&build_path)
                .arg("-c")
                .arg("cmake --build .")
                .output()
                .expect("failed to compile")
        };

        if compile_output2.status.code().unwrap_or(1) != 0 || !compile_output2.stderr.is_empty() {
            panic!(
                "C++ test failed:\nstdout:\n{}\nstderr:\n{}",
                String::from_utf8_lossy(&compile_output2.stdout),
                String::from_utf8_lossy(&compile_output2.stderr),
            );
        }

        // Tet the c++ code
        let test_output = if cfg!(target_os = "windows") {
            process::Command::new("cmd")
                .current_dir(&build_path)
                .arg("/C")
                .arg(format!(".\\Debug\\{}.exe", &test_name))
                .output()
                .expect("failed to test")
        } else {
            process::Command::new("sh")
                .current_dir(&build_path)
                .arg("-c")
                .arg(format!("./{}", &test_name))
                .output()
                .expect("failed to test")
        };

        if test_output.status.code().unwrap_or(1) != 0 {
            panic!(
                "C++ test failed:\nstdout:\n{}\nstderr:\n{}",
                String::from_utf8_lossy(&test_output.stdout),
                String::from_utf8_lossy(&test_output.stderr),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpp::test_utils::*;

    #[test]
    fn termite() {
        run_test("termite", false, false, false);
    }

    #[test]
    fn termite_yaml() {
        run_test("termite_yaml", false, true, false);
    }

    #[test]
    fn termite_json() {
        run_test("termite_json", false, false, true);
    }

    #[test]
    fn header() {
        run_test("header", true, false, false);
    }

    #[test]
    fn footer() {
        run_test("footer", true, false, false);
    }

    #[test]
    fn namespace() {
        run_test("namespace", true, false, false);
    }

    #[test]
    fn outline() {
        run_test("outline", true, false, false);
    }

    #[test]
    fn full_example() {
        run_test("full_example", true, true, false);
    }
}
