use crate::data_model;
use jzon::JsonValue;
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

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

        // Construct all of the types
        let types = JsonValue::new_object();

        // Construct the main type schema
        let schema = jzon::object! {
            "$comment": format!("This schema is generated using Termite Data Model Generator to be compatible with a generated data model using version {}.{}", std::env!("CARGO_PKG_VERSION_MAJOR"), std::env!("CARGO_PKG_VERSION_MINOR")),
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": schema_id,
            "$ref": id,
        };

        return Ok(schema);
    }
}

impl data_model::DataType {
    /// Creates a JSON schema from the data type
    ///
    /// # Parameters
    ///
    /// custom_types: The map of all the custom types, used to check if a type is builtin or not
    ///
    /// dependencies: A set to add all dependencies of this data type to
    pub fn export_schema(
        &self,
        custom_types: &HashMap<String, data_model::DataType>,
        dependencies: &mut HashSet<String>,
    ) -> Result<JsonValue, Error> {
        todo!();
    }
}

impl data_model::DataTypeData {
    /// Creates a JSON schema from the data type data
    ///
    /// # Parameters
    ///
    /// custom_types: The map of all the custom types, used to check if a type is builtin or not
    ///
    /// dependencies: A set to add all dependencies of this data type data to
    pub fn export_schema(
        &self,
        custom_types: &HashMap<String, data_model::DataType>,
        dependencies: &mut HashSet<String>,
    ) -> Result<JsonValue, Error> {
        todo!();
    }
}

impl data_model::Struct {
    /// Creates a JSON schema from the struct
    ///
    /// # Parameters
    ///
    /// custom_types: The map of all the custom types, used to check if a type is builtin or not
    ///
    /// dependencies: A set to add all dependencies of this struct to
    pub fn export_schema(
        &self,
        custom_types: &HashMap<String, data_model::DataType>,
        dependencies: &mut HashSet<String>,
    ) -> Result<JsonValue, Error> {
        todo!();
    }
}

impl data_model::Array {
    /// Creates a JSON schema from the array
    ///
    /// # Parameters
    ///
    /// custom_types: The map of all the custom types, used to check if a type is builtin or not
    ///
    /// dependencies: A set to add all dependencies of this array to
    pub fn export_schema(
        &self,
        custom_types: &HashMap<String, data_model::DataType>,
        dependencies: &mut HashSet<String>,
    ) -> Result<JsonValue, Error> {
        todo!();
    }
}

impl data_model::Variant {
    /// Creates a JSON schema from the variant
    ///
    /// # Parameters
    ///
    /// custom_types: The map of all the custom types, used to check if a type is builtin or not
    ///
    /// dependencies: A set to add all dependencies of this variant to
    pub fn export_schema(
        &self,
        custom_types: &HashMap<String, data_model::DataType>,
        dependencies: &mut HashSet<String>,
    ) -> Result<JsonValue, Error> {
        todo!();
    }
}

impl data_model::Enum {
    /// Creates a JSON schema from the enum
    ///
    /// # Parameters
    ///
    /// custom_types: The map of all the custom types, used to check if a type is builtin or not
    ///
    /// dependencies: A set to add all dependencies of this enum to
    pub fn export_schema(
        &self,
        custom_types: &HashMap<String, data_model::DataType>,
        dependencies: &mut HashSet<String>,
    ) -> Result<JsonValue, Error> {
        // Convert each of the enum types
        let enum_list = self.types.iter().map(|value| {
            let schema = if let Some(data_type) = value.data_type {
                // Check if it is a custom type
                let ref_keyword = if let Some(_) = custom_types.get(&data_type) {
                    dependencies.insert(data_type.clone());
                    "$ref"
                } else {
                    "type"
                };

                // Create the internal type
                let internal_schema = 

                // Create the schema
                let mut schema = JsonValue::new_object();
                schema.insert(&value.name, )
            } else {
                jzon::object! {
                    "type": "string",
                    "pattern": value.name.clone(),
                }
            };
        });

        // Create the schema
        let schema = jzon::object! {
            "enum": ""
        };

        return Ok(schema);
    }
}

impl data_model::ConstrainedType {
    /// Creates a JSON schema from the constrained type
    ///
    /// # Parameters
    ///
    /// custom_types: The map of all the custom types, used to check if a type is builtin or not
    ///
    /// dependencies: A set to add all dependencies of this constrained type to
    pub fn export_schema(
        &self,
        custom_types: &HashMap<String, data_model::DataType>,
        dependencies: &mut HashSet<String>,
    ) -> Result<JsonValue, Error> {
        // Add the type to dependencies
        let ref_keyword = if let Some(_) = custom_types.get(&self.data_type) {
            dependencies.insert(self.data_type.clone());
            "$ref"
        } else {
            "type"
        };

        // Get the list of constraints
        let constraints = self.constraints.join(", ");

        // Create the schema
        let mut schema = jzon::object! {
            "$comment": format!("A constrained type which must keep the following true: [{}]", constraints),
        };
        schema.insert(ref_keyword, self.data_type.clone());

        return Ok(schema);
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
