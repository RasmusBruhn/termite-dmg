use super::*;
use indoc::formatdoc;

/// The type specific information for a constrained type
#[derive(Clone, Debug, PartialEq)]
pub(super) struct ConstrainedType {
    /// The type that is constrained
    pub(super) data_type: String,
    /// All extra constraints for the type
    pub(super) constraints: Vec<String>,
}

impl ConstrainedType {
    /// Constructs a new c++ variant from a generic variant
    ///
    /// # Parameters
    ///
    /// data: The generic variant to convert
    pub(super) fn new(data: crate::ConstrainedType) -> Result<Self, Error> {
        return Ok(Self {
            data_type: data.data_type,
            constraints: data.constraints,
        });
    }

    /// Converts the variant to a string for use in the header file
    ///
    /// # Parameters
    ///
    /// name: The name of the variant
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn get_definition_header(&self, name: &str, indent: usize) -> String {
        let data_type =
            if ["string", "number", "integer", "boolean"].contains(&self.data_type.as_str()) {
                format!("termite::{data_type}", data_type = self.data_type)
            } else {
                self.data_type.clone()
            };

        // Create the constraints description
        let constraints = self
            .constraints
            .iter()
            .map(|constraint| {
                return format!("\n{0:indent$} * - {constraint}", "");
            })
            .collect::<Vec<String>>()
            .join("");

        return formatdoc!("
            class {name} {{
            public:
            {0:indent$}/**
            {0:indent$} * @brief Constructs a new {name} object, it must be valid or an exception will be thrown
            {0:indent$} * 
            {0:indent$} * @param value The value to store 
            {0:indent$} */
            {0:indent$}explicit {name}({data_type} value) : {name}(from_value(std::move(value)).get_ok()) {{}}
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
        );
    }

    /// Converts the variant to a string for use in the source file
    ///
    /// # Parameters
    ///
    /// name: The name of the variant
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn get_definition_source(&self, name: &str, indent: usize) -> String {
        let data_type =
            if ["string", "number", "integer", "boolean"].contains(&self.data_type.as_str()) {
                format!("termite::{data_type}", data_type = self.data_type)
            } else {
                self.data_type.clone()
            };

        // Create the tests
        let tests = self.constraints.iter()
            .map(|constraint| formatdoc!("
                {0:indent$}if (!({constraint})) {{
                {0:indent$}{0:indent$}return termite::Result<termite::Empty>::err(termite::Error(\"Did not pass constraint: {constraint}\"));
                {0:indent$}}}\n\n",
                "",
            ))
            .collect::<Vec<String>>()
            .join("");

        // The name of the validation parameter, should not exist if there are no constraints
        let param_name = if self.constraints.is_empty() {
            "".to_string()
        } else {
            "x".to_string()
        };

        return formatdoc!("
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
        );
    }

    /// Gets the header code for the parser for this variant allowing it to be read from a file
    ///
    /// # Parameters
    ///
    /// name: The name of the variant
    ///
    /// namespace: The namespace of the variant
    pub(super) fn get_parser_header(&self, name: &str, namespace: &[String]) -> String {
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

    /// Gets the source code for the parser for this variant allowing it to be read from a file
    ///
    /// # Parameters
    ///
    /// name: The name of the variant
    ///
    /// indent: The number of spaces to use for indentation
    ///
    /// namespace: The namespace of the variant
    ///
    /// data_types: List of all the data types defined in the data model
    pub(super) fn get_parser_source(
        &self,
        name: &str,
        indent: usize,
        namespace: &[String],
        data_types: &[DataType],
    ) -> String {
        // Get the namespace name
        let namespace = namespace
            .iter()
            .map(|single_name| format!("{single_name}::"))
            .collect::<Vec<String>>()
            .join("");
        let typename = format!("{namespace}{name}");

        // Add possible namespace to the typename
        let data_type = if let Some(_) = data_types
            .iter()
            .find(|data_type| data_type.name == self.data_type)
        {
            format!("{namespace}{data_type}", data_type = self.data_type)
        } else {
            format!("{data_type}", data_type = self.data_type)
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpp::test_utils::*;

    #[test]
    fn basic() {
        // Check c++ code
        compile_and_test("type_constrained/basic");

        // Make sure it generates the correct code
        let data_model = DataModel {
            headers: Headers {
                header: "".to_string(),
                source: "".to_string(),
            },
            footers: Footers {
                header: "".to_string(),
                source: "".to_string(),
            },
            data_types: vec![
                DataType {
                    name: "DataType1".to_string(),
                    description: None,
                    data: DataTypeData::ConstrainedType(ConstrainedType {
                        data_type: "int".to_string(),
                        constraints: vec![],
                    }),
                },
                DataType {
                    name: "DataType2".to_string(),
                    description: None,
                    data: DataTypeData::ConstrainedType(ConstrainedType {
                        data_type: "float".to_string(),
                        constraints: vec![],
                    }),
                },
            ],
            namespace: vec!["test".to_string()],
            macros: HashMap::new(),
        };

        // Create the header file
        let header_file = data_model.get_header("HEADER", 2).unwrap();
        let source_file = data_model.get_source("basic", 2).unwrap();
        let expected_header = include_str!("../../tests/cpp/type_constrained/basic/basic.h");
        let expected_source = include_str!("../../tests/cpp/type_constrained/basic/basic.cpp");
        //println!("header:\n{header_file}\n---\n");
        //println!("source:\n{source_file}\n---\n");

        // Check that they are the same
        assert_eq!(str_diff(&header_file, &expected_header), None);
        assert_eq!(str_diff(&source_file, &expected_source), None);
    }

    #[test]
    fn constraints() {
        // Check c++ code
        compile_and_test("type_constrained/constraints");

        // Make sure it generates the correct code
        let data_model = DataModel {
            headers: Headers {
                header: "".to_string(),
                source: "".to_string(),
            },
            footers: Footers {
                header: "".to_string(),
                source: "".to_string(),
            },
            data_types: vec![
                DataType {
                    name: "DataType1".to_string(),
                    description: None,
                    data: DataTypeData::ConstrainedType(ConstrainedType {
                        data_type: "int".to_string(),
                        constraints: vec!["x > 0".to_string(), "x % 2 == 0".to_string()],
                    }),
                },
                DataType {
                    name: "DataType2".to_string(),
                    description: None,
                    data: DataTypeData::ConstrainedType(ConstrainedType {
                        data_type: "float".to_string(),
                        constraints: vec!["std::abs(x) < 1e-9".to_string()],
                    }),
                },
            ],
            namespace: vec!["test".to_string()],
            macros: HashMap::new(),
        };

        // Create the header file
        let header_file = data_model.get_header("HEADER", 2).unwrap();
        let source_file = data_model.get_source("constraints", 2).unwrap();
        let expected_header =
            include_str!("../../tests/cpp/type_constrained/constraints/constraints.h");
        let expected_source =
            include_str!("../../tests/cpp/type_constrained/constraints/constraints.cpp");
        //println!("header:\n{header_file}\n---\n");
        //println!("source:\n{source_file}\n---\n");

        // Check that they are the same
        assert_eq!(str_diff(&header_file, &expected_header), None);
        assert_eq!(str_diff(&source_file, &expected_source), None);
    }
}
