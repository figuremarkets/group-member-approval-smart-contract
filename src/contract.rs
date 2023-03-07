use crate::execute::approve_group_membership::approve_group_membership;
use crate::instantiate::instantiate_contract::instantiate_contract;
use crate::migrate::contract_upgrade::contract_upgrade;
use crate::query::query_contract_state::query_contract_state;
use crate::types::core::error::ContractError;
use crate::types::core::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response};
use provwasm_std::{ProvenanceMsg, ProvenanceQuery};

/// The entry point used when an account instantiates a stored code wasm payload of this contract on
/// the Provenance Blockchain.
///
/// # Parameters
///
/// * `deps` A dependencies object provided by the cosmwasm framework.  Allows access to useful
/// resources like contract internal storage and a querier to retrieve blockchain objects.
/// * `env` An environment object provided by the cosmwasm framework.  Describes the contract's
/// details, as well as blockchain information at the time of the transaction.
/// * `info` A message information object provided by the cosmwasm framework.  Describes the sender
/// of the instantiation message, as well as the funds provided as an amount during the transaction.
/// * `msg` A custom instantiation message defined by this contract for creating the initial
/// configuration used by the contract.
#[entry_point]
pub fn instantiate(
    deps: DepsMut<ProvenanceQuery>,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    instantiate_contract(deps, env, info, msg)
}

/// The entry point used when an account initiates an execution process defined in the contract.
/// This defines the primary purposes of the contract.
///
/// # Parameters
///
/// * `deps` A dependencies object provided by the cosmwasm framework.  Allows access to useful
/// resources like contract internal storage and a querier to retrieve blockchain objects.
/// * `env` An environment object provided by the cosmwasm framework.  Describes the contract's
/// details, as well as blockchain information at the time of the transaction.
/// * `info` A message information object provided by the cosmwasm framework.  Describes the sender
/// of the instantiation message, as well as the funds provided as an amount during the transaction.
/// * `msg` A custom execution message enum defined by this contract to allow multiple different
/// processes to be defined for the singular execution route entry point allowed by the
/// cosmwasm framework.
#[entry_point]
pub fn execute(
    deps: DepsMut<ProvenanceQuery>,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    match msg {
        ExecuteMsg::ApproveGroupMembership { group_id } => {
            approve_group_membership(deps, info, group_id)
        }
    }
}

/// The entry point used when an account invokes the contract to retrieve information.  Allows
/// access to the internal storage information in an immutable manner.
///
/// # Parameters
///
/// * `deps` A dependencies object provided by the cosmwasm framework.  Allows access to useful
/// resources like contract internal storage and a querier to retrieve blockchain objects.
/// * `_env` An environment object provided by the cosmwasm framework.  Describes the contract's
/// details, as well as blockchain information at the time of the transaction.  Unused by this
/// function, but required by cosmwasm for successfully defined query entrypoint.
/// * `msg` A custom query message enum defined by this contract to allow multiple different results
/// to be determined for this route.
#[entry_point]
pub fn query(
    deps: Deps<ProvenanceQuery>,
    _env: Env,
    msg: QueryMsg,
) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::QueryContractState {} => query_contract_state(deps),
    }
}

/// The entry point used when the contract admin migrates an existing instance of this contract to
/// a new stored code instance on chain.
///
/// # Parameters
///
/// * `deps` A dependencies object provided by the cosmwasm framework.  Allows access to useful
/// resources like contract internal storage and a querier to retrieve blockchain objects.
/// * `_env` An environment object provided by the cosmwasm framework.  Describes the contract's
/// details, as well as blockchain information at the time of the transaction.  Unused by this
/// function, but required by cosmwasm for successfully defined migration entrypoint.
/// * msg` A custom migrate message enum defined by this contract to allow multiple different
/// results of invoking the migrate endpoint.
#[entry_point]
pub fn migrate(
    deps: DepsMut<ProvenanceQuery>,
    _env: Env,
    msg: MigrateMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    match msg {
        MigrateMsg::ContractUpgrade {} => contract_upgrade(deps),
    }
}
