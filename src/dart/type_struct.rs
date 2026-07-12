use std::collections::{HashMap, HashSet};

use indoc::formatdoc;

use crate::data_model;

impl data_model::Struct {
    /// Generates the Dart source code for a struct type
    ///
    /// # Parameters
    ///
    /// name: The name of the struct type
    ///
    /// indent: The number of spaces per indentation level
    ///
    /// macros: The macros defined in the data model used for expanding default values
    pub(super) fn get_dart<'a>(
        &self,
        name: &str,
        indent: usize,
        macros: &'a HashMap<String, data_model::SerializationModel>,
    ) -> Result<String, data_model::Error> {
        let definitions = self
            .fields
            .iter()
            .map(|field| field.get_definition(indent))
            .collect::<Vec<_>>()
            .join(&format!("\n\n{0:indent$}", ""));

        let constructor_parameters = self
            .fields
            .iter()
            .map(|field| field.get_constructor_parameter())
            .collect::<Vec<_>>()
            .join(&format!("\n{0:indent$}{0:indent$}", ""));

        let constructor = self
            .fields
            .iter()
            .filter_map(|field| field.get_constructor())
            .collect::<Vec<_>>()
            .join(&format!("\n{0:indent$}{0:indent$}", ""));

        let default_constructors = self
            .fields
            .iter()
            .filter_map(|field| field.get_default_constructor(indent, macros))
            .collect::<Result<Vec<_>, _>>()?
            .join(&format!("\n\n{0:indent$}", ""));

        let exports = self
            .fields
            .iter()
            .map(|field| field.get_export())
            .collect::<Vec<_>>()
            .join(&format!("\n{0:indent$}{0:indent$}{0:indent$}", ""));

        let printers = self
            .fields
            .iter()
            .map(|field| field.get_printer())
            .collect::<Vec<_>>()
            .join(", ");

        let parsers = self
            .fields
            .iter()
            .map(|field| field.get_parser(name, indent))
            .collect::<Vec<_>>()
            .join(&format!("\n\n{0:indent$}{0:indent$}", ""));

        let parser_returns = self
            .fields
            .iter()
            .map(|field| field.get_parser_return())
            .collect::<Vec<_>>()
            .join(&format!(
                "\n{0:indent$}{0:indent$}{0:indent$}{0:indent$}",
                ""
            ));

        return Ok(formatdoc!("
            class {name} {{
            {0:indent$}{definitions}

            {0:indent$}{name}({{
            {0:indent$}{0:indent$}{constructor_parameters}
            {0:indent$}}}) {{
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
}

impl data_model::StructField {
    /// Generates the Dart source code for a struct field definition
    ///
    /// # Parameters
    ///
    /// indent: The number of spaces per indentation level
    fn get_definition(&self, indent: usize) -> String {
        let type_name = match &self.default {
            data_model::DefaultType::Required => {
                format!("{data_type}", data_type = &self.data_type)
            }
            data_model::DefaultType::Optional => {
                format!("{data_type}?", data_type = &self.data_type)
            }
            data_model::DefaultType::Default(_) => {
                format!("late {data_type}", data_type = &self.data_type)
            }
        };

        let description = match &self.description {
            Some(description) => format!("/// {description}\n{0:indent$}", ""),
            None => "".to_string(),
        };

        return format!("{description}{type_name} {name};", name = &self.name);
    }

    /// Generates the Dart source code for the constructor parameter for a single field in a struct
    fn get_constructor_parameter(&self) -> String {
        return match &self.default {
            data_model::DefaultType::Required => {
                format!("required this.{name},", name = &self.name)
            }
            data_model::DefaultType::Optional => {
                format!("this.{name},", name = &self.name)
            }
            data_model::DefaultType::Default(_) => {
                format!(
                    "{data_type}? {name},",
                    data_type = &self.data_type,
                    name = &self.name
                )
            }
        };
    }

    /// Generates the Dart source code for the constructor assignment for a single field in a struct
    fn get_constructor(&self) -> Option<String> {
        return if let data_model::DefaultType::Default(_) = &self.default {
            Some(format!(
                "this.{name} = {name} ?? getDefault{capital_name}();",
                name = &self.name,
                capital_name = self.get_capitalized_name(),
            ))
        } else {
            None
        };
    }

    /// Generates the Dart source code for the default constructor for a single field in a struct
    ///
    /// # Parameters
    ///
    /// indent: The number of spaces per indentation level
    ///
    /// macros: The macros defined in the data model used for expanding default values
    fn get_default_constructor<'a>(
        &self,
        indent: usize,
        macros: &'a HashMap<String, data_model::SerializationModel>,
    ) -> Option<Result<String, data_model::Error>> {
        return if let data_model::DefaultType::Default(default_value) = &self.default {
            let expanded_default =
                match data_model::expand_macros(default_value, macros, &mut HashSet::new()) {
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
                name = &self.name,
                capital_name = &self.get_capitalized_name(),
                data_type = &self.data_type,
                node = &expanded_default.get_dart(indent, 2 * indent),
            )))
        } else {
            None
        };
    }

    /// Gets the name of the field with the first letter capitalized
    fn get_capitalized_name(&self) -> String {
        let mut name = self.name.clone();
        if let Some(first_char) = name.get_mut(0..1) {
            first_char.make_ascii_uppercase();
        };
        return name;
    }

    /// Generates the Dart source code for the node export of a single field in a struct
    fn get_export(&self) -> String {
        return if let data_model::DefaultType::Optional = &self.default {
            format!("'{name}': {name}?.toNode(),", name = &self.name)
        } else {
            format!("'{name}': {name}.toNode(),", name = &self.name)
        };
    }

    /// Generates the Dart source code for the printing code for a single field in a struct
    fn get_printer(&self) -> String {
        return format!("{name}: ${name}", name = &self.name);
    }

    /// Generates the Dart source code for the parser code for a single field in a struct
    ///
    /// # Parameters
    ///
    /// struct_name: The name of the struct containing the field
    ///
    /// indent: The number of spaces per indentation level
    fn get_parser(&self, struct_name: &str, indent: usize) -> String {
        return match &self.default {
            data_model::DefaultType::Required => {
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
                    name = &self.name,
                    data_type = &self.data_type,
                )
            }
            data_model::DefaultType::Optional => {
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
                    name = &self.name,
                    data_type = &self.data_type,
                )
            }
            data_model::DefaultType::Default(_) => {
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
                    name = &self.name,
                    capital_name = &self.get_capitalized_name(),
                    data_type = &self.data_type,
                )
            }
        };
    }

    /// Generates the Dart source code for the parser return of a single field in a struct
    fn get_parser_return(&self) -> String {
        return format!("{name}: {name},", name = &self.name);
    }
}

#[cfg(test)]
mod tests {
    use crate::dart::test_utils::*;

    #[test]
    fn basic() {
        run_test("type_struct/basic", true, false);
    }

    #[test]
    fn description() {
        run_test("type_struct/description", true, false);
    }

    mod field {
        use super::*;

        #[test]
        fn basic() {
            run_test("type_struct/field/basic", true, false);
        }

        #[test]
        fn description() {
            run_test("type_struct/field/description", true, false);
        }

        #[test]
        fn optional() {
            run_test("type_struct/field/optional", true, false);
        }

        #[test]
        fn macros() {
            run_test("type_struct/field/macros", true, false);
        }
    }
}

impl data_model::SerializationModel {
    /// Generates the Dart source code for a serialization model in a default value
    ///
    /// # Parameters
    ///
    /// indent: The number of spaces per indentation level
    ///
    /// base_indent: The base indentation level for the generated code
    fn get_dart(&self, indent: usize, base_indent: usize) -> String {
        return match self {
            data_model::SerializationModel::Map(map) => {
                let items = map
                    .iter()
                    .map(|(key, item)| {
                        format!(
                            "{0:base_indent$}{0:indent$}'{key}': {value},",
                            "",
                            value = item.get_dart(indent, base_indent + indent),
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                format!("termite.Node.mapping({{\n{items}\n{0:base_indent$}}})", "")
            }
            data_model::SerializationModel::Array(list) => {
                let items = list
                    .iter()
                    .map(|item| {
                        format!(
                            "{0:base_indent$}{0:indent$}{value},",
                            "",
                            value = item.get_dart(indent, base_indent + indent)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                format!("termite.Node.sequence([\n{items}\n{0:base_indent$}])", "")
            }
            data_model::SerializationModel::Value(value) => {
                format!("termite.Node.value('{value}')")
            }
        };
    }
}
