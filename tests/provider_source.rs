use std::collections::HashMap;

use iamctl_rust_sdk::types::{Resource, ResourceAddress};
use iamctl_rust_sdk::utils::provider_source::GithubProviderSource;
use iamctl_rust_sdk::utils::{decode_spec, provider_source::derive_github_release_tag};

#[test]
fn parses_github_source_without_subdir() {
    let src = GithubProviderSource::parse("github:acme/iamctl-providers").unwrap();
    assert_eq!(src.owner, "acme");
    assert_eq!(src.repo, "iamctl-providers");
    assert_eq!(src.subdir, None);
    assert_eq!(src.to_string(), "github:acme/iamctl-providers");
}

#[test]
fn parses_github_source_with_subdir() {
    let src = GithubProviderSource::parse("github:acme/iamctl-providers//keycloak").unwrap();
    assert_eq!(src.owner, "acme");
    assert_eq!(src.repo, "iamctl-providers");
    assert_eq!(src.subdir.as_deref(), Some("keycloak"));
    assert_eq!(src.to_string(), "github:acme/iamctl-providers//keycloak");
}

#[test]
fn derives_provider_release_tag() {
    assert_eq!(
        derive_github_release_tag("keycloak", "0.1.0").unwrap(),
        "keycloak-v0.1.0"
    );
    assert_eq!(
        derive_github_release_tag("keycloak", "v0.1.0").unwrap(),
        "keycloak-v0.1.0"
    );
}

#[derive(serde::Deserialize, Debug, PartialEq)]
struct DemoSpec {
    realm: String,
    enabled: bool,
}

#[test]
fn decodes_spec_with_context() {
    let mut spec = HashMap::new();
    spec.insert(
        "realm".to_string(),
        serde_json::Value::String("demo".to_string()),
    );
    spec.insert("enabled".to_string(), serde_json::Value::Bool(true));

    let resource = Resource {
        address: ResourceAddress {
            resource_type: "realm".to_string(),
            name: "demo".to_string(),
            namespace: None,
        },
        spec,
        metadata: HashMap::new(),
    };

    let decoded: DemoSpec = decode_spec(&resource).unwrap();
    assert_eq!(
        decoded,
        DemoSpec {
            realm: "demo".to_string(),
            enabled: true
        }
    );
}
