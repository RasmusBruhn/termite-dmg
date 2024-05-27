use std::{
    collections::HashMap, 
    iter,
    fmt,
};
use thiserror::Error;
use append_only_vec::AppendOnlyVec;
use crate::io;

/// A full data model with header information for generating code and a list of custom data types
#[derive(Debug)]
pub struct DataModel {
    /// The vector of all the data types
    data_types: AppendOnlyVec<DataType>,
    /// The map of all the header data
    headers: HashMap<String, String>,
}

impl DataModel {
    /// Creates a new empty data model
    pub fn new() -> Self {
        return Self {
            data_types: AppendOnlyVec::new(),
            headers: HashMap::new(),
        };
    }

    /// Creates a data model with the supplied cusom data types
    /// 
    /// # Parameters
    /// 
    /// data: The vector of the names and data of data types for the data model
    /// 
    /// # Errors
    /// 
    /// Returns ModelError::DublicateTypeName if two data types has the same name
    pub fn from_vec(data: Vec<DataType>) -> Result<Self, ModelError> {
        let mut data_model = Self::new();
        
        for data_type in data.into_iter() {
            data_model.add(data_type)?;
        }

        return Ok(data_model);
    }

    /// Adds a new data type to the data model
    /// 
    /// # Parameters
    /// 
    /// name: The name of the new data type
    /// 
    /// data: The data for the new data type
    /// 
    /// # Errors
    /// 
    /// Returns ModelError if there is any errors
    pub fn add(&mut self, data: DataType) -> Result<(), ModelError> {
        // Make sure the object is not a dublicate
        if self.data_types.iter().any(|data_type| {
            data_type.base_type == data.base_type
        }) {
            return Err(ModelError::DublicateTypeName(data.base_type.to_string()));
        };

        self.data_types.push(data);

        return Ok(());
    }

    /// Adds header data to the data model for generation of one file
    /// 
    /// # Parameters
    /// 
    /// name: The name of the file to add this header to
    /// 
    /// header: The data to add to the header
    pub fn add_header(&mut self, name: &str, header: &str) {
        self.headers.insert(name.to_string(), header.to_string());
    }

    /// Gets the header data for one file
    /// 
    /// # Parameters
    /// 
    /// name: The name of the file to get the header data for
    pub fn get_header(&self, name: &str) -> &str {
        return self.headers.get(name).map_or("", |header| header.as_str());
    }

    /// Gets the data for a single data type or None if it does not exist
    /// 
    /// # Parameters
    /// 
    /// name: The name of the data type
    pub fn get(&self, name: &str) -> Option<&DataType> {
        return self.data_types.iter().filter_map(|type_data| {
            return if type_data.name == name {
                Some(type_data)
            } else {
                None
            }
        }).next();
    }

    /// Gets an iterator over all data types with the same base type, like structs or enums
    /// 
    /// # Parameters
    /// 
    /// base_type: The base type to filter for
    pub fn get_group<'a>(&'a self, base_type: &'a str) -> impl Iterator<Item = &'a DataType> {
        return self.data_types.iter()
            .filter(move |item| {
                item.base_type == base_type
            });
    }

    /// Gets an iterator over all data types
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a DataType> {
        return self.data_types.iter();
    }

    /// Converts the data model to an io bridge such that it can be exported to a file
    pub fn to_iobridge(&self) -> io::Bridge {
        let headers = io::Bridge::Struct(
            self.headers.iter().map(|(key, value)| (key.clone(), io::Bridge::Value(value.clone()))).collect()
        );

        let types = io::Bridge::List(
            self.data_types.iter().map(|value| value.to_iobridge()).collect()
        );

        return io::Bridge::Struct(HashMap::from([
            ("types".to_string(), types),
            ("headers".to_string(), headers),
        ]))
    }
}

/// Holds the data for a single custom data type
#[derive(Clone, Debug, PartialEq)]
pub struct DataType {
    /// The name of the data type
    pub name: String,
    /// The description for this type, may be None if there is no description
    pub description: Option<String>,
    /// The base type (like struct or enum) of this data type
    pub base_type: String,
    /// The data for this data type (like struct fields of variant types)
    pub data: Vec<Instance>,
}

impl DataType {
    /// Converts the data type to an io bridge such that it can be exported to a file
    pub fn to_iobridge(&self) -> io::Bridge {
        let data = io::Bridge::List(self.data.iter().map(|instance| instance.to_iobridge()).collect());

        return io::Bridge::Struct([
            ("name".to_string(), io::Bridge::Value(self.name.clone())),
            ("type".to_string(), io::Bridge::Value(self.base_type.clone())),
            ("data".to_string(), data),
        ].into_iter().chain(iter::once(self.description.clone()).filter_map(|description| {
            return if let Some(description) = description {
                Some(("description".to_string(), io::Bridge::Value(description)))
            } else {
                None
            }
        })).collect());
    }
}

/// Data for a single struct field or variant type
#[derive(Clone, Debug, PartialEq)]
pub struct Instance {
    /// The name of the instance
    pub name: String,
    /// All of the data fields, keys may include data_type or description
    pub data: HashMap<String, String>,
}

impl Instance {
    /// Imports an instance from an io bridge
    /*pub fn from_iobridge(bridge: io::Bridge) -> Result<Self, ParseError> {
        // Extract the name
        return if let io::Bridge::Struct(map) = bridge {
            let name =  if let Some((_, name)) = map.remove_entry("name") {
                if let io::Bridge::Value(name) = name {
                    name
                } else {
                    return Err(ParseError { location: "name".to_string(), error: ParseErrorCore::TypeString });
                }
            } else {
                return Err(ParseError { location: "".to_string(), error: ParseErrorCore::MissingName });
            };

            // Setup the data
            map.into_iter().map(|(key, value)| {
                if let io::Bridge::Value(value) = value
            })
        } else {
            return Err(ParseError { location: "".to_string(), error: ParseErrorCore::TypeStruct });
        };
    }*/

    /// Converts the instance to an io bridge such that it can be exported to a file
    pub fn to_iobridge(&self) -> io::Bridge {
        return io::Bridge::Struct(
            iter::once(("name".to_string(), self.name.clone())).chain(
                self.data.iter().map(|(key, value)| (key.clone(), value.clone()))
            ).map(|(key, value)| (key, io::Bridge::Value(value))).collect()
        );
    }
}

/// Errors when working with data models
#[derive(Error, Debug, Clone)]
pub enum ModelError {
    /// An object already has the name which is being added
    #[error("The data type \"{}\" cannot be added to the data model because it already exists", .0)]
    DublicateTypeName(String),
}

/// Errors when parsing data models including the location of the error
#[derive(Debug, Clone)]
pub struct ParseError {
    /// The location where the error occured
    pub location: String,
    /// The actual error that occured
    pub error: ParseErrorCore,
}

impl ParseError {
    /// Sets the current location to be the field of the given base
    /// 
    /// # Parameters
    /// 
    /// base: The base to set in the location
    fn add_field(&mut self, base: &str) {
        if !self.location.is_empty() {
            self.location = format!(".{}", self.location);
        }
        self.location = format!("{}{}", base, self.location);
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}: {}", self.location, self.error);
    }
}

/// Errors when parsing data models
#[derive(Error, Debug, Clone)]
pub enum ParseErrorCore {
    /// Type must be a string
    #[error("The type must be a string")]
    TypeString,
    /// Type must be a struct
    #[error("The type must be a struct")]
    TypeStruct,
    /// Type must be a list
    #[error("The type must be a list")]
    TypeList,
    /// The name field is required
    #[error("The \"name\" field is required")]
    MissingName,
}
