use std::collections::HashMap;
use thiserror::Error;

/// A bridge structure between yaml/json and the data model
#[derive(Clone, Debug, PartialEq)]
pub enum Bridge {
    /// A string value
    Value(String),
    /// A struct with sub field values
    Struct(HashMap<String, Self>),
    /// A list of values
    List(Vec<Self>),
}

impl Bridge {
    /// Loads a bridge from a YAML file
    /// 
    /// # Parameters
    /// 
    /// path: The path of the YAML file
    pub fn from_yaml(s: &str) -> Result<Self, Error> {
        let bridge: serde_yaml::Value = serde_yaml::from_str(s)?;

        return Ok(Self::from(bridge));
    }

    /// Loads a bridge from a JSON file
    /// 
    /// # Parameters
    /// 
    /// path: The path of the YAML file
    pub fn from_json(s: &str) -> Result<Self, Error> {
        // Convert from yaml to an io bridge
        let bridge: serde_json::Value = serde_json::from_str(s)?;

        return Ok(Self::from(bridge));
    }
}

impl From<serde_json::Value> for Bridge {
    fn from(value: serde_json::Value) -> Self {
        return match value {
            serde_json::Value::Null => Self::Value("".to_string()),
            serde_json::Value::Bool(value) => Self::Value(value.to_string()),
            serde_json::Value::Number(value) => Self::Value(value.to_string()),
            serde_json::Value::String(value) => Self::Value(value),
            serde_json::Value::Array(value) => {
                Self::List(value.into_iter().map(|value| Self::from(value)).collect())
            }
            serde_json::Value::Object(value) => {
                Self::Struct(value.into_iter().map(|(key, value)| (key, Self::from(value))).collect())
            }
        }
    }
}

impl From<serde_yaml::Value> for Bridge {
    fn from(value: serde_yaml::Value) -> Self {
        return match value {
            serde_yaml::Value::Null => Self::Value("".to_string()),
            serde_yaml::Value::Bool(value) => Self::Value(value.to_string()),
            serde_yaml::Value::Number(value) => Self::Value(value.to_string()),
            serde_yaml::Value::String(value) => Self::Value(value),
            serde_yaml::Value::Sequence(value) => {
                Self::List(value.into_iter().map(|value| Self::from(value)).collect())
            }
            serde_yaml::Value::Mapping(value) => {
                Self::Struct(value.into_iter().map(|(key, value)| (
                    if let Some(key) = key.as_str() {
                        key.to_string()
                    } else {
                        format!("{:?}", key)
                    }, 
                    Self::from(value))).collect())
            }
            serde_yaml::Value::Tagged(value) => Self::from(value.value),
        }
    }
}

/// Errors when working with data models
#[derive(Error, Debug)]
pub enum Error {
    /// An error occured while converting from yaml
    #[error("Unable to convert from YAML: {:?}", .0)]
    YAML(serde_yaml::Error),
    /// An error occured while converting from json
    #[error("Unable to convert from JSON: {:?}", .0)]
    JSON(serde_json::Error),
}

impl From<serde_yaml::Error> for Error {
    fn from(value: serde_yaml::Error) -> Self {
        return Self::YAML(value);
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        return Self::JSON(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::formatdoc;

    mod yaml {
        use super::*;

        #[test]
        fn value() {
            let bridge = Bridge::from_yaml(formatdoc!("
                Test
            ").as_str()).unwrap();
            let result = Bridge::Value("Test".to_string());
    
            assert_eq!(bridge, result);
        }    

        #[test]
        fn list() {
            let bridge = Bridge::from_yaml(formatdoc!("
                - Test
                - 1
            ").as_str()).unwrap();
            let result = Bridge::List(vec![
                Bridge::Value("Test".to_string()),
                Bridge::Value("1".to_string()),
            ]);
    
            assert_eq!(bridge, result);
        }    

        #[test]
        fn map() {
            let bridge = Bridge::from_yaml(formatdoc!("
                value1: Test
                value2: 1
            ").as_str()).unwrap();
            let result = Bridge::Struct(HashMap::from([
                ("value1".to_string(), Bridge::Value("Test".to_string())),
                ("value2".to_string(), Bridge::Value("1".to_string())),
            ]));
    
            assert_eq!(bridge, result);
        }    

        #[test]
        fn all() {
            let bridge = Bridge::from_yaml(formatdoc!("
                value1: Test
                value2:
                - 1
                - true
                value3:
                    value1: Test
                    value2: 1
            ").as_str()).unwrap();
            let result = Bridge::Struct(HashMap::from([
                ("value1".to_string(), Bridge::Value("Test".to_string())),
                ("value2".to_string(), Bridge::List(vec![
                    Bridge::Value("1".to_string()),
                    Bridge::Value("true".to_string()),
                ])),
                ("value3".to_string(), Bridge::Struct(HashMap::from([
                    ("value1".to_string(), Bridge::Value("Test".to_string())),
                    ("value2".to_string(), Bridge::Value("1".to_string())),
                ]))),
            ]));
    
            assert_eq!(bridge, result);
        }    
    }

    mod json {
        use super::*;

        #[test]
        fn value() {
            let bridge = Bridge::from_json(formatdoc!("
                \"Test\"
            ").as_str()).unwrap();
            let result = Bridge::Value("Test".to_string());
    
            assert_eq!(bridge, result);
        }    

        #[test]
        fn list() {
            let bridge = Bridge::from_json(formatdoc!("
                [
                    \"Test\",
                    \"1\"
                ]
            ").as_str()).unwrap();
            let result = Bridge::List(vec![
                Bridge::Value("Test".to_string()),
                Bridge::Value("1".to_string()),
            ]);
    
            assert_eq!(bridge, result);
        }    

        #[test]
        fn map() {
            let bridge = Bridge::from_json(formatdoc!("
                {{
                    \"value1\": \"Test\",
                    \"value2\": \"1\"
                }}
            ").as_str()).unwrap();
            let result = Bridge::Struct(HashMap::from([
                ("value1".to_string(), Bridge::Value("Test".to_string())),
                ("value2".to_string(), Bridge::Value("1".to_string())),
            ]));
    
            assert_eq!(bridge, result);
        }    

        #[test]
        fn all() {
            let bridge = Bridge::from_json(formatdoc!("
                {{
                    \"value1\": \"Test\",
                    \"value2\": [
                        \"1\",
                        \"true\"
                    ],
                    \"value3\": {{
                        \"value1\": \"Test\",
                        \"value2\": \"1\"
                    }}
                }}
            ").as_str()).unwrap();
            let result = Bridge::Struct(HashMap::from([
                ("value1".to_string(), Bridge::Value("Test".to_string())),
                ("value2".to_string(), Bridge::List(vec![
                    Bridge::Value("1".to_string()),
                    Bridge::Value("true".to_string()),
                ])),
                ("value3".to_string(), Bridge::Struct(HashMap::from([
                    ("value1".to_string(), Bridge::Value("Test".to_string())),
                    ("value2".to_string(), Bridge::Value("1".to_string())),
                ]))),
            ]));
    
            assert_eq!(bridge, result);
        }    
    }
}
