use crate::instantiate::instantiate_contract::instantiate_contract;
use crate::test::test_constants::{
    DEFAULT_CONTRACT_ADMIN, DEFAULT_CONTRACT_ATTRIBUTE, DEFAULT_CONTRACT_NAME,
};
use crate::types::core::msg::InstantiateMsg;
use cosmwasm_std::testing::{message_info, mock_env};
use cosmwasm_std::{Addr, DepsMut};

pub fn test_instantiate(deps: DepsMut) {
    instantiate_contract(
        deps,
        mock_env(),
        message_info(&Addr::unchecked(DEFAULT_CONTRACT_ADMIN), &[]),
        InstantiateMsg {
            contract_name: DEFAULT_CONTRACT_NAME.to_string(),
            attribute_name: DEFAULT_CONTRACT_ATTRIBUTE.to_string(),
            bind_attribute_name: true,
        },
    )
    .expect("contract should be instantiated successfully with default params");
}
