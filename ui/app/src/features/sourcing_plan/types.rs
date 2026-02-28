//! Sourcing Plan domain types

use serde::{Deserialize, Serialize};

/// Sourcing Plan status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum SourcingPlanStatus {
    Draft,
    UnderReview,
    Approved,
    Active,
    Completed,
    Cancelled,
}

impl SourcingPlanStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            SourcingPlanStatus::Draft => "draft",
            SourcingPlanStatus::UnderReview => "under_review",
            SourcingPlanStatus::Approved => "approved",
            SourcingPlanStatus::Active => "active",
            SourcingPlanStatus::Completed => "completed",
            SourcingPlanStatus::Cancelled => "cancelled",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            SourcingPlanStatus::Draft => "Draft",
            SourcingPlanStatus::UnderReview => "Under Review",
            SourcingPlanStatus::Approved => "Approved",
            SourcingPlanStatus::Active => "Active",
            SourcingPlanStatus::Completed => "Completed",
            SourcingPlanStatus::Cancelled => "Cancelled",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "draft" => SourcingPlanStatus::Draft,
            "under_review" => SourcingPlanStatus::UnderReview,
            "approved" => SourcingPlanStatus::Approved,
            "active" => SourcingPlanStatus::Active,
            "completed" => SourcingPlanStatus::Completed,
            "cancelled" => SourcingPlanStatus::Cancelled,
            _ => SourcingPlanStatus::Draft,
        }
    }
}

/// Procurement category within a sourcing plan
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcurementCategory {
    pub id: String,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub allocated_budget: f64,
    pub spent_amount: f64,
    pub planned_tenders: u32,
    pub completed_tenders: u32,
    pub priority: CategoryPriority,
    pub bbbee_target: Option<f64>,
    pub local_content_target: Option<f64>,
}

impl ProcurementCategory {
    pub fn new() -> Self {
        Self {
            id: format!("CAT-{}", uuid_short()),
            name: String::new(),
            code: String::new(),
            description: None,
            allocated_budget: 0.0,
            spent_amount: 0.0,
            planned_tenders: 0,
            completed_tenders: 0,
            priority: CategoryPriority::Medium,
            bbbee_target: None,
            local_content_target: None,
        }
    }

    pub fn remaining_budget(&self) -> f64 {
        self.allocated_budget - self.spent_amount
    }

    pub fn utilization_percentage(&self) -> f64 {
        if self.allocated_budget > 0.0 {
            (self.spent_amount / self.allocated_budget) * 100.0
        } else {
            0.0
        }
    }
}

/// Category priority level
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CategoryPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl CategoryPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            CategoryPriority::Low => "low",
            CategoryPriority::Medium => "medium",
            CategoryPriority::High => "high",
            CategoryPriority::Critical => "critical",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            CategoryPriority::Low => "Low",
            CategoryPriority::Medium => "Medium",
            CategoryPriority::High => "High",
            CategoryPriority::Critical => "Critical",
        }
    }
}

/// Timeline milestone for sourcing plan
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimelineMilestone {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub planned_date: String,
    pub actual_date: Option<String>,
    pub status: MilestoneStatus,
    pub responsible_party: Option<String>,
}

impl TimelineMilestone {
    pub fn new() -> Self {
        Self {
            id: format!("MS-{}", uuid_short()),
            name: String::new(),
            description: None,
            planned_date: String::new(),
            actual_date: None,
            status: MilestoneStatus::Pending,
            responsible_party: None,
        }
    }
}

/// Milestone status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum MilestoneStatus {
    Pending,
    InProgress,
    Completed,
    Delayed,
    Skipped,
}

impl MilestoneStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            MilestoneStatus::Pending => "pending",
            MilestoneStatus::InProgress => "in_progress",
            MilestoneStatus::Completed => "completed",
            MilestoneStatus::Delayed => "delayed",
            MilestoneStatus::Skipped => "skipped",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            MilestoneStatus::Pending => "Pending",
            MilestoneStatus::InProgress => "In Progress",
            MilestoneStatus::Completed => "Completed",
            MilestoneStatus::Delayed => "Delayed",
            MilestoneStatus::Skipped => "Skipped",
        }
    }
}

/// Budget allocation within a sourcing plan
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BudgetAllocation {
    pub total_budget: f64,
    pub allocated_amount: f64,
    pub committed_amount: f64,
    pub spent_amount: f64,
    pub currency: String,
    pub fiscal_year_start: String,
    pub fiscal_year_end: String,
}

impl Default for BudgetAllocation {
    fn default() -> Self {
        Self {
            total_budget: 0.0,
            allocated_amount: 0.0,
            committed_amount: 0.0,
            spent_amount: 0.0,
            currency: "ZAR".to_string(),
            fiscal_year_start: String::new(),
            fiscal_year_end: String::new(),
        }
    }
}

impl BudgetAllocation {
    pub fn available_budget(&self) -> f64 {
        self.total_budget - self.committed_amount - self.spent_amount
    }

    pub fn utilization_percentage(&self) -> f64 {
        if self.total_budget > 0.0 {
            ((self.committed_amount + self.spent_amount) / self.total_budget) * 100.0
        } else {
            0.0
        }
    }
}

/// Approval step for sourcing plan
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlanApprovalStep {
    pub step: u32,
    pub role: String,
    pub approver: Option<String>,
    pub status: String,
    pub date: Option<String>,
    pub comments: Option<String>,
}

/// Complete Sourcing Plan entity
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourcingPlan {
    pub id: String,
    pub title: String,
    pub description: String,
    pub fiscal_year: String,
    pub department: String,
    pub status: SourcingPlanStatus,
    pub categories: Vec<ProcurementCategory>,
    pub budget: BudgetAllocation,
    pub timeline: Vec<TimelineMilestone>,
    pub approval_workflow: Vec<PlanApprovalStep>,
    pub owner: String,
    pub owner_email: String,
    pub created_at: String,
    pub updated_at: String,
    pub approved_at: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub strategic_objectives: Vec<String>,
    pub risk_assessment: Option<String>,
    pub notes: Option<String>,
}

impl Default for SourcingPlan {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            description: String::new(),
            fiscal_year: String::new(),
            department: String::new(),
            status: SourcingPlanStatus::Draft,
            categories: Vec::new(),
            budget: BudgetAllocation::default(),
            timeline: Vec::new(),
            approval_workflow: Vec::new(),
            owner: String::new(),
            owner_email: String::new(),
            created_at: String::new(),
            updated_at: String::new(),
            approved_at: None,
            start_date: String::new(),
            end_date: String::new(),
            strategic_objectives: Vec::new(),
            risk_assessment: None,
            notes: None,
        }
    }
}

impl SourcingPlan {
    pub fn calculate_totals(&mut self) {
        self.budget.allocated_amount = self.categories.iter()
            .map(|c| c.allocated_budget)
            .sum();
        self.budget.spent_amount = self.categories.iter()
            .map(|c| c.spent_amount)
            .sum();
    }

    pub fn total_planned_tenders(&self) -> u32 {
        self.categories.iter()
            .map(|c| c.planned_tenders)
            .sum()
    }

    pub fn total_completed_tenders(&self) -> u32 {
        self.categories.iter()
            .map(|c| c.completed_tenders)
            .sum()
    }

    pub fn completion_percentage(&self) -> f64 {
        let total = self.total_planned_tenders();
        if total > 0 {
            (self.total_completed_tenders() as f64 / total as f64) * 100.0
        } else {
            0.0
        }
    }
}

/// Filter options for sourcing plan list
#[derive(Clone, Debug, Default)]
pub struct SourcingPlanFilter {
    pub search: String,
    pub status: Option<SourcingPlanStatus>,
    pub fiscal_year: Option<String>,
    pub department: Option<String>,
    pub min_budget: Option<f64>,
    pub max_budget: Option<f64>,
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
