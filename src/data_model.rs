use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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
  Default(String),
}
