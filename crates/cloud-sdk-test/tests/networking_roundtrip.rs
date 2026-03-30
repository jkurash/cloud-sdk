use cloud_sdk_core::models::resource::CreateResourceGroupParams;
use cloud_sdk_core::services::ResourceManagerService;
use cloud_sdk_core::services::networking::*;
use cloud_sdk_test::TestHarness;
use std::collections::HashMap;

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

fn vnet_params() -> CreateVirtualNetworkParams {
    CreateVirtualNetworkParams {
        location: "eastus".to_string(),
        properties: VirtualNetworkProperties {
            address_space: AddressSpace {
                address_prefixes: vec!["10.0.0.0/16".to_string()],
            },
            subnets: vec![],
            provisioning_state: None,
            enable_ddos_protection: None,
            enable_vm_protection: None,
            resource_guid: None,
            flow_timeout_in_minutes: None,
            encryption: None,
        },
        tags: HashMap::new(),
    }
}

// ── VNet tests ─────────────────────────────────────────────────────────

#[tokio::test]
async fn create_and_get_vnet() {
    let harness = setup().await;
    let net = harness.provider().networking();

    let vnet = net
        .create_virtual_network("test-rg", "my-vnet", vnet_params())
        .await
        .unwrap();

    assert_eq!(vnet.name, "my-vnet");
    assert_eq!(vnet.resource_type, "Microsoft.Network/virtualNetworks");
    assert_eq!(
        vnet.properties.address_space.address_prefixes,
        vec!["10.0.0.0/16"]
    );
    assert_eq!(
        vnet.properties.provisioning_state.as_deref(),
        Some("Succeeded")
    );

    let fetched = net.get_virtual_network("test-rg", "my-vnet").await.unwrap();
    assert_eq!(fetched.name, "my-vnet");
}

#[tokio::test]
async fn list_and_delete_vnets() {
    let harness = setup().await;
    let net = harness.provider().networking();

    net.create_virtual_network("test-rg", "vnet-1", vnet_params())
        .await
        .unwrap();
    net.create_virtual_network("test-rg", "vnet-2", vnet_params())
        .await
        .unwrap();

    let page = net.list_virtual_networks("test-rg").await.unwrap();
    assert_eq!(page.value.len(), 2);

    net.delete_virtual_network("test-rg", "vnet-1")
        .await
        .unwrap();

    let page = net.list_virtual_networks("test-rg").await.unwrap();
    assert_eq!(page.value.len(), 1);
}

// ── Subnet tests ───────────────────────────────────────────────────────

#[tokio::test]
async fn subnet_lifecycle() {
    let harness = setup().await;
    let net = harness.provider().networking();

    net.create_virtual_network("test-rg", "my-vnet", vnet_params())
        .await
        .unwrap();

    // Create subnet
    let subnet = net
        .create_subnet(
            "test-rg",
            "my-vnet",
            "default",
            CreateSubnetParams {
                properties: SubnetProperties {
                    address_prefix: "10.0.1.0/24".to_string(),
                    network_security_group: None,
                    provisioning_state: None,
                    service_endpoints: None,
                    delegations: None,
                    private_endpoint_network_policies: None,
                    private_link_service_network_policies: None,
                    nat_gateway: None,
                },
            },
        )
        .await
        .unwrap();

    assert_eq!(subnet.name, "default");
    assert_eq!(subnet.properties.address_prefix, "10.0.1.0/24");
    assert_eq!(
        subnet.properties.provisioning_state.as_deref(),
        Some("Succeeded")
    );

    // Get subnet
    let fetched = net
        .get_subnet("test-rg", "my-vnet", "default")
        .await
        .unwrap();
    assert_eq!(fetched.name, "default");

    // List subnets
    let page = net.list_subnets("test-rg", "my-vnet").await.unwrap();
    assert_eq!(page.value.len(), 1);

    // Delete
    net.delete_subnet("test-rg", "my-vnet", "default")
        .await
        .unwrap();

    let page = net.list_subnets("test-rg", "my-vnet").await.unwrap();
    assert!(page.value.is_empty());
}

#[tokio::test]
async fn subnets_appear_in_vnet_get() {
    let harness = setup().await;
    let net = harness.provider().networking();

    net.create_virtual_network("test-rg", "my-vnet", vnet_params())
        .await
        .unwrap();

    net.create_subnet(
        "test-rg",
        "my-vnet",
        "subnet-a",
        CreateSubnetParams {
            properties: SubnetProperties {
                address_prefix: "10.0.1.0/24".to_string(),
                network_security_group: None,
                provisioning_state: None,
                service_endpoints: None,
                delegations: None,
                private_endpoint_network_policies: None,
                private_link_service_network_policies: None,
                nat_gateway: None,
            },
        },
    )
    .await
    .unwrap();

    // GET vnet should include subnets
    let vnet = net.get_virtual_network("test-rg", "my-vnet").await.unwrap();
    assert_eq!(vnet.properties.subnets.len(), 1);
    assert_eq!(vnet.properties.subnets[0].name, "subnet-a");
}

// ── NSG tests ──────────────────────────────────────────────────────────

#[tokio::test]
async fn nsg_lifecycle() {
    let harness = setup().await;
    let net = harness.provider().networking();

    let nsg = net
        .create_network_security_group(
            "test-rg",
            "my-nsg",
            CreateNsgParams {
                location: "eastus".to_string(),
                properties: NsgProperties {
                    security_rules: vec![SecurityRule {
                        id: None,
                        name: "allow-ssh".to_string(),
                        etag: None,
                        resource_type: None,
                        properties: SecurityRuleProperties {
                            description: None,
                            protocol: "Tcp".to_string(),
                            source_address_prefix: "*".to_string(),
                            destination_address_prefix: "*".to_string(),
                            source_port_range: "*".to_string(),
                            destination_port_range: "22".to_string(),
                            source_port_ranges: None,
                            destination_port_ranges: None,
                            source_address_prefixes: None,
                            destination_address_prefixes: None,
                            access: "Allow".to_string(),
                            direction: "Inbound".to_string(),
                            priority: 100,
                            provisioning_state: None,
                        },
                    }],
                    default_security_rules: None,
                    resource_guid: None,
                    provisioning_state: None,
                },
                tags: HashMap::new(),
            },
        )
        .await
        .unwrap();

    assert_eq!(nsg.name, "my-nsg");
    assert_eq!(nsg.properties.security_rules.len(), 1);
    assert_eq!(nsg.properties.security_rules[0].name, "allow-ssh");

    // Get
    let fetched = net
        .get_network_security_group("test-rg", "my-nsg")
        .await
        .unwrap();
    assert_eq!(
        fetched.properties.security_rules[0].properties.priority,
        100
    );

    // List
    let page = net.list_network_security_groups("test-rg").await.unwrap();
    assert_eq!(page.value.len(), 1);

    // Delete
    net.delete_network_security_group("test-rg", "my-nsg")
        .await
        .unwrap();

    let page = net.list_network_security_groups("test-rg").await.unwrap();
    assert!(page.value.is_empty());
}
