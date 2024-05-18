use std::collections::HashMap;
use thiserror::Error;
use append_only_vec::AppendOnlyVec;

#[derive(Debug)]
pub struct DataModel {
    data_types: AppendOnlyVec<(String, DataType)>,
}

impl DataModel {
    pub fn new() -> Self {
        return Self { data_types: AppendOnlyVec::new() };
    }

    pub fn from_vec(data: Vec<(String, DataType)>) -> Result<Self, ModelError> {
        let mut data_model = Self::new();
        
        for (name, data_type) in data.into_iter() {
            data_model.add(name.as_str(), data_type)?;
        }

        return Ok(data_model);
    }

    pub fn add(&mut self, name: &str, data_type: DataType) -> Result<(), ModelError> {
        // Make sure the object is not a dublicate
        if self.data_types.iter().any(|(type_name, _)| {
            type_name == name
        }) {
            return Err(ModelError::DublicateTypeName(name.to_string()));
        };

        self.data_types.push((name.to_string(), data_type));

        return Ok(());
    }

    pub fn get(&self, name: &str) -> Option<&DataType> {
        return self.data_types.iter().filter_map(|(type_name, type_data)| {
            return if type_name == name {
                Some(type_data)
            } else {
                None
            }
        }).next();
    }

    pub fn get_group<'a>(&'a self, base_type: &'a str) -> impl Iterator<Item = &'a (String, DataType)> {
        return self.data_types.iter()
            .filter(move |item| {
                item.1.base_type == base_type
            });
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a (String, DataType)> {
        return self.data_types.iter();
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataType {
    pub base_type: String,
    pub fields: Vec<(String, Instance)>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Instance {
    pub data: HashMap<String, String>,
}

#[derive(Error, Debug, Clone)]
pub enum ModelError {
    /// An object already has the name which is being added
    #[error("The data type \"{:?}\" cannot be added to the data model because it already exists", .0)]
    DublicateTypeName(String),
}
