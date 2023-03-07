use crate::types::core::error::ContractError;
use cosmwasm_std::{Addr, Storage};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const CONTRACT_TYPE: &str = env!("CARGO_CRATE_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const NAMESPACE_CONTRACT_STATE: &str = "contract_state";
const CONTRACT_STATE: Item<ContractState> = Item::new(NAMESPACE_CONTRACT_STATE);

/// Stores the core contract configurations created on instantiated and modified on migration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ContractState {
    /// The bech32 address of the account that has admin rights within this contract.
    pub admin: Addr,
    /// The [Provenance Name Module](https://docs.provenance.io/modules/name-module) fully-qualified
    /// name that is used to bind attributes to accounts when consenting to group membership.
    pub attribute_name: String,
    /// A free-form name defining this particular contract instance.  Used for identification on
    /// query purposes only.
    pub contract_name: String,
    /// The crate name, used to ensure that newly-migrated instances match the same contract format.
    pub contract_type: String,
    /// The crate version, used to ensure that newly-migrated instances do not attempt to use an
    /// identical or older version.
    pub contract_version: String,
}
impl ContractState {
    /// Constructs a new instance of this struct.
    ///
    /// # Parameters
    ///
    /// * `admin` The bech32 address of the account that has admin rights within this contract.
    /// * `attribute_name` The [Provenance Name Module](https://docs.provenance.io/modules/name-module)
    /// fully-qualified name that is used to bind attributes to accounts when consenting to group
    /// membership.
    /// * `contract_name` A free-form name defining this particular contract instance.  Used for
    /// identification on query purposes only.
    pub fn new<S1: Into<String>, S2: Into<String>>(
        admin: Addr,
        attribute_name: S1,
        contract_name: S2,
    ) -> Self {
        Self {
            admin,
            attribute_name: attribute_name.into(),
            contract_name: contract_name.into(),
            contract_type: CONTRACT_TYPE.to_string(),
            contract_version: CONTRACT_VERSION.to_string(),
        }
    }
}

/// Overwrites the existing singleton contract storage instance of [ContractState] with the input
/// reference.  An error is returned if the store write is unsuccessful.
///
/// # Parameters
///
/// * `storage` A mutable instance of the contract storage value, allowing internal store
/// manipulation.
/// * `contract_state` The new value for which an internal storage write will be done.
pub fn set_contract_state(
    storage: &mut dyn Storage,
    contract_state: &ContractState,
) -> Result<(), ContractError> {
    CONTRACT_STATE
        .save(storage, contract_state)
        .map_err(|e| ContractError::StorageError {
            message: format!("{e:?}"),
        })
}

/// Fetches the current contract instance of contract state.  This call should never fail because
/// the state is set on contract instantiation, but an error will be returned if store communication
/// fails.
///
/// # Parameters
///
/// * `storage` An immutable instance of the contract storage value, allowing internal store data
/// fetches.
pub fn get_contract_state(storage: &dyn Storage) -> Result<ContractState, ContractError> {
    CONTRACT_STATE
        .load(storage)
        .map_err(|e| ContractError::StorageError {
            message: format!("{e:?}"),
        })
}

#[cfg(test)]
mod tests {
    use crate::store::contract_state::{
        get_contract_state, set_contract_state, ContractState, CONTRACT_TYPE, CONTRACT_VERSION,
    };
    use crate::test::test_constants::{
        DEFAULT_CONTRACT_ADMIN, DEFAULT_CONTRACT_ATTRIBUTE, DEFAULT_CONTRACT_NAME,
    };
    use cosmwasm_std::Addr;
    use provwasm_mocks::mock_dependencies;

    #[test]
    pub fn test_set_and_get_contract_state() {
        let mut deps = mock_dependencies(&[]);
        set_contract_state(
            &mut deps.storage,
            &ContractState::new(
                Addr::unchecked(DEFAULT_CONTRACT_ADMIN),
                DEFAULT_CONTRACT_ATTRIBUTE,
                DEFAULT_CONTRACT_NAME,
            ),
        )
        .expect("the contract state should be saved successfully");
        let contract_state = get_contract_state(&deps.storage)
            .expect("contract state should be successfully pulled from storage");
        assert_eq!(
            contract_state.admin.as_str(),
            DEFAULT_CONTRACT_ADMIN,
            "unexpected contract admin",
        );
        assert_eq!(
            contract_state.attribute_name, DEFAULT_CONTRACT_ATTRIBUTE,
            "unexpected contract attribute",
        );
        assert_eq!(
            contract_state.contract_name, DEFAULT_CONTRACT_NAME,
            "unexpected contract name",
        );
        assert_eq!(
            contract_state.contract_type, CONTRACT_TYPE,
            "unexpected contract type",
        );
        assert_eq!(
            contract_state.contract_version, CONTRACT_VERSION,
            "unexpected contract version",
        );
    }
}
