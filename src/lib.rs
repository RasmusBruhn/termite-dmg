#![doc = include_str!("../README.md")]

mod data_model;
pub mod cpp;
pub mod schema;

pub use data_model::{DataModel, DataType, DataTypeData, Struct, StructField, DefaultType, Array, Variant, Enum, EnumType, ConstrainedType, SerializationModel};

#[cfg(test)]
mod tests {
  use super::*;
  use std::{
    fs,
    collections::HashMap,
  };

  #[test]
  fn main_model() {
    let model = DataModel {
      namespace: vec!["my_namespace".to_string()],
      headers: HashMap::from([("cpp".to_string(), "// My Header".to_string())]),
      footers: HashMap::from([("cpp".to_string(), "// My Header".to_string())]),
      macros: HashMap::new(),
      data_types: vec![
        DataType {
          name: "PositiveDouble".to_string(),
          description: None,
          data: DataTypeData::ConstrainedType(ConstrainedType {
            data_type: "double".to_string(),
            constraints: vec![
              "x > 0.0".to_string(),
            ],
          }),
        },
        DataType {
          name: "Point".to_string(),
          description: Some("A point in 2D space".to_string()),
          data: DataTypeData::Struct(Struct {
            inherit: None,
            fields: vec![
              StructField {
                name: "x".to_string(),
                description: None,
                data_type: "double".to_string(),
                default: DefaultType::Default(SerializationModel::Value("0.0".to_string())),
              },
              StructField {
                name: "y".to_string(),
                description: None,
                data_type: "double".to_string(),
                default: DefaultType::Default(SerializationModel::Value("0.0".to_string())),
              },
              StructField {
                name: "id".to_string(),
                description: None,
                data_type: "int64_t".to_string(),
                default: DefaultType::Optional,
              },
            ],
          }),
        },
        DataType {
          name: "Size".to_string(),
          description: Some("The size of a box".to_string()),
          data: DataTypeData::Struct(Struct {
            inherit: None,
            fields: vec![
              StructField {
                name: "w".to_string(),
                description: Some("The width".to_string()),
                data_type: "PositiveDouble".to_string(),
                default: DefaultType::Required,
              },
              StructField {
                name: "h".to_string(),
                description: Some("The height".to_string()),
                data_type: "PositiveDouble".to_string(),
                default: DefaultType::Required,
              },
            ],
          }),
        },
        DataType {
          name: "SizeVariant".to_string(),
          description: Some("Is either a Size or just a PositiveDouble if it is a square".to_string()),
          data: DataTypeData::Variant(Variant {
            data_types: vec![
              "PositiveDouble".to_string(),
              "Size".to_string(),
            ],
          }),
        },
        DataType {
          name: "SizeArray".to_string(),
          description: None,
          data: DataTypeData::Array(Array {
            data_type: "SizeVariant".to_string(),
          }),
        },
        DataType {
          name: "Geometry".to_string(),
          description: None,
          data: DataTypeData::Enum(Enum {
            types: vec![
              EnumType {
                name: "Nothing".to_string(),
                description: Some("No geometry".to_string()),
                data_type: None,
              },
              EnumType {
                name: "Sizes".to_string(),
                description: Some("A number of sizes".to_string()),
                data_type: Some("SizeArray".to_string()),
              },
              EnumType {
                name: "Point".to_string(),
                description: Some("A point".to_string()),
                data_type: Some("Point".to_string()),
              },
            ],
          }),
        },
      ],
    };

    let yaml_model = model.export_yaml().unwrap();
    fs::write("target/yaml_example.yaml", yaml_model).unwrap();

    let import = fs::read_to_string("tests/yaml_import.yaml").unwrap();
    let import_model = DataModel::import_yaml(&import).unwrap();

    assert_eq!(model, import_model);
  }
}
