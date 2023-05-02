


use aliri_tokens::sources::oauth2::TokenRequestError;
use thiserror::Error as DeriveError;


#[derive(Clone, DeriveError, Debug)]
pub enum Error {
    #[error("Error building client: {0}")]
    BuildingClient(String),
    #[error("Error building token watcher: {0}")]
    BuildingTokenWatcher(String),
}

/// A `Result` alias where the `Err` case is `aliri_extra_reqwest::Error`.
pub type Result<T> = std::result::Result<T, Error>;

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::BuildingClient(error.to_string())
    }
}

impl From<TokenRequestError> for Error {
    fn from(error: TokenRequestError) -> Self {
        Error::BuildingTokenWatcher(error.to_string())
    }
}

