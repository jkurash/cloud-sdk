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

// ── VNet extended operations ───────────────────────────────────────────

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

#[tokio::test]
async fn list_all_vnets() {
    let harness = setup().await;
    let net = harness.provider().networking();

    net.create_virtual_network("rg1", "vnet-east", vnet_params())
        .await
        .unwrap();
    net.create_virtual_network("rg2", "vnet-west", vnet_params())
        .await
        .unwrap();

    let page = net.list_all_virtual_networks().await.unwrap();
    assert_eq!(page.value.len(), 2);
}

#[tokio::test]
async fn update_vnet_tags() {
    let harness = setup().await;
    let net = harness.provider().networking();

    net.create_virtual_network("rg1", "tag-vnet", vnet_params())
        .await
        .unwrap();

    let mut tags = HashMap::new();
    tags.insert("env".to_string(), "prod".to_string());
    let updated = net
        .update_virtual_network_tags("rg1", "tag-vnet", tags)
        .await
        .unwrap();

    assert_eq!(updated.tags.get("env").unwrap(), "prod");
}

// ── NSG extended operations ────────────────────────────────────────────

#[tokio::test]
async fn list_all_nsgs() {
    let harness = setup().await;
    let net = harness.provider().networking();

    net.create_network_security_group(
        "rg1",
        "nsg-1",
        CreateNsgParams {
            location: "eastus".to_string(),
            properties: NsgProperties {
                security_rules: vec![],
                default_security_rules: None,
                resource_guid: None,
                provisioning_state: None,
            },
            tags: HashMap::new(),
        },
    )
    .await
    .unwrap();
    net.create_network_security_group(
        "rg2",
        "nsg-2",
        CreateNsgParams {
            location: "westus".to_string(),
            properties: NsgProperties {
                security_rules: vec![],
                default_security_rules: None,
                resource_guid: None,
                provisioning_state: None,
            },
            tags: HashMap::new(),
        },
    )
    .await
    .unwrap();

    let page = net.list_all_network_security_groups().await.unwrap();
    assert_eq!(page.value.len(), 2);
}

#[tokio::test]
async fn update_nsg_tags() {
    let harness = setup().await;
    let net = harness.provider().networking();

    net.create_network_security_group(
        "rg1",
        "tag-nsg",
        CreateNsgParams {
            location: "eastus".to_string(),
            properties: NsgProperties {
                security_rules: vec![],
                default_security_rules: None,
                resource_guid: None,
                provisioning_state: None,
            },
            tags: HashMap::new(),
        },
    )
    .await
    .unwrap();

    let mut tags = HashMap::new();
    tags.insert("team".to_string(), "platform".to_string());
    let updated = net.update_nsg_tags("rg1", "tag-nsg", tags).await.unwrap();

    assert_eq!(updated.tags.get("team").unwrap(), "platform");
}

// ── Route Table tests ──────────────────────────────────────────────────

#[tokio::test]
async fn route_table_lifecycle() {
    let harness = setup().await;
    let net = harness.provider().networking();

    let rt = net
        .create_route_table(
            "rg1",
            "my-rt",
            CreateRouteTableParams {
                location: "eastus".to_string(),
                tags: HashMap::new(),
                properties: RouteTableProperties {
                    routes: None,
                    subnets: None,
                    disable_bgp_route_propagation: Some(false),
                    provisioning_state: None,
                    resource_guid: None,
                },
            },
        )
        .await
        .unwrap();

    assert_eq!(rt.name, "my-rt");
    assert_eq!(rt.resource_type, "Microsoft.Network/routeTables");
    assert!(rt.properties.resource_guid.is_some());

    // Get
    let fetched = net.get_route_table("rg1", "my-rt").await.unwrap();
    assert_eq!(fetched.name, "my-rt");

    // List
    let page = net.list_route_tables("rg1").await.unwrap();
    assert_eq!(page.value.len(), 1);

    // Delete
    net.delete_route_table("rg1", "my-rt").await.unwrap();
    let page = net.list_route_tables("rg1").await.unwrap();
    assert!(page.value.is_empty());
}

#[tokio::test]
async fn route_crud_within_table() {
    let harness = setup().await;
    let net = harness.provider().networking();

    net.create_route_table(
        "rg1",
        "rt-1",
        CreateRouteTableParams {
            location: "eastus".to_string(),
            tags: HashMap::new(),
            properties: RouteTableProperties {
                routes: None,
                subnets: None,
                disable_bgp_route_propagation: None,
                provisioning_state: None,
                resource_guid: None,
            },
        },
    )
    .await
    .unwrap();

    // Create route
    let route = net
        .create_route(
            "rg1",
            "rt-1",
            "to-internet",
            CreateRouteParams {
                properties: RouteProperties {
                    address_prefix: Some("0.0.0.0/0".to_string()),
                    next_hop_type: "Internet".to_string(),
                    next_hop_ip_address: None,
                    provisioning_state: None,
                    has_bgp_override: None,
                },
            },
        )
        .await
        .unwrap();

    assert_eq!(route.name.as_deref(), Some("to-internet"));

    // Get
    let fetched = net.get_route("rg1", "rt-1", "to-internet").await.unwrap();
    assert_eq!(fetched.properties.next_hop_type, "Internet");

    // List
    let page = net.list_routes("rg1", "rt-1").await.unwrap();
    assert_eq!(page.value.len(), 1);

    // Delete
    net.delete_route("rg1", "rt-1", "to-internet")
        .await
        .unwrap();
    let page = net.list_routes("rg1", "rt-1").await.unwrap();
    assert!(page.value.is_empty());
}

// ── VNet Peering tests ─────────────────────────────────────────────────

#[tokio::test]
async fn vnet_peering_lifecycle() {
    let harness = setup().await;
    let net = harness.provider().networking();

    // Create two VNets to peer
    net.create_virtual_network("rg1", "vnet-a", vnet_params())
        .await
        .unwrap();

    let mut vnet_b_params = vnet_params();
    vnet_b_params.properties.address_space.address_prefixes = vec!["10.1.0.0/16".to_string()];
    net.create_virtual_network("rg1", "vnet-b", vnet_b_params)
        .await
        .unwrap();

    // Create peering
    let peering = net
        .create_virtual_network_peering(
            "rg1",
            "vnet-a",
            "a-to-b",
            CreateVirtualNetworkPeeringParams {
                properties: VirtualNetworkPeeringProperties {
                    allow_virtual_network_access: Some(true),
                    allow_forwarded_traffic: Some(false),
                    allow_gateway_transit: Some(false),
                    use_remote_gateways: Some(false),
                    remote_virtual_network: Some(SubResourceRef {
                        id: "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.Network/virtualNetworks/vnet-b".to_string(),
                    }),
                    peering_state: None,
                    peering_sync_level: None,
                    provisioning_state: None,
                    remote_address_space: None,
                    remote_bgp_communities: None,
                },
            },
        )
        .await
        .unwrap();

    assert_eq!(peering.name.as_deref(), Some("a-to-b"));

    // Get
    let fetched = net
        .get_virtual_network_peering("rg1", "vnet-a", "a-to-b")
        .await
        .unwrap();
    assert!(fetched.properties.is_some());

    // List
    let page = net
        .list_virtual_network_peerings("rg1", "vnet-a")
        .await
        .unwrap();
    assert_eq!(page.value.len(), 1);

    // Delete
    net.delete_virtual_network_peering("rg1", "vnet-a", "a-to-b")
        .await
        .unwrap();
    let page = net
        .list_virtual_network_peerings("rg1", "vnet-a")
        .await
        .unwrap();
    assert!(page.value.is_empty());
}
