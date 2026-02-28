//! NBAC domain types

use serde::{Deserialize, Serialize};

/// Review item status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReviewStatus {
    Pending,
    Scheduled,
    InReview,
    Approved,
    Rejected,
    Deferred,
    RequiresInfo,
}

impl ReviewStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReviewStatus::Pending => "pending",
            ReviewStatus::Scheduled => "scheduled",
            ReviewStatus::InReview => "in_review",
            ReviewStatus::Approved => "approved",
            ReviewStatus::Rejected => "rejected",
            ReviewStatus::Deferred => "deferred",
            ReviewStatus::RequiresInfo => "requires_info",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ReviewStatus::Pending => "Pending",
            ReviewStatus::Scheduled => "Scheduled",
            ReviewStatus::InReview => "In Review",
            ReviewStatus::Approved => "Approved",
            ReviewStatus::Rejected => "Rejected",
            ReviewStatus::Deferred => "Deferred",
            ReviewStatus::RequiresInfo => "Info Required",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            ReviewStatus::Pending => "var(--orange)",
            ReviewStatus::Scheduled => "var(--blue)",
            ReviewStatus::InReview => "var(--cyan)",
            ReviewStatus::Approved => "var(--green)",
            ReviewStatus::Rejected => "var(--red)",
            ReviewStatus::Deferred => "var(--purple)",
            ReviewStatus::RequiresInfo => "var(--orange)",
        }
    }
}

impl Default for ReviewStatus {
    fn default() -> Self {
        ReviewStatus::Pending
    }
}

/// Decision type for NBAC adjudication
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum DecisionType {
    Award,
    Reject,
    Defer,
    RequestInfo,
    Cancellation,
    Variation,
    Extension,
}

impl DecisionType {
    pub fn label(&self) -> &'static str {
        match self {
            DecisionType::Award => "Award",
            DecisionType::Reject => "Reject",
            DecisionType::Defer => "Defer",
            DecisionType::RequestInfo => "Request Information",
            DecisionType::Cancellation => "Cancellation",
            DecisionType::Variation => "Contract Variation",
            DecisionType::Extension => "Contract Extension",
        }
    }
}

/// Committee member role
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum MemberRole {
    Chairperson,
    DeputyChairperson,
    VotingMember,
    AdvisoryMember,
    Secretary,
    LegalAdvisor,
    TechnicalAdvisor,
    FinanceRepresentative,
}

impl MemberRole {
    pub fn label(&self) -> &'static str {
        match self {
            MemberRole::Chairperson => "Chairperson",
            MemberRole::DeputyChairperson => "Deputy Chairperson",
            MemberRole::VotingMember => "Voting Member",
            MemberRole::AdvisoryMember => "Advisory Member",
            MemberRole::Secretary => "Secretary",
            MemberRole::LegalAdvisor => "Legal Advisor",
            MemberRole::TechnicalAdvisor => "Technical Advisor",
            MemberRole::FinanceRepresentative => "Finance Representative",
        }
    }

    pub fn is_voting(&self) -> bool {
        matches!(
            self,
            MemberRole::Chairperson
                | MemberRole::DeputyChairperson
                | MemberRole::VotingMember
        )
    }
}

/// Committee member
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommitteeMember {
    pub id: String,
    pub name: String,
    pub title: String,
    pub role: MemberRole,
    pub department: String,
    pub email: String,
    pub phone: String,
    pub is_active: bool,
    pub appointment_date: String,
    pub term_end_date: Option<String>,
    pub conflict_of_interest: Vec<String>,
    pub attendance_rate: f64,
}

impl Default for CommitteeMember {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            title: String::new(),
            role: MemberRole::VotingMember,
            department: String::new(),
            email: String::new(),
            phone: String::new(),
            is_active: true,
            appointment_date: String::new(),
            term_end_date: None,
            conflict_of_interest: Vec::new(),
            attendance_rate: 100.0,
        }
    }
}

/// Vote record for a decision
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vote {
    pub member_id: String,
    pub member_name: String,
    pub vote: VoteType,
    pub reason: Option<String>,
    pub voted_at: String,
}

/// Vote type
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum VoteType {
    For,
    Against,
    Abstain,
    Recused,
}

impl VoteType {
    pub fn label(&self) -> &'static str {
        match self {
            VoteType::For => "For",
            VoteType::Against => "Against",
            VoteType::Abstain => "Abstain",
            VoteType::Recused => "Recused",
        }
    }
}

/// Decision record
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Decision {
    pub id: String,
    pub review_id: String,
    pub decision_type: DecisionType,
    pub decision_date: String,
    pub meeting_id: String,
    pub resolution_number: String,
    pub summary: String,
    pub rationale: String,
    pub conditions: Vec<String>,
    pub votes: Vec<Vote>,
    pub votes_for: u32,
    pub votes_against: u32,
    pub votes_abstain: u32,
    pub is_unanimous: bool,
    pub effective_date: String,
    pub approved_value: Option<f64>,
    pub awarded_supplier_id: Option<String>,
    pub awarded_supplier_name: Option<String>,
    pub attachments: Vec<String>,
    pub recorded_by: String,
    pub confirmed_by: Option<String>,
    pub confirmed_at: Option<String>,
}

impl Default for Decision {
    fn default() -> Self {
        Self {
            id: String::new(),
            review_id: String::new(),
            decision_type: DecisionType::Award,
            decision_date: String::new(),
            meeting_id: String::new(),
            resolution_number: String::new(),
            summary: String::new(),
            rationale: String::new(),
            conditions: Vec::new(),
            votes: Vec::new(),
            votes_for: 0,
            votes_against: 0,
            votes_abstain: 0,
            is_unanimous: false,
            effective_date: String::new(),
            approved_value: None,
            awarded_supplier_id: None,
            awarded_supplier_name: None,
            attachments: Vec::new(),
            recorded_by: String::new(),
            confirmed_by: None,
            confirmed_at: None,
        }
    }
}

/// Review item for NBAC consideration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReviewItem {
    pub id: String,
    pub reference_number: String,
    pub tender_id: String,
    pub tender_reference: String,
    pub tender_title: String,
    pub tender_value: f64,
    pub procurement_method: String,
    pub department: String,
    pub category: ReviewCategory,
    pub status: ReviewStatus,
    pub priority: Priority,
    pub submitted_by: String,
    pub submitted_date: String,
    pub scheduled_meeting_id: Option<String>,
    pub scheduled_meeting_date: Option<String>,
    pub evaluation_summary: String,
    pub recommended_supplier_id: Option<String>,
    pub recommended_supplier_name: Option<String>,
    pub recommended_value: Option<f64>,
    pub recommendation: String,
    pub bbbee_level: Option<u8>,
    pub local_content_percent: Option<f64>,
    pub compliance_checks: Vec<ComplianceCheckResult>,
    pub risk_rating: RiskRating,
    pub documents: Vec<ReviewDocument>,
    pub decision: Option<Decision>,
    pub notes: Vec<ReviewNote>,
}

impl Default for ReviewItem {
    fn default() -> Self {
        Self {
            id: String::new(),
            reference_number: String::new(),
            tender_id: String::new(),
            tender_reference: String::new(),
            tender_title: String::new(),
            tender_value: 0.0,
            procurement_method: String::new(),
            department: String::new(),
            category: ReviewCategory::BidAward,
            status: ReviewStatus::Pending,
            priority: Priority::Medium,
            submitted_by: String::new(),
            submitted_date: String::new(),
            scheduled_meeting_id: None,
            scheduled_meeting_date: None,
            evaluation_summary: String::new(),
            recommended_supplier_id: None,
            recommended_supplier_name: None,
            recommended_value: None,
            recommendation: String::new(),
            bbbee_level: None,
            local_content_percent: None,
            compliance_checks: Vec::new(),
            risk_rating: RiskRating::Medium,
            documents: Vec::new(),
            decision: None,
            notes: Vec::new(),
        }
    }
}

/// Review category
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReviewCategory {
    BidAward,
    Deviation,
    ContractVariation,
    ContractExtension,
    Cancellation,
    EmergencyProcurement,
    SingleSource,
    Confinement,
}

impl ReviewCategory {
    pub fn label(&self) -> &'static str {
        match self {
            ReviewCategory::BidAward => "Bid Award",
            ReviewCategory::Deviation => "Deviation",
            ReviewCategory::ContractVariation => "Contract Variation",
            ReviewCategory::ContractExtension => "Contract Extension",
            ReviewCategory::Cancellation => "Cancellation",
            ReviewCategory::EmergencyProcurement => "Emergency Procurement",
            ReviewCategory::SingleSource => "Single Source",
            ReviewCategory::Confinement => "Confinement",
        }
    }
}

impl Default for ReviewCategory {
    fn default() -> Self {
        ReviewCategory::BidAward
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

    pub fn color(&self) -> &'static str {
        match self {
            Priority::Critical => "#8b0000",
            Priority::High => "var(--red)",
            Priority::Medium => "var(--orange)",
            Priority::Low => "var(--blue)",
        }
    }
}

/// Risk rating
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum RiskRating {
    High,
    Medium,
    Low,
}

impl RiskRating {
    pub fn label(&self) -> &'static str {
        match self {
            RiskRating::High => "High",
            RiskRating::Medium => "Medium",
            RiskRating::Low => "Low",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            RiskRating::High => "var(--red)",
            RiskRating::Medium => "var(--orange)",
            RiskRating::Low => "var(--green)",
        }
    }
}

/// Compliance check result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComplianceCheckResult {
    pub check_name: String,
    pub passed: bool,
    pub details: Option<String>,
}

/// Review document
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReviewDocument {
    pub id: String,
    pub name: String,
    pub document_type: DocumentType,
    pub file_path: String,
    pub uploaded_by: String,
    pub uploaded_at: String,
    pub is_mandatory: bool,
}

/// Document type
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum DocumentType {
    EvaluationReport,
    TechnicalReport,
    FinancialAnalysis,
    BbbeeVerification,
    TaxClearance,
    ComplianceReport,
    RiskAssessment,
    LegalOpinion,
    MotivationMemo,
    Other,
}

impl DocumentType {
    pub fn label(&self) -> &'static str {
        match self {
            DocumentType::EvaluationReport => "Evaluation Report",
            DocumentType::TechnicalReport => "Technical Report",
            DocumentType::FinancialAnalysis => "Financial Analysis",
            DocumentType::BbbeeVerification => "B-BBEE Verification",
            DocumentType::TaxClearance => "Tax Clearance",
            DocumentType::ComplianceReport => "Compliance Report",
            DocumentType::RiskAssessment => "Risk Assessment",
            DocumentType::LegalOpinion => "Legal Opinion",
            DocumentType::MotivationMemo => "Motivation Memo",
            DocumentType::Other => "Other",
        }
    }
}

/// Review note
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReviewNote {
    pub id: String,
    pub author_id: String,
    pub author_name: String,
    pub content: String,
    pub created_at: String,
    pub is_internal: bool,
}

/// Meeting status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum MeetingStatus {
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
    Postponed,
}

impl MeetingStatus {
    pub fn label(&self) -> &'static str {
        match self {
            MeetingStatus::Scheduled => "Scheduled",
            MeetingStatus::InProgress => "In Progress",
            MeetingStatus::Completed => "Completed",
            MeetingStatus::Cancelled => "Cancelled",
            MeetingStatus::Postponed => "Postponed",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            MeetingStatus::Scheduled => "var(--blue)",
            MeetingStatus::InProgress => "var(--cyan)",
            MeetingStatus::Completed => "var(--green)",
            MeetingStatus::Cancelled => "var(--red)",
            MeetingStatus::Postponed => "var(--orange)",
        }
    }
}

impl Default for MeetingStatus {
    fn default() -> Self {
        MeetingStatus::Scheduled
    }
}

/// Meeting record
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Meeting {
    pub id: String,
    pub meeting_number: String,
    pub date: String,
    pub time: String,
    pub venue: String,
    pub meeting_type: MeetingType,
    pub status: MeetingStatus,
    pub chairperson_id: String,
    pub chairperson_name: String,
    pub secretary_id: String,
    pub secretary_name: String,
    pub attendees: Vec<MeetingAttendee>,
    pub apologies: Vec<String>,
    pub quorum_required: u32,
    pub quorum_present: u32,
    pub has_quorum: bool,
    pub agenda: Agenda,
    pub minutes_url: Option<String>,
    pub recording_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for Meeting {
    fn default() -> Self {
        Self {
            id: String::new(),
            meeting_number: String::new(),
            date: String::new(),
            time: String::new(),
            venue: String::new(),
            meeting_type: MeetingType::Ordinary,
            status: MeetingStatus::Scheduled,
            chairperson_id: String::new(),
            chairperson_name: String::new(),
            secretary_id: String::new(),
            secretary_name: String::new(),
            attendees: Vec::new(),
            apologies: Vec::new(),
            quorum_required: 5,
            quorum_present: 0,
            has_quorum: false,
            agenda: Agenda::default(),
            minutes_url: None,
            recording_url: None,
            created_at: String::new(),
            updated_at: String::new(),
        }
    }
}

/// Meeting type
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum MeetingType {
    Ordinary,
    Special,
    Emergency,
    AdHoc,
}

impl MeetingType {
    pub fn label(&self) -> &'static str {
        match self {
            MeetingType::Ordinary => "Ordinary",
            MeetingType::Special => "Special",
            MeetingType::Emergency => "Emergency",
            MeetingType::AdHoc => "Ad Hoc",
        }
    }
}

impl Default for MeetingType {
    fn default() -> Self {
        MeetingType::Ordinary
    }
}

/// Meeting attendee
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MeetingAttendee {
    pub member_id: String,
    pub member_name: String,
    pub role: MemberRole,
    pub attended: bool,
    pub arrival_time: Option<String>,
    pub departure_time: Option<String>,
    pub recusals: Vec<String>,
}

/// Agenda
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Agenda {
    pub id: String,
    pub meeting_id: String,
    pub items: Vec<AgendaItem>,
    pub previous_minutes_review: bool,
    pub matters_arising: Vec<String>,
    pub any_other_business: Vec<String>,
    pub next_meeting_date: Option<String>,
    pub finalized: bool,
    pub finalized_at: Option<String>,
}

/// Agenda item
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgendaItem {
    pub id: String,
    pub sequence: u32,
    pub review_id: String,
    pub title: String,
    pub presenter: String,
    pub estimated_duration_minutes: u32,
    pub status: AgendaItemStatus,
    pub outcome: Option<String>,
    pub resolution_number: Option<String>,
}

/// Agenda item status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum AgendaItemStatus {
    Pending,
    InProgress,
    Completed,
    Deferred,
    Withdrawn,
}

impl AgendaItemStatus {
    pub fn label(&self) -> &'static str {
        match self {
            AgendaItemStatus::Pending => "Pending",
            AgendaItemStatus::InProgress => "In Progress",
            AgendaItemStatus::Completed => "Completed",
            AgendaItemStatus::Deferred => "Deferred",
            AgendaItemStatus::Withdrawn => "Withdrawn",
        }
    }
}

impl Default for AgendaItemStatus {
    fn default() -> Self {
        AgendaItemStatus::Pending
    }
}

/// NBAC KPI summary
#[derive(Clone, Debug, Default)]
pub struct NbacKpis {
    pub pending_reviews: u32,
    pub scheduled_reviews: u32,
    pub decisions_this_month: u32,
    pub decisions_ytd: u32,
    pub total_value_approved_ytd: f64,
    pub average_turnaround_days: f64,
    pub approval_rate: f64,
    pub upcoming_meetings: u32,
    pub overdue_reviews: u32,
}

/// Filter criteria for reviews
#[derive(Clone, Debug, Default)]
pub struct ReviewFilter {
    pub status: Option<ReviewStatus>,
    pub category: Option<ReviewCategory>,
    pub priority: Option<Priority>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub department: Option<String>,
    pub search_query: Option<String>,
}
