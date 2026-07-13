use indoc::formatdoc;

use crate::data_model;

impl data_model::Enum {
    /// Generates the Dart source code for a enum type
    ///
    /// # Parameters
    ///
    /// name: The name of the enum type
    ///
    /// indent: The number of spaces per indentation level
    pub(super) fn get_dart(&self, name: &str, indent: usize) -> String {
        let constructors = self
            .types
            .iter()
            .map(|enum_type| enum_type.get_constructor(name, indent))
            .collect::<Vec<_>>()
            .join(&format!("\n\n{0:indent$}", ""));

        let enum_types = self
            .types
            .iter()
            .map(|enum_type| enum_type.get_dart(name, indent))
            .collect::<Vec<_>>()
            .join("\n\n");

        let parsers = self
            .types
            .iter()
            .map(|enum_type| enum_type.get_parser(name, indent))
            .collect::<Vec<_>>()
            .join(&format!("\n{0:indent$}{0:indent$}{0:indent$}", ""));

        return formatdoc!("
            sealed class {name} {{
            {0:indent$}{name}();

            {0:indent$}{constructors}

            {0:indent$}/// Constructs a [{name}] from a [termite.Node].
            {0:indent$}static termite.Result<{name}> fromNode(termite.Node node) {{
            {0:indent$}{0:indent$}return TermiteNodeParser{name}.fromNode(node);
            {0:indent$}}}

            {0:indent$}/// Converts the [{name}] to a [termite.Node]
            {0:indent$}termite.Node toNode();
            }}

            {enum_types}

            extension TermiteNodeParser{name} on {name} {{
            {0:indent$}/// Constructs a [{name}] from a [termite.Node].
            {0:indent$}static termite.Result<{name}> fromNode(termite.Node node) {{
            {0:indent$}{0:indent$}String id;
            {0:indent$}{0:indent$}if (node is termite.Sequence) {{
            {0:indent$}{0:indent$}{0:indent$}return termite.Result.error('Unable to parse ${{node.runtimeType}} as a {name}', '');
            {0:indent$}{0:indent$}}} else if (node is termite.Mapping) {{
            {0:indent$}{0:indent$}{0:indent$}if (node.map.length != 1) {{
            {0:indent$}{0:indent$}{0:indent$}{0:indent$}return const termite.Result.error('Unable to parse a Mapping with more or less than 1 entry as a {name}', '');
            {0:indent$}{0:indent$}{0:indent$}}}
            {0:indent$}{0:indent$}{0:indent$}id = node.map.keys.first;
            {0:indent$}{0:indent$}}} else {{
            {0:indent$}{0:indent$}{0:indent$}id = (node as termite.Value).value;
            {0:indent$}{0:indent$}}}

            {0:indent$}{0:indent$}switch (id) {{
            {0:indent$}{0:indent$}{0:indent$}{parsers}
            {0:indent$}{0:indent$}{0:indent$}default:
            {0:indent$}{0:indent$}{0:indent$}{0:indent$}return termite.Result.error('Unknown type ($id) for {name}', '');
            {0:indent$}{0:indent$}}}
            {0:indent$}}}
            }}",
            "",
        );
    }
}

impl data_model::EnumType {
    /// Generates the Dart source code for an enum type
    ///
    /// # Parameters
    ///
    /// enum_name: The name of the parent enum
    ///
    /// indent: The number of spaces per indentation level
    fn get_dart(&self, enum_name: &str, indent: usize) -> String {
        let description = self.get_description();

        return if let Some(data_type) = self.data_type.as_ref() {
            formatdoc!(
                "
                {description}class {enum_name}Type{name} extends {enum_name} {{
                {0:indent$}{data_type} value;

                {0:indent$}{enum_name}Type{name}._(this.value);

                {0:indent$}@override
                {0:indent$}termite.Node toNode() {{
                {0:indent$}{0:indent$}return termite.Node.mapping({{'{name}': value.toNode()}});
                {0:indent$}}}

                {0:indent$}@override
                {0:indent$}String toString() => '{name}($value)';
                }}",
                "",
                name = self.name,
            )
        } else {
            formatdoc!(
                "
                {description}class {enum_name}Type{name} extends {enum_name} {{
                {0:indent$}{enum_name}Type{name}._();

                {0:indent$}@override
                {0:indent$}termite.Node toNode() {{
                {0:indent$}{0:indent$}return termite.Node.value('{name}');
                {0:indent$}}}

                {0:indent$}@override
                {0:indent$}String toString() => '{name}';
                }}",
                "",
                name = self.name,
            )
        };
    }

    /// Generates the Dart doc comment for an enum type
    fn get_description(&self) -> String {
        return if let Some(description) = self.description.as_ref() {
            format!("/// {description}\n")
        } else {
            "".to_string()
        };
    }

    /// Generates the Dart constructor for an enum type
    ///
    /// # Parameters
    ///
    /// enum_name: The name of the parent enum
    ///
    /// indent: The number of spaces per indentation level
    fn get_constructor(&self, enum_name: &str, indent: usize) -> String {
        return if let Some(data_type) = self.data_type.as_ref() {
            formatdoc!(
                "
                /// Constructs a new [{enum_name}] of type {name} with a value of [value].
                {0:indent$}factory {enum_name}.new{name}({data_type} value) = {enum_name}Type{name}._;",
                "",
                name = self.name,
            )
        } else {
            formatdoc!(
                "
                /// Constructs a new [{enum_name}] of type {name}.
                {0:indent$}factory {enum_name}.new{name}() = {enum_name}Type{name}._;",
                "",
                name = self.name,
            )
        };
    }

    /// Generates the Dart serialization parser for an enum type
    ///
    /// # Parameters
    ///
    /// enum_name: The name of the parent enum
    ///
    /// indent: The number of spaces per indentation level
    fn get_parser(&self, enum_name: &str, indent: usize) -> String {
        return if let Some(data_type) = self.data_type.as_ref() {
            formatdoc!(
                "
                case '{name}':
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}if (node is termite.Mapping) {{
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}final result = TermiteNodeParser{data_type}.fromNode(node.map[id]!);
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}if (result is termite.Ok<{data_type}>) {{
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}return termite.Result.ok({enum_name}.new{name}(result.value));
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}}}
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}final newResult = (result as termite.Error).addField('{name}');
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}return termite.Result.error(newResult.error, newResult.location);
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}}}
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}return const termite.Result.error('{enum_name} type has data and cannot be constructed from a value', '.{name}');",
                "",
                name = self.name,
            )
        } else {
            formatdoc!(
                "
                case '{name}':
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}if (node is termite.Value) {{
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}return termite.Result.ok({enum_name}.new{name}());
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}}}
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}return const termite.Result.error('{enum_name} type has no data and cannot be constructed from a mapping', '.{name}');",
                "",
                name = self.name,
            )
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::dart::test_utils::*;

    #[test]
    fn basic() {
        run_test("type_enum/basic", true, false);
    }
}
