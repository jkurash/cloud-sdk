use cloud_sdk_core::models::Page;
use cloud_sdk_core::models::resource::{
    CreateResourceGroupParams, ProvisioningState, ResourceGroup, ResourceGroupProperties,
    SpendingLimit, Subscription, SubscriptionPolicies, SubscriptionState,
};
use cloud_sdk_core::services::compute::{CreateVirtualMachineParams, PowerState, VirtualMachine};
use cloud_sdk_core::services::identity::{Principal, PrincipalType, RoleAssignment};
use cloud_sdk_core::services::networking::{
    ApplicationSecurityGroup, ApplicationSecurityGroupProperties,
    CreateApplicationSecurityGroupParams, CreateNetworkInterfaceParams, CreateNsgParams,
    CreatePublicIPAddressParams, CreateRouteParams, CreateRouteTableParams,
    CreateSecurityRuleParams, CreateSubnetParams, CreateVirtualNetworkParams,
    CreateVirtualNetworkPeeringParams, NetworkInterface, NetworkSecurityGroup, PublicIPAddress,
    Route, RouteTable, SecurityRule, ServiceTagInformation, ServiceTagInformationProperties,
    ServiceTagsListResult, Subnet, SubnetProperties, VirtualNetwork, VirtualNetworkPeering,
};
use cloud_sdk_core::services::storage::{
    BlobContainer, BlobProperties, CreateStorageAccountParams, StorageAccount,
    StorageAccountProperties, StorageEndpoints, StorageServiceProperties, StorageSku,
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
    network_interfaces: HashMap<String, NetworkInterface>,
    public_ip_addresses: HashMap<String, PublicIPAddress>,
    route_tables: HashMap<String, RouteTableState>,
    application_security_groups: HashMap<String, ApplicationSecurityGroup>,
}

struct VnetState {
    metadata: VirtualNetwork,
    subnets: HashMap<String, Subnet>,
    peerings: HashMap<String, VirtualNetworkPeering>,
}

struct RouteTableState {
    metadata: RouteTable,
    routes: HashMap<String, Route>,
}

struct VmState {
    metadata: VirtualMachine,
    power_state: PowerState,
    etag_version: u64,
}

struct StorageAccountState {
    metadata: StorageAccount,
    containers: HashMap<String, ContainerState>,
    service_properties: StorageServiceProperties,
}

struct ContainerState {
    metadata: BlobContainer,
    container_metadata: HashMap<String, String>,
    blobs: HashMap<String, BlobState>,
}

struct BlobState {
    properties: BlobProperties,
    data: bytes::Bytes,
    uncommitted_blocks: HashMap<String, bytes::Bytes>,
    committed_blocks: Vec<(String, bytes::Bytes)>,
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
                        network_interfaces: HashMap::new(),
                        public_ip_addresses: HashMap::new(),
                        route_tables: HashMap::new(),
                        application_security_groups: HashMap::new(),
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
                network_interfaces: HashMap::new(),
                public_ip_addresses: HashMap::new(),
                route_tables: HashMap::new(),
                application_security_groups: HashMap::new(),
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
            identity: None,
            extended_location: None,
            properties: StorageAccountProperties {
                provisioning_state: Some("Succeeded".to_string()),
                creation_time: Some(chrono::Utc::now().to_rfc3339()),
                primary_location: Some(params.location.clone()),
                secondary_location: None,
                status_of_primary: Some("available".to_string()),
                status_of_secondary: None,
                primary_endpoints: Some(StorageEndpoints {
                    blob: Some(format!("http://127.0.0.1/{name}")),
                    queue: Some(format!("http://127.0.0.1/{name}-queue")),
                    table: Some(format!("http://127.0.0.1/{name}-table")),
                    file: Some(format!("http://127.0.0.1/{name}-file")),
                    dfs: None,
                    web: None,
                    microsoft_endpoints: None,
                    internet_endpoints: None,
                }),
                secondary_endpoints: None,
                access_tier: None,
                allow_blob_public_access: Some(false),
                allow_shared_key_access: Some(true),
                allow_cross_tenant_replication: Some(false),
                default_to_oauth_authentication: None,
                allowed_copy_scope: None,
                minimum_tls_version: Some("TLS1_2".to_string()),
                supports_https_traffic_only: Some(true),
                network_acls: None,
                public_network_access: Some("Enabled".to_string()),
                dns_endpoint_type: None,
                routing_preference: None,
                encryption: None,
                custom_domain: None,
                azure_files_identity_based_authentication: None,
                is_hns_enabled: None,
                is_sftp_enabled: None,
                is_nfs_v3_enabled: None,
                is_local_user_enabled: None,
                enable_extended_groups: None,
                large_file_shares_state: None,
                immutable_storage_with_versioning: None,
                key_policy: None,
                sas_policy: None,
                key_creation_time: None,
                failover_in_progress: None,
            },
        };

        let is_new = !rg.storage_accounts.contains_key(name);
        rg.storage_accounts.insert(
            name.to_string(),
            StorageAccountState {
                metadata: account.clone(),
                containers: HashMap::new(),
                service_properties: StorageServiceProperties {
                    logging: None,
                    hour_metrics: None,
                    minute_metrics: None,
                    cors: None,
                    default_service_version: Some("2023-11-03".to_string()),
                    delete_retention_policy: None,
                    static_website: None,
                },
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

    /// PATCH update — merges JSON into the stored storage account.
    pub async fn update_storage_account(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
        patch: serde_json::Value,
    ) -> Result<StorageAccount, String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        let sa_state = rg
            .storage_accounts
            .get_mut(name)
            .ok_or_else(|| format!("Storage account '{name}' not found"))?;

        let mut current = serde_json::to_value(&sa_state.metadata).unwrap();
        json_merge(&mut current, &patch);
        let updated: StorageAccount =
            serde_json::from_value(current).map_err(|e| format!("failed to apply patch: {e}"))?;

        sa_state.metadata = updated.clone();
        Ok(updated)
    }

    /// List all storage accounts across all resource groups in a subscription.
    pub async fn list_all_storage_accounts(
        &self,
        subscription_id: &str,
    ) -> Option<Page<StorageAccount>> {
        let state = self.inner.read().await;
        let sub = state.subscriptions.get(subscription_id)?;
        let accounts: Vec<StorageAccount> = sub
            .resource_groups
            .values()
            .flat_map(|rg| rg.storage_accounts.values().map(|sa| sa.metadata.clone()))
            .collect();
        Some(Page::new(accounts))
    }

    /// Check if a storage account name is available.
    pub async fn check_storage_name_availability(
        &self,
        name: &str,
    ) -> (bool, Option<String>, Option<String>) {
        let state = self.inner.read().await;
        for sub in state.subscriptions.values() {
            for rg in sub.resource_groups.values() {
                if rg.storage_accounts.contains_key(name) {
                    return (
                        false,
                        Some("AlreadyExists".to_string()),
                        Some(format!(
                            "The storage account named {name} is already taken."
                        )),
                    );
                }
            }
        }
        (true, None, None)
    }

    /// List storage account keys (mock generates deterministic keys).
    pub async fn list_storage_keys(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
    ) -> Result<Vec<(String, String, String)>, String> {
        let state = self.inner.read().await;
        let _sa = state
            .subscriptions
            .get(subscription_id)
            .and_then(|sub| sub.resource_groups.get(resource_group))
            .and_then(|rg| rg.storage_accounts.get(name))
            .ok_or_else(|| format!("Storage account '{name}' not found"))?;

        Ok(vec![
            (
                "key1".to_string(),
                format!("mockkey1{name}000000000000000000000000000000000000=="),
                "FULL".to_string(),
            ),
            (
                "key2".to_string(),
                format!("mockkey2{name}000000000000000000000000000000000000=="),
                "FULL".to_string(),
            ),
        ])
    }

    // ── Blob Containers (data plane) ───────────────────────────────────

    pub async fn create_container(&self, account: &str, container: &str) -> Result<(), String> {
        let mut state = self.inner.write().await;
        let sa = Self::find_storage_account_mut(&mut state, account)
            .ok_or_else(|| format!("Storage account '{account}' not found"))?;

        let now = chrono::Utc::now().to_rfc2822();
        sa.containers.insert(
            container.to_string(),
            ContainerState {
                metadata: BlobContainer {
                    name: container.to_string(),
                    last_modified: Some(now),
                    etag: Some(format!("\"0x{}\"", uuid::Uuid::new_v4().simple())),
                    lease_status: Some("unlocked".to_string()),
                    lease_state: Some("available".to_string()),
                    public_access: None,
                    has_immutability_policy: Some(false),
                    has_legal_hold: Some(false),
                    metadata: HashMap::new(),
                },
                container_metadata: HashMap::new(),
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

        let now = chrono::Utc::now();
        let properties = BlobProperties {
            name: blob_name.to_string(),
            content_length: data.len() as u64,
            content_type: Some(
                content_type
                    .unwrap_or("application/octet-stream")
                    .to_string(),
            ),
            content_encoding: None,
            content_language: None,
            content_disposition: None,
            content_md5: None,
            cache_control: None,
            last_modified: Some(now.to_rfc2822()),
            etag: Some(format!("\"0x{}\"", uuid::Uuid::new_v4().simple())),
            blob_type: Some("BlockBlob".to_string()),
            access_tier: Some("Hot".to_string()),
            lease_status: Some("unlocked".to_string()),
            lease_state: Some("available".to_string()),
            server_encrypted: Some(true),
            creation_time: Some(now.to_rfc3339()),
            copy_id: None,
            copy_status: None,
            copy_source: None,
            copy_progress: None,
            metadata: HashMap::new(),
            tags: HashMap::new(),
        };

        cont.blobs.insert(
            blob_name.to_string(),
            BlobState {
                properties,
                data,
                uncommitted_blocks: HashMap::new(),
                committed_blocks: Vec::new(),
            },
        );
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

    // ── Container extended operations (data plane) ────────────────────

    /// Get container properties (returns the full BlobContainer metadata).
    pub async fn get_container_properties(
        &self,
        account: &str,
        container: &str,
    ) -> Option<BlobContainer> {
        let state = self.inner.read().await;
        let sa = Self::find_storage_account(&state, account)?;
        let cont = sa.containers.get(container)?;
        let mut meta = cont.metadata.clone();
        meta.metadata.clone_from(&cont.container_metadata);
        Some(meta)
    }

    /// Get container metadata only.
    pub async fn get_container_metadata(
        &self,
        account: &str,
        container: &str,
    ) -> Option<HashMap<String, String>> {
        let state = self.inner.read().await;
        let sa = Self::find_storage_account(&state, account)?;
        let cont = sa.containers.get(container)?;
        Some(cont.container_metadata.clone())
    }

    /// Set container metadata.
    pub async fn set_container_metadata(
        &self,
        account: &str,
        container: &str,
        metadata: HashMap<String, String>,
    ) -> Result<(), String> {
        let mut state = self.inner.write().await;
        let sa = Self::find_storage_account_mut(&mut state, account)
            .ok_or_else(|| format!("Storage account '{account}' not found"))?;
        let cont = sa
            .containers
            .get_mut(container)
            .ok_or_else(|| format!("Container '{container}' not found"))?;
        cont.container_metadata = metadata.clone();
        cont.metadata.metadata = metadata;
        cont.metadata.last_modified = Some(chrono::Utc::now().to_rfc2822());
        cont.metadata.etag = Some(format!("\"0x{}\"", uuid::Uuid::new_v4().simple()));
        Ok(())
    }

    // ── Blob metadata/properties/tags (data plane) ────────────────────

    /// Get blob metadata only.
    pub async fn get_blob_metadata(
        &self,
        account: &str,
        container: &str,
        blob_name: &str,
    ) -> Option<HashMap<String, String>> {
        let state = self.inner.read().await;
        let sa = Self::find_storage_account(&state, account)?;
        let cont = sa.containers.get(container)?;
        cont.blobs
            .get(blob_name)
            .map(|b| b.properties.metadata.clone())
    }

    /// Set blob metadata.
    pub async fn set_blob_metadata(
        &self,
        account: &str,
        container: &str,
        blob_name: &str,
        metadata: HashMap<String, String>,
    ) -> Result<(), String> {
        let mut state = self.inner.write().await;
        let sa = Self::find_storage_account_mut(&mut state, account)
            .ok_or_else(|| format!("Storage account '{account}' not found"))?;
        let cont = sa
            .containers
            .get_mut(container)
            .ok_or_else(|| format!("Container '{container}' not found"))?;
        let blob = cont
            .blobs
            .get_mut(blob_name)
            .ok_or_else(|| format!("Blob '{blob_name}' not found"))?;
        blob.properties.metadata = metadata;
        blob.properties.last_modified = Some(chrono::Utc::now().to_rfc2822());
        blob.properties.etag = Some(format!("\"0x{}\"", uuid::Uuid::new_v4().simple()));
        Ok(())
    }

    /// Get blob tags.
    pub async fn get_blob_tags(
        &self,
        account: &str,
        container: &str,
        blob_name: &str,
    ) -> Option<HashMap<String, String>> {
        let state = self.inner.read().await;
        let sa = Self::find_storage_account(&state, account)?;
        let cont = sa.containers.get(container)?;
        cont.blobs.get(blob_name).map(|b| b.properties.tags.clone())
    }

    /// Set blob tags.
    pub async fn set_blob_tags(
        &self,
        account: &str,
        container: &str,
        blob_name: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), String> {
        let mut state = self.inner.write().await;
        let sa = Self::find_storage_account_mut(&mut state, account)
            .ok_or_else(|| format!("Storage account '{account}' not found"))?;
        let cont = sa
            .containers
            .get_mut(container)
            .ok_or_else(|| format!("Container '{container}' not found"))?;
        let blob = cont
            .blobs
            .get_mut(blob_name)
            .ok_or_else(|| format!("Blob '{blob_name}' not found"))?;
        blob.properties.tags = tags;
        Ok(())
    }

    /// Set blob properties (content-type, cache-control, etc.).
    #[allow(clippy::too_many_arguments)]
    pub async fn set_blob_properties(
        &self,
        account: &str,
        container: &str,
        blob_name: &str,
        content_type: Option<String>,
        content_encoding: Option<String>,
        content_language: Option<String>,
        content_disposition: Option<String>,
        cache_control: Option<String>,
    ) -> Result<(), String> {
        let mut state = self.inner.write().await;
        let sa = Self::find_storage_account_mut(&mut state, account)
            .ok_or_else(|| format!("Storage account '{account}' not found"))?;
        let cont = sa
            .containers
            .get_mut(container)
            .ok_or_else(|| format!("Container '{container}' not found"))?;
        let blob = cont
            .blobs
            .get_mut(blob_name)
            .ok_or_else(|| format!("Blob '{blob_name}' not found"))?;

        if let Some(ct) = content_type {
            blob.properties.content_type = Some(ct);
        }
        if let Some(ce) = content_encoding {
            blob.properties.content_encoding = Some(ce);
        }
        if let Some(cl) = content_language {
            blob.properties.content_language = Some(cl);
        }
        if let Some(cd) = content_disposition {
            blob.properties.content_disposition = Some(cd);
        }
        if let Some(cc) = cache_control {
            blob.properties.cache_control = Some(cc);
        }
        blob.properties.last_modified = Some(chrono::Utc::now().to_rfc2822());
        blob.properties.etag = Some(format!("\"0x{}\"", uuid::Uuid::new_v4().simple()));
        Ok(())
    }

    // ── Copy Blob / Snapshot / Service Properties (data plane) ─────

    /// Copy a blob from source to destination. Returns the copy ID.
    pub async fn copy_blob(
        &self,
        account: &str,
        dest_container: &str,
        dest_blob: &str,
        source_account: &str,
        source_container: &str,
        source_blob: &str,
    ) -> Result<String, String> {
        let mut state = self.inner.write().await;

        // Read source blob data + properties
        let src_sa = Self::find_storage_account(&state, source_account)
            .ok_or_else(|| format!("Source storage account '{source_account}' not found"))?;
        let src_cont = src_sa
            .containers
            .get(source_container)
            .ok_or_else(|| format!("Source container '{source_container}' not found"))?;
        let src_blob_state = src_cont
            .blobs
            .get(source_blob)
            .ok_or_else(|| format!("Source blob '{source_blob}' not found"))?;
        let src_data = src_blob_state.data.clone();
        let src_props = src_blob_state.properties.clone();

        // Build destination blob
        let copy_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        let dest_properties = BlobProperties {
            name: dest_blob.to_string(),
            content_length: src_data.len() as u64,
            content_type: src_props.content_type,
            content_encoding: src_props.content_encoding,
            content_language: src_props.content_language,
            content_disposition: src_props.content_disposition,
            content_md5: src_props.content_md5,
            cache_control: src_props.cache_control,
            last_modified: Some(now.to_rfc2822()),
            etag: Some(format!("\"0x{}\"", uuid::Uuid::new_v4().simple())),
            blob_type: src_props.blob_type,
            access_tier: src_props.access_tier,
            lease_status: Some("unlocked".to_string()),
            lease_state: Some("available".to_string()),
            server_encrypted: Some(true),
            creation_time: Some(now.to_rfc3339()),
            copy_id: Some(copy_id.clone()),
            copy_status: Some("success".to_string()),
            copy_source: Some(format!(
                "http://127.0.0.1/{source_account}/{source_container}/{source_blob}"
            )),
            copy_progress: None,
            metadata: src_props.metadata,
            tags: src_props.tags,
        };

        // Write to destination
        let dest_sa = Self::find_storage_account_mut(&mut state, account)
            .ok_or_else(|| format!("Storage account '{account}' not found"))?;
        let dest_cont = dest_sa
            .containers
            .get_mut(dest_container)
            .ok_or_else(|| format!("Container '{dest_container}' not found"))?;
        dest_cont.blobs.insert(
            dest_blob.to_string(),
            BlobState {
                properties: dest_properties,
                data: src_data,
                uncommitted_blocks: HashMap::new(),
                committed_blocks: Vec::new(),
            },
        );

        Ok(copy_id)
    }

    /// Check if a blob exists (used by snapshot).
    pub async fn blob_exists(&self, account: &str, container: &str, blob_name: &str) -> bool {
        let state = self.inner.read().await;
        Self::find_storage_account(&state, account)
            .and_then(|sa| sa.containers.get(container))
            .is_some_and(|cont| cont.blobs.contains_key(blob_name))
    }

    // ── Block blob operations (data plane) ────────────────────────────

    /// Put a single block (uncommitted).
    pub async fn put_block(
        &self,
        account: &str,
        container: &str,
        blob_name: &str,
        block_id: &str,
        data: bytes::Bytes,
    ) -> Result<(), String> {
        let mut state = self.inner.write().await;
        let sa = Self::find_storage_account_mut(&mut state, account)
            .ok_or_else(|| format!("Storage account '{account}' not found"))?;
        let cont = sa
            .containers
            .get_mut(container)
            .ok_or_else(|| format!("Container '{container}' not found"))?;

        let blob = cont.blobs.entry(blob_name.to_string()).or_insert_with(|| {
            let now = chrono::Utc::now();
            BlobState {
                properties: BlobProperties {
                    name: blob_name.to_string(),
                    content_length: 0,
                    content_type: Some("application/octet-stream".to_string()),
                    content_encoding: None,
                    content_language: None,
                    content_disposition: None,
                    content_md5: None,
                    cache_control: None,
                    last_modified: Some(now.to_rfc2822()),
                    etag: Some(format!("\"0x{}\"", uuid::Uuid::new_v4().simple())),
                    blob_type: Some("BlockBlob".to_string()),
                    access_tier: Some("Hot".to_string()),
                    lease_status: Some("unlocked".to_string()),
                    lease_state: Some("available".to_string()),
                    server_encrypted: Some(true),
                    creation_time: Some(now.to_rfc3339()),
                    copy_id: None,
                    copy_status: None,
                    copy_source: None,
                    copy_progress: None,
                    metadata: HashMap::new(),
                    tags: HashMap::new(),
                },
                data: bytes::Bytes::new(),
                uncommitted_blocks: HashMap::new(),
                committed_blocks: Vec::new(),
            }
        });

        blob.uncommitted_blocks.insert(block_id.to_string(), data);
        Ok(())
    }

    /// Commit blocks into the blob.
    pub async fn put_block_list(
        &self,
        account: &str,
        container: &str,
        blob_name: &str,
        block_ids: Vec<String>,
        content_type: Option<&str>,
    ) -> Result<(), String> {
        let mut state = self.inner.write().await;
        let sa = Self::find_storage_account_mut(&mut state, account)
            .ok_or_else(|| format!("Storage account '{account}' not found"))?;
        let cont = sa
            .containers
            .get_mut(container)
            .ok_or_else(|| format!("Container '{container}' not found"))?;
        let blob = cont
            .blobs
            .get_mut(blob_name)
            .ok_or_else(|| format!("Blob '{blob_name}' not found"))?;

        // Resolve each block ID: look in uncommitted first, then committed
        let mut new_committed: Vec<(String, bytes::Bytes)> = Vec::new();
        let mut concatenated = Vec::new();

        for id in &block_ids {
            if let Some(data) = blob.uncommitted_blocks.remove(id) {
                concatenated.extend_from_slice(&data);
                new_committed.push((id.clone(), data));
            } else if let Some((_existing_id, data)) =
                blob.committed_blocks.iter().find(|(bid, _)| bid == id)
            {
                let data = data.clone();
                concatenated.extend_from_slice(&data);
                new_committed.push((id.clone(), data));
            } else {
                return Err(format!("Block ID '{id}' not found"));
            }
        }

        // Clear remaining uncommitted blocks and replace committed list
        blob.uncommitted_blocks.clear();
        blob.committed_blocks = new_committed;
        blob.data = bytes::Bytes::from(concatenated);

        // Update properties
        blob.properties.content_length = blob.data.len() as u64;
        if let Some(ct) = content_type {
            blob.properties.content_type = Some(ct.to_string());
        }
        let now = chrono::Utc::now();
        blob.properties.last_modified = Some(now.to_rfc2822());
        blob.properties.etag = Some(format!("\"0x{}\"", uuid::Uuid::new_v4().simple()));

        Ok(())
    }

    /// Get the block list.
    /// Returns (committed_blocks: [(id, size)], uncommitted_blocks: [(id, size)]).
    pub async fn get_block_list(
        &self,
        account: &str,
        container: &str,
        blob_name: &str,
    ) -> Option<(Vec<(String, usize)>, Vec<(String, usize)>)> {
        let state = self.inner.read().await;
        let sa = Self::find_storage_account(&state, account)?;
        let cont = sa.containers.get(container)?;
        let blob = cont.blobs.get(blob_name)?;

        let committed: Vec<(String, usize)> = blob
            .committed_blocks
            .iter()
            .map(|(id, data)| (id.clone(), data.len()))
            .collect();
        let uncommitted: Vec<(String, usize)> = blob
            .uncommitted_blocks
            .iter()
            .map(|(id, data)| (id.clone(), data.len()))
            .collect();

        Some((committed, uncommitted))
    }

    /// Set the access tier of a blob.
    pub async fn set_blob_tier(
        &self,
        account: &str,
        container: &str,
        blob_name: &str,
        tier: &str,
    ) -> Result<(), String> {
        let mut state = self.inner.write().await;
        let sa = Self::find_storage_account_mut(&mut state, account)
            .ok_or_else(|| format!("Storage account '{account}' not found"))?;
        let cont = sa
            .containers
            .get_mut(container)
            .ok_or_else(|| format!("Container '{container}' not found"))?;
        let blob = cont
            .blobs
            .get_mut(blob_name)
            .ok_or_else(|| format!("Blob '{blob_name}' not found"))?;
        blob.properties.access_tier = Some(tier.to_string());
        Ok(())
    }

    /// Get service properties for a storage account.
    pub async fn get_service_properties(&self, account: &str) -> Option<StorageServiceProperties> {
        let state = self.inner.read().await;
        let sa = Self::find_storage_account(&state, account)?;
        Some(sa.service_properties.clone())
    }

    /// Set service properties for a storage account.
    pub async fn set_service_properties(
        &self,
        account: &str,
        props: StorageServiceProperties,
    ) -> Result<(), String> {
        let mut state = self.inner.write().await;
        let sa = Self::find_storage_account_mut(&mut state, account)
            .ok_or_else(|| format!("Storage account '{account}' not found"))?;
        sa.service_properties = props;
        Ok(())
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

        let vm_resource_id = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group}/providers/Microsoft.Compute/virtualMachines/{name}"
        );
        let is_new = !rg.virtual_machines.contains_key(name);
        let etag_version = if is_new {
            1
        } else {
            rg.virtual_machines
                .get(name)
                .map_or(1, |v| v.etag_version + 1)
        };

        let mut props = params.properties.clone();

        // Server-generated: vmId, provisioningState, timeCreated
        props.vm_id = Some(uuid::Uuid::new_v4().to_string());
        props.provisioning_state = Some("Succeeded".to_string());
        props.time_created = Some(chrono::Utc::now().to_rfc3339());

        // Enrich osDisk: auto-generate managedDisk.id, infer osType, default diskSizeGB
        if let Some(ref mut sp) = props.storage_profile {
            if let Some(ref mut os_disk) = sp.os_disk {
                // Auto-generate managedDisk.id from disk name
                if let Some(ref mut md) = os_disk.managed_disk {
                    if md.id.is_none() {
                        let disk_name = os_disk.name.as_deref().unwrap_or("osdisk");
                        md.id = Some(format!(
                            "/subscriptions/{subscription_id}/resourceGroups/{resource_group}/providers/Microsoft.Compute/disks/{disk_name}"
                        ));
                    }
                }
                // Infer osType from osProfile configuration
                if os_disk.os_type.is_none() {
                    if let Some(ref os_profile) = props.os_profile {
                        if os_profile.linux_configuration.is_some() {
                            os_disk.os_type = Some("Linux".to_string());
                        } else if os_profile.windows_configuration.is_some() {
                            os_disk.os_type = Some("Windows".to_string());
                        }
                    }
                }
                // Default diskSizeGB
                if os_disk.disk_size_gb.is_none() {
                    os_disk.disk_size_gb = Some(30);
                }
            }

            // Enrich dataDisks: auto-generate managedDisk.id for each
            if let Some(ref mut data_disks) = sp.data_disks {
                for disk in data_disks.iter_mut() {
                    if let Some(ref mut md) = disk.managed_disk {
                        if md.id.is_none() {
                            let fallback = format!("datadisk-lun{}", disk.lun);
                            let disk_name = disk.name.as_deref().unwrap_or(&fallback);
                            md.id = Some(format!(
                                "/subscriptions/{subscription_id}/resourceGroups/{resource_group}/providers/Microsoft.Compute/disks/{disk_name}"
                            ));
                        }
                    }
                }
            }
        }

        // Strip adminPassword from stored osProfile (write-only field)
        if let Some(ref mut os_profile) = props.os_profile {
            os_profile.admin_password = None;
            // Default secrets to empty array if not provided
            if os_profile.secrets.is_none() {
                os_profile.secrets = Some(vec![]);
            }
        }

        let vm = VirtualMachine {
            id: vm_resource_id,
            name: name.to_string(),
            resource_type: "Microsoft.Compute/virtualMachines".to_string(),
            location: params.location.clone(),
            tags: params.tags.clone(),
            etag: Some(format!("\"{etag_version}\"")),
            managed_by: None,
            identity: params.identity.clone(),
            zones: params.zones.clone(),
            extended_location: params.extended_location.clone(),
            plan: params.plan.clone(),
            properties: props,
            resources: Some(vec![]),
            placement: params.placement.clone(),
            system_data: None,
        };

        rg.virtual_machines.insert(
            name.to_string(),
            VmState {
                metadata: vm.clone(),
                power_state: PowerState::Running,
                etag_version,
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

    /// PATCH update — merges the provided JSON into the stored VM.
    pub async fn update_virtual_machine(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
        patch: serde_json::Value,
    ) -> Result<VirtualMachine, String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        let vm_state = rg
            .virtual_machines
            .get_mut(name)
            .ok_or_else(|| format!("Virtual machine '{name}' not found"))?;

        // Serialize current VM to JSON, merge patch, deserialize back
        let mut current = serde_json::to_value(&vm_state.metadata).unwrap();
        json_merge(&mut current, &patch);
        let mut updated: VirtualMachine =
            serde_json::from_value(current).map_err(|e| format!("failed to apply patch: {e}"))?;

        // Bump etag
        vm_state.etag_version += 1;
        updated.etag = Some(format!("\"{}\"", vm_state.etag_version));

        // Strip adminPassword if present in patch
        if let Some(ref mut os_profile) = updated.properties.os_profile {
            os_profile.admin_password = None;
        }

        vm_state.metadata = updated.clone();
        Ok(updated)
    }

    /// List all VMs across all resource groups in a subscription.
    pub async fn list_all_virtual_machines(
        &self,
        subscription_id: &str,
    ) -> Option<Page<VirtualMachine>> {
        let state = self.inner.read().await;
        let sub = state.subscriptions.get(subscription_id)?;
        let vms: Vec<VirtualMachine> = sub
            .resource_groups
            .values()
            .flat_map(|rg| rg.virtual_machines.values().map(|vm| vm.metadata.clone()))
            .collect();
        Some(Page::new(vms))
    }

    /// List all VMs at a specific location across all resource groups.
    pub async fn list_virtual_machines_by_location(
        &self,
        subscription_id: &str,
        location: &str,
    ) -> Option<Page<VirtualMachine>> {
        let state = self.inner.read().await;
        let sub = state.subscriptions.get(subscription_id)?;
        let location_lower = location.to_lowercase();
        let vms: Vec<VirtualMachine> = sub
            .resource_groups
            .values()
            .flat_map(|rg| {
                rg.virtual_machines
                    .values()
                    .filter(|vm| vm.metadata.location.to_lowercase() == location_lower)
                    .map(|vm| vm.metadata.clone())
            })
            .collect();
        Some(Page::new(vms))
    }

    /// Mark a VM as generalized.
    pub async fn generalize_virtual_machine(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
    ) -> Result<(), String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        let vm = rg
            .virtual_machines
            .get_mut(name)
            .ok_or_else(|| format!("Virtual machine '{name}' not found"))?;
        vm.power_state = PowerState::Stopped;
        // In real Azure, generalize changes the OS state — we just mark it stopped
        Ok(())
    }

    /// Simulate eviction of a Spot VM — deallocate or delete based on eviction policy.
    pub async fn simulate_eviction(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
    ) -> Result<(), String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        let vm = rg
            .virtual_machines
            .get_mut(name)
            .ok_or_else(|| format!("Virtual machine '{name}' not found"))?;

        let policy = vm
            .metadata
            .properties
            .eviction_policy
            .as_deref()
            .unwrap_or("Deallocate");

        if policy == "Delete" {
            rg.virtual_machines.remove(name);
        } else {
            vm.power_state = PowerState::Deallocated;
        }
        Ok(())
    }

    /// Returns an Azure-compatible instance view JSON for the VM.
    pub async fn get_vm_instance_view(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
    ) -> Option<serde_json::Value> {
        let state = self.inner.read().await;
        let vm_state = state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?
            .virtual_machines
            .get(name)?;

        let power_code = match vm_state.power_state {
            PowerState::Running => "PowerState/running",
            PowerState::Stopped => "PowerState/stopped",
            PowerState::Deallocated => "PowerState/deallocated",
            PowerState::Starting => "PowerState/starting",
            PowerState::Stopping => "PowerState/stopping",
        };
        let power_display = match vm_state.power_state {
            PowerState::Running => "VM running",
            PowerState::Stopped => "VM stopped",
            PowerState::Deallocated => "VM deallocated",
            PowerState::Starting => "VM starting",
            PowerState::Stopping => "VM stopping",
        };

        let computer_name = vm_state
            .metadata
            .properties
            .os_profile
            .as_ref()
            .and_then(|p| p.computer_name.clone())
            .unwrap_or_default();

        let os_name = vm_state
            .metadata
            .properties
            .storage_profile
            .as_ref()
            .and_then(|sp| sp.os_disk.as_ref())
            .and_then(|d| d.os_type.clone())
            .unwrap_or_default();

        Some(serde_json::json!({
            "computerName": computer_name,
            "osName": os_name,
            "osVersion": "",
            "vmAgent": {
                "vmAgentVersion": "2.7.41491.1075",
                "statuses": [{
                    "code": "ProvisioningState/succeeded",
                    "level": "Info",
                    "displayStatus": "Ready",
                    "message": "GuestAgent is running and processing the extensions.",
                    "time": chrono::Utc::now().to_rfc3339()
                }]
            },
            "disks": [],
            "extensions": [],
            "statuses": [
                {
                    "code": "ProvisioningState/succeeded",
                    "level": "Info",
                    "displayStatus": "Provisioning succeeded",
                    "time": chrono::Utc::now().to_rfc3339()
                },
                {
                    "code": power_code,
                    "level": "Info",
                    "displayStatus": power_display
                }
            ]
        }))
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
                peerings: HashMap::new(),
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

    // ── Security Rules (individual CRUD within NSGs) ──────────────────

    pub async fn create_or_update_security_rule(
        &self,
        subscription_id: &str,
        resource_group: &str,
        nsg_name: &str,
        rule_name: &str,
        params: &CreateSecurityRuleParams,
    ) -> Result<(SecurityRule, bool), String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        let nsg = rg
            .network_security_groups
            .get_mut(nsg_name)
            .ok_or_else(|| format!("NSG '{nsg_name}' not found"))?;

        let nsg_id = nsg.id.clone();
        let mut props = params.properties.clone();
        props.provisioning_state = Some("Succeeded".to_string());

        let rule = SecurityRule {
            id: Some(format!("{nsg_id}/securityRules/{rule_name}")),
            name: rule_name.to_string(),
            etag: Some("\"1\"".to_string()),
            resource_type: Some(
                "Microsoft.Network/networkSecurityGroups/securityRules".to_string(),
            ),
            properties: props,
        };

        // Check if existing rule with same name
        let is_new = !nsg
            .properties
            .security_rules
            .iter()
            .any(|r| r.name == rule_name);

        // Remove existing rule if present, then add
        nsg.properties
            .security_rules
            .retain(|r| r.name != rule_name);
        nsg.properties.security_rules.push(rule.clone());

        Ok((rule, is_new))
    }

    pub async fn get_security_rule(
        &self,
        subscription_id: &str,
        resource_group: &str,
        nsg_name: &str,
        rule_name: &str,
    ) -> Option<SecurityRule> {
        let state = self.inner.read().await;
        let nsg = state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?
            .network_security_groups
            .get(nsg_name)?;

        nsg.properties
            .security_rules
            .iter()
            .find(|r| r.name == rule_name)
            .cloned()
    }

    pub async fn list_security_rules(
        &self,
        subscription_id: &str,
        resource_group: &str,
        nsg_name: &str,
    ) -> Option<Page<SecurityRule>> {
        let state = self.inner.read().await;
        let nsg = state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?
            .network_security_groups
            .get(nsg_name)?;

        Some(Page::new(nsg.properties.security_rules.clone()))
    }

    pub async fn delete_security_rule(
        &self,
        subscription_id: &str,
        resource_group: &str,
        nsg_name: &str,
        rule_name: &str,
    ) -> Result<bool, String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        let nsg = rg
            .network_security_groups
            .get_mut(nsg_name)
            .ok_or_else(|| format!("NSG '{nsg_name}' not found"))?;

        let len_before = nsg.properties.security_rules.len();
        nsg.properties
            .security_rules
            .retain(|r| r.name != rule_name);
        Ok(nsg.properties.security_rules.len() < len_before)
    }

    // ── Network Interfaces ────────────────────────────────────────────

    pub async fn create_network_interface(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
        params: &CreateNetworkInterfaceParams,
    ) -> Result<(NetworkInterface, bool), String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;

        let nic_id = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group}/providers/Microsoft.Network/networkInterfaces/{name}"
        );

        let mut props = params.properties.clone();
        props.provisioning_state = Some("Succeeded".to_string());
        props.resource_guid = Some(uuid::Uuid::new_v4().to_string());

        // Auto-generate MAC address
        if props.mac_address.is_none() {
            let mac = Self::generate_mock_mac();
            props.mac_address = Some(mac);
        }

        // Set provisioningState on IP configurations
        if let Some(ref mut ip_configs) = props.ip_configurations {
            for (i, ip_config) in ip_configs.iter_mut().enumerate() {
                if ip_config.id.is_none() {
                    let config_name = ip_config.name.as_deref().unwrap_or("ipconfig");
                    ip_config.id = Some(format!("{nic_id}/ipConfigurations/{config_name}"));
                }
                if let Some(ref mut config_props) = ip_config.properties {
                    config_props.provisioning_state = Some("Succeeded".to_string());
                    if i == 0 && config_props.primary.is_none() {
                        config_props.primary = Some(true);
                    }
                }
            }
        }

        let is_new = !rg.network_interfaces.contains_key(name);

        let nic = NetworkInterface {
            id: nic_id,
            name: name.to_string(),
            resource_type: "Microsoft.Network/networkInterfaces".to_string(),
            location: params.location.clone(),
            tags: params.tags.clone(),
            etag: Some("\"1\"".to_string()),
            properties: props,
        };

        rg.network_interfaces.insert(name.to_string(), nic.clone());
        Ok((nic, is_new))
    }

    pub async fn get_network_interface(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
    ) -> Option<NetworkInterface> {
        let state = self.inner.read().await;
        state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?
            .network_interfaces
            .get(name)
            .cloned()
    }

    pub async fn list_network_interfaces(
        &self,
        subscription_id: &str,
        resource_group: &str,
    ) -> Option<Page<NetworkInterface>> {
        let state = self.inner.read().await;
        let rg = state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?;
        let nics: Vec<NetworkInterface> = rg.network_interfaces.values().cloned().collect();
        Some(Page::new(nics))
    }

    pub async fn delete_network_interface(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
    ) -> Result<bool, String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        Ok(rg.network_interfaces.remove(name).is_some())
    }

    // ── Public IP Addresses ───────────────────────────────────────────

    pub async fn create_public_ip_address(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
        params: &CreatePublicIPAddressParams,
    ) -> Result<(PublicIPAddress, bool), String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;

        let ip_id = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group}/providers/Microsoft.Network/publicIPAddresses/{name}"
        );

        let mut props = params.properties.clone();
        props.provisioning_state = Some("Succeeded".to_string());
        props.resource_guid = Some(uuid::Uuid::new_v4().to_string());

        // Auto-generate IP address for Static allocation
        let method = props
            .public_ip_allocation_method
            .as_deref()
            .unwrap_or("Dynamic");
        if method == "Static" && props.ip_address.is_none() {
            props.ip_address = Some(Self::generate_mock_public_ip());
        }

        // Generate FQDN if dns_settings.domain_name_label is set
        if let Some(ref mut dns) = props.dns_settings {
            if let Some(ref label) = dns.domain_name_label {
                if dns.fqdn.is_none() {
                    dns.fqdn = Some(format!("{label}.eastus.cloudapp.azure.com"));
                }
            }
        }

        let is_new = !rg.public_ip_addresses.contains_key(name);

        let ip = PublicIPAddress {
            id: ip_id,
            name: name.to_string(),
            resource_type: "Microsoft.Network/publicIPAddresses".to_string(),
            location: params.location.clone(),
            tags: params.tags.clone(),
            etag: Some("\"1\"".to_string()),
            sku: params.sku.clone(),
            zones: params.zones.clone(),
            properties: props,
        };

        rg.public_ip_addresses.insert(name.to_string(), ip.clone());
        Ok((ip, is_new))
    }

    pub async fn get_public_ip_address(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
    ) -> Option<PublicIPAddress> {
        let state = self.inner.read().await;
        state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?
            .public_ip_addresses
            .get(name)
            .cloned()
    }

    pub async fn list_public_ip_addresses(
        &self,
        subscription_id: &str,
        resource_group: &str,
    ) -> Option<Page<PublicIPAddress>> {
        let state = self.inner.read().await;
        let rg = state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?;
        let ips: Vec<PublicIPAddress> = rg.public_ip_addresses.values().cloned().collect();
        Some(Page::new(ips))
    }

    pub async fn delete_public_ip_address(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
    ) -> Result<bool, String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        Ok(rg.public_ip_addresses.remove(name).is_some())
    }

    // ── VNet extended operations ─────────────────────────────────────

    /// List all virtual networks across all resource groups in a subscription.
    pub async fn list_all_virtual_networks(
        &self,
        subscription_id: &str,
    ) -> Option<Page<VirtualNetwork>> {
        let state = self.inner.read().await;
        let sub = state.subscriptions.get(subscription_id)?;
        let vnets: Vec<VirtualNetwork> = sub
            .resource_groups
            .values()
            .flat_map(|rg| {
                rg.virtual_networks.values().map(|vs| {
                    let mut v = vs.metadata.clone();
                    v.properties.subnets = vs.subnets.values().cloned().collect();
                    v
                })
            })
            .collect();
        Some(Page::new(vnets))
    }

    /// PATCH update — merge tags into the virtual network.
    pub async fn update_virtual_network_tags(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
        tags: HashMap<String, String>,
    ) -> Result<VirtualNetwork, String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        let vnet_state = rg
            .virtual_networks
            .get_mut(name)
            .ok_or_else(|| format!("Virtual network '{name}' not found"))?;

        for (k, v) in tags {
            vnet_state.metadata.tags.insert(k, v);
        }

        let mut vnet = vnet_state.metadata.clone();
        vnet.properties.subnets = vnet_state.subnets.values().cloned().collect();
        Ok(vnet)
    }

    /// Check if an IP address is available within a virtual network.
    /// Mock: returns available=true unless the IP matches a NIC's private IP.
    pub async fn check_ip_availability(
        &self,
        subscription_id: &str,
        resource_group: &str,
        _vnet_name: &str,
        ip_address: &str,
    ) -> Result<(bool, Vec<String>), String> {
        let state = self.inner.read().await;
        let rg = state
            .subscriptions
            .get(subscription_id)
            .and_then(|s| s.resource_groups.get(resource_group))
            .ok_or_else(|| format!("Resource group '{resource_group}' not found"))?;

        // Check all NICs in this RG for matching private IP
        for nic in rg.network_interfaces.values() {
            if let Some(ref ip_configs) = nic.properties.ip_configurations {
                for ip_config in ip_configs {
                    if let Some(ref props) = ip_config.properties {
                        if props.private_ip_address.as_deref() == Some(ip_address) {
                            return Ok((false, vec![]));
                        }
                    }
                }
            }
        }
        Ok((true, vec![]))
    }

    // ── NSG extended operations ───────────────────────────────────────

    /// List all NSGs across all resource groups in a subscription.
    pub async fn list_all_nsgs(&self, subscription_id: &str) -> Option<Page<NetworkSecurityGroup>> {
        let state = self.inner.read().await;
        let sub = state.subscriptions.get(subscription_id)?;
        let nsgs: Vec<NetworkSecurityGroup> = sub
            .resource_groups
            .values()
            .flat_map(|rg| rg.network_security_groups.values().cloned())
            .collect();
        Some(Page::new(nsgs))
    }

    /// PATCH update — merge tags into the NSG.
    pub async fn update_nsg_tags(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
        tags: HashMap<String, String>,
    ) -> Result<NetworkSecurityGroup, String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        let nsg = rg
            .network_security_groups
            .get_mut(name)
            .ok_or_else(|| format!("NSG '{name}' not found"))?;

        for (k, v) in tags {
            nsg.tags.insert(k, v);
        }

        Ok(nsg.clone())
    }

    // ── Route Tables ─────────────────────────────────────────────────

    pub async fn create_route_table(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
        params: &CreateRouteTableParams,
    ) -> Result<(RouteTable, bool), String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;

        let table_id = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group}/providers/Microsoft.Network/routeTables/{name}"
        );

        let mut props = params.properties.clone();
        props.provisioning_state = Some("Succeeded".to_string());
        props.resource_guid = Some(uuid::Uuid::new_v4().to_string());

        let table = RouteTable {
            id: table_id,
            name: name.to_string(),
            resource_type: "Microsoft.Network/routeTables".to_string(),
            location: params.location.clone(),
            tags: params.tags.clone(),
            etag: Some("\"1\"".to_string()),
            properties: props,
        };

        let is_new = !rg.route_tables.contains_key(name);

        // Extract routes from properties into the separate map
        let routes: HashMap<String, Route> = table
            .properties
            .routes
            .as_ref()
            .map(|rs| {
                rs.iter()
                    .filter_map(|r| r.name.clone().map(|n| (n, r.clone())))
                    .collect()
            })
            .unwrap_or_default();

        rg.route_tables.insert(
            name.to_string(),
            RouteTableState {
                metadata: table.clone(),
                routes,
            },
        );

        Ok((table, is_new))
    }

    pub async fn get_route_table(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
    ) -> Option<RouteTable> {
        let state = self.inner.read().await;
        let ts = state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?
            .route_tables
            .get(name)?;
        let mut table = ts.metadata.clone();
        table.properties.routes = Some(ts.routes.values().cloned().collect());
        Some(table)
    }

    pub async fn list_route_tables(
        &self,
        subscription_id: &str,
        resource_group: &str,
    ) -> Option<Page<RouteTable>> {
        let state = self.inner.read().await;
        let rg = state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?;
        let tables: Vec<RouteTable> = rg
            .route_tables
            .values()
            .map(|ts| {
                let mut t = ts.metadata.clone();
                t.properties.routes = Some(ts.routes.values().cloned().collect());
                t
            })
            .collect();
        Some(Page::new(tables))
    }

    pub async fn delete_route_table(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
    ) -> Result<bool, String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        Ok(rg.route_tables.remove(name).is_some())
    }

    // ── Routes (within Route Tables) ─────────────────────────────────

    pub async fn create_route(
        &self,
        subscription_id: &str,
        resource_group: &str,
        table_name: &str,
        route_name: &str,
        params: &CreateRouteParams,
    ) -> Result<(Route, bool), String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        let table = rg
            .route_tables
            .get_mut(table_name)
            .ok_or_else(|| format!("Route table '{table_name}' not found"))?;

        let mut props = params.properties.clone();
        props.provisioning_state = Some("Succeeded".to_string());

        let route = Route {
            id: Some(format!("{}/routes/{route_name}", table.metadata.id)),
            name: Some(route_name.to_string()),
            etag: Some("\"1\"".to_string()),
            resource_type: Some("Microsoft.Network/routeTables/routes".to_string()),
            properties: props,
        };

        let is_new = !table.routes.contains_key(route_name);
        table.routes.insert(route_name.to_string(), route.clone());
        Ok((route, is_new))
    }

    pub async fn get_route(
        &self,
        subscription_id: &str,
        resource_group: &str,
        table_name: &str,
        route_name: &str,
    ) -> Option<Route> {
        let state = self.inner.read().await;
        state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?
            .route_tables
            .get(table_name)?
            .routes
            .get(route_name)
            .cloned()
    }

    pub async fn list_routes(
        &self,
        subscription_id: &str,
        resource_group: &str,
        table_name: &str,
    ) -> Option<Page<Route>> {
        let state = self.inner.read().await;
        let table = state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?
            .route_tables
            .get(table_name)?;
        Some(Page::new(table.routes.values().cloned().collect()))
    }

    pub async fn delete_route(
        &self,
        subscription_id: &str,
        resource_group: &str,
        table_name: &str,
        route_name: &str,
    ) -> Result<bool, String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        let table = rg
            .route_tables
            .get_mut(table_name)
            .ok_or_else(|| format!("Route table '{table_name}' not found"))?;
        Ok(table.routes.remove(route_name).is_some())
    }

    // ── Virtual Network Peerings ─────────────────────────────────────

    pub async fn create_virtual_network_peering(
        &self,
        subscription_id: &str,
        resource_group: &str,
        vnet_name: &str,
        peering_name: &str,
        params: &CreateVirtualNetworkPeeringParams,
    ) -> Result<(VirtualNetworkPeering, bool), String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        let vnet = rg
            .virtual_networks
            .get_mut(vnet_name)
            .ok_or_else(|| format!("Virtual network '{vnet_name}' not found"))?;

        let peering_id = format!("{}/virtualNetworkPeerings/{peering_name}", vnet.metadata.id);

        let mut props = params.properties.clone();
        props.provisioning_state = Some("Succeeded".to_string());
        if props.peering_state.is_none() {
            props.peering_state = Some("Connected".to_string());
        }

        let peering = VirtualNetworkPeering {
            id: Some(peering_id),
            name: Some(peering_name.to_string()),
            etag: Some("\"1\"".to_string()),
            resource_type: Some(
                "Microsoft.Network/virtualNetworks/virtualNetworkPeerings".to_string(),
            ),
            properties: Some(props),
        };

        let is_new = !vnet.peerings.contains_key(peering_name);
        vnet.peerings
            .insert(peering_name.to_string(), peering.clone());
        Ok((peering, is_new))
    }

    pub async fn get_virtual_network_peering(
        &self,
        subscription_id: &str,
        resource_group: &str,
        vnet_name: &str,
        peering_name: &str,
    ) -> Option<VirtualNetworkPeering> {
        let state = self.inner.read().await;
        state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?
            .virtual_networks
            .get(vnet_name)?
            .peerings
            .get(peering_name)
            .cloned()
    }

    pub async fn list_virtual_network_peerings(
        &self,
        subscription_id: &str,
        resource_group: &str,
        vnet_name: &str,
    ) -> Option<Page<VirtualNetworkPeering>> {
        let state = self.inner.read().await;
        let vnet = state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?
            .virtual_networks
            .get(vnet_name)?;
        Some(Page::new(vnet.peerings.values().cloned().collect()))
    }

    pub async fn delete_virtual_network_peering(
        &self,
        subscription_id: &str,
        resource_group: &str,
        vnet_name: &str,
        peering_name: &str,
    ) -> Result<bool, String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        let vnet = rg
            .virtual_networks
            .get_mut(vnet_name)
            .ok_or_else(|| format!("Virtual network '{vnet_name}' not found"))?;
        Ok(vnet.peerings.remove(peering_name).is_some())
    }

    // ── Application Security Groups ─────────────────────────────────

    pub async fn create_application_security_group(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
        params: &CreateApplicationSecurityGroupParams,
    ) -> Result<(ApplicationSecurityGroup, bool), String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;

        let asg = ApplicationSecurityGroup {
            id: format!(
                "/subscriptions/{subscription_id}/resourceGroups/{resource_group}/providers/Microsoft.Network/applicationSecurityGroups/{name}"
            ),
            name: name.to_string(),
            resource_type: "Microsoft.Network/applicationSecurityGroups".to_string(),
            location: params.location.clone(),
            tags: params.tags.clone(),
            etag: Some("\"1\"".to_string()),
            properties: ApplicationSecurityGroupProperties {
                provisioning_state: Some("Succeeded".to_string()),
                resource_guid: Some(uuid::Uuid::new_v4().to_string()),
            },
        };

        let is_new = !rg.application_security_groups.contains_key(name);
        rg.application_security_groups
            .insert(name.to_string(), asg.clone());
        Ok((asg, is_new))
    }

    pub async fn get_application_security_group(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
    ) -> Option<ApplicationSecurityGroup> {
        let state = self.inner.read().await;
        state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?
            .application_security_groups
            .get(name)
            .cloned()
    }

    pub async fn list_application_security_groups(
        &self,
        subscription_id: &str,
        resource_group: &str,
    ) -> Option<Page<ApplicationSecurityGroup>> {
        let state = self.inner.read().await;
        let rg = state
            .subscriptions
            .get(subscription_id)?
            .resource_groups
            .get(resource_group)?;
        let asgs: Vec<ApplicationSecurityGroup> =
            rg.application_security_groups.values().cloned().collect();
        Some(Page::new(asgs))
    }

    pub async fn list_all_application_security_groups(
        &self,
        subscription_id: &str,
    ) -> Option<Page<ApplicationSecurityGroup>> {
        let state = self.inner.read().await;
        let sub = state.subscriptions.get(subscription_id)?;
        let asgs: Vec<ApplicationSecurityGroup> = sub
            .resource_groups
            .values()
            .flat_map(|rg| rg.application_security_groups.values().cloned())
            .collect();
        Some(Page::new(asgs))
    }

    pub async fn delete_application_security_group(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
    ) -> Result<bool, String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        Ok(rg.application_security_groups.remove(name).is_some())
    }

    pub async fn update_application_security_group_tags(
        &self,
        subscription_id: &str,
        resource_group: &str,
        name: &str,
        tags: HashMap<String, String>,
    ) -> Result<ApplicationSecurityGroup, String> {
        let mut state = self.inner.write().await;
        let rg = Self::get_rg_mut(&mut state, subscription_id, resource_group)?;
        let asg = rg
            .application_security_groups
            .get_mut(name)
            .ok_or_else(|| format!("Application security group '{name}' not found"))?;

        for (k, v) in tags {
            asg.tags.insert(k, v);
        }

        Ok(asg.clone())
    }

    // ── Service Tags ────────────────────────────────────────────────

    pub async fn list_service_tags(&self, location: &str) -> ServiceTagsListResult {
        ServiceTagsListResult {
            name: Some(format!("ServiceTagsListResult_{location}")),
            id: Some(format!(
                "/subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Network/locations/{location}/serviceTags/default"
            )),
            resource_type: Some("Microsoft.Network/serviceTags".to_string()),
            change_number: Some("1".to_string()),
            cloud: Some("Public".to_string()),
            values: Some(vec![
                ServiceTagInformation {
                    name: Some("AzureCloud".to_string()),
                    id: Some("AzureCloud".to_string()),
                    service_tag_change_number: Some("1".to_string()),
                    properties: Some(ServiceTagInformationProperties {
                        change_number: Some("1".to_string()),
                        region: Some(String::new()),
                        system_service: Some("AzureCloud".to_string()),
                        address_prefixes: Some(vec![
                            "13.64.0.0/11".to_string(),
                            "13.96.0.0/13".to_string(),
                        ]),
                    }),
                },
                ServiceTagInformation {
                    name: Some("Storage".to_string()),
                    id: Some("Storage".to_string()),
                    service_tag_change_number: Some("1".to_string()),
                    properties: Some(ServiceTagInformationProperties {
                        change_number: Some("1".to_string()),
                        region: Some(String::new()),
                        system_service: Some("AzureStorage".to_string()),
                        address_prefixes: Some(vec![
                            "20.33.0.0/16".to_string(),
                            "20.34.0.0/15".to_string(),
                        ]),
                    }),
                },
                ServiceTagInformation {
                    name: Some("Sql".to_string()),
                    id: Some("Sql".to_string()),
                    service_tag_change_number: Some("1".to_string()),
                    properties: Some(ServiceTagInformationProperties {
                        change_number: Some("1".to_string()),
                        region: Some(String::new()),
                        system_service: Some("AzureSQL".to_string()),
                        address_prefixes: Some(vec!["20.36.0.0/14".to_string()]),
                    }),
                },
                ServiceTagInformation {
                    name: Some("AzureActiveDirectory".to_string()),
                    id: Some("AzureActiveDirectory".to_string()),
                    service_tag_change_number: Some("1".to_string()),
                    properties: Some(ServiceTagInformationProperties {
                        change_number: Some("1".to_string()),
                        region: Some(String::new()),
                        system_service: Some("AzureAD".to_string()),
                        address_prefixes: Some(vec!["20.190.128.0/18".to_string()]),
                    }),
                },
            ]),
        }
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

    /// Generate a deterministic-looking mock MAC address.
    fn generate_mock_mac() -> String {
        let u = uuid::Uuid::new_v4();
        let bytes = u.as_bytes();
        format!(
            "{:02X}-{:02X}-{:02X}-{:02X}-{:02X}-{:02X}",
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
        )
    }

    /// Generate a mock public IP address (10.x.x.x range for test safety).
    fn generate_mock_public_ip() -> String {
        let u = uuid::Uuid::new_v4();
        let bytes = u.as_bytes();
        format!("20.{}.{}.{}", bytes[0], bytes[1], bytes[2])
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

/// RFC 7396 JSON Merge Patch — recursively merge `patch` into `target`.
fn json_merge(target: &mut serde_json::Value, patch: &serde_json::Value) {
    if let serde_json::Value::Object(patch_obj) = patch {
        if let serde_json::Value::Object(target_obj) = target {
            for (key, value) in patch_obj {
                if value.is_null() {
                    target_obj.remove(key);
                } else if value.is_object() {
                    let entry = target_obj
                        .entry(key.clone())
                        .or_insert(serde_json::Value::Object(serde_json::Map::new()));
                    json_merge(entry, value);
                } else {
                    target_obj.insert(key.clone(), value.clone());
                }
            }
        } else {
            *target = patch.clone();
        }
    } else {
        *target = patch.clone();
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
            properties: None,
            identity: None,
            extended_location: None,
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
