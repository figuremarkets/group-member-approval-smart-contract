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
    use crate::migrate::contract_upgrade::contract_upgrade;
    use crate::store::contract_state::{
        get_contract_state, set_contract_state, CONTRACT_TYPE, CONTRACT_VERSION,
    };
    use crate::test::test_constants::{
        DEFAULT_CONTRACT_ADMIN, DEFAULT_CONTRACT_ATTRIBUTE, DEFAULT_CONTRACT_NAME,
    };
    use crate::test::test_helpers::single_attribute_for_key;
    use crate::test::test_instantiate::test_instantiate;
    use crate::types::core::error::ContractError;
    use crate::types::core::msg::InstantiateMsg;
    use cosmwasm_std::testing::mock_info;
    use provwasm_mocks::mock_dependencies;

    #[test]
    fn test_successful_migration() {
        let mut deps = mock_dependencies(&[]);
        test_instantiate(deps.as_mut());
        let mut contract_state = get_contract_state(deps.as_ref().storage)
            .expect("contract state should load after instantiation");
        contract_state.contract_version = "0.0.1".to_string();
        set_contract_state(deps.as_mut().storage, &contract_state)
            .expect("contract state should save successfully");
        assert_eq!(
            "0.0.1",
            get_contract_state(deps.as_ref().storage)
                .expect("contract state should load after modifications")
                .contract_version,
            "sanity check: contract version should be successfully updated",
        );
        let response = contract_upgrade(deps.as_mut())
            .expect("contract migration should succeed when versions are appropriately set");
        assert!(
            response.messages.is_empty(),
            "migrations should never produce messages",
        );
        assert_eq!(
            2,
            response.attributes.len(),
            "the correct number of attributes should be emitted",
        );
        assert_eq!(
            "migrate_contract",
            single_attribute_for_key(&response, "action"),
            "the correct action attribute value should be produced",
        );
        assert_eq!(
            CONTRACT_VERSION,
            single_attribute_for_key(&response, "new_version"),
            "the correct new_version attribute value should be produced",
        );

        let contract_state = get_contract_state(deps.as_ref().storage)
            .expect("contract state should load after a migration");
        assert_eq!(
            CONTRACT_VERSION, contract_state.contract_version,
            "the contract state should have its contract version altered by the migration",
        );
    }

    #[test]
    fn test_invalid_migration_scenarios() {
        let mut deps = mock_dependencies(&[]);
        test_instantiate(deps.as_mut());
        let mut contract_state = get_contract_state(deps.as_ref().storage)
            .expect("expected contract state to load after instantiation");
        contract_state.contract_type = "unexpected contract type".to_string();
        set_contract_state(deps.as_mut().storage, &contract_state)
            .expect("expected contract state to be stored correctly");
        let err = contract_upgrade(deps.as_mut())
            .expect_err("an error should occur when migrating from a different contract type");
        match err {
            ContractError::MigrationError { message } => {
                assert_eq!(
                    format!("target migration contract type [{}] does not match stored contract type [unexpected contract type]", CONTRACT_TYPE),
                    message,
                    "unexpected error message when bad contract type encountered",
                );
            }
            e => panic!("unexpected error emitted: {:?}", e),
        };
        contract_state.contract_type = CONTRACT_TYPE.to_string();
        contract_state.contract_version = "999.999.999".to_string();
        set_contract_state(deps.as_mut().storage, &contract_state)
            .expect("expected contract state to be stored successfully after a modification");
        let err = contract_upgrade(deps.as_mut()).expect_err(
            "an error should be produced if the contract is downgraded to a lower version",
        );
        match err {
            ContractError::MigrationError { message } => {
                assert_eq!(
                    format!("target migration contract version [{}] is too low to use. stored contract version is [999.999.999]", CONTRACT_VERSION),
                    message,
                    "unexpected error message when bad contract version encountered",
                );
            }
            e => panic!("unexpected error emitted: {:?}", e),
        };
    }
}
