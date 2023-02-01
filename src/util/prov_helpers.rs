use cosmwasm_std::{from_binary, Uint64};
use provwasm_std::{AttributeValueType, Attributes};

pub fn get_group_id_attribute_values<S: Into<String>>(
    attributes: &Attributes,
    name: S,
) -> Vec<Uint64> {
    let name = name.into();
    attributes
        .attributes
        .iter()
        .filter(|attr| attr.name == name && attr.value_type == AttributeValueType::Int)
        .filter_map(|attr| from_binary::<u64>(&attr.value).ok())
        .map(Uint64::new)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::util::prov_helpers::get_group_id_attribute_values;
    use cosmwasm_std::{to_binary, Addr, Binary};
    use provwasm_std::{Attribute, AttributeValueType, Attributes};

    #[test]
    fn test_get_group_id_attribute_values_no_attributes() {
        let attributes = Attributes {
            address: Addr::unchecked("whatever"),
            attributes: vec![],
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
        let attributes = Attributes {
            address: Addr::unchecked("whatever"),
            attributes: vec![Attribute {
                name: "idk".to_string(),
                value: get_binary_int(10),
                value_type: AttributeValueType::Int,
            }],
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
        let attributes = Attributes {
            address: Addr::unchecked("whatever"),
            attributes: vec![
                Attribute {
                    name: "name".to_string(),
                    value: get_binary_int(5),
                    value_type: AttributeValueType::Int,
                },
                Attribute {
                    name: "name".to_string(),
                    value: get_binary_int(12),
                    value_type: AttributeValueType::Int,
                },
                Attribute {
                    name: "name".to_string(),
                    value: get_binary_int(17),
                    value_type: AttributeValueType::Int,
                },
            ],
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
        let attributes = Attributes {
            address: Addr::unchecked("whatever"),
            attributes: vec![
                // Mismatched because it is a different name than the expected one
                Attribute {
                    name: "somename".to_string(),
                    value: get_binary_int(5),
                    value_type: AttributeValueType::Int,
                },
                // Mismatched because the serialized value is not an int
                Attribute {
                    name: "targetname".to_string(),
                    value: to_binary("not an int")
                        .expect("string should serialize to binary successfully"),
                    value_type: AttributeValueType::Int,
                },
                // Mismatched because the value type qualifier is set incorrectly
                Attribute {
                    name: "targetname".to_string(),
                    value: get_binary_int(11),
                    value_type: AttributeValueType::String,
                },
            ],
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
        let attributes = Attributes {
            address: Addr::unchecked("whatever"),
            attributes: vec![
                // Mismatch on name
                Attribute {
                    name: "othername".to_string(),
                    value: get_binary_int(1),
                    value_type: AttributeValueType::Int,
                },
                // Match 1
                Attribute {
                    name: "targetname".to_string(),
                    value: get_binary_int(2),
                    value_type: AttributeValueType::Int,
                },
                // Mismatch on value type
                Attribute {
                    name: "targetname".to_string(),
                    value: get_binary_int(3),
                    value_type: AttributeValueType::Json,
                },
                // Match 2
                Attribute {
                    name: "targetname".to_string(),
                    value: get_binary_int(4),
                    value_type: AttributeValueType::Int,
                },
            ],
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

    fn get_binary_int(value: u64) -> Binary {
        to_binary(&value).expect(&format!(
            "Expected value [{}] to be properly converted to binary",
            value
        ))
    }
}
