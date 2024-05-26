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
    pub fn from_yaml(s: &str) -> Result<Self, serde_yaml::Error> {
        let bridge: serde_yaml::Value = serde_yaml::from_str(s)?;

        return Ok(Self::from(bridge));
    }

    /// Converts the bridge to a YAML string
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        let value = serde_yaml::Value::from(self.clone());

        return serde_yaml::to_string(&value);
    }

    /// Loads a bridge from a JSON file
    /// 
    /// # Parameters
    /// 
    /// path: The path of the YAML file
    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        // Convert from yaml to an io bridge
        let bridge: serde_json::Value = serde_json::from_str(s)?;

        return Ok(Self::from(bridge));
    }

    /// Converts the bridge to a JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        let value = serde_json::Value::from(self.clone());

        return serde_json::to_string(&value);
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

impl From<Bridge> for serde_yaml::Value {
    fn from(value: Bridge) -> Self {
        return match value {
            Bridge::Value(value) => serde_yaml::Value::String(value),
            Bridge::List(value) => serde_yaml::Value::Sequence(value.into_iter().map(|value| serde_yaml::Value::from(value)).collect()),
            Bridge::Struct(value) => serde_yaml::Value::Mapping(value.into_iter().map(|(key, value)| (serde_yaml::Value::String(key), serde_yaml::Value::from(value))).collect()),
        }
    }
}

impl From<serde_json::Value> for Bridge {
    fn from(value: serde_json::Value) -> Self {
        return match value {
            serde_json::Value::Null => Self::Value("".to_string()),
            serde_json::Value::Bool(value) => Self::Value(value.to_string()),
            serde_json::Value::Number(value) => Self::Value(value.to_string()),
            serde_json::Value::String(value) => Self::Value(value),
            serde_json::Value::Array(value) => Self::List(value.into_iter().map(|value| Self::from(value)).collect()),
            serde_json::Value::Object(value) => Self::Struct(value.into_iter().map(|(key, value)| (key, Self::from(value))).collect()),
        }
    }
}

impl From<Bridge> for serde_json::Value {
    fn from(value: Bridge) -> Self {
        return match value {
            Bridge::Value(value) => serde_json::Value::String(value),
            Bridge::List(value) => serde_json::Value::Array(value.into_iter().map(|value| serde_json::Value::from(value)).collect()),
            Bridge::Struct(value) => serde_json::Value::Object(value.into_iter().map(|(key, value)| (key, serde_json::Value::from(value))).collect()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::formatdoc;

    mod yaml {
        use super::*;

        #[test]
        fn from_value() {
            let result = Bridge::from_yaml(formatdoc!("
                Test
            ").as_str()).unwrap();
            let expected = Bridge::Value("Test".to_string());
    
            assert_eq!(result, expected);
        }    

        #[test]
        fn from_list() {
            let result = Bridge::from_yaml(formatdoc!("
                - Test
                - 1
            ").as_str()).unwrap();
            let expected = Bridge::List(vec![
                Bridge::Value("Test".to_string()),
                Bridge::Value("1".to_string()),
            ]);
    
            assert_eq!(result, expected);
        }    

        #[test]
        fn from_struct() {
            let result = Bridge::from_yaml(formatdoc!("
                value1: Test
                value2: 1
            ").as_str()).unwrap();
            let expected = Bridge::Struct(HashMap::from([
                ("value1".to_string(), Bridge::Value("Test".to_string())),
                ("value2".to_string(), Bridge::Value("1".to_string())),
            ]));
    
            assert_eq!(result, expected);
        }    

        #[test]
        fn from_all() {
            let result = Bridge::from_yaml(formatdoc!("
                value1: Test
                value2:
                - 1
                - true
                value3:
                    value1: Test
                    value2: 1
            ").as_str()).unwrap();
            let expected = Bridge::Struct(HashMap::from([
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
    
            assert_eq!(result, expected);
        }

        #[test]
        fn to_value() {
            let expected = Bridge::Value("Test".to_string());
            let result = Bridge::from_yaml(&expected.to_yaml().unwrap()).unwrap();

            assert_eq!(result, expected);
        }    

        #[test]
        fn to_list() {
            let expected = Bridge::List(vec![
                Bridge::Value("Test".to_string()),
                Bridge::Value("1".to_string()),
            ]);
            let result = Bridge::from_yaml(&expected.to_yaml().unwrap()).unwrap();
    
            assert_eq!(result, expected);
        }    

        #[test]
        fn to_struct() {
            let expected = Bridge::Struct(HashMap::from([
                ("value1".to_string(), Bridge::Value("Test".to_string())),
                ("value2".to_string(), Bridge::Value("1".to_string())),
            ]));
            let result = Bridge::from_yaml(&expected.to_yaml().unwrap()).unwrap();
    
            assert_eq!(result, expected);
        }    

        #[test]
        fn to_all() {
            let expected = Bridge::Struct(HashMap::from([
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
            let result = Bridge::from_yaml(&expected.to_yaml().unwrap()).unwrap();
    
            assert_eq!(result, expected);
        }
    }

    mod json {
        use super::*;

        #[test]
        fn from_value() {
            let result = Bridge::from_json(formatdoc!("
                \"Test\"
            ").as_str()).unwrap();
            let expected = Bridge::Value("Test".to_string());
    
            assert_eq!(result, expected);
        }    

        #[test]
        fn from_list() {
            let result = Bridge::from_json(formatdoc!("
                [
                    \"Test\",
                    \"1\"
                ]
            ").as_str()).unwrap();
            let expected = Bridge::List(vec![
                Bridge::Value("Test".to_string()),
                Bridge::Value("1".to_string()),
            ]);
    
            assert_eq!(result, expected);
        }    

        #[test]
        fn from_struct() {
            let result = Bridge::from_json(formatdoc!("
                {{
                    \"value1\": \"Test\",
                    \"value2\": \"1\"
                }}
            ").as_str()).unwrap();
            let expected = Bridge::Struct(HashMap::from([
                ("value1".to_string(), Bridge::Value("Test".to_string())),
                ("value2".to_string(), Bridge::Value("1".to_string())),
            ]));
    
            assert_eq!(result, expected);
        }    

        #[test]
        fn from_all() {
            let result = Bridge::from_json(formatdoc!("
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
            let expected = Bridge::Struct(HashMap::from([
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
    
            assert_eq!(result, expected);
        }    

        #[test]
        fn to_value() {
            let expected = Bridge::Value("Test".to_string());
            let result = Bridge::from_json(&expected.to_json().unwrap()).unwrap();

            assert_eq!(result, expected);
        }    

        #[test]
        fn to_list() {
            let expected = Bridge::List(vec![
                Bridge::Value("Test".to_string()),
                Bridge::Value("1".to_string()),
            ]);
            let result = Bridge::from_json(&expected.to_json().unwrap()).unwrap();
    
            assert_eq!(result, expected);
        }    

        #[test]
        fn to_struct() {
            let expected = Bridge::Struct(HashMap::from([
                ("value1".to_string(), Bridge::Value("Test".to_string())),
                ("value2".to_string(), Bridge::Value("1".to_string())),
            ]));
            let result = Bridge::from_json(&expected.to_json().unwrap()).unwrap();
    
            assert_eq!(result, expected);
        }    

        #[test]
        fn to_all() {
            let expected = Bridge::Struct(HashMap::from([
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
            let result = Bridge::from_json(&expected.to_json().unwrap()).unwrap();
    
            assert_eq!(result, expected);
        }
    }
}
