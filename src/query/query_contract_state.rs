use crate::store::contract_state::get_contract_state;
use crate::types::core::error::ContractError;
use cosmwasm_std::{to_json_binary, Binary, Deps};
use result_extensions::ResultExtensions;

/// Fetches the current values within the [ContractState](crate::store::contract_state::ContractState).
///
/// # Parameters
///
/// * `deps` A dependencies object provided by the cosmwasm framework.  Allows access to useful
/// resources like contract internal storage and a querier to retrieve blockchain objects.
pub fn query_contract_state(deps: Deps) -> Result<Binary, ContractError> {
    to_json_binary(&get_contract_state(deps.storage)?)?.to_ok()
}

#[cfg(test)]
mod tests {
    use crate::query::query_contract_state::query_contract_state;
    use crate::store::contract_state::{set_contract_state, ContractState};
    use crate::test::test_constants::{
        DEFAULT_CONTRACT_ADMIN, DEFAULT_CONTRACT_ATTRIBUTE, DEFAULT_CONTRACT_NAME,
    };
    use crate::types::core::error::ContractError;
    use cosmwasm_std::{from_json, Addr};
    use provwasm_mocks::mock_provenance_dependencies;

    #[test]
    fn test_query_when_missing_contract_state() {
        let deps = mock_provenance_dependencies();
        let result = query_contract_state(deps.as_ref());
        assert!(
            matches!(result, Err(ContractError::StorageError { .. })),
            "a storage error should be emitted when no contract state exists",
        );
    }

    #[test]
    fn test_query_when_contract_state_available() {
        let mut deps = mock_provenance_dependencies();
        let contract_state = ContractState::new(
            Addr::unchecked(DEFAULT_CONTRACT_ADMIN),
            DEFAULT_CONTRACT_ATTRIBUTE,
            DEFAULT_CONTRACT_NAME,
        );
        set_contract_state(deps.as_mut().storage, &contract_state)
            .expect("contract state should be successfully stored");
        let result_binary = query_contract_state(deps.as_ref())
            .expect("contract state should be successfully derived");
        let result_contract_state = from_json::<ContractState>(&result_binary)
            .expect("the contract state should successfully deserialize from binary");
        assert_eq!(
            contract_state, result_contract_state,
            "the resulting contract state should be equivalent to the stored value",
        );
    }
}
