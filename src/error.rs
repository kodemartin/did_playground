//! Library-specific errors
use thiserror::Error;

/// Variants of service-specific errors.
#[derive(Error, Debug)]
pub enum DidPlaygroundError {
    #[error("could not send hello to remote subject")]
    Hello,
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    IotaCore(#[from] identity_iota::iota_core::Error),
    #[error(transparent)]
    Did(#[from] identity_iota::did::Error),
    #[error(transparent)]
    DidClient(#[from] identity_iota::client::Error),
    #[error(transparent)]
    DidAccount(#[from] identity_iota::account::Error),
    #[error(transparent)]
    DidStorage(#[from] identity_iota::account_storage::Error),
    #[error(transparent)]
    DidCore(#[from] identity_iota::core::Error),
    #[error("required env variable unset")]
    EnvVar(#[from] std::env::VarError),
}

/// Alias for a `std::result::Result` that always return an error of type [`DidPlaygroundError`][].
pub type Result<T> = std::result::Result<T, DidPlaygroundError>;
