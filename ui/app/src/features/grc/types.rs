//! GRC domain types

use serde::{Deserialize, Serialize};

/// Compliance check status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    PartiallyCompliant,
    PendingReview,
    NotApplicable,
}

impl ComplianceStatus {
    pub fn label(&self) -> &'static str {
        match self {
            ComplianceStatus::Compliant => "Compliant",
            ComplianceStatus::NonCompliant => "Non-Compliant",
            ComplianceStatus::PartiallyCompliant => "Partially Compliant",
            ComplianceStatus::PendingReview => "Pending Review",
            ComplianceStatus::NotApplicable => "N/A",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            ComplianceStatus::Compliant => "var(--green)",
            ComplianceStatus::NonCompliant => "var(--red)",
            ComplianceStatus::PartiallyCompliant => "var(--orange)",
            ComplianceStatus::PendingReview => "var(--blue)",
            ComplianceStatus::NotApplicable => "var(--text-muted)",
        }
    }
}

impl Default for ComplianceStatus {
    fn default() -> Self {
        ComplianceStatus::PendingReview
    }
}

/// Compliance check representing a regulatory or policy requirement assessment
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComplianceCheck {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: ComplianceCategory,
    pub regulation: String,
    pub status: ComplianceStatus,
    pub score: f64,
    pub last_assessed: String,
    pub next_review: String,
    pub assessor: String,
    pub findings: Vec<ComplianceFinding>,
    pub evidence_count: u32,
    pub priority: Priority,
}

impl Default for ComplianceCheck {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            description: String::new(),
            category: ComplianceCategory::default(),
            regulation: String::new(),
            status: ComplianceStatus::default(),
            score: 0.0,
            last_assessed: String::new(),
            next_review: String::new(),
            assessor: String::new(),
            findings: Vec::new(),
            evidence_count: 0,
            priority: Priority::Medium,
        }
    }
}

/// Compliance category types
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ComplianceCategory {
    PFMA,           // Public Finance Management Act
    MFMA,           // Municipal Finance Management Act
    PPPFA,          // Preferential Procurement Policy Framework Act
    SCM,            // Supply Chain Management
    BBBEE,          // B-BBEE Compliance
    Treasury,       // National Treasury Regulations
    Environmental,  // Environmental compliance
    Labour,         // Labour law compliance
    DataProtection, // POPIA compliance
    AntiCorruption, // Anti-corruption measures
    Internal,       // Internal policies
}

impl ComplianceCategory {
    pub fn label(&self) -> &'static str {
        match self {
            ComplianceCategory::PFMA => "PFMA",
            ComplianceCategory::MFMA => "MFMA",
            ComplianceCategory::PPPFA => "PPPFA",
            ComplianceCategory::SCM => "SCM Regulations",
            ComplianceCategory::BBBEE => "B-BBEE",
            ComplianceCategory::Treasury => "NT Regulations",
            ComplianceCategory::Environmental => "Environmental",
            ComplianceCategory::Labour => "Labour Law",
            ComplianceCategory::DataProtection => "POPIA",
            ComplianceCategory::AntiCorruption => "Anti-Corruption",
            ComplianceCategory::Internal => "Internal Policy",
        }
    }
}

impl Default for ComplianceCategory {
    fn default() -> Self {
        ComplianceCategory::Internal
    }
}

/// Compliance finding from an assessment
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComplianceFinding {
    pub id: String,
    pub description: String,
    pub severity: Severity,
    pub status: FindingStatus,
    pub remediation: String,
    pub due_date: String,
    pub responsible: String,
}

/// Finding status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum FindingStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
    Overdue,
}

impl FindingStatus {
    pub fn label(&self) -> &'static str {
        match self {
            FindingStatus::Open => "Open",
            FindingStatus::InProgress => "In Progress",
            FindingStatus::Resolved => "Resolved",
            FindingStatus::Closed => "Closed",
            FindingStatus::Overdue => "Overdue",
        }
    }
}

/// Severity levels
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

impl Severity {
    pub fn label(&self) -> &'static str {
        match self {
            Severity::Critical => "Critical",
            Severity::High => "High",
            Severity::Medium => "Medium",
            Severity::Low => "Low",
            Severity::Informational => "Info",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            Severity::Critical => "#8b0000",
            Severity::High => "var(--red)",
            Severity::Medium => "var(--orange)",
            Severity::Low => "var(--blue)",
            Severity::Informational => "var(--text-muted)",
        }
    }
}

/// Priority levels
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

impl Priority {
    pub fn label(&self) -> &'static str {
        match self {
            Priority::Critical => "Critical",
            Priority::High => "High",
            Priority::Medium => "Medium",
            Priority::Low => "Low",
        }
    }
}

/// Risk likelihood levels
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Likelihood {
    AlmostCertain,
    Likely,
    Possible,
    Unlikely,
    Rare,
}

impl Likelihood {
    pub fn label(&self) -> &'static str {
        match self {
            Likelihood::AlmostCertain => "Almost Certain",
            Likelihood::Likely => "Likely",
            Likelihood::Possible => "Possible",
            Likelihood::Unlikely => "Unlikely",
            Likelihood::Rare => "Rare",
        }
    }

    pub fn score(&self) -> u8 {
        match self {
            Likelihood::AlmostCertain => 5,
            Likelihood::Likely => 4,
            Likelihood::Possible => 3,
            Likelihood::Unlikely => 2,
            Likelihood::Rare => 1,
        }
    }
}

/// Risk impact levels
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Impact {
    Catastrophic,
    Major,
    Moderate,
    Minor,
    Insignificant,
}

impl Impact {
    pub fn label(&self) -> &'static str {
        match self {
            Impact::Catastrophic => "Catastrophic",
            Impact::Major => "Major",
            Impact::Moderate => "Moderate",
            Impact::Minor => "Minor",
            Impact::Insignificant => "Insignificant",
        }
    }

    pub fn score(&self) -> u8 {
        match self {
            Impact::Catastrophic => 5,
            Impact::Major => 4,
            Impact::Moderate => 3,
            Impact::Minor => 2,
            Impact::Insignificant => 1,
        }
    }
}

/// Risk rating calculated from likelihood and impact
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum RiskLevel {
    Extreme,
    High,
    Medium,
    Low,
}

impl RiskLevel {
    pub fn label(&self) -> &'static str {
        match self {
            RiskLevel::Extreme => "Extreme",
            RiskLevel::High => "High",
            RiskLevel::Medium => "Medium",
            RiskLevel::Low => "Low",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            RiskLevel::Extreme => "#8b0000",
            RiskLevel::High => "var(--red)",
            RiskLevel::Medium => "var(--orange)",
            RiskLevel::Low => "var(--green)",
        }
    }

    pub fn from_score(score: u8) -> Self {
        match score {
            20..=25 => RiskLevel::Extreme,
            12..=19 => RiskLevel::High,
            6..=11 => RiskLevel::Medium,
            _ => RiskLevel::Low,
        }
    }
}

/// Risk assessment record
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub id: String,
    pub name: String,
    pub description: String,
    pub risk_category: RiskCategory,
    pub likelihood: Likelihood,
    pub impact: Impact,
    pub inherent_risk: RiskLevel,
    pub residual_risk: RiskLevel,
    pub risk_score: u8,
    pub residual_score: u8,
    pub owner: String,
    pub department: String,
    pub identified_date: String,
    pub last_review: String,
    pub next_review: String,
    pub mitigations: Vec<RiskMitigation>,
    pub status: RiskStatus,
    pub trend: RiskTrend,
}

impl Default for RiskAssessment {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            description: String::new(),
            risk_category: RiskCategory::default(),
            likelihood: Likelihood::Possible,
            impact: Impact::Moderate,
            inherent_risk: RiskLevel::Medium,
            residual_risk: RiskLevel::Medium,
            risk_score: 9,
            residual_score: 6,
            owner: String::new(),
            department: String::new(),
            identified_date: String::new(),
            last_review: String::new(),
            next_review: String::new(),
            mitigations: Vec::new(),
            status: RiskStatus::Active,
            trend: RiskTrend::Stable,
        }
    }
}

impl RiskAssessment {
    pub fn calculate_score(&self) -> u8 {
        self.likelihood.score() * self.impact.score()
    }
}

/// Risk categories
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum RiskCategory {
    Financial,
    Operational,
    Compliance,
    Reputational,
    Strategic,
    Fraud,
    Cybersecurity,
    SupplyChain,
    Environmental,
    Legal,
}

impl RiskCategory {
    pub fn label(&self) -> &'static str {
        match self {
            RiskCategory::Financial => "Financial",
            RiskCategory::Operational => "Operational",
            RiskCategory::Compliance => "Compliance",
            RiskCategory::Reputational => "Reputational",
            RiskCategory::Strategic => "Strategic",
            RiskCategory::Fraud => "Fraud",
            RiskCategory::Cybersecurity => "Cybersecurity",
            RiskCategory::SupplyChain => "Supply Chain",
            RiskCategory::Environmental => "Environmental",
            RiskCategory::Legal => "Legal",
        }
    }
}

impl Default for RiskCategory {
    fn default() -> Self {
        RiskCategory::Operational
    }
}

/// Risk mitigation action
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RiskMitigation {
    pub id: String,
    pub description: String,
    pub action_type: MitigationType,
    pub status: MitigationStatus,
    pub owner: String,
    pub due_date: String,
    pub completion_date: Option<String>,
    pub effectiveness: Option<u8>,
}

/// Mitigation action types
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum MitigationType {
    Avoid,
    Reduce,
    Transfer,
    Accept,
}

impl MitigationType {
    pub fn label(&self) -> &'static str {
        match self {
            MitigationType::Avoid => "Avoid",
            MitigationType::Reduce => "Reduce",
            MitigationType::Transfer => "Transfer",
            MitigationType::Accept => "Accept",
        }
    }
}

/// Mitigation status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum MitigationStatus {
    Planned,
    InProgress,
    Implemented,
    Verified,
    Ineffective,
}

impl MitigationStatus {
    pub fn label(&self) -> &'static str {
        match self {
            MitigationStatus::Planned => "Planned",
            MitigationStatus::InProgress => "In Progress",
            MitigationStatus::Implemented => "Implemented",
            MitigationStatus::Verified => "Verified",
            MitigationStatus::Ineffective => "Ineffective",
        }
    }
}

/// Risk status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum RiskStatus {
    Active,
    Monitoring,
    Closed,
    Escalated,
}

impl RiskStatus {
    pub fn label(&self) -> &'static str {
        match self {
            RiskStatus::Active => "Active",
            RiskStatus::Monitoring => "Monitoring",
            RiskStatus::Closed => "Closed",
            RiskStatus::Escalated => "Escalated",
        }
    }
}

/// Risk trend indicator
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum RiskTrend {
    Increasing,
    Stable,
    Decreasing,
}

impl RiskTrend {
    pub fn label(&self) -> &'static str {
        match self {
            RiskTrend::Increasing => "Increasing",
            RiskTrend::Stable => "Stable",
            RiskTrend::Decreasing => "Decreasing",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            RiskTrend::Increasing => "arrow-up",
            RiskTrend::Stable => "minus",
            RiskTrend::Decreasing => "arrow-down",
        }
    }
}

/// Policy violation record
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyViolation {
    pub id: String,
    pub policy_name: String,
    pub policy_id: String,
    pub violation_type: ViolationType,
    pub description: String,
    pub detected_date: String,
    pub severity: Severity,
    pub status: ViolationStatus,
    pub affected_entity: String,
    pub entity_type: EntityType,
    pub reported_by: String,
    pub assigned_to: String,
    pub resolution: Option<String>,
    pub resolution_date: Option<String>,
    pub financial_impact: Option<f64>,
    pub corrective_actions: Vec<CorrectiveAction>,
}

impl Default for PolicyViolation {
    fn default() -> Self {
        Self {
            id: String::new(),
            policy_name: String::new(),
            policy_id: String::new(),
            violation_type: ViolationType::Procedural,
            description: String::new(),
            detected_date: String::new(),
            severity: Severity::Medium,
            status: ViolationStatus::Open,
            affected_entity: String::new(),
            entity_type: EntityType::Transaction,
            reported_by: String::new(),
            assigned_to: String::new(),
            resolution: None,
            resolution_date: None,
            financial_impact: None,
            corrective_actions: Vec::new(),
        }
    }
}

/// Violation types
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ViolationType {
    Procedural,      // Process not followed
    Financial,       // Budget/threshold breaches
    Authorization,   // Unauthorized actions
    Documentation,   // Missing/incomplete docs
    Conflict,        // Conflict of interest
    Threshold,       // Exceeded limits
    Timeline,        // Deadline violations
    Segregation,     // Segregation of duties
}

impl ViolationType {
    pub fn label(&self) -> &'static str {
        match self {
            ViolationType::Procedural => "Procedural",
            ViolationType::Financial => "Financial",
            ViolationType::Authorization => "Authorization",
            ViolationType::Documentation => "Documentation",
            ViolationType::Conflict => "Conflict of Interest",
            ViolationType::Threshold => "Threshold Breach",
            ViolationType::Timeline => "Timeline",
            ViolationType::Segregation => "Segregation of Duties",
        }
    }
}

/// Violation status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ViolationStatus {
    Open,
    UnderInvestigation,
    PendingAction,
    Resolved,
    Closed,
    Escalated,
}

impl ViolationStatus {
    pub fn label(&self) -> &'static str {
        match self {
            ViolationStatus::Open => "Open",
            ViolationStatus::UnderInvestigation => "Under Investigation",
            ViolationStatus::PendingAction => "Pending Action",
            ViolationStatus::Resolved => "Resolved",
            ViolationStatus::Closed => "Closed",
            ViolationStatus::Escalated => "Escalated",
        }
    }
}

/// Entity types that can have violations
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum EntityType {
    Requisition,
    Tender,
    Contract,
    PurchaseOrder,
    Supplier,
    User,
    Transaction,
    Department,
}

impl EntityType {
    pub fn label(&self) -> &'static str {
        match self {
            EntityType::Requisition => "Requisition",
            EntityType::Tender => "Tender",
            EntityType::Contract => "Contract",
            EntityType::PurchaseOrder => "Purchase Order",
            EntityType::Supplier => "Supplier",
            EntityType::User => "User",
            EntityType::Transaction => "Transaction",
            EntityType::Department => "Department",
        }
    }
}

/// Corrective action for violations
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CorrectiveAction {
    pub id: String,
    pub description: String,
    pub assigned_to: String,
    pub due_date: String,
    pub status: ActionStatus,
    pub completion_date: Option<String>,
}

/// Corrective action status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ActionStatus {
    Pending,
    InProgress,
    Completed,
    Overdue,
    Cancelled,
}

impl ActionStatus {
    pub fn label(&self) -> &'static str {
        match self {
            ActionStatus::Pending => "Pending",
            ActionStatus::InProgress => "In Progress",
            ActionStatus::Completed => "Completed",
            ActionStatus::Overdue => "Overdue",
            ActionStatus::Cancelled => "Cancelled",
        }
    }
}

/// Internal control record
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ControlStatus {
    pub id: String,
    pub name: String,
    pub description: String,
    pub control_type: ControlType,
    pub category: ControlCategory,
    pub frequency: ControlFrequency,
    pub owner: String,
    pub effectiveness: ControlEffectiveness,
    pub effectiveness_score: u8,
    pub last_tested: String,
    pub next_test: String,
    pub status: ControlOperatingStatus,
    pub related_risks: Vec<String>,
    pub test_results: Vec<ControlTestResult>,
    pub automation_level: AutomationLevel,
}

impl Default for ControlStatus {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            description: String::new(),
            control_type: ControlType::Detective,
            category: ControlCategory::Financial,
            frequency: ControlFrequency::Monthly,
            owner: String::new(),
            effectiveness: ControlEffectiveness::Effective,
            effectiveness_score: 80,
            last_tested: String::new(),
            next_test: String::new(),
            status: ControlOperatingStatus::Operating,
            related_risks: Vec::new(),
            test_results: Vec::new(),
            automation_level: AutomationLevel::Manual,
        }
    }
}

/// Control types
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ControlType {
    Preventive,
    Detective,
    Corrective,
    Directive,
}

impl ControlType {
    pub fn label(&self) -> &'static str {
        match self {
            ControlType::Preventive => "Preventive",
            ControlType::Detective => "Detective",
            ControlType::Corrective => "Corrective",
            ControlType::Directive => "Directive",
        }
    }
}

/// Control categories
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ControlCategory {
    Financial,
    Operational,
    IT,
    Compliance,
    Strategic,
}

impl ControlCategory {
    pub fn label(&self) -> &'static str {
        match self {
            ControlCategory::Financial => "Financial",
            ControlCategory::Operational => "Operational",
            ControlCategory::IT => "IT",
            ControlCategory::Compliance => "Compliance",
            ControlCategory::Strategic => "Strategic",
        }
    }
}

/// Control testing frequency
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ControlFrequency {
    Continuous,
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annually,
}

impl ControlFrequency {
    pub fn label(&self) -> &'static str {
        match self {
            ControlFrequency::Continuous => "Continuous",
            ControlFrequency::Daily => "Daily",
            ControlFrequency::Weekly => "Weekly",
            ControlFrequency::Monthly => "Monthly",
            ControlFrequency::Quarterly => "Quarterly",
            ControlFrequency::Annually => "Annually",
        }
    }
}

/// Control effectiveness rating
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ControlEffectiveness {
    Effective,
    PartiallyEffective,
    Ineffective,
    NotTested,
}

impl ControlEffectiveness {
    pub fn label(&self) -> &'static str {
        match self {
            ControlEffectiveness::Effective => "Effective",
            ControlEffectiveness::PartiallyEffective => "Partially Effective",
            ControlEffectiveness::Ineffective => "Ineffective",
            ControlEffectiveness::NotTested => "Not Tested",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            ControlEffectiveness::Effective => "var(--green)",
            ControlEffectiveness::PartiallyEffective => "var(--orange)",
            ControlEffectiveness::Ineffective => "var(--red)",
            ControlEffectiveness::NotTested => "var(--text-muted)",
        }
    }
}

/// Control operating status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ControlOperatingStatus {
    Operating,
    PartiallyOperating,
    NotOperating,
    UnderReview,
    Retired,
}

impl ControlOperatingStatus {
    pub fn label(&self) -> &'static str {
        match self {
            ControlOperatingStatus::Operating => "Operating",
            ControlOperatingStatus::PartiallyOperating => "Partially Operating",
            ControlOperatingStatus::NotOperating => "Not Operating",
            ControlOperatingStatus::UnderReview => "Under Review",
            ControlOperatingStatus::Retired => "Retired",
        }
    }
}

/// Automation level for controls
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum AutomationLevel {
    Manual,
    SemiAutomated,
    FullyAutomated,
}

impl AutomationLevel {
    pub fn label(&self) -> &'static str {
        match self {
            AutomationLevel::Manual => "Manual",
            AutomationLevel::SemiAutomated => "Semi-Automated",
            AutomationLevel::FullyAutomated => "Fully Automated",
        }
    }
}

/// Control test result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ControlTestResult {
    pub id: String,
    pub test_date: String,
    pub tester: String,
    pub result: TestResult,
    pub findings: String,
    pub sample_size: u32,
    pub exceptions: u32,
}

/// Test result
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum TestResult {
    Pass,
    Fail,
    PartialPass,
    NotApplicable,
}

impl TestResult {
    pub fn label(&self) -> &'static str {
        match self {
            TestResult::Pass => "Pass",
            TestResult::Fail => "Fail",
            TestResult::PartialPass => "Partial Pass",
            TestResult::NotApplicable => "N/A",
        }
    }
}

/// GRC KPI summary
#[derive(Clone, Debug, Default)]
pub struct GrcKpis {
    pub compliance_score: f64,
    pub total_checks: u32,
    pub compliant_checks: u32,
    pub non_compliant_checks: u32,
    pub pending_reviews: u32,
    pub total_risks: u32,
    pub extreme_risks: u32,
    pub high_risks: u32,
    pub medium_risks: u32,
    pub low_risks: u32,
    pub open_violations: u32,
    pub critical_violations: u32,
    pub total_controls: u32,
    pub effective_controls: u32,
    pub ineffective_controls: u32,
    pub control_coverage: f64,
}

/// GRC filter criteria
#[derive(Clone, Debug, Default)]
pub struct GrcFilter {
    pub category: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub search_query: Option<String>,
}

/// Risk matrix cell data
#[derive(Clone, Debug)]
pub struct RiskMatrixCell {
    pub likelihood: Likelihood,
    pub impact: Impact,
    pub count: u32,
    pub risk_level: RiskLevel,
}
