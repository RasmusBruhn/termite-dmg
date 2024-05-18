use crate::{DataModel, Instance};
use indoc::{formatdoc, indoc};
use thiserror::Error;

pub fn header(name: &str, data_model: &DataModel) -> Result<String, HeaderError> {
    // Initialize the header file
    let mut output = String::new();
    output += formatdoc!("
        #ifndef {name}_H_INCLUDED
        #define {name}_H_INCLUDED
    ", name = name).as_str();

    // Add objects
    for (type_name, type_data) in data_model.iter() {
        output = match type_data.base_type.as_str() {
            "struct" => header_struct(type_name, &type_data.fields, output)?,
            _ => return Err(HeaderError::UnknownType(type_data.base_type.clone())),
        }
    };

    // Add footer
    output += indoc!("

        #endif
    ");

    return Ok(output);
}

fn header_struct(name: &str, fields: &Vec<(String, Instance)>, mut output: String) -> Result<String, HeaderError> {
    // Create class name
    output += formatdoc!("

        struct {name} {{
    ", name = name).as_str();

    // Add fields
    for (field_name, field_data) in fields.iter() {
        // Get the data type
        let data_type = field_data.data.get("data_type").ok_or(HeaderError::DataType(name.to_string(), field_name.to_string()))?;

        output += formatdoc!("
            {0:indent$}{data_type} {name};
        ", "", indent = 2, data_type = data_type, name = field_name).as_str();    
    };

    // End class
    output += indoc!("
        };
    ");

    return Ok(output);
}

#[derive(Error, Debug, Clone)]
pub enum HeaderError {
    /// The data object type is not recognized
    #[error("The data object type \"{:?}\" cannot be used in a c++ header file", .0)]
    UnknownType(String),
    /// The data type is not supplied
    #[error("The data type must be supplied for {:?}.{:?}", .0, .1)]
    DataType(String, String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DataModel, DataType};
    use std::collections::HashMap;

    #[test]
    fn basic_struct() {
        // Create the data model
        let mut data_model = DataModel::new();
        data_model.add("CustomType", DataType {
            base_type: "struct".to_string(),
            description: None,
            fields: vec![
                ("field1".to_string(), Instance { data: HashMap::from([
                    ("data_type".to_string(), "int".to_string()),
                ])}),
                ("field2".to_string(), Instance { data: HashMap::from([
                    ("data_type".to_string(), "float".to_string()),
                ])}),
            ]
        }).expect("Dublicate key");
        
        // Create the header file
        let header_file = header("HEADER", &data_model).unwrap();

        let expected = formatdoc!("
            #ifndef HEADER_H_INCLUDED
            #define HEADER_H_INCLUDED

            struct CustomType {{
              int field1;
              float field2;
            }};

            #endif
        ");

        assert_eq!(header_file, expected);
    }
}

