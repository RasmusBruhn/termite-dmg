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
use std::{
  fmt,
  collections::HashMap,
};

mod type_struct;
mod type_array;
mod type_variant;

use type_struct::Struct;
use type_array::Array;
use type_variant::Variant;

/// Obtains the base termite c++ dependency required for all generated data
/// models
pub fn get_termite_dependency() -> &'static str {
  return include_str!("termite.hpp");
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
    let data_types = data.data_types.into_iter()
      .enumerate()
      .map(|(i, data_type)| {
        return match DataType::new(data_type) {
          Ok(result) => Ok(result),
          Err(error) => Err(error.add_element("data_types", i)),
        };
      }).collect::<Result<Vec<DataType>, Error>>()?;
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
  pub fn get_source(&self, name: &str, indent: usize) -> String {
    // Get the namespace starts
    let namespace_start = self.namespace.iter()
      .map(|namespace| format!("namespace {namespace} {{"))
      .collect::<Vec<String>>()
      .join(" ");

    // Get all structs
    let data_types = self.data_types.iter()
      .map(|data_type| data_type.get_source(indent))
      .collect::<Vec<String>>()
      .join("\n\n");

    // Get the namespace end
    let namespace_end = self.namespace.iter()
      .map(|namespace| format!("}} // namespace {namespace}"))
      .collect::<Vec<String>>()
      .join("\n\n");

    let parsers = self.data_types.iter()
      .map(|data_type| data_type.get_parser(indent, &self.namespace))
      .collect::<Vec<String>>()
      .join("\n\n");

    return formatdoc!("
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

      namespace {{

      template <typename T, typename = void>
      struct has_insertion_operator : std::false_type {{}};

      template <typename T>
      struct has_insertion_operator<T, std::void_t<decltype(std::declval<std::ostream &>() << std::declval<T>())>> : std::true_type {{}};

      }} // namespace

      {namespace_start}
      
      namespace {{

      template <typename T>
      typename std::enable_if<has_insertion_operator<T>::value, std::ostream &>::type
      operator<<(std::ostream &os, const std::optional<T> &value) {{
      {0:indent$}if (value) {{
      {0:indent$}{0:indent$}return os << *value;
      {0:indent$}}} else {{
      {0:indent$}{0:indent$}return os << \"nullopt\";
      {0:indent$}}}
      }}

      }} // namespace

      {data_types}

      {namespace_end}

      namespace termite {{
      
      namespace {{

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

      {parsers}

      }} // namespace termite
      
      {footer}
      
      #endif
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

    return Ok(Self {
      source,
    });
  }
}

/// All of the footers for the different files
#[derive(Clone, Debug, PartialEq)]
struct Footers {
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

    return Ok(Self {
      source,
    });
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
  fn get_source(&self, indent: usize) -> String {
    return formatdoc!("
      /**
       * @brief {description}
       * 
       */
      {definition}",
      description = self.get_description(),
      definition = self.data.get_source(&self.name, indent),
    );
  }

  /// Gets the source code for the parser for this type allowing it to be read from a file
  /// 
  /// # Parameters
  /// 
  /// indent: The number of spaces to use for indentation
  /// 
  /// namespace: The namespace of the type
  pub(super) fn get_parser(&self, indent: usize, namespace: &[String]) -> String {
    return self.data.get_parser(&self.name, indent, namespace);
  }
}

/// Supplies the type sepcific information for a data type
#[derive(Clone, Debug, PartialEq)]
enum DataTypeData {
  /// Describes a struct
  Struct(Struct),
  /// Describes an array
  Array(Array),
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
  fn get_source(&self, name: &str, indent: usize) -> String {
    return match self {
      DataTypeData::Struct(data) => data.get_source(name, indent),
      DataTypeData::Array(data) => data.get_source(name, indent),
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
  pub(super) fn get_parser(&self, name: &str, indent: usize, namespace: &[String]) -> String {
    return match self {
      DataTypeData::Struct(data) => data.get_parser(name, indent, namespace),
      DataTypeData::Array(data) => data.get_parser(name, indent, namespace),
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
    }
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
    }
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
  /// A required field was found after an optional one
  #[error("The required field \"{}\" was placed after an optional field", .0)]
  StructFieldOrder(String),
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::{
    collections::HashMap,
    process,
    fs,
    path,
  };

  fn str_diff(lhs: &str, rhs: &str) -> Option<(usize, String, String)> {
    if let Some(error) = lhs.lines()
      .zip(rhs.lines())
      .enumerate()
      .filter_map(|(index, (lhs, rhs))| {
        return if lhs == rhs {
          None
        } else {
          Some((index + 1, lhs.to_string(), rhs.to_string()))
        };
      })
      .next() {
      return Some(error);
    }

    if lhs.lines().count() != rhs.lines().count() {
      return Some((0, "".to_string(), "".to_string()));
    }

    return None;
  }

  fn get_source_path(name: &str) -> path::PathBuf {
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

  fn compile_and_test(name: &str) {
    // Get the paths
    let source_path = get_source_path(name);
    let exe_path = get_exe_path(name);

    // Create the output directory
    fs::create_dir_all(exe_path.parent().unwrap()).unwrap();

    // Compile code
    let compile_output = if cfg!(target_os = "windows") {
      process::Command::new("cmd")
        .arg("/C")
        .arg(format!("g++ {} -Isrc/cpp -Wall -std=c++17 -o {}.exe", source_path.to_str().unwrap(), exe_path.to_str().unwrap()))
        .output()
        .expect("failed to compile")
    } else {
      process::Command::new("sh")
        .arg("-c")
        .arg(format!("g++ {} -Isrc/cpp -Wall -std=c++17 -o {}", source_path.to_str().unwrap(), exe_path.to_str().unwrap()))
        .output()
        .expect("failed to compile")
    };

    // Make sure it comiled without any warnings
    assert_eq!(compile_output.status.code().expect("Unable to compile"), 0);
    assert_eq!(compile_output.stdout.len(), 0);
    assert_eq!(compile_output.stderr.len(), 0);

    // Run the test executable
    let test_output = if cfg!(target_os = "windows") {
      process::Command::new("cmd")
        .arg("/C")
        .arg(format!(".\\{}.exe", exe_path.to_str().unwrap().replace('/', "\\")))
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

  #[test]
  fn termite_basis() {
    let compile_output = if cfg!(target_os = "windows") {
      process::Command::new("cmd")
        .arg("/C")
        .arg("g++ src/cpp/termite_test.cpp -Wall -std=c++17 -o target/debug/build/test_cpp_termite_dependency.exe")
        .output()
        .expect("failed to compile")
    } else {
      process::Command::new("sh")
        .arg("-c")
        .arg("g++ src/cpp/termite_test.cpp -Wall -std=c++17 -o target/debug/build/test_cpp_termite_dependency")
        .output()
        .expect("failed to compile")
    };

    assert_eq!(compile_output.status.code().expect("Unable to compile"), 0);
    assert_eq!(compile_output.stdout.len(), 0);
    assert_eq!(compile_output.stderr.len(), 0);

    let test_output = if cfg!(target_os = "windows") {
      process::Command::new("cmd")
        .arg("/C")
        .arg(".\\target\\debug\\build\\test_cpp_termite_dependency.exe")
        .output()
        .expect("failed to test")
    } else {
      process::Command::new("sh")
        .arg("-c")
        .arg("./target/debug/build/test_cpp_termite_dependency")
        .output()
        .expect("failed to test")
    };

    assert_eq!(test_output.status.code().expect("Unable to compile"), 0);
  }

  #[test]
  fn default_order() {
    let data = crate::DataModel {
      headers: HashMap::new(),
      footers: HashMap::new(),
      data_types: vec![
        crate::DataType {
          name: "DataType".to_string(),
          description: None,
          data: crate::DataTypeData::Struct(crate::Struct {
            fields: vec![
              crate::StructField {
                name: "field1".to_string(),
                description: None,
                data_type: "int".to_string(),
                default: crate::DefaultType::Default("1".to_string()),
                constraints: vec![],
              },
              crate::StructField {
                name: "field1".to_string(),
                description: None,
                data_type: "int".to_string(),
                default: crate::DefaultType::Required,
                constraints: vec![],
              },
            ],
          }),
        }
      ],
      namespace: vec![],
    };

    assert!(DataModel::new(data).is_err());
  }

  #[test]
  fn optional_order() {
    let data = crate::DataModel {
      headers: HashMap::new(),
      footers: HashMap::new(),
      data_types: vec![
        crate::DataType {
          name: "DataType".to_string(),
          description: None,
          data: crate::DataTypeData::Struct(crate::Struct {
            fields: vec![
              crate::StructField {
                name: "field1".to_string(),
                description: None,
                data_type: "int".to_string(),
                default: crate::DefaultType::Optional,
                constraints: vec![],
              },
              crate::StructField {
                name: "field1".to_string(),
                description: None,
                data_type: "int".to_string(),
                default: crate::DefaultType::Required,
                constraints: vec![],
              },
            ],
          }),
        }
      ],
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
      headers: Headers { source: "// header data".to_string() },
      footers: Footers { source: "".to_string() },
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
      headers: Headers { source: "".to_string() },
      footers: Footers { source: "// footer data".to_string() },
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
      headers: Headers { source: "".to_string() },
      footers: Footers { source: "".to_string() },
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
      headers: Headers { source: "// Header".to_string() },
      footers: Footers { source: "// Footer".to_string() },
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
}
