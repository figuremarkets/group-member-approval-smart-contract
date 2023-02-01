use crate::types::core::error::ContractError;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use provwasm_std::{ProvenanceMsg, ProvenanceQuery};

pub fn approve_group_membership(
    deps: DepsMut<ProvenanceQuery>,
    env: Env,
    info: MessageInfo,
    group_id: Uint128,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    panic!("TODO");
}
