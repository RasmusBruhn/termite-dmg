use indoc::formatdoc;

use crate::data_model;

impl data_model::ConstrainedType {
    /// Generates the Dart source code for a constrained type
    ///
    /// # Parameters
    ///
    /// name: The name of the constrained type
    ///
    /// indent: The number of spaces per indentation level
    pub(super) fn get_dart(&self, name: &str, indent: usize) -> String {
        let constraints = self
            .constraints
            .iter()
            .map(|constraint| {
                return format!("- {constraint}", constraint = constraint.get_dart());
            })
            .collect::<Vec<_>>()
            .join(&format!("\n{0:indent$}/// ", ""));

        let validation = self
            .constraints
            .iter()
            .map(|constraint| {
                formatdoc!(
                    "
                    if (!({constraint})) {{
                    {0:indent$}{0:indent$}{0:indent$}return termite.Result.error('{constraint}', '');
                    {0:indent$}{0:indent$}}}",
                    "",
                    constraint = constraint.get_dart(),
                )
            })
            .collect::<Vec<_>>()
            .join(&format!("\n\n{0:indent$}{0:indent$}", ""));

        return formatdoc!("
            class {name} {{
            {0:indent$}{data_type} _value;

            {0:indent$}{name}._(this._value);

            {0:indent$}{data_type} get value => _value;
            {0:indent$}set value({data_type} x) {{
            {0:indent$}{0:indent$}final validation = validate(x);
            {0:indent$}{0:indent$}if (validation is termite.Error<void>) {{
            {0:indent$}{0:indent$}{0:indent$}throw ArgumentError(validation.error);
            {0:indent$}{0:indent$}}}
            {0:indent$}{0:indent$}_value = x;
            {0:indent$}}}

            {0:indent$}/// Constructs a [{name}] from a [{data_type}] if it fulfills the constraints:
            {0:indent$}///
            {0:indent$}/// {constraints}
            {0:indent$}static termite.Result<{name}> fromValue({data_type} x) {{
            {0:indent$}{0:indent$}final validation = validate(x);
            {0:indent$}{0:indent$}if (validation is termite.Error<void>) {{
            {0:indent$}{0:indent$}{0:indent$}return termite.Result.error(validation.error, validation.location);
            {0:indent$}{0:indent$}}}
            {0:indent$}{0:indent$}return termite.Result.ok({name}._(x));
            {0:indent$}}}

            {0:indent$}/// Constructs a [{name}] from a [termite.Node] if it fulfills the constraints:
            {0:indent$}/// 
            {0:indent$}/// {constraints}
            {0:indent$}static termite.Result<{name}> fromNode(termite.Node node) {{
            {0:indent$}{0:indent$}return TermiteNodeParser{name}.fromNode(node);
            {0:indent$}}}

            {0:indent$}/// Converts the [{name}] to a [termite.Node].
            {0:indent$}termite.Node toNode() {{
            {0:indent$}{0:indent$}return _value.toNode();
            {0:indent$}}}

            {0:indent$}/// Validates that [x] fullfills the constraints:
            {0:indent$}///
            {0:indent$}/// {constraints}
            {0:indent$}static termite.Result<void> validate({data_type} x) {{
            {0:indent$}{0:indent$}{validation}

            {0:indent$}{0:indent$}return termite.Result.ok(null);
            {0:indent$}}}

            {0:indent$}@override
            {0:indent$}String toString() => '$_value';
            }}

            extension TermiteNodeParser{name} on {name} {{
            {0:indent$}/// Constructs a [{name}] from a [termite.Node] if it fulfills the constraints:
            {0:indent$}/// 
            {0:indent$}/// {constraints}
            {0:indent$}static termite.Result<{name}> fromNode(termite.Node node) {{
            {0:indent$}{0:indent$}final value = TermiteNodeParser{data_type}.fromNode(node);
            {0:indent$}{0:indent$}if (value is termite.Error<{data_type}>) {{
            {0:indent$}{0:indent$}{0:indent$}return termite.Result.error(value.error, value.location);
            {0:indent$}{0:indent$}}}
            {0:indent$}{0:indent$}return {name}.fromValue((value as termite.Ok<{data_type}>).value);
            {0:indent$}}}
            }}",
            "",
            data_type = &self.data_type,
        );
    }
}

impl data_model::Constraint {
    /// Converts the constraint to a Dart expression
    pub fn get_dart(&self) -> String {
        match self {
            data_model::Constraint::Arithmetic(value) => value.clone(),
            data_model::Constraint::Function(value) => {
                format!("{value}(x)", value = value.replace("::", "."))
            }
        }
    }
}
