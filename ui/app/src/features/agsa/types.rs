//! AGSA domain types

use serde::{Deserialize, Serialize};

/// Compliance status for AGSA audits
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Clean,
    UnqualifiedWithFindings,
    Qualified,
    Adverse,
    Disclaimer,
    PendingAudit,
}

impl ComplianceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ComplianceStatus::Clean => "clean",
            ComplianceStatus::UnqualifiedWithFindings => "unqualified_with_findings",
            ComplianceStatus::Qualified => "qualified",
            ComplianceStatus::Adverse => "adverse",
            ComplianceStatus::Disclaimer => "disclaimer",
            ComplianceStatus::PendingAudit => "pending_audit",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ComplianceStatus::Clean => "Clean Audit",
            ComplianceStatus::UnqualifiedWithFindings => "Unqualified with Findings",
            ComplianceStatus::Qualified => "Qualified Opinion",
            ComplianceStatus::Adverse => "Adverse Opinion",
            ComplianceStatus::Disclaimer => "Disclaimer of Opinion",
            ComplianceStatus::PendingAudit => "Pending Audit",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            ComplianceStatus::Clean => "var(--green)",
            ComplianceStatus::UnqualifiedWithFindings => "var(--blue)",
            ComplianceStatus::Qualified => "var(--orange)",
            ComplianceStatus::Adverse => "var(--red)",
            ComplianceStatus::Disclaimer => "#8b0000",
            ComplianceStatus::PendingAudit => "var(--text-muted)",
        }
    }
}

impl Default for ComplianceStatus {
    fn default() -> Self {
        ComplianceStatus::PendingAudit
    }
}

/// Audit finding severity
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum FindingSeverity {
    Material,
    Significant,
    Minor,
    Observation,
}

impl FindingSeverity {
    pub fn label(&self) -> &'static str {
        match self {
            FindingSeverity::Material => "Material",
            FindingSeverity::Significant => "Significant",
            FindingSeverity::Minor => "Minor",
            FindingSeverity::Observation => "Observation",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            FindingSeverity::Material => "#8b0000",
            FindingSeverity::Significant => "var(--red)",
            FindingSeverity::Minor => "var(--orange)",
            FindingSeverity::Observation => "var(--blue)",
        }
    }
}

impl Default for FindingSeverity {
    fn default() -> Self {
        FindingSeverity::Minor
    }
}

/// Finding status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum FindingStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
    Recurring,
    Overdue,
}

impl FindingStatus {
    pub fn label(&self) -> &'static str {
        match self {
            FindingStatus::Open => "Open",
            FindingStatus::InProgress => "In Progress",
            FindingStatus::Resolved => "Resolved",
            FindingStatus::Closed => "Closed",
            FindingStatus::Recurring => "Recurring",
            FindingStatus::Overdue => "Overdue",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            FindingStatus::Open => "var(--red)",
            FindingStatus::InProgress => "var(--cyan)",
            FindingStatus::Resolved => "var(--blue)",
            FindingStatus::Closed => "var(--green)",
            FindingStatus::Recurring => "var(--purple)",
            FindingStatus::Overdue => "#8b0000",
        }
    }
}

impl Default for FindingStatus {
    fn default() -> Self {
        FindingStatus::Open
    }
}

/// Audit finding category
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum FindingCategory {
    FinancialStatements,
    Compliance,
    PerformanceInformation,
    InternalControls,
    SupplyChainManagement,
    AssetManagement,
    HumanResources,
    InformationTechnology,
    Governance,
    FraudAndIrregularities,
}

impl FindingCategory {
    pub fn label(&self) -> &'static str {
        match self {
            FindingCategory::FinancialStatements => "Financial Statements",
            FindingCategory::Compliance => "Compliance",
            FindingCategory::PerformanceInformation => "Performance Information",
            FindingCategory::InternalControls => "Internal Controls",
            FindingCategory::SupplyChainManagement => "Supply Chain Management",
            FindingCategory::AssetManagement => "Asset Management",
            FindingCategory::HumanResources => "Human Resources",
            FindingCategory::InformationTechnology => "Information Technology",
            FindingCategory::Governance => "Governance",
            FindingCategory::FraudAndIrregularities => "Fraud & Irregularities",
        }
    }
}

impl Default for FindingCategory {
    fn default() -> Self {
        FindingCategory::Compliance
    }
}

/// Action item priority
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ActionPriority {
    Critical,
    High,
    Medium,
    Low,
}

impl ActionPriority {
    pub fn label(&self) -> &'static str {
        match self {
            ActionPriority::Critical => "Critical",
            ActionPriority::High => "High",
            ActionPriority::Medium => "Medium",
            ActionPriority::Low => "Low",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            ActionPriority::Critical => "#8b0000",
            ActionPriority::High => "var(--red)",
            ActionPriority::Medium => "var(--orange)",
            ActionPriority::Low => "var(--blue)",
        }
    }
}

impl Default for ActionPriority {
    fn default() -> Self {
        ActionPriority::Medium
    }
}

/// Action item status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ActionStatus {
    NotStarted,
    InProgress,
    Completed,
    Verified,
    Overdue,
    Cancelled,
}

impl ActionStatus {
    pub fn label(&self) -> &'static str {
        match self {
            ActionStatus::NotStarted => "Not Started",
            ActionStatus::InProgress => "In Progress",
            ActionStatus::Completed => "Completed",
            ActionStatus::Verified => "Verified",
            ActionStatus::Overdue => "Overdue",
            ActionStatus::Cancelled => "Cancelled",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            ActionStatus::NotStarted => "var(--text-muted)",
            ActionStatus::InProgress => "var(--cyan)",
            ActionStatus::Completed => "var(--blue)",
            ActionStatus::Verified => "var(--green)",
            ActionStatus::Overdue => "var(--red)",
            ActionStatus::Cancelled => "var(--text-muted)",
        }
    }
}

impl Default for ActionStatus {
    fn default() -> Self {
        ActionStatus::NotStarted
    }
}

/// Audit finding from AGSA
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditFinding {
    pub id: String,
    pub reference_number: String,
    pub audit_report_id: String,
    pub financial_year: String,
    pub title: String,
    pub description: String,
    pub category: FindingCategory,
    pub severity: FindingSeverity,
    pub status: FindingStatus,
    pub root_cause: String,
    pub risk_implication: String,
    pub management_response: String,
    pub responsible_person: String,
    pub responsible_department: String,
    pub target_date: String,
    pub actual_resolution_date: Option<String>,
    pub is_repeat_finding: bool,
    pub previous_finding_ref: Option<String>,
    pub years_outstanding: u32,
    pub financial_impact: Option<f64>,
    pub action_items: Vec<ActionItem>,
    pub evidence_documents: Vec<String>,
    pub agsa_comments: Vec<AgsaComment>,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for AuditFinding {
    fn default() -> Self {
        Self {
            id: String::new(),
            reference_number: String::new(),
            audit_report_id: String::new(),
            financial_year: String::new(),
            title: String::new(),
            description: String::new(),
            category: FindingCategory::default(),
            severity: FindingSeverity::default(),
            status: FindingStatus::default(),
            root_cause: String::new(),
            risk_implication: String::new(),
            management_response: String::new(),
            responsible_person: String::new(),
            responsible_department: String::new(),
            target_date: String::new(),
            actual_resolution_date: None,
            is_repeat_finding: false,
            previous_finding_ref: None,
            years_outstanding: 0,
            financial_impact: None,
            action_items: Vec::new(),
            evidence_documents: Vec::new(),
            agsa_comments: Vec::new(),
            created_at: String::new(),
            updated_at: String::new(),
        }
    }
}

/// Action item to address an audit finding
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionItem {
    pub id: String,
    pub finding_id: String,
    pub reference_number: String,
    pub description: String,
    pub priority: ActionPriority,
    pub status: ActionStatus,
    pub assigned_to: String,
    pub assigned_department: String,
    pub due_date: String,
    pub completion_date: Option<String>,
    pub verification_date: Option<String>,
    pub verified_by: Option<String>,
    pub progress_percent: u8,
    pub notes: Vec<ActionNote>,
    pub evidence_documents: Vec<String>,
    pub blockers: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for ActionItem {
    fn default() -> Self {
        Self {
            id: String::new(),
            finding_id: String::new(),
            reference_number: String::new(),
            description: String::new(),
            priority: ActionPriority::default(),
            status: ActionStatus::default(),
            assigned_to: String::new(),
            assigned_department: String::new(),
            due_date: String::new(),
            completion_date: None,
            verification_date: None,
            verified_by: None,
            progress_percent: 0,
            notes: Vec::new(),
            evidence_documents: Vec::new(),
            blockers: Vec::new(),
            created_at: String::new(),
            updated_at: String::new(),
        }
    }
}

/// Note on an action item
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionNote {
    pub id: String,
    pub author_id: String,
    pub author_name: String,
    pub content: String,
    pub created_at: String,
}

/// Comment from AGSA on a finding
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgsaComment {
    pub id: String,
    pub author: String,
    pub content: String,
    pub comment_type: CommentType,
    pub created_at: String,
}

/// Type of AGSA comment
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CommentType {
    InitialFinding,
    FollowUp,
    Verification,
    Closure,
    Concern,
}

impl CommentType {
    pub fn label(&self) -> &'static str {
        match self {
            CommentType::InitialFinding => "Initial Finding",
            CommentType::FollowUp => "Follow-up",
            CommentType::Verification => "Verification",
            CommentType::Closure => "Closure",
            CommentType::Concern => "Concern",
        }
    }
}

/// Audit report type
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum AuditType {
    Regularity,
    Performance,
    Special,
    FollowUp,
    Interim,
}

impl AuditType {
    pub fn label(&self) -> &'static str {
        match self {
            AuditType::Regularity => "Regularity Audit",
            AuditType::Performance => "Performance Audit",
            AuditType::Special => "Special Audit",
            AuditType::FollowUp => "Follow-up Audit",
            AuditType::Interim => "Interim Audit",
        }
    }
}

impl Default for AuditType {
    fn default() -> Self {
        AuditType::Regularity
    }
}

/// Audit report from AGSA
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditReport {
    pub id: String,
    pub reference_number: String,
    pub financial_year: String,
    pub audit_type: AuditType,
    pub title: String,
    pub entity_name: String,
    pub audit_period_start: String,
    pub audit_period_end: String,
    pub report_date: String,
    pub compliance_status: ComplianceStatus,
    pub overall_opinion: String,
    pub key_findings_summary: String,
    pub total_findings: u32,
    pub material_findings: u32,
    pub significant_findings: u32,
    pub minor_findings: u32,
    pub repeat_findings: u32,
    pub resolved_findings: u32,
    pub lead_auditor: String,
    pub audit_team: Vec<String>,
    pub management_letter_url: Option<String>,
    pub report_url: Option<String>,
    pub action_plan_due_date: String,
    pub action_plan_submitted: bool,
    pub action_plan_approved: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for AuditReport {
    fn default() -> Self {
        Self {
            id: String::new(),
            reference_number: String::new(),
            financial_year: String::new(),
            audit_type: AuditType::default(),
            title: String::new(),
            entity_name: String::new(),
            audit_period_start: String::new(),
            audit_period_end: String::new(),
            report_date: String::new(),
            compliance_status: ComplianceStatus::default(),
            overall_opinion: String::new(),
            key_findings_summary: String::new(),
            total_findings: 0,
            material_findings: 0,
            significant_findings: 0,
            minor_findings: 0,
            repeat_findings: 0,
            resolved_findings: 0,
            lead_auditor: String::new(),
            audit_team: Vec::new(),
            management_letter_url: None,
            report_url: None,
            action_plan_due_date: String::new(),
            action_plan_submitted: false,
            action_plan_approved: false,
            created_at: String::new(),
            updated_at: String::new(),
        }
    }
}

/// AGSA KPI summary
#[derive(Clone, Debug, Default)]
pub struct AgsaKpis {
    pub total_findings: u32,
    pub open_findings: u32,
    pub in_progress_findings: u32,
    pub resolved_findings: u32,
    pub overdue_findings: u32,
    pub repeat_findings: u32,
    pub total_action_items: u32,
    pub completed_actions: u32,
    pub overdue_actions: u32,
    pub resolution_rate: f64,
    pub average_resolution_days: f64,
    pub current_compliance_status: ComplianceStatus,
    pub financial_impact_total: f64,
}

/// Filter criteria for AGSA findings
#[derive(Clone, Debug, Default)]
pub struct AgsaFilter {
    pub status: Option<FindingStatus>,
    pub category: Option<FindingCategory>,
    pub severity: Option<FindingSeverity>,
    pub financial_year: Option<String>,
    pub department: Option<String>,
    pub is_repeat: Option<bool>,
    pub search_query: Option<String>,
}
