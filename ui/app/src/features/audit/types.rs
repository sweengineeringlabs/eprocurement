//! Audit domain types

use serde::{Deserialize, Serialize};

/// Entity types that can be audited
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AuditEntityType {
    Requisition,
    Tender,
    Bid,
    Evaluation,
    Contract,
    PurchaseOrder,
    GoodsReceipt,
    Supplier,
    User,
    System,
}

impl AuditEntityType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditEntityType::Requisition => "Requisition",
            AuditEntityType::Tender => "Tender",
            AuditEntityType::Bid => "Bid",
            AuditEntityType::Evaluation => "Evaluation",
            AuditEntityType::Contract => "Contract",
            AuditEntityType::PurchaseOrder => "Purchase Order",
            AuditEntityType::GoodsReceipt => "Goods Receipt",
            AuditEntityType::Supplier => "Supplier",
            AuditEntityType::User => "User",
            AuditEntityType::System => "System",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "requisition" => AuditEntityType::Requisition,
            "tender" => AuditEntityType::Tender,
            "bid" => AuditEntityType::Bid,
            "evaluation" => AuditEntityType::Evaluation,
            "contract" => AuditEntityType::Contract,
            "purchase_order" | "purchaseorder" | "purchase order" => AuditEntityType::PurchaseOrder,
            "goods_receipt" | "goodsreceipt" | "goods receipt" => AuditEntityType::GoodsReceipt,
            "supplier" => AuditEntityType::Supplier,
            "user" => AuditEntityType::User,
            "system" => AuditEntityType::System,
            _ => AuditEntityType::System,
        }
    }

    /// Get all entity types for filter dropdown
    pub fn all() -> Vec<AuditEntityType> {
        vec![
            AuditEntityType::Requisition,
            AuditEntityType::Tender,
            AuditEntityType::Bid,
            AuditEntityType::Evaluation,
            AuditEntityType::Contract,
            AuditEntityType::PurchaseOrder,
            AuditEntityType::GoodsReceipt,
            AuditEntityType::Supplier,
            AuditEntityType::User,
            AuditEntityType::System,
        ]
    }
}

impl Default for AuditEntityType {
    fn default() -> Self {
        AuditEntityType::System
    }
}

/// Action types that can be audited
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AuditActionType {
    Create,
    Update,
    Delete,
    View,
    Approve,
    Reject,
    Submit,
    Cancel,
    Login,
    Logout,
    Export,
    Import,
}

impl AuditActionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditActionType::Create => "Create",
            AuditActionType::Update => "Update",
            AuditActionType::Delete => "Delete",
            AuditActionType::View => "View",
            AuditActionType::Approve => "Approve",
            AuditActionType::Reject => "Reject",
            AuditActionType::Submit => "Submit",
            AuditActionType::Cancel => "Cancel",
            AuditActionType::Login => "Login",
            AuditActionType::Logout => "Logout",
            AuditActionType::Export => "Export",
            AuditActionType::Import => "Import",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "create" => AuditActionType::Create,
            "update" => AuditActionType::Update,
            "delete" => AuditActionType::Delete,
            "view" => AuditActionType::View,
            "approve" => AuditActionType::Approve,
            "reject" => AuditActionType::Reject,
            "submit" => AuditActionType::Submit,
            "cancel" => AuditActionType::Cancel,
            "login" => AuditActionType::Login,
            "logout" => AuditActionType::Logout,
            "export" => AuditActionType::Export,
            "import" => AuditActionType::Import,
            _ => AuditActionType::View,
        }
    }

    /// Get all action types for filter dropdown
    pub fn all() -> Vec<AuditActionType> {
        vec![
            AuditActionType::Create,
            AuditActionType::Update,
            AuditActionType::Delete,
            AuditActionType::View,
            AuditActionType::Approve,
            AuditActionType::Reject,
            AuditActionType::Submit,
            AuditActionType::Cancel,
            AuditActionType::Login,
            AuditActionType::Logout,
            AuditActionType::Export,
            AuditActionType::Import,
        ]
    }
}

impl Default for AuditActionType {
    fn default() -> Self {
        AuditActionType::View
    }
}

/// Field change record for tracking what changed
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldChange {
    pub field_name: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

impl FieldChange {
    pub fn new(field_name: &str, old_value: Option<&str>, new_value: Option<&str>) -> Self {
        Self {
            field_name: field_name.to_string(),
            old_value: old_value.map(|s| s.to_string()),
            new_value: new_value.map(|s| s.to_string()),
        }
    }
}

/// Audit entry representing a single audit log record
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: String,
    pub timestamp: String,
    pub user_id: String,
    pub user_name: String,
    pub user_email: String,
    pub action: AuditActionType,
    pub entity_type: AuditEntityType,
    pub entity_id: String,
    pub entity_name: Option<String>,
    pub changes: Vec<FieldChange>,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub session_id: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<String>,
}

impl Default for AuditEntry {
    fn default() -> Self {
        Self {
            id: String::new(),
            timestamp: String::new(),
            user_id: String::new(),
            user_name: String::new(),
            user_email: String::new(),
            action: AuditActionType::View,
            entity_type: AuditEntityType::System,
            entity_id: String::new(),
            entity_name: None,
            changes: Vec::new(),
            ip_address: String::new(),
            user_agent: None,
            session_id: None,
            description: None,
            metadata: None,
        }
    }
}

impl AuditEntry {
    /// Get a human-readable summary of the audit entry
    pub fn summary(&self) -> String {
        let entity_desc = self.entity_name.clone()
            .unwrap_or_else(|| self.entity_id.clone());
        format!(
            "{} {} {} ({})",
            self.user_name,
            self.action.as_str().to_lowercase(),
            self.entity_type.as_str().to_lowercase(),
            entity_desc
        )
    }

    /// Check if entry has changes to display
    pub fn has_changes(&self) -> bool {
        !self.changes.is_empty()
    }
}

/// Filter criteria for audit entries
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AuditFilter {
    pub entity_type: Option<AuditEntityType>,
    pub action_type: Option<AuditActionType>,
    pub user_id: Option<String>,
    pub user_search: Option<String>,
    pub entity_id: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub ip_address: Option<String>,
    pub search: Option<String>,
}

impl AuditFilter {
    pub fn is_empty(&self) -> bool {
        self.entity_type.is_none()
            && self.action_type.is_none()
            && self.user_id.is_none()
            && self.user_search.is_none()
            && self.entity_id.is_none()
            && self.date_from.is_none()
            && self.date_to.is_none()
            && self.ip_address.is_none()
            && self.search.is_none()
    }

    pub fn clear(&mut self) {
        self.entity_type = None;
        self.action_type = None;
        self.user_id = None;
        self.user_search = None;
        self.entity_id = None;
        self.date_from = None;
        self.date_to = None;
        self.ip_address = None;
        self.search = None;
    }
}

/// Audit statistics summary
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AuditStats {
    pub total_entries: usize,
    pub entries_today: usize,
    pub unique_users: usize,
    pub creates: usize,
    pub updates: usize,
    pub deletes: usize,
    pub logins: usize,
}

/// Audit export request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditExportRequest {
    pub filter: AuditFilter,
    pub format: ExportFormat,
    pub include_changes: bool,
}

/// Export format options
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ExportFormat {
    Csv,
    Excel,
    Pdf,
    Json,
}

impl ExportFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            ExportFormat::Csv => "CSV",
            ExportFormat::Excel => "Excel",
            ExportFormat::Pdf => "PDF",
            ExportFormat::Json => "JSON",
        }
    }
}

impl Default for ExportFormat {
    fn default() -> Self {
        ExportFormat::Csv
    }
}
