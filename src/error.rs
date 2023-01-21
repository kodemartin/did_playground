//! Library-specific errors
use thiserror::Error;

/// Variants of service-specific errors.
#[derive(Error, Debug)]
pub enum DidPlaygroundError {
    #[error(transparent)]
    IotaDidAccount(#[from] identity_iota::account::Error),
    #[error(transparent)]
    IotaDidStorage(#[from] identity_iota::account_storage::Error),
    #[error(transparent)]
    IotaDidCore(#[from] identity_iota::core::Error),
    #[error("required env variable unset")]
    EnvVar(#[from] std::env::VarError),
}

/// Alias for a `std::result::Result` that always return an error of type [`DidPlaygroundError`][].
pub type Result<T> = std::result::Result<T, DidPlaygroundError>;
