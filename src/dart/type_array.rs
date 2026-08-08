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
        {0:indent$}{0:indent$}return TermiteNodeParser{name}.fromNode(node);
        {0:indent$}}}

        {0:indent$}/// Converts the [{name}] to a [termite.Node].
        {0:indent$}termite.Node toNode() {{
        {0:indent$}{0:indent$}final list = values.map((element) => element.toNode()).toList();
        {0:indent$}{0:indent$}return termite.Node.sequence(list);
        {0:indent$}}}

        {0:indent$}@override
        {0:indent$}String toString() => '$values';
        }}

        extension TermiteNodeParser{name} on {name} {{
        {0:indent$}/// Constructs a [{name}] from a [termite.Node].
        {0:indent$}static termite.Result<{name}> fromNode(termite.Node node) {{
        {0:indent$}{0:indent$}if (node is! termite.Sequence) {{
        {0:indent$}{0:indent$}{0:indent$}return termite.Result.error('Unable to parse ${{node.runtimeType}} as a {name}', \"\");
        {0:indent$}{0:indent$}}}

        {0:indent$}{0:indent$}termite.Result<List<{data_type}>> values = node.values
        {0:indent$}{0:indent$}{0:indent$}.map((node) => TermiteNodeParser{data_type}.fromNode(node))
        {0:indent$}{0:indent$}{0:indent$}.indexed
        {0:indent$}{0:indent$}{0:indent$}// ignore: prefer_const_constructors
        {0:indent$}{0:indent$}{0:indent$}.fold(termite.Result.ok([]), (acc, result) {{
        {0:indent$}{0:indent$}{0:indent$}{0:indent$}if (acc is termite.Error) return acc;
        {0:indent$}{0:indent$}{0:indent$}{0:indent$}if (result.$2 is termite.Error) {{
        {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}final newError = (result.$2 as termite.Error).addIndex('${{result.$1}}');
        {0:indent$}{0:indent$}{0:indent$}{0:indent$}{0:indent$}return termite.Result.error(newError.error, newError.location);
        {0:indent$}{0:indent$}{0:indent$}{0:indent$}}}
        {0:indent$}{0:indent$}{0:indent$}{0:indent$}List<{data_type}> list = (acc as termite.Ok<List<{data_type}>>).value;
        {0:indent$}{0:indent$}{0:indent$}{0:indent$}list.add((result.$2 as termite.Ok<{data_type}>).value);
        {0:indent$}{0:indent$}{0:indent$}{0:indent$}return termite.Result.ok(list);
        {0:indent$}{0:indent$}{0:indent$}}});
        {0:indent$}{0:indent$}if (values is termite.Error<List<{data_type}>>) {{
        {0:indent$}{0:indent$}{0:indent$}return termite.Result.error(values.error, values.location);
        {0:indent$}{0:indent$}}}
        {0:indent$}{0:indent$}return termite.Result.ok({name}((values as termite.Ok<List<{data_type}>>).value));
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
        run_test("type_array/basic", true, false);
    }
}
