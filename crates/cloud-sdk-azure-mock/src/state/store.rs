use cloud_sdk_core::models::Page;
use cloud_sdk_core::models::resource::{
    CreateResourceGroupParams, ProvisioningState, ResourceGroup, ResourceGroupProperties,
    SpendingLimit, Subscription, SubscriptionPolicies, SubscriptionState,
};
use cloud_sdk_core::services::compute::{CreateVirtualMachineParams, PowerState, VirtualMachine};
use cloud_sdk_core::services::identity::{Principal, PrincipalType, RoleAssignment};
use cloud_sdk_core::services::networking::{
    CreateNsgParams, CreateSubnetParams, CreateVirtualNetworkParams, NetworkSecurityGroup, Subnet,
    SubnetProperties, VirtualNetwork,
};
use cloud_sdk_core::services::storage::{
    BlobContainer, BlobProperties, CreateStorageAccountParams, StorageAccount,
    StorageAccountProperties, StorageEndpoints, StorageSku,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Thread-safe in-memory state store mirroring Azure's resource hierarchy.
///
/// ```text
/// MockState
///   └─ subscriptions
///        ├─ metadata (id, displayName, state, tenantId)
///        └─ resource_groups
///             ├─ metadata (id, name, location, tags, provisioningState)
///             └─ storage_accounts
///                  ├─ metadata (StorageAccount)
///                  └─ containers
///                       ├─ metadata (BlobContainer)
///                       └─ blobs (name → data + properties)
/// ```
///
/// Wrapped in `Arc<RwLock<...>>` for concurrent read access and exclusive writes.
#[derive(Clone)]
pub struct MockState {
    inner: Arc<RwLock<StateInner>>,
}

struct StateInner {
    subscriptions: HashMap<String, SubscriptionState_>,
    current_principal: Principal,
    role_assignments: Vec<RoleAssignment>,
}

/// Internal subscription state (not to be confused with `SubscriptionState` enum).
struct SubscriptionState_ {
    metadata: Subscription,
    resource_groups: HashMap<String, ResourceGroupState>,
}

struct ResourceGroupState {
    metadata: ResourceGroup,
    storage_accounts: HashMap<String, StorageAccountState>,
    virtual_machines: HashMap<String, VmState>,
    virtual_networks: HashMap<String, VnetState>,
    network_security_groups: HashMap<String, NetworkSecurityGroup>,
}

struct VnetState {
    metadata: VirtualNetwork,
    subnets: HashMap<String, Subnet>,
}

struct VmState {
    metadata: VirtualMachine,
    power_state: PowerState,
}

struct StorageAccountState {
    metadata: StorageAccount,
    containers: HashMap<String, ContainerState>,
}

struct ContainerState {
    metadata: BlobContainer,
    blobs: HashMap<String, BlobState>,
}

struct BlobState {
    properties: BlobProperties,
    data: bytes::Bytes,
}

impl MockState {
    /// Create a `MockState` from an `AzureMockConfig`.
    /// Populates subscriptions and seed resource groups from the config.
    pub fn from_config(config: &crate::config::AzureMockConfig) -> Self {
        let mut subscriptions = HashMap::new();

        for sub_config in config.subscriptions.values() {
            let sub_id = sub_config.id.clone();
            let sub_state = match sub_config.state.as_str() {
                "Warned" => SubscriptionState::Warned,
                "PastDue" => SubscriptionState::PastDue,
                "Disabled" => SubscriptionState::Disabled,
                "Deleted" => SubscriptionState::Deleted,
                _ => SubscriptionState::Enabled,
            };

            let mut resource_groups = HashMap::new();
            for rg_seed in &sub_config.resource_groups {
                let rg = ResourceGroup {
                    id: format!("/subscriptions/{}/resourceGroups/{}", sub_id, rg_seed.name),
                    name: rg_seed.name.clone(),
                    resource_type: "Microsoft.Resources/resourceGroups".to_string(),
                    location: rg_seed.location.clone(),
                    managed_by: None,
                    tags: rg_seed.tags.clone(),
                    properties: ResourceGroupProperties {
                        provisioning_state: ProvisioningState::Succeeded,
                    },
                };
                resource_groups.insert(
                    rg_seed.name.clone(),
                    ResourceGroupState {
                        metadata: rg,
                        storage_accounts: HashMap::new(),
                        virtual_machines: HashMap::new(),
                        virtual_networks: HashMap::new(),
                        network_security_groups: HashMap::new(),
                    },
                );
            }

            let sub = SubscriptionState_ {
                metadata: Subscription {
                    id: format!("/subscriptions/{sub_id}"),
                    subscription_id: sub_id.clone(),
                    display_name: sub_config.display_name.clone(),
                    state: sub_state,
                    tenant_id: sub_config.tenant_id.clone(),
                    tags: HashMap::new(),
                    subscription_policies: Some(SubscriptionPolicies {
                        location_placement_id: "Internal_2014-09-01".to_string(),
                        quota_id: "Internal_2014-09-01".to_string(),
                        spending_limit: SpendingLimit::Off,
                    }),
                },
                resource_groups,
            };

            subscriptions.insert(sub_id, sub);
        }

        Self {
            inner: Arc::new(RwLock::new(StateInner {
                subscriptions,
                current_principal: Principal {
                    id: "00000000-0000-0000-0000-000000000002".to_string(),
                    display_name: "Mock Service Principal".to_string(),
                    principal_type: PrincipalType::ServicePrincipal,
                },
                role_assignments: vec![],
            })),
        }
    }

    /// Create a `MockState` with a default config for testing.
    pub fn with_defaults() -> Self {
        let config = crate::config::AzureMockConfig::from_toml(
            r#"
[server]

[subscriptions.default]
id = "00000000-0000-0000-0000-000000000000"
display_name = "Mock Subscription"
tenant_id = "00000000-0000-0000-0000-000000000001"
state = "Enabled"
"#,
        )
        .expect("default config should be valid");
        Self::from_config(&config)
    }

    // ── Subscriptions ──────────────────────────────────────────────────

    pub async fn list_subscriptions(&self) -> Page<Subscription> {
        let state = self.inner.read().await;
        let subs: Vec<Subscription> = state
            .subscriptions
            .values()
            .map(|s| s.metadata.clone())
            .collect();
        Page::new(subs)
    }

    pub async fn get_subscription(&self, subscription_id: &str) -> Option<Subscription> {
        let state = self.inner.read().await;
        state
            .subscriptions
            .get(subscription_id)
            .map(|s| s.metadata.clone())
    }

    // ── Resource Groups ────────────────────────────────────────────────

    pub async fn create_resource_group(
        &self,
        subscription_id: &str,
        name: &str,
        params: &CreateResourceGroupParams,
    ) -> Result<(ResourceGroup, bool), String> {
        let mut state = self.inner.write().await;
        let sub = state
            .subscriptions
            .get_mut(subscription_id)
            .ok_or_else(|| format!("Subscription '{subscription_id}' not found"))?;

        let rg = ResourceGroup {
            id: format!("/subscriptions/{subscription_id}/resourceGroups/{name}"),
            name: name.to_string(),
            resource_type: "Microsoft.Resources/resourceGroups".to_string(),
            location: params.location.clone(),
            managed_by: None,
            tags: params.tags.clone(),
            properties: ResourceGroupProperties {
                provisioning_state: ProvisioningState::Succeeded,
            },
        };

        let is_new = !sub.resource_groups.contains_key(name);
        sub.resource_groups.insert(
            name.to_string(),
            ResourceGroupState {
                metadata: rg.clone(),
                storage_accounts: HashMap::new(),
                virtual_machines: HashMap::new(),
                virtual_networks: HashMap::new(),
                network_security_groups: HashMap::new(),
            },
        );

        Ok((rg, is_new))
    }

    pub async fn get_resource_group(
        &self,
        subscription_id: &str,
        name: &str,
    ) -> Option<ResourceGroup> {
        let state = self.inner.read().await;
        state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(name)
            .map(|rg| rg.metadata.clone())
    }

    pub async fn list_resource_groups(&self, subscription_id: &str) -> Option<Page<ResourceGroup>> {
        let state = self.inner.read().await;
        let sub = state.subscriptions.get(subscription_id)?;
        let groups: Vec<ResourceGroup> = sub
            .resource_groups
            .values()
            .map(|rg| rg.metadata.clone())
            .collect();
        Some(Page::new(groups))
    }

    pub async fn delete_resource_group(
        &self,
        subscription_id: &str,
        name: &str,
    ) -> Result<bool, String> {
        let mut state = self.inner.write().await;
        let sub = state
            .subscriptions
            .get_mut(subscription_id)
            .ok_or_else(|| format!("Subscription '{subscription_id}' not found"))?;

        Ok(sub.resource_groups.remove(name).is_some())
    }

    pub async fn resource_group_exists(&self, subscription_id: &str, name: &str) -> bool {
        let state = self.inner.read().await;
        state
            .subscriptions
            .get(subscription_id)
            .is_some_and(|sub| sub.resource_groups.contains_key(name))
    }

    pub async fn update_resource_group(
        &self,
        subscription_id: &str,
        name: &str,
        tags: Option<HashMap<String, String>>,
    ) -> Option<ResourceGroup> {
        let mut state = self.inner.write().await;
        let rg = state
            .subscriptions
            .get_mut(subscription_id)?
            .resource_groups
            .get_mut(name)?;

        if let Some(tags) = tags {
            rg.metadata.tags = tags;
        }

        Some(rg.metadata.clone())
    }

    // ── Storage Accounts (ARM management plane) ────────────────────────

    pub async fn create_storage_account(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
        params: &CreateStorageAccountParams,
    ) -> Result<(StorageAccount, bool), String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;

        let account = StorageAccount {
            id: format!(
                "/subscriptions/{subscription_id}/resourceGroups/{resource_group}/providers/Microsoft.Storage/storageAccounts/{name}"
            ),
            name: name.to_string(),
            resource_type: "Microsoft.Storage/storageAccounts".to_string(),
            location: params.location.clone(),
            tags: params.tags.clone(),
            kind: params.kind.clone(),
            sku: StorageSku {
                name: params.sku.name.clone(),
                tier: params.sku.tier.clone(),
            },
            properties: StorageAccountProperties {
                provisioning_state: Some("Succeeded".to_string()),
                primary_endpoints: Some(StorageEndpoints {
                    blob: Some(format!("http://127.0.0.1/{name}")),
                    queue: None,
                    table: None,
                    file: None,
                }),
            },
        };

        let is_new = !rg.storage_accounts.contains_key(name);
        rg.storage_accounts.insert(
            name.to_string(),
            StorageAccountState {
                metadata: account.clone(),
                containers: HashMap::new(),
            },
        );

        Ok((account, is_new))
    }

    pub async fn get_storage_account(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
    ) -> Option<StorageAccount> {
        let state = self.inner.read().await;
        state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?
            .storage_accounts
            .get(name)
            .map(|sa| sa.metadata.clone())
    }

    pub async fn list_storage_accounts(
        &self,
        subscription_id: &str,
        resource_group: &str,
    ) -> Option<Page<StorageAccount>> {
        let state = self.inner.read().await;
        let rg = state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?;
        let accounts: Vec<StorageAccount> = rg
            .storage_accounts
            .values()
            .map(|sa| sa.metadata.clone())
            .collect();
        Some(Page::new(accounts))
    }

    pub async fn delete_storage_account(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
    ) -> Result<bool, String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        Ok(rg.storage_accounts.remove(name).is_some())
    }

    // ── Blob Containers (data plane) ───────────────────────────────────

    pub async fn create_container(&self, account: &str, container: &str) -> Result<(), String> {
        let mut state = self.inner.write().await;
        let sa = Self::find_storage_account_mut(&mut state, account)
            .ok_or_else(|| format!("Storage account '{account}' not found"))?;

        sa.containers.insert(
            container.to_string(),
            ContainerState {
                metadata: BlobContainer {
                    name: container.to_string(),
                    last_modified: Some(chrono::Utc::now().to_rfc2822()),
                },
                blobs: HashMap::new(),
            },
        );
        Ok(())
    }

    pub async fn delete_container(&self, account: &str, container: &str) -> Result<bool, String> {
        let mut state = self.inner.write().await;
        let sa = Self::find_storage_account_mut(&mut state, account)
            .ok_or_else(|| format!("Storage account '{account}' not found"))?;
        Ok(sa.containers.remove(container).is_some())
    }

    pub async fn list_containers(&self, account: &str) -> Option<Vec<BlobContainer>> {
        let state = self.inner.read().await;
        let sa = Self::find_storage_account(&state, account)?;
        Some(sa.containers.values().map(|c| c.metadata.clone()).collect())
    }

    // ── Blobs (data plane) ─────────────────────────────────────────────

    pub async fn put_blob(
        &self,
        account: &str,
        container: &str,
        blob_name: &str,
        data: bytes::Bytes,
        content_type: Option<&str>,
    ) -> Result<(), String> {
        let mut state = self.inner.write().await;
        let sa = Self::find_storage_account_mut(&mut state, account)
            .ok_or_else(|| format!("Storage account '{account}' not found"))?;
        let cont = sa
            .containers
            .get_mut(container)
            .ok_or_else(|| format!("Container '{container}' not found"))?;

        let properties = BlobProperties {
            name: blob_name.to_string(),
            content_length: data.len() as u64,
            content_type: Some(
                content_type
                    .unwrap_or("application/octet-stream")
                    .to_string(),
            ),
            last_modified: Some(chrono::Utc::now().to_rfc2822()),
        };

        cont.blobs
            .insert(blob_name.to_string(), BlobState { properties, data });
        Ok(())
    }

    pub async fn get_blob(
        &self,
        account: &str,
        container: &str,
        blob_name: &str,
    ) -> Option<bytes::Bytes> {
        let state = self.inner.read().await;
        let sa = Self::find_storage_account(&state, account)?;
        let cont = sa.containers.get(container)?;
        cont.blobs.get(blob_name).map(|b| b.data.clone())
    }

    pub async fn delete_blob(
        &self,
        account: &str,
        container: &str,
        blob_name: &str,
    ) -> Result<bool, String> {
        let mut state = self.inner.write().await;
        let sa = Self::find_storage_account_mut(&mut state, account)
            .ok_or_else(|| format!("Storage account '{account}' not found"))?;
        let cont = sa
            .containers
            .get_mut(container)
            .ok_or_else(|| format!("Container '{container}' not found"))?;
        Ok(cont.blobs.remove(blob_name).is_some())
    }

    pub async fn list_blobs(&self, account: &str, container: &str) -> Option<Vec<BlobProperties>> {
        let state = self.inner.read().await;
        let sa = Self::find_storage_account(&state, account)?;
        let cont = sa.containers.get(container)?;
        Some(cont.blobs.values().map(|b| b.properties.clone()).collect())
    }

    pub async fn get_blob_properties(
        &self,
        account: &str,
        container: &str,
        blob_name: &str,
    ) -> Option<BlobProperties> {
        let state = self.inner.read().await;
        let sa = Self::find_storage_account(&state, account)?;
        let cont = sa.containers.get(container)?;
        cont.blobs.get(blob_name).map(|b| b.properties.clone())
    }

    // ── Virtual Machines (ARM management plane) ──────────────────────

    pub async fn create_virtual_machine(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
        params: &CreateVirtualMachineParams,
    ) -> Result<(VirtualMachine, bool), String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;

        let vm_id = uuid::Uuid::new_v4().to_string();
        let mut vm = VirtualMachine {
            id: format!(
                "/subscriptions/{subscription_id}/resourceGroups/{resource_group}/providers/Microsoft.Compute/virtualMachines/{name}"
            ),
            name: name.to_string(),
            resource_type: "Microsoft.Compute/virtualMachines".to_string(),
            location: params.location.clone(),
            tags: params.tags.clone(),
            properties: params.properties.clone(),
        };
        // Set server-side fields
        vm.properties.vm_id = Some(vm_id);
        vm.properties.provisioning_state = Some("Succeeded".to_string());

        let is_new = !rg.virtual_machines.contains_key(name);
        rg.virtual_machines.insert(
            name.to_string(),
            VmState {
                metadata: vm.clone(),
                power_state: PowerState::Running,
            },
        );

        Ok((vm, is_new))
    }

    pub async fn get_virtual_machine(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
    ) -> Option<VirtualMachine> {
        let state = self.inner.read().await;
        state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?
            .virtual_machines
            .get(name)
            .map(|vm| vm.metadata.clone())
    }

    pub async fn list_virtual_machines(
        &self,
        subscription_id: &str,
        resource_group: &str,
    ) -> Option<Page<VirtualMachine>> {
        let state = self.inner.read().await;
        let rg = state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?;
        let vms: Vec<VirtualMachine> = rg
            .virtual_machines
            .values()
            .map(|vm| vm.metadata.clone())
            .collect();
        Some(Page::new(vms))
    }

    pub async fn delete_virtual_machine(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
    ) -> Result<bool, String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        Ok(rg.virtual_machines.remove(name).is_some())
    }

    pub async fn get_vm_power_state(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
    ) -> Option<PowerState> {
        let state = self.inner.read().await;
        state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?
            .virtual_machines
            .get(name)
            .map(|vm| vm.power_state.clone())
    }

    pub async fn set_vm_power_state(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
        power_state: PowerState,
    ) -> Result<(), String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        let vm = rg
            .virtual_machines
            .get_mut(name)
            .ok_or_else(|| format!("Virtual machine '{name}' not found"))?;
        vm.power_state = power_state;
        Ok(())
    }

    // ── Virtual Networks ───────────────────────────────────────────────

    pub async fn create_virtual_network(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
        params: &CreateVirtualNetworkParams,
    ) -> Result<(VirtualNetwork, bool), String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;

        let mut vnet = VirtualNetwork {
            id: format!(
                "/subscriptions/{subscription_id}/resourceGroups/{resource_group}/providers/Microsoft.Network/virtualNetworks/{name}"
            ),
            name: name.to_string(),
            resource_type: "Microsoft.Network/virtualNetworks".to_string(),
            location: params.location.clone(),
            tags: params.tags.clone(),
            properties: params.properties.clone(),
        };
        vnet.properties.provisioning_state = Some("Succeeded".to_string());

        let is_new = !rg.virtual_networks.contains_key(name);
        let subnets_from_props: HashMap<String, Subnet> = vnet
            .properties
            .subnets
            .iter()
            .map(|s| (s.name.clone(), s.clone()))
            .collect();

        rg.virtual_networks.insert(
            name.to_string(),
            VnetState {
                metadata: vnet.clone(),
                subnets: subnets_from_props,
            },
        );
        Ok((vnet, is_new))
    }

    pub async fn get_virtual_network(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
    ) -> Option<VirtualNetwork> {
        let state = self.inner.read().await;
        let vnet_state = state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?
            .virtual_networks
            .get(name)?;
        // Rebuild subnets from internal state into the metadata
        let mut vnet = vnet_state.metadata.clone();
        vnet.properties.subnets = vnet_state.subnets.values().cloned().collect();
        Some(vnet)
    }

    pub async fn list_virtual_networks(
        &self,
        subscription_id: &str,
        resource_group: &str,
    ) -> Option<Page<VirtualNetwork>> {
        let state = self.inner.read().await;
        let rg = state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?;
        let vnets: Vec<VirtualNetwork> = rg
            .virtual_networks
            .values()
            .map(|vs| {
                let mut v = vs.metadata.clone();
                v.properties.subnets = vs.subnets.values().cloned().collect();
                v
            })
            .collect();
        Some(Page::new(vnets))
    }

    pub async fn delete_virtual_network(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
    ) -> Result<bool, String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        Ok(rg.virtual_networks.remove(name).is_some())
    }

    // ── Subnets ────────────────────────────────────────────────────────

    pub async fn create_subnet(
        &self,
        subscription_id: &str,
        resource_group: &str,
        vnet_name: &str,
        subnet_name: &str,
        params: &CreateSubnetParams,
    ) -> Result<Subnet, String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        let vnet = rg
            .virtual_networks
            .get_mut(vnet_name)
            .ok_or_else(|| format!("Virtual network '{vnet_name}' not found"))?;

        let subnet = Subnet {
            id: format!("{}/subnets/{subnet_name}", vnet.metadata.id),
            name: subnet_name.to_string(),
            properties: SubnetProperties {
                provisioning_state: Some("Succeeded".to_string()),
                ..params.properties.clone()
            },
        };

        vnet.subnets.insert(subnet_name.to_string(), subnet.clone());
        Ok(subnet)
    }

    pub async fn get_subnet(
        &self,
        subscription_id: &str,
        resource_group: &str,
        vnet_name: &str,
        subnet_name: &str,
    ) -> Option<Subnet> {
        let state = self.inner.read().await;
        state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?
            .virtual_networks
            .get(vnet_name)?
            .subnets
            .get(subnet_name)
            .cloned()
    }

    pub async fn list_subnets(
        &self,
        subscription_id: &str,
        resource_group: &str,
        vnet_name: &str,
    ) -> Option<Page<Subnet>> {
        let state = self.inner.read().await;
        let vnet = state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?
            .virtual_networks
            .get(vnet_name)?;
        Some(Page::new(vnet.subnets.values().cloned().collect()))
    }

    pub async fn delete_subnet(
        &self,
        subscription_id: &str,
        resource_group: &str,
        vnet_name: &str,
        subnet_name: &str,
    ) -> Result<bool, String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        let vnet = rg
            .virtual_networks
            .get_mut(vnet_name)
            .ok_or_else(|| format!("Virtual network '{vnet_name}' not found"))?;
        Ok(vnet.subnets.remove(subnet_name).is_some())
    }

    // ── Network Security Groups ────────────────────────────────────────

    pub async fn create_nsg(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
        params: &CreateNsgParams,
    ) -> Result<(NetworkSecurityGroup, bool), String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;

        let mut nsg = NetworkSecurityGroup {
            id: format!(
                "/subscriptions/{subscription_id}/resourceGroups/{resource_group}/providers/Microsoft.Network/networkSecurityGroups/{name}"
            ),
            name: name.to_string(),
            resource_type: "Microsoft.Network/networkSecurityGroups".to_string(),
            location: params.location.clone(),
            tags: params.tags.clone(),
            properties: params.properties.clone(),
        };
        nsg.properties.provisioning_state = Some("Succeeded".to_string());

        let is_new = !rg.network_security_groups.contains_key(name);
        rg.network_security_groups
            .insert(name.to_string(), nsg.clone());
        Ok((nsg, is_new))
    }

    pub async fn get_nsg(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
    ) -> Option<NetworkSecurityGroup> {
        let state = self.inner.read().await;
        state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?
            .network_security_groups
            .get(name)
            .cloned()
    }

    pub async fn list_nsgs(
        &self,
        subscription_id: &str,
        resource_group: &str,
    ) -> Option<Page<NetworkSecurityGroup>> {
        let state = self.inner.read().await;
        let rg = state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?;
        let nsgs: Vec<NetworkSecurityGroup> =
            rg.network_security_groups.values().cloned().collect();
        Some(Page::new(nsgs))
    }

    pub async fn delete_nsg(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
    ) -> Result<bool, String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        Ok(rg.network_security_groups.remove(name).is_some())
    }

    // ── Identity ───────────────────────────────────────────────────────

    pub async fn get_current_principal(&self) -> Principal {
        let state = self.inner.read().await;
        state.current_principal.clone()
    }

    pub async fn list_role_assignments(&self, _scope: &str) -> Page<RoleAssignment> {
        let state = self.inner.read().await;
        Page::new(state.role_assignments.clone())
    }

    // ── Private helpers ────────────────────────────────────────────────

    fn get_rg_mut<'a>(
        state: &'a mut StateInner,
        subscription_id: &str,
        resource_group: &str,
    ) -> Result<&'a mut ResourceGroupState, String> {
        state
            .subscriptions
            .get_mut(subscription_id)
            .ok_or_else(|| format!("Subscription '{subscription_id}' not found"))?
            .resource_groups
            .get_mut(resource_group)
            .ok_or_else(|| format!("Resource group '{resource_group}' not found"))
    }

    /// Find a storage account by name across all subscriptions/resource groups.
    /// Used for data plane operations where the URL only has the account name.
    fn find_storage_account<'a>(
        state: &'a StateInner,
        account: &str,
    ) -> Option<&'a StorageAccountState> {
        for sub in state.subscriptions.values() {
            for rg in sub.resource_groups.values() {
                if let Some(sa) = rg.storage_accounts.get(account) {
                    return Some(sa);
                }
            }
        }
        None
    }

    fn find_storage_account_mut<'a>(
        state: &'a mut StateInner,
        account: &str,
    ) -> Option<&'a mut StorageAccountState> {
        for sub in state.subscriptions.values_mut() {
            for rg in sub.resource_groups.values_mut() {
                if let Some(sa) = rg.storage_accounts.get_mut(account) {
                    return Some(sa);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SUB_ID: &str = "00000000-0000-0000-0000-000000000000";

    #[tokio::test]
    async fn default_subscription_exists() {
        let state = MockState::with_defaults();
        let sub = state.get_subscription(SUB_ID).await;
        assert!(sub.is_some());
        let sub = sub.unwrap();
        assert_eq!(sub.subscription_id, SUB_ID);
        assert_eq!(sub.display_name, "Mock Subscription");
        assert_eq!(sub.state, SubscriptionState::Enabled);
    }

    #[tokio::test]
    async fn list_subscriptions() {
        let state = MockState::with_defaults();
        let page = state.list_subscriptions().await;
        assert_eq!(page.value.len(), 1);
        assert!(page.next_link.is_none());
    }

    #[tokio::test]
    async fn nonexistent_subscription_returns_none() {
        let state = MockState::with_defaults();
        assert!(state.get_subscription("nonexistent").await.is_none());
    }

    #[tokio::test]
    async fn create_and_get_resource_group() {
        let state = MockState::with_defaults();
        let params = CreateResourceGroupParams {
            location: "eastus".to_string(),
            tags: HashMap::new(),
        };

        let (rg, is_new) = state
            .create_resource_group(SUB_ID, "my-rg", &params)
            .await
            .unwrap();

        assert!(is_new);
        assert_eq!(rg.name, "my-rg");
        assert_eq!(rg.location, "eastus");
        assert_eq!(rg.resource_type, "Microsoft.Resources/resourceGroups");
        assert_eq!(
            rg.properties.provisioning_state,
            ProvisioningState::Succeeded
        );
        assert_eq!(
            rg.id,
            format!("/subscriptions/{SUB_ID}/resourceGroups/my-rg")
        );

        // Get it back
        let fetched = state.get_resource_group(SUB_ID, "my-rg").await.unwrap();
        assert_eq!(fetched.name, "my-rg");
    }

    #[tokio::test]
    async fn create_resource_group_is_idempotent() {
        let state = MockState::with_defaults();
        let params = CreateResourceGroupParams {
            location: "eastus".to_string(),
            tags: HashMap::new(),
        };

        let (_, is_new) = state
            .create_resource_group(SUB_ID, "my-rg", &params)
            .await
            .unwrap();
        assert!(is_new);

        // Second create updates (PUT semantics)
        let (_, is_new) = state
            .create_resource_group(SUB_ID, "my-rg", &params)
            .await
            .unwrap();
        assert!(!is_new);
    }

    #[tokio::test]
    async fn list_resource_groups() {
        let state = MockState::with_defaults();
        let params = CreateResourceGroupParams {
            location: "eastus".to_string(),
            tags: HashMap::new(),
        };

        state
            .create_resource_group(SUB_ID, "rg-1", &params)
            .await
            .unwrap();
        state
            .create_resource_group(SUB_ID, "rg-2", &params)
            .await
            .unwrap();

        let page = state.list_resource_groups(SUB_ID).await.unwrap();
        assert_eq!(page.value.len(), 2);
    }

    #[tokio::test]
    async fn delete_resource_group() {
        let state = MockState::with_defaults();
        let params = CreateResourceGroupParams {
            location: "eastus".to_string(),
            tags: HashMap::new(),
        };

        state
            .create_resource_group(SUB_ID, "my-rg", &params)
            .await
            .unwrap();

        let deleted = state.delete_resource_group(SUB_ID, "my-rg").await.unwrap();
        assert!(deleted);

        // Gone now
        assert!(state.get_resource_group(SUB_ID, "my-rg").await.is_none());

        // Delete again returns false
        let deleted = state.delete_resource_group(SUB_ID, "my-rg").await.unwrap();
        assert!(!deleted);
    }

    #[tokio::test]
    async fn resource_group_exists() {
        let state = MockState::with_defaults();
        assert!(!state.resource_group_exists(SUB_ID, "my-rg").await);

        let params = CreateResourceGroupParams {
            location: "westus".to_string(),
            tags: HashMap::new(),
        };
        state
            .create_resource_group(SUB_ID, "my-rg", &params)
            .await
            .unwrap();

        assert!(state.resource_group_exists(SUB_ID, "my-rg").await);
    }

    #[tokio::test]
    async fn update_resource_group_tags() {
        let state = MockState::with_defaults();
        let params = CreateResourceGroupParams {
            location: "eastus".to_string(),
            tags: HashMap::new(),
        };
        state
            .create_resource_group(SUB_ID, "my-rg", &params)
            .await
            .unwrap();

        let mut tags = HashMap::new();
        tags.insert("env".to_string(), "dev".to_string());

        let updated = state
            .update_resource_group(SUB_ID, "my-rg", Some(tags))
            .await
            .unwrap();

        assert_eq!(updated.tags.get("env").unwrap(), "dev");
        // Location unchanged
        assert_eq!(updated.location, "eastus");
    }

    #[tokio::test]
    async fn create_rg_in_nonexistent_subscription_fails() {
        let state = MockState::with_defaults();
        let params = CreateResourceGroupParams {
            location: "eastus".to_string(),
            tags: HashMap::new(),
        };

        let result = state
            .create_resource_group("nonexistent", "my-rg", &params)
            .await;
        assert!(result.is_err());
    }

    // ── Storage Account tests ──────────────────────────────────────────

    async fn state_with_rg() -> MockState {
        let state = MockState::with_defaults();
        state
            .create_resource_group(
                SUB_ID,
                "test-rg",
                &CreateResourceGroupParams {
                    location: "eastus".to_string(),
                    tags: HashMap::new(),
                },
            )
            .await
            .unwrap();
        state
    }

    fn storage_params() -> CreateStorageAccountParams {
        CreateStorageAccountParams {
            location: "eastus".to_string(),
            kind: "StorageV2".to_string(),
            sku: StorageSku {
                name: "Standard_LRS".to_string(),
                tier: Some("Standard".to_string()),
            },
            tags: HashMap::new(),
        }
    }

    #[tokio::test]
    async fn create_and_get_storage_account() {
        let state = state_with_rg().await;
        let (sa, is_new) = state
            .create_storage_account(SUB_ID, "test-rg", "mystorage", &storage_params())
            .await
            .unwrap();

        assert!(is_new);
        assert_eq!(sa.name, "mystorage");
        assert_eq!(sa.kind, "StorageV2");
        assert_eq!(sa.sku.name, "Standard_LRS");

        let fetched = state
            .get_storage_account(SUB_ID, "test-rg", "mystorage")
            .await
            .unwrap();
        assert_eq!(fetched.name, "mystorage");
    }

    #[tokio::test]
    async fn list_and_delete_storage_accounts() {
        let state = state_with_rg().await;
        state
            .create_storage_account(SUB_ID, "test-rg", "sa1", &storage_params())
            .await
            .unwrap();
        state
            .create_storage_account(SUB_ID, "test-rg", "sa2", &storage_params())
            .await
            .unwrap();

        let page = state
            .list_storage_accounts(SUB_ID, "test-rg")
            .await
            .unwrap();
        assert_eq!(page.value.len(), 2);

        state
            .delete_storage_account(SUB_ID, "test-rg", "sa1")
            .await
            .unwrap();

        let page = state
            .list_storage_accounts(SUB_ID, "test-rg")
            .await
            .unwrap();
        assert_eq!(page.value.len(), 1);
    }

    // ── Blob tests ─────────────────────────────────────────────────────

    #[tokio::test]
    async fn blob_lifecycle() {
        let state = state_with_rg().await;
        state
            .create_storage_account(SUB_ID, "test-rg", "blobacct", &storage_params())
            .await
            .unwrap();

        // Create container
        state
            .create_container("blobacct", "mycontainer")
            .await
            .unwrap();

        let containers = state.list_containers("blobacct").await.unwrap();
        assert_eq!(containers.len(), 1);
        assert_eq!(containers[0].name, "mycontainer");

        // Put blob
        let data = bytes::Bytes::from("hello world");
        state
            .put_blob(
                "blobacct",
                "mycontainer",
                "test.txt",
                data.clone(),
                Some("text/plain"),
            )
            .await
            .unwrap();

        // Get blob
        let fetched = state
            .get_blob("blobacct", "mycontainer", "test.txt")
            .await
            .unwrap();
        assert_eq!(fetched, data);

        // Get blob properties
        let props = state
            .get_blob_properties("blobacct", "mycontainer", "test.txt")
            .await
            .unwrap();
        assert_eq!(props.name, "test.txt");
        assert_eq!(props.content_length, 11);
        assert_eq!(props.content_type.as_deref(), Some("text/plain"));

        // List blobs
        let blobs = state.list_blobs("blobacct", "mycontainer").await.unwrap();
        assert_eq!(blobs.len(), 1);

        // Delete blob
        let deleted = state
            .delete_blob("blobacct", "mycontainer", "test.txt")
            .await
            .unwrap();
        assert!(deleted);

        let blobs = state.list_blobs("blobacct", "mycontainer").await.unwrap();
        assert!(blobs.is_empty());

        // Delete container
        let deleted = state
            .delete_container("blobacct", "mycontainer")
            .await
            .unwrap();
        assert!(deleted);

        let containers = state.list_containers("blobacct").await.unwrap();
        assert!(containers.is_empty());
    }
}
