//! 
//! This module handles generation of c++ code to support a data model, it includes the ability to create
//! a header file, (de)serialization and documentation.
//! 

use std::{
    fmt,
    collections::HashMap,
};

mod header;

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

/// All of the headers for the different files
#[derive(Clone, Debug, PartialEq)]
struct Headers {
    /// For the header fiel
    header: String,
    /// For the source file
    source: String,
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
    fn new(data: HashMap<String, String>) -> Result<Self, Error> {
        todo!()
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
            format!(".{}", self.location)
        } else {
            format!("{}{}", base, self.location)
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
