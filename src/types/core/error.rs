use cosmwasm_std::StdError;
use thiserror::Error;

/// The base error enum that is used to wrap any errors that occur throughout contract execution.
#[derive(Error, Debug)]
pub enum ContractError {
    /// Occurs when an error is encountered during a contract execute route invocation.
    #[error("Contract execution on route [{route}] failed: {message}")]
    ExecuteError {
        /// The route on which the error occurred.
        route: String,
        /// A free-form message describing the nature of the error.
        message: String,
    },

    /// Occurs when an error is encountered during contract instantiation.
    #[error("Contract instantiation failed: {message}")]
    InstantiationError {
        /// A free-form message describing the nature of the error.
        message: String,
    },

    /// An error that occurs when an invalid text format is detected.
    #[error("invalid format: {message}")]
    InvalidFormatError {
        /// A free-form message describing the nature of the error.
        message: String,
    },

    /// Occurs when the account invoking a contract route provides an incorrect amount of funds.
    #[error("Invalid funds: {message}")]
    InvalidFundsError {
        /// A free-form message describing the nature of the error.
        message: String,
    },

    /// Occurs when an error is encountered during a contract migration.
    #[error("Contract migration failed: {message}")]
    MigrationError {
        /// A free-form message describing the nature of the error.
        message: String,
    },

    /// Occurs when the semver library fails an operation.  This wraps the original error to allow
    /// it to conform with the [ContractError] typing.
    #[error("{0}")]
    SemVerError(#[from] semver::Error),

    /// Occurs when the Cosmwasm Std library fails an operation.  This wraps the original error to
    /// allow it to conform with the [ContractError] typing.
    #[error("{0}")]
    Std(#[from] StdError),

    /// Occurs when an error is encountered during contract store communication.
    #[error("Contract storage error occurred: {message}")]
    StorageError {
        /// A free-form message describing the nature of the error.
        message: String,
    },
}
