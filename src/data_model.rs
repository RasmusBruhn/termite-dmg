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

/// Supplies the type sepcific information for a data type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DataTypeData {
  /// Describes a struct
  Struct(Struct),
  /// Describes an array
  Array(Array),
}

/// The type specific information for a struct
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Struct {
  /// A list of all the fields of the struct
  pub fields: Vec<StructField>,
}

/// Describes all data for one field of a struct
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
  /// A list of possible constraints which should always be true
  pub constraints: Vec<String>,
}

/// The type specific infomation for an array
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Array {
  /// The data type for all elements
  pub data_type: String,
  /// All the constraints that all elements must uphold
  pub constraints: Vec<String>,
}

/// The type specific data for a variant
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Variant {
  /// The list of data types the variant can be
  pub data_types: Vec<String>,
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
