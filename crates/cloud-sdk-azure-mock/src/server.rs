use axum::{
    Router,
    routing::{get, post, put},
};
use std::sync::Arc;
use tokio::task::JoinHandle;

use crate::config::AzureMockConfig;
use crate::middleware::{api_version, auth, delay, request_id};
use crate::routes::{
    blobs, compute, identity, networking, oauth, resource_groups, storage_accounts, subscriptions,
};
use crate::state::MockState;

/// Mock Azure REST API server.
///
/// Implements Azure ARM endpoints and blob data plane with in-memory state.
/// Requires an `AzureMockConfig` to start.
pub struct AzureMockServer {
    config: AzureMockConfig,
    state: Arc<MockState>,
}

/// Handle to a running mock server instance.
pub struct AzureMockServerHandle {
    pub port: u16,
    _handle: JoinHandle<()>,
}

impl AzureMockServerHandle {
    /// Base URL of the running mock server (e.g., `http://127.0.0.1:12345`).
    pub fn url(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }
}

impl AzureMockServer {
    /// Create a mock server from a config.
    pub fn from_config(config: AzureMockConfig) -> Self {
        let state = MockState::from_config(&config);
        Self {
            config,
            state: Arc::new(state),
        }
    }

    /// Create a mock server by loading config from a TOML file.
    pub fn from_file(
        path: impl AsRef<std::path::Path>,
    ) -> Result<Self, crate::config::ConfigError> {
        let config = AzureMockConfig::from_file(path)?;
        Ok(Self::from_config(config))
    }

    /// Build the axum router with all routes and middleware.
    pub fn into_router(self) -> Router {
        // ARM management plane routes (require api-version + auth)
        let arm_routes = Router::new()
            // Subscriptions
            .route("/subscriptions", get(subscriptions::list_subscriptions))
            .route(
                "/subscriptions/{subscriptionId}",
                get(subscriptions::get_subscription),
            )
            // Resource Groups
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}",
                put(resource_groups::create_or_update)
                    .get(resource_groups::get)
                    .delete(resource_groups::delete)
                    .patch(resource_groups::update)
                    .head(resource_groups::check_existence),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups",
                get(resource_groups::list),
            )
            // Storage Accounts (ARM)
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Storage/storageAccounts/{accountName}",
                put(storage_accounts::create_or_update)
                    .get(storage_accounts::get)
                    .delete(storage_accounts::delete)
                    .patch(storage_accounts::update),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Storage/storageAccounts",
                get(storage_accounts::list),
            )
            // Storage Accounts — subscription-wide
            .route(
                "/subscriptions/{subscriptionId}/providers/Microsoft.Storage/storageAccounts",
                get(storage_accounts::list_all),
            )
            .route(
                "/subscriptions/{subscriptionId}/providers/Microsoft.Storage/checkNameAvailability",
                post(storage_accounts::check_name_availability),
            )
            // Storage Accounts — key & SAS operations
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Storage/storageAccounts/{accountName}/listKeys",
                post(storage_accounts::list_keys),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Storage/storageAccounts/{accountName}/regenerateKey",
                post(storage_accounts::regenerate_key),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Storage/storageAccounts/{accountName}/ListAccountSas",
                post(storage_accounts::list_account_sas),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Storage/storageAccounts/{accountName}/ListServiceSas",
                post(storage_accounts::list_service_sas),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Storage/storageAccounts/{accountName}/revokeUserDelegationKeys",
                post(storage_accounts::revoke_user_delegation_keys),
            )
            // Virtual Machines (ARM) — CRUD
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}",
                put(compute::create_or_update)
                    .get(compute::get)
                    .delete(compute::delete)
                    .patch(compute::update),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines",
                get(compute::list),
            )
            // VM — subscription-wide listing
            .route(
                "/subscriptions/{subscriptionId}/providers/Microsoft.Compute/virtualMachines",
                get(compute::list_all),
            )
            // VM — list by location
            .route(
                "/subscriptions/{subscriptionId}/providers/Microsoft.Compute/locations/{location}/virtualMachines",
                get(compute::list_by_location),
            )
            // VM — instance view + available sizes
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}/instanceView",
                get(compute::instance_view),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}/vmSizes",
                get(compute::list_available_sizes),
            )
            // VM — power operations
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}/start",
                post(compute::start),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}/powerOff",
                post(compute::power_off),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}/restart",
                post(compute::restart),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}/deallocate",
                post(compute::deallocate),
            )
            // VM — lifecycle operations
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}/generalize",
                post(compute::generalize),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}/reapply",
                post(compute::reapply),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}/simulateEviction",
                post(compute::simulate_eviction),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}/redeploy",
                post(compute::redeploy),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}/reimage",
                post(compute::reimage),
            )
            // Virtual Networks
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Network/virtualNetworks/{vnetName}",
                put(networking::create_or_update_vnet).get(networking::get_vnet).delete(networking::delete_vnet),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Network/virtualNetworks",
                get(networking::list_vnets),
            )
            // Subnets
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Network/virtualNetworks/{vnetName}/subnets/{subnetName}",
                put(networking::create_or_update_subnet).get(networking::get_subnet).delete(networking::delete_subnet),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Network/virtualNetworks/{vnetName}/subnets",
                get(networking::list_subnets),
            )
            // Network Security Groups
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Network/networkSecurityGroups/{nsgName}",
                put(networking::create_or_update_nsg).get(networking::get_nsg).delete(networking::delete_nsg),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Network/networkSecurityGroups",
                get(networking::list_nsgs),
            )
            // Security Rules (individual CRUD within NSGs)
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Network/networkSecurityGroups/{nsgName}/securityRules/{ruleName}",
                put(networking::create_or_update_security_rule).get(networking::get_security_rule).delete(networking::delete_security_rule),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Network/networkSecurityGroups/{nsgName}/securityRules",
                get(networking::list_security_rules),
            )
            // Network Interfaces
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Network/networkInterfaces/{nicName}",
                put(networking::create_or_update_nic).get(networking::get_nic).delete(networking::delete_nic),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Network/networkInterfaces",
                get(networking::list_nics),
            )
            // Public IP Addresses
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Network/publicIPAddresses/{ipName}",
                put(networking::create_or_update_public_ip).get(networking::get_public_ip).delete(networking::delete_public_ip),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Network/publicIPAddresses",
                get(networking::list_public_ips),
            )
            // Identity
            .route("/me", get(identity::get_current_principal))
            .route(
                "/providers/Microsoft.Authorization/roleAssignments",
                get(identity::list_role_assignments),
            )
            .layer(axum::middleware::from_fn(api_version::validate_api_version))
            .layer(axum::middleware::from_fn(auth::validate_auth));

        // Blob data plane routes (no api-version required, use x-ms-version header)
        let blob_routes = Router::new()
            // List containers: GET /{account}?comp=list
            .route("/{account}", get(blobs::list_containers))
            // Container ops: PUT/DELETE /{account}/{container}?restype=container
            // List blobs: GET /{account}/{container}?restype=container&comp=list
            .route(
                "/{account}/{container}",
                put(blobs::create_container)
                    .delete(blobs::delete_container)
                    .get(blobs::list_blobs),
            )
            // Blob ops: PUT/GET/DELETE/HEAD /{account}/{container}/{blob}
            .route(
                "/{account}/{container}/{blob}",
                put(blobs::put_blob)
                    .get(blobs::get_blob)
                    .delete(blobs::delete_blob)
                    .head(blobs::head_blob),
            );

        // OAuth2 token endpoint (no auth/api-version middleware — this IS the auth endpoint)
        let oauth_routes = Router::new().route("/{tenantId}/oauth2/v2.0/token", post(oauth::token));

        // Combine all under shared state + response headers + delay
        let delay_ms = self.config.server.delay_ms;
        Router::new()
            .merge(arm_routes)
            .merge(blob_routes)
            .merge(oauth_routes)
            .layer(axum::middleware::from_fn(request_id::add_response_headers))
            .layer(axum::middleware::from_fn(delay::make_delay_middleware(
                delay_ms,
            )))
            .with_state(self.state)
    }

    /// Start the mock server using the port from config.
    pub async fn start(self) -> Result<AzureMockServerHandle, std::io::Error> {
        let addr = format!("{}:{}", self.config.server.bind, self.config.server.port);
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        let port = listener.local_addr()?.port();
        let router = self.into_router();
        let handle = tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });

        Ok(AzureMockServerHandle {
            port,
            _handle: handle,
        })
    }

    /// Start the mock server on a random available port (ignores config port).
    /// Useful for tests where port conflicts must be avoided.
    pub async fn start_on_random_port(self) -> Result<AzureMockServerHandle, std::io::Error> {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let port = listener.local_addr()?.port();
        let router = self.into_router();
        let handle = tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });

        Ok(AzureMockServerHandle {
            port,
            _handle: handle,
        })
    }
}
