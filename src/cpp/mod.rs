//! 
//! This module handles generation of c++ code to support a data model, it
//! includes the ability to create a header file, (de)serialization and
//! documentation.
//! 
//! For any data model to work the termite dependency must be generatred from
//! get_termite_dependency() and be saved as "termite.hpp" at a location where
//! it can be included as "#include <termite.hpp>"
//! 

use std::{
    fmt,
    collections::HashMap,
};

mod header;

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
    /// List of all header data used to include external packages and start
    /// namespaces
    headers: Headers,
    /// List of all footer data used to end namespaces
    footers: Footers,
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
        })
    }
}

/// All of the headers for the different files
#[derive(Clone, Debug, PartialEq)]
struct Headers {
    /// For the header fiel
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
        let header = match data.remove("header") {
            Some(value) => format!("{}\n", value),
            None => String::new(),
        };
        let source = match data.remove("source") {
            Some(value) => value,
            None => String::new(),
        };

        return Ok(Self {
            header,
            source,
        })
    }
}

/// All of the footers for the different files
#[derive(Clone, Debug, PartialEq)]
struct Footers {
    /// For the header fiel
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
        let header = match data.remove("header") {
            Some(value) => format!("{}\n", value),
            None => String::new(),
        };
        let source = match data.remove("source") {
            Some(value) => value,
            None => String::new(),
        };

        return Ok(Self {
            header,
            source,
        })
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
}

/// Supplies the type sepcific information for a data type
#[derive(Clone, Debug, PartialEq)]
enum DataTypeData {
    /// Describes a struct
    Struct(Struct),
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
        };

        return Ok(result);
    }
}

/// The type specific information for a struct
#[derive(Clone, Debug, PartialEq)]
struct Struct {
    /// A list of all the fields of the struct
    fields: Vec<StructField>,
}

impl Struct {
    /// Constructs a new c++ struct from a generic struct
    /// 
    /// # Parameters
    /// 
    /// data: The generic struct to convert
    fn new(data: crate::Struct) -> Result<Self, Error> {
        // Make sure the required fields are first
        if let Some(name) = data.fields.iter()
            .scan(false, |found_optional, field| {
                if let crate::DefaultType::Required = field.default {
                    if *found_optional {
                        return Some(Some(&field.name));
                    }
                } else {
                    *found_optional = true;
                }

                return Some(None);
            })
            .filter_map(|value| value)
            .next() {
            return Err(Error {
                location: "".to_string(),
                error: ErrorCore::StructFieldOrder(name.clone()),
            })
        }

        // Convert the fields
        let fields = data.fields.into_iter().map(|data| StructField::new(data)).collect::<Result<Vec<StructField>, Error>>()?;

        // Move data
        return Ok(Self {
            fields
        })
    }
}

/// A single field for a struct
#[derive(Clone, Debug, PartialEq)]
struct StructField {
    /// The name of the field
    name: String,
    /// A description of the field
    description: Option<String>,
    /// The data type of the field
    data_type: String,
    /// Describes if the field is required or not, if optional it gives the
    /// default value
    default: crate::DefaultType,
    /// A list of all the constraints the field must uphold
    constraints: Vec<String>,
}

impl StructField {
    /// Constructs a new c++ struct field from a generic struct field
    /// 
    /// # Parameters
    /// 
    /// data: The generic struct field to convert
    fn new(data: crate::StructField) -> Result<Self, Error> {
        return Ok(Self {
            name: data.name,
            description: data.description,
            data_type: data.data_type,
            default: data.default,
            constraints: data.constraints,
        })
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
    use std::process;

    #[test]
    fn termite_result_type() {
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
}
