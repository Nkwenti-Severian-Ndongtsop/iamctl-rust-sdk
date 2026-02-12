use iamctl_rust_sdk::prelude::*;
use proptest::prelude::*;
use serde_json::json;
use std::collections::HashMap;

proptest! {
    #[test]
    fn test_resource_address_properties(
        resource_type in "[a-z0-9-]+",
        name in "[a-z0-9-]+",
        namespace in prop::option::weighted(0.1, "[a-z0-9-]+")
    ) {
        let address = ResourceAddress {
            resource_type: resource_type.clone(),
            name: name.clone(),
            namespace: namespace.clone(),
        };

        let serialized = serde_json::to_value(&address).unwrap();
        let deserialized: ResourceAddress = serde_json::from_value(serialized).unwrap();

        assert_eq!(deserialized.resource_type, resource_type);
        assert_eq!(deserialized.name, name);
        assert_eq!(deserialized.namespace, namespace);
    }

    #[test]
    fn test_change_type_properties(change_type_idx in 0..3u8) {
        let change_type = match change_type_idx {
            0 => ChangeType::Create,
            1 => ChangeType::Update,
            _ => ChangeType::Delete,
        };
        let serialized = serde_json::to_value(&change_type).unwrap();
        assert!(serialized.is_string());
    }

    #[test]
    fn test_resource_spec_valid_json(
        key in "[a-z]+",
        val in "[a-z]+"
    ) {
        let address = ResourceAddress {
            resource_type: "test".to_string(),
            name: "test-resource".to_string(),
            namespace: None,
        };

        let mut spec = HashMap::new();
        spec.insert(key.clone(), json!(val));

        let resource = Resource {
            address,
            spec,
            metadata: HashMap::new(),
        };

        let serialized = serde_json::to_value(&resource).unwrap();
        assert!(serialized.is_object());
    }
}
