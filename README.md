# cloud-sdk

> **Disclaimer:** This is an unofficial, community-built project. It is **not affiliated with, endorsed by, or sponsored by Microsoft Corporation**. "Azure", "Azure Resource Manager", and related terms are trademarks of Microsoft Corporation. This project uses these names solely for descriptive, nominative purposes to indicate compatibility with publicly documented API behavior.

A cloud-agnostic SDK for Rust that lets you develop applications backed by cloud resources in a **local sandbox**, without incurring costs against real cloud APIs.

The SDK provides a mock server that implements a subset of publicly documented ARM (Azure Resource Manager) endpoint behavior for local testing. Your application code works against the mock during development and against the real cloud provider in production — the only difference is a base URL swap.

This is an **independent, clean-room implementation** built entirely from [publicly available REST API documentation](https://learn.microsoft.com/en-us/rest/api/azure/). It does not contain any Microsoft code, SDK internals, proprietary material, or copied implementation details. All mock responses are independently generated fixtures, not reproductions of real service output.

## What This Project Is

- An **unofficial local test double** for selected ARM-compatible endpoints
- A **development tool** for building and testing Rust applications that will eventually run against real cloud infrastructure
- A **community project** with no claim of completeness or parity with real cloud services

## What This Project Is NOT

- An official Microsoft product or service
- A substitute for real cloud infrastructure in production
- A tool to circumvent cloud provider billing or technical controls
- A complete or authoritative implementation of any cloud provider's API

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

    // Create a client pointed at the mock
    let credential = ClientSecretCredential::with_authority(
        "my-tenant-id",
        "my-client-id",
        "my-client-secret",
        mock.url(),
    );

    let client = AzureClient::builder()
        .arm_base_url(mock.url())
        .credential(credential)
        .subscription_id("00000000-0000-0000-0000-000000000000")
        .build()
        .unwrap();

    let provider = AzureProvider::new(client);

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

The mock server requires a TOML configuration file:

```toml
[server]
bind = "127.0.0.1"
port = 8080
delay_ms = 50  # Simulated response latency (ms)

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

## Implemented Endpoints

This project implements a **subset** of publicly documented ARM endpoint behavior. Coverage is not exhaustive and responses are simplified mock fixtures, not reproductions of real service output.

### Management Plane (ARM-compatible)

| Resource | Operations |
|---------|-----------|
| **Subscriptions** | List, Get |
| **Resource Groups** | Create/Update, Get, List, Delete, Check Existence, Update Tags |
| **Storage Accounts** | Create/Update, Get, List, List All, Delete, Update, Check Name Availability, List Keys, Regenerate Key, List Account SAS, List Service SAS, Revoke User Delegation Keys |
| **Virtual Machines** | Create/Update, Get, List, List All, List By Location, Delete, Start, Stop, Restart, Deallocate, Update (PATCH), Instance View, List Available Sizes, Generalize, Reapply, Simulate Eviction, Redeploy, Reimage |
| **Virtual Networks** | Create/Update, Get, List, Delete |
| **Subnets** | Create/Update, Get, List, Delete |
| **Network Security Groups** | Create/Update, Get, List, Delete |
| **Identity** | Get Current Principal, List Role Assignments |

### Data Plane (Blob-compatible)

| Resource | Operations |
|---------|-----------|
| **Blob Containers** | Create, Delete, List |
| **Blobs** | Put, Get, Delete, List, Head (properties) |

### Authentication

| Method | Description |
|--------|------------|
| `MockCredential` | Fixed token for local development |
| `ClientSecretCredential` | Service principal (client ID + secret). Configurable authority URL for mock or real endpoints. |
| `AzureCliCredential` | Shells out to `az account get-access-token` |
| `ChainedCredential` | Tries multiple credentials in order |

The mock server includes an OAuth2-compatible token endpoint so `ClientSecretCredential` works against the mock with zero code changes. Authentication is obviously simulated — any valid-looking Bearer token is accepted. This is purely for local testing; do not use mock authentication in any production or security-sensitive context.

## Workspace Structure

```
cloud-sdk/
  crates/
    cloud-sdk-core/              Shared traits, error types, models (zero provider deps)
    cloud-sdk-azure-client/      ARM-compatible REST client, auth, AzureProvider
    cloud-sdk-azure-mock/        Mock HTTP server (axum-based)
    cloud-sdk-test/              Test harness wiring mock + client together
    cloud-sdk-cli/               Standalone mock server binary (planned)
    cloud-sdk/                   Facade crate with feature flags
```

The client and mock crates are **independent** — the mock is a standalone HTTP server that does not depend on the client. This means:

- Production builds only pull in the client crate
- The mock server can be used with any HTTP client, not just this SDK
- Adding a new provider is a new pair of crates with zero impact on existing ones

## Mock Server Behavior

The mock server implements simplified versions of publicly documented ARM endpoint patterns:

- **URL patterns**: follows the documented ARM resource path structure
- **Request/response JSON**: uses camelCase field names and the `properties` envelope pattern documented in the ARM specification
- **Status codes**: follows documented conventions (200/201 for PUT, 200 for GET, etc.)
- **Error format**: uses the documented `CloudError` structure
- **Headers**: generates `x-ms-request-id` and `x-ms-correlation-id` response headers
- **Middleware**: validates `api-version` query parameter and `Authorization: Bearer` header on ARM routes
- **Pagination**: uses the `value`/`nextLink` envelope pattern on list responses
- **Response delay**: configurable latency (`delay_ms`) to simulate network conditions for async testing

Mock responses are **independently generated fixtures** based on the documented schema. They are not copies of real service responses and may differ in optional fields, default values, or edge-case behavior.

## Building

```sh
cargo build --workspace
cargo test --workspace        # 102 tests
cargo clippy --workspace
cargo fmt --all
```

## Contributing

Contributions are welcome. When adding new endpoints or types, please:

- Reference the [publicly documented REST API specification](https://learn.microsoft.com/en-us/rest/api/azure/) for request/response schemas
- Do not copy proprietary code, SDK internals, or large verbatim sections of documentation
- Ensure mock responses are independently authored fixtures
- Add tests for all new endpoints

## License

Apache-2.0

---

*This project is an independent community effort. Microsoft, Azure, and Azure Resource Manager are trademarks of Microsoft Corporation. Use of these names is for descriptive purposes only and does not imply endorsement.*
