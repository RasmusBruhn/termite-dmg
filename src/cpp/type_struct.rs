use super::ToSnakeCase;
use crate::*;
use indoc::formatdoc;
use std::collections::{HashMap, HashSet};

/// Converts the struct to a string for use in the header file
///
/// # Parameters
///
/// data: The struct to generate code for
///
/// name: The name of the struct
///
/// indent: The number of spaces to use for indentation
pub(super) fn generate_definition_header(data: &Struct, name: &str, indent: usize) -> String {
    let mut fields = data.fields.iter().collect::<Vec<_>>();
    fields.sort_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs));

    // Get the description for the constructor
    let constructor_description = fields
        .iter()
        .map(|(field_name, field)| {
            struct_field::get_constructor_description(field, field_name, indent)
        })
        .collect::<Vec<String>>()
        .join("");

    // Get the constructor parameters
    let constructor_parameters = fields
        .iter()
        .map(|(field_name, field)| struct_field::get_constructor_parameter(field, field_name))
        .collect::<Vec<String>>()
        .join("");

    // Get the list of setters for the internal constructor
    let constructor_setters = fields
        .iter()
        .map(|(field_name, _)| struct_field::get_constructor_setter(field_name))
        .collect::<Vec<String>>()
        .join("");

    // Get all constructors for the fields with default values
    let default_constructors = fields
        .iter()
        .map(|(field_name, field)| {
            struct_field::get_default_constructor_header(field, field_name, indent)
        })
        .collect::<Vec<String>>()
        .join("");
    let default_constructors = format!("\n{default_constructors}");

    // Get the definitions of all the fields but without any initialization
    let field_definitions = fields
        .iter()
        .map(|(field_name, field)| struct_field::get_definition(field, field_name, indent))
        .collect::<Vec<String>>()
        .join("");

    // Generate the code
    return formatdoc!("
        struct {name} {{
        public:
        {0:indent$}/**
        {0:indent$} * @brief Constructs a new {name} object
        {0:indent$} * {constructor_description}
        {0:indent$} * @param extra_fields Any extra fields to attach to this struct
        {0:indent$} */
        {0:indent$}explicit {name}({constructor_parameters}::termite::Node::Map extra_fields = ::termite::Node::Map()) : {constructor_setters}extra_fields(std::move(extra_fields)) {{}}
        {default_constructors}
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
        {field_definitions}
        {0:indent$}/**
        {0:indent$} * @brief All extra fields from when reading which could not be captured
        {0:indent$} * 
        {0:indent$} */
        {0:indent$}::termite::Node::Map extra_fields;
        }};", "",
    );
}

/// Converts the struct to a string for use in the source file
///
/// # Parameters
///
/// data: The struct to generate code for
///
/// name: The name of the struct
///
/// macros: A map of all macros to expand default values
///
/// indent: The number of spaces to use for indentation
pub(super) fn generate_definition_source(
    data: &Struct,
    name: &str,
    macros: &HashMap<String, SerializationModel>,
    indent: usize,
) -> Result<String, Error> {
    let mut fields = data.fields.iter().collect::<Vec<_>>();
    fields.sort_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs));

    // Get the equality test
    let equality_test = fields
        .iter()
        .map(|(field_name, _)| struct_field::get_equality_check(field_name))
        .collect::<Vec<_>>()
        .join("");

    // Get the printout for the operator<< function
    let printout = fields
        .iter()
        .map(|(field_name, _)| struct_field::get_printout(field_name))
        .collect::<Vec<_>>()
        .join("");

    // Get all constructors for the fields with default values
    let default_constructors = fields
        .iter()
        .map(|(field_name, field)| {
            struct_field::get_default_constructor_source(field, field_name, name, macros, indent)
        })
        .collect::<Result<Vec<_>, _>>()?
        .join("");

    // Generate the code
    return Ok(formatdoc!("
        [[nodiscard]] bool {name}::operator==(const {name} &x) const {{
        {0:indent$}return {equality_test}extra_fields == x.extra_fields;
        }}
        {default_constructors}
        std::ostream &operator<<(std::ostream &os, const {name} &x) {{
        {0:indent$}return os << \"{{ \" << {printout}\"extra_fields: \" << x.extra_fields << \" }}\";
        }}", "",
    ));
}

/// Gets the header code for the parser for this struct allowing it to be read from a file
///
/// # Parameters
///
/// data: The struct to generate code for
///
/// name: The name of the struct
///
/// namespace: The namespace of the struct
pub(super) fn generate_parser_header(_data: &Struct, name: &str, namespace: &[String]) -> String {
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
        [[nodiscard]] Result<{typename}> Node::Map::to_value<{typename}>() const;

        template<>
        [[nodiscard]] Node Node::from_value<{typename}>(const {typename} &value);",
    );
}

/// Gets the source code for the parser for this struct allowing it to be read from a file
///
/// # Parameters
///
/// data: The struct to generate code for
///
/// name: The name of the struct
///
/// indent: The number of spaces to use for indentation
///
/// namespace: The namespace of the struct
pub(super) fn generate_parser_source(
    data: &Struct,
    name: &str,
    indent: usize,
    namespace: &[String],
) -> String {
    let mut fields = data.fields.iter().collect::<Vec<_>>();
    fields.sort_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs));

    // Get the namespace name
    let namespace = namespace
        .iter()
        .map(|single_name| format!("{single_name}::"))
        .collect::<Vec<String>>()
        .join("");
    let typename = format!("{namespace}{name}");

    // Get the parameter parsing
    let parsing = fields
        .iter()
        .map(|(field_name, field)| {
            struct_field::get_parsing(field, field_name, &typename, &namespace, indent)
        })
        .collect::<Vec<String>>()
        .join("");

    let parsing_export = fields
        .iter()
        .map(|(field_name, field)| struct_field::get_parsing_export(field, field_name, indent))
        .collect::<Vec<String>>()
        .join("");

    // Get the parameter list for when retrieving them to return at the end
    let parameter_retrievals = fields
        .iter()
        .map(|(field_name, _)| struct_field::get_parameter_retrieval(field_name))
        .collect::<Vec<String>>()
        .join("");

    return formatdoc!("
        template<>
        [[nodiscard]] Result<{typename}> Node::Map::to_value<{typename}>() const {{
        {0:indent$}std::map<std::string, Node> map = map_;
        {parsing}
        {0:indent$}return Result<{typename}>::ok({typename}({parameter_retrievals}Map(std::move(map))));
        }}

        template<>
        [[nodiscard]] Node Node::from_value<{typename}>(const {typename} &value) {{
        {0:indent$}std::map<std::string, Node> map = value.extra_fields.get();
        {parsing_export}
        {0:indent$}return Node(Node::Map(std::move(map)));
        }}",
        "",
    );
}

mod struct_field {
    use super::*;

    /// Constructs the c++ typename of this field
    ///
    /// # Parameters
    ///
    /// data: The struct field to generate code for
    pub(super) fn get_typename(data: &StructField) -> String {
        let data_type = if is_name_builtin(&data.data_type) {
            format!("termite::{data_type}", data_type = data.data_type)
        } else {
            data.data_type.clone()
        };

        return match &data.default {
            DefaultType::Optional => {
                format!("std::optional<{data_type}>")
            }
            _ => data_type,
        };
    }

    /// Gets the description of this field
    ///
    /// # Parameters
    ///
    /// data: The struct field to generate code for
    pub(super) fn get_description(data: &StructField) -> String {
        return match &data.description {
            Some(description) => description.clone(),
            None => "".to_string(),
        };
    }

    /// Gets the description for the public constructor
    ///
    /// # Parameters
    ///
    /// data: The struct field to generate code for
    ///
    /// name: The name of the struct field
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn get_constructor_description(
        data: &StructField,
        name: &str,
        indent: usize,
    ) -> String {
        return format!(
            "\n{0:indent$} * @param {name} {description}",
            "",
            description = get_description(data),
        );
    }

    /// Get the parameter definition for the constructor including default value
    ///
    /// # Parameters
    ///
    /// data: The struct field to generate code for
    ///
    /// name: The name of the struct field
    pub(super) fn get_constructor_parameter(data: &StructField, name: &str) -> String {
        return format!("{typename} {name}, ", typename = get_typename(data),);
    }

    /// Get the parameter definition for the constructor including default value
    ///
    /// # Parameters
    ///
    /// data: The struct field to generate code for
    ///
    /// name: The name of the struct field
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn get_default_constructor_header(
        data: &StructField,
        name: &str,
        indent: usize,
    ) -> String {
        return match &data.default {
            DefaultType::Required => format!(""),
            _ => formatdoc!(
                "
                {0:indent$}/**
                {0:indent$} * @brief Gets the default value for {name}
                {0:indent$} * 
                {0:indent$} * @return The default value for {name}
                {0:indent$} */
                {0:indent$}[[nodiscard]] static {typename} default_{name}();\n",
                "",
                typename = get_typename(data),
            ),
        };
    }

    /// Get the source code for the parameter definition for the constructor including default value
    ///
    /// # Parameters
    ///
    /// data: The struct field to generate code for
    ///
    /// name: The name of the struct field
    ///
    /// main_name: The name of the type which holds this field
    ///
    /// macros: A map of all macros to expand default values
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn get_default_constructor_source(
        data: &StructField,
        name: &str,
        main_name: &str,
        macros: &HashMap<String, SerializationModel>,
        indent: usize,
    ) -> Result<String, Error> {
        return Ok(match &data.default {
            DefaultType::Required => format!(""),
            DefaultType::Optional => formatdoc!(
                "
                \n[[nodiscard]] {typename} {main_name}::default_{snake_case}() {{
                {0:indent$}return std::nullopt;
                }}\n",
                "",
                typename = get_typename(data),
                snake_case = ToSnakeCase::new(&mut name.chars()).collect::<String>(),
            ),
            DefaultType::Default(default_value) => formatdoc!(
                "
                \n[[nodiscard]] {typename} {main_name}::default_{snake_case}() {{
                {default_value}

                {0:indent$}return default_value.to_value<{typename}>().get_ok();
                }}\n",
                "",
                typename = get_typename(data),
                snake_case = ToSnakeCase::new(&mut name.chars()).collect::<String>(),
                default_value = serialization::generate(
                    &expand_macros(default_value, macros, &mut HashSet::new())?,
                    "default_value",
                    indent,
                    indent
                ),
            ),
        });
    }

    /// Gets the equality check for this field
    ///
    /// # Parameters
    ///
    /// name: The name of the struct field
    pub(super) fn get_equality_check(name: &str) -> String {
        return format!("this->{name} == x.{name} && ");
    }

    /// Gets the printout of this field for the operator>> ostream function
    ///
    /// # Parameters
    ///
    /// name: The name of the struct field
    pub(super) fn get_printout(name: &str) -> String {
        return format!("\"{name}: \" << x.{name} << \", \" << ");
    }

    /// Get the setter for this field for the internal constructor
    ///
    /// # Parameters
    ///
    /// name: The name of the struct field
    pub(super) fn get_constructor_setter(name: &str) -> String {
        return format!("{name}(std::move({name})), ");
    }

    /// Gets the description if it is supplied
    ///
    /// # Parameters
    ///
    /// data: The struct field to generate code for
    ///
    /// name: The name of the struct field
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn get_definition(data: &StructField, name: &str, indent: usize) -> String {
        return formatdoc!(
            "
            \n{0:indent$}/**
            {0:indent$} * @brief {description}
            {0:indent$} * 
            {0:indent$} */
            {0:indent$}{typename} {name};",
            "",
            typename = get_typename(data),
            description = get_description(data),
        );
    }

    /// Gets the parsing for this field if it is required
    ///
    /// # Parameters
    ///
    /// data: The struct field to generate code for
    ///
    /// name: The name of the struct field
    ///
    /// main_name: The name of the type which holds this field including namespace
    ///
    /// namespace: The namespace of the struct
    ///
    /// indent: The indentation to use
    pub(super) fn get_parsing_required(
        data: &StructField,
        name: &str,
        main_name: &str,
        namespace: &str,
        indent: usize,
    ) -> String {
        // Add possible namespace to the typename
        let typename = if is_name_builtin(&data.data_type) {
            format!("{data_type}", data_type = data.data_type)
        } else {
            format!("{namespace}{data_type}", data_type = data.data_type)
        };

        return formatdoc!("
            \n{0:indent$}auto location_{name} = map.find(\"{name}\");
            {0:indent$}if (location_{name} == map.end()) {{
            {0:indent$}{0:indent$}return Result<{main_name}>::err(Error(\"Missing {name}\"));
            {0:indent$}}}
            {0:indent$}Result<{typename}> raw_value_{name} = location_{name}->second.to_value<{typename}>();
            {0:indent$}if (!raw_value_{name}.is_ok()) {{
            {0:indent$}{0:indent$}Error error = raw_value_{name}.get_err();
            {0:indent$}{0:indent$}error.add_field(\"{name}\");
            {0:indent$}{0:indent$}return Result<{main_name}>::err(std::move(error));
            {0:indent$}}}
            {0:indent$}{typename} value_{name} = raw_value_{name}.get_ok();
            {0:indent$}map.erase(location_{name});\n",
            "",
        );
    }

    /// Gets the parsing for this field if it is optional
    ///
    /// # Parameters
    ///
    /// data: The struct field to generate code for
    ///
    /// name: The name of the struct field
    ///
    /// main_name: The name of the type which holds this field including namespace
    ///
    /// namespace: The namespace of the struct
    ///
    /// indent: The indentation to use
    pub(super) fn get_parsing_optional(
        data: &StructField,
        name: &str,
        main_name: &str,
        namespace: &str,
        indent: usize,
    ) -> String {
        // Add possible namespace to the typename
        let base_typename = if is_name_builtin(&data.data_type) {
            format!("{data_type}", data_type = data.data_type)
        } else {
            format!("{namespace}{data_type}", data_type = data.data_type)
        };

        let typename = match &data.default {
            DefaultType::Optional => format!("std::optional<{base_typename}>"),
            _ => base_typename.clone(),
        };

        // Get default value
        let default = match &data.default {
            DefaultType::Required => format!(""),
            _ => format!(
                " = {main_name}::default_{snake_case}()",
                snake_case = ToSnakeCase::new(&mut name.chars()).collect::<String>(),
            ),
        };

        return formatdoc!("
            \n{0:indent$}auto location_{name} = map.find(\"{name}\");
            {0:indent$}{typename} value_{name}{default};
            {0:indent$}if (location_{name} != map.end()) {{
            {0:indent$}{0:indent$}Result<{base_typename}> raw_value_{name} = location_{name}->second.to_value<{base_typename}>();
            {0:indent$}{0:indent$}if (!raw_value_{name}.is_ok()) {{
            {0:indent$}{0:indent$}{0:indent$}Error error = raw_value_{name}.get_err();
            {0:indent$}{0:indent$}{0:indent$}error.add_field(\"{name}\");
            {0:indent$}{0:indent$}{0:indent$}return Result<{main_name}>::err(std::move(error));
            {0:indent$}{0:indent$}}}
            {0:indent$}{0:indent$}value_{name} = raw_value_{name}.get_ok();
            {0:indent$}{0:indent$}map.erase(location_{name});
            {0:indent$}}}\n",
            "",
        );
    }

    /// Gets the parsing for this field
    ///
    /// # Parameters
    ///
    /// data: The struct field to generate code for
    ///
    /// name: The name of the struct field
    ///
    /// main_name: The name of the type which holds this field including namespace
    ///
    /// namespace: The namespace of the struct
    ///
    /// indent: The indentation to use
    pub(super) fn get_parsing(
        data: &StructField,
        name: &str,
        main_name: &str,
        namespace: &str,
        indent: usize,
    ) -> String {
        return match data.default {
            DefaultType::Required => get_parsing_required(data, name, main_name, namespace, indent),
            _ => get_parsing_optional(data, name, main_name, namespace, indent),
        };
    }

    /// Gets the parsing export function for this field
    ///
    /// # Parameters
    ///
    /// data: The struct field to generate code for
    ///
    /// name: The name of the struct field
    ///
    /// indent: The indentation to use
    pub(super) fn get_parsing_export(data: &StructField, name: &str, indent: usize) -> String {
        return match data.default {
            DefaultType::Optional => formatdoc!(
                "
                \n{0:indent$}if (value.{name}) {{
                {0:indent$}{0:indent$}map.insert({{\"{name}\", Node::from_value(*value.{name})}});
                {0:indent$}}}\n",
                "",
            ),
            _ => formatdoc!(
                "
                \n{0:indent$}map.insert({{\"{name}\", Node::from_value(value.{name})}});\n",
                "",
            ),
        };
    }

    /// Gets the value of this field when parsing after it is read
    ///
    /// # Parameters
    ///
    /// name: The name of the struct field
    pub(super) fn get_parameter_retrieval(name: &str) -> String {
        return format!("std::move(value_{name}), ");
    }
}

mod serialization {
    use super::*;

    /// Converts a serialization model to c++ code which constructs a termite::Node
    ///
    /// # Parameters
    ///
    /// data: The serialization model to convert
    ///
    /// name: The base name of the temporary variables to construct
    ///
    /// indent: The indentation to use for each level
    ///
    /// total_indent: The total indentation to use for this level
    pub(super) fn generate(
        data: &SerializationModel,
        name: &str,
        indent: usize,
        total_indent: usize,
    ) -> String {
        let next_indent = total_indent + indent;

        return match data {
            SerializationModel::Map(value) => {
                let (temp_values, entries) = value
                    .iter()
                    .enumerate()
                    .map(|(index, (key, value))| {
                        // Construct temporary value for the value
                        let temp_name = format!("{name}_{index}");
                        let temp_value = generate(value, &temp_name, indent, total_indent);

                        // Construct the entry
                        let entry = format!(
                            "{0:next_indent$}{{\"{key}\", {temp_name}}},",
                            "",
                            key = string_sanitize(key),
                        );

                        return (temp_value, entry);
                    })
                    .collect::<(Vec<_>, Vec<_>)>();

                let entries = entries.join("\n");
                let temp_values = temp_values.join("\n");

                formatdoc!("
                    {temp_values}
                    {0:total_indent$}auto {name} = termite::Node(termite::Node::Map(std::map<std::string, termite::Node>({{
                    {entries}
                    {0:total_indent$}}})));",
                    ""
                )
            }
            SerializationModel::Array(ref value) => {
                let (temp_values, entries) = value
                    .iter()
                    .enumerate()
                    .map(|(index, element)| {
                        // Construct temporary value for the value
                        let temp_name = format!("{name}_{index}");
                        let temp_value = generate(element, &temp_name, indent, total_indent);

                        // Construct the entry
                        let entry = format!("{0:next_indent$}{temp_name},", "");

                        return (temp_value, entry);
                    })
                    .collect::<(Vec<_>, Vec<_>)>();

                let entries = entries.join("\n");
                let temp_values = temp_values.join("\n");

                formatdoc!("
                    {temp_values}
                    {0:total_indent$}auto {name} = termite::Node(termite::Node::List(std::vector<termite::Node>({{
                    {entries}
                    {0:total_indent$}}})));",
                    ""
                )
            }
            SerializationModel::Value(ref value) => {
                formatdoc!(
                    "
                    {0:total_indent$}auto {name} = termite::Node(termite::Node::Value(\"{value}\"));",
                    "",
                    value = string_sanitize(value),
                )
            }
        };
    }
}

/// Sanitizes a string for use in c++ code
///
/// # Parameters
///
/// value: The string to sanitize
fn string_sanitize(value: &str) -> String {
    return value
        .replace("\n", "\\n")
        .replace("\r", "\\r")
        .replace("\t", "\\t")
        .replace("\\", "\\\\")
        .replace("\'", "\\\'")
        .replace("\"", "\\\"")
        .replace("\0", "\\0");
}

#[cfg(test)]
mod tests {
    use crate::cpp::test_utils::*;

    #[test]
    fn basic() {
        run_test("type_struct/basic", true, false, false);
    }

    #[test]
    fn description() {
        run_test("type_struct/description", true, false, false);
    }

    mod field {
        use super::*;

        #[test]
        fn basic() {
            run_test("type_struct/field/basic", true, false, false);
        }

        #[test]
        fn description() {
            run_test("type_struct/field/description", true, false, false);
        }

        #[test]
        fn optional() {
            run_test("type_struct/field/optional", true, false, false);
        }

        #[test]
        fn macros() {
            run_test("type_struct/field/macros", true, false, false);
        }
    }
}
