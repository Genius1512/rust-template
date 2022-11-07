use thiserror::Error;

pub type GenericError = Box<dyn std::error::Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error: {0}")]
    Generic(String),
}
