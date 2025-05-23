use super::*;
use indoc::formatdoc;

/// The type specific information for an array
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Array {
    /// The data type for all elements of the array
    pub(super) data_type: String,
}

impl Array {
    /// Constructs a new c++ array from a generic array
    ///
    /// # Parameters
    ///
    /// data: The generic array to convert
    pub(super) fn new(data: crate::Array) -> Result<Self, Error> {
        return Ok(Self {
            data_type: data.data_type,
        });
    }

    /// Converts the array to a string for use in the header file
    ///
    /// # Parameters
    ///
    /// name: The name of the array
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn get_definition_header(&self, name: &str, indent: usize) -> String {
        return formatdoc!("
            struct {name} {{
            public:
            {0:indent$}/**
            {0:indent$} * @brief Constructs a new {name} object
            {0:indent$} * 
            {0:indent$} * @param values The values of the array
            {0:indent$} */
            {0:indent$}explicit {name}(std::vector<{typename}> values) : values(std::move(values)) {{}}

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
            {0:indent$}  return !(*this == x);
            {0:indent$}}}
            {0:indent$}/**
            {0:indent$} * @brief Prints the object onto the output stream
            {0:indent$} * 
            {0:indent$} * @param os The output stream to print to
            {0:indent$} * @param x The object to print
            {0:indent$} * @return The output stream
            {0:indent$} */
            {0:indent$}friend std::ostream &operator<<(std::ostream &os, const {name} &x);

            {0:indent$}/**
            {0:indent$} * @brief The values of the array
            {0:indent$} * 
            {0:indent$} */
            {0:indent$}std::vector<{typename}> values;
            }};",
            "",
            typename = self.data_type,
        );
    }

    /// Converts the array to a string for use in the source file
    ///
    /// # Parameters
    ///
    /// name: The name of the array
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn get_definition_source(&self, name: &str, indent: usize) -> String {
        return formatdoc!("
            bool {name}::operator==(const {name} &x) const {{
            {0:indent$}if (values.size() != x.values.size()) {{
            {0:indent$}{0:indent$}return false;
            {0:indent$}}}

            {0:indent$}for (auto lhs = values.cbegin(), rhs = x.values.cbegin(); lhs < values.cend(); ++lhs, ++rhs) {{
            {0:indent$}{0:indent$}if (*lhs != *rhs) {{
            {0:indent$}{0:indent$}{0:indent$}return false;
            {0:indent$}{0:indent$}}}
            {0:indent$}}}

            {0:indent$}return true;
            }}

            std::ostream &operator<<(std::ostream &os, const {name} &x) {{
            {0:indent$}os << \"{{ values: [ \";
            {0:indent$}for (auto value = x.values.cbegin(); value < x.values.cend(); ++value) {{
            {0:indent$}{0:indent$}if (value != x.values.cbegin()) {{
            {0:indent$}{0:indent$}{0:indent$}os << \", \";
            {0:indent$}{0:indent$}}}
            {0:indent$}{0:indent$}os << *value;
            {0:indent$}}}
            {0:indent$}return os << \" ] }}\";
            }}",
            "",
        );
    }

    /// Gets the header code for the parser for this array allowing it to be read from a file
    ///
    /// # Parameters
    ///
    /// name: The name of the array
    ///
    /// namespace: The namespace of the array
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
            [[nodiscard]] Result<{typename}> Node::List::to_value<{typename}>() const;

            template<>
            [[nodiscard]] Node Node::from_value<{typename}>(const {typename} &value);",
        );
    }

    /// Gets the source code for the parser for this array allowing it to be read from a file
    ///
    /// # Parameters
    ///
    /// name: The name of the array
    ///
    /// indent: The number of spaces to use for indentation
    ///
    /// namespace: The namespace of the array
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
            [[nodiscard]] Result<{typename}> Node::List::to_value<{typename}>() const {{
            {0:indent$}std::vector<{data_type}> values;
            {0:indent$}values.reserve(list_.size());
            {0:indent$}for (auto node = list_.cbegin(); node < list_.cend(); ++node) {{
            {0:indent$}{0:indent$}Result<{data_type}> value = node->to_value<{data_type}>();
            {0:indent$}{0:indent$}if (!value.is_ok()) {{
            {0:indent$}{0:indent$}{0:indent$}Error error = value.get_err();
            {0:indent$}{0:indent$}{0:indent$}error.add_list(node - list_.cbegin());
            {0:indent$}{0:indent$}{0:indent$}return Result<{typename}>::err(std::move(error));
            {0:indent$}{0:indent$}}}
            {0:indent$}{0:indent$}values.push_back(std::move(value.get_ok()));
            {0:indent$}}}

            {0:indent$}return Result<{typename}>::ok({typename}(std::move(values)));
            }}
            
            template<>
            [[nodiscard]] Node Node::from_value<{typename}>(const {typename} &value) {{
            {0:indent$}std::vector<Node> list;
            {0:indent$}list.reserve(value.values.size());
            {0:indent$}std::transform(value.values.cbegin(), value.values.cend(), std::back_inserter(list), [](const {data_type} &value) {{
            {0:indent$}{0:indent$}return Node::from_value(value);
            {0:indent$}}});
            {0:indent$}return Node(Node::List(std::move(list)));
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
        compile_and_test("type_array/basic");

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
                    data: DataTypeData::Array(Array {
                        data_type: "int".to_string(),
                    }),
                },
                DataType {
                    name: "DataType2".to_string(),
                    description: None,
                    data: DataTypeData::Array(Array {
                        data_type: "float".to_string(),
                    }),
                },
            ],
            namespace: vec!["test".to_string()],
        };

        // Create the header file
        let header_file = data_model.get_header("HEADER", 2);
        let source_file = data_model.get_source("basic", 2);
        let expected_header = include_str!("../../tests/cpp/type_array/basic/basic.h");
        let expected_source = include_str!("../../tests/cpp/type_array/basic/basic.cpp");
        //println!("header:\n{header_file}\n---\n");
        //println!("source:\n{source_file}\n---\n");
        
        // Check that they are the same
        assert_eq!(str_diff(&header_file, &expected_header), None);
        assert_eq!(str_diff(&source_file, &expected_source), None);
    }
}
