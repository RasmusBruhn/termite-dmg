//!
//! The Termite Data Model Generator is a crate for generating boiler plate code
//! for data models.
//! 
//! The crate consits of two parts, the first is the data model itself. This is
//! preferably imported from a yaml or json file into a DataModel object.
//! 
//! It can also be defined directly in code, however, this is not as readable or
//! easy to write.
//! 
//! A DataModel object consist of a map of header and footer strings and the
//! data types. The header and footer string are strings to add to the top of
//! the generated files like for adding includes. The data types is a list of
//! names and data for the user defined types defined for the data model and
//! these are used to generated the data model file.
//! 
//! A data type consists of a base type (like struct, variant or enum) and a
//! description and then it has the type data which describes all the specific
//! information for that type.
//! 
//! The base types are:
//! 
//! Struct: A normal struct with a number of public fields (like a rust/c++
//! struct)
//! 
//! Array: A list of objects of the same type (like a rust/c++ vector)
//! 
//! Variant: Can be any of a number of different types, when parsing a variant
//! it will attempt to parse the types from the beginning and stop when one is
//! successful (like a c++ variant)
//! 
//! Enum: Can be any of a number of predefined enum types, each enum type can
//! hold extra data by wrapping another type (like a rust enum)
//! 
//! ConstrainedType: Wraps another type and enforces constraints which only
//! allows parsing if the constraints are respected.
//! 
//! The second part of the Termite crate is generating the code. For now it only
//! supports c++ with the cpp module. From here there are functions to generate
//! header files, (de)serialization and documentation.
//! 
//! # Examples:
//! 
//! ```
//! use termite_dmg as termite;
//! use indoc::formatdoc;
//! 
//! let yaml_model = formatdoc!("
//!   data_types:
//!   - name: PositiveDouble
//!     data: !ConstrainedType
//!       data_type: double
//!       constraints:
//!       - x > 0.0
//!   - name: Point
//!     description: A point in 2D space
//!     data: !Struct
//!       fields:
//!       - name: x
//!         data_type: double
//!         default: !Default '0.0'
//!       - name: y
//!         data_type: double
//!         default: !Default '0.0'
//!       - name: id
//!         data_type: int64_t
//!         default: Optional
//!   - name: Size
//!     description: The size of a box
//!     data: !Struct
//!       fields:
//!       - name: w
//!         description: The width
//!         data_type: PositiveDouble
//!         default: Required
//!       - name: h
//!         description: The height
//!         data_type: PositiveDouble
//!         default: Required
//!   - name: SizeVariant
//!     description: Is either a Size or just a PositiveDouble if it is a square
//!     data: !Variant
//!       data_types:
//!       - PositiveDouble
//!       - Size
//!   - name: SizeArray
//!     data: !Array
//!       data_type: SizeVariant
//!   - name: Geometry
//!     data: !Enum
//!       types:
//!       - name: Nothing
//!         description: No geometry
//!       - name: Sizes
//!         description: A number of sizes
//!         data_type: SizeArray
//!       - name: Point
//!         description: A point
//!         data_type: Point
//!   headers:
//!     cpp: // My Header
//!   footers:
//!     cpp: // My Header
//!   namespace:
//!   - my_namespace
//! ");
//! 
//! let model = termite::DataModel::import_yaml(&yaml_model).unwrap();
//! let cpp_model = termite::cpp::DataModel::new(model).unwrap();
//! 
//! let termite_hpp = termite::cpp::get_termite_dependency();
//! let termite_yaml_hpp = termite::cpp::get_yaml_interface();
//! let model_hpp = cpp_model.get_source("HEADER_GUARD", 2);
//! ```
//! 

mod data_model;
pub mod cpp;

pub use data_model::{DataModel, DataType, DataTypeData, Struct, StructField, DefaultType, Array, Variant, Enum, EnumType, ConstrainedType};

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
            fields: vec![
              StructField {
                name: "x".to_string(),
                description: None,
                data_type: "double".to_string(),
                default: DefaultType::Default("0.0".to_string()),
              },
              StructField {
                name: "y".to_string(),
                description: None,
                data_type: "double".to_string(),
                default: DefaultType::Default("0.0".to_string()),
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
