use crate::execute::approve_group_membership::approve_group_membership;
use crate::instantiate::instantiate_contract::instantiate_contract;
use crate::migrate::contract_upgrade::contract_upgrade;
use crate::query::query_contract_state::query_contract_state;
use crate::types::core::error::ContractError;
use crate::types::core::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response};
use provwasm_std::{ProvenanceMsg, ProvenanceQuery};

#[entry_point]
pub fn instantiate(
    deps: DepsMut<ProvenanceQuery>,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    instantiate_contract(deps, env, info, msg)
}

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
