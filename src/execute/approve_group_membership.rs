use crate::store::contract_state::get_contract_state;
use crate::types::core::error::ContractError;
use crate::util::prov_helpers::get_group_id_attribute_values;
use crate::util::route_helpers::check_funds_are_empty;
use cosmwasm_std::{to_binary, DepsMut, MessageInfo, Response, Uint64};
use provwasm_std::{
    add_attribute, AttributeValueType, ProvenanceMsg, ProvenanceQuerier, ProvenanceQuery,
};
use result_extensions::ResultExtensions;

/// Invoked via the contract's execution functionality.  Adds an attribute to the signer that
/// denotes that they affirm their membership in a [Provenance Blockchain Group](https://docs.cosmos.network/main/modules/group)
/// by setting an int value on the designated attribute equal to the group identifier.  Note:
/// Cosmwasm does not expose functionality to query the group module, so this route does not do any
/// verification that the signer is actually a member of the approved group.  However, consenting to
/// either being or becoming a member of a group is simply an act of compliance.  False claims made
/// herein can be queried from the standard chain routes, which allows external consumers of this
/// attribute to verify this statement after it has been made.  The route does, however, validate
/// that the account does not already have an attribute value affirming the existing group,
/// preventing duplicate writes.
///
/// # Parameters
///
/// * `deps` A dependencies object provided by the cosmwasm framework.  Allows access to useful
/// resources like contract internal storage and a querier to retrieve blockchain objects.
/// * `info` A message information object provided by the cosmwasm framework.  Describes the sender
/// of the instantiation message, as well as the funds provided as an amount during the transaction.
/// * `group_id` The unique identifier of a given group for which the signing account consents to
/// membership.
pub fn approve_group_membership(
    deps: DepsMut<ProvenanceQuery>,
    info: MessageInfo,
    group_id: Uint64,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // Verify that no coin was sent to start this execution route.  The only charge incurred should
    // be a new attribute write
    check_funds_are_empty(&info)?;
    let attribute_name = get_contract_state(deps.storage)?.attribute_name;
    let existing_group_ids = ProvenanceQuerier::new(&deps.querier)
        .get_attributes(info.sender.clone(), Some(attribute_name.clone()))
        .ok()
        .map(|attributes| get_group_id_attribute_values(&attributes, &attribute_name))
        .unwrap_or_default();
    // First, verify that this member has not yet approved itself for this group.  Duplicate ids
    // would be a waste of hash and needlessly increase data storage on chain
    if existing_group_ids
        .iter()
        .any(|id| id.u64() == group_id.u64())
    {
        return ContractError::ExecuteError {
            route: "approve_group_membership".to_string(),
            message: format!(
                "group with id [{}] has already been approved by member [{}]",
                group_id.u64(),
                info.sender.as_str(),
            ),
        }
        .to_err();
    }
    Response::new()
        .add_message(add_attribute(
            info.sender.clone(),
            &attribute_name,
            to_binary(&group_id.u64())?,
            AttributeValueType::Int,
        )?)
        .add_attribute("action", "approve_group_membership")
        .add_attribute("account_address", info.sender.as_str())
        .add_attribute("attribute_name", &attribute_name)
        .add_attribute("group_id", group_id.to_string())
        .to_ok()
}

#[cfg(test)]
mod tests {
    use crate::execute::approve_group_membership::approve_group_membership;
    use crate::test::test_constants::{DEFAULT_CONTRACT_ATTRIBUTE, DEFAULT_GROUP_MEMBER};
    use crate::test::test_helpers::single_attribute_for_key;
    use crate::test::test_instantiate::test_instantiate;
    use crate::types::core::error::ContractError;
    use cosmwasm_std::testing::mock_info;
    use cosmwasm_std::{coins, from_binary, CosmosMsg, Response, Uint64};
    use provwasm_mocks::mock_dependencies;
    use provwasm_std::AttributeMsgParams::AddAttribute;
    use provwasm_std::{AttributeValueType, ProvenanceMsg, ProvenanceMsgParams};

    #[test]
    fn test_rejection_for_provided_funds() {
        let mut deps = mock_dependencies(&[]);
        test_instantiate(deps.as_mut());
        let info = mock_info(DEFAULT_GROUP_MEMBER, &coins(15, "nhash"));
        let err = approve_group_membership(deps.as_mut(), info, Uint64::new(1))
            .expect_err("an error should occur when the sender provides funds");
        assert!(
            matches!(err, ContractError::InvalidFundsError { .. }),
            "an invalid funds error should be emitted when the sender provides funds",
        );
    }

    #[test]
    fn test_rejection_for_existing_attribute() {
        let mut deps = mock_dependencies(&[]);
        test_instantiate(deps.as_mut());
        let info = mock_info(DEFAULT_GROUP_MEMBER, &[]);
        deps.querier.with_attributes(
            DEFAULT_GROUP_MEMBER,
            &[(DEFAULT_CONTRACT_ATTRIBUTE, "1", "int")],
        );
        let err = approve_group_membership(deps.as_mut(), info, Uint64::new(1))
            .expect_err("an error should occur when the member already has an attribute specifying an approval for the target group");
        match err {
            ContractError::ExecuteError { route, message } => {
                assert_eq!(
                    "approve_group_membership", route,
                    "unexpected route in execute error",
                );
                assert_eq!(
                    format!(
                        "group with id [1] has already been approved by member [{DEFAULT_GROUP_MEMBER}]",
                    ),
                    message,
                    "unexpected message in execute error",
                );
            }
            e => panic!("unexpected error emitted: {:?}", e),
        };
    }

    #[test]
    fn test_successful_call_for_new_attribute() {
        let mut deps = mock_dependencies(&[]);
        test_instantiate(deps.as_mut());
        let info = mock_info(DEFAULT_GROUP_MEMBER, &[]);
        let response = approve_group_membership(deps.as_mut(), info, Uint64::new(15))
            .expect("an approval of a new group id should be allowed");
        assert_correct_response_messages(&response, 15);
        assert_correct_response_attributes(&response, 15);
    }

    #[test]
    fn test_successful_call_with_existing_attributes() {
        let mut deps = mock_dependencies(&[]);
        test_instantiate(deps.as_mut());
        let info = mock_info(DEFAULT_GROUP_MEMBER, &[]);
        deps.querier.with_attributes(
            DEFAULT_GROUP_MEMBER,
            &[
                (DEFAULT_CONTRACT_ATTRIBUTE, "1", "int"),
                (DEFAULT_CONTRACT_ATTRIBUTE, "2", "int"),
            ],
        );
        let response = approve_group_membership(deps.as_mut(), info, Uint64::new(3))
            .expect("an approval of a new group id when non-matching existing ids are present should succeed");
        assert_correct_response_messages(&response, 3);
        assert_correct_response_attributes(&response, 3);
    }

    fn assert_correct_response_messages(response: &Response<ProvenanceMsg>, group_id: u64) {
        assert_eq!(
            1,
            response.messages.len(),
            "a single message should be emitted in the response",
        );
        match &response.messages.first().unwrap().msg {
            CosmosMsg::Custom(ProvenanceMsg {
                params:
                    ProvenanceMsgParams::Attribute(AddAttribute {
                        address,
                        name,
                        value,
                        value_type,
                    }),
                ..
            }) => {
                assert_eq!(
                    DEFAULT_GROUP_MEMBER,
                    address.as_str(),
                    "the member should receive the attribute",
                );
                assert_eq!(
                    DEFAULT_CONTRACT_ATTRIBUTE, name,
                    "the name used should be the attribute name stored in the contract",
                );
                assert_eq!(
                    from_binary::<u64>(value).expect(
                        "the binary value in the attribute should deserialize to a u64 correctly"
                    ),
                    group_id,
                    "the group id should be properly added to the attribute",
                );
                assert_eq!(
                    &AttributeValueType::Int,
                    value_type,
                    "the value type should be properly written as Int",
                );
            }
            msg => panic!("unexpected message emitted: {:?}", msg),
        };
    }

    fn assert_correct_response_attributes(response: &Response<ProvenanceMsg>, group_id: u64) {
        assert_eq!(
            4,
            response.attributes.len(),
            "the correct number of attributes should be emitted in the result",
        );
        assert_eq!(
            "approve_group_membership",
            single_attribute_for_key(&response, "action"),
            "the action attribute should have the correct value",
        );
        assert_eq!(
            DEFAULT_GROUP_MEMBER,
            single_attribute_for_key(&response, "account_address"),
            "the account_address attribute should hold the sender's address",
        );
        assert_eq!(
            DEFAULT_CONTRACT_ATTRIBUTE,
            single_attribute_for_key(&response, "attribute_name"),
            "the attribute_name attribute should have the contract's defined attribute name",
        );
        assert_eq!(
            group_id.to_string(),
            single_attribute_for_key(&response, "group_id"),
            "the group_id attribute should have the provided group's id",
        );
    }
}
