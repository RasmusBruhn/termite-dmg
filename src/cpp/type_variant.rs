use super::ToSnakeCase;
use crate::*;
use indoc::formatdoc;

/// Converts the variant to a string for use in the header file
///
/// # Parameters
///
/// data: The variant to generate the code for
///
/// name: The name of the variant
///
/// indent: The number of spaces to use for indentation
pub(super) fn generate_definition_header(data: &Variant, name: &str, indent: usize) -> String {
    // Create list of the variants
    let variant_list = data
        .data_types
        .iter()
        .map(|data_type| variant_type::get_typename(data_type))
        .collect::<Vec<_>>()
        .join(", ");

    let constructors = data
        .data_types
        .iter()
        .map(|data_type| variant_type::get_constructor(data_type, name, indent))
        .collect::<Vec<_>>()
        .join("\n");

    return formatdoc!(
        "
        struct {name} {{
        {0:indent$}/**
        {0:indent$} * @brief Constructs a new {name} object
        {0:indent$} * 
        {0:indent$} * @param value The value of the variant
        {0:indent$} */
        {0:indent$}{name}(std::variant<{variant_list}> value) : value(std::move(value)) {{}}
        {constructors}

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
/// data: The variant to generate the code for
///
/// name: The name of the variant
///
/// indent: The number of spaces to use for indentation
pub(super) fn generate_definition_source(data: &Variant, name: &str, indent: usize) -> String {
    // Create writer
    let writer_specifiers = data
        .data_types
        .iter()
        .enumerate()
        .map(|(index, data_type)| variant_type::get_printer_source(data_type, index, indent))
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
/// data: The variant to generate the code for
///
/// name: The name of the variant
///
/// indent: The number of spaces to use for indentation
///
/// namespace: The namespace of the variant
///
/// data_types: List of all the data types defined in the data model
pub(super) fn generate_parser_header(_data: &Variant, name: &str, namespace: &[String]) -> String {
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
/// data: The variant to generate the code for
///
/// name: The name of the variant
///
/// indent: The number of spaces to use for indentation
///
/// namespace: The namespace of the variant
pub(super) fn generate_parser_source(
    data: &Variant,
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

    // Get all the readers
    let readers = data
        .data_types
        .iter()
        .map(|data_type| variant_type::get_reader_source(data_type, &typename, &namespace, indent))
        .collect::<Vec<String>>()
        .join(&formatdoc!(
            "
            
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

mod variant_type {
    use super::*;

    /// Retrieves the typename corrected with namespace if it is not builtin
    ///
    /// # Parameters
    ///
    /// data: The type name of the variant type
    ///
    /// namespace: The namespace to prepend if the type is not builtin
    pub(super) fn get_typename_parser(data: &str, namespace: &str) -> String {
        if is_name_builtin(data) {
            format!("{data}")
        } else {
            format!("{namespace}{data}")
        }
    }

    /// Retrieves the typename corrected with termite:: if it is builtin
    ///
    /// # Parameters
    ///
    /// data: The type name of the variant type
    pub(super) fn get_typename(data: &str) -> String {
        if is_name_builtin(data) {
            format!("termite::{data}")
        } else {
            format!("{data}")
        }
    }

    /// Constructs the snake_case version of the given data string
    ///
    /// # Parameters
    ///
    /// data: The type name to convert to snake_case
    pub(super) fn get_snake_case(data: &str) -> String {
        return ToSnakeCase::new(&mut data.chars()).collect::<String>();
    }

    /// Constructs the source code for the readers used in to_value
    ///
    /// # Parameters
    ///
    /// data: The type name of the variant type
    ///
    /// name: The name of the variant
    ///
    /// namespace: The namespace to prepend if the type is not builtin
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn get_reader_source(
        data: &str,
        name: &str,
        namespace: &str,
        indent: usize,
    ) -> String {
        let snake_case = get_snake_case(data);
        let data_type = get_typename_parser(&data, namespace);

        return formatdoc!(
            "
            {0:indent$}Result<{data_type}> result_{snake_case} = to_value<{data_type}>();
            {0:indent$}if (result_{snake_case}.is_ok()) {{
            {0:indent$}{0:indent$}return Result<{name}>::ok({name}(result_{snake_case}.get_ok()));
            {0:indent$}}}
            {0:indent$}error << \"{data} {{ \" << result_{snake_case}.get_err() << \" }}\";",
            "",
        );
    }

    /// Constructs the source code for the printer used in the variant type
    ///
    /// # Parameters
    ///
    /// data: The type name of the variant type
    ///
    /// index: The index of the variant in the variant type
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn get_printer_source(data: &str, index: usize, indent: usize) -> String {
        let data_type = get_typename(data);

        return formatdoc!(
            "
            {0:indent$}case {index}:
            {0:indent$}{0:indent$}os << \"{data_type} \" << std::get<{data_type}>(x.value);
            {0:indent$}{0:indent$}break;",
            "",
        );
    }

    /// Constructs the constructor for the variant type
    ///
    /// # Parameters
    ///
    /// data: The type name of the variant type
    ///
    /// name: The name of the variant
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn get_constructor(data: &str, name: &str, indent: usize) -> String {
        let data_type = get_typename(data);

        return formatdoc!(
            "
            {0:indent$}/**
            {0:indent$} * @brief Constructs a new {name} object
            {0:indent$} * 
            {0:indent$} * @param value The value of the variant
            {0:indent$} */
            {0:indent$}{name}({data_type} value) : value(std::move(value)) {{}}
            ",
            "",
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::cpp::test_utils::*;

    #[test]
    fn basic() {
        run_test("type_variant/basic", true, false, false);
    }
}
