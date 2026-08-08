use crate::*;
use std::collections::HashMap;

/// Sorts a map of data types into an ordered list such that if a data type
/// depends on another data type, then the dependi is located after the type it
/// depends on, in the list
///
/// # Parameters
///
/// data_types: The map of all the data types
pub fn sort_data_types(data_types: &HashMap<String, DataType>) -> Vec<DataType> {
    //let mut result = Vec::new();

    // A map to keep track of which data types has been added to result
    todo!()
}
