use std::collections::{HashMap, HashSet};

use indoc::formatdoc;

use crate::*;

/// Generates the Dart source code for a struct type
///
/// # Parameters
///
/// data: The struct to generate code for
///
/// name: The name of the struct type
///
/// indent: The number of spaces per indentation level
///
/// macros: The macros defined in the data model used for expanding default values
pub(super) fn generate<'a>(
    data: &Struct,
    name: &str,
    indent: usize,
    macros: &'a HashMap<String, SerializationModel>,
) -> Result<String, Error> {
    let definitions = data
        .fields
        .iter()
        .map(|field| struct_field::get_definition(field, indent))
        .collect::<Vec<_>>()
        .join(&format!("\n\n{0:indent$}", ""));

    let mut constructor_parameters = data
        .fields
        .iter()
        .map(|field| struct_field::get_constructor_parameter(field))
        .collect::<Vec<_>>()
        .join(&format!("\n{0:indent$}{0:indent$}", ""));
    if !constructor_parameters.is_empty() {
        constructor_parameters = format!(
            "{{\n{0:indent$}{0:indent$}{constructor_parameters}\n{0:indent$}}}",
            ""
        );
    }

    let constructor = data
        .fields
        .iter()
        .filter_map(|field| struct_field::get_constructor(field))
        .collect::<Vec<_>>()
        .join(&format!("\n{0:indent$}{0:indent$}", ""));

    let default_constructors = data
        .fields
        .iter()
        .filter_map(|field| struct_field::get_default_constructor(field, indent, macros))
        .collect::<Result<Vec<_>, _>>()?
        .join(&format!("\n\n{0:indent$}", ""));

    let exports = data
        .fields
        .iter()
        .map(|field| struct_field::get_export(field))
        .collect::<Vec<_>>()
        .join(&format!("\n{0:indent$}{0:indent$}{0:indent$}", ""));

    let printers = data
        .fields
        .iter()
        .map(|field| struct_field::get_printer(field))
        .collect::<Vec<_>>()
        .join(", ");

    let parsers = data
        .fields
        .iter()
        .map(|field| struct_field::get_parser(field, name, indent))
        .collect::<Vec<_>>()
        .join(&format!("\n\n{0:indent$}{0:indent$}", ""));

    let parser_returns = data
        .fields
        .iter()
        .map(|field| struct_field::get_parser_return(field))
        .collect::<Vec<_>>()
        .join(&format!(
            "\n{0:indent$}{0:indent$}{0:indent$}{0:indent$}",
            ""
        ));

    let equality = data
        .fields
        .iter()
        .map(|field| format!("other.{name} == {name}", name = &field.name))
        .collect::<Vec<_>>()
        .join(" && ");
    let equality = if equality.is_empty() {
        format!("true")
    } else {
        equality
    };

    let hash_list = construct_hash(
        &data
            .fields
            .iter()
            .map(|field| format!("{name}", name = &field.name))
            .collect::<Vec<_>>(),
    );

    return Ok(formatdoc!("
        class {name} {{
        {0:indent$}{definitions}

        {0:indent$}{name}({constructor_parameters}) {{
        {0:indent$}{0:indent$}{constructor}
        {0:indent$}}}

        {0:indent$}{default_constructors}

        {0:indent$}/// Constructs a [{name}] from a [termite.Node]
        {0:indent$}static termite.Result<{name}> fromNode(termite.Node node) {{
        {0:indent$}{0:indent$}return TermiteNodeParser{name}.fromNode(node);
        {0:indent$}}}

        {0:indent$}/// Converts the [{name}] to a [termite.Node]
        {0:indent$}termite.Node toNode() {{
        {0:indent$}{0:indent$}final Map<String, termite.Node?> __preMap = {{
        {0:indent$}{0:indent$}{0:indent$}{exports}
        {0:indent$}{0:indent$}}};
        {0:indent$}{0:indent$}final Map<String, termite.Node> map = Map.fromEntries(
        {0:indent$}{0:indent$}{0:indent$}__preMap.entries
        {0:indent$}{0:indent$}{0:indent$}{0:indent$}.where((entry) => entry.value != null)
        {0:indent$}{0:indent$}{0:indent$}{0:indent$}.map((entry) => MapEntry(entry.key, entry.value!)),
        {0:indent$}{0:indent$});
        {0:indent$}{0:indent$}return termite.Node.mapping(map);
        {0:indent$}}}

        {0:indent$}@override
        {0:indent$}String toString() => '{{{printers}}}';

        {0:indent$}@override
        {0:indent$}bool operator ==(Object other) {{
        {0:indent$}{0:indent$}return other is {name} && {equality};
        {0:indent$}}}

        {0:indent$}@override
        {0:indent$}int get hashCode => {hash_list};
        }}

        extension TermiteNodeParser{name} on {name} {{
        {0:indent$}/// Constructs a [{name}] from a [termite.Node]
        {0:indent$}static termite.Result<{name}> fromNode(termite.Node node) {{
        {0:indent$}{0:indent$}if (node is! termite.Mapping) {{
        {0:indent$}{0:indent$}{0:indent$}return termite.Result.error('Unable to parse ${{node.runtimeType}} as a {name}', '');
        {0:indent$}{0:indent$}}}

        {0:indent$}{0:indent$}{parsers}

        {0:indent$}{0:indent$}return termite.Result.ok(
        {0:indent$}{0:indent$}{0:indent$}{name}(
        {0:indent$}{0:indent$}{0:indent$}{0:indent$}{parser_returns}
        {0:indent$}{0:indent$}{0:indent$}),
        {0:indent$}{0:indent$});
        {0:indent$}}}
        }}",
        "",
    ));
}

fn construct_hash(input_list: &[String]) -> String {
    let hash_list = input_list
        .chunks(20)
        .map(|chunk| {
            if chunk.len() == 1 {
                format!("{}.hashCode", chunk[0])
            } else {
                format!("Object.hash({})", chunk.join(", "))
            }
        })
        .collect::<Vec<_>>();

    return if hash_list.is_empty() {
        format!("0")
    } else if hash_list.len() == 1 {
        hash_list.first().unwrap().clone()
    } else {
        construct_hash(&hash_list)
    };
}

mod struct_field {
    use super::*;

    /// Generates the Dart source code for a struct field definition
    ///
    /// # Parameters
    ///
    /// data: The struct field to generate code for
    ///
    /// indent: The number of spaces per indentation level
    pub(super) fn get_definition(data: &StructField, indent: usize) -> String {
        let type_name = match &data.default {
            DefaultType::Required => {
                format!("{data_type}", data_type = &data.data_type)
            }
            DefaultType::Optional => {
                format!("{data_type}?", data_type = &data.data_type)
            }
            DefaultType::Default(_) => {
                format!("late {data_type}", data_type = &data.data_type)
            }
        };

        let description = match &data.description {
            Some(description) => format!("/// {description}\n{0:indent$}", ""),
            None => "".to_string(),
        };

        return format!("{description}{type_name} {name};", name = &data.name);
    }

    /// Generates the Dart source code for the constructor parameter for a single field in a struct
    ///
    /// # Parameters
    ///
    /// data: The struct field to generate code for
    pub(super) fn get_constructor_parameter(data: &StructField) -> String {
        return match &data.default {
            DefaultType::Required => {
                format!("required this.{name},", name = &data.name)
            }
            DefaultType::Optional => {
                format!("this.{name},", name = &data.name)
            }
            DefaultType::Default(_) => {
                format!(
                    "{data_type}? {name},",
                    data_type = &data.data_type,
                    name = &data.name
                )
            }
        };
    }

    /// Generates the Dart source code for the constructor assignment for a single field in a struct
    ///
    /// # Parameters
    ///
    /// data: The struct field to generate code for
    pub(super) fn get_constructor(data: &StructField) -> Option<String> {
        return if let DefaultType::Default(_) = &data.default {
            Some(format!(
                "this.{name} = {name} ?? getDefault{capital_name}();",
                name = &data.name,
                capital_name = get_capitalized_name(data),
            ))
        } else {
            None
        };
    }

    /// Generates the Dart source code for the default constructor for a single field in a struct
    ///
    /// # Parameters
    ///
    /// data: The struct field to generate code for
    ///
    /// indent: The number of spaces per indentation level
    ///
    /// macros: The macros defined in the data model used for expanding default values
    pub(super) fn get_default_constructor<'a>(
        data: &StructField,
        indent: usize,
        macros: &'a HashMap<String, SerializationModel>,
    ) -> Option<Result<String, Error>> {
        return if let DefaultType::Default(default_value) = &data.default {
            let expanded_default = match expand_macros(default_value, macros, &mut HashSet::new()) {
                Ok(value) => value,
                Err(e) => return Some(Err(e)),
            };

            Some(Ok(formatdoc!("
                /// Gets the default value for [{name}]
                {0:indent$}static {data_type} getDefault{capital_name}() {{
                {0:indent$}{0:indent$}final node = {node};
                {0:indent$}{0:indent$}return (TermiteNodeParser{data_type}.fromNode(node) as termite.Ok<{data_type}>).value;
                {0:indent$}}}",
                "",
                name = &data.name,
                capital_name = &get_capitalized_name(data),
                data_type = &data.data_type,
                node = &serialization_model::generate(&expanded_default, indent, 2 * indent),
            )))
        } else {
            None
        };
    }

    /// Gets the name of the field with the first letter capitalized
    ///
    /// # Parameters
    ///
    /// data: The struct field to generate code for
    pub(super) fn get_capitalized_name(data: &StructField) -> String {
        let mut name = data.name.clone();
        if let Some(first_char) = name.get_mut(0..1) {
            first_char.make_ascii_uppercase();
        };
        return name;
    }

    /// Generates the Dart source code for the node export of a single field in a struct
    ///
    /// # Parameters
    ///
    /// data: The struct field to generate code for
    pub(super) fn get_export(data: &StructField) -> String {
        return if let DefaultType::Optional = &data.default {
            format!("'{name}': {name}?.toNode(),", name = &data.name)
        } else {
            format!("'{name}': {name}.toNode(),", name = &data.name)
        };
    }

    /// Generates the Dart source code for the printing code for a single field in a struct
    ///
    /// # Parameters
    ///
    /// data: The struct field to generate code for
    pub(super) fn get_printer(data: &StructField) -> String {
        return format!("{name}: ${name}", name = &data.name);
    }

    /// Generates the Dart source code for the parser code for a single field in a struct
    ///
    /// # Parameters
    ///
    /// data: The struct field to generate code for
    ///
    /// struct_name: The name of the struct containing the field
    ///
    /// indent: The number of spaces per indentation level
    pub(super) fn get_parser(data: &StructField, struct_name: &str, indent: usize) -> String {
        return match &data.default {
            DefaultType::Required => {
                formatdoc!("
                    if (!node.map.containsKey('{name}')) {{
                    {0:indent$}{0:indent$}{0:indent$}return const termite.Result.error('Missing field \"{name}\"', '');
                    {0:indent$}{0:indent$}}}
                    {0:indent$}{0:indent$}final termite.Result<{data_type}> __{name} = TermiteNodeParser{data_type}.fromNode(node.map['{name}']!);
                    {0:indent$}{0:indent$}if (__{name} is termite.Error<{data_type}>) {{
                    {0:indent$}{0:indent$}{0:indent$}final newError = __{name}.addField('{name}');
                    {0:indent$}{0:indent$}{0:indent$}return termite.Result.error(newError.error, newError.location);
                    {0:indent$}{0:indent$}}}
                    {0:indent$}{0:indent$}final {data_type} {name} = (__{name} as termite.Ok<{data_type}>).value;",
                    "",
                    name = &data.name,
                    data_type = &data.data_type,
                )
            }
            DefaultType::Optional => {
                formatdoc!("
                    {data_type}? {name};
                    {0:indent$}{0:indent$}if (node.map.containsKey('{name}')) {{
                    {0:indent$}{0:indent$}{0:indent$}final termite.Result<{data_type}> __{name} = TermiteNodeParser{data_type}.fromNode(node.map['{name}']!);
                    {0:indent$}{0:indent$}{0:indent$}if (__{name} is termite.Error<{data_type}>) {{
                    {0:indent$}{0:indent$}{0:indent$}{0:indent$}final newError = __{name}.addField('{name}');
                    {0:indent$}{0:indent$}{0:indent$}{0:indent$}return termite.Result.error(newError.error, newError.location);
                    {0:indent$}{0:indent$}{0:indent$}}}
                    {0:indent$}{0:indent$}{0:indent$}{name} = (__{name} as termite.Ok<{data_type}>).value;
                    {0:indent$}{0:indent$}}}",
                    "",
                    name = &data.name,
                    data_type = &data.data_type,
                )
            }
            DefaultType::Default(_) => {
                formatdoc!("
                    {data_type} {name} = {struct_name}.getDefault{capital_name}();
                    {0:indent$}{0:indent$}if (node.map.containsKey('{name}')) {{
                    {0:indent$}{0:indent$}{0:indent$}final termite.Result<{data_type}> __{name} = TermiteNodeParser{data_type}.fromNode(node.map['{name}']!);
                    {0:indent$}{0:indent$}{0:indent$}if (__{name} is termite.Error<{data_type}>) {{
                    {0:indent$}{0:indent$}{0:indent$}{0:indent$}final newError = __{name}.addField('{name}');
                    {0:indent$}{0:indent$}{0:indent$}{0:indent$}return termite.Result.error(newError.error, newError.location);
                    {0:indent$}{0:indent$}{0:indent$}}}
                    {0:indent$}{0:indent$}{0:indent$}{name} = (__{name} as termite.Ok<{data_type}>).value;
                    {0:indent$}{0:indent$}}}",
                    "",
                    name = &data.name,
                    capital_name = &get_capitalized_name(data),
                    data_type = &data.data_type,
                )
            }
        };
    }

    /// Generates the Dart source code for the parser return of a single field in a struct
    ///
    /// # Parameters
    ///
    /// data: The struct field to generate code for
    pub(super) fn get_parser_return(data: &StructField) -> String {
        return format!("{name}: {name},", name = &data.name);
    }
}

mod serialization_model {
    use super::*;

    /// Generates the Dart source code for a serialization model in a default value
    ///
    /// # Parameters
    ///
    /// data: The serialization model to generate Dart source code for
    ///
    /// indent: The number of spaces per indentation level
    ///
    /// base_indent: The base indentation level for the generated code
    pub(super) fn generate(data: &SerializationModel, indent: usize, base_indent: usize) -> String {
        return match data {
            SerializationModel::Map(map) => {
                let items = map
                    .iter()
                    .map(|(key, item)| {
                        format!(
                            "{0:base_indent$}{0:indent$}'{key}': {value},",
                            "",
                            value =
                                serialization_model::generate(item, indent, base_indent + indent),
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                format!("termite.Node.mapping({{\n{items}\n{0:base_indent$}}})", "")
            }
            SerializationModel::Array(list) => {
                let items = list
                    .iter()
                    .map(|item| {
                        format!(
                            "{0:base_indent$}{0:indent$}{value},",
                            "",
                            value =
                                serialization_model::generate(item, indent, base_indent + indent)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                format!("termite.Node.sequence([\n{items}\n{0:base_indent$}])", "")
            }
            SerializationModel::Value(value) => {
                format!("termite.Node.value('{value}')")
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::dart::test_utils::*;

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
