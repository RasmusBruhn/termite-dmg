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
            let type_schema = data_types.get(&implement_type).unwrap().export_schema(
                &data_types,
                &mut dependencies,
                &self.macros,
            )?;

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
    ///
    /// macros: A map of all macros to expand default values
    pub fn export_schema(
        &self,
        custom_types: &HashMap<String, data_model::DataType>,
        dependencies: &mut HashSet<String>,
        macros: &HashMap<String, data_model::SerializationModel>,
    ) -> Result<jzon::object::Object, Error> {
        // Convert the data to a schema
        let mut schema = self
            .data
            .export_schema(custom_types, dependencies, macros)?;

        // Add the id
        schema.insert("$id", JsonValue::String(self.name.clone()));

        // Add the description
        if let Some(description) = &self.description {
            schema.insert("description", JsonValue::String(description.clone()));
        }

        return Ok(schema);
    }

    /// Converts a serialization model value into a JSON value of the for of this type
    ///
    /// # Parameters
    ///
    /// value: The serialization model to convert
    ///
    /// custom_types: All the custom types in the schema
    pub fn schema_value(
        &self,
        value: &data_model::SerializationModel,
        custom_types: &HashMap<String, data_model::DataType>,
    ) -> Result<JsonValue, Error> {
        return self.data.schema_value(value, custom_types);
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
    ///
    /// macros: A map of all macros to expand default values
    pub fn export_schema(
        &self,
        custom_types: &HashMap<String, data_model::DataType>,
        dependencies: &mut HashSet<String>,
        macros: &HashMap<String, data_model::SerializationModel>,
    ) -> Result<jzon::object::Object, Error> {
        return match self {
            data_model::DataTypeData::Struct(data) => {
                data.export_schema(custom_types, dependencies, macros)
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

    /// Converts a serialization model value into a JSON value of the for of this type
    ///
    /// # Parameters
    ///
    /// value: The serialization model to convert
    ///
    /// custom_types: All the custom types in the schema
    pub fn schema_value(
        &self,
        value: &data_model::SerializationModel,
        custom_types: &HashMap<String, data_model::DataType>,
    ) -> Result<JsonValue, Error> {
        return match self {
            data_model::DataTypeData::Struct(data) => data.schema_value(value, custom_types),
            data_model::DataTypeData::Array(data) => data.schema_value(value, custom_types),
            data_model::DataTypeData::Variant(data) => data.schema_value(value, custom_types),
            data_model::DataTypeData::Enum(data) => data.schema_value(value, custom_types),
            data_model::DataTypeData::ConstrainedType(data) => {
                data.schema_value(value, custom_types)
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
    ///
    /// macros: A map of all macros to expand default values
    pub fn export_schema(
        &self,
        custom_types: &HashMap<String, data_model::DataType>,
        dependencies: &mut HashSet<String>,
        macros: &HashMap<String, data_model::SerializationModel>,
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
                    field_schema.insert(
                        "default",
                        to_json(
                            &data_model::expand_macros(value, macros, &mut HashSet::new())?,
                            &field.data_type,
                            custom_types,
                        )?,
                    );
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

    /// Converts a serialization model value into a JSON value of the for of this type
    ///
    /// # Parameters
    ///
    /// value: The serialization model to convert
    ///
    /// custom_types: All the custom types in the schema
    pub fn schema_value(
        &self,
        value: &data_model::SerializationModel,
        custom_types: &HashMap<String, data_model::DataType>,
    ) -> Result<JsonValue, Error> {
        return match value {
            data_model::SerializationModel::Map(value) => {
                // Convert each field
                let mut json_object = jzon::object::Object::new();
                for field in self.fields.iter() {
                    if let Some(field_value) = value.get(&field.name) {
                        match to_json(field_value, &field.data_type, custom_types) {
                            Ok(value) => {
                                json_object.insert(&field.name, value);
                            }
                            Err(error) => return Err(error.add_field(&field.name)),
                        }
                    } else if let DefaultType::Required = field.default {
                        return Err(Error {
                            location: "".to_string(),
                            error: ErrorCore::StructConversionMissingField(
                                value.clone(),
                                field.name.clone(),
                            ),
                        });
                    }
                }

                // Make sure all fields are used
                if json_object.len() != value.len() {
                    Err(Error {
                        location: "".to_string(),
                        error: ErrorCore::StructConversionExcessFields(value.clone()),
                    })
                } else {
                    Ok(JsonValue::Object(json_object))
                }
            }
            _ => Err(Error {
                location: "".to_string(),
                error: ErrorCore::SerializationModel(value.clone(), "struct".to_string()),
            }),
        };
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

    /// Converts a serialization model value into a JSON value of the for of this type
    ///
    /// # Parameters
    ///
    /// value: The serialization model to convert
    ///
    /// custom_types: All the custom types in the schema
    pub fn schema_value(
        &self,
        value: &data_model::SerializationModel,
        custom_types: &HashMap<String, data_model::DataType>,
    ) -> Result<JsonValue, Error> {
        return match value {
            data_model::SerializationModel::Array(values) => {
                match values
                    .iter()
                    .enumerate()
                    .map(|(i, value)| {
                        to_json(value, &self.data_type, custom_types).map_err(|error| (i, error))
                    })
                    .collect::<Result<Vec<_>, _>>()
                {
                    Ok(value) => Ok(JsonValue::Array(value)),
                    Err((i, error)) => Err(error.add_element(i)),
                }
            }
            _ => Err(Error {
                location: "".to_string(),
                error: ErrorCore::SerializationModel(value.clone(), "array".to_string()),
            }),
        };
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
            .collect::<Result<Vec<_>, Error>>()?;

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

    /// Converts a serialization model value into a JSON value of the for of this type
    ///
    /// # Parameters
    ///
    /// value: The serialization model to convert
    ///
    /// custom_types: All the custom types in the schema
    pub fn schema_value(
        &self,
        value: &data_model::SerializationModel,
        custom_types: &HashMap<String, data_model::DataType>,
    ) -> Result<JsonValue, Error> {
        let mut failures = Vec::new();
        for data_type in self.data_types.iter() {
            match to_json(value, data_type, custom_types) {
                Ok(value) => return Ok(value),
                Err(error) => failures.push((data_type.clone(), error)),
            }
        }

        return Err(Error {
            location: "".to_string(),
            error: ErrorCore::VariantConversion(value.clone(), failures),
        });
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
            .collect::<Result<Vec<_>, Error>>()?;

        // Create the
        let mut schema = jzon::object::Object::new();
        schema.insert(
            "$comment",
            JsonValue::String("A rust-like enum which can be typed".to_string()),
        );
        schema.insert("enum", JsonValue::Array(enum_list));

        return Ok(schema);
    }

    /// Converts a serialization model value into a JSON value of the for of this type
    ///
    /// # Parameters
    ///
    /// value: The serialization model to convert
    ///
    /// custom_types: All the custom types in the schema
    pub fn schema_value(
        &self,
        value: &data_model::SerializationModel,
        custom_types: &HashMap<String, data_model::DataType>,
    ) -> Result<JsonValue, Error> {
        return match value {
            data_model::SerializationModel::Value(value) => {
                if self
                    .types
                    .iter()
                    .filter(|enum_type| enum_type.data_type.is_none())
                    .any(|enum_type| &enum_type.name == value)
                {
                    return Ok(JsonValue::String(value.clone()));
                } else {
                    Err(Error {
                        location: "".to_string(),
                        error: ErrorCore::EnumConversion(value.clone()),
                    })
                }
            }
            data_model::SerializationModel::Map(value) => {
                if value.len() != 1 {
                    return Err(Error {
                        location: "".to_string(),
                        error: ErrorCore::TypedEnumLayout(value.clone()),
                    });
                }

                let (key, val) = value.iter().next().unwrap();
                let enum_type = self
                    .types
                    .iter()
                    .filter(|enum_type| enum_type.data_type.is_some())
                    .find(|enum_type| &enum_type.name == key);
                if let Some(enum_type) = enum_type {
                    let internal_type =
                        match to_json(val, enum_type.data_type.as_ref().unwrap(), custom_types) {
                            Ok(value) => value,
                            Err(error) => return Err(error.add_field(key)),
                        };

                    let mut json_object = jzon::object::Object::new();
                    json_object.insert(key, internal_type);
                    Ok(JsonValue::Object(json_object))
                } else {
                    return Err(Error {
                        location: "".to_string(),
                        error: ErrorCore::TypedEnumConversion(value.clone()),
                    });
                }
            }
            data_model::SerializationModel::Array(_) => Err(Error {
                location: "".to_string(),
                error: ErrorCore::SerializationModel(value.clone(), "enum".to_string()),
            }),
        };
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

    /// Converts a serialization model value into a JSON value of the for of this type
    ///
    /// # Parameters
    ///
    /// value: The serialization model to convert
    ///
    /// custom_types: All the custom types in the schema
    pub fn schema_value(
        &self,
        value: &data_model::SerializationModel,
        custom_types: &HashMap<String, data_model::DataType>,
    ) -> Result<JsonValue, Error> {
        return to_json(value, &self.data_type, custom_types);
    }
}

/// Converts a serialization model of a specific type to a JSON value
///
/// # Parameters
///
/// value: The serialization model to convert
///
/// data_type: The data type of the serialization model
///
/// custom_types: All the custom types in the schema
fn to_json(
    value: &data_model::SerializationModel,
    data_type: &str,
    custom_types: &HashMap<String, data_model::DataType>,
) -> Result<JsonValue, Error> {
    return match data_type {
        "boolean" => match value {
            data_model::SerializationModel::Value(value) => match value.as_str() {
                "true" => Ok(JsonValue::Boolean(true)),
                "false" => Ok(JsonValue::Boolean(false)),
                _ => Err(Error {
                    location: "".to_string(),
                    error: ErrorCore::BoolConversion(value.clone()),
                }),
            },
            _ => Err(Error {
                location: "".to_string(),
                error: ErrorCore::SerializationModel(value.clone(), "boolean".to_string()),
            }),
        },
        "integer" => match value {
            data_model::SerializationModel::Value(value) => match value.parse::<i64>() {
                Ok(value) => Ok(JsonValue::Number(jzon::number::Number::from(value))),
                Err(_) => Err(Error {
                    location: "".to_string(),
                    error: ErrorCore::IntegerConversion(value.clone()),
                }),
            },
            _ => Err(Error {
                location: "".to_string(),
                error: ErrorCore::SerializationModel(value.clone(), "integer".to_string()),
            }),
        },
        "number" => match value {
            data_model::SerializationModel::Value(value) => match value.parse::<f64>() {
                Ok(value) => Ok(JsonValue::Number(jzon::number::Number::from(value))),
                Err(_) => Err(Error {
                    location: "".to_string(),
                    error: ErrorCore::FloatConversion(value.clone()),
                }),
            },
            _ => Err(Error {
                location: "".to_string(),
                error: ErrorCore::SerializationModel(value.clone(), "number".to_string()),
            }),
        },
        "string" => match value {
            data_model::SerializationModel::Value(value) => Ok(JsonValue::String(value.clone())),
            _ => Err(Error {
                location: "".to_string(),
                error: ErrorCore::SerializationModel(value.clone(), "string".to_string()),
            }),
        },
        _ => {
            if let Some(custom_type) = custom_types.get(data_type) {
                custom_type.schema_value(value, custom_types)
            } else {
                Err(Error {
                    location: "".to_string(),
                    error: ErrorCore::UnknownType(data_type.to_string()),
                })
            }
        }
    };
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
        let location = format!(".{}{}", base, self.location);

        return Error {
            location,
            error: self.error,
        };
    }

    /// Sets the current location to be the element of a field of the given base
    ///
    /// # Parameters
    ///
    /// index: The index of the field
    fn add_element(self, index: usize) -> Error {
        let location = format!("[{}]{}", index, self.location);

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

impl From<data_model::Error> for Error {
    fn from(value: data_model::Error) -> Self {
        return Error {
            location: value.location.clone(),
            error: ErrorCore::MacroError(value),
        };
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
    /// Serialization model has an incompatible type
    #[error("The serialization model {:?} is incompatible with the type: {:}", .0, .1)]
    SerializationModel(data_model::SerializationModel, String),
    /// Unable to convert to boolean
    #[error("Unable to convert \"{:}\" to a boolean", .0)]
    BoolConversion(String),
    /// Unable to convert to integer
    #[error("Unable to convert \"{:}\" to an integer", .0)]
    IntegerConversion(String),
    /// Unable to convert to float
    #[error("Unable to convert \"{:}\" to a float", .0)]
    FloatConversion(String),
    /// Unable to convert to enum
    #[error("Unable to convert \"{:?}\" to an enum", .0)]
    EnumConversion(String),
    /// Unable to convert to typed enum
    #[error("Unable to convert \"{:?}\" to a typed enum", .0)]
    TypedEnumConversion(HashMap<String, data_model::SerializationModel>),
    /// The map had more or less than one element when converting to a typed enum
    #[error("Unable to convert \"{:?}\" to a typed enum because it did not have a single field", .0)]
    TypedEnumLayout(HashMap<String, data_model::SerializationModel>),
    /// Unable to convert to variant
    #[error("Unable to convert \"{:?}\" to a variant with the following errors: {:?}", .0, .1)]
    VariantConversion(data_model::SerializationModel, Vec<(String, Error)>),
    /// Unable to convert to struct due to missing field
    #[error("Unable to convert \"{:?}\" to an struct because it is missing field {:}", .0, .1)]
    StructConversionMissingField(HashMap<String, data_model::SerializationModel>, String),
    /// Unable to convert to struct due to excess fields
    #[error("Unable to convert \"{:?}\" to an struct because it has excess fields", .0)]
    StructConversionExcessFields(HashMap<String, data_model::SerializationModel>),
    /// Error expanding macros
    #[error("An error occured when expanding macros: {:?}", .0)]
    MacroError(data_model::Error),
}
