use indoc::formatdoc;

use crate::*;

/// Generates the Dart source code for a enum type
///
/// # Parameters
///
/// data: The enum to generate Dart source code for
///
/// name: The name of the enum type
///
/// indent: The number of spaces per indentation level
pub(super) fn generate(data: &Enum, name: &str, indent: usize) -> String {
    let constructors = data
        .types
        .iter()
        .map(|enum_type| enum_type::get_constructor(enum_type, name, indent))
        .collect::<Vec<_>>()
        .join(&format!("\n\n{0:indent$}", ""));

    let enum_types = data
        .types
        .iter()
        .map(|enum_type| enum_type::generate(enum_type, name, indent))
        .collect::<Vec<_>>()
        .join("\n\n");

    let parsers = data
        .types
        .iter()
        .map(|enum_type| enum_type::get_parser(enum_type, name, indent))
        .collect::<Vec<_>>()
        .join(&format!("\n{0:indent$}{0:indent$}{0:indent$}", ""));

    let parsers_object = data
        .types
        .iter()
        .map(|enum_type| enum_type::get_parser_object(enum_type, name, indent))
        .collect::<Vec<_>>()
        .join(&format!("\n{0:indent$}{0:indent$}{0:indent$}", ""));

    return formatdoc!("
        sealed class {name} {{
        {0:indent$}{name}();

        {0:indent$}{constructors}

        {0:indent$}/// Constructs a [{name}] from a [Object].
        {0:indent$}static termite.Result<{name}> fromObject(Object obj) {{
        {0:indent$}{0:indent$}return TermiteExtension{name}.fromObject(obj);
        {0:indent$}}}

        {0:indent$}/// Constructs a [{name}] from a [termite.Node].
        {0:indent$}static termite.Result<{name}> fromNode(termite.Node node) {{
        {0:indent$}{0:indent$}return TermiteExtension{name}.fromNode(node);
        {0:indent$}}}

        {0:indent$}/// Converts the [{name}] to a [termite.Node]
        {0:indent$}termite.Node toNode();
        }}

        {enum_types}

        extension TermiteExtension{name} on {name} {{
        {0:indent$}/// Constructs a [{name}] from a [Object].
        {0:indent$}static termite.Result<{name}> fromObject(Object obj) {{
        {0:indent$}{0:indent$}String id;
        {0:indent$}{0:indent$}if (obj is Map) {{
        {0:indent$}{0:indent$}{0:indent$}if (obj.length != 1) {{
        {0:indent$}{0:indent$}{0:indent$}{0:indent$}return const termite.Result.error('Unable to parse a Map with more or less than 1 entry as a {name}', '');
        {0:indent$}{0:indent$}{0:indent$}}}
        {0:indent$}{0:indent$}{0:indent$}if (obj.keys.first is! String) {{
        {0:indent$}{0:indent$}{0:indent$}{0:indent$}return const termite.Result.error('Unable to parse a Map with a non-String key as a {name}', '');
        {0:indent$}{0:indent$}{0:indent$}}}
        {0:indent$}{0:indent$}{0:indent$}id = obj.keys.first;
        {0:indent$}{0:indent$}}} else if (obj is String) {{
        {0:indent$}{0:indent$}{0:indent$}id = obj;
        {0:indent$}{0:indent$}}} else {{
        {0:indent$}{0:indent$}{0:indent$}return termite.Result.error('Unable to parse ${{obj.runtimeType}} as a {name}', '');
        {0:indent$}{0:indent$}}}

        {0:indent$}{0:indent$}switch (id) {{
        {0:indent$}{0:indent$}{0:indent$}{parsers_object}
        {0:indent$}{0:indent$}{0:indent$}default:
        {0:indent$}{0:indent$}{0:indent$}{0:indent$}return termite.Result.error('Unknown type ($id) for {name}', '');
        {0:indent$}{0:indent$}}}
        {0:indent$}}}

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

mod enum_type {
    use super::*;

    /// Generates the Dart source code for an enum type
    ///
    /// # Parameters
    ///
    /// data: The enum type to generate Dart source code for
    ///
    /// enum_name: The name of the parent enum
    ///
    /// indent: The number of spaces per indentation level
    pub(super) fn generate(data: &EnumType, enum_name: &str, indent: usize) -> String {
        let description = get_description(data);

        return if let Some(data_type) = data.data_type.as_ref() {
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

                {0:indent$}@override
                {0:indent$}bool operator ==(Object other) {{
                {0:indent$}{0:indent$}return other is {enum_name}Type{name} && other.value == value;
                {0:indent$}}}

                {0:indent$}@override
                {0:indent$}int get hashCode => value.hashCode;
                }}",
                "",
                name = data.name,
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

                {0:indent$}@override
                {0:indent$}bool operator ==(Object other) {{
                {0:indent$}{0:indent$}return other is {enum_name}Type{name};
                {0:indent$}}}

                {0:indent$}@override
                {0:indent$}int get hashCode => 0;
                }}",
                "",
                name = data.name,
            )
        };
    }

    /// Generates the Dart doc comment for an enum type
    ///
    /// # Parameters
    ///
    /// data: The enum type to generate the doc comment for
    pub(super) fn get_description(data: &EnumType) -> String {
        return if let Some(description) = data.description.as_ref() {
            format!("/// {description}\n")
        } else {
            "".to_string()
        };
    }

    /// Generates the Dart constructor for an enum type
    ///
    /// # Parameters
    ///
    /// data: The enum type to generate the constructor for
    ///
    /// enum_name: The name of the parent enum
    ///
    /// indent: The number of spaces per indentation level
    pub(super) fn get_constructor(data: &EnumType, enum_name: &str, indent: usize) -> String {
        return if let Some(data_type) = data.data_type.as_ref() {
            formatdoc!(
                "
                /// Constructs a new [{enum_name}] of type {name} with a value of [value].
                {0:indent$}factory {enum_name}.new{name}({data_type} value) = {enum_name}Type{name}._;",
                "",
                name = data.name,
            )
        } else {
            formatdoc!(
                "
                /// Constructs a new [{enum_name}] of type {name}.
                {0:indent$}factory {enum_name}.new{name}() = {enum_name}Type{name}._;",
                "",
                name = data.name,
            )
        };
    }

    /// Generates the Dart serialization parser for an enum type
    ///
    /// # Parameters
    ///
    /// data: The enum type to generate the parser for
    ///
    /// enum_name: The name of the parent enum
    ///
    /// indent: The number of spaces per indentation level
    pub(super) fn get_parser(data: &EnumType, enum_name: &str, indent: usize) -> String {
        return if let Some(data_type) = data.data_type.as_ref() {
            formatdoc!(
                "
                case '{name}':
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}if (node is termite.Mapping) {{
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}final result = TermiteExtension{data_type}.fromNode(node.map[id]!);
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}if (result.isOk()) {{
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}return result.asOk().asNewOk((value) => {enum_name}.new{name}(value));
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}}}
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}return result.asError().addField('{name}').asNewError();
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}}}
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}return const termite.Result.error('{enum_name} type has data and cannot be constructed from a Value', '.{name}');",
                "",
                name = data.name,
            )
        } else {
            formatdoc!(
                "
                case '{name}':
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}if (node is termite.Value) {{
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}return termite.Result.ok({enum_name}.new{name}());
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}}}
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}return const termite.Result.error('{enum_name} type has no data and cannot be constructed from a Mapping', '.{name}');",
                "",
                name = data.name,
            )
        };
    }

    /// Generates the Dart serialization object parser for an enum type
    ///
    /// # Parameters
    ///
    /// data: The enum type to generate the parser for
    ///
    /// enum_name: The name of the parent enum
    ///
    /// indent: The number of spaces per indentation level
    pub(super) fn get_parser_object(data: &EnumType, enum_name: &str, indent: usize) -> String {
        return if let Some(data_type) = data.data_type.as_ref() {
            formatdoc!(
                "
                case '{name}':
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}if (obj is Map) {{
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}final result = TermiteExtension{data_type}.fromObject(obj[id]!);
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}if (result.isOk()) {{
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}return result.asOk().asNewOk((value) => {enum_name}.new{name}(value));
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}}}
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}return result.asError().addField('{name}').asNewError();
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}}}
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}return const termite.Result.error('{enum_name} type has data and cannot be constructed from a Value', '.{name}');",
                "",
                name = data.name,
            )
        } else {
            formatdoc!(
                "
                case '{name}':
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}if (obj is String) {{
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}return termite.Result.ok({enum_name}.new{name}());
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}}}
                {0:indent$}{0:indent$}{0:indent$}{0:indent$}return const termite.Result.error('{enum_name} type has no data and cannot be constructed from a Map', '.{name}');",
                "",
                name = data.name,
            )
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::dart::test_utils::*;

    #[test]
    fn basic() {
        run_test("type_enum/basic", true, false, false);
    }
}
