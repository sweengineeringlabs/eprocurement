//! Purchase Order domain types

use serde::{Deserialize, Serialize};

/// Purchase Order status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum PurchaseOrderStatus {
    Draft,
    PendingApproval,
    Approved,
    Sent,
    Acknowledged,
    PartiallyDelivered,
    Delivered,
    Invoiced,
    Closed,
    Cancelled,
}

impl PurchaseOrderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            PurchaseOrderStatus::Draft => "Draft",
            PurchaseOrderStatus::PendingApproval => "Pending Approval",
            PurchaseOrderStatus::Approved => "Approved",
            PurchaseOrderStatus::Sent => "Sent to Supplier",
            PurchaseOrderStatus::Acknowledged => "Acknowledged",
            PurchaseOrderStatus::PartiallyDelivered => "Partially Delivered",
            PurchaseOrderStatus::Delivered => "Delivered",
            PurchaseOrderStatus::Invoiced => "Invoiced",
            PurchaseOrderStatus::Closed => "Closed",
            PurchaseOrderStatus::Cancelled => "Cancelled",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "draft" => PurchaseOrderStatus::Draft,
            "pending_approval" | "pending approval" => PurchaseOrderStatus::PendingApproval,
            "approved" => PurchaseOrderStatus::Approved,
            "sent" | "sent to supplier" => PurchaseOrderStatus::Sent,
            "acknowledged" => PurchaseOrderStatus::Acknowledged,
            "partially_delivered" | "partially delivered" => PurchaseOrderStatus::PartiallyDelivered,
            "delivered" => PurchaseOrderStatus::Delivered,
            "invoiced" => PurchaseOrderStatus::Invoiced,
            "closed" => PurchaseOrderStatus::Closed,
            "cancelled" => PurchaseOrderStatus::Cancelled,
            _ => PurchaseOrderStatus::Draft,
        }
    }
}

impl Default for PurchaseOrderStatus {
    fn default() -> Self {
        PurchaseOrderStatus::Draft
    }
}

/// Purchase Order line item
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LineItem {
    pub id: String,
    pub item_code: String,
    pub description: String,
    pub quantity: u32,
    pub unit: String,
    pub unit_price: f64,
    pub total_price: f64,
    pub tax_rate: f64,
    pub tax_amount: f64,
    pub delivery_date: String,
    pub delivered_quantity: u32,
    pub notes: Option<String>,
}

impl Default for LineItem {
    fn default() -> Self {
        Self {
            id: String::new(),
            item_code: String::new(),
            description: String::new(),
            quantity: 1,
            unit: "Each".to_string(),
            unit_price: 0.0,
            total_price: 0.0,
            tax_rate: 15.0, // VAT rate in South Africa
            tax_amount: 0.0,
            delivery_date: String::new(),
            delivered_quantity: 0,
            notes: None,
        }
    }
}

impl LineItem {
    /// Calculate total price from quantity and unit price
    pub fn calculate_totals(&mut self) {
        self.total_price = self.quantity as f64 * self.unit_price;
        self.tax_amount = self.total_price * (self.tax_rate / 100.0);
    }

    /// Get outstanding quantity to be delivered
    pub fn outstanding_quantity(&self) -> u32 {
        if self.delivered_quantity >= self.quantity {
            0
        } else {
            self.quantity - self.delivered_quantity
        }
    }

    /// Check if line item is fully delivered
    pub fn is_fully_delivered(&self) -> bool {
        self.delivered_quantity >= self.quantity
    }
}

/// Delivery address
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryAddress {
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub city: String,
    pub province: String,
    pub postal_code: String,
    pub country: String,
    pub contact_person: String,
    pub contact_phone: String,
    pub contact_email: String,
    pub delivery_instructions: Option<String>,
}

impl Default for DeliveryAddress {
    fn default() -> Self {
        Self {
            address_line1: String::new(),
            address_line2: None,
            city: String::new(),
            province: String::new(),
            postal_code: String::new(),
            country: "South Africa".to_string(),
            contact_person: String::new(),
            contact_phone: String::new(),
            contact_email: String::new(),
            delivery_instructions: None,
        }
    }
}

/// Supplier reference
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Supplier {
    pub id: String,
    pub name: String,
    pub registration_number: String,
    pub tax_number: String,
    pub bbbee_level: u8,
    pub contact_person: String,
    pub contact_email: String,
    pub contact_phone: String,
    pub address: String,
}

impl Default for Supplier {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            registration_number: String::new(),
            tax_number: String::new(),
            bbbee_level: 4,
            contact_person: String::new(),
            contact_email: String::new(),
            contact_phone: String::new(),
            address: String::new(),
        }
    }
}

/// Purchase Order data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PurchaseOrder {
    pub id: String,
    pub po_number: String,
    pub contract_ref: Option<String>,
    pub requisition_ref: Option<String>,
    pub tender_ref: Option<String>,
    pub supplier: Supplier,
    pub line_items: Vec<LineItem>,
    pub delivery_address: DeliveryAddress,
    pub status: PurchaseOrderStatus,
    pub subtotal: f64,
    pub tax_total: f64,
    pub total_amount: f64,
    pub currency: String,
    pub payment_terms: String,
    pub order_date: String,
    pub expected_delivery_date: String,
    pub actual_delivery_date: Option<String>,
    pub notes: Option<String>,
    pub internal_notes: Option<String>,
    pub attachments: Vec<String>,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
    pub approved_by: Option<String>,
    pub approved_at: Option<String>,
    pub sent_at: Option<String>,
    pub acknowledged_at: Option<String>,
}

impl Default for PurchaseOrder {
    fn default() -> Self {
        Self {
            id: String::new(),
            po_number: String::new(),
            contract_ref: None,
            requisition_ref: None,
            tender_ref: None,
            supplier: Supplier::default(),
            line_items: Vec::new(),
            delivery_address: DeliveryAddress::default(),
            status: PurchaseOrderStatus::Draft,
            subtotal: 0.0,
            tax_total: 0.0,
            total_amount: 0.0,
            currency: "ZAR".to_string(),
            payment_terms: "30 days from invoice".to_string(),
            order_date: String::new(),
            expected_delivery_date: String::new(),
            actual_delivery_date: None,
            notes: None,
            internal_notes: None,
            attachments: Vec::new(),
            created_by: String::new(),
            created_at: String::new(),
            updated_at: String::new(),
            approved_by: None,
            approved_at: None,
            sent_at: None,
            acknowledged_at: None,
        }
    }
}

impl PurchaseOrder {
    /// Calculate totals from line items
    pub fn calculate_totals(&mut self) {
        self.subtotal = self.line_items.iter().map(|item| item.total_price).sum();
        self.tax_total = self.line_items.iter().map(|item| item.tax_amount).sum();
        self.total_amount = self.subtotal + self.tax_total;
    }

    /// Get number of line items
    pub fn line_item_count(&self) -> usize {
        self.line_items.len()
    }

    /// Check if PO is fully delivered
    pub fn is_fully_delivered(&self) -> bool {
        !self.line_items.is_empty() && self.line_items.iter().all(|item| item.is_fully_delivered())
    }

    /// Get delivery progress percentage
    pub fn delivery_progress(&self) -> f64 {
        if self.line_items.is_empty() {
            return 0.0;
        }

        let total_quantity: u32 = self.line_items.iter().map(|item| item.quantity).sum();
        let delivered_quantity: u32 = self.line_items.iter().map(|item| item.delivered_quantity).sum();

        if total_quantity == 0 {
            0.0
        } else {
            (delivered_quantity as f64 / total_quantity as f64) * 100.0
        }
    }

    /// Check if PO can be sent to supplier
    pub fn can_be_sent(&self) -> bool {
        self.status == PurchaseOrderStatus::Approved
    }

    /// Check if PO can be edited
    pub fn can_be_edited(&self) -> bool {
        matches!(self.status, PurchaseOrderStatus::Draft | PurchaseOrderStatus::PendingApproval)
    }

    /// Check if PO can be cancelled
    pub fn can_be_cancelled(&self) -> bool {
        !matches!(
            self.status,
            PurchaseOrderStatus::Delivered
                | PurchaseOrderStatus::Invoiced
                | PurchaseOrderStatus::Closed
                | PurchaseOrderStatus::Cancelled
        )
    }
}

/// Purchase Order filter criteria
#[derive(Clone, Debug, Default)]
pub struct PurchaseOrderFilter {
    pub status: Option<PurchaseOrderStatus>,
    pub supplier_id: Option<String>,
    pub contract_ref: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub search: Option<String>,
    pub pending_delivery: Option<bool>,
}

/// Purchase Order summary for list view
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PurchaseOrderSummary {
    pub id: String,
    pub po_number: String,
    pub contract_ref: Option<String>,
    pub supplier_name: String,
    pub supplier_bbbee_level: u8,
    pub total_amount: f64,
    pub currency: String,
    pub status: PurchaseOrderStatus,
    pub order_date: String,
    pub expected_delivery_date: String,
    pub delivery_progress: f64,
    pub line_item_count: usize,
}

impl From<&PurchaseOrder> for PurchaseOrderSummary {
    fn from(po: &PurchaseOrder) -> Self {
        Self {
            id: po.id.clone(),
            po_number: po.po_number.clone(),
            contract_ref: po.contract_ref.clone(),
            supplier_name: po.supplier.name.clone(),
            supplier_bbbee_level: po.supplier.bbbee_level,
            total_amount: po.total_amount,
            currency: po.currency.clone(),
            status: po.status,
            order_date: po.order_date.clone(),
            expected_delivery_date: po.expected_delivery_date.clone(),
            delivery_progress: po.delivery_progress(),
            line_item_count: po.line_item_count(),
        }
    }
}
