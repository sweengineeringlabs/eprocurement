//! GRC store

use components::prelude::*;
use super::types::{
    ComplianceCheck, ComplianceStatus, ComplianceCategory, ComplianceFinding, FindingStatus,
    RiskAssessment, RiskCategory, RiskLevel, RiskStatus, RiskTrend, Likelihood, Impact,
    RiskMitigation, MitigationType, MitigationStatus,
    PolicyViolation, ViolationType, ViolationStatus, EntityType, CorrectiveAction, ActionStatus,
    ControlStatus, ControlType, ControlCategory, ControlFrequency, ControlEffectiveness,
    ControlOperatingStatus, AutomationLevel, ControlTestResult, TestResult,
    Severity, Priority, GrcKpis, GrcFilter,
};

/// GRC state store
#[derive(Clone)]
pub struct GrcStore {
    pub compliance_checks: Signal<Vec<ComplianceCheck>>,
    pub risk_assessments: Signal<Vec<RiskAssessment>>,
    pub policy_violations: Signal<Vec<PolicyViolation>>,
    pub controls: Signal<Vec<ControlStatus>>,
    pub selected_compliance: Signal<Option<ComplianceCheck>>,
    pub selected_risk: Signal<Option<RiskAssessment>>,
    pub selected_violation: Signal<Option<PolicyViolation>>,
    pub selected_control: Signal<Option<ControlStatus>>,
    pub kpis: Signal<GrcKpis>,
    pub filter: Signal<GrcFilter>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
}

impl GrcStore {
    pub fn new() -> Self {
        Self {
            compliance_checks: signal(Vec::new()),
            risk_assessments: signal(Vec::new()),
            policy_violations: signal(Vec::new()),
            controls: signal(Vec::new()),
            selected_compliance: signal(None),
            selected_risk: signal(None),
            selected_violation: signal(None),
            selected_control: signal(None),
            kpis: signal(GrcKpis::default()),
            filter: signal(GrcFilter::default()),
            loading: signal(false),
            error: signal(None),
        }
    }
}

/// Load mock GRC data for demo
pub fn load_mock_data(store: &GrcStore) {
    // Mock compliance checks
    let compliance_checks = vec![
        ComplianceCheck {
            id: "CMP-001".to_string(),
            name: "PFMA Section 38 - CFO Responsibilities".to_string(),
            description: "Assessment of compliance with CFO responsibilities under PFMA".to_string(),
            category: ComplianceCategory::PFMA,
            regulation: "Public Finance Management Act, Section 38".to_string(),
            status: ComplianceStatus::Compliant,
            score: 95.0,
            last_assessed: "2025-01-15".to_string(),
            next_review: "2025-04-15".to_string(),
            assessor: "Internal Audit".to_string(),
            findings: vec![],
            evidence_count: 12,
            priority: Priority::High,
        },
        ComplianceCheck {
            id: "CMP-002".to_string(),
            name: "SCM Regulations - Quotation Processes".to_string(),
            description: "Compliance with quotation thresholds and processes per NT SCM regulations".to_string(),
            category: ComplianceCategory::SCM,
            regulation: "National Treasury SCM Instruction Note 3 of 2021".to_string(),
            status: ComplianceStatus::PartiallyCompliant,
            score: 72.0,
            last_assessed: "2025-02-01".to_string(),
            next_review: "2025-05-01".to_string(),
            assessor: "SCM Unit".to_string(),
            findings: vec![
                ComplianceFinding {
                    id: "FND-001".to_string(),
                    description: "Some quotations missing rotation evidence".to_string(),
                    severity: Severity::Medium,
                    status: FindingStatus::InProgress,
                    remediation: "Implement supplier rotation tracking".to_string(),
                    due_date: "2025-03-15".to_string(),
                    responsible: "SCM Manager".to_string(),
                },
            ],
            evidence_count: 8,
            priority: Priority::High,
        },
        ComplianceCheck {
            id: "CMP-003".to_string(),
            name: "B-BBEE Procurement Spend Target".to_string(),
            description: "Assessment of B-BBEE preferential procurement compliance".to_string(),
            category: ComplianceCategory::BBBEE,
            regulation: "PPPFA and B-BBEE Codes of Good Practice".to_string(),
            status: ComplianceStatus::Compliant,
            score: 88.0,
            last_assessed: "2025-02-10".to_string(),
            next_review: "2025-05-10".to_string(),
            assessor: "Transformation Office".to_string(),
            findings: vec![],
            evidence_count: 15,
            priority: Priority::High,
        },
        ComplianceCheck {
            id: "CMP-004".to_string(),
            name: "POPIA - Supplier Data Protection".to_string(),
            description: "Compliance with POPIA requirements for supplier personal information".to_string(),
            category: ComplianceCategory::DataProtection,
            regulation: "Protection of Personal Information Act".to_string(),
            status: ComplianceStatus::PartiallyCompliant,
            score: 68.0,
            last_assessed: "2025-01-20".to_string(),
            next_review: "2025-04-20".to_string(),
            assessor: "Information Officer".to_string(),
            findings: vec![
                ComplianceFinding {
                    id: "FND-002".to_string(),
                    description: "Data retention policy not fully implemented".to_string(),
                    severity: Severity::High,
                    status: FindingStatus::Open,
                    remediation: "Finalize and implement data retention procedures".to_string(),
                    due_date: "2025-03-01".to_string(),
                    responsible: "IT Manager".to_string(),
                },
            ],
            evidence_count: 6,
            priority: Priority::Critical,
        },
        ComplianceCheck {
            id: "CMP-005".to_string(),
            name: "Treasury Regulations - Contract Management".to_string(),
            description: "Compliance with NT regulations on contract management and variations".to_string(),
            category: ComplianceCategory::Treasury,
            regulation: "Treasury Regulation 16A".to_string(),
            status: ComplianceStatus::NonCompliant,
            score: 45.0,
            last_assessed: "2025-02-05".to_string(),
            next_review: "2025-03-05".to_string(),
            assessor: "Internal Audit".to_string(),
            findings: vec![
                ComplianceFinding {
                    id: "FND-003".to_string(),
                    description: "Contract variations exceeding 15% not properly approved".to_string(),
                    severity: Severity::Critical,
                    status: FindingStatus::Open,
                    remediation: "Review all contract variations and obtain proper approvals".to_string(),
                    due_date: "2025-02-28".to_string(),
                    responsible: "Contracts Manager".to_string(),
                },
                ComplianceFinding {
                    id: "FND-004".to_string(),
                    description: "Performance monitoring reports not submitted timely".to_string(),
                    severity: Severity::High,
                    status: FindingStatus::InProgress,
                    remediation: "Establish monthly reporting schedule".to_string(),
                    due_date: "2025-03-15".to_string(),
                    responsible: "Contract Officers".to_string(),
                },
            ],
            evidence_count: 4,
            priority: Priority::Critical,
        },
        ComplianceCheck {
            id: "CMP-006".to_string(),
            name: "Anti-Corruption Declaration Compliance".to_string(),
            description: "Verification of SBD4 declarations from all awarded suppliers".to_string(),
            category: ComplianceCategory::AntiCorruption,
            regulation: "Prevention and Combating of Corrupt Activities Act".to_string(),
            status: ComplianceStatus::Compliant,
            score: 100.0,
            last_assessed: "2025-02-12".to_string(),
            next_review: "2025-05-12".to_string(),
            assessor: "Bid Adjudication Committee".to_string(),
            findings: vec![],
            evidence_count: 45,
            priority: Priority::High,
        },
        ComplianceCheck {
            id: "CMP-007".to_string(),
            name: "Environmental Impact Assessment".to_string(),
            description: "Compliance with NEMA requirements for procurement activities".to_string(),
            category: ComplianceCategory::Environmental,
            regulation: "National Environmental Management Act".to_string(),
            status: ComplianceStatus::PendingReview,
            score: 0.0,
            last_assessed: "2024-11-30".to_string(),
            next_review: "2025-02-28".to_string(),
            assessor: "Environmental Officer".to_string(),
            findings: vec![],
            evidence_count: 0,
            priority: Priority::Medium,
        },
    ];

    // Mock risk assessments
    let risk_assessments = vec![
        RiskAssessment {
            id: "RISK-001".to_string(),
            name: "Supplier Concentration Risk".to_string(),
            description: "Over-reliance on limited number of suppliers for critical goods/services".to_string(),
            risk_category: RiskCategory::SupplyChain,
            likelihood: Likelihood::Likely,
            impact: Impact::Major,
            inherent_risk: RiskLevel::High,
            residual_risk: RiskLevel::Medium,
            risk_score: 16,
            residual_score: 9,
            owner: "SCM Manager".to_string(),
            department: "Supply Chain".to_string(),
            identified_date: "2024-06-15".to_string(),
            last_review: "2025-01-20".to_string(),
            next_review: "2025-04-20".to_string(),
            mitigations: vec![
                RiskMitigation {
                    id: "MIT-001".to_string(),
                    description: "Develop alternative supplier database".to_string(),
                    action_type: MitigationType::Reduce,
                    status: MitigationStatus::Implemented,
                    owner: "Procurement Officer".to_string(),
                    due_date: "2024-12-31".to_string(),
                    completion_date: Some("2024-12-15".to_string()),
                    effectiveness: Some(75),
                },
                RiskMitigation {
                    id: "MIT-002".to_string(),
                    description: "Implement supplier performance monitoring".to_string(),
                    action_type: MitigationType::Reduce,
                    status: MitigationStatus::InProgress,
                    owner: "SCM Manager".to_string(),
                    due_date: "2025-03-31".to_string(),
                    completion_date: None,
                    effectiveness: None,
                },
            ],
            status: RiskStatus::Active,
            trend: RiskTrend::Decreasing,
        },
        RiskAssessment {
            id: "RISK-002".to_string(),
            name: "Procurement Fraud Risk".to_string(),
            description: "Risk of fraudulent activities in procurement processes including collusion and bid rigging".to_string(),
            risk_category: RiskCategory::Fraud,
            likelihood: Likelihood::Possible,
            impact: Impact::Catastrophic,
            inherent_risk: RiskLevel::Extreme,
            residual_risk: RiskLevel::High,
            risk_score: 15,
            residual_score: 12,
            owner: "Chief Risk Officer".to_string(),
            department: "Risk Management".to_string(),
            identified_date: "2024-01-01".to_string(),
            last_review: "2025-02-01".to_string(),
            next_review: "2025-05-01".to_string(),
            mitigations: vec![
                RiskMitigation {
                    id: "MIT-003".to_string(),
                    description: "Implement bid analysis software".to_string(),
                    action_type: MitigationType::Reduce,
                    status: MitigationStatus::Verified,
                    owner: "IT Manager".to_string(),
                    due_date: "2024-09-30".to_string(),
                    completion_date: Some("2024-09-15".to_string()),
                    effectiveness: Some(80),
                },
                RiskMitigation {
                    id: "MIT-004".to_string(),
                    description: "Mandatory conflict of interest declarations".to_string(),
                    action_type: MitigationType::Reduce,
                    status: MitigationStatus::Implemented,
                    owner: "HR Manager".to_string(),
                    due_date: "2024-06-30".to_string(),
                    completion_date: Some("2024-06-28".to_string()),
                    effectiveness: Some(85),
                },
            ],
            status: RiskStatus::Active,
            trend: RiskTrend::Stable,
        },
        RiskAssessment {
            id: "RISK-003".to_string(),
            name: "Budget Overrun Risk".to_string(),
            description: "Risk of procurement expenditure exceeding approved budgets".to_string(),
            risk_category: RiskCategory::Financial,
            likelihood: Likelihood::Likely,
            impact: Impact::Moderate,
            inherent_risk: RiskLevel::Medium,
            residual_risk: RiskLevel::Low,
            risk_score: 12,
            residual_score: 4,
            owner: "CFO".to_string(),
            department: "Finance".to_string(),
            identified_date: "2024-04-01".to_string(),
            last_review: "2025-01-31".to_string(),
            next_review: "2025-04-30".to_string(),
            mitigations: vec![
                RiskMitigation {
                    id: "MIT-005".to_string(),
                    description: "Implement real-time budget monitoring".to_string(),
                    action_type: MitigationType::Reduce,
                    status: MitigationStatus::Verified,
                    owner: "Finance Manager".to_string(),
                    due_date: "2024-07-31".to_string(),
                    completion_date: Some("2024-07-20".to_string()),
                    effectiveness: Some(90),
                },
            ],
            status: RiskStatus::Monitoring,
            trend: RiskTrend::Decreasing,
        },
        RiskAssessment {
            id: "RISK-004".to_string(),
            name: "Cybersecurity Threat - Procurement Systems".to_string(),
            description: "Risk of cyber attacks targeting e-procurement systems and supplier data".to_string(),
            risk_category: RiskCategory::Cybersecurity,
            likelihood: Likelihood::Possible,
            impact: Impact::Major,
            inherent_risk: RiskLevel::High,
            residual_risk: RiskLevel::Medium,
            risk_score: 12,
            residual_score: 8,
            owner: "IT Security Manager".to_string(),
            department: "IT".to_string(),
            identified_date: "2024-03-15".to_string(),
            last_review: "2025-02-10".to_string(),
            next_review: "2025-05-10".to_string(),
            mitigations: vec![
                RiskMitigation {
                    id: "MIT-006".to_string(),
                    description: "Multi-factor authentication for all users".to_string(),
                    action_type: MitigationType::Reduce,
                    status: MitigationStatus::Implemented,
                    owner: "IT Security".to_string(),
                    due_date: "2024-08-31".to_string(),
                    completion_date: Some("2024-08-15".to_string()),
                    effectiveness: Some(85),
                },
            ],
            status: RiskStatus::Active,
            trend: RiskTrend::Stable,
        },
        RiskAssessment {
            id: "RISK-005".to_string(),
            name: "Regulatory Non-Compliance".to_string(),
            description: "Risk of non-compliance with changing SCM regulations and PFMA requirements".to_string(),
            risk_category: RiskCategory::Compliance,
            likelihood: Likelihood::Unlikely,
            impact: Impact::Major,
            inherent_risk: RiskLevel::Medium,
            residual_risk: RiskLevel::Low,
            risk_score: 8,
            residual_score: 4,
            owner: "Compliance Officer".to_string(),
            department: "Legal".to_string(),
            identified_date: "2024-02-01".to_string(),
            last_review: "2025-02-05".to_string(),
            next_review: "2025-05-05".to_string(),
            mitigations: vec![
                RiskMitigation {
                    id: "MIT-007".to_string(),
                    description: "Regular regulatory update training".to_string(),
                    action_type: MitigationType::Reduce,
                    status: MitigationStatus::Verified,
                    owner: "Training Manager".to_string(),
                    due_date: "2024-05-31".to_string(),
                    completion_date: Some("2024-05-25".to_string()),
                    effectiveness: Some(88),
                },
            ],
            status: RiskStatus::Monitoring,
            trend: RiskTrend::Decreasing,
        },
        RiskAssessment {
            id: "RISK-006".to_string(),
            name: "Reputational Damage - Supplier Misconduct".to_string(),
            description: "Risk of negative publicity from supplier ethical violations or misconduct".to_string(),
            risk_category: RiskCategory::Reputational,
            likelihood: Likelihood::Possible,
            impact: Impact::Moderate,
            inherent_risk: RiskLevel::Medium,
            residual_risk: RiskLevel::Medium,
            risk_score: 9,
            residual_score: 6,
            owner: "Communications Director".to_string(),
            department: "Corporate Affairs".to_string(),
            identified_date: "2024-05-20".to_string(),
            last_review: "2025-01-25".to_string(),
            next_review: "2025-04-25".to_string(),
            mitigations: vec![
                RiskMitigation {
                    id: "MIT-008".to_string(),
                    description: "Enhanced supplier due diligence".to_string(),
                    action_type: MitigationType::Reduce,
                    status: MitigationStatus::InProgress,
                    owner: "SCM Manager".to_string(),
                    due_date: "2025-03-31".to_string(),
                    completion_date: None,
                    effectiveness: None,
                },
            ],
            status: RiskStatus::Active,
            trend: RiskTrend::Stable,
        },
    ];

    // Mock policy violations
    let policy_violations = vec![
        PolicyViolation {
            id: "VIO-001".to_string(),
            policy_name: "Procurement Threshold Policy".to_string(),
            policy_id: "POL-SCM-003".to_string(),
            violation_type: ViolationType::Threshold,
            description: "Purchase order issued for R520,000 without competitive bidding (threshold R500,000)".to_string(),
            detected_date: "2025-02-10".to_string(),
            severity: Severity::High,
            status: ViolationStatus::UnderInvestigation,
            affected_entity: "PO-2025-0234".to_string(),
            entity_type: EntityType::PurchaseOrder,
            reported_by: "System Alert".to_string(),
            assigned_to: "SCM Manager".to_string(),
            resolution: None,
            resolution_date: None,
            financial_impact: Some(520_000.0),
            corrective_actions: vec![
                CorrectiveAction {
                    id: "CA-001".to_string(),
                    description: "Obtain retrospective approval from BAC".to_string(),
                    assigned_to: "Procurement Officer".to_string(),
                    due_date: "2025-02-20".to_string(),
                    status: ActionStatus::InProgress,
                    completion_date: None,
                },
            ],
        },
        PolicyViolation {
            id: "VIO-002".to_string(),
            policy_name: "Conflict of Interest Policy".to_string(),
            policy_id: "POL-ETH-001".to_string(),
            violation_type: ViolationType::Conflict,
            description: "Bid evaluation committee member had undisclosed relationship with bidding supplier".to_string(),
            detected_date: "2025-02-05".to_string(),
            severity: Severity::Critical,
            status: ViolationStatus::Escalated,
            affected_entity: "TND-2025-0045".to_string(),
            entity_type: EntityType::Tender,
            reported_by: "Whistleblower Hotline".to_string(),
            assigned_to: "Chief Risk Officer".to_string(),
            resolution: None,
            resolution_date: None,
            financial_impact: Some(2_500_000.0),
            corrective_actions: vec![
                CorrectiveAction {
                    id: "CA-002".to_string(),
                    description: "Re-evaluate tender with new committee".to_string(),
                    assigned_to: "BAC Chairperson".to_string(),
                    due_date: "2025-02-28".to_string(),
                    status: ActionStatus::Pending,
                    completion_date: None,
                },
                CorrectiveAction {
                    id: "CA-003".to_string(),
                    description: "Disciplinary process for employee".to_string(),
                    assigned_to: "HR Director".to_string(),
                    due_date: "2025-03-15".to_string(),
                    status: ActionStatus::InProgress,
                    completion_date: None,
                },
            ],
        },
        PolicyViolation {
            id: "VIO-003".to_string(),
            policy_name: "Documentation Requirements".to_string(),
            policy_id: "POL-SCM-001".to_string(),
            violation_type: ViolationType::Documentation,
            description: "Requisition approved without complete specifications document".to_string(),
            detected_date: "2025-02-08".to_string(),
            severity: Severity::Medium,
            status: ViolationStatus::Resolved,
            affected_entity: "REQ-2025-0789".to_string(),
            entity_type: EntityType::Requisition,
            reported_by: "Quality Assurance".to_string(),
            assigned_to: "Department Head".to_string(),
            resolution: Some("Specifications document obtained and attached".to_string()),
            resolution_date: Some("2025-02-12".to_string()),
            financial_impact: None,
            corrective_actions: vec![
                CorrectiveAction {
                    id: "CA-004".to_string(),
                    description: "Update checklist in system".to_string(),
                    assigned_to: "IT Support".to_string(),
                    due_date: "2025-02-15".to_string(),
                    status: ActionStatus::Completed,
                    completion_date: Some("2025-02-14".to_string()),
                },
            ],
        },
        PolicyViolation {
            id: "VIO-004".to_string(),
            policy_name: "Segregation of Duties".to_string(),
            policy_id: "POL-INT-002".to_string(),
            violation_type: ViolationType::Segregation,
            description: "Same user created requisition and approved purchase order".to_string(),
            detected_date: "2025-02-01".to_string(),
            severity: Severity::High,
            status: ViolationStatus::PendingAction,
            affected_entity: "PO-2025-0198".to_string(),
            entity_type: EntityType::PurchaseOrder,
            reported_by: "System Alert".to_string(),
            assigned_to: "Internal Audit".to_string(),
            resolution: None,
            resolution_date: None,
            financial_impact: Some(85_000.0),
            corrective_actions: vec![
                CorrectiveAction {
                    id: "CA-005".to_string(),
                    description: "Review and strengthen system access controls".to_string(),
                    assigned_to: "IT Security".to_string(),
                    due_date: "2025-02-25".to_string(),
                    status: ActionStatus::InProgress,
                    completion_date: None,
                },
            ],
        },
        PolicyViolation {
            id: "VIO-005".to_string(),
            policy_name: "Supplier Registration Policy".to_string(),
            policy_id: "POL-SUP-001".to_string(),
            violation_type: ViolationType::Authorization,
            description: "Contract awarded to supplier not on approved supplier database".to_string(),
            detected_date: "2025-01-28".to_string(),
            severity: Severity::High,
            status: ViolationStatus::Closed,
            affected_entity: "CTR-2025-0156".to_string(),
            entity_type: EntityType::Contract,
            reported_by: "Contract Administrator".to_string(),
            assigned_to: "SCM Manager".to_string(),
            resolution: Some("Supplier registration completed retrospectively, process reviewed".to_string()),
            resolution_date: Some("2025-02-10".to_string()),
            financial_impact: Some(350_000.0),
            corrective_actions: vec![
                CorrectiveAction {
                    id: "CA-006".to_string(),
                    description: "System block for unregistered suppliers".to_string(),
                    assigned_to: "IT Development".to_string(),
                    due_date: "2025-02-05".to_string(),
                    status: ActionStatus::Completed,
                    completion_date: Some("2025-02-05".to_string()),
                },
            ],
        },
    ];

    // Mock controls
    let controls = vec![
        ControlStatus {
            id: "CTL-001".to_string(),
            name: "Three-Way Match Verification".to_string(),
            description: "Automated matching of PO, GRN, and Invoice before payment".to_string(),
            control_type: ControlType::Preventive,
            category: ControlCategory::Financial,
            frequency: ControlFrequency::Continuous,
            owner: "Finance Manager".to_string(),
            effectiveness: ControlEffectiveness::Effective,
            effectiveness_score: 95,
            last_tested: "2025-02-01".to_string(),
            next_test: "2025-03-01".to_string(),
            status: ControlOperatingStatus::Operating,
            related_risks: vec!["RISK-001".to_string(), "RISK-002".to_string()],
            test_results: vec![
                ControlTestResult {
                    id: "TST-001".to_string(),
                    test_date: "2025-02-01".to_string(),
                    tester: "Internal Audit".to_string(),
                    result: TestResult::Pass,
                    findings: "Control operating effectively".to_string(),
                    sample_size: 50,
                    exceptions: 0,
                },
            ],
            automation_level: AutomationLevel::FullyAutomated,
        },
        ControlStatus {
            id: "CTL-002".to_string(),
            name: "Segregation of Duties Matrix".to_string(),
            description: "System-enforced segregation between requestor, approver, and receiver".to_string(),
            control_type: ControlType::Preventive,
            category: ControlCategory::Operational,
            frequency: ControlFrequency::Continuous,
            owner: "IT Security Manager".to_string(),
            effectiveness: ControlEffectiveness::PartiallyEffective,
            effectiveness_score: 72,
            last_tested: "2025-01-15".to_string(),
            next_test: "2025-02-15".to_string(),
            status: ControlOperatingStatus::PartiallyOperating,
            related_risks: vec!["RISK-002".to_string()],
            test_results: vec![
                ControlTestResult {
                    id: "TST-002".to_string(),
                    test_date: "2025-01-15".to_string(),
                    tester: "IT Audit".to_string(),
                    result: TestResult::PartialPass,
                    findings: "Some override capabilities exist that need review".to_string(),
                    sample_size: 100,
                    exceptions: 3,
                },
            ],
            automation_level: AutomationLevel::SemiAutomated,
        },
        ControlStatus {
            id: "CTL-003".to_string(),
            name: "Supplier Verification".to_string(),
            description: "CSD verification of all suppliers before registration approval".to_string(),
            control_type: ControlType::Detective,
            category: ControlCategory::Compliance,
            frequency: ControlFrequency::Continuous,
            owner: "SCM Manager".to_string(),
            effectiveness: ControlEffectiveness::Effective,
            effectiveness_score: 92,
            last_tested: "2025-02-05".to_string(),
            next_test: "2025-03-05".to_string(),
            status: ControlOperatingStatus::Operating,
            related_risks: vec!["RISK-002".to_string(), "RISK-006".to_string()],
            test_results: vec![
                ControlTestResult {
                    id: "TST-003".to_string(),
                    test_date: "2025-02-05".to_string(),
                    tester: "Compliance Team".to_string(),
                    result: TestResult::Pass,
                    findings: "All sampled suppliers verified against CSD".to_string(),
                    sample_size: 30,
                    exceptions: 0,
                },
            ],
            automation_level: AutomationLevel::FullyAutomated,
        },
        ControlStatus {
            id: "CTL-004".to_string(),
            name: "Budget Availability Check".to_string(),
            description: "Automated budget verification before requisition approval".to_string(),
            control_type: ControlType::Preventive,
            category: ControlCategory::Financial,
            frequency: ControlFrequency::Continuous,
            owner: "Finance Manager".to_string(),
            effectiveness: ControlEffectiveness::Effective,
            effectiveness_score: 98,
            last_tested: "2025-01-20".to_string(),
            next_test: "2025-02-20".to_string(),
            status: ControlOperatingStatus::Operating,
            related_risks: vec!["RISK-003".to_string()],
            test_results: vec![
                ControlTestResult {
                    id: "TST-004".to_string(),
                    test_date: "2025-01-20".to_string(),
                    tester: "Internal Audit".to_string(),
                    result: TestResult::Pass,
                    findings: "Control operating as designed".to_string(),
                    sample_size: 75,
                    exceptions: 0,
                },
            ],
            automation_level: AutomationLevel::FullyAutomated,
        },
        ControlStatus {
            id: "CTL-005".to_string(),
            name: "Bid Committee Review".to_string(),
            description: "Multi-person review and approval of bids above threshold".to_string(),
            control_type: ControlType::Preventive,
            category: ControlCategory::Operational,
            frequency: ControlFrequency::Weekly,
            owner: "BAC Chairperson".to_string(),
            effectiveness: ControlEffectiveness::Effective,
            effectiveness_score: 88,
            last_tested: "2025-02-08".to_string(),
            next_test: "2025-03-08".to_string(),
            status: ControlOperatingStatus::Operating,
            related_risks: vec!["RISK-002".to_string()],
            test_results: vec![
                ControlTestResult {
                    id: "TST-005".to_string(),
                    test_date: "2025-02-08".to_string(),
                    tester: "Internal Audit".to_string(),
                    result: TestResult::Pass,
                    findings: "Committee meetings held as scheduled, proper quorum maintained".to_string(),
                    sample_size: 12,
                    exceptions: 0,
                },
            ],
            automation_level: AutomationLevel::Manual,
        },
        ControlStatus {
            id: "CTL-006".to_string(),
            name: "Contract Expiry Monitoring".to_string(),
            description: "Automated alerts for contracts expiring within 90 days".to_string(),
            control_type: ControlType::Detective,
            category: ControlCategory::Operational,
            frequency: ControlFrequency::Daily,
            owner: "Contract Manager".to_string(),
            effectiveness: ControlEffectiveness::Effective,
            effectiveness_score: 85,
            last_tested: "2025-01-25".to_string(),
            next_test: "2025-02-25".to_string(),
            status: ControlOperatingStatus::Operating,
            related_risks: vec!["RISK-001".to_string()],
            test_results: vec![
                ControlTestResult {
                    id: "TST-006".to_string(),
                    test_date: "2025-01-25".to_string(),
                    tester: "IT Audit".to_string(),
                    result: TestResult::Pass,
                    findings: "Alerts being generated correctly".to_string(),
                    sample_size: 20,
                    exceptions: 1,
                },
            ],
            automation_level: AutomationLevel::FullyAutomated,
        },
        ControlStatus {
            id: "CTL-007".to_string(),
            name: "Conflict of Interest Declaration".to_string(),
            description: "Annual and transaction-based COI declarations from all procurement staff".to_string(),
            control_type: ControlType::Preventive,
            category: ControlCategory::Compliance,
            frequency: ControlFrequency::Annually,
            owner: "HR Manager".to_string(),
            effectiveness: ControlEffectiveness::PartiallyEffective,
            effectiveness_score: 68,
            last_tested: "2025-01-10".to_string(),
            next_test: "2025-04-10".to_string(),
            status: ControlOperatingStatus::PartiallyOperating,
            related_risks: vec!["RISK-002".to_string()],
            test_results: vec![
                ControlTestResult {
                    id: "TST-007".to_string(),
                    test_date: "2025-01-10".to_string(),
                    tester: "Compliance Team".to_string(),
                    result: TestResult::PartialPass,
                    findings: "5 employees with outstanding declarations".to_string(),
                    sample_size: 50,
                    exceptions: 5,
                },
            ],
            automation_level: AutomationLevel::SemiAutomated,
        },
        ControlStatus {
            id: "CTL-008".to_string(),
            name: "Access Control Review".to_string(),
            description: "Quarterly review of user access rights to procurement systems".to_string(),
            control_type: ControlType::Detective,
            category: ControlCategory::IT,
            frequency: ControlFrequency::Quarterly,
            owner: "IT Security Manager".to_string(),
            effectiveness: ControlEffectiveness::Ineffective,
            effectiveness_score: 45,
            last_tested: "2024-12-15".to_string(),
            next_test: "2025-03-15".to_string(),
            status: ControlOperatingStatus::UnderReview,
            related_risks: vec!["RISK-002".to_string(), "RISK-004".to_string()],
            test_results: vec![
                ControlTestResult {
                    id: "TST-008".to_string(),
                    test_date: "2024-12-15".to_string(),
                    tester: "IT Audit".to_string(),
                    result: TestResult::Fail,
                    findings: "Several terminated employees still had active access".to_string(),
                    sample_size: 40,
                    exceptions: 8,
                },
            ],
            automation_level: AutomationLevel::Manual,
        },
    ];

    // Set data
    store.compliance_checks.set(compliance_checks.clone());
    store.risk_assessments.set(risk_assessments.clone());
    store.policy_violations.set(policy_violations.clone());
    store.controls.set(controls.clone());

    // Calculate KPIs
    let total_checks = compliance_checks.len() as u32;
    let compliant = compliance_checks.iter()
        .filter(|c| c.status == ComplianceStatus::Compliant)
        .count() as u32;
    let non_compliant = compliance_checks.iter()
        .filter(|c| c.status == ComplianceStatus::NonCompliant)
        .count() as u32;
    let pending = compliance_checks.iter()
        .filter(|c| c.status == ComplianceStatus::PendingReview)
        .count() as u32;
    let compliance_score = if total_checks > 0 {
        compliance_checks.iter().map(|c| c.score).sum::<f64>() / total_checks as f64
    } else {
        0.0
    };

    let total_risks = risk_assessments.len() as u32;
    let extreme = risk_assessments.iter()
        .filter(|r| r.residual_risk == RiskLevel::Extreme)
        .count() as u32;
    let high = risk_assessments.iter()
        .filter(|r| r.residual_risk == RiskLevel::High)
        .count() as u32;
    let medium = risk_assessments.iter()
        .filter(|r| r.residual_risk == RiskLevel::Medium)
        .count() as u32;
    let low = risk_assessments.iter()
        .filter(|r| r.residual_risk == RiskLevel::Low)
        .count() as u32;

    let open_violations = policy_violations.iter()
        .filter(|v| v.status != ViolationStatus::Closed && v.status != ViolationStatus::Resolved)
        .count() as u32;
    let critical_violations = policy_violations.iter()
        .filter(|v| v.severity == Severity::Critical && v.status != ViolationStatus::Closed)
        .count() as u32;

    let total_controls = controls.len() as u32;
    let effective = controls.iter()
        .filter(|c| c.effectiveness == ControlEffectiveness::Effective)
        .count() as u32;
    let ineffective = controls.iter()
        .filter(|c| c.effectiveness == ControlEffectiveness::Ineffective)
        .count() as u32;
    let control_coverage = if total_controls > 0 {
        (effective as f64 / total_controls as f64) * 100.0
    } else {
        0.0
    };

    store.kpis.set(GrcKpis {
        compliance_score,
        total_checks,
        compliant_checks: compliant,
        non_compliant_checks: non_compliant,
        pending_reviews: pending,
        total_risks,
        extreme_risks: extreme,
        high_risks: high,
        medium_risks: medium,
        low_risks: low,
        open_violations,
        critical_violations,
        total_controls,
        effective_controls: effective,
        ineffective_controls: ineffective,
        control_coverage,
    });
}

/// Select a compliance check by ID
pub fn select_compliance(store: &GrcStore, compliance_id: &str) {
    let check = store.compliance_checks.get().iter()
        .find(|c| c.id == compliance_id)
        .cloned();
    store.selected_compliance.set(check);
}

/// Select a risk assessment by ID
pub fn select_risk(store: &GrcStore, risk_id: &str) {
    let risk = store.risk_assessments.get().iter()
        .find(|r| r.id == risk_id)
        .cloned();
    store.selected_risk.set(risk);
}

/// Select a policy violation by ID
pub fn select_violation(store: &GrcStore, violation_id: &str) {
    let violation = store.policy_violations.get().iter()
        .find(|v| v.id == violation_id)
        .cloned();
    store.selected_violation.set(violation);
}

/// Select a control by ID
pub fn select_control(store: &GrcStore, control_id: &str) {
    let control = store.controls.get().iter()
        .find(|c| c.id == control_id)
        .cloned();
    store.selected_control.set(control);
}

/// Clear all selections
pub fn clear_selections(store: &GrcStore) {
    store.selected_compliance.set(None);
    store.selected_risk.set(None);
    store.selected_violation.set(None);
    store.selected_control.set(None);
}
