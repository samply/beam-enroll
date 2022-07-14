use openssl::error::ErrorStack;

#[derive(Debug)]
pub enum SamplyBeamError {
    WrongBrokerUri(&'static str),
    InvalidPath,
    InvalidProxyIdString(String),
    ConfigurationFailed(String),
    InvalidBeamId(String),
    OpenSSLError(String)
}

impl From<ErrorStack> for SamplyBeamError {
    fn from(e: ErrorStack) -> Self {
        Self::OpenSSLError(e.to_string())
    }
}