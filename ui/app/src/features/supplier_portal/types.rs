//! Supplier Portal domain types

use serde::{Deserialize, Serialize};

/// Tender opportunity status from supplier perspective
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum OpportunityStatus {
    /// Open for bidding
    Open,
    /// Closing soon (within 7 days)
    ClosingSoon,
    /// Bidding closed
    Closed,
    /// Already submitted a bid
    BidSubmitted,
    /// Shortlisted for evaluation
    Shortlisted,
}

impl OpportunityStatus {
    pub fn label(&self) -> &'static str {
        match self {
            OpportunityStatus::Open => "Open",
            OpportunityStatus::ClosingSoon => "Closing Soon",
            OpportunityStatus::Closed => "Closed",
            OpportunityStatus::BidSubmitted => "Bid Submitted",
            OpportunityStatus::Shortlisted => "Shortlisted",
        }
    }

    pub fn css_class(&self) -> &'static str {
        match self {
            OpportunityStatus::Open => "status-open",
            OpportunityStatus::ClosingSoon => "status-closing-soon",
            OpportunityStatus::Closed => "status-closed",
            OpportunityStatus::BidSubmitted => "status-submitted",
            OpportunityStatus::Shortlisted => "status-shortlisted",
        }
    }
}

/// Tender opportunity visible to suppliers
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TenderOpportunity {
    pub id: String,
    pub reference_number: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub estimated_value: f64,
    pub currency: String,
    pub publish_date: String,
    pub closing_date: String,
    pub briefing_date: Option<String>,
    pub department: String,
    pub delivery_location: String,
    pub contract_duration: String,
    pub status: OpportunityStatus,
    pub days_remaining: i32,
    pub document_count: u32,
    pub mandatory_briefing: bool,
    pub tender_type: String, // RFQ, RFP, RFT
}

impl Default for TenderOpportunity {
    fn default() -> Self {
        Self {
            id: String::new(),
            reference_number: String::new(),
            title: String::new(),
            description: String::new(),
            category: String::new(),
            estimated_value: 0.0,
            currency: "ZAR".to_string(),
            publish_date: String::new(),
            closing_date: String::new(),
            briefing_date: None,
            department: String::new(),
            delivery_location: String::new(),
            contract_duration: String::new(),
            status: OpportunityStatus::Open,
            days_remaining: 0,
            document_count: 0,
            mandatory_briefing: false,
            tender_type: "RFQ".to_string(),
        }
    }
}

/// Bid submission status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum BidSubmissionStatus {
    /// Draft - not yet submitted
    Draft,
    /// Submitted and awaiting evaluation
    Submitted,
    /// Under technical evaluation
    UnderEvaluation,
    /// Compliant with requirements
    Compliant,
    /// Non-compliant with requirements
    NonCompliant,
    /// Shortlisted for final evaluation
    Shortlisted,
    /// Awarded the contract
    Awarded,
    /// Unsuccessful bid
    Unsuccessful,
    /// Withdrawn by supplier
    Withdrawn,
}

impl BidSubmissionStatus {
    pub fn label(&self) -> &'static str {
        match self {
            BidSubmissionStatus::Draft => "Draft",
            BidSubmissionStatus::Submitted => "Submitted",
            BidSubmissionStatus::UnderEvaluation => "Under Evaluation",
            BidSubmissionStatus::Compliant => "Compliant",
            BidSubmissionStatus::NonCompliant => "Non-Compliant",
            BidSubmissionStatus::Shortlisted => "Shortlisted",
            BidSubmissionStatus::Awarded => "Awarded",
            BidSubmissionStatus::Unsuccessful => "Unsuccessful",
            BidSubmissionStatus::Withdrawn => "Withdrawn",
        }
    }

    pub fn css_class(&self) -> &'static str {
        match self {
            BidSubmissionStatus::Draft => "status-draft",
            BidSubmissionStatus::Submitted => "status-submitted",
            BidSubmissionStatus::UnderEvaluation => "status-evaluation",
            BidSubmissionStatus::Compliant => "status-compliant",
            BidSubmissionStatus::NonCompliant => "status-non-compliant",
            BidSubmissionStatus::Shortlisted => "status-shortlisted",
            BidSubmissionStatus::Awarded => "status-awarded",
            BidSubmissionStatus::Unsuccessful => "status-unsuccessful",
            BidSubmissionStatus::Withdrawn => "status-withdrawn",
        }
    }
}

/// Bid submission from supplier
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BidSubmission {
    pub id: String,
    pub tender_id: String,
    pub tender_reference: String,
    pub tender_title: String,
    pub submitted_at: Option<String>,
    pub total_price: f64,
    pub currency: String,
    pub status: BidSubmissionStatus,
    pub technical_compliance: Option<f64>,
    pub price_score: Option<f64>,
    pub total_score: Option<f64>,
    pub rank: Option<u32>,
    pub documents_uploaded: u32,
    pub documents_required: u32,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for BidSubmission {
    fn default() -> Self {
        Self {
            id: String::new(),
            tender_id: String::new(),
            tender_reference: String::new(),
            tender_title: String::new(),
            submitted_at: None,
            total_price: 0.0,
            currency: "ZAR".to_string(),
            status: BidSubmissionStatus::Draft,
            technical_compliance: None,
            price_score: None,
            total_score: None,
            rank: None,
            documents_uploaded: 0,
            documents_required: 0,
            notes: None,
            created_at: String::new(),
            updated_at: String::new(),
        }
    }
}

/// Contract award status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ContractAwardStatus {
    /// Award notification received
    Notified,
    /// Awaiting contract signing
    AwaitingSignature,
    /// Contract signed and active
    Active,
    /// Contract completed
    Completed,
    /// Contract terminated
    Terminated,
}

impl ContractAwardStatus {
    pub fn label(&self) -> &'static str {
        match self {
            ContractAwardStatus::Notified => "Notified",
            ContractAwardStatus::AwaitingSignature => "Awaiting Signature",
            ContractAwardStatus::Active => "Active",
            ContractAwardStatus::Completed => "Completed",
            ContractAwardStatus::Terminated => "Terminated",
        }
    }

    pub fn css_class(&self) -> &'static str {
        match self {
            ContractAwardStatus::Notified => "status-notified",
            ContractAwardStatus::AwaitingSignature => "status-awaiting",
            ContractAwardStatus::Active => "status-active",
            ContractAwardStatus::Completed => "status-completed",
            ContractAwardStatus::Terminated => "status-terminated",
        }
    }
}

/// Contract award visible to supplier
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContractAward {
    pub id: String,
    pub contract_number: String,
    pub tender_reference: String,
    pub title: String,
    pub description: String,
    pub value: f64,
    pub currency: String,
    pub start_date: String,
    pub end_date: String,
    pub status: ContractAwardStatus,
    pub award_date: String,
    pub buyer_name: String,
    pub buyer_department: String,
    pub payment_terms: String,
    pub next_milestone: Option<String>,
    pub next_milestone_date: Option<String>,
    pub total_invoiced: f64,
    pub total_paid: f64,
    pub documents: Vec<PortalDocument>,
}

impl Default for ContractAward {
    fn default() -> Self {
        Self {
            id: String::new(),
            contract_number: String::new(),
            tender_reference: String::new(),
            title: String::new(),
            description: String::new(),
            value: 0.0,
            currency: "ZAR".to_string(),
            start_date: String::new(),
            end_date: String::new(),
            status: ContractAwardStatus::Notified,
            award_date: String::new(),
            buyer_name: String::new(),
            buyer_department: String::new(),
            payment_terms: String::new(),
            next_milestone: None,
            next_milestone_date: None,
            total_invoiced: 0.0,
            total_paid: 0.0,
            documents: Vec::new(),
        }
    }
}

/// Document type for portal
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum PortalDocumentType {
    /// Tender specification document
    TenderSpecification,
    /// Terms and conditions
    TermsConditions,
    /// Evaluation criteria
    EvaluationCriteria,
    /// Annexure document
    Annexure,
    /// Bid submission document
    BidDocument,
    /// Technical proposal
    TechnicalProposal,
    /// Financial proposal
    FinancialProposal,
    /// B-BBEE certificate
    BbbeeCertificate,
    /// Tax clearance
    TaxClearance,
    /// Company registration
    CompanyRegistration,
    /// Contract document
    Contract,
    /// Award letter
    AwardLetter,
    /// Purchase order
    PurchaseOrder,
    /// Invoice
    Invoice,
    /// Delivery note
    DeliveryNote,
    /// Other document
    Other,
}

impl PortalDocumentType {
    pub fn label(&self) -> &'static str {
        match self {
            PortalDocumentType::TenderSpecification => "Tender Specification",
            PortalDocumentType::TermsConditions => "Terms & Conditions",
            PortalDocumentType::EvaluationCriteria => "Evaluation Criteria",
            PortalDocumentType::Annexure => "Annexure",
            PortalDocumentType::BidDocument => "Bid Document",
            PortalDocumentType::TechnicalProposal => "Technical Proposal",
            PortalDocumentType::FinancialProposal => "Financial Proposal",
            PortalDocumentType::BbbeeCertificate => "B-BBEE Certificate",
            PortalDocumentType::TaxClearance => "Tax Clearance",
            PortalDocumentType::CompanyRegistration => "Company Registration",
            PortalDocumentType::Contract => "Contract",
            PortalDocumentType::AwardLetter => "Award Letter",
            PortalDocumentType::PurchaseOrder => "Purchase Order",
            PortalDocumentType::Invoice => "Invoice",
            PortalDocumentType::DeliveryNote => "Delivery Note",
            PortalDocumentType::Other => "Other",
        }
    }
}

/// Portal document
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PortalDocument {
    pub id: String,
    pub name: String,
    pub file_type: String,
    pub size: u64,
    pub document_type: PortalDocumentType,
    pub uploaded_at: String,
    pub uploaded_by: String,
    pub reference_id: String, // tender_id, bid_id, or contract_id
    pub reference_type: String, // "tender", "bid", "contract"
    pub is_mandatory: bool,
    pub is_uploaded: bool,
    pub download_url: Option<String>,
}

impl Default for PortalDocument {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            file_type: String::new(),
            size: 0,
            document_type: PortalDocumentType::Other,
            uploaded_at: String::new(),
            uploaded_by: String::new(),
            reference_id: String::new(),
            reference_type: String::new(),
            is_mandatory: false,
            is_uploaded: false,
            download_url: None,
        }
    }
}

/// Filter criteria for tender opportunities
#[derive(Clone, Debug, Default)]
pub struct OpportunityFilter {
    pub category: Option<String>,
    pub tender_type: Option<String>,
    pub status: Option<OpportunityStatus>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub search_query: Option<String>,
    pub closing_within_days: Option<u32>,
}

/// Filter criteria for bid submissions
#[derive(Clone, Debug, Default)]
pub struct BidSubmissionFilter {
    pub status: Option<BidSubmissionStatus>,
    pub search_query: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

/// Filter criteria for contract awards
#[derive(Clone, Debug, Default)]
pub struct ContractAwardFilter {
    pub status: Option<ContractAwardStatus>,
    pub search_query: Option<String>,
    pub year: Option<u32>,
}

/// Portal dashboard KPI data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PortalKpi {
    pub open_opportunities: u32,
    pub closing_soon: u32,
    pub active_bids: u32,
    pub awarded_contracts: u32,
    pub total_contract_value: f64,
    pub pending_documents: u32,
    pub success_rate: f64,
    pub ytd_revenue: f64,
}

impl Default for PortalKpi {
    fn default() -> Self {
        Self {
            open_opportunities: 12,
            closing_soon: 3,
            active_bids: 5,
            awarded_contracts: 8,
            total_contract_value: 45_000_000.0,
            pending_documents: 2,
            success_rate: 35.5,
            ytd_revenue: 12_500_000.0,
        }
    }
}

/// Portal notification
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PortalNotification {
    pub id: String,
    pub title: String,
    pub message: String,
    pub notification_type: String, // "opportunity", "bid", "contract", "document", "system"
    pub reference_id: Option<String>,
    pub created_at: String,
    pub read: bool,
    pub priority: String, // "high", "medium", "low"
}

impl Default for PortalNotification {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            message: String::new(),
            notification_type: "system".to_string(),
            reference_id: None,
            created_at: String::new(),
            read: false,
            priority: "medium".to_string(),
        }
    }
}
