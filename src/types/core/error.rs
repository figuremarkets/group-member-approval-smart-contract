use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Contract instantiation failed: {message}")]
    InstantiationError { message: String },

    #[error("Invalid funds: {message}")]
    InvalidFundsError { message: String },

    #[error("Contract migration failed: {message}")]
    MigrationError { message: String },

    #[error("{0}")]
    SemVerError(#[from] semver::Error),

    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Contract storage error occurred: {message}")]
    StorageError { message: String },
}
