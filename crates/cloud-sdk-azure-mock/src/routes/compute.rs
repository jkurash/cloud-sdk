use axum::{
    Json,
    extract::{Path, State},
};
use cloud_sdk_core::services::compute::{CreateVirtualMachineParams, PowerState};
use http::StatusCode;
use std::sync::Arc;

use super::error_response;
use crate::state::MockState;

/// PUT .../providers/Microsoft.Compute/virtualMachines/{vmName}
pub async fn create_or_update(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name, vm_name)): Path<(String, String, String)>,
    Json(params): Json<CreateVirtualMachineParams>,
) -> axum::response::Response {
    match state
        .create_virtual_machine(&subscription_id, &rg_name, &vm_name, &params)
        .await
    {
        Ok((vm, is_new)) => {
            let status = if is_new {
                StatusCode::CREATED
            } else {
                StatusCode::OK
            };
            let json = serde_json::to_string(&vm).unwrap();
            axum::response::Response::builder()
                .status(status)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

/// GET .../providers/Microsoft.Compute/virtualMachines/{vmName}
pub async fn get(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name, vm_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state
        .get_virtual_machine(&subscription_id, &rg_name, &vm_name)
        .await
    {
        Some(vm) => {
            let json = serde_json::to_string(&vm).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("The virtual machine '{vm_name}' was not found."),
        ),
    }
}

/// GET .../providers/Microsoft.Compute/virtualMachines
pub async fn list(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name)): Path<(String, String)>,
) -> axum::response::Response {
    match state
        .list_virtual_machines(&subscription_id, &rg_name)
        .await
    {
        Some(page) => {
            let json = serde_json::to_string(&page).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceGroupNotFound",
            &format!("Resource group '{rg_name}' could not be found."),
        ),
    }
}

/// DELETE .../providers/Microsoft.Compute/virtualMachines/{vmName}
pub async fn delete(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name, vm_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state
        .delete_virtual_machine(&subscription_id, &rg_name, &vm_name)
        .await
    {
        Ok(true) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        Ok(false) => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("The virtual machine '{vm_name}' was not found."),
        ),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

/// GET .../virtualMachines/{vmName}/instanceView
pub async fn instance_view(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name, vm_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state
        .get_vm_instance_view(&subscription_id, &rg_name, &vm_name)
        .await
    {
        Some(view) => {
            let json = serde_json::to_string(&view).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("The virtual machine '{vm_name}' was not found."),
        ),
    }
}

/// POST .../virtualMachines/{vmName}/start
pub async fn start(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name, vm_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    power_action(
        &state,
        &subscription_id,
        &rg_name,
        &vm_name,
        PowerState::Running,
    )
    .await
}

/// POST .../virtualMachines/{vmName}/powerOff
pub async fn power_off(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name, vm_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    power_action(
        &state,
        &subscription_id,
        &rg_name,
        &vm_name,
        PowerState::Stopped,
    )
    .await
}

/// POST .../virtualMachines/{vmName}/restart
pub async fn restart(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name, vm_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    power_action(
        &state,
        &subscription_id,
        &rg_name,
        &vm_name,
        PowerState::Running,
    )
    .await
}

/// POST .../virtualMachines/{vmName}/deallocate
pub async fn deallocate(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name, vm_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    power_action(
        &state,
        &subscription_id,
        &rg_name,
        &vm_name,
        PowerState::Deallocated,
    )
    .await
}

/// PATCH .../virtualMachines/{vmName}
pub async fn update(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name, vm_name)): Path<(String, String, String)>,
    axum::Json(patch): axum::Json<serde_json::Value>,
) -> axum::response::Response {
    match state
        .update_virtual_machine(&subscription_id, &rg_name, &vm_name, patch)
        .await
    {
        Ok(vm) => {
            let json = serde_json::to_string(&vm).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

/// GET /subscriptions/{sub}/providers/Microsoft.Compute/virtualMachines
pub async fn list_all(
    State(state): State<Arc<MockState>>,
    Path(subscription_id): Path<String>,
) -> axum::response::Response {
    match state.list_all_virtual_machines(&subscription_id).await {
        Some(page) => {
            let json = serde_json::to_string(&page).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "SubscriptionNotFound",
            &format!("Subscription '{subscription_id}' not found."),
        ),
    }
}

/// GET /subscriptions/{sub}/providers/Microsoft.Compute/locations/{location}/virtualMachines
pub async fn list_by_location(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, location)): Path<(String, String)>,
) -> axum::response::Response {
    match state
        .list_virtual_machines_by_location(&subscription_id, &location)
        .await
    {
        Some(page) => {
            let json = serde_json::to_string(&page).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "SubscriptionNotFound",
            &format!("Subscription '{subscription_id}' not found."),
        ),
    }
}

/// GET .../virtualMachines/{vmName}/vmSizes
pub async fn list_available_sizes(
    State(_state): State<Arc<MockState>>,
    Path((_subscription_id, _rg_name, _vm_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    // Return a static list of common Azure VM sizes
    let sizes = serde_json::json!({
        "value": [
            { "name": "Standard_A1_v2", "numberOfCores": 1, "osDiskSizeInMB": 1047552, "resourceDiskSizeInMB": 10240, "memoryInMB": 2048, "maxDataDiskCount": 2 },
            { "name": "Standard_A2_v2", "numberOfCores": 2, "osDiskSizeInMB": 1047552, "resourceDiskSizeInMB": 20480, "memoryInMB": 4096, "maxDataDiskCount": 4 },
            { "name": "Standard_B1s", "numberOfCores": 1, "osDiskSizeInMB": 1047552, "resourceDiskSizeInMB": 4096, "memoryInMB": 1024, "maxDataDiskCount": 2 },
            { "name": "Standard_B2s", "numberOfCores": 2, "osDiskSizeInMB": 1047552, "resourceDiskSizeInMB": 8192, "memoryInMB": 4096, "maxDataDiskCount": 4 },
            { "name": "Standard_D2s_v3", "numberOfCores": 2, "osDiskSizeInMB": 1047552, "resourceDiskSizeInMB": 16384, "memoryInMB": 8192, "maxDataDiskCount": 4 },
            { "name": "Standard_D4s_v3", "numberOfCores": 4, "osDiskSizeInMB": 1047552, "resourceDiskSizeInMB": 32768, "memoryInMB": 16384, "maxDataDiskCount": 8 },
            { "name": "Standard_D8s_v3", "numberOfCores": 8, "osDiskSizeInMB": 1047552, "resourceDiskSizeInMB": 65536, "memoryInMB": 32768, "maxDataDiskCount": 16 },
            { "name": "Standard_DS1_v2", "numberOfCores": 1, "osDiskSizeInMB": 1047552, "resourceDiskSizeInMB": 7168, "memoryInMB": 3584, "maxDataDiskCount": 4 },
            { "name": "Standard_DS2_v2", "numberOfCores": 2, "osDiskSizeInMB": 1047552, "resourceDiskSizeInMB": 14336, "memoryInMB": 7168, "maxDataDiskCount": 8 },
            { "name": "Standard_DS3_v2", "numberOfCores": 4, "osDiskSizeInMB": 1047552, "resourceDiskSizeInMB": 28672, "memoryInMB": 14336, "maxDataDiskCount": 16 },
            { "name": "Standard_E2s_v3", "numberOfCores": 2, "osDiskSizeInMB": 1047552, "resourceDiskSizeInMB": 32768, "memoryInMB": 16384, "maxDataDiskCount": 4 },
            { "name": "Standard_F2s_v2", "numberOfCores": 2, "osDiskSizeInMB": 1047552, "resourceDiskSizeInMB": 16384, "memoryInMB": 4096, "maxDataDiskCount": 4 }
        ]
    });
    let json = serde_json::to_string(&sizes).unwrap();
    axum::response::Response::builder()
        .status(StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(axum::body::Body::from(json))
        .unwrap()
}

/// POST .../virtualMachines/{vmName}/generalize
pub async fn generalize(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name, vm_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state
        .generalize_virtual_machine(&subscription_id, &rg_name, &vm_name)
        .await
    {
        Ok(()) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

/// POST .../virtualMachines/{vmName}/reapply
pub async fn reapply(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name, vm_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    // Verify VM exists, then no-op
    match state
        .get_virtual_machine(&subscription_id, &rg_name, &vm_name)
        .await
    {
        Some(_) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Virtual machine '{vm_name}' not found."),
        ),
    }
}

/// POST .../virtualMachines/{vmName}/simulateEviction
pub async fn simulate_eviction(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name, vm_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state
        .simulate_eviction(&subscription_id, &rg_name, &vm_name)
        .await
    {
        Ok(()) => axum::response::Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(axum::body::Body::empty())
            .unwrap(),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

/// POST .../virtualMachines/{vmName}/redeploy
pub async fn redeploy(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name, vm_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state
        .get_virtual_machine(&subscription_id, &rg_name, &vm_name)
        .await
    {
        Some(_) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Virtual machine '{vm_name}' not found."),
        ),
    }
}

/// POST .../virtualMachines/{vmName}/reimage
pub async fn reimage(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name, vm_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state
        .get_virtual_machine(&subscription_id, &rg_name, &vm_name)
        .await
    {
        Some(_) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Virtual machine '{vm_name}' not found."),
        ),
    }
}

async fn power_action(
    state: &MockState,
    subscription_id: &str,
    rg_name: &str,
    vm_name: &str,
    target_state: PowerState,
) -> axum::response::Response {
    match state
        .set_vm_power_state(subscription_id, rg_name, vm_name, target_state)
        .await
    {
        Ok(()) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}
