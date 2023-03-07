use crate::store::contract_state::{
    get_contract_state, set_contract_state, ContractState, CONTRACT_TYPE, CONTRACT_VERSION,
};
use crate::types::core::error::ContractError;
use cosmwasm_std::{to_binary, DepsMut, Response};
use provwasm_std::{ProvenanceMsg, ProvenanceQuery};
use result_extensions::ResultExtensions;
use semver::Version;

/// The main entrypoint function for running a code migration.  Auxiliary code run when a stored
/// instance of this contract on chain is migrated over the existing instance.  Verifies that the
/// new code instance is a newer version than the current version, and then modifies the contract
/// state to reflect the new version information contained in the stored file.
///
/// # Parameters
///
/// * `deps` A dependencies object provided by the cosmwasm framework.  Allows access to useful
/// resources like contract internal storage and a querier to retrieve blockchain objects.
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

/// Verifies that the executing migration has a valid contract type and contract version based on
/// the currently-stored values.
///
/// # Parameters
///
/// * `contract_state` The current contract state instance from the store.
fn check_valid_migration(contract_state: &ContractState) -> Result<(), ContractError> {
    // Prevent other contracts of different types from migrating over this one
    if CONTRACT_TYPE != contract_state.contract_type {
        return ContractError::MigrationError {
            message: format!(
                "target migration contract type [{CONTRACT_TYPE}] does not match stored contract type [{}]",
                contract_state.contract_type,
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
                "target migration contract version [{CONTRACT_VERSION}] is too low to use. stored contract version is [{}]",
                &contract_state.contract_version,
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
    use crate::test::test_helpers::single_attribute_for_key;
    use crate::test::test_instantiate::test_instantiate;
    use crate::types::core::error::ContractError;
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
                    format!("target migration contract type [{CONTRACT_TYPE}] does not match stored contract type [unexpected contract type]"),
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
                    format!("target migration contract version [{CONTRACT_VERSION}] is too low to use. stored contract version is [999.999.999]"),
                    message,
                    "unexpected error message when bad contract version encountered",
                );
            }
            e => panic!("unexpected error emitted: {:?}", e),
        };
    }
}
