use crate::*;
use std::collections::HashMap;

/// Sorts a map of data types into an ordered list such that if a data type
/// depends on another data type, then the depending type is located after the
/// type it depends on, in the list. Also makes sure that the name is valid and
/// that there are no recursive data types, and that all dependencies exist.
///
/// # Parameters
///
/// data_types: The map of all the data types
pub fn sort_data_types(
    data_types: &HashMap<String, DataType>,
) -> Result<Vec<(String, DataType)>, Error> {
    let mut result = Vec::new();

    // A map to keep track of which data types has been added to result
    let mut sort_states = data_types
        .iter()
        .map(|(name, data_type)| {
            validate_name(name)?;
            return if is_name_builtin(name) {
                Err(Error {
                    location: "".to_string(),
                    error: ErrorCore::InvalidBuiltInTypeName(name.clone()),
                })
            } else {
                Ok((name.clone(), (data_type.clone(), SortState::NotAdded)))
            };
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    // The stack of data types to process, the first elemement is the name, the
    // second element is the stack trace
    let mut stack = data_types
        .iter()
        .map(|(name, _)| {
            (
                name.clone(),
                Error {
                    location: name.clone(),
                    error: ErrorCore::None,
                },
            )
        })
        .collect::<Vec<_>>();

    while !stack.is_empty() {
        let (name, stack_trace) = stack.pop().unwrap();

        let dependencies = {
            let (data_type, state) = sort_states.get_mut(&name).unwrap();

            match state {
                SortState::Added => None,
                SortState::InProgress => {
                    *state = SortState::Added;
                    result.push((name.clone(), data_type.clone()));
                    None
                }
                SortState::NotAdded => {
                    *state = SortState::InProgress;
                    stack.push((name.clone(), stack_trace.clone()));

                    Some(match &data_type.data {
                        DataTypeData::ConstrainedType(value) => vec![value.data_type.clone()],
                        DataTypeData::Array(value) => vec![value.data_type.clone()],
                        DataTypeData::Enum(value) => value
                            .types
                            .iter()
                            .filter_map(|value| value.data_type.clone())
                            .collect(),
                        DataTypeData::Variant(value) => value.data_types.clone(),
                        DataTypeData::Struct(value) => value
                            .fields
                            .iter()
                            .map(|value| value.data_type.clone())
                            .collect(),
                    })
                }
            }
        };

        let dependencies = match dependencies {
            None => continue,
            Some(dependencies) => dependencies,
        };

        for dependency in dependencies {
            let new_stack_trace = stack_trace.clone().add_field(&dependency, false);

            // Skip if it is a built-in type
            if dependency == "number"
                || dependency == "integer"
                || dependency == "boolean"
                || dependency == "string"
            {
                continue;
            }

            // Make sure the dependency exists
            let (_, state) = sort_states.get(&dependency).ok_or(Error {
                location: new_stack_trace.location.clone(),
                error: ErrorCore::UnknownDataType(name.clone()),
            })?;

            // Make sure it is not in progress
            match state {
                SortState::Added => continue,
                SortState::InProgress => {
                    return Err(Error {
                        location: new_stack_trace.location.clone(),
                        error: ErrorCore::RecursiveDataType(name.clone()),
                    })
                }
                SortState::NotAdded => {
                    stack.push((dependency.clone(), new_stack_trace));
                }
            }
        }
    }

    return Ok(result);
}

/// An enum to represent the state of a data type in the sorting process
enum SortState {
    /// The data type has been added to the list of sorted data types
    Added,
    /// The data type is currently being processed
    InProgress,
    /// The data type has not yet being touched in the sorting process
    NotAdded,
}
