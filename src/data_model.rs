use std::collections::HashMap;
use thiserror::Error;
use append_only_vec::AppendOnlyVec;

/// A full data model with header information for generating code and a list of custom data types
#[derive(Debug)]
pub struct DataModel {
    /// The vector of all the data types
    data_types: AppendOnlyVec<(String, DataType)>,
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
    pub fn from_vec(data: Vec<(String, DataType)>) -> Result<Self, ModelError> {
        let mut data_model = Self::new();
        
        for (name, data_type) in data.into_iter() {
            data_model.add(name.as_str(), data_type)?;
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
    pub fn add(&mut self, name: &str, data: DataType) -> Result<(), ModelError> {
        // Make sure the object is not a dublicate
        if self.data_types.iter().any(|(type_name, _)| {
            type_name == name
        }) {
            return Err(ModelError::DublicateTypeName(name.to_string()));
        };

        self.data_types.push((name.to_string(), data));

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
        return self.data_types.iter().filter_map(|(type_name, type_data)| {
            return if type_name == name {
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
    pub fn get_group<'a>(&'a self, base_type: &'a str) -> impl Iterator<Item = &'a (String, DataType)> {
        return self.data_types.iter()
            .filter(move |item| {
                item.1.base_type == base_type
            });
    }

    /// Gets an iterator over all data types
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a (String, DataType)> {
        return self.data_types.iter();
    }
}

/// Holds the data for a single custom data type
#[derive(Clone, Debug, PartialEq)]
pub struct DataType {
    /// The base type (like struct or enum) of this data type
    pub base_type: String,
    /// The data for this data type (like struct fields of variant types)
    pub data: Vec<(String, Instance)>,
    /// The description for this type, may be None if there is no description
    pub description: Option<String>,
}

/// Data for a single struct field or variant type
#[derive(Clone, Debug, PartialEq)]
pub struct Instance {
    /// All of the data fields, keys may include data_type or description
    pub data: HashMap<String, String>,
}

/// Errors when working with data models
#[derive(Error, Debug, Clone)]
pub enum ModelError {
    /// An object already has the name which is being added
    #[error("The data type \"{:?}\" cannot be added to the data model because it already exists", .0)]
    DublicateTypeName(String),
}
