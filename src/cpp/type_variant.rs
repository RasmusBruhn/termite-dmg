use super::*;
use indoc::formatdoc;

/// The type specific information for a variant
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Variant {
    /// The possible types for the variant
    pub(super) data_types: Vec<String>,
}

impl Variant {
    /// Constructs a new c++ variant from a generic variant
    ///
    /// # Parameters
    ///
    /// data: The generic variant to convert
    pub(super) fn new(data: crate::Variant) -> Result<Self, Error> {
        return Ok(Self {
            data_types: data.data_types,
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
        // Create list of the variants
        let variant_list = self.data_types.join(", ");

        return formatdoc!(
            "
            struct {name} {{
            {0:indent$}/**
            {0:indent$} * @brief Constructs a new {name} object
            {0:indent$} * 
            {0:indent$} * @param value The value of the variant
            {0:indent$} */
            {0:indent$}explicit {name}(std::variant<{variant_list}> value) : value(std::move(value)) {{}}

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

            {0:indent$}/**
            {0:indent$} * @brief The value of the variant
            {0:indent$} * 
            {0:indent$} */
            {0:indent$}std::variant<{variant_list}> value;
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
        // Create writer
        let writer_specifiers = self
            .data_types
            .iter()
            .enumerate()
            .map(|(index, data_type)| {
                formatdoc!(
                    "
                    {0:indent$}case {index}:
                    {0:indent$}{0:indent$}os << \"{data_type} \" << std::get<{data_type}>(x.value);
                    {0:indent$}{0:indent$}break;",
                    "",
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        return formatdoc!(
            "
            [[nodiscard]] bool {name}::operator==(const {name} &x) const {{
            {0:indent$}return value == x.value;
            }}

            std::ostream &operator<<(std::ostream &os, const {name} &x) {{
            {0:indent$}os << \"{{ value: \";
            {0:indent$}switch (x.value.index()) {{
            {writer_specifiers}
            {0:indent$}default:
            {0:indent$}{0:indent$}os << \"Unknown(\" << x.value.index() << \")\";
            {0:indent$}{0:indent$}break;
            {0:indent$}}}
            {0:indent$}return os << \" }}\";
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
    /// indent: The number of spaces to use for indentation
    ///
    /// namespace: The namespace of the variant
    ///
    /// data_types: List of all the data types defined in the data model
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

        // Get snake case naming
        let snake_case_data_types = self
            .data_types
            .iter()
            .map(|data_type| ToSnakeCase::new(&mut data_type.chars()).collect::<String>())
            .collect::<Vec<String>>();

        // Get all the readers
        let readers = self.data_types.iter()
            .zip(snake_case_data_types.iter())
            .map(|(data_type, snake_case)| {
                // Add possible namespace to the typename
                let data_type = if let Some(_) = data_types.iter().find(|new_data_type| &new_data_type.name == data_type) {
                    format!("{namespace}{data_type}")
                } else {
                    format!("{data_type}")
                };

                return formatdoc!("
                    {0:indent$}Result<{data_type}> result_{snake_case} = to_value<{data_type}>();
                    {0:indent$}if (result_{snake_case}.is_ok()) {{
                    {0:indent$}{0:indent$}return Result<{typename}>::ok({typename}(result_{snake_case}.get_ok()));
                    {0:indent$}}}
                    {0:indent$}error << \"{data_type} {{ \" << result_{snake_case}.get_err() << \" }}\";",
                    "",
                );
            })
            .collect::<Vec<String>>()
            .join(&formatdoc!("
                
                {0:indent$}error << \", \";

                ",
                "",
            ));

        return formatdoc!(
            "
            template<>
            [[nodiscard]] Result<{typename}> Node::to_value<{typename}>() const {{
            {0:indent$}std::stringstream error;
            {0:indent$}error << \"Unable to parse any variant: [ \";

            {readers}
            
            {0:indent$}error << \" ]\";

            {0:indent$}return Result<{typename}>::err(Error(error.str()));
            }}

            template<>
            [[nodiscard]] Node Node::from_value<{typename}>(const {typename} &value) {{
            {0:indent$}return std::visit([](const auto &x) {{
            {0:indent$}{0:indent$}return Node::from_value(x);
            {0:indent$}}}, value.value);
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
        compile_and_test("type_variant/basic");

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
            data_types: vec![DataType {
                name: "DataType".to_string(),
                description: None,
                data: DataTypeData::Variant(Variant {
                    data_types: vec!["int".to_string(), "float".to_string()],
                }),
            }],
            namespace: vec!["test".to_string()],
            macros: HashMap::new(),
        };

        // Create the header file
        let header_file = data_model.get_header("HEADER", 2);
        let source_file = data_model.get_source("basic", 2).unwrap();
        let expected_header = include_str!("../../tests/cpp/type_variant/basic/basic.h");
        let expected_source = include_str!("../../tests/cpp/type_variant/basic/basic.cpp");
        //println!("header:\n{header_file}\n---\n");
        //println!("source:\n{source_file}\n---\n");

        // Check that they are the same
        assert_eq!(str_diff(&header_file, &expected_header), None);
        assert_eq!(str_diff(&source_file, &expected_source), None);
    }
}
