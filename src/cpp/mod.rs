//! 
//! This module handles generation of c++ code to support a data model, it includes the ability to create
//! a header file, (de)serialization and documentation.
//! 

mod header;

pub use header::header;
pub use header::HeaderError;