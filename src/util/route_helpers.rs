use crate::types::core::error::ContractError;
use cosmwasm_std::MessageInfo;
use result_extensions::ResultExtensions;

/// Verifies that the provided info does not include and funds, ensuring that the account invoking
/// the contract does not accidentally store funds in the contract that cannot be retrieved due to
/// lack of tracking.
///
/// # Parameters
///
/// * `info` A message information object provided by the cosmwasm framework.  Describes the sender
/// of the instantiation message, as well as the funds provided as an amount during the transaction.
pub fn check_funds_are_empty(info: &MessageInfo) -> Result<(), ContractError> {
    if !info.funds.is_empty() {
        ContractError::InvalidFundsError {
            message: "route requires no funds be present".to_string(),
        }
        .to_err()
    } else {
        ().to_ok()
    }
}
