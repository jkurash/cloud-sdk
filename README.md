# cloud-sdk

A cloud-agnostic SDK for Rust that lets you develop applications backed by cloud resources in a local sandbox, without incurring costs against real cloud APIs.

The SDK provides a full mock server that replicates cloud provider REST APIs locally. Your application code works identically against the mock and the real cloud -- the only difference is a base URL swap.

Starting with **Azure**, the architecture is trait-based for future extension to AWS, GCP, and other providers.

## Quick Start

### Using the SDK with the mock server

```rust
use cloud_sdk_azure_client::{AzureClient, AzureProvider, auth::ClientSecretCredential};
use cloud_sdk_azure_mock::{AzureMockConfig, AzureMockServer};
use cloud_sdk_core::services::ResourceManagerService;
use cloud_sdk_core::models::resource::CreateResourceGroupParams;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Start the mock server
    let config = AzureMockConfig::from_file("azure-mock.toml").unwrap();
    let mock = AzureMockServer::from_config(config)
        .start_on_random_port()
        .await
        .unwrap();

    // Create a client pointed at the mock (same code works against real Azure)
    let credential = ClientSecretCredential::with_authority(
        "my-tenant-id",
        "my-client-id",
        "my-client-secret",
        mock.url(), // swap to "https://login.microsoftonline.com" for real Azure
    );

    let client = AzureClient::builder()
        .arm_base_url(mock.url()) // swap to "https://management.azure.com" for real Azure
        .credential(credential)
        .subscription_id("00000000-0000-0000-0000-000000000000")
        .build()
        .unwrap();

    let provider = AzureProvider::new(client);

    // Use the SDK -- identical code for mock and real Azure
    let rg = provider
        .resource_manager()
        .create_resource_group(
            "my-resource-group",
            CreateResourceGroupParams {
                location: "eastus".to_string(),
                tags: HashMap::new(),
            },
        )
        .await
        .unwrap();

    println!("Created resource group: {}", rg.name);
}
```

### Mock server configuration

The mock server requires an `azure-mock.toml` configuration file:

```toml
[server]
bind = "127.0.0.1"
port = 8080

[subscriptions.default]
id = "00000000-0000-0000-0000-000000000000"
display_name = "My Mock Subscription"
tenant_id = "00000000-0000-0000-0000-000000000001"
state = "Enabled"

# Optional: pre-populate resources
[[subscriptions.default.resource_groups]]
name = "dev-rg"
location = "eastus"

[[subscriptions.default.resource_groups]]
name = "staging-rg"
location = "westus2"
tags = { env = "staging" }
```

## Azure Services Implemented

### Management Plane (ARM)

| Service | Operations |
|---------|-----------|
| **Subscriptions** | List, Get |
| **Resource Groups** | Create/Update, Get, List, Delete, Check Existence, Update Tags |
| **Storage Accounts** | Create/Update, Get, List, Delete |
| **Virtual Machines** | Create/Update, Get, List, Delete, Start, Stop, Restart, Deallocate |
| **Virtual Networks** | Create/Update, Get, List, Delete |
| **Subnets** | Create/Update, Get, List, Delete |
| **Network Security Groups** | Create/Update, Get, List, Delete |
| **Identity** | Get Current Principal, List Role Assignments |

### Data Plane

| Service | Operations |
|---------|-----------|
| **Blob Containers** | Create, Delete, List |
| **Blobs** | Put, Get, Delete, List, Head (properties) |

### Authentication

| Method | Description |
|--------|------------|
| `MockCredential` | Fixed token for local development |
| `ClientSecretCredential` | Service principal (client ID + secret). Configurable authority URL for mock or real Azure AD. |
| `AzureCliCredential` | Shells out to `az account get-access-token` |
| `ChainedCredential` | Tries multiple credentials in order (like `DefaultAzureCredential`) |

The mock server includes an OAuth2 token endpoint (`POST /{tenantId}/oauth2/v2.0/token`) so `ClientSecretCredential` works against the mock with zero code changes.

## Workspace Structure

```
cloud-sdk/
  crates/
    cloud-sdk-core/              Shared traits, error types, models (zero provider deps)
    cloud-sdk-azure-client/      Azure REST API client, auth, AzureProvider
    cloud-sdk-azure-mock/        Mock Azure HTTP server (axum-based)
    cloud-sdk-test/              Test harness wiring mock + client together
    cloud-sdk-cli/               Standalone mock server binary (planned)
    cloud-sdk/                   Facade crate with feature flags
```

The `azure-client` and `azure-mock` crates are **independent** -- the mock is a standalone HTTP server that does not depend on the client. This means:

- Production builds only pull in `cloud-sdk-azure-client`
- The mock server can be used with any HTTP client, not just this SDK
- Adding a new provider (e.g., AWS) is a new pair of crates with zero impact on Azure

## Mock Server Fidelity

The mock replicates Azure's REST API structure exactly:

- **URL patterns**: `/subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.Compute/virtualMachines/{vm}`
- **Request/response JSON**: matches Azure's schemas (camelCase field names, `properties` envelope, `provisioningState`)
- **Status codes**: 200/201 for PUT, 200 for GET, 200/202 for DELETE, 204/404 for HEAD existence checks
- **Error format**: `{ "error": { "code": "...", "message": "..." } }` matching Azure's `CloudError`
- **Headers**: `x-ms-request-id`, `x-ms-correlation-id`, `Content-Type: application/json`
- **Middleware**: validates `api-version` query parameter and `Authorization: Bearer` header on ARM routes
- **Pagination**: `{ "value": [...], "nextLink": "..." }` envelope on list responses

## Building

```sh
cargo build --workspace       # Build all crates
cargo test --workspace        # Run all 68 tests
cargo clippy --workspace      # Lint
cargo fmt --all               # Format
```

## License

Apache-2.0
