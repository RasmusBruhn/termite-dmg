use indoc::formatdoc;

use crate::*;

/// Generates the Dart source code for a variant type
///
/// # Parameters
///
/// data: The variant to generate code for
///
/// name: The name of the variant type
///
/// indent: The number of spaces per indentation level
pub(super) fn generate(data: &Variant, name: &str, indent: usize) -> String {
    let variant_types = data
        .data_types
        .iter()
        .map(|variant_type| {
            formatdoc!(
                "
                class {name}Type{variant_type} extends {name} {{
                {0:indent$}/// The stored variant value of type [{variant_type}].
                {0:indent$}{variant_type} value;

                {0:indent$}{name}Type{variant_type}._(this.value);

                {0:indent$}@override
                {0:indent$}termite.Node toNode() {{
                {0:indent$}{0:indent$}return value.toNode();
                {0:indent$}}}

                {0:indent$}@override
                {0:indent$}String toString() => '$value';
                }}",
                ""
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    let constructors = data
        .data_types
        .iter()
        .map(|variant_type| {
            formatdoc!(
                "
                /// Constructs a new [{name}] of type [{variant_type}] with a value of [value].
                {0:indent$}factory {name}.new{variant_type}({variant_type} value) = {name}Type{variant_type}._;",
                ""
            )
        })
        .collect::<Vec<_>>()
        .join(&format!("\n\n{0:indent$}", ""));

    let parsers = data
        .data_types
        .iter()
        .map(|variant_type| {
            formatdoc!(
                "
                termite.Result<{variant_type}> __{variant_type} = TermiteNodeParser{variant_type}.fromNode(node);
                {0:indent$}{0:indent$}if (__{variant_type} is termite.Ok<{variant_type}>) {{
                {0:indent$}{0:indent$}{0:indent$}return termite.Result.ok({name}.new{variant_type}(__{variant_type}.value));
                {0:indent$}{0:indent$}}}
                {0:indent$}{0:indent$}__{variant_type} = (__{variant_type} as termite.Error<{variant_type}>).addField('{variant_type}');",
                ""
            )
        })
        .collect::<Vec<_>>()
        .join(&format!("\n\n{0:indent$}{0:indent$}", ""));

    let parser_errors = data
        .data_types
        .iter()
        .map(|variant_type| format!("{{${{__{variant_type}.getMessage()}}}}"))
        .collect::<Vec<_>>()
        .join(", ");

    return formatdoc!(
        "
        sealed class {name} {{
        {0:indent$}{name}();

        {0:indent$}{constructors}

        {0:indent$}/// Constructs a [{name}] from a [termite.Node].
        {0:indent$}static termite.Result<{name}> fromNode(termite.Node node) {{
        {0:indent$}{0:indent$}return TermiteNodeParser{name}.fromNode(node);
        {0:indent$}}}

        {0:indent$}/// Converts the [{name}] to a [termite.Node].
        {0:indent$}termite.Node toNode();
        }}

        {variant_types}

        extension TermiteNodeParser{name} on {name} {{
        {0:indent$}/// Constructs a [{name}] from a [termite.Node].
        {0:indent$}static termite.Result<{name}> fromNode(termite.Node node) {{
        {0:indent$}{0:indent$}{parsers}

        {0:indent$}{0:indent$}return termite.Result.error('{parser_errors}', '');
        {0:indent$}}}
        }}",
        "",
    );
}

#[cfg(test)]
mod tests {
    use crate::dart::test_utils::*;

    #[test]
    fn basic() {
        run_test("type_variant/basic", true, false);
    }
}
