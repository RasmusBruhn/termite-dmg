use indoc::formatdoc;

use crate::*;

/// Generates the Dart source code for an array
///
/// # Parameters
///
/// data: The array to generate Dart source code for
///
/// name: The name of the array type
///
/// indent: The number of spaces per indentation level
pub(super) fn generate(data: &Array, name: &str, indent: usize) -> String {
    return formatdoc!("
        class {name} {{
        {0:indent$}List<{data_type}> values;

        {0:indent$}{name}(this.values);

        {0:indent$}/// Constructs a [{name}] from a [termite.Node].
        {0:indent$}static termite.Result<{name}> fromNode(termite.Node node) {{
        {0:indent$}{0:indent$}return TermiteExtension{name}.fromNode(node);
        {0:indent$}}}

        {0:indent$}/// Converts the [{name}] to a [termite.Node].
        {0:indent$}termite.Node toNode() {{
        {0:indent$}{0:indent$}final list = values.map((element) => element.toNode()).toList();
        {0:indent$}{0:indent$}return termite.Node.sequence(list);
        {0:indent$}}}

        {0:indent$}@override
        {0:indent$}String toString() => '$values';

        {0:indent$}@override
        {0:indent$}bool operator ==(Object other) {{
        {0:indent$}{0:indent$}return other is {name} && ListEquality().equals(other.values, values);
        {0:indent$}}}

        {0:indent$}@override
        {0:indent$}int get hashCode => ListEquality().hash(values);
        }}

        extension TermiteExtension{name} on {name} {{
        {0:indent$}/// Constructs a [{name}] from a [termite.Node].
        {0:indent$}static termite.Result<{name}> fromNode(termite.Node node) {{
        {0:indent$}{0:indent$}if (node is! termite.Sequence) {{
        {0:indent$}{0:indent$}{0:indent$}return termite.Result.error('Unable to parse ${{node.runtimeType}} as a {name}', \"\");
        {0:indent$}{0:indent$}}}

        {0:indent$}{0:indent$}final values = node.values
        {0:indent$}{0:indent$}{0:indent$}.map((node) => TermiteExtension{data_type}.fromNode(node))
        {0:indent$}{0:indent$}{0:indent$}.indexed
        {0:indent$}{0:indent$}{0:indent$}// ignore: prefer_const_constructors
        {0:indent$}{0:indent$}{0:indent$}.fold(termite.Result<List<{data_type}>>.ok([]), (acc, result) {{
        {0:indent$}{0:indent$}{0:indent$}{0:indent$}if (!acc.isOk()) return acc;
        {0:indent$}{0:indent$}{0:indent$}{0:indent$}if (!result.$2.isOk()) {{
        {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}return result.$2.asError().addIndex('${{result.$1}}').asNewError<List<{data_type}>>();
        {0:indent$}{0:indent$}{0:indent$}{0:indent$}}}
        {0:indent$}{0:indent$}{0:indent$}{0:indent$}final list = acc.asOk().value;
        {0:indent$}{0:indent$}{0:indent$}{0:indent$}list.add(result.$2.asOk().value);
        {0:indent$}{0:indent$}{0:indent$}{0:indent$}return termite.Result.ok(list);
        {0:indent$}{0:indent$}{0:indent$}}});
        {0:indent$}{0:indent$}if (!values.isOk()) {{
        {0:indent$}{0:indent$}{0:indent$}return values.asError().asNewError<{name}>();
        {0:indent$}{0:indent$}}}
        {0:indent$}{0:indent$}return termite.Result<{name}>.ok({name}(values.asOk().value));
        {0:indent$}}}
        }}",
        "",
        data_type = &data.data_type,
    );
}

#[cfg(test)]
mod tests {
    use crate::dart::test_utils::*;

    #[test]
    fn basic() {
        run_test("type_array/basic", true, false, false);
    }
}
