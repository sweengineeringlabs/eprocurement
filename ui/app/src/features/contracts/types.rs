//! Contract domain types

use serde::{Deserialize, Serialize};

/// Contract status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ContractStatus {
    Draft,
    PendingApproval,
    Active,
    Suspended,
    Expired,
    Terminated,
    Completed,
}

impl ContractStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContractStatus::Draft => "Draft",
            ContractStatus::PendingApproval => "Pending Approval",
            ContractStatus::Active => "Active",
            ContractStatus::Suspended => "Suspended",
            ContractStatus::Expired => "Expired",
            ContractStatus::Terminated => "Terminated",
            ContractStatus::Completed => "Completed",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "draft" => ContractStatus::Draft,
            "pending_approval" | "pending approval" => ContractStatus::PendingApproval,
            "active" => ContractStatus::Active,
            "suspended" => ContractStatus::Suspended,
            "expired" => ContractStatus::Expired,
            "terminated" => ContractStatus::Terminated,
            "completed" => ContractStatus::Completed,
            _ => ContractStatus::Draft,
        }
    }
}

impl Default for ContractStatus {
    fn default() -> Self {
        ContractStatus::Draft
    }
}

/// Milestone status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum MilestoneStatus {
    Pending,
    InProgress,
    Completed,
    Overdue,
    Cancelled,
}

impl MilestoneStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            MilestoneStatus::Pending => "Pending",
            MilestoneStatus::InProgress => "In Progress",
            MilestoneStatus::Completed => "Completed",
            MilestoneStatus::Overdue => "Overdue",
            MilestoneStatus::Cancelled => "Cancelled",
        }
    }
}

impl Default for MilestoneStatus {
    fn default() -> Self {
        MilestoneStatus::Pending
    }
}

/// Contract milestone
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContractMilestone {
    pub id: String,
    pub contract_id: String,
    pub title: String,
    pub description: String,
    pub due_date: String,
    pub completed_date: Option<String>,
    pub payment_amount: f64,
    pub payment_percentage: f64,
    pub status: MilestoneStatus,
    pub deliverables: Vec<String>,
    pub notes: Option<String>,
}

impl Default for ContractMilestone {
    fn default() -> Self {
        Self {
            id: String::new(),
            contract_id: String::new(),
            title: String::new(),
            description: String::new(),
            due_date: String::new(),
            completed_date: None,
            payment_amount: 0.0,
            payment_percentage: 0.0,
            status: MilestoneStatus::Pending,
            deliverables: Vec::new(),
            notes: None,
        }
    }
}

/// Contract SLA terms
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContractSla {
    pub response_time_hours: u32,
    pub resolution_time_hours: u32,
    pub availability_percent: f64,
    pub penalty_clause: String,
    pub escalation_procedure: String,
}

impl Default for ContractSla {
    fn default() -> Self {
        Self {
            response_time_hours: 4,
            resolution_time_hours: 24,
            availability_percent: 99.5,
            penalty_clause: String::new(),
            escalation_procedure: String::new(),
        }
    }
}

/// Contract deliverable
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContractDeliverable {
    pub id: String,
    pub description: String,
    pub quantity: u32,
    pub unit: String,
    pub unit_price: f64,
    pub total_price: f64,
    pub delivery_date: String,
    pub delivered: bool,
}

impl Default for ContractDeliverable {
    fn default() -> Self {
        Self {
            id: String::new(),
            description: String::new(),
            quantity: 1,
            unit: "Each".to_string(),
            unit_price: 0.0,
            total_price: 0.0,
            delivery_date: String::new(),
            delivered: false,
        }
    }
}

/// Contract terms
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContractTerms {
    pub payment_terms: String,
    pub warranty_period_months: u32,
    pub notice_period_days: u32,
    pub renewal_terms: String,
    pub termination_clause: String,
    pub dispute_resolution: String,
    pub governing_law: String,
    pub special_conditions: Vec<String>,
}

impl Default for ContractTerms {
    fn default() -> Self {
        Self {
            payment_terms: "30 days from invoice".to_string(),
            warranty_period_months: 12,
            notice_period_days: 30,
            renewal_terms: String::new(),
            termination_clause: String::new(),
            dispute_resolution: "Arbitration".to_string(),
            governing_law: "South African Law".to_string(),
            special_conditions: Vec::new(),
        }
    }
}

/// Contract data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Contract {
    pub id: String,
    pub title: String,
    pub description: String,
    pub supplier_id: String,
    pub supplier_name: String,
    pub supplier_bbbee_level: u8,
    pub value: f64,
    pub start_date: String,
    pub end_date: String,
    pub status: ContractStatus,
    pub contract_type: String,
    pub reference_number: String,
    pub tender_id: Option<String>,
    pub purchase_order_id: Option<String>,
    pub terms: ContractTerms,
    pub sla: Option<ContractSla>,
    pub deliverables: Vec<ContractDeliverable>,
    pub milestones: Vec<ContractMilestone>,
    pub documents: Vec<String>,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
    pub approved_by: Option<String>,
    pub approved_at: Option<String>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            description: String::new(),
            supplier_id: String::new(),
            supplier_name: String::new(),
            supplier_bbbee_level: 4,
            value: 0.0,
            start_date: String::new(),
            end_date: String::new(),
            status: ContractStatus::Draft,
            contract_type: "Goods".to_string(),
            reference_number: String::new(),
            tender_id: None,
            purchase_order_id: None,
            terms: ContractTerms::default(),
            sla: None,
            deliverables: Vec::new(),
            milestones: Vec::new(),
            documents: Vec::new(),
            created_by: String::new(),
            created_at: String::new(),
            updated_at: String::new(),
            approved_by: None,
            approved_at: None,
        }
    }
}

impl Contract {
    /// Check if contract is expiring within given days
    pub fn is_expiring_within_days(&self, days: u32) -> bool {
        // Simplified check - in production would use proper date comparison
        // For now, just check if end_date is within range
        if self.end_date.is_empty() {
            return false;
        }
        // Mock implementation - would compare with current date
        days <= 90 && self.status == ContractStatus::Active
    }

    /// Calculate total milestone payments
    pub fn total_milestone_payments(&self) -> f64 {
        self.milestones.iter().map(|m| m.payment_amount).sum()
    }

    /// Calculate completed milestone payments
    pub fn completed_milestone_payments(&self) -> f64 {
        self.milestones
            .iter()
            .filter(|m| m.status == MilestoneStatus::Completed)
            .map(|m| m.payment_amount)
            .sum()
    }

    /// Get progress percentage based on milestones
    pub fn milestone_progress(&self) -> f64 {
        if self.milestones.is_empty() {
            return 0.0;
        }
        let completed = self.milestones.iter().filter(|m| m.status == MilestoneStatus::Completed).count();
        (completed as f64 / self.milestones.len() as f64) * 100.0
    }
}

/// Contract filter criteria
#[derive(Clone, Debug, Default)]
pub struct ContractFilter {
    pub status: Option<ContractStatus>,
    pub supplier_id: Option<String>,
    pub contract_type: Option<String>,
    pub search: Option<String>,
    pub expiring_within_days: Option<u32>,
}

/// Contract summary for list view
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContractSummary {
    pub id: String,
    pub title: String,
    pub supplier_name: String,
    pub supplier_bbbee_level: u8,
    pub value: f64,
    pub start_date: String,
    pub end_date: String,
    pub status: ContractStatus,
    pub days_to_expiry: Option<i32>,
    pub milestone_progress: f64,
}

impl From<&Contract> for ContractSummary {
    fn from(contract: &Contract) -> Self {
        Self {
            id: contract.id.clone(),
            title: contract.title.clone(),
            supplier_name: contract.supplier_name.clone(),
            supplier_bbbee_level: contract.supplier_bbbee_level,
            value: contract.value,
            start_date: contract.start_date.clone(),
            end_date: contract.end_date.clone(),
            status: contract.status,
            days_to_expiry: None, // Would be calculated from end_date
            milestone_progress: contract.milestone_progress(),
        }
    }
}
