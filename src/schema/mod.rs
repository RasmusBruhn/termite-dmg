use crate::{data_model, DefaultType};
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

        // Construct all of the definitions
        let mut defs = jzon::object::Object::new();
        let mut dependencies = HashSet::new();
        let mut implemented_types = HashSet::new();
        dependencies.insert(main_type.name.clone());
        while !dependencies.is_empty() {
            // Get the type to implement
            let implement_type = if let Some(value) = dependencies
                .iter()
                .filter(|value| !implemented_types.contains(*value))
                .next()
            {
                value.clone()
            } else {
                break;
            };

            // Construct the type schema
            let type_schema = data_types
                .get(&implement_type)
                .unwrap()
                .export_schema(&data_types, &mut dependencies)?;

            // Add it to the definitions
            defs.insert(&implement_type, JsonValue::Object(type_schema));

            // Add to the implemented types
            implemented_types.insert(implement_type);
        }

        // Construct the main type schema
        let schema = jzon::object! {
            "$comment": format!("This schema is generated using Termite Data Model Generator to be compatible with a generated data model using version {}.{}", std::env!("CARGO_PKG_VERSION_MAJOR"), std::env!("CARGO_PKG_VERSION_MINOR")),
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": schema_id.to_string(),
            "$ref": id.to_string(),
            "$defs": JsonValue::Object(defs),
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
    ) -> Result<jzon::object::Object, Error> {
        // Convert the data to a schema
        let mut schema = self.data.export_schema(custom_types, dependencies)?;

        // Add the id
        schema.insert("$id", JsonValue::String(self.name.clone()));

        // Add the description
        if let Some(description) = &self.description {
            schema.insert("description", JsonValue::String(description.clone()));
        }

        return Ok(schema);
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
    ) -> Result<jzon::object::Object, Error> {
        return match self {
            data_model::DataTypeData::Struct(data) => {
                data.export_schema(custom_types, dependencies)
            }
            data_model::DataTypeData::Array(data) => data.export_schema(custom_types, dependencies),
            data_model::DataTypeData::Variant(data) => {
                data.export_schema(custom_types, dependencies)
            }
            data_model::DataTypeData::Enum(data) => data.export_schema(custom_types, dependencies),
            data_model::DataTypeData::ConstrainedType(data) => {
                data.export_schema(custom_types, dependencies)
            }
        };
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
    ) -> Result<jzon::object::Object, Error> {
        // Setup the required list
        let mut required = vec![];

        // Setup the properties
        let mut properties = jzon::object::Object::new();

        // Add all the fields
        for field in self.fields.iter() {
            // Add the type to dependencies
            let ref_keyword = is_schema_type(&field.data_type, custom_types, dependencies)
                .or_else(|error| {
                    return Err(error.add_field(&field.name));
                })?;

            // Construct the schema
            let mut field_schema = jzon::object::Object::new();
            field_schema.insert(ref_keyword, JsonValue::String(field.data_type.clone()));
            if let Some(description) = &field.description {
                field_schema.insert("description", JsonValue::String(description.clone()));
            }
            match &field.default {
                DefaultType::Optional => (),
                DefaultType::Required => required.push(JsonValue::String(field.name.clone())),
                DefaultType::Default(value) => {
                    field_schema.insert("default", JsonValue::String(value.clone()))
                }
            }

            // Add to the properties
            properties.insert(&field.name, JsonValue::Object(field_schema));
        }

        // Setup the schema
        let mut schema = jzon::object::Object::new();
        schema.insert("properties", JsonValue::Object(properties));
        schema.insert("required", JsonValue::Array(required));
        schema.insert("type", JsonValue::String("object".to_string()));
        schema.insert(
            "$comment",
            JsonValue::String("A struct which contains a set of fields".to_string()),
        );

        // Get the inheritence
        if let Some(inherit) = &self.inherit {
            match custom_types.get(inherit) {
                Some(value) => match &value.data {
                    data_model::DataTypeData::Struct(_) => {
                        dependencies.insert(inherit.clone());
                        schema.insert("$ref", JsonValue::String(inherit.clone()));
                    }
                    _ => {
                        return Err(Error {
                            location: "".to_string(),
                            error: ErrorCore::EnheritStruct(inherit.clone()),
                        })
                    }
                },
                None => {
                    return Err(Error {
                        location: "".to_string(),
                        error: ErrorCore::UnknownID(inherit.clone()),
                    });
                }
            }
        }

        return Ok(schema);
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
    ) -> Result<jzon::object::Object, Error> {
        // Add the type to dependencies
        let ref_keyword = is_schema_type(&self.data_type, custom_types, dependencies)?;

        // Construct the element schema
        let mut element_schema = jzon::object::Object::new();
        element_schema.insert(ref_keyword, JsonValue::String(self.data_type.clone()));

        // Construct the schema
        let mut schema = jzon::object::Object::new();
        schema.insert(
            "$comment",
            JsonValue::String("An array of the same type elements".to_string()),
        );
        schema.insert("type", JsonValue::String("array".to_string()));
        schema.insert("items", JsonValue::Object(element_schema));

        return Ok(schema);
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
    ) -> Result<jzon::object::Object, Error> {
        // Construct all variant types
        let variant_types = self
            .data_types
            .iter()
            .map(|name| {
                // Add the type to dependencies
                let ref_keyword = is_schema_type(name, custom_types, dependencies)?;

                // Create the schema
                let mut schema = jzon::object::Object::new();
                schema.insert(ref_keyword, JsonValue::String(name.clone()));

                return Ok(JsonValue::Object(schema));
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Create the schema
        let mut schema = jzon::object::Object::new();
        schema.insert(
            "$comment",
            JsonValue::String(
                "A variant which will convert an input to the first type it matches".to_string(),
            ),
        );
        schema.insert("anyOf", JsonValue::Array(variant_types));

        return Ok(schema);
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
    ) -> Result<jzon::object::Object, Error> {
        // Convert each of the enum types
        let enum_list = self
            .types
            .iter()
            .map(|value| {
                let mut schema = if let Some(data_type) = &value.data_type {
                    // Check if it is a custom type
                    let ref_keyword = is_schema_type(data_type, custom_types, dependencies)?;

                    // Create the internal type
                    let mut internal_schema = jzon::object::Object::new();
                    internal_schema.insert(ref_keyword, JsonValue::String(data_type.clone()));

                    // Create the properties
                    let mut properties = jzon::object::Object::new();
                    properties.insert(&value.name, JsonValue::Object(internal_schema));

                    // Create the schema
                    let mut schema = jzon::object::Object::new();
                    schema.insert("type", JsonValue::String("object".to_string()));
                    schema.insert("additionalProperties", JsonValue::Boolean(false));
                    schema.insert("properties", JsonValue::Object(properties));
                    schema.insert(
                        "required",
                        JsonValue::Array(vec![JsonValue::String(value.name.clone())]),
                    );

                    schema
                } else {
                    let mut schema = jzon::object::Object::new();
                    schema.insert("type", JsonValue::String("string".to_string()));
                    schema.insert("pattern", JsonValue::String(value.name.clone()));

                    schema
                };

                if let Some(description) = &value.description {
                    schema.insert("description", JsonValue::String(description.clone()));
                }

                return Ok(JsonValue::Object(schema));
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Create the
        let mut schema = jzon::object::Object::new();
        schema.insert(
            "$comment",
            JsonValue::String("A rust-like enum which can be typed".to_string()),
        );
        schema.insert("enum", JsonValue::Array(enum_list));

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
    ) -> Result<jzon::object::Object, Error> {
        // Add the type to dependencies
        let ref_keyword = is_schema_type(&self.data_type, custom_types, dependencies)?;

        // Get the list of constraints
        let constraints = self.constraints.join(", ");

        // Create the schema
        let mut schema = jzon::object::Object::new();
        schema.insert(
            "$comment",
            JsonValue::String(format!(
                "A constrained type which must keep the following true: [{}]",
                constraints
            )),
        );
        schema.insert(ref_keyword, JsonValue::String(self.data_type.clone()));

        return Ok(schema);
    }
}

/// Checks if a given type is a builtin schema type or a custom type, if it is a custom type then it adds it to the dependencies
///
/// # Parameters
///
/// name: The name of the type
///
/// custom_types: All the custom types in the schema
///
/// dependencies: The set to add the type to if it is a custom type
///
/// # Errors
///
/// If the type is not a custom type or a builtin type then an error it thrown
fn is_schema_type(
    name: &str,
    custom_types: &HashMap<String, data_model::DataType>,
    dependencies: &mut HashSet<String>,
) -> Result<&'static str, Error> {
    return Ok(if let Some(_) = custom_types.get(name) {
        dependencies.insert(name.to_string());
        "$ref"
    } else {
        if !vec!["boolean", "integer", "number", "string"].contains(&name) {
            return Err(Error {
                location: "".to_string(),
                error: ErrorCore::UnknownType(name.to_string()),
            });
        }
        "type"
    });
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
    /// The type is neither a builtin type or a custom type
    #[error("The type id \"{:}\" does not exist in the model or as a builtin type", .0)]
    UnknownType(String),
    /// The type must be a struct
    #[error("The type id \"{:}\" must refer to a struct when inheriting", .0)]
    EnheritStruct(String),
}
