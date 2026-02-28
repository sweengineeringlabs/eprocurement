//! Requisition domain types

use serde::{Deserialize, Serialize};

/// Requisition status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum RequisitionStatus {
    Draft,
    Submitted,
    PendingApproval,
    Approved,
    Rejected,
    Cancelled,
    InProgress,
    Complete,
}

impl RequisitionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            RequisitionStatus::Draft => "draft",
            RequisitionStatus::Submitted => "submitted",
            RequisitionStatus::PendingApproval => "pending",
            RequisitionStatus::Approved => "approved",
            RequisitionStatus::Rejected => "rejected",
            RequisitionStatus::Cancelled => "cancelled",
            RequisitionStatus::InProgress => "in_progress",
            RequisitionStatus::Complete => "complete",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            RequisitionStatus::Draft => "Draft",
            RequisitionStatus::Submitted => "Submitted",
            RequisitionStatus::PendingApproval => "Pending Approval",
            RequisitionStatus::Approved => "Approved",
            RequisitionStatus::Rejected => "Rejected",
            RequisitionStatus::Cancelled => "Cancelled",
            RequisitionStatus::InProgress => "In Progress",
            RequisitionStatus::Complete => "Complete",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "draft" => RequisitionStatus::Draft,
            "submitted" => RequisitionStatus::Submitted,
            "pending" => RequisitionStatus::PendingApproval,
            "approved" => RequisitionStatus::Approved,
            "rejected" => RequisitionStatus::Rejected,
            "cancelled" => RequisitionStatus::Cancelled,
            "in_progress" => RequisitionStatus::InProgress,
            "complete" => RequisitionStatus::Complete,
            _ => RequisitionStatus::Draft,
        }
    }
}

/// Priority level
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

impl Priority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
            Priority::Urgent => "urgent",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
            Priority::Urgent => "Urgent",
        }
    }
}

/// Line item in a requisition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LineItem {
    pub id: String,
    pub description: String,
    pub category: String,
    pub quantity: u32,
    pub unit: String,
    pub unit_price: f64,
    pub total: f64,
    pub specifications: Option<String>,
    pub catalogue_item_id: Option<String>,
}

impl LineItem {
    pub fn new() -> Self {
        Self {
            id: format!("LI-{}", uuid_short()),
            description: String::new(),
            category: String::new(),
            quantity: 1,
            unit: "Each".to_string(),
            unit_price: 0.0,
            total: 0.0,
            specifications: None,
            catalogue_item_id: None,
        }
    }

    pub fn calculate_total(&mut self) {
        self.total = self.quantity as f64 * self.unit_price;
    }
}

/// Attachment for requisition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attachment {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub file_type: String,
    pub uploaded_at: String,
}

/// Approval step in workflow
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApprovalStep {
    pub step: u32,
    pub role: String,
    pub approver: Option<String>,
    pub status: String, // "pending", "approved", "rejected"
    pub date: Option<String>,
    pub comments: Option<String>,
}

/// Complete requisition entity
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Requisition {
    pub id: String,
    pub description: String,
    pub justification: String,
    pub amount: f64,
    pub status: RequisitionStatus,
    pub priority: Priority,
    pub department: String,
    pub cost_center: String,
    pub requester: String,
    pub requester_email: String,
    pub line_items: Vec<LineItem>,
    pub attachments: Vec<Attachment>,
    pub approval_workflow: Vec<ApprovalStep>,
    pub created_at: String,
    pub updated_at: String,
    pub required_by: Option<String>,
    pub delivery_address: String,
    pub notes: Option<String>,
}

impl Default for Requisition {
    fn default() -> Self {
        Self {
            id: String::new(),
            description: String::new(),
            justification: String::new(),
            amount: 0.0,
            status: RequisitionStatus::Draft,
            priority: Priority::Medium,
            department: String::new(),
            cost_center: String::new(),
            requester: String::new(),
            requester_email: String::new(),
            line_items: Vec::new(),
            attachments: Vec::new(),
            approval_workflow: Vec::new(),
            created_at: String::new(),
            updated_at: String::new(),
            required_by: None,
            delivery_address: String::new(),
            notes: None,
        }
    }
}

impl Requisition {
    pub fn calculate_total(&mut self) {
        self.amount = self.line_items.iter().map(|li| li.total).sum();
    }
}

/// Filter options for requisition list
#[derive(Clone, Debug, Default)]
pub struct RequisitionFilter {
    pub search: String,
    pub status: Option<RequisitionStatus>,
    pub department: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
}

/// Pagination state
#[derive(Clone, Debug)]
pub struct PaginationState {
    pub current_page: u32,
    pub page_size: u32,
    pub total_items: u32,
    pub total_pages: u32,
}

impl Default for PaginationState {
    fn default() -> Self {
        Self {
            current_page: 1,
            page_size: 10,
            total_items: 0,
            total_pages: 0,
        }
    }
}

impl PaginationState {
    pub fn update_totals(&mut self, total_items: u32) {
        self.total_items = total_items;
        self.total_pages = (total_items + self.page_size - 1) / self.page_size;
        if self.current_page > self.total_pages && self.total_pages > 0 {
            self.current_page = self.total_pages;
        }
    }
}

/// Helper to generate short UUID
fn uuid_short() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("{:x}", time)
}
