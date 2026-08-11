use crate::*;
use indoc::formatdoc;

/// Converts the array to a string for use in the header file
///
/// # Parameters
///
/// data: The array to generate code for
///
/// name: The name of the array
///
/// indent: The number of spaces to use for indentation
pub(super) fn generate_definition_header(data: &Array, name: &str, indent: usize) -> String {
    let data_type = if ["string", "number", "integer", "boolean"].contains(&data.data_type.as_str())
    {
        format!("termite::{data_type}", data_type = data.data_type)
    } else {
        data.data_type.clone()
    };

    return formatdoc!(
        "
        struct {name} {{
        public:
        {0:indent$}/**
        {0:indent$} * @brief Constructs a new {name} object
        {0:indent$} * 
        {0:indent$} * @param values The values of the array
        {0:indent$} */
        {0:indent$}explicit {name}(std::vector<{data_type}> values) : values(std::move(values)) {{}}

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
        {0:indent$}std::vector<{data_type}> values;
        }};",
        "",
    );
}

/// Converts the array to a string for use in the source file
///
/// # Parameters
///
/// data: The array to generate code for
///
/// name: The name of the array
///
/// indent: The number of spaces to use for indentation
pub(super) fn generate_definition_source(_data: &Array, name: &str, indent: usize) -> String {
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
/// data: The array to generate code for
///
/// name: The name of the array
///
/// namespace: The namespace of the array
pub(super) fn generate_parser_header(_data: &Array, name: &str, namespace: &[String]) -> String {
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
/// data: The array to generate code for
///
/// name: The name of the array
///
/// indent: The number of spaces to use for indentation
///
/// namespace: The namespace of the array
pub(super) fn generate_parser_source(
    data: &Array,
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

#[cfg(test)]
mod tests {
    use crate::cpp::test_utils::*;

    #[test]
    fn basic() {
        run_test("type_array/basic", true, false, false);
    }
}
