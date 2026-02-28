//! Purchase Orders service - API calls

use super::store::{PurchaseOrdersStore, load_mock_purchase_orders, get_mock_purchase_order};
use super::types::{PurchaseOrder, PurchaseOrderStatus};

/// Load all purchase orders
pub async fn load_purchase_orders(store: &PurchaseOrdersStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    // For now, load mock data
    load_mock_purchase_orders(store);

    store.loading.set(false);
}

/// Load a single purchase order by ID
pub async fn load_purchase_order(store: &PurchaseOrdersStore, id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    // For now, get mock data
    if let Some(po) = get_mock_purchase_order(id) {
        store.selected.set(Some(po));
    } else {
        store.error.set(Some(format!("Purchase Order {} not found", id)));
    }

    store.loading.set(false);
}

/// Create a new purchase order
pub async fn create_purchase_order(store: &PurchaseOrdersStore, mut po: PurchaseOrder) -> Result<String, String> {
    store.saving.set(true);
    store.error.set(None);

    // Validate required fields
    if po.supplier.id.is_empty() {
        store.saving.set(false);
        return Err("Supplier is required".to_string());
    }

    if po.line_items.is_empty() {
        store.saving.set(false);
        return Err("At least one line item is required".to_string());
    }

    if po.delivery_address.address_line1.is_empty() {
        store.saving.set(false);
        return Err("Delivery address is required".to_string());
    }

    if po.expected_delivery_date.is_empty() {
        store.saving.set(false);
        return Err("Expected delivery date is required".to_string());
    }

    // Validate line items
    for (idx, item) in po.line_items.iter().enumerate() {
        if item.description.is_empty() {
            store.saving.set(false);
            return Err(format!("Line item {} description is required", idx + 1));
        }
        if item.quantity == 0 {
            store.saving.set(false);
            return Err(format!("Line item {} quantity must be greater than zero", idx + 1));
        }
        if item.unit_price <= 0.0 {
            store.saving.set(false);
            return Err(format!("Line item {} unit price must be greater than zero", idx + 1));
        }
    }

    // Calculate totals
    po.calculate_totals();

    // Generate new ID
    let new_id = format!("PO-2025-{:04}", rand_id());
    po.id = new_id.clone();
    po.po_number = new_id.clone();
    po.order_date = chrono_now_date();
    po.created_at = chrono_now();
    po.updated_at = chrono_now();

    // In production, POST to API and reload list
    load_mock_purchase_orders(store);

    store.saving.set(false);
    Ok(new_id)
}

/// Update an existing purchase order
pub async fn update_purchase_order(store: &PurchaseOrdersStore, mut po: PurchaseOrder) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    // Validate required fields
    if po.id.is_empty() {
        store.saving.set(false);
        return Err("Purchase Order ID is required for update".to_string());
    }

    // Check if PO can be edited
    if !po.can_be_edited() {
        store.saving.set(false);
        return Err(format!(
            "Purchase Order cannot be edited in {} status",
            po.status.as_str()
        ));
    }

    // Calculate totals
    po.calculate_totals();
    po.updated_at = chrono_now();

    // In production, PUT to API and reload
    store.selected.set(Some(po));
    load_mock_purchase_orders(store);

    store.saving.set(false);
    Ok(())
}

/// Update purchase order status
pub async fn update_purchase_order_status(
    store: &PurchaseOrdersStore,
    po_id: &str,
    new_status: PurchaseOrderStatus,
) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    if let Some(mut po) = store.selected.get() {
        if po.id == po_id {
            // Validate status transition
            let valid_transition = match (po.status, new_status) {
                (PurchaseOrderStatus::Draft, PurchaseOrderStatus::PendingApproval) => true,
                (PurchaseOrderStatus::PendingApproval, PurchaseOrderStatus::Approved) => true,
                (PurchaseOrderStatus::PendingApproval, PurchaseOrderStatus::Draft) => true,
                (PurchaseOrderStatus::Approved, PurchaseOrderStatus::Sent) => true,
                (PurchaseOrderStatus::Sent, PurchaseOrderStatus::Acknowledged) => true,
                (PurchaseOrderStatus::Acknowledged, PurchaseOrderStatus::PartiallyDelivered) => true,
                (PurchaseOrderStatus::Acknowledged, PurchaseOrderStatus::Delivered) => true,
                (PurchaseOrderStatus::PartiallyDelivered, PurchaseOrderStatus::Delivered) => true,
                (PurchaseOrderStatus::Delivered, PurchaseOrderStatus::Invoiced) => true,
                (PurchaseOrderStatus::Invoiced, PurchaseOrderStatus::Closed) => true,
                // Cancellation can happen from various states
                (PurchaseOrderStatus::Draft, PurchaseOrderStatus::Cancelled) => true,
                (PurchaseOrderStatus::PendingApproval, PurchaseOrderStatus::Cancelled) => true,
                (PurchaseOrderStatus::Approved, PurchaseOrderStatus::Cancelled) => true,
                (PurchaseOrderStatus::Sent, PurchaseOrderStatus::Cancelled) => true,
                (PurchaseOrderStatus::Acknowledged, PurchaseOrderStatus::Cancelled) => true,
                _ => false,
            };

            if !valid_transition {
                store.saving.set(false);
                return Err(format!(
                    "Invalid status transition from {} to {}",
                    po.status.as_str(),
                    new_status.as_str()
                ));
            }

            po.status = new_status;
            po.updated_at = chrono_now();

            // Set timestamps based on status
            match new_status {
                PurchaseOrderStatus::Sent => {
                    po.sent_at = Some(chrono_now());
                }
                PurchaseOrderStatus::Acknowledged => {
                    po.acknowledged_at = Some(chrono_now());
                }
                PurchaseOrderStatus::Delivered => {
                    po.actual_delivery_date = Some(chrono_now_date());
                }
                _ => {}
            }

            store.selected.set(Some(po));
        }
    }

    load_mock_purchase_orders(store);
    store.saving.set(false);
    Ok(())
}

/// Submit purchase order for approval
pub async fn submit_for_approval(store: &PurchaseOrdersStore, po_id: &str) -> Result<(), String> {
    update_purchase_order_status(store, po_id, PurchaseOrderStatus::PendingApproval).await
}

/// Approve a purchase order
pub async fn approve_purchase_order(
    store: &PurchaseOrdersStore,
    po_id: &str,
    approver: &str,
) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    if let Some(mut po) = store.selected.get() {
        if po.id == po_id {
            if po.status != PurchaseOrderStatus::PendingApproval {
                store.saving.set(false);
                return Err("Purchase Order must be in Pending Approval status to approve".to_string());
            }

            po.status = PurchaseOrderStatus::Approved;
            po.approved_by = Some(approver.to_string());
            po.approved_at = Some(chrono_now());
            po.updated_at = chrono_now();
            store.selected.set(Some(po));
        }
    }

    load_mock_purchase_orders(store);
    store.saving.set(false);
    Ok(())
}

/// Reject a purchase order (return to draft)
pub async fn reject_purchase_order(
    store: &PurchaseOrdersStore,
    po_id: &str,
    reason: &str,
) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    if reason.is_empty() {
        store.saving.set(false);
        return Err("Rejection reason is required".to_string());
    }

    if let Some(mut po) = store.selected.get() {
        if po.id == po_id {
            if po.status != PurchaseOrderStatus::PendingApproval {
                store.saving.set(false);
                return Err("Purchase Order must be in Pending Approval status to reject".to_string());
            }

            po.status = PurchaseOrderStatus::Draft;
            po.internal_notes = Some(format!(
                "{}Rejected: {}",
                po.internal_notes.map(|n| format!("{}\n\n", n)).unwrap_or_default(),
                reason
            ));
            po.updated_at = chrono_now();
            store.selected.set(Some(po));
        }
    }

    load_mock_purchase_orders(store);
    store.saving.set(false);
    Ok(())
}

/// Send purchase order to supplier
pub async fn send_to_supplier(store: &PurchaseOrdersStore, po_id: &str) -> Result<(), String> {
    let po = store.selected.get();

    if let Some(ref po) = po {
        if !po.can_be_sent() {
            store.saving.set(false);
            return Err("Purchase Order must be approved before sending to supplier".to_string());
        }
    }

    update_purchase_order_status(store, po_id, PurchaseOrderStatus::Sent).await
}

/// Record supplier acknowledgement
pub async fn record_acknowledgement(store: &PurchaseOrdersStore, po_id: &str) -> Result<(), String> {
    update_purchase_order_status(store, po_id, PurchaseOrderStatus::Acknowledged).await
}

/// Record delivery (partial or full)
pub async fn record_delivery(
    store: &PurchaseOrdersStore,
    po_id: &str,
    is_full_delivery: bool,
) -> Result<(), String> {
    let new_status = if is_full_delivery {
        PurchaseOrderStatus::Delivered
    } else {
        PurchaseOrderStatus::PartiallyDelivered
    };
    update_purchase_order_status(store, po_id, new_status).await
}

/// Cancel a purchase order
pub async fn cancel_purchase_order(
    store: &PurchaseOrdersStore,
    po_id: &str,
    reason: &str,
) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    if reason.is_empty() {
        store.saving.set(false);
        return Err("Cancellation reason is required".to_string());
    }

    if let Some(ref po) = store.selected.get() {
        if !po.can_be_cancelled() {
            store.saving.set(false);
            return Err(format!(
                "Purchase Order in {} status cannot be cancelled",
                po.status.as_str()
            ));
        }
    }

    if let Some(mut po) = store.selected.get() {
        if po.id == po_id {
            po.internal_notes = Some(format!(
                "{}Cancelled: {}",
                po.internal_notes.map(|n| format!("{}\n\n", n)).unwrap_or_default(),
                reason
            ));
            store.selected.set(Some(po));
        }
    }

    update_purchase_order_status(store, po_id, PurchaseOrderStatus::Cancelled).await
}

/// Delete a draft purchase order
pub async fn delete_purchase_order(store: &PurchaseOrdersStore, po_id: &str) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    if let Some(ref po) = store.selected.get() {
        if po.id == po_id && po.status != PurchaseOrderStatus::Draft {
            store.saving.set(false);
            return Err("Only draft purchase orders can be deleted".to_string());
        }
    }

    // In production, DELETE to API
    store.selected.set(None);
    load_mock_purchase_orders(store);

    store.saving.set(false);
    Ok(())
}

/// Export purchase order as PDF
pub async fn export_purchase_order_pdf(po_id: &str) -> Result<Vec<u8>, String> {
    // In production, this would call the API to generate PDF
    // For now, return empty bytes
    Ok(Vec::new())
}

/// Duplicate a purchase order (create new from existing)
pub async fn duplicate_purchase_order(store: &PurchaseOrdersStore, po_id: &str) -> Result<String, String> {
    store.saving.set(true);
    store.error.set(None);

    if let Some(existing_po) = get_mock_purchase_order(po_id) {
        let mut new_po = existing_po.clone();
        let new_id = format!("PO-2025-{:04}", rand_id());

        new_po.id = new_id.clone();
        new_po.po_number = new_id.clone();
        new_po.status = PurchaseOrderStatus::Draft;
        new_po.order_date = chrono_now_date();
        new_po.expected_delivery_date = String::new();
        new_po.actual_delivery_date = None;
        new_po.created_at = chrono_now();
        new_po.updated_at = chrono_now();
        new_po.approved_by = None;
        new_po.approved_at = None;
        new_po.sent_at = None;
        new_po.acknowledged_at = None;

        // Reset delivery quantities
        for item in &mut new_po.line_items {
            item.delivered_quantity = 0;
        }

        store.selected.set(Some(new_po));
        load_mock_purchase_orders(store);

        store.saving.set(false);
        return Ok(new_id);
    }

    store.saving.set(false);
    Err(format!("Purchase Order {} not found", po_id))
}

/// Get purchase orders pending delivery
pub async fn get_pending_delivery_orders(store: &PurchaseOrdersStore) {
    store.loading.set(true);
    store.set_filter_pending_delivery(Some(true));
    load_mock_purchase_orders(store);
    store.loading.set(false);
}

/// Get purchase orders by supplier
pub async fn get_orders_by_supplier(store: &PurchaseOrdersStore, supplier_id: &str) {
    store.loading.set(true);
    store.set_filter_supplier(Some(supplier_id.to_string()));
    load_mock_purchase_orders(store);
    store.loading.set(false);
}

/// Get purchase orders by contract
pub async fn get_orders_by_contract(store: &PurchaseOrdersStore, contract_ref: &str) {
    store.loading.set(true);
    store.set_filter_contract(Some(contract_ref.to_string()));
    load_mock_purchase_orders(store);
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

fn chrono_now_date() -> String {
    // In production, use chrono crate
    "2025-02-27".to_string()
}
