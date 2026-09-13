use super::{Error, ErrorCore};
use crate::*;
use indoc::formatdoc;
use std::collections::HashMap;

/// Converts the constrained type to a string for use in the header file
///
/// # Parameters
///
/// data: The constrained type to generate code for
///
/// name: The name of the constrained type
///
/// all_types: A map of all data types available for reference
///
/// indent: The number of spaces to use for indentation
pub(super) fn generate_definition_header(
    data: &ConstrainedType,
    name: &str,
    all_types: &HashMap<String, DataType>,
    indent: usize,
) -> Result<String, Error> {
    let data_type = if is_name_builtin(&data.data_type) {
        format!("termite::{data_type}", data_type = data.data_type)
    } else {
        data.data_type.clone()
    };

    let string_constructor = if data.data_type == "string" {
        formatdoc!("
            {0:indent$}/**
            {0:indent$} * @brief Constructs a new {name} object, it must be valid or an exception will be thrown
            {0:indent$} * 
            {0:indent$} * @param value The value to store 
            {0:indent$} */
            {0:indent$}{name}(const char *value) : {name}(std::string(value)) {{}}
        ", "")
    } else {
        format!("")
    };

    // Create the constraints description
    let constraints = data
        .constraints
        .iter()
        .map(|constraint| {
            Ok(format!(
                "\n{0:indent$} * - {constraint}",
                "",
                constraint = constraint::generate(constraint, &data.data_type, all_types)?
            ))
        })
        .collect::<Result<Vec<_>, Error>>()?
        .join("");

    return Ok(formatdoc!("
            class {name} {{
            public:
            {0:indent$}/**
            {0:indent$} * @brief Constructs a new {name} object, it must be valid or an exception will be thrown
            {0:indent$} * 
            {0:indent$} * @param value The value to store 
            {0:indent$} */
            {0:indent$}{name}({data_type} value) : {name}(from_value(std::move(value)).get_ok()) {{}}
            {string_constructor}
            {0:indent$}/**
            {0:indent$} * @brief Constructs a new {name} object
            {0:indent$} * 
            {0:indent$} * @param value The value to store 
            {0:indent$} * @return The new constrained type or an error if some constraints were not upheld
            {0:indent$} */
            {0:indent$}[[nodiscard]] static termite::Result<{name}> from_value({data_type} value);

            {0:indent$}/**
            {0:indent$} * @brief Sets the value if it fulfills the constraints:{constraints}
            {0:indent$} * 
            {0:indent$} * @param value The value to set
            {0:indent$} * @return An error if one of the constraints were not fulfilled
            {0:indent$} */
            {0:indent$}[[nodiscard]] termite::Result<termite::Empty> set({data_type} value);

            {0:indent$}/**
            {0:indent$} * @brief Retrieves a reference to the value
            {0:indent$} * 
            {0:indent$} * @return The reference
            {0:indent$} */
            {0:indent$}[[nodiscard]] const {data_type} &get() const {{
            {0:indent$}{0:indent$}return value_;
            {0:indent$}}}

            {0:indent$}/**
            {0:indent$} * @brief Checks if this object and the other object are identical
            {0:indent$} * 
            {0:indent$} * @param x The other object to compare with
            {0:indent$} * @return true if they are identical, false if not
            {0:indent$} */
            {0:indent$}[[nodiscard]] bool operator==(const {name} &x) const;
            {0:indent$}/**
            {0:indent$} * @brief Checks if this object and the other object are different
            {0:indent$} * 
            {0:indent$} * @param x The other object to compare with
            {0:indent$} * @return true if they are different, false if not
            {0:indent$} */
            {0:indent$}[[nodiscard]] bool operator!=(const {name} &x) const {{
            {0:indent$}{0:indent$}return !(*this == x);
            {0:indent$}}}
            {0:indent$}/**
            {0:indent$} * @brief Prints the object onto the output stream
            {0:indent$} * 
            {0:indent$} * @param os The output stream to print to
            {0:indent$} * @param x The object to print
            {0:indent$} * @return The output stream
            {0:indent$} */
            {0:indent$}friend std::ostream &operator<<(std::ostream &os, const {name} &x);

            private:
            {0:indent$}/**
            {0:indent$} * @brief Constructs a new {name} object
            {0:indent$} * 
            {0:indent$} * @param value The value to store
            {0:indent$} * @param _ A nullptr
            {0:indent$} */
            {0:indent$}explicit {name}({data_type} value, void *) : value_(std::move(value)) {{}}

            {0:indent$}/**
            {0:indent$} * @brief Validates if value is correct using the following constraints:{constraints}
            {0:indent$} * 
            {0:indent$} * @param x The value of the parameter to validate
            {0:indent$} */
            {0:indent$}[[nodiscard]] static termite::Result<termite::Empty> validate(const {data_type} &x);

            {0:indent$}/**
            {0:indent$} * @brief The validated value
            {0:indent$} * 
            {0:indent$} */
            {0:indent$}{data_type} value_;
            }};",
            "",
        ));
}

/// Converts the constrained type to a string for use in the source file
///
/// # Parameters
///
/// data: The constrained type to generate code for
///
/// name: The name of the constrained type
///
/// all_types: A map of all data types available for reference
///
/// indent: The number of spaces to use for indentation
pub(super) fn generate_definition_source(
    data: &ConstrainedType,
    name: &str,
    all_types: &HashMap<String, DataType>,
    indent: usize,
) -> Result<String, Error> {
    let data_type = if is_name_builtin(&data.data_type) {
        format!("termite::{data_type}", data_type = data.data_type)
    } else {
        data.data_type.clone()
    };

    // Create the tests
    let tests = data.constraints.iter()
            .map(|constraint| Ok(formatdoc!("
                {0:indent$}if (!({constraint})) {{
                {0:indent$}{0:indent$}return termite::Result<termite::Empty>::err(termite::Error(\"Did not pass constraint: {constraint}\"));
                {0:indent$}}}\n\n",
                "",
                constraint = constraint::generate(constraint, &data.data_type, all_types)?,
            )))
            .collect::<Result<Vec<_>, Error>>()?
            .join("");

    // The name of the validation parameter, should not exist if there are no constraints
    let param_name = if data.constraints.is_empty() {
        "".to_string()
    } else {
        "x".to_string()
    };

    return Ok(formatdoc!("
            [[nodiscard]] termite::Result<{name}> {name}::from_value({data_type} value) {{
            {0:indent$}termite::Result<termite::Empty> validate_result = validate(value);
            {0:indent$}if (!validate_result.is_ok()) {{
            {0:indent$}{0:indent$}termite::Error error = validate_result.get_err();
            {0:indent$}{0:indent$}return termite::Result<{name}>::err(std::move(error));
            {0:indent$}}}

            {0:indent$}return termite::Result<{name}>::ok({name}(std::move(value), nullptr));
            }}

            [[nodiscard]] termite::Result<termite::Empty> {name}::set({data_type} value) {{
            {0:indent$}termite::Result<termite::Empty> validate_result = validate(value);
            {0:indent$}if (!validate_result.is_ok()) {{
            {0:indent$}{0:indent$}return validate_result;
            {0:indent$}}}

            {0:indent$}value_ = std::move(value);
            {0:indent$}return termite::Result<termite::Empty>::ok(termite::Empty());
            }}

            [[nodiscard]] bool {name}::operator==(const {name} &x) const {{
            {0:indent$}return value_ == x.value_;
            }}
            std::ostream &operator<<(std::ostream &os, const {name} &x) {{
            {0:indent$}return os << x.value_;
            }}

            [[nodiscard]] termite::Result<termite::Empty> {name}::validate(const {data_type} &{param_name}) {{
            {tests}{0:indent$}return termite::Result<termite::Empty>::ok(termite::Empty());
            }}",
            "",
        ));
}

/// Gets the header code for the parser for this constrained type allowing it to be read from a file
///
/// # Parameters
///
/// data: The constrained type to generate code for
///
/// name: The name of the constrained type
///
/// namespace: The namespace of the constrained type
pub(super) fn generate_parser_header(
    _data: &ConstrainedType,
    name: &str,
    namespace: &[String],
) -> String {
    // Get the namespace name
    let namespace = namespace
        .iter()
        .map(|single_name| format!("{single_name}::"))
        .collect::<Vec<String>>()
        .join("");
    let typename = format!("{namespace}{name}");

    return formatdoc!(
        "
            template<>
            [[nodiscard]] Result<{typename}> Node::to_value<{typename}>() const;

            template<>
            [[nodiscard]] Node Node::from_value<{typename}>(const {typename} &value);",
    );
}

/// Gets the source code for the parser for this constrained type allowing it to be read from a file
///
/// # Parameters
///
/// data: The constrained type to generate code for
///
/// name: The name of the constrained type
///
/// indent: The number of spaces to use for indentation
///
/// namespace: The namespace of the constrained type
pub(super) fn generate_parser_source(
    data: &ConstrainedType,
    name: &str,
    indent: usize,
    namespace: &[String],
) -> String {
    // Get the namespace name
    let namespace = namespace
        .iter()
        .map(|single_name| format!("{single_name}::"))
        .collect::<Vec<String>>()
        .join("");
    let typename = format!("{namespace}{name}");

    // Add possible namespace to the typename
    let data_type = if is_name_builtin(&data.data_type) {
        format!("{data_type}", data_type = data.data_type)
    } else {
        format!("{namespace}{data_type}", data_type = data.data_type)
    };

    return formatdoc!(
        "
            template<>
            [[nodiscard]] Result<{typename}> Node::to_value<{typename}>() const {{
            {0:indent$}Result<{data_type}> value = to_value<{data_type}>();
            {0:indent$}if (!value.is_ok()) {{
            {0:indent$}{0:indent$}return Result<{typename}>::err(Error(value.get_err()));
            {0:indent$}}}

            {0:indent$}return {typename}::from_value(value.get_ok());
            }}

            template<>
            [[nodiscard]] Node Node::from_value<{typename}>(const {typename} &value) {{
            {0:indent$}return Node::from_value(value.get());
            }}",
        "",
    );
}

mod constraint {
    use super::*;

    /// Converts the constraint to a C++ expression
    ///
    /// # Parameters
    ///
    /// data: The constraint to convert to a C++ expression
    ///
    /// data_type: The data type of the constrained type
    ///
    /// all_types: A map of all data types available for reference
    pub(super) fn generate(
        data: &Constraint,
        data_type: &str,
        all_types: &HashMap<String, DataType>,
    ) -> Result<String, Error> {
        return match data {
            Constraint::MinLength(value) => generate_min_length(value, data_type, all_types),
            Constraint::MaxLength(value) => generate_max_length(value, data_type, all_types),
            Constraint::Arithmetic(value) => generate_arithmetic(value),
            Constraint::Function(value) => generate_function(value),
        };
    }

    /// Generates the C++ expression for a min length constraint
    ///
    /// # Parameters
    ///
    /// value: The min length constraint value to convert to a C++ expression
    ///
    /// data_type: The data type of the constrained type
    ///
    /// all_types: A map of all data types available for reference
    fn generate_min_length(
        value: &usize,
        data_type: &str,
        all_types: &HashMap<String, DataType>,
    ) -> Result<String, Error> {
        let simplified_type = get_simplified_type(data_type, all_types)?;

        return match simplified_type {
            SimplifiedType::String => Ok(format!("termite::utf8_code_point_count(x) >= {value}")),
            SimplifiedType::Array => Ok(format!("x.values.size() >= {value}")),
            _ => Err(Error::new(ErrorCore::UnsupportedConstraintForType(
                "MinLength".to_string(),
                data_type.to_string(),
            ))),
        };
    }

    /// Generates the C++ expression for a max length constraint
    ///
    /// # Parameters
    ///
    /// value: The max length constraint value to convert to a C++ expression
    ///
    /// data_type: The data type of the constrained type
    ///
    /// all_types: A map of all data types available for reference
    fn generate_max_length(
        value: &usize,
        data_type: &str,
        all_types: &HashMap<String, DataType>,
    ) -> Result<String, Error> {
        let simplified_type = get_simplified_type(data_type, all_types)?;

        return match simplified_type {
            SimplifiedType::String => Ok(format!("termite::utf8_code_point_count(x) <= {value}")),
            SimplifiedType::Array => Ok(format!("x.values.size() <= {value}")),
            _ => Err(Error::new(ErrorCore::UnsupportedConstraintForType(
                "MaxLength".to_string(),
                data_type.to_string(),
            ))),
        };
    }

    /// Generates the C++ expression for an arithmetic constraint
    ///
    /// # Parameters
    ///
    /// value: The arithmetic constraint value to convert to a C++ expression
    fn generate_arithmetic(value: &str) -> Result<String, Error> {
        return Ok(value.to_string());
    }

    /// Generates the C++ expression for a function constraint
    ///
    /// # Parameters
    ///
    /// value: The function constraint value to convert to a C++ expression
    fn generate_function(value: &str) -> Result<String, Error> {
        return Ok(format!("{value}(x)"));
    }
}

#[cfg(test)]
mod tests {
    use crate::cpp::test_utils::*;

    #[test]
    fn basic() {
        run_test("type_constrained/basic", true, false, false);
    }

    #[test]
    fn constraints() {
        run_test("type_constrained/constraints", true, false, false);
    }

    #[test]
    fn min_length_array() {
        run_test("type_constrained/min_length_array", true, false, false);
    }

    #[test]
    fn max_length_array() {
        run_test("type_constrained/max_length_array", true, false, false);
    }

    #[test]
    fn min_length_string() {
        run_test("type_constrained/min_length_string", true, false, false);
    }

    #[test]
    fn max_length_string() {
        run_test("type_constrained/max_length_string", true, false, false);
    }
}
