//!
//! This module handles generation of Dart code to support a data model, it
//! includes the ability to create a source file, (de)serialization and
//! documentation.
//!
//! For any data model to work the termite dependency must be generated from
//! get_termite_dependency() and be saved as "termite.dart" and
//! "termite_types.dart" at a location where they can be included as "import
//! 'termite.dart';" and "import 'termite_types.dart';"
//!

mod type_array;
mod type_constrained;
mod type_enum;
mod type_struct;
mod type_variant;

/// Obtains the base termite Dart dependencies required for all generated data
/// models, must be saved as "termite.dart" and "termite_types.dart"
pub fn get_termite_dependency() -> (&'static str, &'static str) {
    return (
        include_str!("termite.dart"),
        include_str!("termite-types.dart"),
    );
}

/// Obtains the JSON interface source for reading and writing json objects
pub fn get_json_interface() -> &'static str {
    return include_str!("termite-json.dart");
}
