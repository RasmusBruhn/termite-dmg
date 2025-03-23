//!
//! This module handles generation of c++ code to support a data model, it
//! includes the ability to create a header file, (de)serialization and
//! documentation.
//!
//! For any data model to work the termite dependency must be generatred from
//! get_termite_dependency() and be saved as "termite.hpp" at a location where
//! it can be included as "#include <termite.hpp>"
//!

use indoc::formatdoc;
use std::{char::ToLowercase, collections::HashMap, fmt};

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

/// Obtains the yaml-cpp interface header for reading and writing yaml files
pub fn get_yaml_interface() -> &'static str {
    return include_str!("termite-yaml.hpp");
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
        });
    }

    /// Generates the header file
    ///
    /// # Parameters
    ///
    /// name: The name of the header file (used for header guard so should be capslocked)
    ///
    /// indent: The number of spaces to use for indentation
    pub fn get_header(&self, name: &str, indent: usize) -> String {
        // Get the namespace
        let namespace = self.namespace.join("::");

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

        return formatdoc!(
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

            namespace {namespace} {{

            {data_types}

            }} // namespace {namespace}

            namespace termite {{

            {parsers}

            }} // namespace termite
            
            {footer}
            
            #endif
            ",
            header = self.headers.header,
            footer = self.footers.header,
        );
    }

    /// Generates the source file
    ///
    /// # Parameters
    ///
    /// name: The file location for the associated header file (is used for #include "name")
    ///
    /// indent: The number of spaces to use for indentation
    pub fn get_source(&self, name: &str, indent: usize) -> String {
        // Get the namespace
        let namespace = self.namespace.join("::");

        // Get all structs
        let data_types = self
            .data_types
            .iter()
            .map(|data_type| data_type.get_definition_source(indent))
            .collect::<Vec<String>>()
            .join("\n\n");

        // Get all parsers
        let parsers = self
            .data_types
            .iter()
            .map(|data_type| data_type.get_parser_source(indent, &self.namespace, &self.data_types))
            .collect::<Vec<String>>()
            .join("\n\n");

        return formatdoc!("
            // Generated with the Termite Data Model Generator
            #include \"{name}.h\"

            {header}

            namespace {namespace} {{

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

            }} // namespace {namespace}

            namespace termite {{
            
            {parsers}

            }} // namespace termite
            
            {footer}
            ",
            "",
            header = self.headers.source,
            footer = self.footers.source,
        );
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
    /// indent: The number of spaces to use for indentation
    fn get_definition_source(&self, indent: usize) -> String {
        return formatdoc!(
            "
            {definition}",
            definition = self.data.get_definition_source(&self.name, indent),
        );
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
    /// indent: The number of spaces to use for indentation
    fn get_definition_source(&self, name: &str, indent: usize) -> String {
        return match self {
            DataTypeData::Struct(data) => data.get_definition_source(name, indent),
            DataTypeData::Array(data) => data.get_definition_source(name, indent),
            DataTypeData::Variant(data) => data.get_definition_source(name, indent),
            DataTypeData::Enum(data) => data.get_definition_source(name, indent),
            DataTypeData::ConstrainedType(data) => data.get_definition_source(name, indent),
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

/// Errors for when converting generic data models into c++ data models
#[derive(thiserror::Error, Debug, Clone)]
pub enum ErrorCore {
    /// No error has occured
    #[error("No error has occured")]
    NoError(),
}

#[cfg(test)]
pub(crate) mod test_utils {
    use std::{fs, path, process};

    pub(crate) fn str_diff(lhs: &str, rhs: &str) -> Option<(usize, String, String)> {
        if let Some(error) = lhs
            .trim()
            .lines()
            .zip(rhs.trim().lines())
            .enumerate()
            .filter_map(|(index, (lhs, rhs))| {
                return if lhs.trim() == rhs.trim() {
                    None
                } else {
                    Some((index + 1, lhs.trim().to_string(), rhs.trim().to_string()))
                };
            })
            .next()
        {
            return Some(error);
        }

        if lhs.trim().lines().count() != rhs.trim().lines().count() {
            return Some((
                0,
                format!("{}", lhs.trim().lines().count()),
                format!("{}", rhs.trim().lines().count()),
            ));
        }

        return None;
    }

    fn get_source_path(name: &str) -> path::PathBuf {
        // Get the filename
        let filename = path::Path::new(name).file_name().unwrap().to_str().unwrap();

        return path::Path::new("tests/cpp")
            .join(format!("{name}"))
            .join(format!("{filename}.cpp"));
    }

    fn get_test_path(name: &str) -> path::PathBuf {
        // Get the filename
        let filename = path::Path::new(name).file_name().unwrap().to_str().unwrap();

        return path::Path::new("tests/cpp")
            .join(format!("{name}"))
            .join(format!("{filename}_test.cpp"));
    }

    fn get_exe_path(name: &str) -> path::PathBuf {
        // Get the filename
        let filename = path::Path::new(name).file_name().unwrap().to_str().unwrap();

        return path::Path::new("target/tests/cpp")
            .join(format!("{name}"))
            .join(filename);
    }

    pub(crate) fn compile_and_test(name: &str) {
        // Get the paths
        let source_path = get_source_path(name);
        let test_path = get_test_path(name);
        let exe_path = get_exe_path(name);

        // Create the output directory
        fs::create_dir_all(exe_path.parent().unwrap()).unwrap();

        // Compile code
        let compile_output = if cfg!(target_os = "windows") {
            process::Command::new("cmd")
                .arg("/C")
                .arg(format!(
                    "g++ {} {} -Isrc/cpp -Wall -std=c++17 -o {}.exe",
                    source_path.to_str().unwrap(),
                    test_path.to_str().unwrap(),
                    exe_path.to_str().unwrap()
                ))
                .output()
                .expect("failed to compile")
        } else {
            process::Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "g++ {} {} -Isrc/cpp -Wall -std=c++17 -o {}",
                    source_path.to_str().unwrap(),
                    test_path.to_str().unwrap(),
                    exe_path.to_str().unwrap()
                ))
                .output()
                .expect("failed to compile")
        };
        println!(
            "g++ {} {} -Isrc/cpp -Wall -std=c++17 -o {}.exe",
            source_path.to_str().unwrap(),
            test_path.to_str().unwrap(),
            exe_path.to_str().unwrap()
        );
        // Make sure it comiled without any warnings
        assert_eq!(compile_output.status.code().expect("Unable to compile"), 0);
        assert_eq!(compile_output.stdout.len(), 0);
        assert_eq!(compile_output.stderr.len(), 0);

        // Run the test executable
        let test_output = if cfg!(target_os = "windows") {
            process::Command::new("cmd")
                .arg("/C")
                .arg(format!(
                    ".\\{}.exe",
                    exe_path.to_str().unwrap().replace('/', "\\")
                ))
                .output()
                .expect("failed to test")
        } else {
            process::Command::new("sh")
                .arg("-c")
                .arg(format!("./{}", exe_path.to_str().unwrap()))
                .output()
                .expect("failed to test")
        };

        assert_eq!(test_output.status.code().expect("Unable to run test"), 0);
    }
}

/*#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashMap, fs, path, process};

       #[test]
       fn termite_basis() {
           if cfg!(target_os = "windows") {
               process::Command::new("cmd")
                   .current_dir("tests/cpp/termite")
                   .arg("/C")
                   .arg("mkdir build")
                   .output()
                   .expect("failed to compile");
           } else {
               process::Command::new("sh")
                   .current_dir("tests/cpp/termite")
                   .arg("-c")
                   .arg("mkdir build")
                   .output()
                   .expect("failed to compile");
           };

           let compile_output = if cfg!(target_os = "windows") {
               process::Command::new("cmd")
                   .current_dir("tests/cpp/termite/build")
                   .arg("/C")
                   .arg("cmake ..")
                   .output()
                   .expect("failed to compile")
           } else {
               process::Command::new("sh")
                   .current_dir("tests/cpp/termite/build")
                   .arg("-c")
                   .arg("cmake ..")
                   .output()
                   .expect("failed to compile")
           };

           assert_eq!(compile_output.status.code().expect("Unable to compile"), 0);
           assert_eq!(compile_output.stderr.len(), 0);

           let compile_output2 = if cfg!(target_os = "windows") {
               process::Command::new("cmd")
                   .current_dir("tests/cpp/termite/build")
                   .arg("/C")
                   .arg("cmake --build .")
                   .output()
                   .expect("failed to compile")
           } else {
               process::Command::new("sh")
                   .current_dir("tests/cpp/termite/build")
                   .arg("-c")
                   .arg("cmake --build .")
                   .output()
                   .expect("failed to compile")
           };

           assert_eq!(compile_output2.status.code().expect("Unable to compile"), 0);
           assert_eq!(compile_output2.stderr.len(), 0);

           let test_output = if cfg!(target_os = "windows") {
               process::Command::new("cmd")
                   .current_dir("tests/cpp/termite/build")
                   .arg("/C")
                   .arg(".\\Debug\\termite.exe")
                   .output()
                   .expect("failed to test")
           } else {
               process::Command::new("sh")
                   .current_dir("tests/cpp/termite/build")
                   .arg("-c")
                   .arg("./termite")
                   .output()
                   .expect("failed to test")
           };

           assert_eq!(test_output.status.code().expect("Unable to run"), 0);

           let test_output_yaml = if cfg!(target_os = "windows") {
               process::Command::new("cmd")
                   .current_dir("tests/cpp/termite/build")
                   .arg("/C")
                   .arg(".\\Debug\\termite-yaml.exe")
                   .output()
                   .expect("failed to test")
           } else {
               process::Command::new("sh")
                   .current_dir("tests/cpp/termite/build")
                   .arg("-c")
                   .arg("./termite-yaml")
                   .output()
                   .expect("failed to test")
           };

           assert_eq!(test_output_yaml.status.code().expect("Unable to run"), 0);
       }

       #[test]
       fn default_order() {
           let data = crate::DataModel {
               headers: HashMap::new(),
               footers: HashMap::new(),
               data_types: vec![crate::DataType {
                   name: "DataType".to_string(),
                   description: None,
                   data: crate::DataTypeData::Struct(crate::Struct {
                       fields: vec![
                           crate::StructField {
                               name: "field1".to_string(),
                               description: None,
                               data_type: "int".to_string(),
                               default: crate::DefaultType::Default("1".to_string()),
                           },
                           crate::StructField {
                               name: "field1".to_string(),
                               description: None,
                               data_type: "int".to_string(),
                               default: crate::DefaultType::Required,
                           },
                       ],
                   }),
               }],
               namespace: vec![],
           };

           assert!(DataModel::new(data).is_err());
       }

       #[test]
       fn optional_order() {
           let data = crate::DataModel {
               headers: HashMap::new(),
               footers: HashMap::new(),
               data_types: vec![crate::DataType {
                   name: "DataType".to_string(),
                   description: None,
                   data: crate::DataTypeData::Struct(crate::Struct {
                       fields: vec![
                           crate::StructField {
                               name: "field1".to_string(),
                               description: None,
                               data_type: "int".to_string(),
                               default: crate::DefaultType::Optional,
                           },
                           crate::StructField {
                               name: "field1".to_string(),
                               description: None,
                               data_type: "int".to_string(),
                               default: crate::DefaultType::Required,
                           },
                       ],
                   }),
               }],
               namespace: vec![],
           };

           assert!(DataModel::new(data).is_err());
       }

       #[test]
       fn header() {
           // Check c++ code
           compile_and_test("header");

           // Make sure it generates the correct code
           let data_model = DataModel {
               headers: Headers {
                   source: "// header data".to_string(),
               },
               footers: Footers {
                   source: "".to_string(),
               },
               data_types: vec![],
               namespace: vec![],
           };

           // Create the header file
           let header_file = data_model.get_source("HEADER", 2);
           let expected = include_str!("../../tests/cpp/header/header.hpp");

           // Check that they are the same
           assert_eq!(str_diff(&header_file, &expected), None);
       }

       #[test]
       fn footer() {
           // Check c++ code
           compile_and_test("footer");

           // Make sure it generates the correct code
           let data_model = DataModel {
               headers: Headers {
                   source: "".to_string(),
               },
               footers: Footers {
                   source: "// footer data".to_string(),
               },
               data_types: vec![],
               namespace: vec![],
           };

           // Create the header file
           let header_file = data_model.get_source("HEADER", 2);
           let expected = include_str!("../../tests/cpp/footer/footer.hpp");

           // Check that they are the same
           assert_eq!(str_diff(&header_file, &expected), None);
       }

       #[test]
       fn namespace() {
           // Check c++ code
           compile_and_test("namespace");

           // Make sure it generates the correct code
           let data_model = DataModel {
               headers: Headers {
                   source: "".to_string(),
               },
               footers: Footers {
                   source: "".to_string(),
               },
               data_types: vec![],
               namespace: vec!["test1".to_string(), "test2".to_string()],
           };

           // Create the header file
           let header_file = data_model.get_source("HEADER", 2);
           let expected = include_str!("../../tests/cpp/namespace/namespace.hpp");

           // Check that they are the same
           assert_eq!(str_diff(&header_file, &expected), None);
       }

       #[test]
       fn outline() {
           // Check c++ code
           compile_and_test("outline");

           // Make sure it generates the correct code
           let data_model = DataModel {
               headers: Headers {
                   source: "// Header".to_string(),
               },
               footers: Footers {
                   source: "// Footer".to_string(),
               },
               data_types: vec![
                   DataType {
                       name: "DataType1".to_string(),
                       description: Some("description1".to_string()),
                       data: DataTypeData::Struct(Struct { fields: vec![] }),
                   },
                   DataType {
                       name: "DataType2".to_string(),
                       description: Some("description2".to_string()),
                       data: DataTypeData::Struct(Struct { fields: vec![] }),
                   },
               ],
               namespace: vec!["test".to_string()],
           };

           // Create the header file
           let header_file = data_model.get_source("HEADER", 2);
           let expected = include_str!("../../tests/cpp/outline/outline.hpp");

           // Check that they are the same
           assert_eq!(str_diff(&header_file, &expected), None);
       }
}*/
