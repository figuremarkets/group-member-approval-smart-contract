use crate::store::contract_state::get_contract_state;
use crate::types::core::error::ContractError;
use crate::util::prov_helpers::get_group_id_attribute_values;
use crate::util::route_helpers::check_funds_are_empty;
use cosmwasm_std::{to_binary, DepsMut, MessageInfo, Response, Uint64};
use provwasm_std::{
    add_attribute, AttributeValueType, ProvenanceMsg, ProvenanceQuerier, ProvenanceQuery,
};
use result_extensions::ResultExtensions;

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
        .unwrap_or(vec![]);
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
    use crate::test::test_constants::DEFAULT_CONTRACT_ADMIN;
    use crate::test::test_instantiate::test_instantiate;
    use crate::types::core::error::ContractError;
    use cosmwasm_std::testing::mock_info;
    use cosmwasm_std::{coins, Uint64};
    use provwasm_mocks::mock_dependencies;

    #[test]
    fn test_rejection_for_provided_funds() {
        let mut deps = mock_dependencies(&[]);
        test_instantiate(deps.as_mut());
        let info = mock_info(DEFAULT_CONTRACT_ADMIN, &coins(15, "nhash"));
        let err = approve_group_membership(deps.as_mut(), info, Uint64::new(1))
            .expect_err("an error should occur when the sender provides funds");
        assert!(
            matches!(err, ContractError::InvalidFundsError { .. }),
            "an invalid funds error should be emitted when the sender provides funds",
        );
    }
}
