//! Goods Receipt service - API calls

use super::store::{GoodsReceiptStore, load_mock_receipts, get_mock_receipt};
use super::types::{GoodsReceipt, GoodsReceiptStatus, InspectionStatus, ReceivedItem};

/// Load all goods receipts
pub async fn load_receipts(store: &GoodsReceiptStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    // For now, load mock data
    load_mock_receipts(store);

    store.loading.set(false);
}

/// Load a single goods receipt by ID
pub async fn load_receipt(store: &GoodsReceiptStore, id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    // For now, get mock data
    if let Some(receipt) = get_mock_receipt(id) {
        store.selected.set(Some(receipt));
    } else {
        store.error.set(Some(format!("Goods Receipt {} not found", id)));
    }

    store.loading.set(false);
}

/// Create a new goods receipt
pub async fn create_receipt(
    store: &GoodsReceiptStore,
    receipt: GoodsReceipt,
) -> Result<String, String> {
    store.saving.set(true);
    store.error.set(None);

    // Validate required fields
    if receipt.po_reference.po_number.is_empty() {
        store.saving.set(false);
        return Err("Purchase Order reference is required".to_string());
    }

    if receipt.supplier.id.is_empty() {
        store.saving.set(false);
        return Err("Supplier is required".to_string());
    }

    if receipt.received_by.is_empty() {
        store.saving.set(false);
        return Err("Received by is required".to_string());
    }

    if receipt.warehouse_location.is_empty() {
        store.saving.set(false);
        return Err("Warehouse location is required".to_string());
    }

    // Generate new ID
    let new_id = format!("GR-2025-{:04}", rand_id());

    // In production, POST to API and reload list
    load_mock_receipts(store);

    store.saving.set(false);
    Ok(new_id)
}

/// Update an existing goods receipt
pub async fn update_receipt(
    store: &GoodsReceiptStore,
    receipt: GoodsReceipt,
) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    // Validate required fields
    if receipt.id.is_empty() {
        store.saving.set(false);
        return Err("Goods Receipt ID is required for update".to_string());
    }

    // In production, PUT to API and reload
    store.selected.set(Some(receipt));
    load_mock_receipts(store);

    store.saving.set(false);
    Ok(())
}

/// Update goods receipt status
pub async fn update_receipt_status(
    store: &GoodsReceiptStore,
    receipt_id: &str,
    new_status: GoodsReceiptStatus,
) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    if let Some(mut receipt) = store.selected.get() {
        if receipt.id == receipt_id {
            // Validate status transition
            let valid_transition = match (receipt.status, new_status) {
                (GoodsReceiptStatus::Draft, GoodsReceiptStatus::Pending) => true,
                (GoodsReceiptStatus::Pending, GoodsReceiptStatus::PartiallyReceived) => true,
                (GoodsReceiptStatus::Pending, GoodsReceiptStatus::Completed) => true,
                (GoodsReceiptStatus::Pending, GoodsReceiptStatus::Rejected) => true,
                (GoodsReceiptStatus::Pending, GoodsReceiptStatus::Cancelled) => true,
                (GoodsReceiptStatus::PartiallyReceived, GoodsReceiptStatus::Completed) => true,
                (GoodsReceiptStatus::PartiallyReceived, GoodsReceiptStatus::Rejected) => true,
                (GoodsReceiptStatus::PartiallyReceived, GoodsReceiptStatus::Cancelled) => true,
                (GoodsReceiptStatus::Draft, GoodsReceiptStatus::Cancelled) => true,
                _ => false,
            };

            if !valid_transition {
                store.saving.set(false);
                return Err(format!(
                    "Invalid status transition from {} to {}",
                    receipt.status.as_str(),
                    new_status.as_str()
                ));
            }

            receipt.status = new_status;
            receipt.updated_at = chrono_now();

            if new_status == GoodsReceiptStatus::Completed {
                receipt.completed_at = Some(chrono_now());
            }

            store.selected.set(Some(receipt));
        }
    }

    load_mock_receipts(store);
    store.saving.set(false);
    Ok(())
}

/// Record item receipt (update received quantities)
pub async fn record_item_receipt(
    store: &GoodsReceiptStore,
    receipt_id: &str,
    item_id: &str,
    received_quantity: u32,
    batch_number: Option<String>,
    serial_numbers: Vec<String>,
    storage_location: Option<String>,
) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    if let Some(mut receipt) = store.selected.get() {
        if receipt.id == receipt_id {
            if let Some(item) = receipt.items_received.iter_mut().find(|i| i.id == item_id) {
                if received_quantity > item.ordered_quantity {
                    store.saving.set(false);
                    return Err("Received quantity cannot exceed ordered quantity".to_string());
                }

                item.received_quantity = received_quantity;
                item.batch_number = batch_number;
                item.serial_numbers = serial_numbers;
                item.storage_location = storage_location;

                receipt.updated_at = chrono_now();

                // Update receipt status based on items
                if receipt.is_fully_received() {
                    receipt.status = GoodsReceiptStatus::Completed;
                } else if receipt.items_received.iter().any(|i| i.received_quantity > 0) {
                    receipt.status = GoodsReceiptStatus::PartiallyReceived;
                }

                store.selected.set(Some(receipt));
            } else {
                store.saving.set(false);
                return Err(format!("Item {} not found in receipt", item_id));
            }
        }
    }

    store.saving.set(false);
    Ok(())
}

/// Record inspection result for an item
pub async fn record_inspection(
    store: &GoodsReceiptStore,
    receipt_id: &str,
    item_id: &str,
    inspection_status: InspectionStatus,
    accepted_quantity: u32,
    rejected_quantity: u32,
    notes: Option<String>,
) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    if let Some(mut receipt) = store.selected.get() {
        if receipt.id == receipt_id {
            if let Some(item) = receipt.items_received.iter_mut().find(|i| i.id == item_id) {
                if accepted_quantity + rejected_quantity > item.received_quantity {
                    store.saving.set(false);
                    return Err(
                        "Accepted + rejected quantity cannot exceed received quantity".to_string()
                    );
                }

                item.inspection_status = inspection_status;
                item.accepted_quantity = accepted_quantity;
                item.rejected_quantity = rejected_quantity;
                item.inspection_notes = notes;

                receipt.updated_at = chrono_now();

                // Update overall inspection status
                update_overall_inspection_status(&mut receipt);

                store.selected.set(Some(receipt));
            } else {
                store.saving.set(false);
                return Err(format!("Item {} not found in receipt", item_id));
            }
        }
    }

    store.saving.set(false);
    Ok(())
}

/// Complete goods receipt
pub async fn complete_receipt(
    store: &GoodsReceiptStore,
    receipt_id: &str,
    completed_by: &str,
) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    if let Some(mut receipt) = store.selected.get() {
        if receipt.id == receipt_id {
            // Validate all items have been inspected
            let pending_inspection = receipt.pending_inspection_count();
            if pending_inspection > 0 {
                store.saving.set(false);
                return Err(format!(
                    "{} item(s) still pending inspection",
                    pending_inspection
                ));
            }

            receipt.status = GoodsReceiptStatus::Completed;
            receipt.completed_at = Some(chrono_now());
            receipt.completed_by = Some(completed_by.to_string());
            receipt.updated_at = chrono_now();

            store.selected.set(Some(receipt));
        }
    }

    load_mock_receipts(store);
    store.saving.set(false);
    Ok(())
}

/// Reject goods receipt
pub async fn reject_receipt(
    store: &GoodsReceiptStore,
    receipt_id: &str,
    reason: &str,
) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    if reason.is_empty() {
        store.saving.set(false);
        return Err("Rejection reason is required".to_string());
    }

    if let Some(mut receipt) = store.selected.get() {
        if receipt.id == receipt_id {
            receipt.status = GoodsReceiptStatus::Rejected;
            receipt.notes = Some(format!(
                "REJECTED: {}{}",
                reason,
                receipt.notes.map(|n| format!("\n\nPrevious notes: {}", n)).unwrap_or_default()
            ));
            receipt.updated_at = chrono_now();

            store.selected.set(Some(receipt));
        }
    }

    load_mock_receipts(store);
    store.saving.set(false);
    Ok(())
}

/// Cancel goods receipt
pub async fn cancel_receipt(
    store: &GoodsReceiptStore,
    receipt_id: &str,
    reason: &str,
) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    if reason.is_empty() {
        store.saving.set(false);
        return Err("Cancellation reason is required".to_string());
    }

    if let Some(receipt) = store.selected.get() {
        if receipt.id == receipt_id {
            if receipt.status == GoodsReceiptStatus::Completed {
                store.saving.set(false);
                return Err("Cannot cancel a completed goods receipt".to_string());
            }
        }
    }

    update_receipt_status(store, receipt_id, GoodsReceiptStatus::Cancelled).await
}

/// Delete a draft goods receipt
pub async fn delete_receipt(store: &GoodsReceiptStore, receipt_id: &str) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    if let Some(receipt) = store.selected.get() {
        if receipt.id == receipt_id && receipt.status != GoodsReceiptStatus::Draft {
            store.saving.set(false);
            return Err("Only draft goods receipts can be deleted".to_string());
        }
    }

    // In production, DELETE to API
    store.selected.set(None);
    load_mock_receipts(store);

    store.saving.set(false);
    Ok(())
}

/// Get receipts by PO number
pub async fn get_receipts_by_po(store: &GoodsReceiptStore, po_number: &str) {
    store.loading.set(true);

    let mut filter = store.filter.get();
    filter.search = Some(po_number.to_string());
    store.filter.set(filter);

    load_mock_receipts(store);
    store.loading.set(false);
}

/// Export goods receipt as PDF
pub async fn export_receipt_pdf(receipt_id: &str) -> Result<Vec<u8>, String> {
    // In production, this would call the API to generate PDF
    // For now, return empty bytes
    Ok(Vec::new())
}

/// Print goods receipt note
pub async fn print_receipt_note(receipt_id: &str) -> Result<(), String> {
    // In production, this would trigger print
    Ok(())
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

fn update_overall_inspection_status(receipt: &mut GoodsReceipt) {
    let all_passed = receipt.items_received.iter().all(|i| {
        i.inspection_status == InspectionStatus::Passed
            || i.inspection_status == InspectionStatus::Waived
    });

    let all_failed = receipt
        .items_received
        .iter()
        .all(|i| i.inspection_status == InspectionStatus::Failed);

    let any_in_progress = receipt
        .items_received
        .iter()
        .any(|i| i.inspection_status == InspectionStatus::InProgress);

    let any_pending = receipt
        .items_received
        .iter()
        .any(|i| i.inspection_status == InspectionStatus::Pending);

    if all_passed {
        receipt.inspection_status = InspectionStatus::Passed;
    } else if all_failed {
        receipt.inspection_status = InspectionStatus::Failed;
    } else if any_in_progress {
        receipt.inspection_status = InspectionStatus::InProgress;
    } else if any_pending {
        receipt.inspection_status = InspectionStatus::Pending;
    } else {
        receipt.inspection_status = InspectionStatus::PartialPass;
    }
}
