use crate::store::contract_state::{set_contract_state, ContractState};
use crate::types::core::error::ContractError;
use crate::types::core::msg::InstantiateMsg;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use provwasm_std::{bind_name, NameBinding, ProvenanceMsg, ProvenanceQuery};
use result_extensions::ResultExtensions;

pub fn instantiate_contract(
    deps: DepsMut<ProvenanceQuery>,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    if msg.contract_name.is_empty() {
        return ContractError::InstantiationError {
            message: "Provided contract name must not be empty".to_string(),
        }
        .to_err();
    }
    if msg.attribute_name.is_empty() {
        return ContractError::InstantiationError {
            message: "Provided attribute name must not be empty".to_string(),
        }
        .to_err();
    }
    let contract_state = ContractState::new(info.sender, &msg.attribute_name, &msg.contract_name);
    set_contract_state(deps.storage, &contract_state)?;
    let mut response = Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract_name", &msg.contract_name)
        .add_attribute("contract_attribute", &msg.attribute_name);
    if msg.bind_attribute_name {
        response = response.add_message(bind_name(
            msg.attribute_name,
            env.contract.address,
            NameBinding::Restricted,
        )?);
    }
    response.to_ok()
}

#[cfg(test)]
mod tests {
    use crate::instantiate::instantiate_contract::instantiate_contract;
    use crate::test::test_constants::{
        DEFAULT_CONTRACT_ADMIN, DEFAULT_CONTRACT_ATTRIBUTE, DEFAULT_CONTRACT_NAME,
    };
    use crate::types::core::error::ContractError;
    use crate::types::core::msg::InstantiateMsg;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use provwasm_mocks::mock_dependencies;

    #[test]
    fn test_instantiate_with_invalid_contract_name() {
        let mut deps = mock_dependencies(&[]);
        let info = mock_info(DEFAULT_CONTRACT_ADMIN, &[]);
        let msg = InstantiateMsg {
            contract_name: "".to_string(),
            attribute_name: DEFAULT_CONTRACT_ATTRIBUTE.to_string(),
            bind_attribute_name: true,
        };
        let result = instantiate_contract(deps.as_mut(), mock_env(), info, msg);
        assert!(
            matches!(result, Err(ContractError::InstantiationError { .. })),
            "An instantiation error should occur when an invalid contract name is used",
        );
    }

    #[test]
    fn test_instantiate_with_invalid_attribute_name() {
        let mut deps = mock_dependencies(&[]);
        let info = mock_info(DEFAULT_CONTRACT_ADMIN, &[]);
        let msg = InstantiateMsg {
            contract_name: DEFAULT_CONTRACT_NAME.to_string(),
            attribute_name: "".to_string(),
            bind_attribute_name: true,
        };
        let result = instantiate_contract(deps.as_mut(), mock_env(), info, msg);
        assert!(
            matches!(result, Err(ContractError::InstantiationError { .. })),
            "An instantiation error should occur when an invalid contract name is used",
        );
    }
}
