use crate::{DataModel, Instance};
use indoc::{formatdoc, indoc};
use thiserror::Error;

/// This functions generates the header file including all structs, variants and enums used in the data model
/// along with printing and comparison support
/// 
/// # Parameters
/// 
/// name: The name of the file, used in the header guard to bu {name}_H_INCLUDED
/// 
/// data_model: The data model to generate
/// 
/// # Errors
/// 
/// Returns a HeaderError if there are any errors
pub fn header(name: &str, data_model: &DataModel) -> Result<String, HeaderError> {
    // Initialize the header file
    let mut output = String::new();
    output += formatdoc!("
        #ifndef {name}_H_INCLUDED
        #define {name}_H_INCLUDED

        #include <optional>
        #include <variant>
    ", name = name).as_str();

    let mut custom_header = data_model.get_header("header").to_string();
    if !custom_header.is_empty() {
        custom_header += "\n";
    }
    output += custom_header.as_str();

    // Add objects
    for (type_name, type_data) in data_model.iter() {
        // Add newline
        output += "\n";

        // Add description
        if let Some(description) = &type_data.description {
            output += formatdoc!("
                /**
                 * \\brief {desc}
                 * 
                 */
            ", desc = description).as_str();    
        }
    
        // Add code
        output = match type_data.base_type.as_str() {
            "struct" => header_struct(type_name, &type_data.data, output)?,
            _ => return Err(HeaderError::UnknownType(type_data.base_type.clone())),
        }
    };

    // Add footer
    output += indoc!("

        #endif
    ");

    return Ok(output);
}

/// Generates the code for a single struct, returns output with the struct appended
/// 
/// # Parameters
/// 
/// name: The name of the struct
/// 
/// fields: The data for all the struct fields
/// 
/// output: The outut string from before the struct
/// 
/// # Errors
/// 
/// Returns a HeaderError if any error occures
fn header_struct(name: &str, fields: &Vec<(String, Instance)>, mut output: String) -> Result<String, HeaderError> {
    // Create class name
    output += formatdoc!("
        struct {name} {{
    ", name = name).as_str();

    // Add fields
    for (field_name, field_data) in fields.iter() {
        // Get the data
        let data_type = field_data.data.get("data_type").ok_or(HeaderError::DataType(name.to_string(), field_name.to_string()))?;

        // Add description
        if let Some(description) = field_data.data.get("description") {
            output += formatdoc!("
                {0:indent$}/**
                {0:indent$} * \\brief {desc}
                {0:indent$} * 
                {0:indent$} */
            ", "", indent = 2, desc = description).as_str();    
        }

        // Create default value
        let default_value = if let Some(default_value) = field_data.data.get("default") {
            format!(" = {value}", value = default_value)
        } else {
            "".to_string()
        };

        // Create default value
        let typename = if let Some(typename) = field_data.data.get("optional") {
            let boolean: bool = if let Ok(boolean) = typename.parse() {
                boolean
            } else {
                return Err(HeaderError::ParseBoolean(name.to_string(), field_name.clone(), "optional".to_string()));
            };
    
            if boolean {
                format!("std::optional<{data_type}>")
            } else {
                data_type.clone()
            }
        } else {
            data_type.clone()
        };
        
        // Write the field
        output += formatdoc!("
            {0:indent$}{typename} {name}{default};
        ", "", indent = 2, typename = typename, name = field_name, default = default_value).as_str();    
    };

    // End class
    output += indoc!("
        };
    ");

    return Ok(output);
}

/// Any error which may occur during generation of a header file
#[derive(Error, Debug, Clone)]
pub enum HeaderError {
    /// The data object type is not recognized
    #[error("The data object type \"{:?}\" cannot be used in a c++ header file", .0)]
    UnknownType(String),
    /// The data type is not supplied
    #[error("The data type must be supplied for {:?}.{:?}", .0, .1)]
    DataType(String, String),
    /// The boolean could not be parsed
    #[error("Unable to pass boolean for {:?}.{:?}.{:?}", .0, .1, .2)]
    ParseBoolean(String, String, String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DataModel, DataType};
    use std::collections::HashMap;

    #[test]
    fn struct_basic() {
        // Create the data model
        let data_model = DataModel::from_vec(
            vec![("CustomType".to_string(), DataType {
                base_type: "struct".to_string(),
                description: None,
                data: vec![
                    ("field1".to_string(), Instance { data: HashMap::from([
                        ("data_type".to_string(), "int".to_string()),
                    ])}),
                    ("field2".to_string(), Instance { data: HashMap::from([
                        ("data_type".to_string(), "float".to_string()),
                    ])}),
                ]
        })]).expect("Dublicate key");
        
        // Create the header file
        let header_file = header("HEADER", &data_model).unwrap();

        let expected = formatdoc!("
            #ifndef HEADER_H_INCLUDED
            #define HEADER_H_INCLUDED
            
            #include <optional>
            #include <variant>

            struct CustomType {{
              int field1;
              float field2;
            }};

            #endif
        ");

        assert_eq!(header_file, expected);
    }

    #[test]
    fn struct_header() {
        // Create the data model
        let mut data_model = DataModel::from_vec(
            vec![("CustomType".to_string(), DataType {
                base_type: "struct".to_string(),
                description: None,
                data: vec![
                    ("field1".to_string(), Instance { data: HashMap::from([
                        ("data_type".to_string(), "int".to_string()),
                    ])}),
                    ("field2".to_string(), Instance { data: HashMap::from([
                        ("data_type".to_string(), "float".to_string()),
                    ])}),
                ]
        })]).expect("Dublicate key");
        data_model.add_header("header", "#include \"sample.h\"");
        
        // Create the header file
        let header_file = header("HEADER", &data_model).unwrap();

        let expected = formatdoc!("
            #ifndef HEADER_H_INCLUDED
            #define HEADER_H_INCLUDED
            
            #include <optional>
            #include <variant>
            #include \"sample.h\"

            struct CustomType {{
              int field1;
              float field2;
            }};

            #endif
        ");

        assert_eq!(header_file, expected);
    }

    #[test]
    fn struct_description() {
        // Create the data model
        let data_model = DataModel::from_vec(
            vec![("CustomType".to_string(), DataType {
                base_type: "struct".to_string(),
                description: Some("CustomDescription".to_string()),
                data: vec![
                    ("field1".to_string(), Instance { data: HashMap::from([
                        ("data_type".to_string(), "int".to_string()),
                        ("description".to_string(), "FieldDescription1".to_string()),
                    ])}),
                    ("field2".to_string(), Instance { data: HashMap::from([
                        ("data_type".to_string(), "float".to_string()),
                        ("description".to_string(), "FieldDescription2".to_string()),
                    ])}),
                ]
        })]).expect("Dublicate key");
        
        // Create the header file
        let header_file = header("HEADER", &data_model).unwrap();

        let expected = formatdoc!("
            #ifndef HEADER_H_INCLUDED
            #define HEADER_H_INCLUDED

            #include <optional>
            #include <variant>

            /**
             * \\brief CustomDescription
             * 
             */
            struct CustomType {{
              /**
               * \\brief FieldDescription1
               * 
               */
              int field1;
              /**
               * \\brief FieldDescription2
               * 
               */
              float field2;
            }};

            #endif
        ");

        assert_eq!(header_file, expected);
    }

    #[test]
    fn struct_default() {
        // Create the data model
        let data_model = DataModel::from_vec(
            vec![("CustomType".to_string(), DataType {
                base_type: "struct".to_string(),
                description: None,
                data: vec![
                    ("field1".to_string(), Instance { data: HashMap::from([
                        ("data_type".to_string(), "int".to_string()),
                        ("default".to_string(), "42".to_string()),
                    ])}),
                    ("field2".to_string(), Instance { data: HashMap::from([
                        ("data_type".to_string(), "float".to_string()),
                        ("default".to_string(), "3.14159".to_string()),
                    ])}),
                ]
        })]).expect("Dublicate key");
        
        // Create the header file
        let header_file = header("HEADER", &data_model).unwrap();

        let expected = formatdoc!("
            #ifndef HEADER_H_INCLUDED
            #define HEADER_H_INCLUDED

            #include <optional>
            #include <variant>

            struct CustomType {{
              int field1 = 42;
              float field2 = 3.14159;
            }};

            #endif
        ");

        assert_eq!(header_file, expected);
    }

    #[test]
    fn struct_optional() {
        // Create the data model
        let data_model = DataModel::from_vec(
            vec![("CustomType".to_string(), DataType {
                base_type: "struct".to_string(),
                description: None,
                data: vec![
                    ("field1".to_string(), Instance { data: HashMap::from([
                        ("data_type".to_string(), "int".to_string()),
                        ("optional".to_string(), "true".to_string()),
                    ])}),
                    ("field2".to_string(), Instance { data: HashMap::from([
                        ("data_type".to_string(), "float".to_string()),
                        ("optional".to_string(), "true".to_string()),
                    ])}),
                ]
        })]).expect("Dublicate key");
        
        // Create the header file
        let header_file = header("HEADER", &data_model).unwrap();

        let expected = formatdoc!("
            #ifndef HEADER_H_INCLUDED
            #define HEADER_H_INCLUDED
            
            #include <optional>
            #include <variant>

            struct CustomType {{
              std::optional<int> field1;
              std::optional<float> field2;
            }};

            #endif
        ");

        assert_eq!(header_file, expected);
    }
}

