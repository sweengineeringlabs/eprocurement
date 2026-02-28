//! Contracts service - API calls

use super::store::{ContractsStore, load_mock_contracts, get_mock_contract};
use super::types::{Contract, ContractStatus};

/// Load all contracts
pub async fn load_contracts(store: &ContractsStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    // For now, load mock data
    load_mock_contracts(store);

    store.loading.set(false);
}

/// Load a single contract by ID
pub async fn load_contract(store: &ContractsStore, id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    // For now, get mock data
    if let Some(contract) = get_mock_contract(id) {
        store.selected.set(Some(contract));
    } else {
        store.error.set(Some(format!("Contract {} not found", id)));
    }

    store.loading.set(false);
}

/// Create a new contract
pub async fn create_contract(store: &ContractsStore, contract: Contract) -> Result<String, String> {
    store.saving.set(true);
    store.error.set(None);

    // In production, this would call the API
    // Simulate API delay
    // await sleep(500);

    // Validate required fields
    if contract.title.is_empty() {
        store.saving.set(false);
        return Err("Contract title is required".to_string());
    }

    if contract.supplier_id.is_empty() {
        store.saving.set(false);
        return Err("Supplier is required".to_string());
    }

    if contract.value <= 0.0 {
        store.saving.set(false);
        return Err("Contract value must be greater than zero".to_string());
    }

    if contract.start_date.is_empty() || contract.end_date.is_empty() {
        store.saving.set(false);
        return Err("Contract dates are required".to_string());
    }

    // Generate new ID
    let new_id = format!("CTR-2025-{:04}", rand_id());

    // In production, POST to API and reload list
    load_mock_contracts(store);

    store.saving.set(false);
    Ok(new_id)
}

/// Update an existing contract
pub async fn update_contract(store: &ContractsStore, contract: Contract) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    // In production, this would call the API
    // Validate required fields
    if contract.id.is_empty() {
        store.saving.set(false);
        return Err("Contract ID is required for update".to_string());
    }

    if contract.title.is_empty() {
        store.saving.set(false);
        return Err("Contract title is required".to_string());
    }

    // In production, PUT to API and reload
    store.selected.set(Some(contract));
    load_mock_contracts(store);

    store.saving.set(false);
    Ok(())
}

/// Update contract status
pub async fn update_contract_status(
    store: &ContractsStore,
    contract_id: &str,
    new_status: ContractStatus,
) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    // In production, this would call the API
    // Validate status transition
    if let Some(mut contract) = store.selected.get() {
        if contract.id == contract_id {
            // Validate status transition
            let valid_transition = match (contract.status, new_status) {
                (ContractStatus::Draft, ContractStatus::PendingApproval) => true,
                (ContractStatus::PendingApproval, ContractStatus::Active) => true,
                (ContractStatus::PendingApproval, ContractStatus::Draft) => true,
                (ContractStatus::Active, ContractStatus::Suspended) => true,
                (ContractStatus::Active, ContractStatus::Terminated) => true,
                (ContractStatus::Active, ContractStatus::Completed) => true,
                (ContractStatus::Suspended, ContractStatus::Active) => true,
                (ContractStatus::Suspended, ContractStatus::Terminated) => true,
                _ => false,
            };

            if !valid_transition {
                store.saving.set(false);
                return Err(format!(
                    "Invalid status transition from {} to {}",
                    contract.status.as_str(),
                    new_status.as_str()
                ));
            }

            contract.status = new_status;
            store.selected.set(Some(contract));
        }
    }

    load_mock_contracts(store);
    store.saving.set(false);
    Ok(())
}

/// Submit contract for approval
pub async fn submit_for_approval(store: &ContractsStore, contract_id: &str) -> Result<(), String> {
    update_contract_status(store, contract_id, ContractStatus::PendingApproval).await
}

/// Approve a contract
pub async fn approve_contract(
    store: &ContractsStore,
    contract_id: &str,
    approver: &str,
) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    if let Some(mut contract) = store.selected.get() {
        if contract.id == contract_id {
            if contract.status != ContractStatus::PendingApproval {
                store.saving.set(false);
                return Err("Contract must be in Pending Approval status to approve".to_string());
            }

            contract.status = ContractStatus::Active;
            contract.approved_by = Some(approver.to_string());
            contract.approved_at = Some(chrono_now());
            store.selected.set(Some(contract));
        }
    }

    load_mock_contracts(store);
    store.saving.set(false);
    Ok(())
}

/// Terminate a contract
pub async fn terminate_contract(
    store: &ContractsStore,
    contract_id: &str,
    reason: &str,
) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    if reason.is_empty() {
        store.saving.set(false);
        return Err("Termination reason is required".to_string());
    }

    update_contract_status(store, contract_id, ContractStatus::Terminated).await
}

/// Delete a draft contract
pub async fn delete_contract(store: &ContractsStore, contract_id: &str) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    if let Some(contract) = store.selected.get() {
        if contract.id == contract_id && contract.status != ContractStatus::Draft {
            store.saving.set(false);
            return Err("Only draft contracts can be deleted".to_string());
        }
    }

    // In production, DELETE to API
    store.selected.set(None);
    load_mock_contracts(store);

    store.saving.set(false);
    Ok(())
}

/// Export contract as PDF
pub async fn export_contract_pdf(contract_id: &str) -> Result<Vec<u8>, String> {
    // In production, this would call the API to generate PDF
    // For now, return empty bytes
    Ok(Vec::new())
}

/// Get contracts expiring within days
pub async fn get_expiring_contracts(store: &ContractsStore, days: u32) {
    store.loading.set(true);

    let mut filter = store.filter.get();
    filter.expiring_within_days = Some(days);
    filter.status = Some(ContractStatus::Active);
    store.filter.set(filter);

    load_mock_contracts(store);
    store.loading.set(false);
}

// Helper functions
fn rand_id() -> u32 {
    // Simple pseudo-random for demo
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    (duration.as_millis() % 10000) as u32
}

fn chrono_now() -> String {
    // In production, use chrono crate
    "2025-02-27T10:00:00Z".to_string()
}
