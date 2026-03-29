# Cloud-Agnostic SDK — Implementation Plan

## Context

Build a cloud-agnostic Rust SDK where client code works identically against real Azure and a local mock server — the only difference is a base URL swap (controlled via feature gates). The mock server replicates Azure's REST API exactly, per https://learn.microsoft.com/en-us/rest/api/azure/.

## Key Constraint: Azure API Fidelity

The mock server must implement Azure's REST API **exactly** — same URL patterns, same request/response JSON schemas, same headers, same status codes. Client code must work against real Azure with zero code changes.

Azure has two API planes that the mock must replicate:
1. **Management plane (ARM)**: `management.azure.com` — resource lifecycle (create/delete VMs, storage accounts, resource groups, etc.)
2. **Data plane**: per-service endpoints like `{account}.blob.core.windows.net` — actual resource operations (read/write blobs, etc.)

## API Version Isolation Strategy

Each Azure API version is a **frozen, self-contained module**. Once released, a version's code is never modified — new versions add new modules alongside existing ones.

### Module Structure (per service, per version)

**Azure client — versioned operations and models:**
```
crates/cloud-sdk-azure/src/services/
  resource_manager/
    mod.rs                          (re-exports latest version as default)
    v2021_04_01/
      mod.rs
      models.rs                     (request/response types for this version)
      operations.rs                 (client methods)
    v2024_01_01/                    (future version — added alongside, not replacing)
      mod.rs
      models.rs
      operations.rs
  compute/
    mod.rs
    v2024_07_01/
      mod.rs
      models.rs
      operations.rs
  storage/
    mod.rs
    v2023_05_01/
      mod.rs
      models.rs
      operations.rs
```

**Mock server — versioned handlers:**
```
crates/cloud-sdk-mock/src/routes/
  resource_groups/
    mod.rs                          (router dispatches based on api-version query param)
    v2021_04_01.rs                  (handlers for this version)
  compute/
    mod.rs
    v2024_07_01.rs
```

### Feature-Gated API Versions

Each API version is behind a **Cargo feature flag** in both `cloud-sdk-azure-client` and `cloud-sdk-azure-mock`. This allows selecting which versions to include at build time — binaries only ship the versions they need.

```toml
# crates/cloud-sdk-azure-mock/Cargo.toml
[features]
default = ["resource_manager_v2021_04_01", "compute_v2024_07_01", "storage_v2023_05_01", "networking_v2024_01_01"]
resource_manager_v2021_04_01 = []
compute_v2024_07_01 = []
storage_v2023_05_01 = []
networking_v2024_01_01 = []
```

Version modules and route registrations are conditionally compiled:
```rust
#[cfg(feature = "resource_manager_v2021_04_01")]
pub mod v2021_04_01;

// Router setup:
#[cfg(feature = "resource_manager_v2021_04_01")]
let router = router.merge(resource_groups::v2021_04_01::routes(state.clone()));
```

### How it works
1. The mock router reads the `api-version` query param and dispatches to the correct version's handler (only enabled versions are registered)
2. If an unknown/disabled version is requested, return a `400 InvalidApiVersionParameter` error (matching Azure behavior)
3. Each version module contains its own serde models — if Azure changes a field between versions, both versions coexist
4. The `mod.rs` at each service level re-exports the latest *enabled* version as the default, so `use cloud_sdk_azure::services::compute::*` gives you the latest
5. Users who need a specific version import explicitly: `use cloud_sdk_azure::services::compute::v2024_07_01::*`

### Version Registry
```rust
// Each service registers its supported versions
pub struct ApiVersionRegistry {
    versions: HashMap<(&'static str, &'static str), Box<dyn VersionedHandler>>,
    // key: (resource_provider, api_version) e.g. ("Microsoft.Compute/virtualMachines", "2024-07-01")
}
```

## Mock Server Configuration (TOML)

Each mock server is configured via a **required** TOML file (e.g., `azure-mock.toml`). The server will not start without one.

```toml
[server]
bind = "127.0.0.1"
port = 8080

[subscriptions.default]
id = "00000000-0000-0000-0000-000000000000"
display_name = "Mock Subscription"
tenant_id = "00000000-0000-0000-0000-000000000001"
state = "Enabled"

# Seed data — pre-populated resources
[[subscriptions.default.resource_groups]]
name = "dev-rg"
location = "eastus"

[[subscriptions.default.resource_groups]]
name = "staging-rg"
location = "westus2"
tags = { env = "staging" }
```

- **Required to start**: `AzureMockServer::from_config(config)` or `AzureMockServer::from_file("azure-mock.toml")`
- **Per-provider**: `azure-mock.toml`, `aws-mock.toml`, etc.
- **Seed data**: optional pre-populated resource groups (and later VMs, storage accounts, etc.)
- **TestHarness**: programmatically constructs a config with defaults — no file needed for tests

## Workspace Structure

Per-provider crate pairs (`{provider}-client` + `{provider}-mock`), extending cleanly when new providers are added:

```
cloud-sdk/
  Cargo.toml                            (workspace manifest, resolver = "3")
  crates/
    cloud-sdk-core/                     (shared traits, error types, common models — zero provider deps)
    cloud-sdk-azure-client/             (AzureClient, AzureProvider, auth, pipeline, Azure serde types)
    cloud-sdk-azure-mock/               (mock HTTP server, state store, axum routes — does NOT depend on azure-client)
    cloud-sdk-test/                     (test harness: starts mock server, points AzureClient at it)
    cloud-sdk-cli/                      (standalone mock server binary)
    cloud-sdk/                          (facade: re-exports via feature flags)
    # Future:
    # cloud-sdk-aws-client/
    # cloud-sdk-aws-mock/
    # cloud-sdk-gcp-client/
    # cloud-sdk-gcp-mock/
```

**Key dependency rule:** `azure-mock` does NOT depend on `azure-client`. They're both HTTP-level (server and client) that independently implement Azure's JSON schemas. `cloud-sdk-test` depends on both and wires them together for integration tests. Production builds only need `azure-client`.

## Core Trait Design

### CloudProvider (Abstract Factory)
```rust
pub trait CloudProvider: Send + Sync {
    type ComputeService: ComputeService;
    type StorageService: StorageService;
    type NetworkingService: NetworkingService;
    type IdentityService: IdentityService;
    type ResourceManagerService: ResourceManagerService;
    fn compute(&self) -> &Self::ComputeService;
    fn storage(&self) -> &Self::StorageService;
    fn networking(&self) -> &Self::NetworkingService;
    fn identity(&self) -> &Self::IdentityService;
    fn resource_manager(&self) -> &Self::ResourceManagerService;
    fn name(&self) -> &str;
}
```
Uses **associated types** for static dispatch + native async fn in traits (edition 2024).

### Service Traits
- **ResourceManagerService**: subscriptions list/get, resource group CRUD
- **ComputeService**: VM create/get/list/delete/start/stop/restart/deallocate
- **StorageService**: storage account CRUD (ARM) + blob container CRUD + blob put/get/delete/list (data plane)
- **NetworkingService**: VNets, subnets, NSGs
- **IdentityService**: current principal, role assignments

### Error Type
Single `CloudSdkError` enum. The `ProviderError` variant carries the Azure `CloudError` format exactly:
```rust
#[derive(Debug, thiserror::Error)]
pub enum CloudSdkError {
    #[error("resource not found: {resource_type} '{name}'")] NotFound { resource_type: String, name: String },
    #[error("resource already exists: {resource_type} '{name}'")] AlreadyExists { resource_type: String, name: String },
    #[error("authentication failed: {message}")] AuthenticationError { message: String },
    #[error("authorization denied: {message}")] AuthorizationError { message: String },
    #[error("invalid input: {message}")] ValidationError { message: String },
    #[error("rate limited, retry after {retry_after_secs}s")] RateLimited { retry_after_secs: u64 },
    #[error("provider error ({provider}): {status} - {message}")] ProviderError { provider: String, status: u16, code: String, message: String },
    #[error("HTTP transport error: {0}")] HttpError(#[from] reqwest::Error),
    #[error("serialization error: {0}")] SerializationError(#[from] serde_json::Error),
    #[error("internal error: {0}")] Internal(String),
}
```

### Auth Abstraction
```rust
pub trait Credential: Send + Sync {
    async fn get_token(&self, scopes: &[&str]) -> Result<AccessToken, CloudSdkError>;
}
```

## Azure REST API — Exact Structure to Replicate

### Management Plane (ARM) — `management.azure.com`

All requests require:
- `Authorization: Bearer {token}` header
- `api-version` query parameter
- `Content-Type: application/json`

Response headers include: `x-ms-request-id`, `x-ms-correlation-id`

#### Subscriptions (api-version: 2022-12-01)
```
GET  /subscriptions                      → 200 SubscriptionListResult
GET  /subscriptions/{subscriptionId}     → 200 Subscription
```

**Subscription response shape:**
```json
{
  "id": "/subscriptions/{id}",
  "subscriptionId": "{guid}",
  "displayName": "...",
  "state": "Enabled|Warned|PastDue|Disabled|Deleted",
  "tenantId": "{guid}",
  "tags": {},
  "subscriptionPolicies": { "locationPlacementId": "...", "quotaId": "...", "spendingLimit": "On|Off" }
}
```

#### Resource Groups (api-version: 2021-04-01)
```
PUT    /subscriptions/{sub}/resourcegroups/{rg}   → 200/201 ResourceGroup
GET    /subscriptions/{sub}/resourcegroups/{rg}   → 200 ResourceGroup
GET    /subscriptions/{sub}/resourcegroups         → 200 ResourceGroupListResult
DELETE /subscriptions/{sub}/resourcegroups/{rg}   → 200/202
PATCH  /subscriptions/{sub}/resourcegroups/{rg}   → 200 ResourceGroup
HEAD   /subscriptions/{sub}/resourcegroups/{rg}   → 204/404
```

**ResourceGroup shape:**
```json
{
  "id": "/subscriptions/{sub}/resourceGroups/{rg}",
  "name": "{rg}",
  "type": "Microsoft.Resources/resourceGroups",
  "location": "eastus",
  "tags": {},
  "properties": { "provisioningState": "Succeeded" }
}
```

**PUT request body:** `{ "location": "eastus", "tags": {} }`

**Error response (all endpoints):**
```json
{
  "error": {
    "code": "ResourceGroupNotFound",
    "message": "Resource group 'foo' could not be found.",
    "details": [],
    "additionalInfo": []
  }
}
```

#### Virtual Machines (api-version: 2024-07-01)
```
PUT    .../providers/Microsoft.Compute/virtualMachines/{vm}           → 200/201
GET    .../providers/Microsoft.Compute/virtualMachines/{vm}           → 200
GET    .../providers/Microsoft.Compute/virtualMachines                → 200 (list)
DELETE .../providers/Microsoft.Compute/virtualMachines/{vm}           → 200/202/204
POST   .../providers/Microsoft.Compute/virtualMachines/{vm}/start    → 200/202
POST   .../providers/Microsoft.Compute/virtualMachines/{vm}/powerOff → 200/202
POST   .../providers/Microsoft.Compute/virtualMachines/{vm}/restart  → 200/202
POST   .../providers/Microsoft.Compute/virtualMachines/{vm}/deallocate → 200/202
GET    .../providers/Microsoft.Compute/virtualMachines/{vm}/instanceView → 200
```

**VM create request body (key fields):**
```json
{
  "location": "westus",
  "properties": {
    "hardwareProfile": { "vmSize": "Standard_D2s_v3" },
    "storageProfile": {
      "imageReference": { "publisher": "Canonical", "offer": "UbuntuServer", "sku": "18.04-LTS", "version": "latest" },
      "osDisk": { "name": "myDisk", "createOption": "FromImage", "caching": "ReadWrite", "managedDisk": { "storageAccountType": "Premium_LRS" } }
    },
    "osProfile": { "computerName": "myVM", "adminUsername": "azureuser", "adminPassword": "...", "linuxConfiguration": { "disablePasswordAuthentication": false } },
    "networkProfile": { "networkInterfaces": [{ "id": "/subscriptions/.../networkInterfaces/myNIC", "properties": { "primary": true } }] }
  },
  "tags": {}
}
```

#### Storage Accounts (ARM — api-version: 2023-05-01)
```
PUT    .../providers/Microsoft.Storage/storageAccounts/{name}   → 200/202
GET    .../providers/Microsoft.Storage/storageAccounts/{name}   → 200
GET    .../providers/Microsoft.Storage/storageAccounts           → 200 (list)
DELETE .../providers/Microsoft.Storage/storageAccounts/{name}   → 200/202/204
```

#### Networking (api-version: 2024-01-01)
```
PUT/GET/DELETE/LIST .../providers/Microsoft.Network/virtualNetworks/{vnet}
PUT/GET/DELETE/LIST .../providers/Microsoft.Network/virtualNetworks/{vnet}/subnets/{subnet}
PUT/GET/DELETE/LIST .../providers/Microsoft.Network/networkSecurityGroups/{nsg}
PUT/GET/DELETE/LIST .../providers/Microsoft.Network/networkInterfaces/{nic}
```

### Data Plane — Blob Storage

Separate endpoint: `http://{account}.blob.core.windows.net` (real) or `http://127.0.0.1:{port}/{account}` (mock, path-based routing).

Key operations:
```
GET    /{account}?comp=list                                → List Containers
PUT    /{account}/{container}?restype=container            → Create Container
DELETE /{account}/{container}?restype=container            → Delete Container
GET    /{account}/{container}?restype=container&comp=list  → List Blobs
PUT    /{account}/{container}/{blob}                       → Put Blob
GET    /{account}/{container}/{blob}                       → Get Blob
DELETE /{account}/{container}/{blob}                       → Delete Blob
HEAD   /{account}/{container}/{blob}                       → Get Blob Properties
```

Required headers: `x-ms-version` (e.g. `2023-11-03`), `x-ms-date`, `Authorization` (SharedKey or Bearer).

### Pagination Pattern (both planes)
List responses use `nextLink`/`value` envelope:
```json
{ "value": [...], "nextLink": "https://...?$skiptoken=..." }
```

## Azure Client Architecture

### Request Pipeline
```rust
pub trait HttpTransport: Send + Sync {
    async fn send(&self, request: &Request) -> Result<Response, CloudSdkError>;
}
```
`ReqwestTransport` — used for both real Azure AND mock (just different base URLs).

Pipeline chain: Auth → Retry → Logging → Send.

### AzureClient
```rust
pub struct AzureClient {
    pipeline: Pipeline,
    config: AzureConfig,
}
pub struct AzureConfig {
    pub subscription_id: String,
    pub arm_base_url: Url,          // https://management.azure.com or http://localhost:PORT
    pub storage_base_url: Option<Url>, // for blob data plane, None = derive from account name
    pub api_versions: HashMap<String, String>,
}
```

URL construction follows ARM patterns exactly:
```
{arm_base_url}/subscriptions/{sub}/resourceGroups/{rg}/providers/{provider}/{type}/{name}?api-version={ver}
```

### Auth Implementations
- `MockCredential` — always returns a fixed token
- `ClientSecretCredential` — service principal via `/oauth2/v2.0/token`
- `AzureCliCredential` — `az account get-access-token`
- `ChainedCredential` — tries multiple in order (DefaultAzureCredential equivalent)

## Mock Server Architecture

### Mock Server (cloud-sdk-azure-mock)
Standalone axum HTTP server. Does NOT depend on `cloud-sdk-azure-client`. Owns its own serde types for Azure JSON schemas.

```rust
// crates/cloud-sdk-azure-mock/src/server.rs
pub struct AzureMockServer {
    state: Arc<MockState>,
}
impl AzureMockServer {
    pub fn new() -> Self { ... }
    pub fn into_router(self) -> axum::Router { ... }
    /// Start on a random port, return the handle
    pub async fn start(self) -> Result<AzureMockServerHandle, std::io::Error> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let port = listener.local_addr()?.port();
        let router = self.into_router();
        let handle = tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
        Ok(AzureMockServerHandle { handle, port })
    }
}
pub struct AzureMockServerHandle {
    handle: JoinHandle<()>,
    pub port: u16,
}
impl AzureMockServerHandle {
    pub fn url(&self) -> String { format!("http://127.0.0.1:{}", self.port) }
}
```

### Test Harness (cloud-sdk-test) — wires mock + client together
```rust
// crates/cloud-sdk-test/src/harness.rs
pub struct TestHarness {
    mock_handle: AzureMockServerHandle,
    provider: AzureProvider,
}
impl TestHarness {
    pub async fn start() -> Result<Self, CloudSdkError> {
        let mock = AzureMockServer::new().start().await?;
        let client = AzureClient::builder()
            .arm_base_url(mock.url())
            .storage_base_url(mock.url())
            .credential(MockCredential)
            .subscription_id("00000000-0000-0000-0000-000000000000")
            .build()?;
        let provider = AzureProvider::new(client);
        Ok(Self { mock_handle: mock, provider })
    }
    pub fn provider(&self) -> &AzureProvider { &self.provider }
    pub fn mock_url(&self) -> String { self.mock_handle.url() }
}
```

### Mock Server: Two Routers in One
Single axum server handles both planes via path discrimination:
- ARM routes: `/subscriptions/...` (management plane)
- Blob data routes: `/{account}/{container}/{blob}` (data plane, path-based instead of subdomain-based)

### State Management
Hierarchical in-memory store mirroring Azure's resource model:
```
MockState (Arc<RwLock<StateInner>>)
  └─ subscriptions: HashMap<String, Subscription>
       ├─ metadata (id, displayName, state, tenantId)
       └─ resource_groups: HashMap<String, ResourceGroupState>
            ├─ metadata (id, name, location, tags, provisioningState)
            ├─ virtual_machines: HashMap<String, VirtualMachine>
            ├─ storage_accounts: HashMap<String, StorageAccountState>
            │    └─ containers: HashMap<String, ContainerState>
            │         └─ blobs: HashMap<String, BlobState { properties, data: Bytes }>
            ├─ virtual_networks: HashMap<String, VirtualNetworkState>
            │    └─ subnets: HashMap<String, Subnet>
            ├─ network_security_groups: HashMap<String, NetworkSecurityGroup>
            └─ network_interfaces: HashMap<String, NetworkInterface>
```

### Middleware (matching Azure behavior)
- Validate `api-version` query param present (400 if missing)
- Validate `Authorization: Bearer ...` header present (401 if missing)
- Generate `x-ms-request-id` UUID response header
- Generate `x-ms-correlation-id` response header
- Set `Content-Type: application/json` on responses

### Response Fidelity
**Replicated exactly:**
- HTTP status codes per Azure docs (200/201 for PUT, 200/202 for DELETE, 204 for HEAD existence check)
- Azure resource envelope: `{ "id", "name", "type", "location", "tags", "properties": {...} }`
- Azure error format: `{ "error": { "code", "message", "details", "additionalInfo" } }`
- List pagination: `{ "value": [...], "nextLink": "..." }`
- Resource IDs follow Azure format: `/subscriptions/{sub}/resourceGroups/{rg}/providers/{provider}/{type}/{name}`

**Not replicated (Phase 1):**
- Full RBAC evaluation (accept any Bearer token)
- Async provisioning (resources created synchronously, 200/201 not 202)
- Complex cross-field validation
- Exact provisioning delays

## Key Dependencies

| Purpose | Crate | Used In |
|---------|-------|---------|
| Async runtime | `tokio` (full) | All |
| Serialization | `serde` + `serde_json` | All |
| HTTP client | `reqwest` (rustls-tls, json) | azure |
| Mock server | `axum` 0.8 | mock |
| Middleware | `tower` + `tower-http` | mock, azure |
| Errors | `thiserror` 2 | core |
| UUIDs | `uuid` (v4, serde) | core, mock |
| Timestamps | `chrono` (serde) | core, mock |
| URLs | `url` | azure |
| Logging | `tracing` | All |
| Binary data | `bytes` | core |

## Implementation Order

### Phase 1: Foundation (build first, prove the architecture)
1. **Workspace skeleton** — Convert root `Cargo.toml` to workspace, create all crate dirs, `cargo check` passes
2. **cloud-sdk-core** — All traits, error types, model structs matching Azure response schemas exactly
3. **cloud-sdk-azure-mock: state store** — `MockState` with subscription + resource group state, unit tests
4. **cloud-sdk-azure-mock: axum server** — ARM router with subscription list/get + resource group CRUD, middleware (api-version, auth, request-id), `AzureMockServer::start()`
5. **cloud-sdk-cli** — standalone binary that starts `AzureMockServer` on a configurable port
6. **cloud-sdk-azure-client** — `AzureClient`, `Pipeline`, `ReqwestTransport`, `AzureConfig` with ARM URL builder, retry policy, auth policy, `AzureProvider` implementing `ResourceManagerService`
7. **cloud-sdk-test** — `TestHarness` that starts mock + points AzureClient at it. Integration tests: round-trip resource group CRUD through full HTTP stack

### Phase 2: Storage (highest-value service)
8. **cloud-sdk-azure-mock**: storage account ARM routes + mock state + blob data plane routes (container CRUD, blob put/get/delete/list)
9. **cloud-sdk-azure-client**: `StorageService` trait implementation for `AzureProvider`
10. **cloud-sdk-test**: integration tests — create storage account → create container → put blob → get blob → delete

### Phase 3: Compute
11. **cloud-sdk-azure-mock**: VM CRUD + power operations (start/stop/restart/deallocate) + instance view. State machine in mock.
12. **cloud-sdk-azure-client**: `ComputeService` trait implementation
13. **cloud-sdk-test**: integration tests — full VM lifecycle

### Phase 4: Networking
14. **cloud-sdk-azure-mock**: VNets, subnets, NSGs, NICs CRUD + cross-resource references
15. **cloud-sdk-azure-client**: `NetworkingService` trait implementation
16. **cloud-sdk-test**: integration tests

### Phase 5: Identity & Auth
17. **cloud-sdk-azure-client**: real Azure auth (ClientSecretCredential, AzureCliCredential, ChainedCredential)
18. Identity service (current principal, role assignments — read-only mock)

### Phase 6: VM Response Fidelity
Align `VirtualMachine` types with the full Azure REST API response schema (`GET /virtualMachines/{vm}`, api-version 2025-04-01). 16 missing fields identified via smoke test against the Azure docs.

**Top-level `VirtualMachine`** (3 fields):
- `etag`
- `resources` (VM extensions array)
- `managedBy`

**`VirtualMachineProperties`** (7 fields):
- `availabilitySet` (`{ id }`)
- `proximityPlacementGroup` (`{ id }`)
- `applicationProfile` (gallery applications)
- `userData` (base64)
- `diagnosticsProfile` (boot diagnostics)
- `extensionsTimeBudget`
- `timeCreated` (datetime)

**`StorageProfile`** (1 field):
- `dataDisks` (array of data disks with lun, name, createOption, caching, managedDisk, diskSizeGB)

**`OsDisk`** (2 fields):
- `osType` (Windows/Linux)
- `diskSizeGB`

**`OsProfile`** (3 fields):
- `adminPassword` (write-only, never returned in GET)
- `windowsConfiguration` (provisionVMAgent, enableAutomaticUpdates, patchSettings)
- `secrets` (array)

All fields should be `Option<...>` with `skip_serializing_if` so they don't break existing code. The mock passes through whatever the client sends and adds server-generated fields (`vmId`, `provisioningState`, `timeCreated`).

### Phase 7: Polish
20. `cloud-sdk` facade crate with feature flags
21. `cloud-sdk-cli` standalone binary
22. Examples directory

## Verification (per crate, after each implementation step)

Run in this order after completing a crate's implementation:

1. **Tests** — `cargo test -p <crate>`
2. **Coverage** — `cargo tarpaulin -p <crate> --out stdout` — review coverage, add tests if below acceptable threshold
3. **Format** — `cargo fmt --all`
4. **Lint** — `cargo clippy --workspace` — fix all warnings

### Additional verification
- Round-trip test: `TestHarness::start()` → call SDK operations → verify returned objects match Azure JSON schemas
- Mock fidelity tests: raw HTTP requests via `reqwest` to mock server → assert response JSON matches Azure REST API docs exactly
- Standalone binary test: start `cloud-sdk-cli`, curl endpoints, verify responses
