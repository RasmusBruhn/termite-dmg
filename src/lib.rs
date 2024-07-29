//!
//! The Termite Data Model Generator is a crate for generating boiler plate code for data models.
//! 
//! The crate consits of two parts, the first is the data model itself. This is preferably imported 
//! from a yaml or json file into a DataModel object.
//! 
//! TODO: Example of importing a data model
//! 
//! It can also be defined directly in code, however, this is not as readable or easy to write.
//! 
//! A DataModel object consist of a map of header strings and the data types. The header string
//! are strings to add to the top of the generated files like for adding includes. The data types
//! is a list of names and data for the user types defined for the data model and these are used
//! to generated the data model file.
//! 
//! A data type consists of a base type (like struct, variant or enum) and a description and then 
//! it has the type data which is a vector of instance. An instance holds the data for this object
//! and may define a struct field or a variant type.
//! 
//! The second part of the Termite crate is generating the code. For now it only supports c++
//! with the cpp module. From here there are functions to generate header files, (de)serialization
//! and documentation.
//! 

mod data_model;
pub mod cpp;

pub use data_model::{DataModel, DataType, DataTypeData, Struct, StructField, DefaultType, Array, Variant, ConstrainedType};
