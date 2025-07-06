use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(#[from] std::env::VarError),

    #[error("Browser error: {0}")]
    Browser(String),

    #[error("Transport error: {0}")]
    Transport(#[from] tonic::transport::Error),

    #[error("Screenshot capture failed: {0}")]
    Screenshot(String),

    #[error("Element not found: {0}")]
    ElementNotFound(String),

    #[error("Navigation failed: {0}")]
    Navigation(String),

    #[error("Interaction failed: {0}")]
    Interaction(String),

    #[error("Address parse error: {0}")]
    AddressParse(#[from] std::net::AddrParseError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<AppError> for tonic::Status {
    fn from(err: AppError) -> Self {
        tonic::Status::internal(err.to_string())
    }
}
