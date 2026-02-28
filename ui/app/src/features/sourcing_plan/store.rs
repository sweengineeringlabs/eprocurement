//! Sourcing Plan store

use components::prelude::*;
use super::types::{
    SourcingPlan, SourcingPlanStatus, SourcingPlanFilter, PaginationState,
    ProcurementCategory, CategoryPriority, TimelineMilestone, MilestoneStatus,
    BudgetAllocation, PlanApprovalStep,
};

/// Sourcing Plan state store
#[derive(Clone)]
pub struct SourcingPlanStore {
    pub plans: Signal<Vec<SourcingPlan>>,
    pub selected: Signal<Option<SourcingPlan>>,
    pub filter: Signal<SourcingPlanFilter>,
    pub pagination: Signal<PaginationState>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
    pub form_step: Signal<u32>,
    pub form_data: Signal<SourcingPlan>,
}

impl SourcingPlanStore {
    pub fn new() -> Self {
        Self {
            plans: signal(Vec::new()),
            selected: signal(None),
            filter: signal(SourcingPlanFilter::default()),
            pagination: signal(PaginationState::default()),
            loading: signal(false),
            error: signal(None),
            form_step: signal(1),
            form_data: signal(SourcingPlan::default()),
        }
    }

    pub fn reset_form(&self) {
        self.form_step.set(1);
        self.form_data.set(SourcingPlan::default());
    }

    pub fn set_selected(&self, id: &str) {
        let plan = self.plans.get()
            .iter()
            .find(|p| p.id == id)
            .cloned();
        self.selected.set(plan);
    }
}

/// Load mock data for demo
pub fn load_mock_data(store: &SourcingPlanStore) {
    let mock_plans = vec![
        SourcingPlan {
            id: "SP-2025-001".to_string(),
            title: "Annual Procurement Plan FY 2025/26".to_string(),
            description: "Comprehensive annual sourcing plan for all government departments covering IT, facilities, and operational requirements.".to_string(),
            fiscal_year: "2025/26".to_string(),
            department: "Procurement Division".to_string(),
            status: SourcingPlanStatus::Active,
            categories: vec![
                ProcurementCategory {
                    id: "CAT-001".to_string(),
                    name: "Information Technology".to_string(),
                    code: "IT-HW-SW".to_string(),
                    description: Some("Hardware, software, and IT services".to_string()),
                    allocated_budget: 45_000_000.0,
                    spent_amount: 12_500_000.0,
                    planned_tenders: 15,
                    completed_tenders: 4,
                    priority: CategoryPriority::High,
                    bbbee_target: Some(40.0),
                    local_content_target: Some(50.0),
                },
                ProcurementCategory {
                    id: "CAT-002".to_string(),
                    name: "Facilities Management".to_string(),
                    code: "FAC-MGT".to_string(),
                    description: Some("Building maintenance, security, and cleaning services".to_string()),
                    allocated_budget: 28_000_000.0,
                    spent_amount: 8_750_000.0,
                    planned_tenders: 8,
                    completed_tenders: 3,
                    priority: CategoryPriority::Medium,
                    bbbee_target: Some(50.0),
                    local_content_target: Some(70.0),
                },
                ProcurementCategory {
                    id: "CAT-003".to_string(),
                    name: "Fleet & Transport".to_string(),
                    code: "FLT-TRN".to_string(),
                    description: Some("Vehicle procurement, maintenance, and fuel".to_string()),
                    allocated_budget: 18_500_000.0,
                    spent_amount: 4_200_000.0,
                    planned_tenders: 6,
                    completed_tenders: 2,
                    priority: CategoryPriority::Medium,
                    bbbee_target: Some(35.0),
                    local_content_target: Some(60.0),
                },
                ProcurementCategory {
                    id: "CAT-004".to_string(),
                    name: "Professional Services".to_string(),
                    code: "PRO-SVC".to_string(),
                    description: Some("Consulting, legal, and audit services".to_string()),
                    allocated_budget: 22_000_000.0,
                    spent_amount: 5_500_000.0,
                    planned_tenders: 10,
                    completed_tenders: 3,
                    priority: CategoryPriority::High,
                    bbbee_target: Some(60.0),
                    local_content_target: Some(80.0),
                },
            ],
            budget: BudgetAllocation {
                total_budget: 120_000_000.0,
                allocated_amount: 113_500_000.0,
                committed_amount: 45_000_000.0,
                spent_amount: 30_950_000.0,
                currency: "ZAR".to_string(),
                fiscal_year_start: "2025-04-01".to_string(),
                fiscal_year_end: "2026-03-31".to_string(),
            },
            timeline: vec![
                TimelineMilestone {
                    id: "MS-001".to_string(),
                    name: "Plan Approval".to_string(),
                    description: Some("Final approval from CFO and Accounting Officer".to_string()),
                    planned_date: "2025-03-15".to_string(),
                    actual_date: Some("2025-03-12".to_string()),
                    status: MilestoneStatus::Completed,
                    responsible_party: Some("CFO Office".to_string()),
                },
                TimelineMilestone {
                    id: "MS-002".to_string(),
                    name: "Q1 Tenders Published".to_string(),
                    description: Some("First quarter tender advertisements".to_string()),
                    planned_date: "2025-04-30".to_string(),
                    actual_date: Some("2025-04-28".to_string()),
                    status: MilestoneStatus::Completed,
                    responsible_party: Some("SCM Unit".to_string()),
                },
                TimelineMilestone {
                    id: "MS-003".to_string(),
                    name: "Q2 Tenders Published".to_string(),
                    description: Some("Second quarter tender advertisements".to_string()),
                    planned_date: "2025-07-31".to_string(),
                    actual_date: None,
                    status: MilestoneStatus::InProgress,
                    responsible_party: Some("SCM Unit".to_string()),
                },
                TimelineMilestone {
                    id: "MS-004".to_string(),
                    name: "Mid-Year Review".to_string(),
                    description: Some("Budget utilization and plan adjustment review".to_string()),
                    planned_date: "2025-09-30".to_string(),
                    actual_date: None,
                    status: MilestoneStatus::Pending,
                    responsible_party: Some("Finance & SCM".to_string()),
                },
                TimelineMilestone {
                    id: "MS-005".to_string(),
                    name: "Q3/Q4 Completion".to_string(),
                    description: Some("Complete remaining planned procurements".to_string()),
                    planned_date: "2026-02-28".to_string(),
                    actual_date: None,
                    status: MilestoneStatus::Pending,
                    responsible_party: Some("SCM Unit".to_string()),
                },
            ],
            approval_workflow: vec![
                PlanApprovalStep {
                    step: 1,
                    role: "SCM Manager".to_string(),
                    approver: Some("Thandi Nkosi".to_string()),
                    status: "approved".to_string(),
                    date: Some("2025-03-01".to_string()),
                    comments: Some("Plan aligns with departmental objectives".to_string()),
                },
                PlanApprovalStep {
                    step: 2,
                    role: "CFO".to_string(),
                    approver: Some("Nomvula Zulu".to_string()),
                    status: "approved".to_string(),
                    date: Some("2025-03-08".to_string()),
                    comments: Some("Budget allocation approved".to_string()),
                },
                PlanApprovalStep {
                    step: 3,
                    role: "Accounting Officer".to_string(),
                    approver: Some("Dr. James Molefe".to_string()),
                    status: "approved".to_string(),
                    date: Some("2025-03-12".to_string()),
                    comments: None,
                },
            ],
            owner: "Sipho Dlamini".to_string(),
            owner_email: "sipho.dlamini@gov.za".to_string(),
            created_at: "2025-02-01T10:00:00Z".to_string(),
            updated_at: "2025-07-15T14:30:00Z".to_string(),
            approved_at: Some("2025-03-12T09:00:00Z".to_string()),
            start_date: "2025-04-01".to_string(),
            end_date: "2026-03-31".to_string(),
            strategic_objectives: vec![
                "Achieve 40% B-BBEE Level 1-4 supplier participation".to_string(),
                "Reduce procurement cycle time by 15%".to_string(),
                "Increase local content to 60% across categories".to_string(),
                "Implement e-procurement for all tenders above R500,000".to_string(),
            ],
            risk_assessment: Some("Key risks include market volatility, supplier capacity constraints, and budget adjustments. Mitigation strategies in place.".to_string()),
            notes: Some("This plan supersedes SP-2024-001. Quarterly reviews scheduled.".to_string()),
        },
        SourcingPlan {
            id: "SP-2025-002".to_string(),
            title: "ICT Modernization Programme 2025".to_string(),
            description: "Strategic sourcing plan for digital transformation and ICT infrastructure modernization.".to_string(),
            fiscal_year: "2025/26".to_string(),
            department: "Information Technology".to_string(),
            status: SourcingPlanStatus::Active,
            categories: vec![
                ProcurementCategory {
                    id: "CAT-005".to_string(),
                    name: "Cloud Services".to_string(),
                    code: "ICT-CLD".to_string(),
                    description: Some("Public and private cloud infrastructure".to_string()),
                    allocated_budget: 15_000_000.0,
                    spent_amount: 3_200_000.0,
                    planned_tenders: 4,
                    completed_tenders: 1,
                    priority: CategoryPriority::Critical,
                    bbbee_target: Some(30.0),
                    local_content_target: Some(40.0),
                },
                ProcurementCategory {
                    id: "CAT-006".to_string(),
                    name: "Cybersecurity".to_string(),
                    code: "ICT-SEC".to_string(),
                    description: Some("Security software, hardware, and managed services".to_string()),
                    allocated_budget: 12_000_000.0,
                    spent_amount: 4_500_000.0,
                    planned_tenders: 5,
                    completed_tenders: 2,
                    priority: CategoryPriority::Critical,
                    bbbee_target: Some(35.0),
                    local_content_target: Some(45.0),
                },
                ProcurementCategory {
                    id: "CAT-007".to_string(),
                    name: "Network Infrastructure".to_string(),
                    code: "ICT-NET".to_string(),
                    description: Some("LAN, WAN, and telecommunications".to_string()),
                    allocated_budget: 8_000_000.0,
                    spent_amount: 1_800_000.0,
                    planned_tenders: 3,
                    completed_tenders: 1,
                    priority: CategoryPriority::High,
                    bbbee_target: Some(40.0),
                    local_content_target: Some(50.0),
                },
            ],
            budget: BudgetAllocation {
                total_budget: 40_000_000.0,
                allocated_amount: 35_000_000.0,
                committed_amount: 15_000_000.0,
                spent_amount: 9_500_000.0,
                currency: "ZAR".to_string(),
                fiscal_year_start: "2025-04-01".to_string(),
                fiscal_year_end: "2026-03-31".to_string(),
            },
            timeline: vec![
                TimelineMilestone {
                    id: "MS-006".to_string(),
                    name: "Security Assessment".to_string(),
                    description: Some("Complete cybersecurity gap analysis".to_string()),
                    planned_date: "2025-05-31".to_string(),
                    actual_date: Some("2025-05-28".to_string()),
                    status: MilestoneStatus::Completed,
                    responsible_party: Some("IT Security".to_string()),
                },
                TimelineMilestone {
                    id: "MS-007".to_string(),
                    name: "Cloud Migration Phase 1".to_string(),
                    description: Some("Migrate non-critical workloads".to_string()),
                    planned_date: "2025-08-31".to_string(),
                    actual_date: None,
                    status: MilestoneStatus::InProgress,
                    responsible_party: Some("IT Operations".to_string()),
                },
            ],
            approval_workflow: vec![
                PlanApprovalStep {
                    step: 1,
                    role: "CIO".to_string(),
                    approver: Some("Dr. Sarah Chen".to_string()),
                    status: "approved".to_string(),
                    date: Some("2025-02-20".to_string()),
                    comments: Some("Aligned with IT strategy".to_string()),
                },
                PlanApprovalStep {
                    step: 2,
                    role: "CFO".to_string(),
                    approver: Some("Nomvula Zulu".to_string()),
                    status: "approved".to_string(),
                    date: Some("2025-02-25".to_string()),
                    comments: None,
                },
            ],
            owner: "Peter Mokwena".to_string(),
            owner_email: "peter.mokwena@gov.za".to_string(),
            created_at: "2025-01-15T08:00:00Z".to_string(),
            updated_at: "2025-07-10T11:00:00Z".to_string(),
            approved_at: Some("2025-02-25T14:00:00Z".to_string()),
            start_date: "2025-04-01".to_string(),
            end_date: "2026-03-31".to_string(),
            strategic_objectives: vec![
                "Migrate 70% of workloads to cloud by end of FY".to_string(),
                "Achieve SOC 2 compliance for all systems".to_string(),
                "Reduce infrastructure costs by 20%".to_string(),
            ],
            risk_assessment: Some("Data security and vendor lock-in are primary concerns. Multi-cloud strategy recommended.".to_string()),
            notes: None,
        },
        SourcingPlan {
            id: "SP-2025-003".to_string(),
            title: "Infrastructure Maintenance Plan 2025/26".to_string(),
            description: "Annual plan for building maintenance, repairs, and facility upgrades across all government properties.".to_string(),
            fiscal_year: "2025/26".to_string(),
            department: "Facilities".to_string(),
            status: SourcingPlanStatus::Approved,
            categories: vec![
                ProcurementCategory {
                    id: "CAT-008".to_string(),
                    name: "Building Maintenance".to_string(),
                    code: "FAC-BLD".to_string(),
                    description: Some("Routine and preventive maintenance".to_string()),
                    allocated_budget: 18_000_000.0,
                    spent_amount: 0.0,
                    planned_tenders: 6,
                    completed_tenders: 0,
                    priority: CategoryPriority::High,
                    bbbee_target: Some(50.0),
                    local_content_target: Some(80.0),
                },
                ProcurementCategory {
                    id: "CAT-009".to_string(),
                    name: "HVAC Systems".to_string(),
                    code: "FAC-HVC".to_string(),
                    description: Some("Heating, ventilation, and air conditioning".to_string()),
                    allocated_budget: 8_500_000.0,
                    spent_amount: 0.0,
                    planned_tenders: 3,
                    completed_tenders: 0,
                    priority: CategoryPriority::Medium,
                    bbbee_target: Some(45.0),
                    local_content_target: Some(60.0),
                },
            ],
            budget: BudgetAllocation {
                total_budget: 30_000_000.0,
                allocated_amount: 26_500_000.0,
                committed_amount: 0.0,
                spent_amount: 0.0,
                currency: "ZAR".to_string(),
                fiscal_year_start: "2025-04-01".to_string(),
                fiscal_year_end: "2026-03-31".to_string(),
            },
            timeline: vec![
                TimelineMilestone {
                    id: "MS-008".to_string(),
                    name: "Tender Specifications".to_string(),
                    description: Some("Complete all tender specifications".to_string()),
                    planned_date: "2025-04-15".to_string(),
                    actual_date: None,
                    status: MilestoneStatus::Pending,
                    responsible_party: Some("Facilities Team".to_string()),
                },
            ],
            approval_workflow: vec![
                PlanApprovalStep {
                    step: 1,
                    role: "Facilities Manager".to_string(),
                    approver: Some("Grace Nkosi".to_string()),
                    status: "approved".to_string(),
                    date: Some("2025-03-20".to_string()),
                    comments: None,
                },
                PlanApprovalStep {
                    step: 2,
                    role: "CFO".to_string(),
                    approver: Some("Nomvula Zulu".to_string()),
                    status: "approved".to_string(),
                    date: Some("2025-03-25".to_string()),
                    comments: Some("Proceed with Q1 priorities".to_string()),
                },
            ],
            owner: "Grace Nkosi".to_string(),
            owner_email: "grace.nkosi@gov.za".to_string(),
            created_at: "2025-03-01T09:00:00Z".to_string(),
            updated_at: "2025-03-25T16:00:00Z".to_string(),
            approved_at: Some("2025-03-25T16:00:00Z".to_string()),
            start_date: "2025-04-01".to_string(),
            end_date: "2026-03-31".to_string(),
            strategic_objectives: vec![
                "Maintain all buildings to Grade A standard".to_string(),
                "Reduce energy consumption by 10%".to_string(),
                "Prioritize local SMME contractors".to_string(),
            ],
            risk_assessment: None,
            notes: Some("Pending final budget allocation confirmation".to_string()),
        },
        SourcingPlan {
            id: "SP-2025-004".to_string(),
            title: "Fleet Renewal Programme".to_string(),
            description: "Multi-year vehicle replacement and fleet modernization plan.".to_string(),
            fiscal_year: "2025/26".to_string(),
            department: "Transport".to_string(),
            status: SourcingPlanStatus::UnderReview,
            categories: vec![
                ProcurementCategory {
                    id: "CAT-010".to_string(),
                    name: "Vehicle Procurement".to_string(),
                    code: "FLT-VEH".to_string(),
                    description: Some("New vehicle purchases".to_string()),
                    allocated_budget: 25_000_000.0,
                    spent_amount: 0.0,
                    planned_tenders: 3,
                    completed_tenders: 0,
                    priority: CategoryPriority::High,
                    bbbee_target: Some(30.0),
                    local_content_target: Some(65.0),
                },
            ],
            budget: BudgetAllocation {
                total_budget: 35_000_000.0,
                allocated_amount: 25_000_000.0,
                committed_amount: 0.0,
                spent_amount: 0.0,
                currency: "ZAR".to_string(),
                fiscal_year_start: "2025-04-01".to_string(),
                fiscal_year_end: "2026-03-31".to_string(),
            },
            timeline: vec![],
            approval_workflow: vec![
                PlanApprovalStep {
                    step: 1,
                    role: "Transport Manager".to_string(),
                    approver: Some("Samuel Mthembu".to_string()),
                    status: "approved".to_string(),
                    date: Some("2025-03-10".to_string()),
                    comments: None,
                },
                PlanApprovalStep {
                    step: 2,
                    role: "CFO".to_string(),
                    approver: None,
                    status: "pending".to_string(),
                    date: None,
                    comments: None,
                },
            ],
            owner: "Samuel Mthembu".to_string(),
            owner_email: "samuel.mthembu@gov.za".to_string(),
            created_at: "2025-03-05T11:00:00Z".to_string(),
            updated_at: "2025-03-10T14:00:00Z".to_string(),
            approved_at: None,
            start_date: "2025-04-01".to_string(),
            end_date: "2026-03-31".to_string(),
            strategic_objectives: vec![
                "Replace 50 vehicles older than 10 years".to_string(),
                "Introduce 10 electric vehicles".to_string(),
            ],
            risk_assessment: Some("Supply chain delays possible due to global vehicle shortages.".to_string()),
            notes: None,
        },
        SourcingPlan {
            id: "SP-2025-005".to_string(),
            title: "Training & Development Programme".to_string(),
            description: "Annual staff training and capacity building procurement plan.".to_string(),
            fiscal_year: "2025/26".to_string(),
            department: "Human Resources".to_string(),
            status: SourcingPlanStatus::Draft,
            categories: vec![
                ProcurementCategory {
                    id: "CAT-011".to_string(),
                    name: "Leadership Training".to_string(),
                    code: "TRN-LDR".to_string(),
                    description: Some("Executive and management development".to_string()),
                    allocated_budget: 3_500_000.0,
                    spent_amount: 0.0,
                    planned_tenders: 2,
                    completed_tenders: 0,
                    priority: CategoryPriority::Medium,
                    bbbee_target: Some(60.0),
                    local_content_target: Some(90.0),
                },
                ProcurementCategory {
                    id: "CAT-012".to_string(),
                    name: "Technical Skills".to_string(),
                    code: "TRN-TEC".to_string(),
                    description: Some("IT, finance, and specialized training".to_string()),
                    allocated_budget: 4_500_000.0,
                    spent_amount: 0.0,
                    planned_tenders: 4,
                    completed_tenders: 0,
                    priority: CategoryPriority::High,
                    bbbee_target: Some(50.0),
                    local_content_target: Some(85.0),
                },
            ],
            budget: BudgetAllocation {
                total_budget: 10_000_000.0,
                allocated_amount: 8_000_000.0,
                committed_amount: 0.0,
                spent_amount: 0.0,
                currency: "ZAR".to_string(),
                fiscal_year_start: "2025-04-01".to_string(),
                fiscal_year_end: "2026-03-31".to_string(),
            },
            timeline: vec![],
            approval_workflow: vec![],
            owner: "Patricia Venter".to_string(),
            owner_email: "patricia.venter@gov.za".to_string(),
            created_at: "2025-03-15T10:00:00Z".to_string(),
            updated_at: "2025-03-15T10:00:00Z".to_string(),
            approved_at: None,
            start_date: "2025-04-01".to_string(),
            end_date: "2026-03-31".to_string(),
            strategic_objectives: vec![
                "Train 500 staff members in digital skills".to_string(),
                "Complete leadership programme for 50 managers".to_string(),
            ],
            risk_assessment: None,
            notes: Some("Draft - awaiting training needs analysis".to_string()),
        },
        SourcingPlan {
            id: "SP-2024-001".to_string(),
            title: "Annual Procurement Plan FY 2024/25".to_string(),
            description: "Previous fiscal year comprehensive procurement plan.".to_string(),
            fiscal_year: "2024/25".to_string(),
            department: "Procurement Division".to_string(),
            status: SourcingPlanStatus::Completed,
            categories: vec![
                ProcurementCategory {
                    id: "CAT-013".to_string(),
                    name: "General Procurement".to_string(),
                    code: "GEN-PRO".to_string(),
                    description: Some("All categories".to_string()),
                    allocated_budget: 95_000_000.0,
                    spent_amount: 92_500_000.0,
                    planned_tenders: 45,
                    completed_tenders: 43,
                    priority: CategoryPriority::High,
                    bbbee_target: Some(35.0),
                    local_content_target: Some(55.0),
                },
            ],
            budget: BudgetAllocation {
                total_budget: 100_000_000.0,
                allocated_amount: 95_000_000.0,
                committed_amount: 0.0,
                spent_amount: 92_500_000.0,
                currency: "ZAR".to_string(),
                fiscal_year_start: "2024-04-01".to_string(),
                fiscal_year_end: "2025-03-31".to_string(),
            },
            timeline: vec![],
            approval_workflow: vec![],
            owner: "Sipho Dlamini".to_string(),
            owner_email: "sipho.dlamini@gov.za".to_string(),
            created_at: "2024-02-01T10:00:00Z".to_string(),
            updated_at: "2025-03-31T23:59:59Z".to_string(),
            approved_at: Some("2024-03-15T09:00:00Z".to_string()),
            start_date: "2024-04-01".to_string(),
            end_date: "2025-03-31".to_string(),
            strategic_objectives: vec![
                "97% budget utilization achieved".to_string(),
                "95% tender completion rate".to_string(),
            ],
            risk_assessment: None,
            notes: Some("Closed - Final report submitted to Treasury".to_string()),
        },
    ];

    // Update pagination
    let mut pagination = store.pagination.get();
    pagination.update_totals(mock_plans.len() as u32);
    store.pagination.set(pagination);

    store.plans.set(mock_plans);
}

/// Get filtered sourcing plans
pub fn get_filtered_plans(store: &SourcingPlanStore) -> Vec<SourcingPlan> {
    let filter = store.filter.get();
    let all = store.plans.get();

    all.iter()
        .filter(|p| {
            // Search filter
            if !filter.search.is_empty() {
                let search = filter.search.to_lowercase();
                if !p.id.to_lowercase().contains(&search)
                    && !p.title.to_lowercase().contains(&search)
                    && !p.department.to_lowercase().contains(&search)
                    && !p.owner.to_lowercase().contains(&search)
                {
                    return false;
                }
            }

            // Status filter
            if let Some(status) = &filter.status {
                if p.status != *status {
                    return false;
                }
            }

            // Fiscal year filter
            if let Some(fy) = &filter.fiscal_year {
                if !fy.is_empty() && p.fiscal_year != *fy {
                    return false;
                }
            }

            // Department filter
            if let Some(dept) = &filter.department {
                if !dept.is_empty() && p.department != *dept {
                    return false;
                }
            }

            // Budget range filter
            if let Some(min) = filter.min_budget {
                if p.budget.total_budget < min {
                    return false;
                }
            }

            if let Some(max) = filter.max_budget {
                if p.budget.total_budget > max {
                    return false;
                }
            }

            true
        })
        .cloned()
        .collect()
}

/// Get paginated sourcing plans
pub fn get_paginated_plans(store: &SourcingPlanStore) -> Vec<SourcingPlan> {
    let filtered = get_filtered_plans(store);
    let pagination = store.pagination.get();

    let start = ((pagination.current_page - 1) * pagination.page_size) as usize;
    let end = (start + pagination.page_size as usize).min(filtered.len());

    if start >= filtered.len() {
        Vec::new()
    } else {
        filtered[start..end].to_vec()
    }
}
