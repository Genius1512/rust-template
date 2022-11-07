pub use crate::error::{Error, GenericError};

pub type Result<T> = core::result::Result<T, GenericError>;
