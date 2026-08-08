use std::fmt;

/// Errors for when converting generic data models into code
#[derive(Debug, Clone)]
pub struct Error {
    /// The location where the error occured
    pub location: String,
    /// The actual error that occured
    pub error: ErrorCore,
}

impl Error {
    /// Sets the current location to be the field of the given base
    ///
    /// # Parameters
    ///
    /// base: The base to set in the location
    pub fn add_field(self, base: &str) -> Error {
        let location = format!(".{}{}", base, self.location);

        return Error {
            location,
            error: self.error,
        };
    }

    /// Sets the current location to be the element of a field of the given base
    ///
    /// # Parameters
    ///
    /// index: The index of the field
    pub fn add_element(self, index: usize) -> Error {
        let location = format!("[{}]{}", index, self.location);

        return Error {
            location,
            error: self.error,
        };
    }

    /// Sets the current location to be the element of a field of the given base
    ///
    /// # Parameters
    ///
    /// index: The index of the field
    pub fn add_macro(self, index: &str) -> Error {
        let location = format!("[{}]{}", index, self.location);

        return Error {
            location,
            error: self.error,
        };
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}: {}", self.location, self.error);
    }
}

/// Errors for when converting generic data models into JSON schema data models
#[derive(thiserror::Error, Debug, Clone)]
pub enum ErrorCore {
    /// Macros used recursively
    #[error("The macro \"{}\" is used recursively", .0)]
    RecursiveMacro(String),
    /// Macro is missing
    #[error("The macro \"{}\" is not defined", .0)]
    MissingMacro(String),
    /// Macro is incomplete
    #[error("The string \"{}\" begins a macro without ending it", .0)]
    IncompleteMacro(String),
    /// A partial macro insertion can only have a string value
    #[error("The partial macro insertion of \"{}\" in \"{}\" must be a string", .0, .1)]
    PartialMacro(String, String),
}
