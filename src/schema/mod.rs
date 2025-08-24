use crate::data_model;
use jzon::JsonValue;
use std::{collections::HashMap, fmt};

impl data_model::DataModel {
    /// Creates a JSON schema from the data model
    ///
    /// # Parameters
    ///
    /// id: The name of the type to export as a schema
    /// 
    /// schema_id: The JSON Schema id to put in the json object
    pub fn export_schema(&self, id: &str, schema_id: &str) -> Result<JsonValue, Error> {
        // Convert the data model to a map
        let data_types = HashMap::<String, data_model::DataType>::from_iter(
            self.data_types
                .iter()
                .map(|data_type| (data_type.name.clone(), data_type.clone())),
        );

        // Find the main type
        let main_type = data_types.get(id).ok_or(Error {
            location: "".to_string(),
            error: ErrorCore::UnknownID(id.to_string()),
        })?;

        

        todo!()
    }
}

/// Errors for when converting generic data models into JSON schema data models
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

/// Errors for when converting generic data models into JSON schema data models
#[derive(thiserror::Error, Debug, Clone)]
pub enum ErrorCore {
    /// Unable to find the type id
    #[error("The type id \"{:}\" does not exist in the model", .0)]
    UnknownID(String),
}
