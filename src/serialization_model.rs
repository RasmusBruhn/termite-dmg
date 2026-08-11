use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// A generic serialization model which can be used to serialize any data model
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SerializationModel {
    /// A generic key-value pair map where the key must be a string
    Map(HashMap<String, SerializationModel>),
    /// An array of other serialization models
    Array(Vec<SerializationModel>),
    /// A single value, must be a string
    Value(String),
}

/// Expands all macros in a serialization model
///
/// # Parameters
///
/// value: The serialization model to expand macros in
///
/// macros: The macros to use for expansions
///
/// used_macros: A set of macros that are currently being used, used to prevent infinite recursion
pub fn expand_macros<'a>(
    value: &SerializationModel,
    macros: &'a HashMap<String, SerializationModel>,
    used_macros: &mut HashSet<&'a str>,
) -> Result<SerializationModel, Error> {
    return match value {
        SerializationModel::Map(value) => value
            .iter()
            .map(|(k, v)| match expand_macros(v, macros, used_macros) {
                Ok(value) => Ok((k.clone(), value)),
                Err(error) => Err(error.add_field(k, true)),
            })
            .collect::<Result<HashMap<_, _>, _>>()
            .map(SerializationModel::Map),
        SerializationModel::Array(value) => value
            .iter()
            .enumerate()
            .map(|(i, v)| match expand_macros(v, macros, used_macros) {
                Ok(value) => Ok(value),
                Err(error) => Err(error.add_element(i, true)),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(SerializationModel::Array),
        SerializationModel::Value(value) => {
            // Do a full macro insert if the string is just a macro definition
            if value.starts_with('$')
                && value.ends_with('$')
                && value.len() > 2
                && value.chars().filter(|c| *c == '$').count() == 2
            {
                let macro_name = &value[1..value.len() - 1];

                // Prevent infinite recursion
                if used_macros.contains(macro_name) {
                    return Err(Error {
                        location: "".to_string(),
                        error: ErrorCore::RecursiveMacro(macro_name.to_string()),
                    });
                }

                // Insert the macro
                return if let Some((macro_key, macro_value)) = macros.get_key_value(macro_name) {
                    used_macros.insert(macro_key.as_str());
                    let expanded_macro = expand_macros(macro_value, macros, used_macros);
                    used_macros.remove(macro_key.as_str());
                    match expanded_macro {
                        Ok(value) => Ok(value),
                        Err(error) => Err(error.add_macro(macro_name)),
                    }
                } else {
                    Err(Error {
                        location: "".to_string(),
                        error: ErrorCore::MissingMacro(macro_name.to_string()),
                    })
                };
            }

            // Otherwise do a partial macro insertion
            let mut expanded_string = String::new();
            let mut current_index = 0;
            while current_index < value.len() {
                // Find the beginning of the next macro
                if let Some(start_index) = value[current_index..].find('$') {
                    let start_index = start_index + current_index + 1;
                    expanded_string.push_str(&value[current_index..start_index - 1]);

                    // Skip if it should just be interpreted as a dollar sign
                    if start_index < value.len() && &value[start_index..start_index + 1] == "$" {
                        expanded_string.push('$');
                        current_index = start_index + 1;
                        continue;
                    }

                    // Find the end of the macro
                    if let Some(end_index) = value[start_index..].find('$') {
                        let end_index = end_index + start_index;
                        let macro_name = &value[start_index..end_index];

                        // Prevent infinite recursion
                        if used_macros.contains(macro_name) {
                            return Err(Error {
                                location: "".to_string(),
                                error: ErrorCore::RecursiveMacro(macro_name.to_string()),
                            });
                        }

                        if let Some((macro_key, macro_value)) = macros.get_key_value(macro_name) {
                            // Insert the macro
                            used_macros.insert(macro_key.as_str());
                            let expanded_macro = expand_macros(macro_value, macros, used_macros);
                            used_macros.remove(macro_key.as_str());
                            match expanded_macro {
                                Ok(ok_value) => match ok_value {
                                    SerializationModel::Value(value) => {
                                        expanded_string.push_str(&value);
                                    }
                                    _ => {
                                        return Err(Error {
                                            location: "".to_string(),
                                            error: ErrorCore::PartialMacro(
                                                macro_name.to_string(),
                                                value.clone(),
                                            ),
                                        });
                                    }
                                },
                                Err(error) => {
                                    return Err(error.add_macro(macro_name));
                                }
                            }
                        } else {
                            return Err(Error {
                                location: "".to_string(),
                                error: ErrorCore::MissingMacro(macro_name.to_string()),
                            });
                        }

                        current_index = end_index + 1;
                    } else {
                        return Err(Error {
                            location: "".to_string(),
                            error: ErrorCore::IncompleteMacro(value.clone()),
                        });
                    }
                } else {
                    expanded_string.push_str(&value[current_index..]);
                    break;
                }
            }

            Ok(SerializationModel::Value(expanded_string))
        }
    };
}
