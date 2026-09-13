use super::{Error, ErrorCore};
use crate::*;
use indoc::formatdoc;
use std::collections::HashMap;

/// Generates the Dart source code for a constrained type
///
/// # Parameters
///
/// data: The constrained type to generate Dart source code for
///
/// name: The name of the constrained type
///
/// all_types: A map of all data types available for reference
///
/// indent: The number of spaces per indentation level
pub(super) fn generate(
    data: &ConstrainedType,
    name: &str,
    all_types: &HashMap<String, DataType>,
    indent: usize,
) -> Result<String, Error> {
    let constraints = data
        .constraints
        .iter()
        .map(|constraint| constraint::generate(constraint, &data.data_type, all_types))
        .collect::<Result<Vec<_>, Error>>()?;

    let documentation = constraints
        .iter()
        .map(|constraint| return format!("- {constraint}"))
        .collect::<Vec<_>>()
        .join(&format!("\n{0:indent$}/// ", ""));

    let validation = constraints
        .iter()
        .map(|constraint| {
            formatdoc!(
                "
                if (!({constraint})) {{
                {0:indent$}{0:indent$}{0:indent$}return const termite.Result.error('{constraint}', '');
                {0:indent$}{0:indent$}}}",
                ""
            )
        })
        .collect::<Vec<_>>()
        .join(&format!("\n\n{0:indent$}{0:indent$}", ""));

    return Ok(formatdoc!("
        class {name} {{
        {0:indent$}{data_type} _value;

        {0:indent$}{name}._(this._value);
        {0:indent$}{name}(this._value) {{
        {0:indent$}{0:indent$}if (validate(_value) is termite.Error<void>) {{
        {0:indent$}{0:indent$}{0:indent$}throw ArgumentError('Invalid value for {name}');
        {0:indent$}{0:indent$}}}
        {0:indent$}}}

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
        {0:indent$}/// {documentation}
        {0:indent$}static termite.Result<{name}> fromValue({data_type} x) {{
        {0:indent$}{0:indent$}final validation = validate(x);
        {0:indent$}{0:indent$}if (validation is termite.Error<void>) {{
        {0:indent$}{0:indent$}{0:indent$}return termite.Result.error(validation.error, validation.location);
        {0:indent$}{0:indent$}}}
        {0:indent$}{0:indent$}return termite.Result.ok({name}._(x));
        {0:indent$}}}

        {0:indent$}/// Constructs a [{name}] from a [Object] if it fulfills the constraints:
        {0:indent$}/// 
        {0:indent$}/// {documentation}
        {0:indent$}static termite.Result<{name}> fromObject(Object obj) {{
        {0:indent$}{0:indent$}return TermiteExtension{name}.fromObject(obj);
        {0:indent$}}}

        {0:indent$}/// Constructs a [{name}] from a [termite.Node] if it fulfills the constraints:
        {0:indent$}/// 
        {0:indent$}/// {documentation}
        {0:indent$}static termite.Result<{name}> fromNode(termite.Node node) {{
        {0:indent$}{0:indent$}return TermiteExtension{name}.fromNode(node);
        {0:indent$}}}

        {0:indent$}/// Converts the [{name}] to a [termite.Node].
        {0:indent$}termite.Node toNode() {{
        {0:indent$}{0:indent$}return _value.toNode();
        {0:indent$}}}

        {0:indent$}/// Validates that [x] fullfills the constraints:
        {0:indent$}///
        {0:indent$}/// {documentation}
        {0:indent$}static termite.Result<void> validate({data_type} x) {{
        {0:indent$}{0:indent$}{validation}

        {0:indent$}{0:indent$}return const termite.Result.ok(null);
        {0:indent$}}}

        {0:indent$}@override
        {0:indent$}String toString() => '$_value';

        {0:indent$}@override
        {0:indent$}bool operator ==(Object other) {{
        {0:indent$}{0:indent$}return other is {name} && other._value == _value;
        {0:indent$}}}

        {0:indent$}@override
        {0:indent$}int get hashCode => _value.hashCode;
        }}

        extension TermiteExtension{name} on {name} {{
        {0:indent$}/// Constructs a [{name}] from a [Object] if it fulfills the constraints:
        {0:indent$}/// 
        {0:indent$}/// {documentation}
        {0:indent$}static termite.Result<{name}> fromObject(Object obj) {{
        {0:indent$}{0:indent$}final value = TermiteExtension{data_type}.fromObject(obj);
        {0:indent$}{0:indent$}if (!value.isOk()) {{
        {0:indent$}{0:indent$}{0:indent$}return value.asError().addField('{data_type}').asNewError<{name}>();
        {0:indent$}{0:indent$}}}
        {0:indent$}{0:indent$}return {name}.fromValue(value.getOk());
        {0:indent$}}}

        {0:indent$}/// Constructs a [{name}] from a [termite.Node] if it fulfills the constraints:
        {0:indent$}/// 
        {0:indent$}/// {documentation}
        {0:indent$}static termite.Result<{name}> fromNode(termite.Node node) {{
        {0:indent$}{0:indent$}final value = TermiteExtension{data_type}.fromNode(node);
        {0:indent$}{0:indent$}if (!value.isOk()) {{
        {0:indent$}{0:indent$}{0:indent$}return value.asError().addField('{data_type}').asNewError<{name}>();
        {0:indent$}{0:indent$}}}
        {0:indent$}{0:indent$}return {name}.fromValue(value.getOk());
        {0:indent$}}}
        }}",
        "",
        data_type = &data.data_type,
    ));
}

mod constraint {
    use super::*;

    /// Converts the constraint to a Dart expression
    ///
    /// # Parameters
    ///
    /// data: The constraint to convert to a Dart expression
    ///
    /// data_type: The data type of the constrained type
    ///
    /// all_types: A map of all data types available for reference
    pub(super) fn generate(
        data: &Constraint,
        data_type: &str,
        all_types: &HashMap<String, DataType>,
    ) -> Result<String, Error> {
        return match data {
            Constraint::MinLength(value) => generate_min_length(value, data_type, all_types),
            Constraint::MaxLength(value) => generate_max_length(value, data_type, all_types),
            Constraint::Arithmetic(value) => generate_arithmetic(value),
            Constraint::Function(value) => generate_function(value),
        };
    }

    /// Generates a Dart expression for the given constraint value
    ///
    /// # Parameters
    ///
    /// value: The minimum length constraint value
    ///
    /// data_type: The data type of the constrained type
    ///
    /// all_types: A map of all data types available for reference
    pub(super) fn generate_min_length(
        value: &usize,
        data_type: &str,
        all_types: &HashMap<String, DataType>,
    ) -> Result<String, Error> {
        let simplified_type = get_simplified_type(data_type, all_types)?;

        return match simplified_type {
            SimplifiedType::String => Ok(format!("termite.utf8CodePointCount(x) >= {value}")),
            SimplifiedType::Array => Ok(format!("x.values.length >= {value}")),
            _ => Err(Error::new(ErrorCore::UnsupportedConstraintForType(
                "MinLength".to_string(),
                data_type.to_string(),
            ))),
        };
    }

    /// Generates a Dart expression for the given constraint value
    ///
    /// # Parameters
    ///
    /// value: The maximum length constraint value
    ///
    /// data_type: The data type of the constrained type
    ///
    /// all_types: A map of all data types available for reference
    pub(super) fn generate_max_length(
        value: &usize,
        data_type: &str,
        all_types: &HashMap<String, DataType>,
    ) -> Result<String, Error> {
        let simplified_type = get_simplified_type(data_type, all_types)?;

        return match simplified_type {
            SimplifiedType::String => Ok(format!("termite.utf8CodePointCount(x) <= {value}")),
            SimplifiedType::Array => Ok(format!("x.values.length <= {value}")),
            _ => Err(Error::new(ErrorCore::UnsupportedConstraintForType(
                "MaxLength".to_string(),
                data_type.to_string(),
            ))),
        };
    }

    /// Generates a Dart expression for the given constraint value
    ///
    /// # Parameters
    ///
    /// value: The arithmetic expression
    ///
    /// data_type: The data type of the constrained type
    ///
    /// all_types: A map of all data types available for reference
    pub(super) fn generate_arithmetic(value: &str) -> Result<String, Error> {
        return Ok(value.to_string());
    }

    /// Generates a Dart expression for the given constraint value
    ///
    /// # Parameters
    ///
    /// value: The function expression
    ///
    /// data_type: The data type of the constrained type
    ///
    /// all_types: A map of all data types available for reference
    pub(super) fn generate_function(value: &str) -> Result<String, Error> {
        return Ok(format!("{value}(x)", value = value.replace("::", ".")));
    }
}

#[cfg(test)]
mod tests {
    use crate::dart::test_utils::*;

    #[test]
    fn basic() {
        run_test("type_constrained/basic", true, false, false);
    }

    #[test]
    fn constraints() {
        run_test("type_constrained/constraints", true, false, false);
    }

    #[test]
    fn min_length_array() {
        run_test("type_constrained/min_length_array", true, false, false);
    }

    #[test]
    fn max_length_array() {
        run_test("type_constrained/max_length_array", true, false, false);
    }
}
