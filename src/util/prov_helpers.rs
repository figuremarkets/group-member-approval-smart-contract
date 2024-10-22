use cosmwasm_std::{from_json, CustomQuery, Uint64};
use provwasm_std::types::{
    cosmos::base::query::v1beta1::{PageRequest, PageResponse},
    provenance::{
        attribute::v1::{AttributeQuerier, AttributeType, QueryAttributesResponse},
        name::v1::{MsgBindNameRequest, NameRecord},
    },
};
use result_extensions::ResultExtensions;

use crate::types::core::error::ContractError;

/// Parses all group ids from the [Provenance Blockchain Attributes](https://docs.provenance.io/modules/account)
/// provided by filtering for all values that match the given name and have an assigned int value.
///
/// # Parameters
///
/// * `attributes` Pages of Attributes fetched via a chain query.
/// * `name` A [Provenance Blockchain Name Module](https://docs.provenance.io/modules/name-module)
/// name used to write the attribute.
pub fn get_group_id_attribute_values_paginated<S: Into<String>>(
    attributes: Vec<QueryAttributesResponse>,
    name: S,
) -> Vec<Uint64> {
    let name = name.into();
    attributes
        .iter()
        .flat_map(|page| &page.attributes)
        .filter(|attr| attr.name == name && attr.attribute_type() == AttributeType::Int)
        .filter_map(|attr| from_json::<u64>(&attr.value).ok())
        .map(Uint64::new)
        .collect()
}

/// Parses all group ids from the [Provenance Blockchain Attributes](https://docs.provenance.io/modules/account)
/// provided by filtering for all values that match the given name and have an assigned int value.
///
/// # Parameters
///
/// * `attributes` Attributes fetched via a chain query.
/// * `name` A [Provenance Blockchain Name Module](https://docs.provenance.io/modules/name-module)
/// name used to write the attribute.
pub fn get_group_id_attribute_values<S: Into<String>>(
    attributes: &QueryAttributesResponse,
    name: S,
) -> Vec<Uint64> {
    get_group_id_attribute_values_paginated(vec![attributes.clone()], name)
}
/// Generates a [name bind msg](MsgBindNameRequest) that will properly assign the given name value
/// to a target address.  Assumes the parent name is unrestricted or that the contract has access to
/// bind a name to the parent name.
///
/// # Parameters
/// * `name` The dot-qualified name to use on-chain for name binding. Ex: myname.sc.pb will generate
/// a msg that binds "myname" to the existing parent name "sc.pb".
/// * `bind_to_address` The bech32 address to which the name will be bound.
/// * `restricted` If true, the name will be bound as a restricted name, preventing future name
/// bindings from using it as a parent name.
pub fn msg_bind_name<S1: Into<String>, S2: Into<String>>(
    name: S1,
    bind_to_address: S2,
    restricted: bool,
) -> Result<MsgBindNameRequest, ContractError> {
    let fully_qualified_name = name.into();
    let mut name_parts = fully_qualified_name.split('.').collect::<Vec<&str>>();
    let bind_address = bind_to_address.into();
    let bind_record = if let Some(bind) = name_parts.to_owned().first() {
        if bind.is_empty() {
            return ContractError::InvalidFormatError {
                message: format!(
                    "cannot bind to an empty name string [{}]",
                    fully_qualified_name
                ),
            }
            .to_err();
        }
        Some(NameRecord {
            name: bind.to_string(),
            address: bind_address.to_owned(),
            restricted,
        })
    } else {
        return ContractError::InvalidFormatError {
            message: format!(
                "cannot derive bind name from input [{}]",
                fully_qualified_name
            ),
        }
        .to_err();
    };
    let parent_record = if name_parts.len() > 1 {
        // Trim the first element, because that is the new name to be bound
        name_parts.remove(0);
        let parent_name = name_parts.join(".").to_string();
        Some(NameRecord {
            name: parent_name.to_owned(),
            // The parent record must also use the address being bound to as its address in order for
            // the bind to succeed.  This is the only way in which Provenance accepts a non-restricted
            // name bind
            address: bind_address,
            restricted: false,
        })
    } else {
        None
    };
    MsgBindNameRequest {
        record: bind_record,
        parent: parent_record,
    }
    .to_ok()
}

fn build_page_request(key: Vec<u8>) -> Option<PageRequest> {
    Some(PageRequest {
        key: key,
        offset: 0,
        limit: 100,
        count_total: false,
        reverse: false,
    })
}

/// Fetches all attributes for an address, handling paging as appropriate
/// 
/// # Parameters
/// * `querier` The Provenance Blockchain AttributeQuerier to use for fetching pages of attributes
/// * `address` The address to fetch attributes on
pub fn get_all_attributes<Q: CustomQuery, S1: Into<String> + Copy>(
    querier: AttributeQuerier<Q>,
    address: S1,
) -> Result<Vec<QueryAttributesResponse>, ContractError> {
    let mut results = vec![];
    let mut pagination = build_page_request(vec![]);
    loop {
        let res = querier.attributes(address.into(), pagination.clone())?;
        results.push(res.clone());
        match res.pagination {
            Some(PageResponse {
                next_key: Some(..), ..
            }) => pagination = build_page_request(res.pagination.unwrap().next_key.unwrap()),
            Some(PageResponse { next_key: None, .. }) => break,
            None => break,
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use crate::{
        test::test_constants::{DEFAULT_CONTRACT_ATTRIBUTE, DEFAULT_GROUP_MEMBER},
        types::core::error::ContractError,
        util::prov_helpers::{get_group_id_attribute_values, msg_bind_name},
    };
    use cosmwasm_std::{from_json, to_json_vec, Binary, ContractResult, SystemResult};
    use provwasm_mocks::{mock_provenance_dependencies, MockProvenanceQuerier};
    use provwasm_std::types::{
        cosmos::base::query::v1beta1::{PageRequest, PageResponse},
        provenance::attribute::v1::{
            Attribute, AttributeQuerier, AttributeType, QueryAttributesRequest,
            QueryAttributesResponse,
        },
        tendermint::abci::ResponseQuery,
    };

    use super::get_all_attributes;

    /// Helper function to handle returning mocked responses with pagination, where the key is a json-encoded string of the next page index
    fn mock_paged_attributes(
        querier: &mut MockProvenanceQuerier,
        account: &'static str,
        pages: Vec<Vec<Attribute>>,
    ) {
        querier.registered_custom_queries.insert(
            "/provenance.attribute.v1.Query/Attributes".into(),
            Box::new(move |binary: &Binary| {
                let request = QueryAttributesRequest::try_from(binary.clone()).unwrap();
                let idx = match request.pagination {
                    None => 0,
                    Some(PageRequest { key, .. }) => {
                        if key.len() == 0 {
                            0
                        } else {
                            from_json(key).unwrap_or(0)
                        }
                    }
                };
                let page = pages.get(idx).expect(&format!(
                    "Expected to be able to fetch page {} of attributes",
                    idx
                ));
                let response = QueryAttributesResponse {
                    account: account.into(),
                    attributes: page.to_vec(),
                    pagination: if idx < pages.len() - 1 {
                        Some(PageResponse {
                            next_key: Some(
                                to_json_vec::<usize>(&(idx + 1))
                                    .expect("Failed to serialize pagination index to vector"),
                            ),
                            total: pages.iter().fold(0, |acc, attrs| acc + attrs.len() as u64),
                        })
                    } else {
                        None
                    },
                };
                SystemResult::Ok(ContractResult::Ok(Binary::new(
                    ResponseQuery {
                        code: 0,
                        log: "".into(),
                        info: "".into(),
                        index: 0,
                        key: vec![],
                        value: response.to_proto_bytes(),
                        proof_ops: None,
                        height: 1234,
                        codespace: "".into(),
                    }
                    .to_proto_bytes(),
                )))
            }),
        );
    }

    #[test]
    fn get_all_attributes_paginates_properly() {
        let mut deps = mock_provenance_dependencies();
        let address = "someaddress";
        mock_paged_attributes(
            &mut deps.querier,
            address,
            vec![
                vec![Attribute {
                    name: DEFAULT_CONTRACT_ATTRIBUTE.into(),
                    value: to_json_vec(&1u64).unwrap(),
                    attribute_type: AttributeType::Int.into(),
                    address: DEFAULT_GROUP_MEMBER.into(),
                    expiration_date: None,
                }],
                vec![Attribute {
                    name: DEFAULT_CONTRACT_ATTRIBUTE.into(),
                    value: to_json_vec(&2u64).unwrap(),
                    attribute_type: AttributeType::Int.into(),
                    address: DEFAULT_GROUP_MEMBER.into(),
                    expiration_date: None,
                }],
            ],
        );
        let all_attributes =
            get_all_attributes(AttributeQuerier::new(&deps.as_mut().querier), address)
                .expect("expected get_all_attributes to successfully respond");
        assert_eq!(
            2,
            all_attributes.len(),
            "expected two pages for all attributes"
        );
        let mut attribute_values = all_attributes
            .iter()
            .flat_map(|attrs| attrs.attributes.to_owned())
            .map(|attr| from_json::<u64>(attr.value).expect("Failed to parse value to integer"))
            .collect::<Vec<_>>();
        attribute_values.sort();
        assert_eq!(
            vec![1u64, 2u64],
            attribute_values,
            "Expected all attributes to be present"
        )
    }

    #[test]
    fn test_get_group_id_attribute_values_no_attributes() {
        let attributes = QueryAttributesResponse {
            account: "whatever".to_string(),
            attributes: vec![],
            pagination: None,
        };
        let resulting_values = get_group_id_attribute_values(&attributes, "some name");
        assert!(
            resulting_values.is_empty(),
            "no values should be derived from empty attributes, but got: {:?}",
            resulting_values,
        );
    }

    #[test]
    fn test_get_group_id_attribute_values_single_matching_attribute() {
        let attributes = QueryAttributesResponse {
            account: "whatever".to_string(),
            attributes: vec![Attribute {
                name: "idk".to_string(),
                value: get_json_vector_int(10),
                attribute_type: AttributeType::Int.into(),
                address: "something".to_string(),
                expiration_date: None,
            }],
            pagination: None,
        };
        let resulting_values = get_group_id_attribute_values(&attributes, "idk");
        assert_eq!(
            1,
            resulting_values.len(),
            "expected a single result to be derived, but got: {:?}",
            resulting_values,
        );
        assert_eq!(
            10,
            resulting_values.first().unwrap().u64(),
            "the single value in the resulting values should be correctly derived",
        );
    }

    #[test]
    fn test_get_group_id_attribute_values_multiple_matching_attributes() {
        let attributes = QueryAttributesResponse {
            account: "whatever".to_string(),
            attributes: vec![
                Attribute {
                    name: "name".to_string(),
                    value: get_json_vector_int(5),
                    attribute_type: AttributeType::Int.into(),
                    address: "something".to_string(),
                    expiration_date: None,
                },
                Attribute {
                    name: "name".to_string(),
                    value: get_json_vector_int(12),
                    attribute_type: AttributeType::Int.into(),
                    address: "something".to_string(),
                    expiration_date: None,
                },
                Attribute {
                    name: "name".to_string(),
                    value: get_json_vector_int(17),
                    attribute_type: AttributeType::Int.into(),
                    address: "something".to_string(),
                    expiration_date: None,
                },
            ],
            pagination: None,
        };
        let resulting_values = get_group_id_attribute_values(&attributes, "name");
        assert_eq!(
            3,
            resulting_values.len(),
            "expected three results to be derived, but got: {:?}",
            resulting_values,
        );
        assert_eq!(
            5,
            resulting_values[0].u64(),
            "the first value should be correctly derived",
        );
        assert_eq!(
            12,
            resulting_values[1].u64(),
            "the second value should be correctly derived",
        );
        assert_eq!(
            17,
            resulting_values[2].u64(),
            "the third value should be correctly derived",
        );
    }

    #[test]
    fn test_get_group_id_attribute_values_all_mismatches() {
        let attributes = QueryAttributesResponse {
            account: "whatever".to_string(),
            attributes: vec![
                // Mismatched because it is a different name than the expected one
                Attribute {
                    name: "somename".to_string(),
                    value: get_json_vector_int(5),
                    attribute_type: AttributeType::Int.into(),
                    address: "something".to_string(),
                    expiration_date: None,
                },
                // Mismatched because the serialized value is not an int
                Attribute {
                    name: "targetname".to_string(),
                    value: to_json_vec("not an int")
                        .expect("string should serialize to binary successfully"),
                    attribute_type: AttributeType::Int.into(),
                    address: "something".to_string(),
                    expiration_date: None,
                },
                // Mismatched because the value type qualifier is set incorrectly
                Attribute {
                    name: "targetname".to_string(),
                    value: get_json_vector_int(11),
                    attribute_type: AttributeType::String.into(),
                    address: "something".to_string(),
                    expiration_date: None,
                },
            ],
            pagination: None,
        };
        let resulting_values = get_group_id_attribute_values(&attributes, "targetname");
        assert!(
            resulting_values.is_empty(),
            "no values should be derived when no proper attribute values are set, but got: {:?}",
            resulting_values,
        );
    }

    #[test]
    fn test_get_group_id_attribute_values_some_matches_some_mismatches() {
        let attributes = QueryAttributesResponse {
            account: "whatever".to_string(),
            attributes: vec![
                // Mismatch on name
                Attribute {
                    name: "othername".to_string(),
                    value: get_json_vector_int(1),
                    attribute_type: AttributeType::Int.into(),
                    address: "something".to_string(),
                    expiration_date: None,
                },
                // Match 1
                Attribute {
                    name: "targetname".to_string(),
                    value: get_json_vector_int(2),
                    attribute_type: AttributeType::Int.into(),
                    address: "something".to_string(),
                    expiration_date: None,
                },
                // Mismatch on value type
                Attribute {
                    name: "targetname".to_string(),
                    value: get_json_vector_int(3),
                    attribute_type: AttributeType::Json.into(),
                    address: "something".to_string(),
                    expiration_date: None,
                },
                // Match 2
                Attribute {
                    name: "targetname".to_string(),
                    value: get_json_vector_int(4),
                    attribute_type: AttributeType::Int.into(),
                    address: "something".to_string(),
                    expiration_date: None,
                },
            ],
            pagination: None,
        };
        let resulting_values = get_group_id_attribute_values(&attributes, "targetname");
        assert_eq!(
            2,
            resulting_values.len(),
            "the resulting vector should contain two values, but got: {:?}",
            resulting_values,
        );
        assert_eq!(
            2,
            resulting_values[0].u64(),
            "the first value should be correctly defined",
        );
        assert_eq!(
            4,
            resulting_values[1].u64(),
            "the second value should be correctly defined",
        );
    }

    #[test]
    fn msg_bind_name_creates_proper_binding_with_fully_qualified_name() {
        let name = "test.name.bro";
        let address = "some-address";
        let msg =
            msg_bind_name(name, address, true).expect("valid input should not yield an error");
        let parent = msg.parent.expect("the result should include a parent msg");
        assert_eq!(
            "name.bro", parent.name,
            "parent name should be properly derived",
        );
        assert_eq!(
            address, parent.address,
            "parent address value should be set as the bind address because that's what enables binds to unrestricted parent addresses",
        );
        assert!(
            !parent.restricted,
            "parent restricted should always be false",
        );
        let bind = msg.record.expect("the result should include a name record");
        assert_eq!(
            "test", bind.name,
            "the bound name should be properly derived",
        );
        assert_eq!(
            address, bind.address,
            "the bound name should have the specified address",
        );
        assert!(
            bind.restricted,
            "the restricted value should equate to the value specified",
        );
    }

    #[test]
    fn msg_bind_name_creates_proper_binding_with_single_node_name() {
        let name = "name";
        let address = "address";
        let msg = msg_bind_name(name, address, false)
            .expect("proper input should produce a success result");
        assert!(
            msg.parent.is_none(),
            "the parent record should not be set because the name bind does not require it",
        );
        let bind = msg.record.expect("the result should include a name record");
        assert_eq!(
            "name", bind.name,
            "the bound name should be properly derived",
        );
        assert_eq!(
            address, bind.address,
            "the bound name should have the specified address",
        );
        assert!(
            !bind.restricted,
            "the restricted value should equate to the value specified",
        );
    }

    #[test]
    fn msg_bind_name_should_properly_guard_against_bad_input() {
        let _expected_error_message = "cannot derive bind name from input []".to_string();
        assert!(
            matches!(
                msg_bind_name("", "address", true)
                    .expect_err("an error should occur when no name is specified"),
                ContractError::InvalidFormatError {
                    message: _expected_error_message,
                },
            ),
            "unexpected error message when specifying an empty name",
        );
        let _expected_error_message = "cannot bind to an empty name string [.suffix]".to_string();
        assert!(
            matches!(
                msg_bind_name(".suffix", "address", true)
                    .expect_err("an error should occur when specifying a malformed name"),
                ContractError::InvalidFormatError {
                    message: _expected_error_message,
                },
            ),
            "unexpected error message when specifying a malformed name",
        );
    }

    fn get_json_vector_int(value: u64) -> Vec<u8> {
        to_json_vec(&value).expect(&format!(
            "Expected value [{value}] to be properly converted to binary",
        ))
    }
}
