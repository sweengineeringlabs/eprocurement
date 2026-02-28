//! Dashboard domain types

use serde::{Deserialize, Serialize};

/// Dashboard KPI data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DashboardKpi {
    pub active_requisitions: u32,
    pub open_tenders: u32,
    pub ytd_spend: f64,
    pub active_suppliers: u32,
    pub pending_approvals: u32,
    pub active_contracts: u32,
    pub bbbee_spend_percent: f64,
    pub compliance_issues: u32,
}

impl Default for DashboardKpi {
    fn default() -> Self {
        Self {
            active_requisitions: 127,
            open_tenders: 23,
            ytd_spend: 847_000_000.0,
            active_suppliers: 1284,
            pending_approvals: 34,
            active_contracts: 456,
            bbbee_spend_percent: 78.4,
            compliance_issues: 7,
        }
    }
}

/// Recent requisition for dashboard
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecentRequisition {
    pub id: String,
    pub description: String,
    pub amount: f64,
    pub status: String,
}

/// Activity timeline item
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActivityItem {
    pub timestamp: String,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

/// Spend category data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpendCategory {
    pub name: String,
    pub amount: f64,
}

/// B-BBEE breakdown data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BbbeeBreakdown {
    pub level_1: f64,
    pub level_2: f64,
    pub level_3_4: f64,
    pub other: f64,
    pub target: f64,
    pub current: f64,
}

impl Default for BbbeeBreakdown {
    fn default() -> Self {
        Self {
            level_1: 42.0,
            level_2: 28.0,
            level_3_4: 18.0,
            other: 12.0,
            target: 80.0,
            current: 78.4,
        }
    }
}
