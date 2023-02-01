use crate::store::contract_state::{
    get_contract_state, set_contract_state, ContractState, CONTRACT_TYPE, CONTRACT_VERSION,
};
use crate::types::core::error::ContractError;
use cosmwasm_std::{to_binary, DepsMut, Response};
use provwasm_std::{ProvenanceMsg, ProvenanceQuery};
use result_extensions::ResultExtensions;
use semver::Version;

pub fn contract_upgrade(
    deps: DepsMut<ProvenanceQuery>,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let mut contract_state = get_contract_state(deps.storage)?;
    check_valid_migration(&contract_state)?;
    contract_state.contract_version = CONTRACT_VERSION.to_string();
    set_contract_state(deps.storage, &contract_state)?;
    Response::new()
        .add_attribute("action", "migrate_contract")
        .add_attribute("new_version", CONTRACT_VERSION)
        .set_data(to_binary(&contract_state)?)
        .to_ok()
}

fn check_valid_migration(contract_state: &ContractState) -> Result<(), ContractError> {
    // Prevent other contracts of different types from migrating over this one
    if CONTRACT_TYPE != contract_state.contract_type {
        return ContractError::MigrationError {
            message: format!(
                "target migration contract type [{}] does not match stored contract type [{}]",
                CONTRACT_TYPE, contract_state.contract_type,
            ),
        }
        .to_err();
    }
    let existing_contract_version = contract_state.contract_version.parse::<Version>()?;
    let new_contract_version = CONTRACT_VERSION.parse::<Version>()?;
    // Ensure only new contract versions are allowed
    if existing_contract_version >= new_contract_version {
        return ContractError::MigrationError {
            message: format!(
                "target migration contract version [{}] is too low to use. stored contract version is [{}]",
                CONTRACT_VERSION, &contract_state.contract_version,
            )
        }
        .to_err();
    }
    ().to_ok()
}

#[cfg(test)]
mod tests {
    use provwasm_mocks::mock_dependencies;

    #[test]
    fn test_successful_migration() {
        let mut deps = mock_dependencies(&[]);
    }
}
