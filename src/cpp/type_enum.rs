use super::*;
use indoc::formatdoc;

/// The type specific information for a variant
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Enum {
    /// The possible types for the variant
    pub(super) types: Vec<EnumType>,
}

impl Enum {
    /// Constructs a new c++ enum from a generic enum
    ///
    /// # Parameters
    ///
    /// data: The generic enum to convert
    pub(super) fn new(data: crate::Enum) -> Result<Self, Error> {
        // Convert the types
        let types = data
            .types
            .into_iter()
            .map(|data| EnumType::new(data))
            .collect::<Result<Vec<EnumType>, Error>>()?;

        return Ok(Self { types });
    }

    /// Converts the enum to a string for use in the header file
    ///
    /// # Parameters
    ///
    /// name: The name of the enum
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn get_definition_header(&self, name: &str, indent: usize) -> String {
        // Get enum type definitions
        let type_definition = self
            .types
            .iter()
            .map(|enum_type| enum_type.get_definition(indent))
            .collect::<Vec<String>>()
            .join("\n");

        // Get the type wrappers
        let type_wrappers = self
            .types
            .iter()
            .map(|enum_type| enum_type.get_wrapper_header(indent))
            .collect::<Vec<String>>()
            .join("\n\n");

        // Get the wrapper name list
        let wrapper_list = self
            .types
            .iter()
            .map(|enum_type| enum_type.get_wrapper_name())
            .collect::<Vec<String>>()
            .join(", ");

        return formatdoc!(
            "
            struct {name} {{
            {0:indent$}/**
            {0:indent$} * @brief The values of this enum
            {0:indent$} * 
            {0:indent$} */
            {0:indent$}enum class Enum {{
            {type_definition}
            {0:indent$}}};

            {type_wrappers}

            {0:indent$}/**
            {0:indent$} * @brief Constructs a new {name} object
            {0:indent$} * 
            {0:indent$} * @param value The value of the enum
            {0:indent$} */
            {0:indent$}explicit {name}(std::variant<{wrapper_list}> value) : value(std::move(value)) {{}}

            {0:indent$}/**
            {0:indent$} * @brief Returns the enum type that is stored
            {0:indent$} * 
            {0:indent$} * @return The enum type
            {0:indent$} */
            {0:indent$}[[nodiscard]] Enum enum_type() const {{
            {0:indent$}{0:indent$}return static_cast<Enum>(value.index());
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
        
            {0:indent$}/**
            {0:indent$} * @brief The value of the enum
            {0:indent$} * 
            {0:indent$} */
            {0:indent$}std::variant<{wrapper_list}> value;
            }};",
            "",
        );
    }

    /// Converts the enum to a string for use in the source file
    ///
    /// # Parameters
    ///
    /// name: The name of the enum
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn get_definition_source(&self, name: &str, indent: usize) -> String {
        // Get the type wrappers
        let type_wrappers = self
            .types
            .iter()
            .map(|enum_type| enum_type.get_wrapper_source(name, indent))
            .collect::<Vec<String>>()
            .join("\n\n");

        // Get the printers
        let printers = self
            .types
            .iter()
            .map(|enum_type| enum_type.get_printer(name, indent))
            .collect::<Vec<String>>()
            .join("\n");

        return formatdoc!(
            "
            {type_wrappers}

            [[nodiscard]] bool {name}::operator==(const {name} &x) const {{
            {0:indent$}return value == x.value;
            }}

            std::ostream &operator<<(std::ostream &os, const {name} &x) {{
            {0:indent$}os << \"{{ value: \";
            {0:indent$}switch (static_cast<{name}::Enum>(x.value.index())) {{
            {printers}
            {0:indent$}default:
            {0:indent$}{0:indent$}os << \"Unknown (\" << x.value.index() << \")\";
            {0:indent$}{0:indent$}break;
            {0:indent$}}}
            {0:indent$}return os << \" }}\";
            }}",
            "",
        );
    }

    /// Gets the header code for the parser for this enum allowing it to be read from a file
    ///
    /// # Parameters
    ///
    /// name: The name of the enum
    ///
    /// namespace: The namespace of the enum
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
            [[nodiscard]] Result<{typename}> Node::Value::to_value<{typename}>() const;
            
            template<>
            [[nodiscard]] Result<{typename}> Node::Map::to_value<{typename}>() const;

            template<>
            [[nodiscard]] Node Node::from_value<{typename}>(const {typename} &value);",
        );
    }

    /// Gets the source code for the parser for this enum allowing it to be read from a file
    ///
    /// # Parameters
    ///
    /// name: The name of the enum
    ///
    /// indent: The number of spaces to use for indentation
    ///
    /// namespace: The namespace of the enum
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

        // Get the value parser
        let value_parsers = self
            .types
            .iter()
            .map(|enum_type| enum_type.get_parser_value(&typename, indent))
            .collect::<Vec<String>>()
            .join("\n");

        // Get the map parser
        let map_parsers = self
            .types
            .iter()
            .map(|enum_type| enum_type.get_parser_map(&typename, &namespace, data_types, indent))
            .collect::<Vec<String>>()
            .join("\n");

        // Get the export parser
        let export_parsers = self
            .types
            .iter()
            .map(|enum_type| enum_type.get_parser_export(&typename, indent))
            .collect::<Vec<String>>()
            .join("");

        return formatdoc!("
            template<>
            [[nodiscard]] Result<{typename}> Node::Value::to_value<{typename}>() const {{
            {value_parsers}

            {0:indent$}std::stringstream ss;
            {0:indent$}ss << \"Unknown enum type \\\"\" << value_ << \"\\\"\";
            {0:indent$}return Result<{typename}>::err(Error(ss.str()));
            }}
            
            template<>
            [[nodiscard]] Result<{typename}> Node::Map::to_value<{typename}>() const {{
            {0:indent$}if (map_.size() != 1) {{
            {0:indent$}{0:indent$}std::stringstream ss;
            {0:indent$}{0:indent$}ss << \"There must be exactly one enum type specified but received \" << map_.size();
            {0:indent$}{0:indent$}return Result<{typename}>::err(Error(ss.str()));
            {0:indent$}}}

            {map_parsers}

            {0:indent$}std::stringstream ss;
            {0:indent$}ss << \"Unknown enum type \\\"\" << map_.cbegin()->first << \"\\\"\";
            {0:indent$}return Result<{typename}>::err(Error(ss.str()));
            }}

            template<>
            [[nodiscard]] Node Node::from_value<{typename}>(const {typename} &value) {{
            {0:indent$}std::map<std::string, Node> map;
            {0:indent$}switch (value.enum_type()) {{
            {export_parsers}{0:indent$}default:
            {0:indent$}{0:indent$}return Node(Node::Value(\"\"));
            {0:indent$}}}
            }}",
            "",
        );
    }
}

/// The data for an enum type
#[derive(Clone, Debug, PartialEq)]
pub(super) struct EnumType {
    /// The name of this enum type
    pub(super) name: String,
    /// The description describing this enum type
    pub(super) description: Option<String>,
    /// The type for this enum type, may be omitted for an empty type
    pub(super) data_type: Option<String>,
}

impl EnumType {
    /// Constructs a new c++ enum type from a generic enum type
    ///
    /// # Parameters
    ///
    /// data: The generic enum type to convert
    pub(super) fn new(data: crate::EnumType) -> Result<Self, Error> {
        return Ok(Self {
            name: data.name,
            description: data.description,
            data_type: data.data_type,
        });
    }

    /// Gets the description of this enum type
    fn get_description(&self) -> String {
        return match &self.description {
            Some(description) => description.clone(),
            None => "".to_string(),
        };
    }

    /// Gets the definition of this enum type
    ///
    /// # Parameters
    ///
    /// indent: The indentation to use
    fn get_definition(&self, indent: usize) -> String {
        return formatdoc!(
            "
            {0:indent$}{0:indent$}/**
            {0:indent$}{0:indent$} * @brief {description}
            {0:indent$}{0:indent$} * 
            {0:indent$}{0:indent$} */
            {0:indent$}{0:indent$}k{name},",
            "",
            description = self.get_description(),
            name = self.name,
        );
    }

    /// Gets the header code for the wrapper struct of this enum type
    ///
    /// # Parameters
    ///
    /// indent: The indentation to use
    fn get_wrapper_header(&self, indent: usize) -> String {
        // Get the definition of the type
        let type_definition = match &self.data_type {
            Some(data_type) => formatdoc!(
                "
                {0:indent$}{0:indent$}/**
                {0:indent$}{0:indent$} * @brief The value
                {0:indent$}{0:indent$} * 
                {0:indent$}{0:indent$} */
                {0:indent$}{0:indent$}{data_type} value;

                {0:indent$}{0:indent$}/**
                {0:indent$}{0:indent$} * @brief Constructs a new {name} object
                {0:indent$}{0:indent$} * 
                {0:indent$}{0:indent$} * @param value The value of the enum
                {0:indent$}{0:indent$} */
                {0:indent$}{0:indent$}explicit Type{name}({data_type} value) : value(std::move(value)) {{}}\n\n",
                "",
                name = self.name,
            ),
            None => "".to_string(),
        };

        return formatdoc!("
            {0:indent$}/**
            {0:indent$} * @brief The data for when the enum is a {name}
            {0:indent$} * 
            {0:indent$} */
            {0:indent$}struct Type{name} {{
            {type_definition}{0:indent$}{0:indent$}/**
            {0:indent$}{0:indent$} * @brief Checks if this object and the other object are identical
            {0:indent$}{0:indent$} * 
            {0:indent$}{0:indent$} * @param x The other object to compare with
            {0:indent$}{0:indent$} * @return true if they are identical, false if not
            {0:indent$}{0:indent$} */
            {0:indent$}{0:indent$}[[nodiscard]] bool operator==(const Type{name} &x) const;
            {0:indent$}{0:indent$}/**
            {0:indent$}{0:indent$} * @brief Checks if this object and the other object are different
            {0:indent$}{0:indent$} * 
            {0:indent$}{0:indent$} * @param x The other object to compare with
            {0:indent$}{0:indent$} * @return true if they are different, false if not
            {0:indent$}{0:indent$} */
            {0:indent$}{0:indent$}[[nodiscard]] bool operator!=(const Type{name} &x) const {{
            {0:indent$}{0:indent$}{0:indent$}return !(*this == x);
            {0:indent$}{0:indent$}}}
            {0:indent$}{0:indent$}/**
            {0:indent$}{0:indent$} * @brief Prints the object onto the output stream
            {0:indent$}{0:indent$} * 
            {0:indent$}{0:indent$} * @param os The output stream to print to
            {0:indent$}{0:indent$} * @param x The object to print
            {0:indent$}{0:indent$} * @return The output stream
            {0:indent$}{0:indent$} */
            {0:indent$}{0:indent$}friend std::ostream &operator<<(std::ostream &os, const Type{name} &x);
            {0:indent$}}};",
            "",
            name = self.name,
        );
    }

    /// Gets the source code for the wrapper struct of this enum type
    ///
    /// # Parameters
    ///
    /// enum_name: The name of the enum
    ///
    /// indent: The indentation to use
    fn get_wrapper_source(&self, enum_name: &str, indent: usize) -> String {
        // Get the parameter name
        let param_name = match &self.data_type {
            Some(_) => "x",
            None => "",
        };

        // Get the comparison operation
        let comparison = match &self.data_type {
            Some(_) => "value == x.value",
            None => "true",
        };

        // Get the printer
        let printer = match &self.data_type {
            Some(_) => "\"{ value: \" << x.value << \" }\"",
            None => "\"{  }\"",
        };

        return formatdoc!("
            [[nodiscard]] bool {enum_name}::Type{name}::operator==(const Type{name} &{param_name}) const {{
            {0:indent$}return {comparison};
            }}

            std::ostream &operator<<(std::ostream &os, const {enum_name}::Type{name} &{param_name}) {{
            {0:indent$}return os << {printer};
            }}",
            "",
            name = self.name,
        );
    }

    /// Gets the name of the wrapper struct of this enum type
    fn get_wrapper_name(&self) -> String {
        return format!("Type{name}", name = self.name);
    }

    /// Gets the printer of this enum type
    ///
    /// # Parameters
    ///
    /// main_name: The name of the main enum
    ///
    /// indent: The indentation to use
    fn get_printer(&self, main_name: &str, indent: usize) -> String {
        // Get what it should print
        let printer = match self.data_type {
            Some(_) => format!(
                "\"{name}(\" << std::get<{main_name}::Type{name}>(x.value).value << \")\"",
                name = self.name
            ),
            None => "\"Empty\"".to_string(),
        };

        return formatdoc!(
            "
            {0:indent$}case {main_name}::Enum::k{name}:
            {0:indent$}{0:indent$}os << {printer};
            {0:indent$}{0:indent$}break;",
            "",
            name = self.name,
        );
    }

    /// Gets the parser for the node value for this enum type
    ///
    /// # Parameters
    ///
    /// typename: The typename of the main type
    ///
    /// indent: The indentation to use
    fn get_parser_value(&self, typename: &str, indent: usize) -> String {
        let internal = match &self.data_type {
            Some(_) => format!(
                "return Result<{typename}>::err(Error(\"Enum type {name} must contain a value\"));",
                name = self.name
            ),
            None => format!(
                "return Result<{typename}>::ok({typename}({typename}::Type{name}{{}}));",
                name = self.name
            ),
        };

        return formatdoc!(
            "
            {0:indent$}if (value_ == \"{name}\") {{
            {0:indent$}{0:indent$}{internal}
            {0:indent$}}}",
            "",
            name = self.name,
        );
    }

    /// Gets the parser for the node map for this enum type
    ///
    /// # Parameters
    ///
    /// typename: The typename of the main type
    ///
    /// namespace: The namespace of the enum
    ///
    /// data_types: List of all the data types defined in the data model
    ///
    /// indent: The indentation to use
    fn get_parser_map(
        &self,
        typename: &str,
        namespace: &str,
        data_types: &[DataType],
        indent: usize,
    ) -> String {
        let internal = match &self.data_type {
            Some(data_type) => {
                // Add possible namespace to the typename
                let data_type = if let Some(_) = data_types.iter().find(|new_data_type| &new_data_type.name == data_type) {
                    format!("{namespace}{data_type}")
                } else {
                    format!("{data_type}")
                };

                formatdoc!("
                    {0:indent$}{0:indent$}Result<{data_type}> value = map_.cbegin()->second.to_value<{data_type}>();
                    {0:indent$}{0:indent$}if (value.is_ok()) {{
                    {0:indent$}{0:indent$}{0:indent$}return Result<{typename}>::ok({typename}({typename}::Type{name}{{value.get_ok()}}));
                    {0:indent$}{0:indent$}}}
                    {0:indent$}{0:indent$}return Result<{typename}>::err(value.get_err().add_field(\"{name}\"));",
                    "",
                    name = self.name,
                )
            },
            None => format!("{0:indent$}{0:indent$}return Result<{typename}>::err(Error(\"Enum type {name} must not include values\"));", "", name = self.name),
        };

        return formatdoc!(
            "
            {0:indent$}if (map_.cbegin()->first == \"{name}\") {{
            {internal}
            {0:indent$}}}",
            "",
            name = self.name,
        );
    }

    /// Gets the parser for export the enum type to a node
    ///
    /// # Parameters
    ///
    /// typename: The typename of the main type
    ///
    /// indent: The indentation to use
    fn get_parser_export(&self, typename: &str, indent: usize) -> String {
        let internal = match &self.data_type {
            Some(_) => formatdoc!("
                {0:indent$}{0:indent$}map.insert({{
                {0:indent$}{0:indent$}{0:indent$}\"{name}\",
                {0:indent$}{0:indent$}{0:indent$}Node::from_value(std::get<{typename}::Type{name}>(value.value).value)
                {0:indent$}{0:indent$}}});
                {0:indent$}{0:indent$}return Node(Node::Map(std::move(map)));",
                "",
                name = self.name
            ),
            None => formatdoc!("
                {0:indent$}{0:indent$}return Node(Node::Value(\"{name}\"));",
                "",
                name = self.name
            ),
        };

        return formatdoc!(
            "
            {0:indent$}case {typename}::Enum::k{name}:
            {internal}
            ",
            "",
            name = self.name,
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
        compile_and_test("type_enum/basic");

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
                data: DataTypeData::Enum(Enum {
                    types: vec![
                        EnumType {
                            name: "Int1".to_string(),
                            description: Some("An integer".to_string()),
                            data_type: Some("int".to_string()),
                        },
                        EnumType {
                            name: "Int2".to_string(),
                            description: Some("Another integer".to_string()),
                            data_type: Some("int".to_string()),
                        },
                        EnumType {
                            name: "Float".to_string(),
                            description: None,
                            data_type: Some("float".to_string()),
                        },
                        EnumType {
                            name: "Empty".to_string(),
                            description: Some("Nothing".to_string()),
                            data_type: None,
                        },
                    ],
                }),
            }],
            namespace: vec!["test".to_string()],
            macros: HashMap::new(),
        };

        // Create the header file
        let header_file = data_model.get_header("HEADER", 2);
        let source_file = data_model.get_source("basic", 2).unwrap();
        let expected_header = include_str!("../../tests/cpp/type_enum/basic/basic.h");
        let expected_source = include_str!("../../tests/cpp/type_enum/basic/basic.cpp");
        //println!("header:\n{header_file}\n---\n");
        //println!("source:\n{source_file}\n---\n");

        // Check that they are the same
        assert_eq!(str_diff(&header_file, &expected_header), None);
        assert_eq!(str_diff(&source_file, &expected_source), None);
    }
}
