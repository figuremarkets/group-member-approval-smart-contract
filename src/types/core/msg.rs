use cosmwasm_std::Uint64;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The msg that is sent to the chain in order to instantiate a new instance of this contract's
/// stored code.  Used in the functionality defined in [instantiate_contract](crate::instantiate::instantiate_contract::instantiate_contract).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    /// A free-form name defining this particular contract instance.  Used for identification on
    /// query purposes only.
    pub contract_name: String,
    /// The [Provenance Name Module](https://docs.provenance.io/modules/name-module) fully-qualified
    /// name that is used to bind attributes to accounts when consenting to group membership.
    pub attribute_name: String,
    /// If true, a new [Provenance Name Module](https://docs.provenance.io/modules/name-module) name
    /// will be bound directly to the contract.  This contract will not function unless a name has
    /// been bound, but this option exists to remedy a common issue with the name module: If the
    /// parent name desired is restricted, its owner must manually bind that name to the contract
    /// after its instantiation.  Attempting a bind of a restricted name will cause instantiation
    /// to fail.
    pub bind_attribute_name: bool,
}

/// All defined payloads to be used when executing routes on this contract instance.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// A route that allows the signing account to approve its membership to a
    /// [Provenance Blockchain Group](https://docs.cosmos.network/main/modules/group) by adding an
    /// attribute to their account that includes the given group id.  This invokes the functionality
    /// defined in [approve_group_membership](crate::execute::approve_group_membership::approve_group_membership).
    ApproveGroupMembership {
        /// The unique identifier of the group for which the signing account consents to membership.
        group_id: Uint64,
    },
}

/// All defined payloads to be used when querying routes on this contract instance.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// A route that returns the current [ContractState](crate::store::contract_state::ContractState)
    /// value stored in state.  Invokes the functionality defined in [query_contract_state](crate::query::query_contract_state::query_contract_state).
    QueryContractState {},
}

/// All defined payloads to be used when migrating to a new instance of this contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {
    /// The standard migration route that modifies [ContractState](crate::store::contract_state::ContractState)
    /// to include the new values defined in a target code instance.  Invokes the functionality
    /// defined in [contract_upgrade](crate::migrate::contract_upgrade::contract_upgrade).
    ContractUpgrade {},
}
