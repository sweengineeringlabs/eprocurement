//! Dashboard store

use components::prelude::*;
use super::types::{DashboardKpi, RecentRequisition, ActivityItem, SpendCategory, BbbeeBreakdown};

/// Dashboard state store
#[derive(Clone)]
pub struct DashboardStore {
    pub kpis: Signal<DashboardKpi>,
    pub recent_requisitions: Signal<Vec<RecentRequisition>>,
    pub activities: Signal<Vec<ActivityItem>>,
    pub spend_by_category: Signal<Vec<SpendCategory>>,
    pub bbbee_breakdown: Signal<BbbeeBreakdown>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
}

impl DashboardStore {
    pub fn new() -> Self {
        Self {
            kpis: signal(DashboardKpi::default()),
            recent_requisitions: signal(Vec::new()),
            activities: signal(Vec::new()),
            spend_by_category: signal(Vec::new()),
            bbbee_breakdown: signal(BbbeeBreakdown::default()),
            loading: signal(false),
            error: signal(None),
        }
    }
}

/// Load mock data for demo
pub fn load_mock_data(store: &DashboardStore) {
    store.recent_requisitions.set(vec![
        RecentRequisition {
            id: "REQ-2025-0847".to_string(),
            description: "IT Hardware - Laptops Q4".to_string(),
            amount: 2_450_000.0,
            status: "pending".to_string(),
        },
        RecentRequisition {
            id: "REQ-2025-0846".to_string(),
            description: "Office Supplies - Stationery".to_string(),
            amount: 125_000.0,
            status: "approved".to_string(),
        },
        RecentRequisition {
            id: "REQ-2025-0845".to_string(),
            description: "Security Services - Annual".to_string(),
            amount: 8_750_000.0,
            status: "evaluation".to_string(),
        },
        RecentRequisition {
            id: "REQ-2025-0844".to_string(),
            description: "Fleet Maintenance Services".to_string(),
            amount: 1_200_000.0,
            status: "approved".to_string(),
        },
        RecentRequisition {
            id: "REQ-2025-0843".to_string(),
            description: "Training - Staff Development".to_string(),
            amount: 560_000.0,
            status: "draft".to_string(),
        },
    ]);

    store.activities.set(vec![
        ActivityItem {
            timestamp: "Today, 14:32".to_string(),
            title: "Tender RFP-2025-0089 Published".to_string(),
            description: "IT Infrastructure Upgrade - Open for bidding".to_string(),
            completed: true,
        },
        ActivityItem {
            timestamp: "Today, 11:15".to_string(),
            title: "Contract CTR-2025-0234 Signed".to_string(),
            description: "Awarded to TechSolutions SA (Pty) Ltd".to_string(),
            completed: true,
        },
        ActivityItem {
            timestamp: "Today, 09:45".to_string(),
            title: "Requisition REQ-2025-0847 Submitted".to_string(),
            description: "Awaiting budget holder approval".to_string(),
            completed: false,
        },
        ActivityItem {
            timestamp: "Yesterday, 16:20".to_string(),
            title: "NBAC Meeting Completed".to_string(),
            description: "3 bids adjudicated and approved".to_string(),
            completed: true,
        },
        ActivityItem {
            timestamp: "Yesterday, 10:00".to_string(),
            title: "Supplier Onboarded".to_string(),
            description: "GreenTech Solutions - B-BBEE Level 1".to_string(),
            completed: true,
        },
    ]);

    store.spend_by_category.set(vec![
        SpendCategory { name: "IT".to_string(), amount: 250_000_000.0 },
        SpendCategory { name: "Facilities".to_string(), amount: 180_000_000.0 },
        SpendCategory { name: "Prof. Svcs".to_string(), amount: 200_000_000.0 },
        SpendCategory { name: "Security".to_string(), amount: 120_000_000.0 },
        SpendCategory { name: "Fleet".to_string(), amount: 97_000_000.0 },
    ]);
}
