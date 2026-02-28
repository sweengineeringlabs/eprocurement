//! Requisitions service - API calls

use super::store::{RequisitionsStore, load_mock_data};
use super::types::{Requisition, RequisitionStatus};

/// Load requisitions data
pub async fn load_requisitions(store: &RequisitionsStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    // For now, load mock data
    load_mock_data(store);

    store.loading.set(false);
}

/// Create a new requisition
pub async fn create_requisition(store: &RequisitionsStore, mut requisition: Requisition) -> Result<Requisition, String> {
    store.loading.set(true);
    store.error.set(None);

    // Generate ID
    let count = store.requisitions.get().len() + 1;
    requisition.id = format!("REQ-2025-{:04}", 847 + count);
    requisition.status = RequisitionStatus::Draft;
    requisition.created_at = chrono_now();
    requisition.updated_at = chrono_now();

    // Calculate total
    requisition.calculate_total();

    // In production, this would POST to the API
    // For now, add to local store
    let mut requisitions = store.requisitions.get();
    requisitions.insert(0, requisition.clone());
    store.requisitions.set(requisitions);

    // Update pagination
    let mut pagination = store.pagination.get();
    pagination.update_totals(store.requisitions.get().len() as u32);
    store.pagination.set(pagination);

    store.loading.set(false);

    Ok(requisition)
}

/// Update an existing requisition
pub async fn update_requisition(store: &RequisitionsStore, requisition: Requisition) -> Result<Requisition, String> {
    store.loading.set(true);
    store.error.set(None);

    let mut updated = requisition.clone();
    updated.updated_at = chrono_now();
    updated.calculate_total();

    // In production, this would PUT to the API
    // For now, update in local store
    let mut requisitions = store.requisitions.get();
    if let Some(pos) = requisitions.iter().position(|r| r.id == updated.id) {
        requisitions[pos] = updated.clone();
        store.requisitions.set(requisitions);
    } else {
        store.loading.set(false);
        store.error.set(Some("Requisition not found".to_string()));
        return Err("Requisition not found".to_string());
    }

    store.loading.set(false);

    Ok(updated)
}

/// Submit a requisition for approval
pub async fn submit_requisition(store: &RequisitionsStore, id: &str) -> Result<Requisition, String> {
    store.loading.set(true);
    store.error.set(None);

    let mut requisitions = store.requisitions.get();
    if let Some(pos) = requisitions.iter().position(|r| r.id == id) {
        requisitions[pos].status = RequisitionStatus::Submitted;
        requisitions[pos].updated_at = chrono_now();

        // Add initial approval workflow step if empty
        if requisitions[pos].approval_workflow.is_empty() {
            requisitions[pos].approval_workflow.push(super::types::ApprovalStep {
                step: 1,
                role: "Line Manager".to_string(),
                approver: None,
                status: "pending".to_string(),
                date: None,
                comments: None,
            });
        }

        let updated = requisitions[pos].clone();
        store.requisitions.set(requisitions);
        store.loading.set(false);

        Ok(updated)
    } else {
        store.loading.set(false);
        store.error.set(Some("Requisition not found".to_string()));
        Err("Requisition not found".to_string())
    }
}

/// Delete a requisition (only drafts)
pub async fn delete_requisition(store: &RequisitionsStore, id: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut requisitions = store.requisitions.get();
    if let Some(pos) = requisitions.iter().position(|r| r.id == id) {
        if requisitions[pos].status != RequisitionStatus::Draft {
            store.loading.set(false);
            store.error.set(Some("Only draft requisitions can be deleted".to_string()));
            return Err("Only draft requisitions can be deleted".to_string());
        }

        requisitions.remove(pos);
        store.requisitions.set(requisitions);

        // Update pagination
        let mut pagination = store.pagination.get();
        pagination.update_totals(store.requisitions.get().len() as u32);
        store.pagination.set(pagination);

        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Requisition not found".to_string()));
        Err("Requisition not found".to_string())
    }
}

/// Get a single requisition by ID
pub async fn get_requisition(store: &RequisitionsStore, id: &str) -> Option<Requisition> {
    store.requisitions.get()
        .iter()
        .find(|r| r.id == id)
        .cloned()
}

/// Helper to get current timestamp
fn chrono_now() -> String {
    // In production, use chrono crate
    // For now, return a fixed timestamp
    "2025-01-17T16:00:00Z".to_string()
}
