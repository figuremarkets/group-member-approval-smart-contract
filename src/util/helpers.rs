use crate::types::core::error::ContractError;
use cosmwasm_std::MessageInfo;
use result_extensions::ResultExtensions;

pub fn check_funds_are_empty(info: &MessageInfo) -> Result<(), ContractError> {
    if !info.funds.is_empty() {
        ContractError::InvalidFundsError {
            message: "route requires no funds be present".to_string(),
        }
        .to_err()
    } else {
        Ok(())
    }
}
