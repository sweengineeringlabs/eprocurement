//! Requisitions store

use components::prelude::*;
use super::types::{
    Requisition, RequisitionStatus, RequisitionFilter, PaginationState,
    LineItem, Attachment, ApprovalStep, Priority,
};

/// Requisitions state store
#[derive(Clone)]
pub struct RequisitionsStore {
    pub requisitions: Signal<Vec<Requisition>>,
    pub selected: Signal<Option<Requisition>>,
    pub filter: Signal<RequisitionFilter>,
    pub pagination: Signal<PaginationState>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
    pub form_step: Signal<u32>,
    pub form_data: Signal<Requisition>,
}

impl RequisitionsStore {
    pub fn new() -> Self {
        Self {
            requisitions: signal(Vec::new()),
            selected: signal(None),
            filter: signal(RequisitionFilter::default()),
            pagination: signal(PaginationState::default()),
            loading: signal(false),
            error: signal(None),
            form_step: signal(1),
            form_data: signal(Requisition::default()),
        }
    }

    pub fn reset_form(&self) {
        self.form_step.set(1);
        self.form_data.set(Requisition::default());
    }

    pub fn set_selected(&self, id: &str) {
        let requisition = self.requisitions.get()
            .iter()
            .find(|r| r.id == id)
            .cloned();
        self.selected.set(requisition);
    }
}

/// Load mock data for demo
pub fn load_mock_data(store: &RequisitionsStore) {
    let mock_requisitions = vec![
        Requisition {
            id: "REQ-2025-0847".to_string(),
            description: "IT Hardware - Laptops Q4".to_string(),
            justification: "Replacement of outdated laptops for the IT department to improve productivity and support latest software requirements.".to_string(),
            amount: 2_450_000.0,
            status: RequisitionStatus::PendingApproval,
            priority: Priority::High,
            department: "Information Technology".to_string(),
            cost_center: "CC-IT-001".to_string(),
            requester: "John Ndlovu".to_string(),
            requester_email: "john.ndlovu@gov.za".to_string(),
            line_items: vec![
                LineItem {
                    id: "LI-001".to_string(),
                    description: "Dell Latitude 5540 Laptop".to_string(),
                    category: "IT Equipment".to_string(),
                    quantity: 50,
                    unit: "Each".to_string(),
                    unit_price: 28_500.0,
                    total: 1_425_000.0,
                    specifications: Some("Intel i7, 16GB RAM, 512GB SSD".to_string()),
                    catalogue_item_id: Some("CAT-IT-001".to_string()),
                },
                LineItem {
                    id: "LI-002".to_string(),
                    description: "Dell Docking Station WD19S".to_string(),
                    category: "IT Equipment".to_string(),
                    quantity: 50,
                    unit: "Each".to_string(),
                    unit_price: 4_500.0,
                    total: 225_000.0,
                    specifications: None,
                    catalogue_item_id: Some("CAT-IT-002".to_string()),
                },
                LineItem {
                    id: "LI-003".to_string(),
                    description: "Microsoft Office 365 E3 License (Annual)".to_string(),
                    category: "Software".to_string(),
                    quantity: 50,
                    unit: "License".to_string(),
                    unit_price: 16_000.0,
                    total: 800_000.0,
                    specifications: None,
                    catalogue_item_id: Some("CAT-SW-001".to_string()),
                },
            ],
            attachments: vec![
                Attachment {
                    id: "ATT-001".to_string(),
                    name: "IT_Hardware_Specifications.pdf".to_string(),
                    size: 2_450_000,
                    file_type: "application/pdf".to_string(),
                    uploaded_at: "2025-01-15T10:30:00Z".to_string(),
                },
                Attachment {
                    id: "ATT-002".to_string(),
                    name: "Budget_Approval_Q4.pdf".to_string(),
                    size: 890_000,
                    file_type: "application/pdf".to_string(),
                    uploaded_at: "2025-01-15T10:35:00Z".to_string(),
                },
            ],
            approval_workflow: vec![
                ApprovalStep {
                    step: 1,
                    role: "Line Manager".to_string(),
                    approver: Some("Sarah Dlamini".to_string()),
                    status: "approved".to_string(),
                    date: Some("2025-01-16T09:00:00Z".to_string()),
                    comments: Some("Approved - aligns with IT refresh strategy".to_string()),
                },
                ApprovalStep {
                    step: 2,
                    role: "Budget Holder".to_string(),
                    approver: Some("Peter Mokwena".to_string()),
                    status: "pending".to_string(),
                    date: None,
                    comments: None,
                },
                ApprovalStep {
                    step: 3,
                    role: "CFO".to_string(),
                    approver: None,
                    status: "pending".to_string(),
                    date: None,
                    comments: None,
                },
            ],
            created_at: "2025-01-15T10:00:00Z".to_string(),
            updated_at: "2025-01-16T09:00:00Z".to_string(),
            required_by: Some("2025-03-31".to_string()),
            delivery_address: "123 Government Building, Pretoria, 0001".to_string(),
            notes: Some("Urgent - required for new financial year".to_string()),
        },
        Requisition {
            id: "REQ-2025-0846".to_string(),
            description: "Office Supplies - Stationery Q1".to_string(),
            justification: "Quarterly stationery replenishment for all departments.".to_string(),
            amount: 125_000.0,
            status: RequisitionStatus::Approved,
            priority: Priority::Medium,
            department: "Administration".to_string(),
            cost_center: "CC-ADM-001".to_string(),
            requester: "Mary Khumalo".to_string(),
            requester_email: "mary.khumalo@gov.za".to_string(),
            line_items: vec![
                LineItem {
                    id: "LI-004".to_string(),
                    description: "A4 Paper (Box of 5 Reams)".to_string(),
                    category: "Office Supplies".to_string(),
                    quantity: 200,
                    unit: "Box".to_string(),
                    unit_price: 350.0,
                    total: 70_000.0,
                    specifications: None,
                    catalogue_item_id: Some("CAT-OS-001".to_string()),
                },
                LineItem {
                    id: "LI-005".to_string(),
                    description: "Assorted Stationery Pack".to_string(),
                    category: "Office Supplies".to_string(),
                    quantity: 100,
                    unit: "Pack".to_string(),
                    unit_price: 550.0,
                    total: 55_000.0,
                    specifications: Some("Pens, pencils, staplers, tape".to_string()),
                    catalogue_item_id: Some("CAT-OS-002".to_string()),
                },
            ],
            attachments: vec![],
            approval_workflow: vec![
                ApprovalStep {
                    step: 1,
                    role: "Line Manager".to_string(),
                    approver: Some("David Sithole".to_string()),
                    status: "approved".to_string(),
                    date: Some("2025-01-14T11:00:00Z".to_string()),
                    comments: None,
                },
            ],
            created_at: "2025-01-14T08:00:00Z".to_string(),
            updated_at: "2025-01-14T11:00:00Z".to_string(),
            required_by: Some("2025-02-15".to_string()),
            delivery_address: "456 Admin Block, Cape Town, 8001".to_string(),
            notes: None,
        },
        Requisition {
            id: "REQ-2025-0845".to_string(),
            description: "Security Services - Annual Contract".to_string(),
            justification: "Renewal of security services contract for all government buildings in Gauteng province.".to_string(),
            amount: 8_750_000.0,
            status: RequisitionStatus::InProgress,
            priority: Priority::High,
            department: "Facilities".to_string(),
            cost_center: "CC-FAC-001".to_string(),
            requester: "Thabo Mokoena".to_string(),
            requester_email: "thabo.mokoena@gov.za".to_string(),
            line_items: vec![
                LineItem {
                    id: "LI-006".to_string(),
                    description: "24/7 Security Guard Services".to_string(),
                    category: "Services".to_string(),
                    quantity: 12,
                    unit: "Month".to_string(),
                    unit_price: 650_000.0,
                    total: 7_800_000.0,
                    specifications: Some("Armed response, access control, CCTV monitoring".to_string()),
                    catalogue_item_id: None,
                },
                LineItem {
                    id: "LI-007".to_string(),
                    description: "Security Equipment Maintenance".to_string(),
                    category: "Services".to_string(),
                    quantity: 12,
                    unit: "Month".to_string(),
                    unit_price: 79_166.67,
                    total: 950_000.0,
                    specifications: None,
                    catalogue_item_id: None,
                },
            ],
            attachments: vec![
                Attachment {
                    id: "ATT-003".to_string(),
                    name: "Security_Assessment_Report.pdf".to_string(),
                    size: 5_200_000,
                    file_type: "application/pdf".to_string(),
                    uploaded_at: "2025-01-10T14:00:00Z".to_string(),
                },
            ],
            approval_workflow: vec![
                ApprovalStep {
                    step: 1,
                    role: "Line Manager".to_string(),
                    approver: Some("Grace Nkosi".to_string()),
                    status: "approved".to_string(),
                    date: Some("2025-01-11T09:00:00Z".to_string()),
                    comments: None,
                },
                ApprovalStep {
                    step: 2,
                    role: "Budget Holder".to_string(),
                    approver: Some("Peter Mokwena".to_string()),
                    status: "approved".to_string(),
                    date: Some("2025-01-12T10:00:00Z".to_string()),
                    comments: None,
                },
                ApprovalStep {
                    step: 3,
                    role: "CFO".to_string(),
                    approver: Some("Nomvula Zulu".to_string()),
                    status: "approved".to_string(),
                    date: Some("2025-01-13T15:00:00Z".to_string()),
                    comments: None,
                },
            ],
            created_at: "2025-01-10T10:00:00Z".to_string(),
            updated_at: "2025-01-13T15:00:00Z".to_string(),
            required_by: Some("2025-02-01".to_string()),
            delivery_address: "All Gauteng Government Buildings".to_string(),
            notes: Some("Existing contract expires 31 Jan 2025".to_string()),
        },
        Requisition {
            id: "REQ-2025-0844".to_string(),
            description: "Fleet Maintenance Services".to_string(),
            justification: "Annual vehicle maintenance and servicing for government fleet.".to_string(),
            amount: 1_200_000.0,
            status: RequisitionStatus::Approved,
            priority: Priority::Medium,
            department: "Transport".to_string(),
            cost_center: "CC-TRN-001".to_string(),
            requester: "Samuel Mthembu".to_string(),
            requester_email: "samuel.mthembu@gov.za".to_string(),
            line_items: vec![
                LineItem {
                    id: "LI-008".to_string(),
                    description: "Vehicle Servicing - Sedan".to_string(),
                    category: "Fleet Services".to_string(),
                    quantity: 50,
                    unit: "Service".to_string(),
                    unit_price: 8_500.0,
                    total: 425_000.0,
                    specifications: Some("Full service including oil change, filters, brake check".to_string()),
                    catalogue_item_id: None,
                },
                LineItem {
                    id: "LI-009".to_string(),
                    description: "Vehicle Servicing - SUV/4x4".to_string(),
                    category: "Fleet Services".to_string(),
                    quantity: 35,
                    unit: "Service".to_string(),
                    unit_price: 12_500.0,
                    total: 437_500.0,
                    specifications: Some("Full service including 4x4 drivetrain check".to_string()),
                    catalogue_item_id: None,
                },
                LineItem {
                    id: "LI-010".to_string(),
                    description: "Tyre Replacement".to_string(),
                    category: "Fleet Services".to_string(),
                    quantity: 150,
                    unit: "Tyre".to_string(),
                    unit_price: 2_250.0,
                    total: 337_500.0,
                    specifications: None,
                    catalogue_item_id: None,
                },
            ],
            attachments: vec![],
            approval_workflow: vec![
                ApprovalStep {
                    step: 1,
                    role: "Line Manager".to_string(),
                    approver: Some("Linda Mbeki".to_string()),
                    status: "approved".to_string(),
                    date: Some("2025-01-08T14:00:00Z".to_string()),
                    comments: None,
                },
                ApprovalStep {
                    step: 2,
                    role: "Budget Holder".to_string(),
                    approver: Some("James Botha".to_string()),
                    status: "approved".to_string(),
                    date: Some("2025-01-09T09:00:00Z".to_string()),
                    comments: None,
                },
            ],
            created_at: "2025-01-07T11:00:00Z".to_string(),
            updated_at: "2025-01-09T09:00:00Z".to_string(),
            required_by: Some("2025-01-31".to_string()),
            delivery_address: "Government Fleet Depot, Johannesburg".to_string(),
            notes: None,
        },
        Requisition {
            id: "REQ-2025-0843".to_string(),
            description: "Training - Staff Development Programme".to_string(),
            justification: "Leadership and management training for senior staff across departments.".to_string(),
            amount: 560_000.0,
            status: RequisitionStatus::Draft,
            priority: Priority::Low,
            department: "Human Resources".to_string(),
            cost_center: "CC-HR-001".to_string(),
            requester: "Patricia Venter".to_string(),
            requester_email: "patricia.venter@gov.za".to_string(),
            line_items: vec![
                LineItem {
                    id: "LI-011".to_string(),
                    description: "Leadership Development Programme".to_string(),
                    category: "Training".to_string(),
                    quantity: 20,
                    unit: "Participant".to_string(),
                    unit_price: 18_000.0,
                    total: 360_000.0,
                    specifications: Some("5-day intensive leadership course".to_string()),
                    catalogue_item_id: None,
                },
                LineItem {
                    id: "LI-012".to_string(),
                    description: "Project Management Certification".to_string(),
                    category: "Training".to_string(),
                    quantity: 10,
                    unit: "Participant".to_string(),
                    unit_price: 20_000.0,
                    total: 200_000.0,
                    specifications: Some("PMP certification training and exam".to_string()),
                    catalogue_item_id: None,
                },
            ],
            attachments: vec![],
            approval_workflow: vec![],
            created_at: "2025-01-17T09:00:00Z".to_string(),
            updated_at: "2025-01-17T09:00:00Z".to_string(),
            required_by: Some("2025-04-30".to_string()),
            delivery_address: "N/A - Virtual Training".to_string(),
            notes: Some("Awaiting training needs analysis completion".to_string()),
        },
        Requisition {
            id: "REQ-2025-0842".to_string(),
            description: "Medical Supplies - Clinic Replenishment".to_string(),
            justification: "Emergency replenishment of medical supplies for on-site clinic.".to_string(),
            amount: 85_000.0,
            status: RequisitionStatus::Submitted,
            priority: Priority::Urgent,
            department: "Health Services".to_string(),
            cost_center: "CC-HS-001".to_string(),
            requester: "Dr. Fatima Patel".to_string(),
            requester_email: "fatima.patel@gov.za".to_string(),
            line_items: vec![
                LineItem {
                    id: "LI-013".to_string(),
                    description: "First Aid Supplies Kit".to_string(),
                    category: "Medical".to_string(),
                    quantity: 50,
                    unit: "Kit".to_string(),
                    unit_price: 850.0,
                    total: 42_500.0,
                    specifications: Some("Comprehensive workplace first aid kit".to_string()),
                    catalogue_item_id: Some("CAT-MED-001".to_string()),
                },
                LineItem {
                    id: "LI-014".to_string(),
                    description: "PPE - Disposable Masks (Box)".to_string(),
                    category: "Medical".to_string(),
                    quantity: 100,
                    unit: "Box".to_string(),
                    unit_price: 250.0,
                    total: 25_000.0,
                    specifications: Some("50 masks per box".to_string()),
                    catalogue_item_id: Some("CAT-MED-002".to_string()),
                },
                LineItem {
                    id: "LI-015".to_string(),
                    description: "Hand Sanitizer (5L)".to_string(),
                    category: "Medical".to_string(),
                    quantity: 50,
                    unit: "Container".to_string(),
                    unit_price: 350.0,
                    total: 17_500.0,
                    specifications: Some("70% alcohol content".to_string()),
                    catalogue_item_id: Some("CAT-MED-003".to_string()),
                },
            ],
            attachments: vec![],
            approval_workflow: vec![
                ApprovalStep {
                    step: 1,
                    role: "Line Manager".to_string(),
                    approver: None,
                    status: "pending".to_string(),
                    date: None,
                    comments: None,
                },
            ],
            created_at: "2025-01-17T14:00:00Z".to_string(),
            updated_at: "2025-01-17T14:00:00Z".to_string(),
            required_by: Some("2025-01-25".to_string()),
            delivery_address: "Government Clinic, Building A, Pretoria".to_string(),
            notes: Some("URGENT - Current stock critically low".to_string()),
        },
        Requisition {
            id: "REQ-2025-0841".to_string(),
            description: "Furniture - New Office Setup".to_string(),
            justification: "Office furniture for new regional office in Durban.".to_string(),
            amount: 450_000.0,
            status: RequisitionStatus::Rejected,
            priority: Priority::Medium,
            department: "Facilities".to_string(),
            cost_center: "CC-FAC-002".to_string(),
            requester: "Michael van der Merwe".to_string(),
            requester_email: "michael.vdm@gov.za".to_string(),
            line_items: vec![
                LineItem {
                    id: "LI-016".to_string(),
                    description: "Executive Desk".to_string(),
                    category: "Furniture".to_string(),
                    quantity: 5,
                    unit: "Each".to_string(),
                    unit_price: 25_000.0,
                    total: 125_000.0,
                    specifications: None,
                    catalogue_item_id: None,
                },
                LineItem {
                    id: "LI-017".to_string(),
                    description: "Workstation Desk".to_string(),
                    category: "Furniture".to_string(),
                    quantity: 25,
                    unit: "Each".to_string(),
                    unit_price: 8_500.0,
                    total: 212_500.0,
                    specifications: None,
                    catalogue_item_id: None,
                },
                LineItem {
                    id: "LI-018".to_string(),
                    description: "Ergonomic Office Chair".to_string(),
                    category: "Furniture".to_string(),
                    quantity: 30,
                    unit: "Each".to_string(),
                    unit_price: 3_750.0,
                    total: 112_500.0,
                    specifications: None,
                    catalogue_item_id: None,
                },
            ],
            attachments: vec![],
            approval_workflow: vec![
                ApprovalStep {
                    step: 1,
                    role: "Line Manager".to_string(),
                    approver: Some("Grace Nkosi".to_string()),
                    status: "approved".to_string(),
                    date: Some("2025-01-05T10:00:00Z".to_string()),
                    comments: None,
                },
                ApprovalStep {
                    step: 2,
                    role: "Budget Holder".to_string(),
                    approver: Some("Peter Mokwena".to_string()),
                    status: "rejected".to_string(),
                    date: Some("2025-01-06T15:00:00Z".to_string()),
                    comments: Some("Budget not available for Q1. Please resubmit for Q2.".to_string()),
                },
            ],
            created_at: "2025-01-04T09:00:00Z".to_string(),
            updated_at: "2025-01-06T15:00:00Z".to_string(),
            required_by: Some("2025-02-28".to_string()),
            delivery_address: "New Regional Office, Durban, 4001".to_string(),
            notes: None,
        },
    ];

    // Update pagination
    let mut pagination = store.pagination.get();
    pagination.update_totals(mock_requisitions.len() as u32);
    store.pagination.set(pagination);

    store.requisitions.set(mock_requisitions);
}

/// Get filtered requisitions
pub fn get_filtered_requisitions(store: &RequisitionsStore) -> Vec<Requisition> {
    let filter = store.filter.get();
    let all = store.requisitions.get();

    all.iter()
        .filter(|r| {
            // Search filter
            if !filter.search.is_empty() {
                let search = filter.search.to_lowercase();
                if !r.id.to_lowercase().contains(&search)
                    && !r.description.to_lowercase().contains(&search)
                    && !r.requester.to_lowercase().contains(&search)
                    && !r.department.to_lowercase().contains(&search)
                {
                    return false;
                }
            }

            // Status filter
            if let Some(status) = &filter.status {
                if r.status != *status {
                    return false;
                }
            }

            // Department filter
            if let Some(dept) = &filter.department {
                if !dept.is_empty() && r.department != *dept {
                    return false;
                }
            }

            true
        })
        .cloned()
        .collect()
}

/// Get paginated requisitions
pub fn get_paginated_requisitions(store: &RequisitionsStore) -> Vec<Requisition> {
    let filtered = get_filtered_requisitions(store);
    let pagination = store.pagination.get();

    let start = ((pagination.current_page - 1) * pagination.page_size) as usize;
    let end = (start + pagination.page_size as usize).min(filtered.len());

    if start >= filtered.len() {
        Vec::new()
    } else {
        filtered[start..end].to_vec()
    }
}
