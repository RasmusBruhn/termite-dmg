use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Helper function to deserialize a `usize` from either a string or a number in the input data
fn deserialize_usize_from_string_or_number<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum UsizeOrString {
        Number(usize),
        String(String),
    }

    match UsizeOrString::deserialize(deserializer)? {
        UsizeOrString::Number(value) => Ok(value),
        UsizeOrString::String(value) => value
            .parse::<usize>()
            .map_err(|_| serde::de::Error::custom(format!("invalid usize string \"{value}\""))),
    }
}

/// An entire data model
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]

pub struct DataModel {
    /// List of the the data types to implement
    pub data_types: HashMap<String, DataType>,
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
        return serde_yaml::from_value(sanitize_yaml(serde_yaml::from_str(mode)?));
    }

    /// Imports a data model from a json string
    pub fn import_json(mode: &str) -> Result<DataModel, serde_json::Error> {
        return serde_json::from_value(sanitize_json(serde_json::from_str(mode)?));
    }
}

fn sanitize_yaml(value: serde_yaml::Value) -> serde_yaml::Value {
    match value {
        serde_yaml::Value::Bool(value) => {
            if value {
                serde_yaml::Value::String("true".to_string())
            } else {
                serde_yaml::Value::String("false".to_string())
            }
        }
        serde_yaml::Value::Mapping(value) => serde_yaml::Value::Mapping(
            value
                .into_iter()
                .map(|(k, v)| (k, sanitize_yaml(v)))
                .collect(),
        ),
        serde_yaml::Value::Number(value) => serde_yaml::Value::String(value.to_string()),
        serde_yaml::Value::Sequence(value) => {
            serde_yaml::Value::Sequence(value.into_iter().map(sanitize_yaml).collect())
        }
        serde_yaml::Value::Tagged(value) => {
            serde_yaml::Value::Tagged(Box::new(serde_yaml::value::TaggedValue {
                tag: value.tag,
                value: sanitize_yaml(value.value),
            }))
        }
        _ => value,
    }
}

fn sanitize_json(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Bool(value) => {
            if value {
                serde_json::Value::String("true".to_string())
            } else {
                serde_json::Value::String("false".to_string())
            }
        }
        serde_json::Value::Object(value) => serde_json::Value::Object(
            value
                .into_iter()
                .map(|(k, v)| (k, sanitize_json(v)))
                .collect(),
        ),
        serde_json::Value::Number(value) => serde_json::Value::String(value.to_string()),
        serde_json::Value::Array(value) => {
            serde_json::Value::Array(value.into_iter().map(sanitize_json).collect())
        }
        _ => value,
    }
}

/// Any data type (struct, variant, ect.)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataType {
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
    /// A map of all fields in the struct, the key is the name of the field
    pub fields: HashMap<String, StructField>,
    /// The name of a different Struct this Struct builds onto, used in Schema
    /// generation
    pub inherit: Option<String>,
}

/// The data for a single field in a struct
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructField {
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
    pub constraints: Vec<Constraint>,
}

/// Defines a single constraint
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Constraint {
    /// The minimum length of a string (unicode code points) or array, invalid
    /// for any other types
    MinLength(#[serde(deserialize_with = "deserialize_usize_from_string_or_number")] usize),
    /// The maximum length of a string (unicode code points) or array, invalid
    /// for any other types
    MaxLength(#[serde(deserialize_with = "deserialize_usize_from_string_or_number")] usize),
    /// Any constraint using c-like arithmetic, must result in a boolean value
    Arithmetic(String),
    /// Name of a function to call f_name(x), must return a boolean value, the
    /// name must be a C++ style with possible namespaces
    Function(String),
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

/// A simplified representation of a data type. Any constrained type can be
/// reduced to its inner type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SimplifiedType {
    Boolean,
    Integer,
    Float,
    String,
    Array,
    Struct,
    Enum,
    Variant,
}

/// Returns the simplified type for a given data type name
///
/// # Parameters
///
/// name: The name of the data type to simplify
///
/// all_types: A hashmap containing all data types by their names
pub fn get_simplified_type(
    name: &str,
    all_types: &HashMap<String, DataType>,
) -> Result<SimplifiedType, Error> {
    return match name {
        "boolean" => Ok(SimplifiedType::Boolean),
        "integer" => Ok(SimplifiedType::Integer),
        "number" => Ok(SimplifiedType::Float),
        "string" => Ok(SimplifiedType::String),
        _ => {
            if let Some(data_type) = all_types.get(name) {
                match &data_type.data {
                    DataTypeData::Array(_) => Ok(SimplifiedType::Array),
                    DataTypeData::Struct(_) => Ok(SimplifiedType::Struct),
                    DataTypeData::Enum(_) => Ok(SimplifiedType::Enum),
                    DataTypeData::Variant(_) => Ok(SimplifiedType::Variant),
                    DataTypeData::ConstrainedType(data) => {
                        get_simplified_type(&data.data_type, all_types)
                    }
                }
            } else {
                Err(Error::new(ErrorCore::UnknownDataType(name.to_string())))
            }
        }
    };
}
