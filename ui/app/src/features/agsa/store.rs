//! AGSA store

use components::prelude::*;
use super::types::{
    AuditFinding, AuditReport, ActionItem, ComplianceStatus, FindingStatus,
    FindingCategory, FindingSeverity, ActionPriority, ActionStatus, AuditType,
    AgsaKpis, AgsaFilter, AgsaComment, CommentType, ActionNote,
};

/// AGSA state store
#[derive(Clone)]
pub struct AgsaStore {
    pub findings: Signal<Vec<AuditFinding>>,
    pub selected_finding: Signal<Option<AuditFinding>>,
    pub audit_reports: Signal<Vec<AuditReport>>,
    pub selected_report: Signal<Option<AuditReport>>,
    pub action_items: Signal<Vec<ActionItem>>,
    pub kpis: Signal<AgsaKpis>,
    pub filter: Signal<AgsaFilter>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
}

impl AgsaStore {
    pub fn new() -> Self {
        Self {
            findings: signal(Vec::new()),
            selected_finding: signal(None),
            audit_reports: signal(Vec::new()),
            selected_report: signal(None),
            action_items: signal(Vec::new()),
            kpis: signal(AgsaKpis::default()),
            filter: signal(AgsaFilter::default()),
            loading: signal(false),
            error: signal(None),
        }
    }

    /// Select a finding for detailed view
    pub fn select_finding(&self, finding_id: &str) {
        let findings = self.findings.get();
        if let Some(finding) = findings.iter().find(|f| f.id == finding_id) {
            self.selected_finding.set(Some(finding.clone()));
        }
    }

    /// Clear finding selection
    pub fn clear_finding_selection(&self) {
        self.selected_finding.set(None);
    }

    /// Select an audit report for detailed view
    pub fn select_report(&self, report_id: &str) {
        let reports = self.audit_reports.get();
        if let Some(report) = reports.iter().find(|r| r.id == report_id) {
            self.selected_report.set(Some(report.clone()));
        }
    }

    /// Clear report selection
    pub fn clear_report_selection(&self) {
        self.selected_report.set(None);
    }

    /// Update status filter
    pub fn set_status_filter(&self, status: Option<FindingStatus>) {
        let mut filter = self.filter.get().clone();
        filter.status = status;
        self.filter.set(filter);
    }

    /// Update category filter
    pub fn set_category_filter(&self, category: Option<FindingCategory>) {
        let mut filter = self.filter.get().clone();
        filter.category = category;
        self.filter.set(filter);
    }

    /// Update severity filter
    pub fn set_severity_filter(&self, severity: Option<FindingSeverity>) {
        let mut filter = self.filter.get().clone();
        filter.severity = severity;
        self.filter.set(filter);
    }

    /// Get filtered findings
    pub fn filtered_findings(&self) -> Vec<AuditFinding> {
        let findings = self.findings.get();
        let filter = self.filter.get();

        findings.into_iter().filter(|finding| {
            // Status filter
            if let Some(ref status) = filter.status {
                if &finding.status != status {
                    return false;
                }
            }

            // Category filter
            if let Some(ref category) = filter.category {
                if &finding.category != category {
                    return false;
                }
            }

            // Severity filter
            if let Some(ref severity) = filter.severity {
                if &finding.severity != severity {
                    return false;
                }
            }

            // Repeat finding filter
            if let Some(is_repeat) = filter.is_repeat {
                if finding.is_repeat_finding != is_repeat {
                    return false;
                }
            }

            // Search query
            if let Some(ref query) = filter.search_query {
                let query_lower = query.to_lowercase();
                if !finding.title.to_lowercase().contains(&query_lower)
                    && !finding.reference_number.to_lowercase().contains(&query_lower)
                    && !finding.description.to_lowercase().contains(&query_lower)
                {
                    return false;
                }
            }

            true
        }).collect()
    }

    /// Get action items by finding
    pub fn action_items_for_finding(&self, finding_id: &str) -> Vec<ActionItem> {
        self.action_items.get().into_iter()
            .filter(|a| a.finding_id == finding_id)
            .collect()
    }

    /// Get overdue action items
    pub fn overdue_action_items(&self) -> Vec<ActionItem> {
        self.action_items.get().into_iter()
            .filter(|a| matches!(a.status, ActionStatus::Overdue))
            .collect()
    }

    /// Get repeat findings
    pub fn repeat_findings(&self) -> Vec<AuditFinding> {
        self.findings.get().into_iter()
            .filter(|f| f.is_repeat_finding)
            .collect()
    }

    /// Get findings by severity
    pub fn findings_by_severity(&self, severity: FindingSeverity) -> Vec<AuditFinding> {
        self.findings.get().into_iter()
            .filter(|f| f.severity == severity)
            .collect()
    }
}

/// Load mock data for demo
pub fn load_mock_data(store: &AgsaStore) {
    // Audit reports
    let mock_reports = vec![
        AuditReport {
            id: "rpt_001".to_string(),
            reference_number: "AGSA/2024-25/001".to_string(),
            financial_year: "2024/25".to_string(),
            audit_type: AuditType::Regularity,
            title: "Annual Financial Statements Audit 2024/25".to_string(),
            entity_name: "Department of Public Works".to_string(),
            audit_period_start: "2024-04-01".to_string(),
            audit_period_end: "2025-03-31".to_string(),
            report_date: "2025-07-31".to_string(),
            compliance_status: ComplianceStatus::UnqualifiedWithFindings,
            overall_opinion: "The financial statements present fairly, in all material respects, the financial position of the department as at 31 March 2025, and its financial performance and cash flows for the year then ended in accordance with the Modified Cash Standard of Generally Recognised Accounting Practice (GRAP) and the requirements of the Public Finance Management Act of South Africa, 1999 (Act No.1 of 1999) (PFMA).".to_string(),
            key_findings_summary: "Material findings were identified in the areas of supply chain management compliance, performance information reliability, and asset management. 12 findings require management attention.".to_string(),
            total_findings: 12,
            material_findings: 2,
            significant_findings: 5,
            minor_findings: 5,
            repeat_findings: 4,
            resolved_findings: 0,
            lead_auditor: "Ms. Thembi Khoza".to_string(),
            audit_team: vec!["Mr. Johan van der Merwe".to_string(), "Ms. Priya Naidoo".to_string(), "Mr. Sipho Dlamini".to_string()],
            management_letter_url: Some("/documents/agsa/2024-25/management_letter.pdf".to_string()),
            report_url: Some("/documents/agsa/2024-25/audit_report.pdf".to_string()),
            action_plan_due_date: "2025-09-30".to_string(),
            action_plan_submitted: true,
            action_plan_approved: false,
            created_at: "2025-07-31T10:00:00Z".to_string(),
            updated_at: "2025-08-15T14:30:00Z".to_string(),
        },
        AuditReport {
            id: "rpt_002".to_string(),
            reference_number: "AGSA/2023-24/001".to_string(),
            financial_year: "2023/24".to_string(),
            audit_type: AuditType::Regularity,
            title: "Annual Financial Statements Audit 2023/24".to_string(),
            entity_name: "Department of Public Works".to_string(),
            audit_period_start: "2023-04-01".to_string(),
            audit_period_end: "2024-03-31".to_string(),
            report_date: "2024-07-31".to_string(),
            compliance_status: ComplianceStatus::Qualified,
            overall_opinion: "Qualified opinion due to material misstatements in asset register and inadequate supporting documentation for irregular expenditure.".to_string(),
            key_findings_summary: "Material findings in asset management, irregular expenditure, and supply chain management compliance. 18 findings identified.".to_string(),
            total_findings: 18,
            material_findings: 4,
            significant_findings: 7,
            minor_findings: 7,
            repeat_findings: 6,
            resolved_findings: 14,
            lead_auditor: "Ms. Thembi Khoza".to_string(),
            audit_team: vec!["Mr. Johan van der Merwe".to_string(), "Ms. Priya Naidoo".to_string()],
            management_letter_url: Some("/documents/agsa/2023-24/management_letter.pdf".to_string()),
            report_url: Some("/documents/agsa/2023-24/audit_report.pdf".to_string()),
            action_plan_due_date: "2024-09-30".to_string(),
            action_plan_submitted: true,
            action_plan_approved: true,
            created_at: "2024-07-31T10:00:00Z".to_string(),
            updated_at: "2024-10-15T14:30:00Z".to_string(),
        },
    ];

    // Audit findings
    let mock_findings = vec![
        AuditFinding {
            id: "fnd_001".to_string(),
            reference_number: "AGSA/FND/2024-25/001".to_string(),
            audit_report_id: "rpt_001".to_string(),
            financial_year: "2024/25".to_string(),
            title: "Non-compliance with SCM Regulations - Three Quotations".to_string(),
            description: "Procurement transactions totaling R4.2 million were processed without obtaining the required three quotations as prescribed by Treasury Regulation 16A6.1. In 23 instances, single quotations were used without proper deviation approval.".to_string(),
            category: FindingCategory::SupplyChainManagement,
            severity: FindingSeverity::Material,
            status: FindingStatus::InProgress,
            root_cause: "Inadequate monitoring of procurement processes and lack of awareness of SCM requirements among end-users.".to_string(),
            risk_implication: "Non-compliance exposes the department to risk of not achieving value for money and potential irregular expenditure.".to_string(),
            management_response: "Management acknowledges the finding. A comprehensive SCM training program has been initiated and enhanced controls are being implemented.".to_string(),
            responsible_person: "Dr. Thandi Nkosi".to_string(),
            responsible_department: "Supply Chain Management".to_string(),
            target_date: "2025-12-31".to_string(),
            actual_resolution_date: None,
            is_repeat_finding: true,
            previous_finding_ref: Some("AGSA/FND/2023-24/003".to_string()),
            years_outstanding: 2,
            financial_impact: Some(4_200_000.0),
            action_items: Vec::new(),
            evidence_documents: vec!["/documents/agsa/fnd_001/evidence_1.pdf".to_string()],
            agsa_comments: vec![
                AgsaComment {
                    id: "cmt_001".to_string(),
                    author: "Ms. Thembi Khoza (AGSA)".to_string(),
                    content: "This is a repeat finding from the 2023/24 financial year. Management's previous commitments were not fully implemented.".to_string(),
                    comment_type: CommentType::InitialFinding,
                    created_at: "2025-07-31T10:00:00Z".to_string(),
                },
            ],
            created_at: "2025-07-31T10:00:00Z".to_string(),
            updated_at: "2025-08-20T09:00:00Z".to_string(),
        },
        AuditFinding {
            id: "fnd_002".to_string(),
            reference_number: "AGSA/FND/2024-25/002".to_string(),
            audit_report_id: "rpt_001".to_string(),
            financial_year: "2024/25".to_string(),
            title: "Asset Register Incomplete and Inaccurate".to_string(),
            description: "The asset register did not include all assets owned by the department. Physical verification identified R12.5 million in assets not recorded, and R8.3 million in assets that could not be physically located.".to_string(),
            category: FindingCategory::AssetManagement,
            severity: FindingSeverity::Material,
            status: FindingStatus::Open,
            root_cause: "Inadequate asset management procedures and lack of regular physical verification exercises.".to_string(),
            risk_implication: "Risk of asset loss, misappropriation, and material misstatement in the financial statements.".to_string(),
            management_response: "A comprehensive asset verification project has been initiated. An external service provider has been appointed to assist with the verification and update of the asset register.".to_string(),
            responsible_person: "Mr. Johannes Botha".to_string(),
            responsible_department: "Finance".to_string(),
            target_date: "2025-11-30".to_string(),
            actual_resolution_date: None,
            is_repeat_finding: true,
            previous_finding_ref: Some("AGSA/FND/2023-24/001".to_string()),
            years_outstanding: 3,
            financial_impact: Some(20_800_000.0),
            action_items: Vec::new(),
            evidence_documents: Vec::new(),
            agsa_comments: Vec::new(),
            created_at: "2025-07-31T10:00:00Z".to_string(),
            updated_at: "2025-08-10T14:00:00Z".to_string(),
        },
        AuditFinding {
            id: "fnd_003".to_string(),
            reference_number: "AGSA/FND/2024-25/003".to_string(),
            audit_report_id: "rpt_001".to_string(),
            financial_year: "2024/25".to_string(),
            title: "Performance Information Not Reliable".to_string(),
            description: "Supporting evidence for 4 out of 12 performance indicators tested could not be provided. The reported performance information was not accurate and complete.".to_string(),
            category: FindingCategory::PerformanceInformation,
            severity: FindingSeverity::Significant,
            status: FindingStatus::InProgress,
            root_cause: "Inadequate record keeping and lack of standard operating procedures for performance information collection.".to_string(),
            risk_implication: "Inability to assess actual performance against planned targets and accountability for service delivery.".to_string(),
            management_response: "Performance management unit has been strengthened with additional staff. A performance information management system is being implemented.".to_string(),
            responsible_person: "Ms. Naledi Mokoena".to_string(),
            responsible_department: "Strategic Planning".to_string(),
            target_date: "2025-10-31".to_string(),
            actual_resolution_date: None,
            is_repeat_finding: false,
            previous_finding_ref: None,
            years_outstanding: 1,
            financial_impact: None,
            action_items: Vec::new(),
            evidence_documents: Vec::new(),
            agsa_comments: Vec::new(),
            created_at: "2025-07-31T10:00:00Z".to_string(),
            updated_at: "2025-08-18T11:00:00Z".to_string(),
        },
        AuditFinding {
            id: "fnd_004".to_string(),
            reference_number: "AGSA/FND/2024-25/004".to_string(),
            audit_report_id: "rpt_001".to_string(),
            financial_year: "2024/25".to_string(),
            title: "Contract Management Deficiencies".to_string(),
            description: "Contract performance monitoring was inadequate. In 8 contracts valued at R45 million, deliverables were not properly verified before payment.".to_string(),
            category: FindingCategory::SupplyChainManagement,
            severity: FindingSeverity::Significant,
            status: FindingStatus::Open,
            root_cause: "Lack of dedicated contract management function and inadequate monitoring controls.".to_string(),
            risk_implication: "Payments for goods and services not received, and inability to enforce contract terms.".to_string(),
            management_response: "A Contract Management Unit will be established by Q3 2025. Contract management guidelines are being developed.".to_string(),
            responsible_person: "Dr. Thandi Nkosi".to_string(),
            responsible_department: "Supply Chain Management".to_string(),
            target_date: "2025-12-31".to_string(),
            actual_resolution_date: None,
            is_repeat_finding: true,
            previous_finding_ref: Some("AGSA/FND/2023-24/005".to_string()),
            years_outstanding: 2,
            financial_impact: Some(45_000_000.0),
            action_items: Vec::new(),
            evidence_documents: Vec::new(),
            agsa_comments: Vec::new(),
            created_at: "2025-07-31T10:00:00Z".to_string(),
            updated_at: "2025-08-05T16:00:00Z".to_string(),
        },
        AuditFinding {
            id: "fnd_005".to_string(),
            reference_number: "AGSA/FND/2024-25/005".to_string(),
            audit_report_id: "rpt_001".to_string(),
            financial_year: "2024/25".to_string(),
            title: "IT General Controls Weaknesses".to_string(),
            description: "User access management controls were not adequate. 15 terminated employees still had active system access, and password policies were not enforced.".to_string(),
            category: FindingCategory::InformationTechnology,
            severity: FindingSeverity::Significant,
            status: FindingStatus::InProgress,
            root_cause: "Inadequate coordination between HR and IT for user access termination, and outdated IT policies.".to_string(),
            risk_implication: "Unauthorized access to systems and data, potential fraud and data breaches.".to_string(),
            management_response: "Automated user provisioning and deprovisioning is being implemented. HR and IT integration project has commenced.".to_string(),
            responsible_person: "Mr. Sipho Dlamini".to_string(),
            responsible_department: "Information Technology".to_string(),
            target_date: "2025-09-30".to_string(),
            actual_resolution_date: None,
            is_repeat_finding: false,
            previous_finding_ref: None,
            years_outstanding: 1,
            financial_impact: None,
            action_items: Vec::new(),
            evidence_documents: Vec::new(),
            agsa_comments: Vec::new(),
            created_at: "2025-07-31T10:00:00Z".to_string(),
            updated_at: "2025-08-12T10:00:00Z".to_string(),
        },
        AuditFinding {
            id: "fnd_006".to_string(),
            reference_number: "AGSA/FND/2024-25/006".to_string(),
            audit_report_id: "rpt_001".to_string(),
            financial_year: "2024/25".to_string(),
            title: "B-BBEE Compliance Not Verified".to_string(),
            description: "In 28 procurement transactions, B-BBEE certificates were either expired at the time of award or not verified for authenticity.".to_string(),
            category: FindingCategory::Compliance,
            severity: FindingSeverity::Minor,
            status: FindingStatus::Resolved,
            root_cause: "Inadequate verification procedures during bid evaluation.".to_string(),
            risk_implication: "Potential preferential treatment based on fraudulent B-BBEE status.".to_string(),
            management_response: "B-BBEE verification has been integrated into the bid evaluation checklist. Online verification is now mandatory.".to_string(),
            responsible_person: "Dr. Thandi Nkosi".to_string(),
            responsible_department: "Supply Chain Management".to_string(),
            target_date: "2025-06-30".to_string(),
            actual_resolution_date: Some("2025-06-15".to_string()),
            is_repeat_finding: false,
            previous_finding_ref: None,
            years_outstanding: 0,
            financial_impact: None,
            action_items: Vec::new(),
            evidence_documents: Vec::new(),
            agsa_comments: vec![
                AgsaComment {
                    id: "cmt_002".to_string(),
                    author: "Mr. Johan van der Merwe (AGSA)".to_string(),
                    content: "Management action verified. Enhanced verification procedures are now in place.".to_string(),
                    comment_type: CommentType::Verification,
                    created_at: "2025-08-10T10:00:00Z".to_string(),
                },
            ],
            created_at: "2025-07-31T10:00:00Z".to_string(),
            updated_at: "2025-08-10T10:00:00Z".to_string(),
        },
    ];

    // Action items
    let mock_actions = vec![
        ActionItem {
            id: "act_001".to_string(),
            finding_id: "fnd_001".to_string(),
            reference_number: "ACT/2024-25/001".to_string(),
            description: "Develop and roll out comprehensive SCM training program for all users".to_string(),
            priority: ActionPriority::High,
            status: ActionStatus::InProgress,
            assigned_to: "Ms. Priya Naidoo".to_string(),
            assigned_department: "Supply Chain Management".to_string(),
            due_date: "2025-10-31".to_string(),
            completion_date: None,
            verification_date: None,
            verified_by: None,
            progress_percent: 60,
            notes: vec![
                ActionNote {
                    id: "note_001".to_string(),
                    author_id: "user_001".to_string(),
                    author_name: "Ms. Priya Naidoo".to_string(),
                    content: "Training material development completed. Pilot sessions scheduled for September.".to_string(),
                    created_at: "2025-08-20T10:00:00Z".to_string(),
                },
            ],
            evidence_documents: vec!["/documents/actions/act_001/training_plan.pdf".to_string()],
            blockers: Vec::new(),
            created_at: "2025-08-01T10:00:00Z".to_string(),
            updated_at: "2025-08-20T10:00:00Z".to_string(),
        },
        ActionItem {
            id: "act_002".to_string(),
            finding_id: "fnd_001".to_string(),
            reference_number: "ACT/2024-25/002".to_string(),
            description: "Implement enhanced SCM monitoring dashboard and exception reporting".to_string(),
            priority: ActionPriority::High,
            status: ActionStatus::InProgress,
            assigned_to: "Mr. Sipho Dlamini".to_string(),
            assigned_department: "Information Technology".to_string(),
            due_date: "2025-11-30".to_string(),
            completion_date: None,
            verification_date: None,
            verified_by: None,
            progress_percent: 35,
            notes: Vec::new(),
            evidence_documents: Vec::new(),
            blockers: vec!["Dependency on ERP upgrade scheduled for October".to_string()],
            created_at: "2025-08-01T10:00:00Z".to_string(),
            updated_at: "2025-08-15T14:00:00Z".to_string(),
        },
        ActionItem {
            id: "act_003".to_string(),
            finding_id: "fnd_002".to_string(),
            reference_number: "ACT/2024-25/003".to_string(),
            description: "Complete physical verification of all assets and update asset register".to_string(),
            priority: ActionPriority::Critical,
            status: ActionStatus::InProgress,
            assigned_to: "Mr. Johannes Botha".to_string(),
            assigned_department: "Finance".to_string(),
            due_date: "2025-09-30".to_string(),
            completion_date: None,
            verification_date: None,
            verified_by: None,
            progress_percent: 45,
            notes: Vec::new(),
            evidence_documents: Vec::new(),
            blockers: Vec::new(),
            created_at: "2025-08-01T10:00:00Z".to_string(),
            updated_at: "2025-08-22T09:00:00Z".to_string(),
        },
        ActionItem {
            id: "act_004".to_string(),
            finding_id: "fnd_004".to_string(),
            reference_number: "ACT/2024-25/004".to_string(),
            description: "Establish dedicated Contract Management Unit with clear TORs".to_string(),
            priority: ActionPriority::High,
            status: ActionStatus::NotStarted,
            assigned_to: "Dr. Thandi Nkosi".to_string(),
            assigned_department: "Supply Chain Management".to_string(),
            due_date: "2025-09-30".to_string(),
            completion_date: None,
            verification_date: None,
            verified_by: None,
            progress_percent: 0,
            notes: Vec::new(),
            evidence_documents: Vec::new(),
            blockers: vec!["Budget allocation pending Treasury approval".to_string()],
            created_at: "2025-08-01T10:00:00Z".to_string(),
            updated_at: "2025-08-01T10:00:00Z".to_string(),
        },
        ActionItem {
            id: "act_005".to_string(),
            finding_id: "fnd_005".to_string(),
            reference_number: "ACT/2024-25/005".to_string(),
            description: "Implement automated user access deprovisioning integrated with HR system".to_string(),
            priority: ActionPriority::High,
            status: ActionStatus::InProgress,
            assigned_to: "Mr. Sipho Dlamini".to_string(),
            assigned_department: "Information Technology".to_string(),
            due_date: "2025-09-15".to_string(),
            completion_date: None,
            verification_date: None,
            verified_by: None,
            progress_percent: 75,
            notes: vec![
                ActionNote {
                    id: "note_002".to_string(),
                    author_id: "user_003".to_string(),
                    author_name: "Mr. Sipho Dlamini".to_string(),
                    content: "Integration with HR system completed. UAT in progress.".to_string(),
                    created_at: "2025-08-18T14:00:00Z".to_string(),
                },
            ],
            evidence_documents: Vec::new(),
            blockers: Vec::new(),
            created_at: "2025-08-01T10:00:00Z".to_string(),
            updated_at: "2025-08-18T14:00:00Z".to_string(),
        },
        ActionItem {
            id: "act_006".to_string(),
            finding_id: "fnd_003".to_string(),
            reference_number: "ACT/2024-25/006".to_string(),
            description: "Implement performance information management system and SOPs".to_string(),
            priority: ActionPriority::Medium,
            status: ActionStatus::InProgress,
            assigned_to: "Ms. Naledi Mokoena".to_string(),
            assigned_department: "Strategic Planning".to_string(),
            due_date: "2025-10-15".to_string(),
            completion_date: None,
            verification_date: None,
            verified_by: None,
            progress_percent: 50,
            notes: Vec::new(),
            evidence_documents: Vec::new(),
            blockers: Vec::new(),
            created_at: "2025-08-01T10:00:00Z".to_string(),
            updated_at: "2025-08-16T11:00:00Z".to_string(),
        },
    ];

    // KPIs
    let kpis = AgsaKpis {
        total_findings: 12,
        open_findings: 2,
        in_progress_findings: 4,
        resolved_findings: 1,
        overdue_findings: 0,
        repeat_findings: 4,
        total_action_items: 18,
        completed_actions: 2,
        overdue_actions: 1,
        resolution_rate: 8.3,
        average_resolution_days: 45.0,
        current_compliance_status: ComplianceStatus::UnqualifiedWithFindings,
        financial_impact_total: 70_000_000.0,
    };

    store.audit_reports.set(mock_reports);
    store.findings.set(mock_findings);
    store.action_items.set(mock_actions);
    store.kpis.set(kpis);
}
