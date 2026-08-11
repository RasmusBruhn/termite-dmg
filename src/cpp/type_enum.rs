use crate::*;
use indoc::formatdoc;

/// Converts the enum to a string for use in the header file
///
/// # Parameters
///
/// data: The enum to generate code for
///
/// name: The name of the enum
///
/// indent: The number of spaces to use for indentation
pub(super) fn generate_definition_header(data: &Enum, name: &str, indent: usize) -> String {
    // Get enum type definitions
    let type_definition = data
        .types
        .iter()
        .map(|enum_type| enum_type::get_definition(enum_type, indent))
        .collect::<Vec<String>>()
        .join("\n");

    // Get the type wrappers
    let type_wrappers = data
        .types
        .iter()
        .map(|enum_type| enum_type::get_wrapper_header(enum_type, indent))
        .collect::<Vec<String>>()
        .join("\n\n");

    // Get the wrapper name list
    let wrapper_list = data
        .types
        .iter()
        .map(|enum_type| enum_type::get_wrapper_name(enum_type))
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
/// data: The enum to generate code for
///
/// name: The name of the enum
///
/// indent: The number of spaces to use for indentation
pub(super) fn generate_definition_source(data: &Enum, name: &str, indent: usize) -> String {
    // Get the type wrappers
    let type_wrappers = data
        .types
        .iter()
        .map(|enum_type| enum_type::get_wrapper_source(enum_type, name, indent))
        .collect::<Vec<String>>()
        .join("\n\n");

    // Get the printers
    let printers = data
        .types
        .iter()
        .map(|enum_type| enum_type::get_printer(enum_type, name, indent))
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
/// data: The enum to generate code for
///
/// name: The name of the enum
///
/// namespace: The namespace of the enum
pub(super) fn generate_parser_header(_data: &Enum, name: &str, namespace: &[String]) -> String {
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
/// data: The enum to generate code for
///
/// name: The name of the enum
///
/// indent: The number of spaces to use for indentation
///
/// namespace: The namespace of the enum
pub(super) fn generate_parser_source(
    data: &Enum,
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

    // Get the value parser
    let value_parsers = data
        .types
        .iter()
        .map(|enum_type| enum_type::get_parser_value(enum_type, &typename, indent))
        .collect::<Vec<String>>()
        .join("\n");

    // Get the map parser
    let map_parsers = data
        .types
        .iter()
        .map(|enum_type| enum_type::get_parser_map(enum_type, &typename, &namespace, indent))
        .collect::<Vec<String>>()
        .join("\n");

    // Get the export parser
    let export_parsers = data
        .types
        .iter()
        .map(|enum_type| enum_type::get_parser_export(enum_type, &typename, indent))
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

mod enum_type {
    use super::*;

    /// Gets the description of this enum type
    ///
    /// # Parameters
    ///
    /// data: The enum type to generate code for
    pub(super) fn get_description(data: &EnumType) -> String {
        return match &data.description {
            Some(description) => description.clone(),
            None => "".to_string(),
        };
    }

    /// Gets the definition of this enum type
    ///
    /// # Parameters
    ///
    /// data: The enum type to generate code for
    ///
    /// indent: The indentation to use
    pub(super) fn get_definition(data: &EnumType, indent: usize) -> String {
        return formatdoc!(
            "
            {0:indent$}{0:indent$}/**
            {0:indent$}{0:indent$} * @brief {description}
            {0:indent$}{0:indent$} * 
            {0:indent$}{0:indent$} */
            {0:indent$}{0:indent$}k{name},",
            "",
            description = get_description(data),
            name = data.name,
        );
    }

    /// Gets the header code for the wrapper struct of this enum type
    ///
    /// # Parameters
    ///
    /// data: The enum type to generate code for
    ///
    /// indent: The indentation to use
    pub(super) fn get_wrapper_header(data: &EnumType, indent: usize) -> String {
        // Get the definition of the type
        let type_definition = match &data.data_type {
            Some(data_type) => {
                let data_type =
                    if ["string", "number", "integer", "boolean"].contains(&data_type.as_str()) {
                        format!("termite::{data_type}")
                    } else {
                        data_type.clone()
                    };

                formatdoc!(
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
                    name = data.name,
                )
            }
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
            name = data.name,
        );
    }

    /// Gets the source code for the wrapper struct of this enum type
    ///
    /// # Parameters
    ///
    /// data: The enum type to generate code for
    ///
    /// enum_name: The name of the enum
    ///
    /// indent: The indentation to use
    pub(super) fn get_wrapper_source(data: &EnumType, enum_name: &str, indent: usize) -> String {
        // Get the parameter name
        let param_name = match &data.data_type {
            Some(_) => "x",
            None => "",
        };

        // Get the comparison operation
        let comparison = match &data.data_type {
            Some(_) => "value == x.value",
            None => "true",
        };

        // Get the printer
        let printer = match &data.data_type {
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
            name = data.name,
        );
    }

    /// Gets the name of the wrapper struct of this enum type
    ///
    /// # Parameters
    ///
    /// data: The enum type to generate code for
    pub(super) fn get_wrapper_name(data: &EnumType) -> String {
        return format!("Type{name}", name = data.name);
    }

    /// Gets the printer of this enum type
    ///
    /// # Parameters
    ///
    /// data: The enum type to generate code for
    ///
    /// main_name: The name of the main enum
    ///
    /// indent: The indentation to use
    pub(super) fn get_printer(data: &EnumType, main_name: &str, indent: usize) -> String {
        // Get what it should print
        let printer = match data.data_type {
            Some(_) => format!(
                "\"{name}(\" << std::get<{main_name}::Type{name}>(x.value).value << \")\"",
                name = data.name
            ),
            None => "\"Empty\"".to_string(),
        };

        return formatdoc!(
            "
            {0:indent$}case {main_name}::Enum::k{name}:
            {0:indent$}{0:indent$}os << {printer};
            {0:indent$}{0:indent$}break;",
            "",
            name = data.name,
        );
    }

    /// Gets the parser for the node value for this enum type
    ///
    /// # Parameters
    ///
    /// data: The enum type to generate code for
    ///
    /// typename: The typename of the main type
    ///
    /// indent: The indentation to use
    pub(super) fn get_parser_value(data: &EnumType, typename: &str, indent: usize) -> String {
        let internal = match &data.data_type {
            Some(_) => format!(
                "return Result<{typename}>::err(Error(\"Enum type {name} must contain a value\"));",
                name = data.name
            ),
            None => format!(
                "return Result<{typename}>::ok({typename}({typename}::Type{name}{{}}));",
                name = data.name
            ),
        };

        return formatdoc!(
            "
            {0:indent$}if (value_ == \"{name}\") {{
            {0:indent$}{0:indent$}{internal}
            {0:indent$}}}",
            "",
            name = data.name,
        );
    }

    /// Gets the parser for the node map for this enum type
    ///
    /// # Parameters
    ///
    /// data: The enum type to generate code for
    ///
    /// typename: The typename of the main type
    ///
    /// namespace: The namespace of the enum
    ///
    /// indent: The indentation to use
    pub(super) fn get_parser_map(
        data: &EnumType,
        typename: &str,
        namespace: &str,
        indent: usize,
    ) -> String {
        let internal = match &data.data_type {
            Some(data_type) => {
                // Add possible namespace to the typename
                let data_type = if is_name_builtin(data_type) {
                    format!("{data_type}")
                } else {
                    format!("{namespace}{data_type}")
                };

                formatdoc!("
                    {0:indent$}{0:indent$}Result<{data_type}> value = map_.cbegin()->second.to_value<{data_type}>();
                    {0:indent$}{0:indent$}if (value.is_ok()) {{
                    {0:indent$}{0:indent$}{0:indent$}return Result<{typename}>::ok({typename}({typename}::Type{name}{{value.get_ok()}}));
                    {0:indent$}{0:indent$}}}
                    {0:indent$}{0:indent$}return Result<{typename}>::err(value.get_err().add_field(\"{name}\"));",
                    "",
                    name = data.name,
                )
            },
            None => format!("{0:indent$}{0:indent$}return Result<{typename}>::err(Error(\"Enum type {name} must not include values\"));", "", name = data.name),
        };

        return formatdoc!(
            "
            {0:indent$}if (map_.cbegin()->first == \"{name}\") {{
            {internal}
            {0:indent$}}}",
            "",
            name = data.name,
        );
    }

    /// Gets the parser for export the enum type to a node
    ///
    /// # Parameters
    ///
    /// data: The enum type to generate code for
    ///
    /// typename: The typename of the main type
    ///
    /// indent: The indentation to use
    pub(super) fn get_parser_export(data: &EnumType, typename: &str, indent: usize) -> String {
        let internal = match &data.data_type {
            Some(_) => formatdoc!("
                {0:indent$}{0:indent$}map.insert({{
                {0:indent$}{0:indent$}{0:indent$}\"{name}\",
                {0:indent$}{0:indent$}{0:indent$}Node::from_value(std::get<{typename}::Type{name}>(value.value).value)
                {0:indent$}{0:indent$}}});
                {0:indent$}{0:indent$}return Node(Node::Map(std::move(map)));",
                "",
                name = data.name
            ),
            None => formatdoc!("
                {0:indent$}{0:indent$}return Node(Node::Value(\"{name}\"));",
                "",
                name = data.name
            ),
        };

        return formatdoc!(
            "
            {0:indent$}case {typename}::Enum::k{name}:
            {internal}
            ",
            "",
            name = data.name,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::cpp::test_utils::*;

    #[test]
    fn basic() {
        run_test("type_enum/basic", true, false, false);
    }
}
