use crate::data_model;
use jzon::JsonValue;
use thiserror::Error;

impl data_model::DataModel {
    /// Creates a JSON schema from the data model
    /// 
    /// # Parameters
    /// 
    /// id: The name of the type to export as a schema
    pub fn export_schema(&self, id: &str) -> Result<JsonValue, > {
        todo!();
    }
}

/// The error types for when creating a new state
#[derive(Error, Debug, Clone)]
pub enum NewStateError {
    /// The width or height of the window is too small
    #[error("The width and height of the window must be larger than 0 but received {:?}", .0)]
    InvalidSize(PhysicalSize<u32>),
    /// The render state could not be created
    #[error("Unable to initialize the render state: {:?}", 0.)]
    RenderInitError(NewRenderStateError),
}
