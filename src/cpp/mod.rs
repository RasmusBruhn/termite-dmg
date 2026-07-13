//!
//! This module handles generation of c++ code to support a data model, it
//! includes the ability to create a header file, (de)serialization and
//! documentation.
//!
//! For any data model to work the termite dependency must be generated from
//! get_termite_dependency() and be saved as "termite.hpp" at a location where
//! it can be included as "#include <termite.hpp>"
//!

use indoc::formatdoc;
use std::{
    char::ToLowercase,
    collections::{HashMap, HashSet},
    fmt,
};

mod type_array;
mod type_constrained;
mod type_enum;
mod type_struct;
mod type_variant;

use type_array::Array;
use type_constrained::ConstrainedType;
use type_enum::Enum;
use type_struct::Struct;
use type_variant::Variant;

use crate::data_model;

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

/// An entire data model
#[derive(Clone, Debug, PartialEq)]
pub struct DataModel {
    /// List of the the data types to implement
    data_types: Vec<DataType>,
    /// List of all header data used to include external packages
    headers: Headers,
    /// List of all footer data
    footers: Footers,
    /// The nested namespace to put the data model into
    namespace: Vec<String>,
    /// A map of all macros to expand default values
    macros: HashMap<String, data_model::SerializationModel>,
}

impl DataModel {
    /// Constructs a new c++ data model from a generic data model
    ///
    /// # Parameters
    ///
    /// data: The generic data type to convert
    pub fn new(data: crate::DataModel) -> Result<Self, Error> {
        let data_types = data
            .data_types
            .into_iter()
            .enumerate()
            .map(|(i, data_type)| {
                return match DataType::new(data_type) {
                    Ok(result) => Ok(result),
                    Err(error) => Err(error.add_element("data_types", i)),
                };
            })
            .collect::<Result<Vec<DataType>, Error>>()?;
        let headers = match Headers::new(data.headers) {
            Ok(result) => result,
            Err(error) => return Err(error.add_field("headers")),
        };
        let footers = match Footers::new(data.footers) {
            Ok(result) => result,
            Err(error) => return Err(error.add_field("footers")),
        };

        return Ok(Self {
            data_types,
            headers,
            footers,
            namespace: data.namespace,
            macros: data.macros,
        });
    }

    /// Generates the header file
    ///
    /// # Parameters
    ///
    /// name: The name of the header file (used for header guard so should be capslocked)
    ///
    /// indent: The number of spaces to use for indentation
    pub fn get_header(&self, name: &str, indent: usize) -> Result<String, Error> {
        // Get the namespace
        let namespace = self.namespace.join("::");
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
        let data_types = self
            .data_types
            .iter()
            .map(|data_type| data_type.get_definition_header(indent))
            .collect::<Vec<String>>()
            .join("\n\n");

        // Get all parsers
        let parsers = self
            .data_types
            .iter()
            .map(|data_type| data_type.get_parser_header(&self.namespace))
            .collect::<Vec<String>>()
            .join("\n\n");

        // Expand macros in the header and footer
        let header = match data_model::expand_macros(
            &data_model::SerializationModel::Value(self.headers.header.clone()),
            &self.macros,
            &mut HashSet::new(),
        )? {
            data_model::SerializationModel::Value(value) => value,
            _ => {
                return Err(Error {
                    location: "".to_string(),
                    error: ErrorCore::HeaderMacro(self.headers.header.clone()),
                })
            }
        };
        let footer = match data_model::expand_macros(
            &data_model::SerializationModel::Value(self.footers.header.clone()),
            &self.macros,
            &mut HashSet::new(),
        )? {
            data_model::SerializationModel::Value(value) => value,
            _ => {
                return Err(Error {
                    location: "".to_string(),
                    error: ErrorCore::FooterMacro(self.footers.header.clone()),
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

            {data_types}

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
    /// name: The file location for the associated header file (is used for #include "name")
    ///
    /// indent: The number of spaces to use for indentation
    pub fn get_source(&self, name: &str, indent: usize) -> Result<String, Error> {
        // Get the namespace
        let namespace = self.namespace.join("::");
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
        let data_types = self
            .data_types
            .iter()
            .map(|data_type| data_type.get_definition_source(&self.macros, indent))
            .collect::<Result<Vec<_>, _>>()?
            .join("\n\n");

        // Get all parsers
        let parsers = self
            .data_types
            .iter()
            .map(|data_type| data_type.get_parser_source(indent, &self.namespace, &self.data_types))
            .collect::<Vec<String>>()
            .join("\n\n");

        // Expand macros in the header and footer
        let header = match data_model::expand_macros(
            &data_model::SerializationModel::Value(self.headers.source.clone()),
            &self.macros,
            &mut HashSet::new(),
        )? {
            data_model::SerializationModel::Value(value) => value,
            _ => {
                return Err(Error {
                    location: "".to_string(),
                    error: ErrorCore::HeaderMacro(self.headers.source.clone()),
                })
            }
        };
        let footer = match data_model::expand_macros(
            &data_model::SerializationModel::Value(self.footers.source.clone()),
            &self.macros,
            &mut HashSet::new(),
        )? {
            data_model::SerializationModel::Value(value) => value,
            _ => {
                return Err(Error {
                    location: "".to_string(),
                    error: ErrorCore::FooterMacro(self.footers.source.clone()),
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

            {data_types}

            {namespace_end}

            namespace termite {{
            
            {parsers}

            }} // namespace termite
            
            {footer}
            ",
            "",
        ));
    }
}

/// All of the headers for the different files
#[derive(Clone, Debug, PartialEq)]
struct Headers {
    /// For the header file
    header: String,
    /// For the source file
    source: String,
}

impl Headers {
    /// Constructs a new c++ header from a generic header
    ///
    /// # Parameters
    ///
    /// data: The generic data type to convert
    fn new(mut data: HashMap<String, String>) -> Result<Self, Error> {
        let source = match data.remove("cpp-source") {
            Some(value) => value,
            None => String::new(),
        };
        let header = match data.remove("cpp-header") {
            Some(value) => value,
            None => String::new(),
        };

        return Ok(Self { header, source });
    }
}

/// All of the footers for the different files
#[derive(Clone, Debug, PartialEq)]
struct Footers {
    /// For the header file
    header: String,
    /// For the source file
    source: String,
}

impl Footers {
    /// Constructs a new c++ footer from a generic footer
    ///
    /// # Parameters
    ///
    /// data: The generic data type to convert
    fn new(mut data: HashMap<String, String>) -> Result<Self, Error> {
        let source = match data.remove("cpp-source") {
            Some(value) => value,
            None => String::new(),
        };
        let header = match data.remove("cpp-header") {
            Some(value) => value,
            None => String::new(),
        };

        return Ok(Self { header, source });
    }
}

/// Any data type (struct, variant, ect.)
#[derive(Clone, Debug, PartialEq)]
struct DataType {
    /// The name of the type
    name: String,
    /// The description of the type
    description: Option<String>,
    /// The type specific data
    data: DataTypeData,
}

impl DataType {
    /// Constructs a new c++ data type from a generic data type
    ///
    /// # Parameters
    ///
    /// data: The generic data type to convert
    fn new(data: crate::DataType) -> Result<Self, Error> {
        // Convert the data
        let processed_data = match DataTypeData::new(data.data) {
            Ok(data) => data,
            Err(error) => return Err(error.add_field(&data.name)),
        };

        return Ok(Self {
            name: data.name,
            description: data.description,
            data: processed_data,
        });
    }

    /// Generates the description if it is supplied
    fn get_description(&self) -> String {
        return match &self.description {
            Some(description) => description.clone(),
            None => "".to_string(),
        };
    }

    /// Converts the data type to a string for use in the header file
    ///
    /// # Parameters
    ///
    /// indent: The number of spaces to use for indentation
    fn get_definition_header(&self, indent: usize) -> String {
        return formatdoc!(
            "
            /**
             * @brief {description}
             * 
             */
            {definition}",
            description = self.get_description(),
            definition = self.data.get_definition_header(&self.name, indent),
        );
    }

    /// Converts the data type to a string for use in the source file
    ///
    /// # Parameters
    ///
    /// macros: A map of all macros to expand default values
    ///
    /// indent: The number of spaces to use for indentation
    fn get_definition_source(
        &self,
        macros: &HashMap<String, data_model::SerializationModel>,
        indent: usize,
    ) -> Result<String, Error> {
        return Ok(formatdoc!(
            "
            {definition}",
            definition = self
                .data
                .get_definition_source(&self.name, macros, indent)?,
        ));
    }

    /// Gets the header code for the parser for this type allowing it to be read from a file
    ///
    /// # Parameters
    ///
    /// namespace: The namespace of the type
    pub(super) fn get_parser_header(&self, namespace: &[String]) -> String {
        return self.data.get_parser_header(&self.name, namespace);
    }

    /// Gets the source code for the parser for this type allowing it to be read from a file
    ///
    /// # Parameters
    ///
    /// indent: The number of spaces to use for indentation
    ///
    /// namespace: The namespace of the type
    ///
    /// data_types: List of all the data types defined in the data model
    pub(super) fn get_parser_source(
        &self,
        indent: usize,
        namespace: &[String],
        data_types: &[DataType],
    ) -> String {
        return self
            .data
            .get_parser_source(&self.name, indent, namespace, data_types);
    }
}

/// Supplies the type specific information for a data type
#[derive(Clone, Debug, PartialEq)]
enum DataTypeData {
    /// Describes a struct
    Struct(Struct),
    /// Describes an array
    Array(Array),
    /// Describes a variant
    Variant(Variant),
    /// Describes an enum
    Enum(Enum),
    /// Describes a constrained type
    ConstrainedType(ConstrainedType),
}

impl DataTypeData {
    /// Constructs a new c++ data type data from a generic data type data
    ///
    /// # Parameters
    ///
    /// data: The generic data type data to convert
    fn new(data: crate::DataTypeData) -> Result<Self, Error> {
        let result = match data {
            crate::DataTypeData::Struct(data) => DataTypeData::Struct(Struct::new(data)?),
            crate::DataTypeData::Array(data) => DataTypeData::Array(Array::new(data)?),
            crate::DataTypeData::Variant(data) => DataTypeData::Variant(Variant::new(data)?),
            crate::DataTypeData::Enum(data) => DataTypeData::Enum(Enum::new(data)?),
            crate::DataTypeData::ConstrainedType(data) => {
                DataTypeData::ConstrainedType(ConstrainedType::new(data)?)
            }
        };

        return Ok(result);
    }

    /// Converts the data type data to a string for use in the header file
    ///
    /// # Parameters
    ///
    /// name: The name of the data type
    ///
    /// indent: The number of spaces to use for indentation
    fn get_definition_header(&self, name: &str, indent: usize) -> String {
        return match self {
            DataTypeData::Struct(data) => data.get_definition_header(name, indent),
            DataTypeData::Array(data) => data.get_definition_header(name, indent),
            DataTypeData::Variant(data) => data.get_definition_header(name, indent),
            DataTypeData::Enum(data) => data.get_definition_header(name, indent),
            DataTypeData::ConstrainedType(data) => data.get_definition_header(name, indent),
        };
    }

    /// Converts the data type data to a string for use in the source file
    ///
    /// # Parameters
    ///
    /// name: The name of the data type
    ///
    /// macros: A map of all macros to expand default values
    ///
    /// indent: The number of spaces to use for indentation
    fn get_definition_source(
        &self,
        name: &str,
        macros: &HashMap<String, data_model::SerializationModel>,
        indent: usize,
    ) -> Result<String, Error> {
        return match self {
            DataTypeData::Struct(data) => data.get_definition_source(name, macros, indent),
            DataTypeData::Array(data) => Ok(data.get_definition_source(name, indent)),
            DataTypeData::Variant(data) => Ok(data.get_definition_source(name, indent)),
            DataTypeData::Enum(data) => Ok(data.get_definition_source(name, indent)),
            DataTypeData::ConstrainedType(data) => Ok(data.get_definition_source(name, indent)),
        };
    }

    /// Gets the header code for the parser for this type allowing it to be read from a file
    ///
    /// # Parameters
    ///
    /// name: The name of the type
    ///
    /// namespace: The namespace of the type
    pub(super) fn get_parser_header(&self, name: &str, namespace: &[String]) -> String {
        return match self {
            DataTypeData::Struct(data) => data.get_parser_header(name, namespace),
            DataTypeData::Array(data) => data.get_parser_header(name, namespace),
            DataTypeData::Variant(data) => data.get_parser_header(name, namespace),
            DataTypeData::Enum(data) => data.get_parser_header(name, namespace),
            DataTypeData::ConstrainedType(data) => data.get_parser_header(name, namespace),
        };
    }

    /// Gets the source code for the parser for this type allowing it to be read from a file
    ///
    /// # Parameters
    ///
    /// name: The name of the type
    ///
    /// indent: The number of spaces to use for indentation
    ///
    /// namespace: The namespace of the type
    ///
    /// data_types: List of all the data types defined in the data model
    pub(super) fn get_parser_source(
        &self,
        name: &str,
        indent: usize,
        namespace: &[String],
        data_types: &[DataType],
    ) -> String {
        return match self {
            DataTypeData::Struct(data) => {
                data.get_parser_source(name, indent, namespace, data_types)
            }
            DataTypeData::Array(data) => {
                data.get_parser_source(name, indent, namespace, data_types)
            }
            DataTypeData::Variant(data) => {
                data.get_parser_source(name, indent, namespace, data_types)
            }
            DataTypeData::Enum(data) => data.get_parser_source(name, indent, namespace, data_types),
            DataTypeData::ConstrainedType(data) => {
                data.get_parser_source(name, indent, namespace, data_types)
            }
        };
    }
}

/// Errors for when converting generic data models into c++ data models
/// including location
#[derive(Debug, Clone)]
pub struct Error {
    /// The location where the error occured
    pub location: String,
    /// The actual error that occured
    pub error: ErrorCore,
}

impl Error {
    /// Sets the current location to be the field of the given base
    ///
    /// # Parameters
    ///
    /// base: The base to set in the location
    fn add_field(self, base: &str) -> Error {
        let location = if !self.location.is_empty() {
            format!("{}.{}", base, self.location)
        } else {
            base.to_string()
        };

        return Error {
            location,
            error: self.error,
        };
    }

    /// Sets the current location to be the element of a field of the given base
    ///
    /// # Parameters
    ///
    /// base: The base to set in the location
    ///
    /// index: The index of the field
    fn add_element(self, base: &str, index: usize) -> Error {
        let location = if !self.location.is_empty() {
            format!("{}[{}].{}", base, index, self.location)
        } else {
            format!("{}[{}]", base, index)
        };

        return Error {
            location,
            error: self.error,
        };
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}: {}", self.location, self.error);
    }
}

impl From<data_model::Error> for Error {
    fn from(value: data_model::Error) -> Self {
        return Error {
            location: value.location.clone(),
            error: ErrorCore::MacroError(value),
        };
    }
}

/// Errors for when converting generic data models into c++ data models
#[derive(thiserror::Error, Debug, Clone)]
pub enum ErrorCore {
    /// Error expanding macros
    #[error("An error occured when expanding macros: {:?}", .0)]
    MacroError(data_model::Error),
    /// The macro expansion in the header failed
    #[error("The header \"{:?}\" must only expand to a string when using macros", .0)]
    HeaderMacro(String),
    /// The macro expansion in the footer failed
    #[error("The footer \"{:?}\" must only expand to a string when using macros", .0)]
    FooterMacro(String),
}

#[cfg(test)]
pub(crate) mod test_utils {
    use super::DataModel;
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
            let data_model = DataModel::new(model).unwrap();

            // Create the header file
            let header_path = generated_path.join(format!("{}.h", test_name));
            let source_path = generated_path.join(format!("{}.cpp", test_name));
            let header_file = data_model.get_header(&test_name.to_uppercase(), 2).unwrap();
            let source_file = data_model.get_source(test_name, 2).unwrap();
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
