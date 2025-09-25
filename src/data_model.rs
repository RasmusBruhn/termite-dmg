use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

/// An entire data model
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataModel {
    /// List of the the data types to implement
    pub data_types: Vec<DataType>,
    /// List of all header data used to include external packages
    pub headers: HashMap<String, String>,
    /// List of all footer data
    pub footers: HashMap<String, String>,
    /// The nested namespace to put the data model into
    pub namespace: Vec<String>,
    /// A set of replacement macros to use for default values
    pub macros: HashMap<String, SerializationModel>,
}

impl DataModel {
    /// Exports the data model to a yaml string
    pub fn export_yaml(&self) -> Result<String, serde_yaml::Error> {
        return serde_yaml::to_string(self);
    }

    /// Exports the data model to a json string
    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        return serde_json::to_string(self);
    }

    /// Imports a data model from a yaml string
    pub fn import_yaml(mode: &str) -> Result<DataModel, serde_yaml::Error> {
        return serde_yaml::from_str(mode);
    }

    /// Imports a data model from a json string
    pub fn import_json(mode: &str) -> Result<DataModel, serde_json::Error> {
        return serde_json::from_str(mode);
    }
}

/// Any data type (struct, variant, ect.)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataType {
    /// The name of the type
    pub name: String,
    /// The description of the type
    pub description: Option<String>,
    /// The type specific data
    pub data: DataTypeData,
}

/// Supplies the type specific information for a data type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DataTypeData {
    /// Describes a struct
    Struct(Struct),
    /// Describes an array
    Array(Array),
    /// Describes a variant
    Variant(Variant),
    /// Describes an enum
    Enum(Enum),
    /// Describes a constrained type
    ConstrainedType(ConstrainedType),
}

/// A struct which has a number of fields
///
/// It will automatically add a termite::Node::Map field called extra_fields
/// which holds all fields which were not captured when parsing
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Struct {
    /// A list of all the fields of the struct
    pub fields: Vec<StructField>,
    /// The name of a different Struct this Struct builds onto, used in Schema
    /// generation
    pub inherit: Option<String>,
}

/// The data for a single field in a struct
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructField {
    /// The name of the field
    pub name: String,
    /// The description of the field
    pub description: Option<String>,
    /// What type the field is, without Option<>
    pub data_type: String,
    /// A default value if it it not required
    pub default: DefaultType,
}

/// An array of values of the same data type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Array {
    /// The data type for all elements
    pub data_type: String,
}

/// A variant which can be any of a number of different types, when parsing it
/// will attempt to parse all types from the start until it is successful
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Variant {
    /// The list of data types the variant can be
    pub data_types: Vec<String>,
}

/// An enum, includes a number of enum values
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Enum {
    /// All the possible enum values
    pub types: Vec<EnumType>,
}

/// An enum value, describes a specific enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnumType {
    /// The name of this enum type
    pub name: String,
    /// The description describing this enum type
    pub description: Option<String>,
    /// The type this enum type is wrapping, may be omitted for an empty type
    pub data_type: Option<String>,
}

/// A constrained type, wraps any other type and adds constraints onto them
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConstrainedType {
    /// The type that is constrained
    pub data_type: String,
    /// All extra constraints for the type, must be written as an expression where
    /// the constrained value is denoted x
    pub constraints: Vec<String>,
}

/// Describes whether a field is required or optional
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DefaultType {
    /// The field must be supplied
    Required,
    /// The field can be supplied, the type of the field will be
    /// Option<data_type>, if not supplied it defaults to None
    Optional,
    /// The field can be supplied, if not supplied it defaults to the default
    /// value
    Default(SerializationModel),
}

/// A generic serialization model which can be used to serialize any data model
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SerializationModel {
    /// A generic key-value pair map where the key must be a string
    Map(HashMap<String, SerializationModel>),
    /// An array of other serialization models
    Array(Vec<SerializationModel>),
    /// A single value, must be a string
    Value(String),
}

pub(crate) fn expand_macros<'a>(
    value: &SerializationModel,
    macros: &'a HashMap<String, SerializationModel>,
    used_macros: &mut HashSet<&'a str>,
) -> Result<SerializationModel, Error> {
    return match value {
        SerializationModel::Map(value) => value
            .iter()
            .map(|(k, v)| match expand_macros(v, macros, used_macros) {
                Ok(value) => Ok((k.clone(), value)),
                Err(error) => Err(error.add_field(k)),
            })
            .collect::<Result<HashMap<_, _>, _>>()
            .map(SerializationModel::Map),
        SerializationModel::Array(value) => value
            .iter()
            .enumerate()
            .map(|(i, v)| match expand_macros(v, macros, used_macros) {
                Ok(value) => Ok(value),
                Err(error) => Err(error.add_element(i)),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(SerializationModel::Array),
        SerializationModel::Value(value) => {
            // Do a full macro insert if the string is just a macro definition
            if value.starts_with('$')
                && value.ends_with('$')
                && value.len() > 2
                && value.chars().filter(|c| *c == '$').count() == 2
            {
                let macro_name = &value[1..value.len() - 1];

                // Prevent infinite recursion
                if used_macros.contains(macro_name) {
                    return Err(Error {
                        location: "".to_string(),
                        error: ErrorCore::RecursiveMacro(macro_name.to_string()),
                    });
                }

                // Insert the macro
                return if let Some((macro_key, macro_value)) = macros.get_key_value(macro_name) {
                    used_macros.insert(macro_key.as_str());
                    let expanded_macro = expand_macros(macro_value, macros, used_macros);
                    used_macros.remove(macro_key.as_str());
                    match expanded_macro {
                        Ok(value) => Ok(value),
                        Err(error) => Err(error.add_macro(macro_name)),
                    }
                } else {
                    Err(Error {
                        location: "".to_string(),
                        error: ErrorCore::MissingMacro(macro_name.to_string()),
                    })
                };
            }

            // Otherwise do a partial macro insertion
            let mut expanded_string = String::new();
            let mut current_index = 0;
            while current_index < value.len() {
                // Find the beginning of the next macro
                if let Some(start_index) = value[current_index..].find('$') {
                    let start_index = start_index + current_index + 1;
                    expanded_string.push_str(&value[current_index..start_index - 1]);

                    // Skip if it should just be interpreted as a dollar sign
                    if start_index < value.len() && &value[start_index..start_index + 1] == "$" {
                        expanded_string.push('$');
                        current_index = start_index + 1;
                        continue;
                    }

                    // Find the end of the macro
                    if let Some(end_index) = value[start_index..].find('$') {
                        let end_index = end_index + start_index;
                        let macro_name = &value[start_index..end_index];

                        // Prevent infinite recursion
                        if used_macros.contains(macro_name) {
                            return Err(Error {
                                location: "".to_string(),
                                error: ErrorCore::RecursiveMacro(macro_name.to_string()),
                            });
                        }

                        if let Some((macro_key, macro_value)) = macros.get_key_value(macro_name) {
                            // Insert the macro
                            used_macros.insert(macro_key.as_str());
                            let expanded_macro = expand_macros(macro_value, macros, used_macros);
                            used_macros.remove(macro_key.as_str());
                            match expanded_macro {
                                Ok(ok_value) => match ok_value {
                                    SerializationModel::Value(value) => {
                                        expanded_string.push_str(&value);
                                    }
                                    _ => {
                                        return Err(Error {
                                            location: "".to_string(),
                                            error: ErrorCore::PartialMacro(
                                                macro_name.to_string(),
                                                value.clone(),
                                            ),
                                        });
                                    }
                                },
                                Err(error) => {
                                    return Err(error.add_macro(macro_name));
                                }
                            }
                        } else {
                            return Err(Error {
                                location: "".to_string(),
                                error: ErrorCore::MissingMacro(macro_name.to_string()),
                            });
                        }

                        current_index = end_index + 1;
                    } else {
                        return Err(Error {
                            location: "".to_string(),
                            error: ErrorCore::IncompleteMacro(value.clone()),
                        });
                    }
                }
            }

            Ok(SerializationModel::Value(expanded_string))
        }
    };
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

    /// Sets the current location to be the element of a field of the given base
    ///
    /// # Parameters
    ///
    /// index: The index of the field
    fn add_macro(self, index: &str) -> Error {
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

/// Errors for when converting generic data models into JSON schema data models
#[derive(thiserror::Error, Debug, Clone)]
pub enum ErrorCore {
    /// Macros used recursively
    #[error("The macro \"{}\" is used recursively", .0)]
    RecursiveMacro(String),
    /// Macro is missing
    #[error("The macro \"{}\" is not defined", .0)]
    MissingMacro(String),
    /// Macro is incomplete
    #[error("The string \"{}\" begins a macro without ending it", .0)]
    IncompleteMacro(String),
    /// A partial macro insertion can only have a string value
    #[error("The partial macro insertion of \"{}\" in \"{}\" must be a string", .0, .1)]
    PartialMacro(String, String),
}
