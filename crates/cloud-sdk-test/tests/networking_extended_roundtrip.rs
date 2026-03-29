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

// ── Network Interface tests ────────────────────────────────────────────

#[tokio::test]
async fn create_and_get_nic() {
    let harness = setup().await;
    let net = harness.provider().networking();

    // Create a VNet + subnet first (NIC needs a subnet reference)
    net.create_virtual_network(
        "test-rg",
        "my-vnet",
        CreateVirtualNetworkParams {
            location: "eastus".to_string(),
            properties: VirtualNetworkProperties {
                address_space: AddressSpace {
                    address_prefixes: vec!["10.0.0.0/16".to_string()],
                },
                subnets: vec![],
                provisioning_state: None,
            },
            tags: HashMap::new(),
        },
    )
    .await
    .unwrap();

    net.create_subnet(
        "test-rg",
        "my-vnet",
        "default",
        CreateSubnetParams {
            properties: SubnetProperties {
                address_prefix: "10.0.1.0/24".to_string(),
                network_security_group: None,
                provisioning_state: None,
            },
        },
    )
    .await
    .unwrap();

    let nic = net
        .create_network_interface(
            "test-rg",
            "my-nic",
            CreateNetworkInterfaceParams {
                location: "eastus".to_string(),
                tags: HashMap::new(),
                properties: NetworkInterfaceProperties {
                    provisioning_state: None,
                    ip_configurations: Some(vec![NetworkInterfaceIPConfiguration {
                        id: None,
                        name: Some("ipconfig1".to_string()),
                        etag: None,
                        properties: Some(NetworkInterfaceIPConfigurationProperties {
                            private_ip_address: None,
                            private_ip_allocation_method: Some("Dynamic".to_string()),
                            private_ip_address_version: None,
                            subnet: Some(SubResourceRef {
                                id: "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/test-rg/providers/Microsoft.Network/virtualNetworks/my-vnet/subnets/default".to_string(),
                            }),
                            public_ip_address: None,
                            primary: Some(true),
                            provisioning_state: None,
                        }),
                    }]),
                    dns_settings: None,
                    mac_address: None,
                    primary: None,
                    enable_accelerated_networking: None,
                    enable_ip_forwarding: Some(false),
                    network_security_group: None,
                    resource_guid: None,
                },
            },
        )
        .await
        .unwrap();

    assert_eq!(nic.name, "my-nic");
    assert_eq!(nic.resource_type, "Microsoft.Network/networkInterfaces");
    assert!(
        nic.properties.mac_address.is_some(),
        "macAddress should be auto-generated"
    );
    assert!(
        nic.properties.resource_guid.is_some(),
        "resourceGuid should be auto-generated"
    );

    // Get it back
    let fetched = net
        .get_network_interface("test-rg", "my-nic")
        .await
        .unwrap();
    assert_eq!(fetched.name, "my-nic");
}

#[tokio::test]
async fn list_and_delete_nics() {
    let harness = setup().await;
    let net = harness.provider().networking();

    for name in &["nic-1", "nic-2"] {
        net.create_network_interface(
            "test-rg",
            name,
            CreateNetworkInterfaceParams {
                location: "eastus".to_string(),
                tags: HashMap::new(),
                properties: NetworkInterfaceProperties {
                    provisioning_state: None,
                    ip_configurations: None,
                    dns_settings: None,
                    mac_address: None,
                    primary: None,
                    enable_accelerated_networking: None,
                    enable_ip_forwarding: None,
                    network_security_group: None,
                    resource_guid: None,
                },
            },
        )
        .await
        .unwrap();
    }

    let page = net.list_network_interfaces("test-rg").await.unwrap();
    assert_eq!(page.value.len(), 2);

    net.delete_network_interface("test-rg", "nic-1")
        .await
        .unwrap();

    let page = net.list_network_interfaces("test-rg").await.unwrap();
    assert_eq!(page.value.len(), 1);
}

// ── Public IP Address tests ────────────────────────────────────────────

#[tokio::test]
async fn create_and_get_public_ip() {
    let harness = setup().await;
    let net = harness.provider().networking();

    let ip = net
        .create_public_ip_address(
            "test-rg",
            "my-ip",
            CreatePublicIPAddressParams {
                location: "eastus".to_string(),
                tags: HashMap::new(),
                sku: Some(PublicIPAddressSku {
                    name: Some("Standard".to_string()),
                    tier: Some("Regional".to_string()),
                }),
                zones: None,
                properties: PublicIPAddressProperties {
                    provisioning_state: None,
                    public_ip_allocation_method: Some("Static".to_string()),
                    public_ip_address_version: Some("IPv4".to_string()),
                    ip_address: None,
                    idle_timeout_in_minutes: Some(4),
                    dns_settings: Some(PublicIPAddressDnsSettings {
                        domain_name_label: Some("myapp".to_string()),
                        fqdn: None,
                        reverse_fqdn: None,
                        domain_name_label_scope: None,
                    }),
                    ip_configuration: None,
                    resource_guid: None,
                },
            },
        )
        .await
        .unwrap();

    assert_eq!(ip.name, "my-ip");
    assert_eq!(ip.resource_type, "Microsoft.Network/publicIPAddresses");
    assert!(
        ip.properties.ip_address.is_some(),
        "Static IP should be auto-assigned"
    );
    assert!(ip.properties.resource_guid.is_some());

    // DNS fqdn should be generated
    if let Some(ref dns) = ip.properties.dns_settings {
        assert!(
            dns.fqdn.is_some(),
            "fqdn should be auto-generated from domain label"
        );
    }

    // Get it back
    let fetched = net.get_public_ip_address("test-rg", "my-ip").await.unwrap();
    assert_eq!(fetched.name, "my-ip");
}

#[tokio::test]
async fn list_and_delete_public_ips() {
    let harness = setup().await;
    let net = harness.provider().networking();

    for name in &["ip-1", "ip-2"] {
        net.create_public_ip_address(
            "test-rg",
            name,
            CreatePublicIPAddressParams {
                location: "eastus".to_string(),
                tags: HashMap::new(),
                sku: None,
                zones: None,
                properties: PublicIPAddressProperties {
                    provisioning_state: None,
                    public_ip_allocation_method: Some("Dynamic".to_string()),
                    public_ip_address_version: None,
                    ip_address: None,
                    idle_timeout_in_minutes: None,
                    dns_settings: None,
                    ip_configuration: None,
                    resource_guid: None,
                },
            },
        )
        .await
        .unwrap();
    }

    let page = net.list_public_ip_addresses("test-rg").await.unwrap();
    assert_eq!(page.value.len(), 2);

    net.delete_public_ip_address("test-rg", "ip-1")
        .await
        .unwrap();

    let page = net.list_public_ip_addresses("test-rg").await.unwrap();
    assert_eq!(page.value.len(), 1);
}

// ── Security Rule tests (individual CRUD) ──────────────────────────────

#[tokio::test]
async fn security_rule_crud() {
    let harness = setup().await;
    let net = harness.provider().networking();

    // Create NSG first
    net.create_network_security_group(
        "test-rg",
        "my-nsg",
        CreateNsgParams {
            location: "eastus".to_string(),
            properties: NsgProperties {
                security_rules: vec![],
                provisioning_state: None,
            },
            tags: HashMap::new(),
        },
    )
    .await
    .unwrap();

    // Create individual rule
    let rule = net
        .create_or_update_security_rule(
            "test-rg",
            "my-nsg",
            "allow-http",
            CreateSecurityRuleParams {
                properties: SecurityRuleProperties {
                    protocol: "Tcp".to_string(),
                    source_address_prefix: "*".to_string(),
                    destination_address_prefix: "*".to_string(),
                    source_port_range: "*".to_string(),
                    destination_port_range: "80".to_string(),
                    access: "Allow".to_string(),
                    direction: "Inbound".to_string(),
                    priority: 100,
                    description: Some("Allow HTTP traffic".to_string()),
                    source_port_ranges: None,
                    destination_port_ranges: None,
                    source_address_prefixes: None,
                    destination_address_prefixes: None,
                    provisioning_state: None,
                },
            },
        )
        .await
        .unwrap();

    assert_eq!(rule.name, "allow-http");
    assert_eq!(rule.properties.access, "Allow");
    assert_eq!(rule.properties.priority, 100);

    // Get
    let fetched = net
        .get_security_rule("test-rg", "my-nsg", "allow-http")
        .await
        .unwrap();
    assert_eq!(fetched.properties.destination_port_range, "80");

    // List
    let page = net.list_security_rules("test-rg", "my-nsg").await.unwrap();
    assert_eq!(page.value.len(), 1);

    // Delete
    net.delete_security_rule("test-rg", "my-nsg", "allow-http")
        .await
        .unwrap();

    let page = net.list_security_rules("test-rg", "my-nsg").await.unwrap();
    assert!(page.value.is_empty());
}
