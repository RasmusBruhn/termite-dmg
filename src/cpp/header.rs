use indoc::formatdoc;
use crate::DefaultType;
use super::*;

impl DataModel {
    /// Generates the header file
    /// 
    /// # Parameters
    /// 
    /// name: The name of the header file (used for header guard so should be capslocked)
    /// 
    /// indent: The number of spaces to use for indentation
    pub fn gen_header(&self, name: &str, indent: usize) -> String {
        // Get header and footer
        let header = self.headers.header.clone();
        let footer = self.footers.header.clone();

        // Get all structs
        let data_types = self.data_types.iter()
            .map(|data_type| data_type.gen_header(indent))
            .collect::<Vec<String>>()
            .join("\n\n");

        return formatdoc!("
            // Generated with the Termite Data Model Generator
            #ifndef {name}_TERMITE_H_INCLUDED
            #define {name}_TERMITE_H_INCLUDED

            // User header
            {header}

            // User data types
            {data_types}

            // User footer
            {footer}

            #endif
        ");
    }
}

impl DataType {
    /// Generates the description if it is supplied
    fn gen_description(&self) -> String {
        return match &self.description {
            Some(description) => formatdoc!("
                /**
                 * \\brief {description}
                 * 
                 */
            "),
            None => "".to_string(),
        };
    }

    /// Converts the data type to a string for use in the header file
    /// 
    /// # Parameters
    /// 
    /// indent: The number of spaces to use for indentation
    fn gen_header(&self, indent: usize) -> String {
        let description = self.gen_description();
        let definition = self.data.gen_header(&self.name, indent);

        return format!("{description}{definition}");
    }
}

impl DataTypeData {
    /// Converts the data type data to a string for use in the header file
    /// 
    /// # Parameters
    /// 
    /// name: The name of the data type
    /// 
    /// indent: The number of spaces to use for indentation
    fn gen_header(&self, name: &str, indent: usize) -> String {
        return match self {
            DataTypeData::Struct(data) => data.gen_header(name, indent),
        };
    }
}

impl Struct {
    /// Converts the struct to a string for use in the header file
    /// 
    /// # Parameters
    /// 
    /// name: The name of the struct
    /// 
    /// indent: The number of spaces to use for indentation
    fn gen_header(&self, name: &str, indent: usize) -> String {
        // Get the definitions of all the fields but without any initialization
        let field_definitions = todo!();


    }
}

impl StructField {
    /// Constructs the c++ typename of this field
    fn get_typename(&self) -> String {
        return match &self.default {
            DefaultType::Optional => format!(
                "std::optional<{data_type}>",
                data_type = self.data_type,
            ),
            _ => self.data_type.clone(),
        };
    }

    /// Constructs the default value from the supplied default value or optional parameter
    fn get_default_value(&self) -> String {
        return match &self.default {
            DefaultType::Required => "".to_string(),
            DefaultType::Optional => " = std::nullopt".to_string(),
            DefaultType::Default(value) => format!(" = {value}"),
        };
    }

    /// Constructs the full definition of the field
    fn get_definition(&self) -> String {
        return format!(
            "{typename} {name}{default_value}",
            typename = self.get_typename(),
            name = self.name,
            default_value = self.get_default_value(),      
        )
    }

    /// Generates the description if it is supplied
    /// 
    /// # Parameters
    /// 
    /// indent: The number of spaces to use for indentation
    fn gen_description(&self, indent: usize) -> String {
        return match &self.description {
            Some(description) => formatdoc!("
                {0:indent$}/**
                {0:indent$} * \\brief {description}
                {0:indent$} * 
                {0:indent$} */
            ", ""),
            None => "".to_string(),
        };
    }

    /// Generates the definition of the field
    /// 
    /// # Parameters
    /// 
    /// indent: The number of spaces to use for indentation
    fn gen_definition(&self, indent: usize) -> String {
        return format!(
            "{desc}{0:indent$}{definition};\n",
            "",
            desc = self.gen_description(indent),
            definition = self.get_definition(),
        );
    }
}

/*
use crate::{DataModel, Instance};
use indoc::{formatdoc, indoc};
use thiserror::Error;

/// A data structure with all the data for a struct field
#[derive(Clone, Copy, Debug, PartialEq)]
struct StructFieldData<'a> {
    /// The name of this field
    name: &'a String,
    /// The name of the data type of this field
    data_type: &'a String,
    /// The description of the field, None if there is no description
    description: Option<&'a String>,
    /// The default value if none is specified, None if it is required
    default_value: Option<&'a String>,
    /// True if it is an optional, the default value will then be None
    optional: bool,
}

impl<'a> StructFieldData<'a> {
    /// Constructs a new struct field data from a field name and the instance data
    /// 
    /// # Parameters
    /// 
    /// name: The name of the field
    /// 
    /// data: The instance data for the field
    fn new(data: &'a Instance) -> Result<Self, StructFieldError> {
        let data_type = data.data.get("data_type").ok_or(StructFieldError::DataType)?;
        let optional =  if let Some(optional) = data.data.get("optional") {
            if let Ok(optional) = optional.parse() {
                optional
            } else {
                return Err(StructFieldError::ParseBoolean("optional".to_string()));
            }
        } else {
            false
        };

        return Ok(Self {
            name: &data.name,
            data_type,
            description: data.data.get("description"),
            default_value: data.data.get("default"),
            optional,
        })
    }

    /// Returns true if the field is required
    fn is_required(&self) -> bool {
        return !self.optional && self.default_value == None;
    }

    /// Constructs the c++ typename of this field
    fn get_typename(&self) -> String {
        return if self.optional {
            format!(
                "std::optional<{data_type}>",
                data_type = self.data_type,
            )
        } else {
            self.data_type.to_string()
        };
    }

    /// Constructs the default value from the supplied default value or optional parameter
    fn get_default_value(&self) -> String {
        return if let Some(default_value) = self.default_value {
            format!(
                " = {default_value}",
                default_value = default_value,
            )
        } else if self.optional {
            " = std::nullopt".to_string()
        } else {
            "".to_string()
        }
    }

    /// Constructs the full definition of the field
    fn get_definition(&self) -> String {
        return format!(
            "{typename} {name}{default_value}",
            typename = self.get_typename(),
            name = self.name,
            default_value = self.get_default_value(),      
        )
    }

    /// Generates the description if it is supplied
    fn gen_description(&self, indent: usize) -> String {
        return if let Some(description) = self.description {
            formatdoc!("
                    {0:indent$}/**
                    {0:indent$} * \\brief {desc}
                    {0:indent$} * 
                    {0:indent$} */
                ", 
                "", 
                indent = indent, 
                desc = description,
            )
        } else {
            return "".to_string()
        }
    }

    /// Generates the definition of the field
    fn gen_definition(&self, indent: usize) -> String {
        return format!(
            "{desc}{0:indent$}{definition};\n",
            "",
            indent = indent,
            desc = self.gen_description(indent),
            definition = self.get_definition(),
        );
    }
}

/// Any error which may occur during handling of struct fields
#[derive(Error, Debug, Clone)]
pub enum StructFieldError {
    /// The data type is not supplied
    #[error("The data type must be supplied")]
    DataType,
    /// The boolean could not be parsed
    #[error("Unable to parse boolean for {:?}", .0)]
    ParseBoolean(String),
}

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
pub fn header(name: &str, data_model: &DataModel, indent: usize) -> Result<String, HeaderError> {
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
    for type_data in data_model.iter() {
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
            "struct" => header_struct(&type_data.name, &type_data.data, indent, output)?,
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
fn header_struct(name: &str, fields: &Vec<Instance>, indent: usize, mut output: String) -> Result<String, HeaderError> {
    // Create class name
    output += formatdoc!("
        struct {name} {{
        ", 
        name = name
    ).as_str();

    // Setup strings
    let mut field_definitions = String::new();

    // Add required fields
    let fields = fields.iter()
        .map(|field_data| {
            let result = StructFieldData::new(field_data);

            match result {
                Ok(data) => Ok(data),
                Err(err) => Err(HeaderError::StructField(format!("{base_name}.{field_name}", base_name = name, field_name = field_data.name), err)),
            }
        })
        .collect::<Result<Vec<StructFieldData>, HeaderError>>()?;

    // Reorganize them such that required are before optional
    for field in fields.iter().filter(|field| field.is_required()).chain(
        fields.iter().filter(|field| !field.is_required())
    ) {
        field_definitions += field.gen_definition(indent).as_str();
    };

    // Add field definitions
    output += field_definitions.as_str();

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
    #[error("An error occured while parsing the field {:?}: {:?}", .0, .1)]
    StructField(String, StructFieldError),
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
            vec![DataType {
                name: "CustomType".to_string(),
                base_type: "struct".to_string(),
                description: None,
                data: vec![
                    Instance {
                        name: "field1".to_string(),
                        data: HashMap::from([
                            ("data_type".to_string(), "int".to_string()),
                        ]),
                    },
                    Instance {
                        name: "field2".to_string(),
                        data: HashMap::from([
                            ("data_type".to_string(), "float".to_string()),
                        ]),
                    },
                ]
        }]).expect("Dublicate key");
        
        // Create the header file
        let header_file = header("HEADER", &data_model, 2).unwrap();

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
            vec![DataType {
                name: "CustomType".to_string(),
                base_type: "struct".to_string(),
                description: None,
                data: vec![
                    Instance {
                        name: "field1".to_string(),
                        data: HashMap::from([
                            ("data_type".to_string(), "int".to_string()),
                        ]),
                    },
                    Instance {
                        name: "field2".to_string(),
                        data: HashMap::from([
                            ("data_type".to_string(), "float".to_string()),
                        ]),
                    },
                ]
        }]).expect("Dublicate key");
        data_model.add_header("header", "#include \"sample.h\"");
        
        // Create the header file
        let header_file = header("HEADER", &data_model, 2).unwrap();

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
            vec![DataType {
                name: "CustomType".to_string(),
                base_type: "struct".to_string(),
                description: Some("CustomDescription".to_string()),
                data: vec![
                    Instance {
                        name: "field1".to_string(),
                        data: HashMap::from([
                            ("data_type".to_string(), "int".to_string()),
                            ("description".to_string(), "FieldDescription1".to_string()),
                        ]),
                    },
                    Instance {
                        name: "field2".to_string(),
                        data: HashMap::from([
                            ("data_type".to_string(), "float".to_string()),
                            ("description".to_string(), "FieldDescription2".to_string()),
                        ]),
                    },
                ]
        }]).expect("Dublicate key");

        // Create the header file
        let header_file = header("HEADER", &data_model, 2).unwrap();

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
            vec![DataType {
                name: "CustomType".to_string(),
                base_type: "struct".to_string(),
                description: None,
                data: vec![
                    Instance {
                        name: "field1".to_string(),
                        data: HashMap::from([
                            ("data_type".to_string(), "int".to_string()),
                            ("default".to_string(), "42".to_string()),
                        ]),
                    },
                    Instance {
                        name: "field2".to_string(),
                        data: HashMap::from([
                            ("data_type".to_string(), "float".to_string()),
                        ]),
                    },
                ]
        }]).expect("Dublicate key");
        
        // Create the header file
        let header_file = header("HEADER", &data_model, 2).unwrap();

        let expected = formatdoc!("
            #ifndef HEADER_H_INCLUDED
            #define HEADER_H_INCLUDED

            #include <optional>
            #include <variant>

            struct CustomType {{
              float field2;
              int field1 = 42;
            }};

            #endif
        ");

        assert_eq!(header_file, expected);
    }

    #[test]
    fn struct_optional() {
        // Create the data model
        let data_model = DataModel::from_vec(
            vec![DataType {
                name: "CustomType".to_string(),
                base_type: "struct".to_string(),
                description: None,
                data: vec![
                    Instance {
                        name: "field1".to_string(),
                        data: HashMap::from([
                            ("data_type".to_string(), "int".to_string()),
                            ("optional".to_string(), "true".to_string()),
                        ]),
                    },
                    Instance {
                        name: "field2".to_string(),
                        data: HashMap::from([
                            ("data_type".to_string(), "float".to_string()),
                            ("optional".to_string(), "true".to_string()),
                            ("default".to_string(), "1.5".to_string()),
                        ]),
                    },
                    Instance {
                        name: "field3".to_string(),
                        data: HashMap::from([
                            ("data_type".to_string(), "int".to_string()),
                        ]),
                    },
                ]
        }]).expect("Dublicate key");
        
        // Create the header file
        let header_file = header("HEADER", &data_model, 2).unwrap();

        let expected = formatdoc!("
            #ifndef HEADER_H_INCLUDED
            #define HEADER_H_INCLUDED
            
            #include <optional>
            #include <variant>

            struct CustomType {{
              int field3;
              std::optional<int> field1 = std::nullopt;
              std::optional<float> field2 = 1.5;
            }};

            #endif
        ");

        assert_eq!(header_file, expected);
    }
}

 */