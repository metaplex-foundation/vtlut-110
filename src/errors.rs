use thiserror::Error;

#[derive(Debug, Error)]
pub enum VtError {
    #[error("Client Error: {0}")]
    ClientError(String),
    #[error("Deserialization Error: {0}")]
    SerializationError(String),
    #[error("Conversion Error: {0}")]
    ConversionError(String),
    #[error("Signer Error: {0}")]
    SignerError(String),
}
