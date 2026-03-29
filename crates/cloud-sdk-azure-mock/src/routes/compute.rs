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
