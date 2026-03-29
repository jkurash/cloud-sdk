//! Smoke test: verify the mock VM GET response matches Azure's documented response shape.
//! Reference: https://learn.microsoft.com/en-us/rest/api/compute/virtual-machines/get

use cloud_sdk_azure_mock::{AzureMockConfig, AzureMockServer};
use serde_json::Value;

const BEARER: &str = "Bearer mock-token";
const SUB_ID: &str = "00000000-0000-0000-0000-000000000000";

fn test_config() -> AzureMockConfig {
    AzureMockConfig::from_toml(
        r#"
[server]
port = 0

[subscriptions.default]
id = "00000000-0000-0000-0000-000000000000"
display_name = "Mock Subscription"
tenant_id = "00000000-0000-0000-0000-000000000001"
state = "Enabled"

[[subscriptions.default.resource_groups]]
name = "myResourceGroup"
location = "westus"
"#,
    )
    .unwrap()
}

async fn start_server() -> (reqwest::Client, String) {
    let handle = AzureMockServer::from_config(test_config())
        .start_on_random_port()
        .await
        .unwrap();
    let base = handle.url();
    let client = reqwest::Client::new();
    std::mem::forget(handle);
    (client, base)
}

fn auth() -> (&'static str, &'static str) {
    ("Authorization", BEARER)
}

/// Create a VM with a realistic request body matching Azure docs, then GET it
/// and verify all response fields match the Azure API shape.
#[tokio::test]
async fn vm_get_response_matches_azure_shape() {
    let (client, base) = start_server().await;

    // Create a VM with a full request body
    let create_body = serde_json::json!({
        "location": "westus",
        "tags": { "myTag1": "tagValue1" },
        "properties": {
            "hardwareProfile": {
                "vmSize": "Standard_DS3_v2"
            },
            "storageProfile": {
                "imageReference": {
                    "publisher": "MicrosoftWindowsServer",
                    "offer": "WindowsServer",
                    "sku": "2016-Datacenter",
                    "version": "latest"
                },
                "osDisk": {
                    "name": "myOsDisk",
                    "createOption": "FromImage",
                    "caching": "ReadWrite",
                    "managedDisk": {
                        "storageAccountType": "Premium_LRS"
                    }
                },
                "dataDisks": [
                    {
                        "lun": 0,
                        "name": "myDataDisk0",
                        "createOption": "Empty",
                        "caching": "ReadWrite",
                        "managedDisk": {
                            "storageAccountType": "Premium_LRS"
                        },
                        "diskSizeGB": 30
                    },
                    {
                        "lun": 1,
                        "name": "myDataDisk1",
                        "createOption": "Attach",
                        "caching": "ReadWrite",
                        "managedDisk": {
                            "storageAccountType": "Premium_LRS"
                        },
                        "diskSizeGB": 100
                    }
                ]
            },
            "osProfile": {
                "computerName": "myVM",
                "adminUsername": "admin",
                "adminPassword": "SuperSecret123!",
                "windowsConfiguration": {
                    "provisionVMAgent": true,
                    "enableAutomaticUpdates": false
                }
            },
            "networkProfile": {
                "networkInterfaces": [
                    {
                        "id": "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myResourceGroup/providers/Microsoft.Network/networkInterfaces/myNIC"
                    }
                ]
            },
            "diagnosticsProfile": {
                "bootDiagnostics": {
                    "enabled": true,
                    "storageUri": "http://mystorageaccount.blob.core.windows.net"
                }
            },
            "extensionsTimeBudget": "PT50M"
        }
    });

    let resp = client
        .put(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/myResourceGroup/providers/Microsoft.Compute/virtualMachines/myVM?api-version=2024-07-01"
        ))
        .header(auth().0, auth().1)
        .json(&create_body)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 201);

    // GET the VM
    let resp = client
        .get(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/myResourceGroup/providers/Microsoft.Compute/virtualMachines/myVM?api-version=2024-07-01"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    let vm: Value = resp.json().await.unwrap();

    // ── Top-level fields ───────────────────────────────────────────────
    assert_eq!(vm["name"], "myVM");
    assert_eq!(
        vm["id"],
        format!(
            "/subscriptions/{SUB_ID}/resourceGroups/myResourceGroup/providers/Microsoft.Compute/virtualMachines/myVM"
        )
    );
    assert_eq!(vm["type"], "Microsoft.Compute/virtualMachines");
    assert_eq!(vm["location"], "westus");
    assert_eq!(vm["tags"]["myTag1"], "tagValue1");
    assert!(vm["etag"].is_string(), "etag should be present");
    assert!(vm["resources"].is_array(), "resources should be an array");

    // ── Server-generated properties ────────────────────────────────────
    let props = &vm["properties"];
    assert!(props["vmId"].is_string(), "vmId should be a UUID string");
    assert_eq!(props["provisioningState"], "Succeeded");
    assert!(
        props["timeCreated"].is_string(),
        "timeCreated should be set"
    );

    // ── hardwareProfile ────────────────────────────────────────────────
    assert_eq!(props["hardwareProfile"]["vmSize"], "Standard_DS3_v2");

    // ── storageProfile ─────────────────────────────────────────────────
    let sp = &props["storageProfile"];

    // imageReference
    assert_eq!(sp["imageReference"]["publisher"], "MicrosoftWindowsServer");
    assert_eq!(sp["imageReference"]["offer"], "WindowsServer");
    assert_eq!(sp["imageReference"]["sku"], "2016-Datacenter");
    assert_eq!(sp["imageReference"]["version"], "latest");

    // osDisk — server-enriched fields
    let os_disk = &sp["osDisk"];
    assert_eq!(os_disk["name"], "myOsDisk");
    assert_eq!(os_disk["createOption"], "FromImage");
    assert_eq!(os_disk["caching"], "ReadWrite");
    assert_eq!(
        os_disk["osType"], "Windows",
        "osType should be inferred from windowsConfiguration"
    );
    assert_eq!(os_disk["diskSizeGB"], 30, "diskSizeGB should default to 30");
    assert_eq!(os_disk["managedDisk"]["storageAccountType"], "Premium_LRS");
    assert!(
        os_disk["managedDisk"]["id"]
            .as_str()
            .unwrap()
            .contains("/disks/myOsDisk"),
        "managedDisk.id should be auto-generated: got {}",
        os_disk["managedDisk"]["id"]
    );

    // dataDisks — server-enriched
    let data_disks = sp["dataDisks"].as_array().unwrap();
    assert_eq!(data_disks.len(), 2);
    assert_eq!(data_disks[0]["lun"], 0);
    assert_eq!(data_disks[0]["name"], "myDataDisk0");
    assert_eq!(data_disks[0]["diskSizeGB"], 30);
    assert!(
        data_disks[0]["managedDisk"]["id"]
            .as_str()
            .unwrap()
            .contains("/disks/myDataDisk0"),
        "dataDisks[0].managedDisk.id should be auto-generated"
    );
    assert_eq!(data_disks[1]["lun"], 1);
    assert_eq!(data_disks[1]["diskSizeGB"], 100);
    assert!(
        data_disks[1]["managedDisk"]["id"]
            .as_str()
            .unwrap()
            .contains("/disks/myDataDisk1"),
        "dataDisks[1].managedDisk.id should be auto-generated"
    );

    // ── osProfile (adminPassword stripped) ─────────────────────────────
    let os_profile = &props["osProfile"];
    assert_eq!(os_profile["computerName"], "myVM");
    assert_eq!(os_profile["adminUsername"], "admin");
    assert!(
        os_profile.get("adminPassword").is_none() || os_profile["adminPassword"].is_null(),
        "adminPassword should be stripped from GET response"
    );
    assert_eq!(os_profile["windowsConfiguration"]["provisionVMAgent"], true);
    assert_eq!(
        os_profile["windowsConfiguration"]["enableAutomaticUpdates"],
        false
    );
    assert!(
        os_profile["secrets"].is_array(),
        "secrets should default to empty array"
    );

    // ── networkProfile ─────────────────────────────────────────────────
    let nic = &props["networkProfile"]["networkInterfaces"][0];
    assert!(nic["id"].as_str().unwrap().contains("myNIC"));

    // ── diagnosticsProfile ─────────────────────────────────────────────
    assert_eq!(
        props["diagnosticsProfile"]["bootDiagnostics"]["enabled"],
        true
    );
    assert_eq!(
        props["diagnosticsProfile"]["bootDiagnostics"]["storageUri"],
        "http://mystorageaccount.blob.core.windows.net"
    );

    // ── extensionsTimeBudget ───────────────────────────────────────────
    assert_eq!(props["extensionsTimeBudget"], "PT50M");

    // ── Azure response headers ─────────────────────────────────────────
    // (checked on the GET response above, but let's verify on a fresh call)
    let resp = client
        .get(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/myResourceGroup/providers/Microsoft.Compute/virtualMachines/myVM?api-version=2024-07-01"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();
    assert!(resp.headers().get("x-ms-request-id").is_some());
    assert!(resp.headers().get("x-ms-correlation-id").is_some());
}

/// Verify instance view returns power state in Azure-compatible format.
#[tokio::test]
async fn vm_instance_view_returns_power_state() {
    let (client, base) = start_server().await;

    // Create VM
    client
        .put(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/myResourceGroup/providers/Microsoft.Compute/virtualMachines/viewVM?api-version=2024-07-01"
        ))
        .header(auth().0, auth().1)
        .json(&serde_json::json!({
            "location": "eastus",
            "properties": {
                "hardwareProfile": { "vmSize": "Standard_B1s" },
                "storageProfile": {
                    "osDisk": { "createOption": "FromImage" }
                },
                "osProfile": {
                    "computerName": "viewVM",
                    "adminUsername": "azureuser",
                    "linuxConfiguration": { "disablePasswordAuthentication": true }
                },
                "networkProfile": { "networkInterfaces": [] }
            }
        }))
        .send()
        .await
        .unwrap();

    // Get instance view
    let resp = client
        .get(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/myResourceGroup/providers/Microsoft.Compute/virtualMachines/viewVM/instanceView?api-version=2024-07-01"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    let view: Value = resp.json().await.unwrap();
    assert_eq!(view["computerName"], "viewVM");
    assert!(view["statuses"].is_array());

    let statuses = view["statuses"].as_array().unwrap();
    assert!(statuses.len() >= 2);

    // First status: provisioning
    assert_eq!(statuses[0]["code"], "ProvisioningState/succeeded");

    // Second status: power state (should be running after create)
    assert_eq!(statuses[1]["code"], "PowerState/running");
    assert_eq!(statuses[1]["displayStatus"], "VM running");

    // Stop the VM and check again
    client
        .post(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/myResourceGroup/providers/Microsoft.Compute/virtualMachines/viewVM/powerOff?api-version=2024-07-01"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();

    let resp = client
        .get(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/myResourceGroup/providers/Microsoft.Compute/virtualMachines/viewVM/instanceView?api-version=2024-07-01"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();
    let view: Value = resp.json().await.unwrap();
    let statuses = view["statuses"].as_array().unwrap();
    assert_eq!(statuses[1]["code"], "PowerState/stopped");
    assert_eq!(statuses[1]["displayStatus"], "VM stopped");
}
