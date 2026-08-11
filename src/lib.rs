#![doc = include_str!("../README.md")]

pub mod cpp;
pub mod dart;

mod data_model;
pub use data_model::{
    Array, ConstrainedType, Constraint, DataModel, DataType, DataTypeData, DefaultType, Enum,
    EnumType, Struct, StructField, Variant,
};

mod error;
pub use error::{Error, ErrorCore};

mod serialization_model;
pub use serialization_model::{expand_macros, SerializationModel};

mod sort;
pub use sort::sort_data_types;

mod name;
pub use name::{is_name_builtin, validate_name};

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashMap, fs};

    #[test]
    fn main_model() {
        let model = DataModel {
            namespace: vec!["my_namespace".to_string()],
            macros: HashMap::from([
                (
                    "DEFAULT_COORDINATE".to_string(),
                    SerializationModel::Value("0.0".to_string()),
                ),
                (
                    "MESSAGE".to_string(),
                    SerializationModel::Value("This is a macro message".to_string()),
                ),
            ]),
            headers: HashMap::from([
                (
                    "cpp-header".to_string(),
                    "// My .h Header with message: $MESSAGE$".to_string(),
                ),
                (
                    "cpp-source".to_string(),
                    "// My .cpp Header and this is a dollar sign: $$".to_string(),
                ),
            ]),
            footers: HashMap::from([
                ("cpp-header".to_string(), "// My .h Footer".to_string()),
                ("cpp-source".to_string(), "// My .cpp Footer".to_string()),
            ]),
            data_types: HashMap::from([
                (
                    "PositiveDouble".to_string(),
                    DataType {
                        description: None,
                        data: DataTypeData::ConstrainedType(ConstrainedType {
                            data_type: "number".to_string(),
                            constraints: vec![Constraint::Arithmetic("x > 0.0".to_string())],
                        }),
                    },
                ),
                (
                    "Point".to_string(),
                    DataType {
                        description: Some("A point in 2D space".to_string()),
                        data: DataTypeData::Struct(Struct {
                            inherit: None,
                            fields: HashMap::from([
                                (
                                    "x".to_string(),
                                    StructField {
                                        description: None,
                                        data_type: "number".to_string(),
                                        default: DefaultType::Default(SerializationModel::Value(
                                            "0.0".to_string(),
                                        )),
                                    },
                                ),
                                (
                                    "y".to_string(),
                                    StructField {
                                        description: None,
                                        data_type: "number".to_string(),
                                        default: DefaultType::Default(SerializationModel::Value(
                                            "$DEFAULT_COORDINATE$".to_string(),
                                        )),
                                    },
                                ),
                                (
                                    "id".to_string(),
                                    StructField {
                                        description: None,
                                        data_type: "integer".to_string(),
                                        default: DefaultType::Optional,
                                    },
                                ),
                            ]),
                        }),
                    },
                ),
                (
                    "Size".to_string(),
                    DataType {
                        description: Some("The size of a box".to_string()),
                        data: DataTypeData::Struct(Struct {
                            inherit: None,
                            fields: HashMap::from([
                                (
                                    "w".to_string(),
                                    StructField {
                                        description: Some("The width".to_string()),
                                        data_type: "PositiveDouble".to_string(),
                                        default: DefaultType::Required,
                                    },
                                ),
                                (
                                    "h".to_string(),
                                    StructField {
                                        description: Some("The height".to_string()),
                                        data_type: "PositiveDouble".to_string(),
                                        default: DefaultType::Required,
                                    },
                                ),
                            ]),
                        }),
                    },
                ),
                (
                    "SizeVariant".to_string(),
                    DataType {
                        description: Some(
                            "Is either a Size or just a PositiveDouble if it is a square"
                                .to_string(),
                        ),
                        data: DataTypeData::Variant(Variant {
                            data_types: vec!["PositiveDouble".to_string(), "Size".to_string()],
                        }),
                    },
                ),
                (
                    "SizeArray".to_string(),
                    DataType {
                        description: None,
                        data: DataTypeData::Array(Array {
                            data_type: "SizeVariant".to_string(),
                        }),
                    },
                ),
                (
                    "Geometry".to_string(),
                    DataType {
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
                ),
                (
                    "NamedGeometry".to_string(),
                    DataType {
                        description: None,
                        data: DataTypeData::Struct(Struct {
                            fields: HashMap::from([
                                (
                                    "geometry".to_string(),
                                    StructField {
                                        description: Some("The geometry data".to_string()),
                                        data_type: "Geometry".to_string(),
                                        default: DefaultType::Default(SerializationModel::Map(
                                            HashMap::from([(
                                                "Point".to_string(),
                                                SerializationModel::Map(HashMap::from([
                                                    (
                                                        "x".to_string(),
                                                        SerializationModel::Value(
                                                            "1.0".to_string(),
                                                        ),
                                                    ),
                                                    (
                                                        "id".to_string(),
                                                        SerializationModel::Value("0".to_string()),
                                                    ),
                                                ])),
                                            )]),
                                        )),
                                    },
                                ),
                                (
                                    "name".to_string(),
                                    StructField {
                                        description: Some("The name of the geometry".to_string()),
                                        data_type: "string".to_string(),
                                        default: DefaultType::Required,
                                    },
                                ),
                            ]),
                            inherit: None,
                        }),
                    },
                ),
            ]),
        };

        let import = fs::read_to_string("tests/yaml_import.yaml").unwrap();
        let import_model = DataModel::import_yaml(&import).unwrap();

        assert_eq!(model, import_model);
    }
}
