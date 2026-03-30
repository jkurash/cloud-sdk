use cloud_sdk_core::services::networking::*;
use cloud_sdk_test::TestHarness;
use std::collections::HashMap;

async fn setup() -> TestHarness {
    let config = cloud_sdk_azure_mock::AzureMockConfig::from_toml(
        r#"
[server]

[subscriptions.default]
id = "00000000-0000-0000-0000-000000000000"
display_name = "Mock Subscription"
tenant_id = "00000000-0000-0000-0000-000000000001"
state = "Enabled"

[[subscriptions.default.resource_groups]]
name = "rg1"
location = "eastus"

[[subscriptions.default.resource_groups]]
name = "rg2"
location = "westus"
"#,
    )
    .unwrap();
    TestHarness::start_with_config(config).await.unwrap()
}

// ── Application Security Group tests ───────────────────────────────────

#[tokio::test]
async fn asg_lifecycle() {
    let harness = setup().await;
    let net = harness.provider().networking();

    let asg = net
        .create_application_security_group(
            "rg1",
            "my-asg",
            CreateApplicationSecurityGroupParams {
                location: "eastus".to_string(),
                tags: HashMap::new(),
            },
        )
        .await
        .unwrap();

    assert_eq!(asg.name, "my-asg");
    assert_eq!(
        asg.resource_type,
        "Microsoft.Network/applicationSecurityGroups"
    );
    assert!(asg.properties.resource_guid.is_some());
    assert_eq!(
        asg.properties.provisioning_state.as_deref(),
        Some("Succeeded")
    );

    // Get
    let fetched = net
        .get_application_security_group("rg1", "my-asg")
        .await
        .unwrap();
    assert_eq!(fetched.name, "my-asg");

    // List
    let page = net.list_application_security_groups("rg1").await.unwrap();
    assert_eq!(page.value.len(), 1);

    // Delete
    net.delete_application_security_group("rg1", "my-asg")
        .await
        .unwrap();
    let page = net.list_application_security_groups("rg1").await.unwrap();
    assert!(page.value.is_empty());
}

#[tokio::test]
async fn asg_list_all_and_update_tags() {
    let harness = setup().await;
    let net = harness.provider().networking();

    net.create_application_security_group(
        "rg1",
        "asg-1",
        CreateApplicationSecurityGroupParams {
            location: "eastus".to_string(),
            tags: HashMap::new(),
        },
    )
    .await
    .unwrap();
    net.create_application_security_group(
        "rg2",
        "asg-2",
        CreateApplicationSecurityGroupParams {
            location: "westus".to_string(),
            tags: HashMap::new(),
        },
    )
    .await
    .unwrap();

    // List All
    let page = net.list_all_application_security_groups().await.unwrap();
    assert_eq!(page.value.len(), 2);

    // Update Tags
    let mut tags = HashMap::new();
    tags.insert("env".to_string(), "dev".to_string());
    let updated = net
        .update_application_security_group_tags("rg1", "asg-1", tags)
        .await
        .unwrap();
    assert_eq!(updated.tags.get("env").unwrap(), "dev");
}

// ── Service Tags test ──────────────────────────────────────────────────

#[tokio::test]
async fn list_service_tags() {
    let harness = setup().await;
    let net = harness.provider().networking();

    let result = net.list_service_tags("eastus").await.unwrap();
    assert!(result.values.is_some());
    let values = result.values.unwrap();
    assert!(!values.is_empty());

    // Should have common tags
    let names: Vec<&str> = values.iter().filter_map(|v| v.name.as_deref()).collect();
    assert!(names.contains(&"AzureCloud"));
    assert!(names.contains(&"Storage"));
}
