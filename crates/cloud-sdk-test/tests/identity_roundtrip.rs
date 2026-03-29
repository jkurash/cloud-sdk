use cloud_sdk_core::services::identity::{IdentityService, PrincipalType};
use cloud_sdk_test::TestHarness;

#[tokio::test]
async fn get_current_principal() {
    let harness = TestHarness::start().await.unwrap();
    let identity = harness.provider().identity();

    let principal = identity.get_current_principal().await.unwrap();
    assert_eq!(principal.display_name, "Mock Service Principal");
    assert_eq!(principal.principal_type, PrincipalType::ServicePrincipal);
    assert!(!principal.id.is_empty());
}

#[tokio::test]
async fn list_role_assignments_empty() {
    let harness = TestHarness::start().await.unwrap();
    let identity = harness.provider().identity();

    let page = identity.list_role_assignments("/").await.unwrap();
    assert!(page.value.is_empty());
}
