use crate::store::contract_state::{set_contract_state, ContractState};
use crate::types::core::error::ContractError;
use crate::types::core::msg::InstantiateMsg;
use crate::util::route_helpers::check_funds_are_empty;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use provwasm_std::{bind_name, NameBinding, ProvenanceMsg, ProvenanceQuery};
use result_extensions::ResultExtensions;

/// The main functionality executed when the smart contract is first instantiated.  This creates the
/// singleton instance of [ContractState] used to verify the attribute name used in the contract,
/// as well as optionally binding the contract's name if it does not need to be bound after creation
/// due to namespace restrictions.
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
pub fn instantiate_contract(
    deps: DepsMut<ProvenanceQuery>,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    check_funds_are_empty(&info)?;
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
    use crate::store::contract_state::get_contract_state;
    use crate::test::test_constants::{
        DEFAULT_CONTRACT_ADMIN, DEFAULT_CONTRACT_ATTRIBUTE, DEFAULT_CONTRACT_NAME,
    };
    use crate::test::test_helpers::single_attribute_for_key;
    use crate::types::core::error::ContractError;
    use crate::types::core::msg::InstantiateMsg;
    use cosmwasm_std::testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{coins, CosmosMsg};
    use provwasm_mocks::mock_dependencies;
    use provwasm_std::{NameMsgParams, ProvenanceMsg, ProvenanceMsgParams};

    #[test]
    fn test_instantiate_with_provided_funds() {
        let mut deps = mock_dependencies(&[]);
        let info = mock_info(DEFAULT_CONTRACT_ADMIN, &coins(100, "nhash"));
        let msg = InstantiateMsg {
            contract_name: DEFAULT_CONTRACT_NAME.to_string(),
            attribute_name: DEFAULT_CONTRACT_ATTRIBUTE.to_string(),
            bind_attribute_name: true,
        };
        let result = instantiate_contract(deps.as_mut(), mock_env(), info, msg);
        assert!(
            matches!(result, Err(ContractError::InvalidFundsError { .. })),
            "an invalid funds error should occur when funds are provided to instantiation",
        );
    }

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
            "an instantiation error should occur when an invalid contract name is used",
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
            "an instantiation error should occur when an invalid contract name is used",
        );
    }

    #[test]
    fn test_valid_instantiate_without_binding_name() {
        let mut deps = mock_dependencies(&[]);
        let info = mock_info(DEFAULT_CONTRACT_ADMIN, &[]);
        let msg = InstantiateMsg {
            contract_name: "some contract name".to_string(),
            attribute_name: "some attribute name".to_string(),
            bind_attribute_name: false,
        };
        let response = instantiate_contract(deps.as_mut(), mock_env(), info, msg)
            .expect("the contract should be successfully instantiated");
        assert!(
            response.messages.is_empty(),
            "no messages should be emitted when the name binding is not requested",
        );
        assert_eq!(
            3,
            response.attributes.len(),
            "the correct number of attributes should be emitted",
        );
        assert_eq!(
            "instantiate",
            single_attribute_for_key(&response, "action"),
            "the action attribute in the response should be set correctly",
        );
        assert_eq!(
            "some contract name",
            single_attribute_for_key(&response, "contract_name"),
            "the contract_name attribute in the response should be set correctly",
        );
        assert_eq!(
            "some attribute name",
            single_attribute_for_key(&response, "contract_attribute"),
            "the contract_attribute attribute in the response should be set correctly",
        );
        let contract_state = get_contract_state(deps.as_ref().storage)
            .expect("a contract state record should be available after instantiation");
        assert_eq!(
            "some contract name", &contract_state.contract_name,
            "the supplied contract name should be used in the contract state",
        );
        assert_eq!(
            "some attribute name", &contract_state.attribute_name,
            "the supplied attribute name should be used in the contract state",
        );
    }

    #[test]
    fn test_valid_instantiate_with_bind_name() {
        let mut deps = mock_dependencies(&[]);
        let info = mock_info(DEFAULT_CONTRACT_ADMIN, &[]);
        let msg = InstantiateMsg {
            contract_name: "some contract name".to_string(),
            attribute_name: "some attribute name".to_string(),
            bind_attribute_name: true,
        };
        let response = instantiate_contract(deps.as_mut(), mock_env(), info, msg)
            .expect("the contract should be successfully instantiated");
        assert_eq!(
            1,
            response.messages.len(),
            "a single message should be emitted when name binding is requested",
        );
        let message = response.messages.first().unwrap();
        match &message.msg {
            CosmosMsg::Custom(ProvenanceMsg {
                params:
                    ProvenanceMsgParams::Name(NameMsgParams::BindName {
                        name,
                        address,
                        restrict,
                    }),
                ..
            }) => {
                assert_eq!(
                    "some attribute name", name,
                    "the provided attribute name should be used in the name binding msg",
                );
                assert_eq!(
                    MOCK_CONTRACT_ADDR,
                    address.as_str(),
                    "the contract address should be used as the bound address for the name",
                );
                assert!(restrict, "the newly bound name should be restricted",);
            }
            msg => panic!("unexpected msg type emitted from instantiate: {:?}", msg),
        }
        assert_eq!(
            3,
            response.attributes.len(),
            "the correct number of attributes should be emitted",
        );
        assert_eq!(
            "instantiate",
            single_attribute_for_key(&response, "action"),
            "the action attribute in the response should be set correctly",
        );
        assert_eq!(
            "some contract name",
            single_attribute_for_key(&response, "contract_name"),
            "the contract_name attribute in the response should be set correctly",
        );
        assert_eq!(
            "some attribute name",
            single_attribute_for_key(&response, "contract_attribute"),
            "the contract_attribute attribute in the response should be set correctly",
        );
        let contract_state = get_contract_state(deps.as_ref().storage)
            .expect("a contract state record should be available after instantiation");
        assert_eq!(
            "some contract name", &contract_state.contract_name,
            "the supplied contract name should be used in the contract state",
        );
        assert_eq!(
            "some attribute name", &contract_state.attribute_name,
            "the supplied attribute name should be used in the contract state",
        );
    }
}
