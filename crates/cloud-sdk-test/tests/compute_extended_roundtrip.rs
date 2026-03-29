use cloud_sdk_core::services::compute::*;
use cloud_sdk_test::TestHarness;
use std::collections::HashMap;

fn simple_vm_params(location: &str) -> CreateVirtualMachineParams {
    CreateVirtualMachineParams {
        location: location.to_string(),
        properties: VirtualMachineProperties {
            hardware_profile: Some(HardwareProfile {
                vm_size: Some("Standard_D2s_v3".to_string()),
                vm_size_properties: None,
            }),
            storage_profile: Some(StorageProfile {
                os_disk: Some(OsDisk {
                    create_option: "FromImage".to_string(),
                    name: None,
                    caching: None,
                    managed_disk: None,
                    os_type: None,
                    disk_size_gb: None,
                    write_accelerator_enabled: None,
                    image: None,
                    vhd: None,
                    encryption_settings: None,
                    delete_option: None,
                    diff_disk_settings: None,
                }),
                image_reference: None,
                data_disks: None,
                disk_controller_type: None,
            }),
            network_profile: Some(NetworkProfile {
                network_interfaces: Some(vec![]),
                network_interface_configurations: None,
                network_api_version: None,
            }),
            vm_id: None,
            provisioning_state: None,
            os_profile: None,
            security_profile: None,
            diagnostics_profile: None,
            availability_set: None,
            virtual_machine_scale_set: None,
            proximity_placement_group: None,
            host_group: None,
            host: None,
            license_type: None,
            time_created: None,
            additional_capabilities: None,
            billing_profile: None,
            eviction_policy: None,
            priority: None,
            scheduled_events_profile: None,
            user_data: None,
            capacity_reservation: None,
            application_profile: None,
            extensions_time_budget: None,
            instance_view: None,
            platform_fault_domain: None,
            scheduled_events_policy: None,
        },
        tags: HashMap::new(),
        zones: None,
        identity: None,
        extended_location: None,
        plan: None,
        placement: None,
    }
}

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

#[tokio::test]
async fn update_vm_via_patch() {
    let harness = setup().await;
    let compute = harness.provider().compute();

    compute
        .create_virtual_machine("rg1", "patch-vm", simple_vm_params("eastus"))
        .await
        .unwrap();

    let updated = compute
        .update_virtual_machine(
            "rg1",
            "patch-vm",
            serde_json::json!({ "tags": { "env": "production" } }),
        )
        .await
        .unwrap();

    assert_eq!(updated.tags.get("env").unwrap(), "production");
    assert_eq!(updated.name, "patch-vm");
}

#[tokio::test]
async fn list_all_vms_across_rgs() {
    let harness = setup().await;
    let compute = harness.provider().compute();

    compute
        .create_virtual_machine("rg1", "vm-east", simple_vm_params("eastus"))
        .await
        .unwrap();
    compute
        .create_virtual_machine("rg2", "vm-west", simple_vm_params("westus"))
        .await
        .unwrap();

    let page = compute.list_all_virtual_machines().await.unwrap();
    assert_eq!(page.value.len(), 2);
}

#[tokio::test]
async fn list_vms_by_location() {
    let harness = setup().await;
    let compute = harness.provider().compute();

    compute
        .create_virtual_machine("rg1", "vm-east", simple_vm_params("eastus"))
        .await
        .unwrap();
    compute
        .create_virtual_machine("rg2", "vm-west", simple_vm_params("westus"))
        .await
        .unwrap();

    let page = compute
        .list_virtual_machines_by_location("eastus")
        .await
        .unwrap();
    assert_eq!(page.value.len(), 1);
    assert_eq!(page.value[0].name, "vm-east");
}

#[tokio::test]
async fn list_available_sizes() {
    let harness = setup().await;
    let compute = harness.provider().compute();

    compute
        .create_virtual_machine("rg1", "size-vm", simple_vm_params("eastus"))
        .await
        .unwrap();

    let result = compute
        .list_available_sizes("rg1", "size-vm")
        .await
        .unwrap();
    assert!(!result.value.is_empty());
    assert!(result.value.iter().any(|s| s.name == "Standard_D2s_v3"));
    assert!(result.value[0].number_of_cores > 0);
    assert!(result.value[0].memory_in_mb > 0);
}

#[tokio::test]
async fn generalize_vm() {
    let harness = setup().await;
    let compute = harness.provider().compute();

    compute
        .create_virtual_machine("rg1", "gen-vm", simple_vm_params("eastus"))
        .await
        .unwrap();

    compute
        .generalize_virtual_machine("rg1", "gen-vm")
        .await
        .unwrap();
}

#[tokio::test]
async fn reapply_vm() {
    let harness = setup().await;
    let compute = harness.provider().compute();

    compute
        .create_virtual_machine("rg1", "reapply-vm", simple_vm_params("eastus"))
        .await
        .unwrap();

    compute
        .reapply_virtual_machine("rg1", "reapply-vm")
        .await
        .unwrap();
}

#[tokio::test]
async fn simulate_eviction_vm() {
    let harness = setup().await;
    let compute = harness.provider().compute();

    compute
        .create_virtual_machine("rg1", "spot-vm", simple_vm_params("eastus"))
        .await
        .unwrap();

    compute.simulate_eviction("rg1", "spot-vm").await.unwrap();
}

#[tokio::test]
async fn redeploy_vm() {
    let harness = setup().await;
    let compute = harness.provider().compute();

    compute
        .create_virtual_machine("rg1", "redeploy-vm", simple_vm_params("eastus"))
        .await
        .unwrap();

    compute
        .redeploy_virtual_machine("rg1", "redeploy-vm")
        .await
        .unwrap();
}

#[tokio::test]
async fn reimage_vm() {
    let harness = setup().await;
    let compute = harness.provider().compute();

    compute
        .create_virtual_machine("rg1", "reimage-vm", simple_vm_params("eastus"))
        .await
        .unwrap();

    compute
        .reimage_virtual_machine("rg1", "reimage-vm")
        .await
        .unwrap();
}
