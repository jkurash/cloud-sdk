use cloud_sdk_core::models::resource::CreateResourceGroupParams;
use cloud_sdk_core::services::ResourceManagerService;
use cloud_sdk_test::TestHarness;
use std::collections::HashMap;

#[tokio::test]
async fn list_subscriptions() {
    let harness = TestHarness::start().await.unwrap();
    let rm = harness.provider().resource_manager();

    let page = rm.list_subscriptions().await.unwrap();
    assert_eq!(page.value.len(), 1);
    assert_eq!(
        page.value[0].subscription_id,
        "00000000-0000-0000-0000-000000000000"
    );
    assert_eq!(page.value[0].display_name, "Mock Subscription");
}

#[tokio::test]
async fn get_subscription() {
    let harness = TestHarness::start().await.unwrap();
    let rm = harness.provider().resource_manager();

    let sub = rm
        .get_subscription("00000000-0000-0000-0000-000000000000")
        .await
        .unwrap();
    assert_eq!(sub.display_name, "Mock Subscription");
}

#[tokio::test]
async fn create_and_get_resource_group() {
    let harness = TestHarness::start().await.unwrap();
    let rm = harness.provider().resource_manager();

    let params = CreateResourceGroupParams {
        location: "eastus".to_string(),
        tags: HashMap::new(),
    };

    let rg = rm.create_resource_group("test-rg", params).await.unwrap();
    assert_eq!(rg.name, "test-rg");
    assert_eq!(rg.location, "eastus");
    assert_eq!(rg.resource_type, "Microsoft.Resources/resourceGroups");

    // Fetch it back
    let fetched = rm.get_resource_group("test-rg").await.unwrap();
    assert_eq!(fetched.name, "test-rg");
    assert_eq!(fetched.location, "eastus");
}

#[tokio::test]
async fn list_resource_groups() {
    let harness = TestHarness::start().await.unwrap();
    let rm = harness.provider().resource_manager();

    // Create two
    for name in &["rg-a", "rg-b"] {
        rm.create_resource_group(
            name,
            CreateResourceGroupParams {
                location: "westus".to_string(),
                tags: HashMap::new(),
            },
        )
        .await
        .unwrap();
    }

    let page = rm.list_resource_groups().await.unwrap();
    assert_eq!(page.value.len(), 2);
}

#[tokio::test]
async fn delete_resource_group() {
    let harness = TestHarness::start().await.unwrap();
    let rm = harness.provider().resource_manager();

    rm.create_resource_group(
        "del-rg",
        CreateResourceGroupParams {
            location: "eastus".to_string(),
            tags: HashMap::new(),
        },
    )
    .await
    .unwrap();

    rm.delete_resource_group("del-rg").await.unwrap();

    // Should be gone
    let result = rm.get_resource_group("del-rg").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn resource_group_exists() {
    let harness = TestHarness::start().await.unwrap();
    let rm = harness.provider().resource_manager();

    assert!(!rm.resource_group_exists("nope").await.unwrap());

    rm.create_resource_group(
        "exists-rg",
        CreateResourceGroupParams {
            location: "eastus".to_string(),
            tags: HashMap::new(),
        },
    )
    .await
    .unwrap();

    assert!(rm.resource_group_exists("exists-rg").await.unwrap());
}

#[tokio::test]
async fn get_nonexistent_resource_group_returns_error() {
    let harness = TestHarness::start().await.unwrap();
    let rm = harness.provider().resource_manager();

    let result = rm.get_resource_group("nonexistent").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn create_resource_group_with_tags() {
    let harness = TestHarness::start().await.unwrap();
    let rm = harness.provider().resource_manager();

    let mut tags = HashMap::new();
    tags.insert("env".to_string(), "dev".to_string());
    tags.insert("team".to_string(), "platform".to_string());

    let rg = rm
        .create_resource_group(
            "tagged-rg",
            CreateResourceGroupParams {
                location: "northeurope".to_string(),
                tags,
            },
        )
        .await
        .unwrap();

    assert_eq!(rg.tags.get("env").unwrap(), "dev");
    assert_eq!(rg.tags.get("team").unwrap(), "platform");
    assert_eq!(rg.location, "northeurope");
}
