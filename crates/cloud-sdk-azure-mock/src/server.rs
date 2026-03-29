use axum::{
    Router,
    routing::{get, post, put},
};
use std::sync::Arc;
use tokio::task::JoinHandle;

use crate::config::AzureMockConfig;
use crate::middleware::{api_version, auth, request_id};
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
                    .delete(storage_accounts::delete),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Storage/storageAccounts",
                get(storage_accounts::list),
            )
            // Virtual Machines (ARM)
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}",
                put(compute::create_or_update)
                    .get(compute::get)
                    .delete(compute::delete),
            )
            .route(
                "/subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines",
                get(compute::list),
            )
            // VM power operations
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

        // Combine all under shared state + response headers
        Router::new()
            .merge(arm_routes)
            .merge(blob_routes)
            .merge(oauth_routes)
            .layer(axum::middleware::from_fn(request_id::add_response_headers))
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
