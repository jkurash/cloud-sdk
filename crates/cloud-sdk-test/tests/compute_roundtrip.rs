use cloud_sdk_core::models::resource::CreateResourceGroupParams;
use cloud_sdk_core::services::ResourceManagerService;
use cloud_sdk_core::services::compute::*;
use cloud_sdk_test::TestHarness;
use std::collections::HashMap;

fn vm_params() -> CreateVirtualMachineParams {
    CreateVirtualMachineParams {
        location: "eastus".to_string(),
        properties: VirtualMachineProperties {
            vm_id: None,
            provisioning_state: None,
            hardware_profile: Some(HardwareProfile {
                vm_size: Some("Standard_D2s_v3".to_string()),
                vm_size_properties: None,
            }),
            storage_profile: Some(StorageProfile {
                image_reference: Some(ImageReference {
                    id: None,
                    publisher: Some("Canonical".to_string()),
                    offer: Some("UbuntuServer".to_string()),
                    sku: Some("18.04-LTS".to_string()),
                    version: Some("latest".to_string()),
                    exact_version: None,
                    shared_gallery_image_id: None,
                    community_gallery_image_id: None,
                }),
                os_disk: Some(OsDisk {
                    name: Some("osdisk".to_string()),
                    create_option: "FromImage".to_string(),
                    caching: Some("ReadWrite".to_string()),
                    managed_disk: Some(ManagedDiskParameters {
                        storage_account_type: Some("Premium_LRS".to_string()),
                        id: None,
                        disk_encryption_set: None,
                        security_profile: None,
                    }),
                    os_type: None,
                    disk_size_gb: None,
                    write_accelerator_enabled: None,
                    image: None,
                    vhd: None,
                    encryption_settings: None,
                    delete_option: None,
                    diff_disk_settings: None,
                }),
                data_disks: None,
                disk_controller_type: None,
            }),
            os_profile: Some(OsProfile {
                computer_name: Some("testvm".to_string()),
                admin_username: Some("azureuser".to_string()),
                admin_password: None,
                custom_data: None,
                linux_configuration: Some(LinuxConfiguration {
                    disable_password_authentication: Some(true),
                    ssh: None,
                    provision_vm_agent: None,
                    patch_settings: None,
                    enable_vm_agent_platform_updates: None,
                }),
                windows_configuration: None,
                secrets: None,
                allow_extension_operations: None,
                require_guest_provision_signal: None,
            }),
            network_profile: Some(NetworkProfile {
                network_interfaces: Some(vec![NetworkInterfaceReference {
                    id: Some("/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/test-rg/providers/Microsoft.Network/networkInterfaces/myNIC".to_string()),
                    properties: Some(NetworkInterfaceReferenceProperties { primary: Some(true), delete_option: None }),
                }]),
                network_interface_configurations: None,
                network_api_version: None,
            }),
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
    let harness = TestHarness::start().await.unwrap();
    harness
        .provider()
        .resource_manager()
        .create_resource_group(
            "test-rg",
            CreateResourceGroupParams {
                location: "eastus".to_string(),
                tags: HashMap::new(),
            },
        )
        .await
        .unwrap();
    harness
}

#[tokio::test]
async fn create_and_get_vm() {
    let harness = setup().await;
    let compute = harness.provider().compute();

    let vm = compute
        .create_virtual_machine("test-rg", "my-vm", vm_params())
        .await
        .unwrap();

    assert_eq!(vm.name, "my-vm");
    assert_eq!(vm.location, "eastus");
    assert_eq!(vm.resource_type, "Microsoft.Compute/virtualMachines");
    assert!(vm.properties.vm_id.is_some());
    assert_eq!(
        vm.properties.provisioning_state.as_deref(),
        Some("Succeeded")
    );
    assert_eq!(
        vm.properties
            .hardware_profile
            .as_ref()
            .unwrap()
            .vm_size
            .as_deref(),
        Some("Standard_D2s_v3")
    );

    // Fetch it back
    let fetched = compute
        .get_virtual_machine("test-rg", "my-vm")
        .await
        .unwrap();
    assert_eq!(fetched.name, "my-vm");
    assert_eq!(
        fetched
            .properties
            .hardware_profile
            .as_ref()
            .unwrap()
            .vm_size
            .as_deref(),
        Some("Standard_D2s_v3")
    );
}

#[tokio::test]
async fn list_virtual_machines() {
    let harness = setup().await;
    let compute = harness.provider().compute();

    compute
        .create_virtual_machine("test-rg", "vm-1", vm_params())
        .await
        .unwrap();
    compute
        .create_virtual_machine("test-rg", "vm-2", vm_params())
        .await
        .unwrap();

    let page = compute.list_virtual_machines("test-rg").await.unwrap();
    assert_eq!(page.value.len(), 2);
}

#[tokio::test]
async fn delete_virtual_machine() {
    let harness = setup().await;
    let compute = harness.provider().compute();

    compute
        .create_virtual_machine("test-rg", "del-vm", vm_params())
        .await
        .unwrap();

    compute
        .delete_virtual_machine("test-rg", "del-vm")
        .await
        .unwrap();

    let result = compute.get_virtual_machine("test-rg", "del-vm").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn vm_power_operations() {
    let harness = setup().await;
    let compute = harness.provider().compute();

    compute
        .create_virtual_machine("test-rg", "power-vm", vm_params())
        .await
        .unwrap();

    // Stop
    compute
        .stop_virtual_machine("test-rg", "power-vm")
        .await
        .unwrap();

    // Start
    compute
        .start_virtual_machine("test-rg", "power-vm")
        .await
        .unwrap();

    // Restart
    compute
        .restart_virtual_machine("test-rg", "power-vm")
        .await
        .unwrap();

    // Deallocate
    compute
        .deallocate_virtual_machine("test-rg", "power-vm")
        .await
        .unwrap();

    // VM should still exist after power operations
    let vm = compute
        .get_virtual_machine("test-rg", "power-vm")
        .await
        .unwrap();
    assert_eq!(vm.name, "power-vm");
}

#[tokio::test]
async fn get_nonexistent_vm_returns_error() {
    let harness = setup().await;
    let compute = harness.provider().compute();

    let result = compute.get_virtual_machine("test-rg", "nope").await;
    assert!(result.is_err());
}
