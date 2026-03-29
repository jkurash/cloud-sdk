use cloud_sdk_core::auth::Credential;
use cloud_sdk_core::error::CloudSdkError;
use url::Url;

use crate::auth::BoxedCredential;
use crate::client::AzureClient;

/// Configuration for connecting to Azure (or a mock server).
#[derive(Clone)]
pub struct AzureConfig {
    pub subscription_id: String,
    pub arm_base_url: Url,
    pub storage_base_url: Option<Url>,
}

impl AzureConfig {
    /// Build the ARM URL for a resource group.
    pub fn resource_group_url(&self, rg_name: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}",
            self.subscription_id, rg_name
        );
        self.arm_base_url.join(&path).unwrap()
    }

    /// Build the ARM URL for listing resource groups.
    pub fn resource_groups_url(&self) -> Url {
        let path = format!("subscriptions/{}/resourcegroups", self.subscription_id);
        self.arm_base_url.join(&path).unwrap()
    }

    /// Build the ARM URL for a subscription.
    pub fn subscription_url(&self, subscription_id: &str) -> Url {
        let path = format!("subscriptions/{subscription_id}");
        self.arm_base_url.join(&path).unwrap()
    }

    /// Build the ARM URL for listing subscriptions.
    pub fn subscriptions_url(&self) -> Url {
        self.arm_base_url.join("subscriptions").unwrap()
    }

    // ── Identity URLs ───────────────────────────────────────────────

    pub fn me_url(&self) -> Url {
        self.arm_base_url.join("me").unwrap()
    }

    pub fn role_assignments_url(&self) -> Url {
        self.arm_base_url
            .join("providers/Microsoft.Authorization/roleAssignments")
            .unwrap()
    }

    // ── Compute ARM URLs ─────────────────────────────────────────────

    /// ARM URL for a virtual machine.
    pub fn virtual_machine_url(&self, rg_name: &str, vm_name: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Compute/virtualMachines/{}",
            self.subscription_id, rg_name, vm_name
        );
        self.arm_base_url.join(&path).unwrap()
    }

    /// ARM URL for listing virtual machines in a resource group.
    pub fn virtual_machines_url(&self, rg_name: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Compute/virtualMachines",
            self.subscription_id, rg_name
        );
        self.arm_base_url.join(&path).unwrap()
    }

    /// ARM URL for a VM power action (start, powerOff, restart, deallocate).
    pub fn virtual_machine_action_url(&self, rg_name: &str, vm_name: &str, action: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Compute/virtualMachines/{}/{}",
            self.subscription_id, rg_name, vm_name, action
        );
        self.arm_base_url.join(&path).unwrap()
    }

    /// ARM URL for listing all VMs in a subscription.
    pub fn virtual_machines_all_url(&self) -> Url {
        let path = format!(
            "subscriptions/{}/providers/Microsoft.Compute/virtualMachines",
            self.subscription_id
        );
        self.arm_base_url.join(&path).unwrap()
    }

    /// ARM URL for listing VMs by location.
    pub fn virtual_machines_by_location_url(&self, location: &str) -> Url {
        let path = format!(
            "subscriptions/{}/providers/Microsoft.Compute/locations/{}/virtualMachines",
            self.subscription_id, location
        );
        self.arm_base_url.join(&path).unwrap()
    }

    /// ARM URL for listing available VM sizes.
    pub fn virtual_machine_sizes_url(&self, rg_name: &str, vm_name: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Compute/virtualMachines/{}/vmSizes",
            self.subscription_id, rg_name, vm_name
        );
        self.arm_base_url.join(&path).unwrap()
    }

    // ── Networking ARM URLs ──────────────────────────────────────────

    pub fn virtual_network_url(&self, rg: &str, vnet: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Network/virtualNetworks/{}",
            self.subscription_id, rg, vnet
        );
        self.arm_base_url.join(&path).unwrap()
    }

    pub fn virtual_networks_url(&self, rg: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Network/virtualNetworks",
            self.subscription_id, rg
        );
        self.arm_base_url.join(&path).unwrap()
    }

    pub fn subnet_url(&self, rg: &str, vnet: &str, subnet: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Network/virtualNetworks/{}/subnets/{}",
            self.subscription_id, rg, vnet, subnet
        );
        self.arm_base_url.join(&path).unwrap()
    }

    pub fn subnets_url(&self, rg: &str, vnet: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Network/virtualNetworks/{}/subnets",
            self.subscription_id, rg, vnet
        );
        self.arm_base_url.join(&path).unwrap()
    }

    pub fn nsg_url(&self, rg: &str, nsg: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Network/networkSecurityGroups/{}",
            self.subscription_id, rg, nsg
        );
        self.arm_base_url.join(&path).unwrap()
    }

    pub fn nsgs_url(&self, rg: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Network/networkSecurityGroups",
            self.subscription_id, rg
        );
        self.arm_base_url.join(&path).unwrap()
    }

    pub fn security_rule_url(&self, rg: &str, nsg: &str, rule: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Network/networkSecurityGroups/{}/securityRules/{}",
            self.subscription_id, rg, nsg, rule
        );
        self.arm_base_url.join(&path).unwrap()
    }

    pub fn security_rules_url(&self, rg: &str, nsg: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Network/networkSecurityGroups/{}/securityRules",
            self.subscription_id, rg, nsg
        );
        self.arm_base_url.join(&path).unwrap()
    }

    pub fn network_interface_url(&self, rg: &str, nic: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Network/networkInterfaces/{}",
            self.subscription_id, rg, nic
        );
        self.arm_base_url.join(&path).unwrap()
    }

    pub fn network_interfaces_url(&self, rg: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Network/networkInterfaces",
            self.subscription_id, rg
        );
        self.arm_base_url.join(&path).unwrap()
    }

    pub fn public_ip_address_url(&self, rg: &str, ip: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Network/publicIPAddresses/{}",
            self.subscription_id, rg, ip
        );
        self.arm_base_url.join(&path).unwrap()
    }

    pub fn public_ip_addresses_url(&self, rg: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Network/publicIPAddresses",
            self.subscription_id, rg
        );
        self.arm_base_url.join(&path).unwrap()
    }

    /// ARM URL for listing all VNets in a subscription.
    pub fn virtual_networks_all_url(&self) -> Url {
        let path = format!(
            "subscriptions/{}/providers/Microsoft.Network/virtualNetworks",
            self.subscription_id
        );
        self.arm_base_url.join(&path).unwrap()
    }

    /// ARM URL for listing all NSGs in a subscription.
    pub fn nsgs_all_url(&self) -> Url {
        let path = format!(
            "subscriptions/{}/providers/Microsoft.Network/networkSecurityGroups",
            self.subscription_id
        );
        self.arm_base_url.join(&path).unwrap()
    }

    // ── Route Table ARM URLs ────────────────────────────────────────────

    pub fn route_table_url(&self, rg: &str, table: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Network/routeTables/{}",
            self.subscription_id, rg, table
        );
        self.arm_base_url.join(&path).unwrap()
    }

    pub fn route_tables_url(&self, rg: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Network/routeTables",
            self.subscription_id, rg
        );
        self.arm_base_url.join(&path).unwrap()
    }

    pub fn route_url(&self, rg: &str, table: &str, route: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Network/routeTables/{}/routes/{}",
            self.subscription_id, rg, table, route
        );
        self.arm_base_url.join(&path).unwrap()
    }

    pub fn routes_url(&self, rg: &str, table: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Network/routeTables/{}/routes",
            self.subscription_id, rg, table
        );
        self.arm_base_url.join(&path).unwrap()
    }

    // ── VNet Peering ARM URLs ────────────────────────────────────���──────

    pub fn virtual_network_peering_url(&self, rg: &str, vnet: &str, peering: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Network/virtualNetworks/{}/virtualNetworkPeerings/{}",
            self.subscription_id, rg, vnet, peering
        );
        self.arm_base_url.join(&path).unwrap()
    }

    pub fn virtual_network_peerings_url(&self, rg: &str, vnet: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Network/virtualNetworks/{}/virtualNetworkPeerings",
            self.subscription_id, rg, vnet
        );
        self.arm_base_url.join(&path).unwrap()
    }

    // ── Storage Account ARM URLs ───────────────────────────────────────

    /// ARM URL for a storage account.
    pub fn storage_account_url(&self, rg_name: &str, account_name: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Storage/storageAccounts/{}",
            self.subscription_id, rg_name, account_name
        );
        self.arm_base_url.join(&path).unwrap()
    }

    /// ARM URL for listing storage accounts in a resource group.
    pub fn storage_accounts_url(&self, rg_name: &str) -> Url {
        let path = format!(
            "subscriptions/{}/resourcegroups/{}/providers/Microsoft.Storage/storageAccounts",
            self.subscription_id, rg_name
        );
        self.arm_base_url.join(&path).unwrap()
    }

    // ── Blob data plane URLs ───────────────────────────────────────────

    /// Data plane URL for container operations: `{base}/{account}/{container}`
    pub fn blob_container_url(&self, account: &str, container: &str) -> Option<Url> {
        let base = self.storage_base_url.as_ref()?;
        Some(base.join(&format!("{account}/{container}")).unwrap())
    }

    /// Data plane URL for listing containers: `{base}/{account}`
    pub fn blob_account_url(&self, account: &str) -> Option<Url> {
        let base = self.storage_base_url.as_ref()?;
        Some(base.join(account).unwrap())
    }

    /// Data plane URL for blob operations: `{base}/{account}/{container}/{blob}`
    pub fn blob_url(&self, account: &str, container: &str, blob: &str) -> Option<Url> {
        let base = self.storage_base_url.as_ref()?;
        Some(base.join(&format!("{account}/{container}/{blob}")).unwrap())
    }
}

/// Builder for constructing an `AzureClient`.
pub struct AzureClientBuilder {
    subscription_id: Option<String>,
    arm_base_url: Option<String>,
    storage_base_url: Option<String>,
    credential: Option<BoxedCredential>,
}

impl AzureClientBuilder {
    pub fn new() -> Self {
        Self {
            subscription_id: None,
            arm_base_url: None,
            storage_base_url: None,
            credential: None,
        }
    }

    pub fn subscription_id(mut self, id: impl Into<String>) -> Self {
        self.subscription_id = Some(id.into());
        self
    }

    /// Set the ARM base URL. Defaults to `https://management.azure.com`.
    pub fn arm_base_url(mut self, url: impl Into<String>) -> Self {
        self.arm_base_url = Some(url.into());
        self
    }

    /// Set the storage data plane base URL (for blob operations).
    pub fn storage_base_url(mut self, url: impl Into<String>) -> Self {
        self.storage_base_url = Some(url.into());
        self
    }

    /// Set the credential provider.
    pub fn credential(mut self, cred: impl Credential + 'static) -> Self {
        self.credential = Some(BoxedCredential::new(cred));
        self
    }

    pub fn build(self) -> Result<AzureClient, CloudSdkError> {
        let subscription_id =
            self.subscription_id
                .ok_or_else(|| CloudSdkError::ValidationError {
                    message: "subscription_id is required".to_string(),
                })?;

        let credential = self
            .credential
            .ok_or_else(|| CloudSdkError::ValidationError {
                message: "credential is required".to_string(),
            })?;

        let arm_base_str = self
            .arm_base_url
            .unwrap_or_else(|| "https://management.azure.com".to_string());
        // Ensure trailing slash for proper URL joining
        let arm_base_str = if arm_base_str.ends_with('/') {
            arm_base_str
        } else {
            format!("{arm_base_str}/")
        };
        let arm_base_url =
            Url::parse(&arm_base_str).map_err(|e| CloudSdkError::ValidationError {
                message: format!("invalid arm_base_url: {e}"),
            })?;

        let storage_base_url = self
            .storage_base_url
            .map(|s| {
                let s = if s.ends_with('/') { s } else { format!("{s}/") };
                Url::parse(&s)
            })
            .transpose()
            .map_err(|e| CloudSdkError::ValidationError {
                message: format!("invalid storage_base_url: {e}"),
            })?;

        let config = AzureConfig {
            subscription_id,
            arm_base_url,
            storage_base_url,
        };

        Ok(AzureClient::new(config, credential))
    }
}

impl Default for AzureClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
