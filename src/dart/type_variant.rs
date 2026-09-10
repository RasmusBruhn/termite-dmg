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
        .map(|variant_type| variant_type::get_implementation(variant_type, name, indent))
        .collect::<Vec<_>>()
        .join("\n\n");

    let constructors = data
        .data_types
        .iter()
        .map(|variant_type| variant_type::get_constructor(variant_type, name, indent))
        .collect::<Vec<_>>()
        .join(&format!("\n\n{0:indent$}", ""));

    let parsers = data
        .data_types
        .iter()
        .map(|variant_type| variant_type::get_parser(variant_type, name, indent))
        .collect::<Vec<_>>()
        .join(&format!("\n\n{0:indent$}{0:indent$}", ""));

    let parsers_object = data
        .data_types
        .iter()
        .map(|variant_type| variant_type::get_parser_object(variant_type, name, indent))
        .collect::<Vec<_>>()
        .join(&format!("\n\n{0:indent$}{0:indent$}", ""));

    let parser_errors = data
        .data_types
        .iter()
        .map(|variant_type| variant_type::get_parser_error(variant_type))
        .collect::<Vec<_>>()
        .join(", ");

    return formatdoc!(
        "
        sealed class {name} {{
        {0:indent$}{name}();

        {0:indent$}{constructors}

        {0:indent$}/// Constructs a [{name}] from a [termite.Node].
        {0:indent$}static termite.Result<{name}> fromNode(termite.Node node) {{
        {0:indent$}{0:indent$}return TermiteExtension{name}.fromNode(node);
        {0:indent$}}}

        {0:indent$}/// Converts the [{name}] to a [termite.Node].
        {0:indent$}termite.Node toNode();
        }}

        {variant_types}

        extension TermiteExtension{name} on {name} {{
        {0:indent$}/// Constructs a [{name}] from a [Object].
        {0:indent$}static termite.Result<{name}> fromObject(Object obj) {{
        {0:indent$}{0:indent$}{parsers_object}

        {0:indent$}{0:indent$}return termite.Result.error('{parser_errors}', '');
        {0:indent$}}}

        {0:indent$}/// Constructs a [{name}] from a [termite.Node].
        {0:indent$}static termite.Result<{name}> fromNode(termite.Node node) {{
        {0:indent$}{0:indent$}{parsers}

        {0:indent$}{0:indent$}return termite.Result.error('{parser_errors}', '');
        {0:indent$}}}
        }}",
        "",
    );
}

mod variant_type {
    use super::*;

    /// Constructs the implementation of a single variant type
    ///
    /// # Parameters
    ///
    /// data: The name of the variant data type
    ///
    /// name: The name of the main variant
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn get_implementation(data: &str, name: &str, indent: usize) -> String {
        return formatdoc!(
            "
            class {name}Type{data} extends {name} {{
            {0:indent$}/// The stored variant value of type [{data}].
            {0:indent$}{data} value;

            {0:indent$}{name}Type{data}._(this.value);

            {0:indent$}@override
            {0:indent$}termite.Node toNode() {{
            {0:indent$}{0:indent$}return value.toNode();
            {0:indent$}}}

            {0:indent$}@override
            {0:indent$}String toString() => '{data}($value)';

            {0:indent$}@override
            {0:indent$}bool operator ==(Object other) {{
            {0:indent$}{0:indent$}return other is {name}Type{data} && other.value == value;
            {0:indent$}}}

            {0:indent$}@override
            {0:indent$}int get hashCode => value.hashCode;
            }}",
            ""
        );
    }

    /// Generates the constructor for a single variant data type
    ///
    /// # Parameters
    ///
    /// data: The name of the variant data type
    ///
    /// name: The name of the main variant
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn get_constructor(data: &str, name: &str, indent: usize) -> String {
        return formatdoc!(
            "
            /// Constructs a new [{name}] of type [{data}] with a value of [value].
            {0:indent$}factory {name}.new{data}({data} value) = {name}Type{data}._;",
            ""
        );
    }

    /// Generates the node parser for a single variant data type
    ///
    /// # Parameters
    ///
    /// data: The name of the variant data type
    ///
    /// name: The name of the main variant
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn get_parser(data: &str, name: &str, indent: usize) -> String {
        return formatdoc!(
            "
            final __{data} = TermiteExtension{data}.fromNode(node);
            {0:indent$}{0:indent$}if (__{data}.isOk()) {{
            {0:indent$}{0:indent$}{0:indent$}return __{data}.asOk().asNewOk((value) => {name}.new{data}(value));
            {0:indent$}{0:indent$}}}
            {0:indent$}{0:indent$}final __{data}Error = __{data}.asError().addField('{data}');",
            ""
        );
    }

    /// Generates the object parser for a single variant data type
    ///
    /// # Parameters
    ///
    /// data: The name of the variant data type
    ///
    /// name: The name of the main variant
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn get_parser_object(data: &str, name: &str, indent: usize) -> String {
        return formatdoc!(
            "
            final __{data} = TermiteExtension{data}.fromObject(obj);
            {0:indent$}{0:indent$}if (__{data}.isOk()) {{
            {0:indent$}{0:indent$}{0:indent$}return __{data}.asOk().asNewOk((value) => {name}.new{data}(value));
            {0:indent$}{0:indent$}}}
            {0:indent$}{0:indent$}final __{data}Error = __{data}.asError().addField('{data}');",
            ""
        );
    }

    /// Constructs the parser error constructor for a single variant data type
    ///
    /// # Parameters
    ///
    /// data: The name of the variant data type
    ///
    /// name: The name of the main variant
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn get_parser_error(data: &str) -> String {
        return format!("{{${{__{data}Error.getMessage()}}}}");
    }
}

#[cfg(test)]
mod tests {
    use crate::dart::test_utils::*;

    #[test]
    fn basic() {
        run_test("type_variant/basic", true, false, false);
    }
}
