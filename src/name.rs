use crate::*;
use regex::regex;

/// Validates if a name is a valid type name. It is valid if it only consists of
/// alphanumeric characters and underscores, and that it does not start with a
/// number
///
/// # Parameters
///
/// name: The name to validate
pub fn validate_name(name: &str) -> Result<(), Error> {
    return if regex!(r"^[a-zA-Z_][a-zA-Z0-9_]*$").is_match(name) {
        Ok(())
    } else {
        Err(Error {
            location: "".to_string(),
            error: ErrorCore::InvalidDataTypeName(name.to_string()),
        })
    };
}

/// Checks if a name is a built-in type name. It is a built-in type name if it is
/// one of the following: "number", "integer", "boolean", "string"
///
/// # Parameters
///
/// name: The name to check
pub fn is_name_builtin(name: &str) -> bool {
    return name == "number" || name == "integer" || name == "boolean" || name == "string";
}
