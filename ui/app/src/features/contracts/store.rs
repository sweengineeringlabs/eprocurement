//! Contracts store

use components::prelude::*;
use super::types::{Contract, ContractSummary, ContractFilter, ContractStatus, ContractMilestone, MilestoneStatus, ContractTerms, ContractSla, ContractDeliverable};

/// Contracts state store
#[derive(Clone)]
pub struct ContractsStore {
    pub contracts: Signal<Vec<ContractSummary>>,
    pub selected: Signal<Option<Contract>>,
    pub filter: Signal<ContractFilter>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
    pub saving: Signal<bool>,
}

impl ContractsStore {
    pub fn new() -> Self {
        Self {
            contracts: signal(Vec::new()),
            selected: signal(None),
            filter: signal(ContractFilter::default()),
            loading: signal(false),
            error: signal(None),
            saving: signal(false),
        }
    }

    /// Update filter status
    pub fn set_filter_status(&self, status: Option<ContractStatus>) {
        let mut filter = self.filter.get();
        filter.status = status;
        self.filter.set(filter);
    }

    /// Update filter supplier
    pub fn set_filter_supplier(&self, supplier_id: Option<String>) {
        let mut filter = self.filter.get();
        filter.supplier_id = supplier_id;
        self.filter.set(filter);
    }

    /// Update filter search
    pub fn set_filter_search(&self, search: Option<String>) {
        let mut filter = self.filter.get();
        filter.search = search;
        self.filter.set(filter);
    }

    /// Update filter for expiring contracts
    pub fn set_filter_expiring(&self, days: Option<u32>) {
        let mut filter = self.filter.get();
        filter.expiring_within_days = days;
        self.filter.set(filter);
    }

    /// Clear all filters
    pub fn clear_filters(&self) {
        self.filter.set(ContractFilter::default());
    }

    /// Get filtered contracts
    pub fn get_filtered_contracts(&self) -> Vec<ContractSummary> {
        let contracts = self.contracts.get();
        let filter = self.filter.get();

        contracts
            .iter()
            .filter(|c| {
                // Filter by status
                if let Some(status) = &filter.status {
                    if c.status != *status {
                        return false;
                    }
                }

                // Filter by supplier
                if let Some(supplier_id) = &filter.supplier_id {
                    if !supplier_id.is_empty() && c.supplier_name.to_lowercase().contains(&supplier_id.to_lowercase()) {
                        // Continue
                    } else if !supplier_id.is_empty() {
                        return false;
                    }
                }

                // Filter by search
                if let Some(search) = &filter.search {
                    if !search.is_empty() {
                        let search_lower = search.to_lowercase();
                        if !c.title.to_lowercase().contains(&search_lower)
                            && !c.supplier_name.to_lowercase().contains(&search_lower)
                            && !c.id.to_lowercase().contains(&search_lower)
                        {
                            return false;
                        }
                    }
                }

                // Filter by expiring within days
                if let Some(days) = filter.expiring_within_days {
                    if let Some(days_to_expiry) = c.days_to_expiry {
                        if days_to_expiry > days as i32 || days_to_expiry < 0 {
                            return false;
                        }
                    }
                }

                true
            })
            .cloned()
            .collect()
    }
}

/// Load mock contracts data for demo
pub fn load_mock_contracts(store: &ContractsStore) {
    let contracts = vec![
        ContractSummary {
            id: "CTR-2025-0234".to_string(),
            title: "IT Infrastructure Support Services".to_string(),
            supplier_name: "TechSolutions SA (Pty) Ltd".to_string(),
            supplier_bbbee_level: 1,
            value: 12_500_000.0,
            start_date: "2025-01-01".to_string(),
            end_date: "2027-12-31".to_string(),
            status: ContractStatus::Active,
            days_to_expiry: Some(1038),
            milestone_progress: 33.3,
        },
        ContractSummary {
            id: "CTR-2025-0198".to_string(),
            title: "Office Supplies Framework Agreement".to_string(),
            supplier_name: "Office Pro Distributors".to_string(),
            supplier_bbbee_level: 2,
            value: 3_200_000.0,
            start_date: "2025-02-01".to_string(),
            end_date: "2026-01-31".to_string(),
            status: ContractStatus::Active,
            days_to_expiry: Some(340),
            milestone_progress: 50.0,
        },
        ContractSummary {
            id: "CTR-2025-0156".to_string(),
            title: "Security Services - Head Office".to_string(),
            supplier_name: "SecureGuard Holdings".to_string(),
            supplier_bbbee_level: 1,
            value: 8_750_000.0,
            start_date: "2024-04-01".to_string(),
            end_date: "2025-03-31".to_string(),
            status: ContractStatus::Active,
            days_to_expiry: Some(33),
            milestone_progress: 91.7,
        },
        ContractSummary {
            id: "CTR-2025-0089".to_string(),
            title: "Fleet Maintenance Services".to_string(),
            supplier_name: "AutoCare Fleet Management".to_string(),
            supplier_bbbee_level: 3,
            value: 4_500_000.0,
            start_date: "2024-07-01".to_string(),
            end_date: "2025-06-30".to_string(),
            status: ContractStatus::Active,
            days_to_expiry: Some(124),
            milestone_progress: 75.0,
        },
        ContractSummary {
            id: "CTR-2024-0456".to_string(),
            title: "Catering Services - Annual".to_string(),
            supplier_name: "Gourmet Corporate Catering".to_string(),
            supplier_bbbee_level: 2,
            value: 1_800_000.0,
            start_date: "2024-01-01".to_string(),
            end_date: "2024-12-31".to_string(),
            status: ContractStatus::Expired,
            days_to_expiry: Some(-58),
            milestone_progress: 100.0,
        },
        ContractSummary {
            id: "CTR-2025-0301".to_string(),
            title: "Cloud Hosting Services".to_string(),
            supplier_name: "CloudFirst SA".to_string(),
            supplier_bbbee_level: 1,
            value: 6_200_000.0,
            start_date: "2025-03-01".to_string(),
            end_date: "2028-02-28".to_string(),
            status: ContractStatus::PendingApproval,
            days_to_expiry: None,
            milestone_progress: 0.0,
        },
        ContractSummary {
            id: "CTR-2025-0287".to_string(),
            title: "Professional Training Services".to_string(),
            supplier_name: "Skills Development Academy".to_string(),
            supplier_bbbee_level: 1,
            value: 2_400_000.0,
            start_date: "2025-04-01".to_string(),
            end_date: "2026-03-31".to_string(),
            status: ContractStatus::Draft,
            days_to_expiry: None,
            milestone_progress: 0.0,
        },
        ContractSummary {
            id: "CTR-2024-0123".to_string(),
            title: "Cleaning Services - Regional Offices".to_string(),
            supplier_name: "CleanCorp Services".to_string(),
            supplier_bbbee_level: 2,
            value: 2_100_000.0,
            start_date: "2024-03-01".to_string(),
            end_date: "2025-02-28".to_string(),
            status: ContractStatus::Active,
            days_to_expiry: Some(2),
            milestone_progress: 100.0,
        },
    ];

    store.contracts.set(contracts);
}

/// Load mock contract details
pub fn get_mock_contract(id: &str) -> Option<Contract> {
    match id {
        "CTR-2025-0234" => Some(Contract {
            id: "CTR-2025-0234".to_string(),
            title: "IT Infrastructure Support Services".to_string(),
            description: "Comprehensive IT infrastructure support including hardware maintenance, software support, network management, and 24/7 help desk services.".to_string(),
            supplier_id: "SUP-001".to_string(),
            supplier_name: "TechSolutions SA (Pty) Ltd".to_string(),
            supplier_bbbee_level: 1,
            value: 12_500_000.0,
            start_date: "2025-01-01".to_string(),
            end_date: "2027-12-31".to_string(),
            status: ContractStatus::Active,
            contract_type: "Services".to_string(),
            reference_number: "PFMA/2024/IT/089".to_string(),
            tender_id: Some("TND-2024-0089".to_string()),
            purchase_order_id: Some("PO-2025-0234".to_string()),
            terms: ContractTerms {
                payment_terms: "Monthly in arrears, 30 days from invoice".to_string(),
                warranty_period_months: 12,
                notice_period_days: 90,
                renewal_terms: "Automatic renewal for 12 months unless terminated with 90 days notice".to_string(),
                termination_clause: "Either party may terminate with 90 days written notice. Immediate termination for material breach.".to_string(),
                dispute_resolution: "Mediation followed by arbitration in accordance with AFSA rules".to_string(),
                governing_law: "Laws of the Republic of South Africa".to_string(),
                special_conditions: vec![
                    "Supplier must maintain B-BBEE Level 1 certification".to_string(),
                    "All support staff must have security clearance".to_string(),
                    "Quarterly performance reviews required".to_string(),
                ],
            },
            sla: Some(ContractSla {
                response_time_hours: 2,
                resolution_time_hours: 8,
                availability_percent: 99.9,
                penalty_clause: "1% of monthly fee per hour SLA breach, capped at 10% monthly".to_string(),
                escalation_procedure: "Level 1: Service Desk -> Level 2: Technical Lead (2hrs) -> Level 3: Account Manager (4hrs) -> Level 4: Executive (8hrs)".to_string(),
            }),
            deliverables: vec![
                ContractDeliverable {
                    id: "DEL-001".to_string(),
                    description: "24/7 Help Desk Support".to_string(),
                    quantity: 12,
                    unit: "Months".to_string(),
                    unit_price: 450_000.0,
                    total_price: 5_400_000.0,
                    delivery_date: "2025-12-31".to_string(),
                    delivered: false,
                },
                ContractDeliverable {
                    id: "DEL-002".to_string(),
                    description: "Network Infrastructure Management".to_string(),
                    quantity: 12,
                    unit: "Months".to_string(),
                    unit_price: 350_000.0,
                    total_price: 4_200_000.0,
                    delivery_date: "2025-12-31".to_string(),
                    delivered: false,
                },
                ContractDeliverable {
                    id: "DEL-003".to_string(),
                    description: "Hardware Maintenance & Support".to_string(),
                    quantity: 12,
                    unit: "Months".to_string(),
                    unit_price: 241_667.0,
                    total_price: 2_900_000.0,
                    delivery_date: "2025-12-31".to_string(),
                    delivered: false,
                },
            ],
            milestones: vec![
                ContractMilestone {
                    id: "MS-001".to_string(),
                    contract_id: "CTR-2025-0234".to_string(),
                    title: "Project Initiation & Handover".to_string(),
                    description: "Complete project initiation, knowledge transfer, and systems handover from previous vendor".to_string(),
                    due_date: "2025-01-31".to_string(),
                    completed_date: Some("2025-01-28".to_string()),
                    payment_amount: 1_250_000.0,
                    payment_percentage: 10.0,
                    status: MilestoneStatus::Completed,
                    deliverables: vec![
                        "Project charter signed".to_string(),
                        "Knowledge transfer complete".to_string(),
                        "Systems access configured".to_string(),
                    ],
                    notes: Some("Completed ahead of schedule".to_string()),
                },
                ContractMilestone {
                    id: "MS-002".to_string(),
                    contract_id: "CTR-2025-0234".to_string(),
                    title: "Q1 Service Delivery".to_string(),
                    description: "First quarter service delivery with full SLA compliance".to_string(),
                    due_date: "2025-03-31".to_string(),
                    completed_date: None,
                    payment_amount: 3_125_000.0,
                    payment_percentage: 25.0,
                    status: MilestoneStatus::InProgress,
                    deliverables: vec![
                        "Monthly service reports".to_string(),
                        "SLA compliance >99.9%".to_string(),
                        "Quarterly review meeting".to_string(),
                    ],
                    notes: None,
                },
                ContractMilestone {
                    id: "MS-003".to_string(),
                    contract_id: "CTR-2025-0234".to_string(),
                    title: "Q2 Service Delivery".to_string(),
                    description: "Second quarter service delivery with continuous improvement".to_string(),
                    due_date: "2025-06-30".to_string(),
                    completed_date: None,
                    payment_amount: 3_125_000.0,
                    payment_percentage: 25.0,
                    status: MilestoneStatus::Pending,
                    deliverables: vec![
                        "Monthly service reports".to_string(),
                        "SLA compliance >99.9%".to_string(),
                        "Improvement initiatives report".to_string(),
                    ],
                    notes: None,
                },
                ContractMilestone {
                    id: "MS-004".to_string(),
                    contract_id: "CTR-2025-0234".to_string(),
                    title: "Year 1 Completion".to_string(),
                    description: "Complete first year of service delivery with annual review".to_string(),
                    due_date: "2025-12-31".to_string(),
                    completed_date: None,
                    payment_amount: 5_000_000.0,
                    payment_percentage: 40.0,
                    status: MilestoneStatus::Pending,
                    deliverables: vec![
                        "Annual performance report".to_string(),
                        "Audit compliance certificate".to_string(),
                        "Contract renewal recommendation".to_string(),
                    ],
                    notes: None,
                },
            ],
            documents: vec![
                "contract_signed.pdf".to_string(),
                "sla_document.pdf".to_string(),
                "tender_award_letter.pdf".to_string(),
                "insurance_certificate.pdf".to_string(),
                "bbbee_certificate.pdf".to_string(),
            ],
            created_by: "John Smith".to_string(),
            created_at: "2024-12-15T10:30:00Z".to_string(),
            updated_at: "2025-02-26T14:22:00Z".to_string(),
            approved_by: Some("Sarah Johnson".to_string()),
            approved_at: Some("2024-12-20T09:15:00Z".to_string()),
        }),
        "CTR-2025-0156" => Some(Contract {
            id: "CTR-2025-0156".to_string(),
            title: "Security Services - Head Office".to_string(),
            description: "Physical security services for head office premises including access control, CCTV monitoring, and patrol services.".to_string(),
            supplier_id: "SUP-003".to_string(),
            supplier_name: "SecureGuard Holdings".to_string(),
            supplier_bbbee_level: 1,
            value: 8_750_000.0,
            start_date: "2024-04-01".to_string(),
            end_date: "2025-03-31".to_string(),
            status: ContractStatus::Active,
            contract_type: "Services".to_string(),
            reference_number: "PFMA/2024/SEC/056".to_string(),
            tender_id: Some("TND-2024-0056".to_string()),
            purchase_order_id: Some("PO-2024-0156".to_string()),
            terms: ContractTerms::default(),
            sla: Some(ContractSla {
                response_time_hours: 1,
                resolution_time_hours: 4,
                availability_percent: 99.99,
                penalty_clause: "Deduction of daily fee for any security incident due to negligence".to_string(),
                escalation_procedure: "Site Supervisor -> Area Manager -> Regional Director -> CEO".to_string(),
            }),
            deliverables: Vec::new(),
            milestones: vec![
                ContractMilestone {
                    id: "MS-001".to_string(),
                    contract_id: "CTR-2025-0156".to_string(),
                    title: "Q1 Security Services".to_string(),
                    description: "First quarter security services delivery".to_string(),
                    due_date: "2024-06-30".to_string(),
                    completed_date: Some("2024-06-30".to_string()),
                    payment_amount: 2_187_500.0,
                    payment_percentage: 25.0,
                    status: MilestoneStatus::Completed,
                    deliverables: Vec::new(),
                    notes: None,
                },
                ContractMilestone {
                    id: "MS-002".to_string(),
                    contract_id: "CTR-2025-0156".to_string(),
                    title: "Q2 Security Services".to_string(),
                    description: "Second quarter security services delivery".to_string(),
                    due_date: "2024-09-30".to_string(),
                    completed_date: Some("2024-09-30".to_string()),
                    payment_amount: 2_187_500.0,
                    payment_percentage: 25.0,
                    status: MilestoneStatus::Completed,
                    deliverables: Vec::new(),
                    notes: None,
                },
                ContractMilestone {
                    id: "MS-003".to_string(),
                    contract_id: "CTR-2025-0156".to_string(),
                    title: "Q3 Security Services".to_string(),
                    description: "Third quarter security services delivery".to_string(),
                    due_date: "2024-12-31".to_string(),
                    completed_date: Some("2024-12-31".to_string()),
                    payment_amount: 2_187_500.0,
                    payment_percentage: 25.0,
                    status: MilestoneStatus::Completed,
                    deliverables: Vec::new(),
                    notes: None,
                },
                ContractMilestone {
                    id: "MS-004".to_string(),
                    contract_id: "CTR-2025-0156".to_string(),
                    title: "Q4 Security Services".to_string(),
                    description: "Fourth quarter and contract completion".to_string(),
                    due_date: "2025-03-31".to_string(),
                    completed_date: None,
                    payment_amount: 2_187_500.0,
                    payment_percentage: 25.0,
                    status: MilestoneStatus::InProgress,
                    deliverables: Vec::new(),
                    notes: Some("Contract expiring - renewal process initiated".to_string()),
                },
            ],
            documents: Vec::new(),
            created_by: "Jane Doe".to_string(),
            created_at: "2024-03-01T08:00:00Z".to_string(),
            updated_at: "2025-02-25T16:45:00Z".to_string(),
            approved_by: Some("Mike Wilson".to_string()),
            approved_at: Some("2024-03-15T11:30:00Z".to_string()),
        }),
        _ => None,
    }
}
