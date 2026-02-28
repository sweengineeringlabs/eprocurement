//! Goods Receipt domain types

use serde::{Deserialize, Serialize};

/// Inspection status for goods received
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum InspectionStatus {
    Pending,
    InProgress,
    Passed,
    Failed,
    PartialPass,
    Waived,
}

impl InspectionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            InspectionStatus::Pending => "Pending",
            InspectionStatus::InProgress => "In Progress",
            InspectionStatus::Passed => "Passed",
            InspectionStatus::Failed => "Failed",
            InspectionStatus::PartialPass => "Partial Pass",
            InspectionStatus::Waived => "Waived",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "pending" => InspectionStatus::Pending,
            "in_progress" | "in progress" => InspectionStatus::InProgress,
            "passed" => InspectionStatus::Passed,
            "failed" => InspectionStatus::Failed,
            "partial_pass" | "partial pass" => InspectionStatus::PartialPass,
            "waived" => InspectionStatus::Waived,
            _ => InspectionStatus::Pending,
        }
    }
}

impl Default for InspectionStatus {
    fn default() -> Self {
        InspectionStatus::Pending
    }
}

/// Goods receipt status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum GoodsReceiptStatus {
    Draft,
    Pending,
    PartiallyReceived,
    Completed,
    Rejected,
    Cancelled,
}

impl GoodsReceiptStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            GoodsReceiptStatus::Draft => "Draft",
            GoodsReceiptStatus::Pending => "Pending",
            GoodsReceiptStatus::PartiallyReceived => "Partially Received",
            GoodsReceiptStatus::Completed => "Completed",
            GoodsReceiptStatus::Rejected => "Rejected",
            GoodsReceiptStatus::Cancelled => "Cancelled",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "draft" => GoodsReceiptStatus::Draft,
            "pending" => GoodsReceiptStatus::Pending,
            "partially_received" | "partially received" => GoodsReceiptStatus::PartiallyReceived,
            "completed" => GoodsReceiptStatus::Completed,
            "rejected" => GoodsReceiptStatus::Rejected,
            "cancelled" => GoodsReceiptStatus::Cancelled,
            _ => GoodsReceiptStatus::Draft,
        }
    }
}

impl Default for GoodsReceiptStatus {
    fn default() -> Self {
        GoodsReceiptStatus::Draft
    }
}

/// Item received in a goods receipt
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReceivedItem {
    pub id: String,
    pub item_code: String,
    pub description: String,
    pub ordered_quantity: u32,
    pub received_quantity: u32,
    pub accepted_quantity: u32,
    pub rejected_quantity: u32,
    pub unit: String,
    pub unit_price: f64,
    pub inspection_status: InspectionStatus,
    pub inspection_notes: Option<String>,
    pub batch_number: Option<String>,
    pub serial_numbers: Vec<String>,
    pub storage_location: Option<String>,
}

impl Default for ReceivedItem {
    fn default() -> Self {
        Self {
            id: String::new(),
            item_code: String::new(),
            description: String::new(),
            ordered_quantity: 0,
            received_quantity: 0,
            accepted_quantity: 0,
            rejected_quantity: 0,
            unit: "Each".to_string(),
            unit_price: 0.0,
            inspection_status: InspectionStatus::Pending,
            inspection_notes: None,
            batch_number: None,
            serial_numbers: Vec::new(),
            storage_location: None,
        }
    }
}

impl ReceivedItem {
    /// Calculate pending quantity to receive
    pub fn pending_quantity(&self) -> u32 {
        if self.ordered_quantity > self.received_quantity {
            self.ordered_quantity - self.received_quantity
        } else {
            0
        }
    }

    /// Calculate total value of received items
    pub fn received_value(&self) -> f64 {
        self.received_quantity as f64 * self.unit_price
    }

    /// Calculate total value of accepted items
    pub fn accepted_value(&self) -> f64 {
        self.accepted_quantity as f64 * self.unit_price
    }
}

/// Supplier information for goods receipt
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GoodsReceiptSupplier {
    pub id: String,
    pub name: String,
    pub contact_person: Option<String>,
    pub contact_phone: Option<String>,
    pub bbbee_level: u8,
}

impl Default for GoodsReceiptSupplier {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            contact_person: None,
            contact_phone: None,
            bbbee_level: 4,
        }
    }
}

/// Purchase order reference
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct POReference {
    pub po_number: String,
    pub po_date: String,
    pub contract_id: Option<String>,
    pub delivery_date: String,
    pub total_value: f64,
}

impl Default for POReference {
    fn default() -> Self {
        Self {
            po_number: String::new(),
            po_date: String::new(),
            contract_id: None,
            delivery_date: String::new(),
            total_value: 0.0,
        }
    }
}

/// Goods receipt data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GoodsReceipt {
    pub id: String,
    pub po_reference: POReference,
    pub supplier: GoodsReceiptSupplier,
    pub items_received: Vec<ReceivedItem>,
    pub inspection_status: InspectionStatus,
    pub received_by: String,
    pub status: GoodsReceiptStatus,
    pub receipt_date: String,
    pub delivery_note_number: Option<String>,
    pub invoice_number: Option<String>,
    pub warehouse_location: String,
    pub notes: Option<String>,
    pub documents: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
    pub completed_by: Option<String>,
}

impl Default for GoodsReceipt {
    fn default() -> Self {
        Self {
            id: String::new(),
            po_reference: POReference::default(),
            supplier: GoodsReceiptSupplier::default(),
            items_received: Vec::new(),
            inspection_status: InspectionStatus::Pending,
            received_by: String::new(),
            status: GoodsReceiptStatus::Draft,
            receipt_date: String::new(),
            delivery_note_number: None,
            invoice_number: None,
            warehouse_location: String::new(),
            notes: None,
            documents: Vec::new(),
            created_at: String::new(),
            updated_at: String::new(),
            completed_at: None,
            completed_by: None,
        }
    }
}

impl GoodsReceipt {
    /// Calculate total received value
    pub fn total_received_value(&self) -> f64 {
        self.items_received.iter().map(|i| i.received_value()).sum()
    }

    /// Calculate total accepted value
    pub fn total_accepted_value(&self) -> f64 {
        self.items_received.iter().map(|i| i.accepted_value()).sum()
    }

    /// Get receipt completion percentage
    pub fn completion_percentage(&self) -> f64 {
        if self.items_received.is_empty() {
            return 0.0;
        }

        let total_ordered: u32 = self.items_received.iter().map(|i| i.ordered_quantity).sum();
        let total_received: u32 = self.items_received.iter().map(|i| i.received_quantity).sum();

        if total_ordered == 0 {
            return 0.0;
        }

        (total_received as f64 / total_ordered as f64) * 100.0
    }

    /// Check if all items have passed inspection
    pub fn all_inspected(&self) -> bool {
        self.items_received.iter().all(|i| {
            i.inspection_status == InspectionStatus::Passed
                || i.inspection_status == InspectionStatus::Waived
        })
    }

    /// Check if receipt is fully complete
    pub fn is_fully_received(&self) -> bool {
        self.items_received
            .iter()
            .all(|i| i.received_quantity >= i.ordered_quantity)
    }

    /// Get count of items pending inspection
    pub fn pending_inspection_count(&self) -> usize {
        self.items_received
            .iter()
            .filter(|i| {
                i.inspection_status == InspectionStatus::Pending
                    || i.inspection_status == InspectionStatus::InProgress
            })
            .count()
    }
}

/// Goods receipt filter criteria
#[derive(Clone, Debug, Default)]
pub struct GoodsReceiptFilter {
    pub status: Option<GoodsReceiptStatus>,
    pub inspection_status: Option<InspectionStatus>,
    pub supplier_id: Option<String>,
    pub search: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub received_by: Option<String>,
}

/// Goods receipt summary for list view
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GoodsReceiptSummary {
    pub id: String,
    pub po_number: String,
    pub supplier_name: String,
    pub supplier_bbbee_level: u8,
    pub total_items: usize,
    pub total_value: f64,
    pub receipt_date: String,
    pub status: GoodsReceiptStatus,
    pub inspection_status: InspectionStatus,
    pub received_by: String,
    pub completion_percentage: f64,
}

impl From<&GoodsReceipt> for GoodsReceiptSummary {
    fn from(gr: &GoodsReceipt) -> Self {
        Self {
            id: gr.id.clone(),
            po_number: gr.po_reference.po_number.clone(),
            supplier_name: gr.supplier.name.clone(),
            supplier_bbbee_level: gr.supplier.bbbee_level,
            total_items: gr.items_received.len(),
            total_value: gr.total_received_value(),
            receipt_date: gr.receipt_date.clone(),
            status: gr.status,
            inspection_status: gr.inspection_status,
            received_by: gr.received_by.clone(),
            completion_percentage: gr.completion_percentage(),
        }
    }
}
