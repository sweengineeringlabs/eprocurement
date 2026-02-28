//! Tender domain types

use serde::{Deserialize, Serialize};

/// Tender type enumeration
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum TenderType {
    /// Request for Quotation - for lower value procurement
    Rfq,
    /// Request for Proposal - for complex procurement requiring evaluation
    Rfp,
    /// Request for Tender - formal competitive tender
    Rft,
}

impl TenderType {
    pub fn label(&self) -> &'static str {
        match self {
            TenderType::Rfq => "RFQ",
            TenderType::Rfp => "RFP",
            TenderType::Rft => "RFT",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            TenderType::Rfq => "Request for Quotation",
            TenderType::Rfp => "Request for Proposal",
            TenderType::Rft => "Request for Tender",
        }
    }
}

impl Default for TenderType {
    fn default() -> Self {
        TenderType::Rfq
    }
}

/// Tender status enumeration
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum TenderStatus {
    /// Initial draft state
    Draft,
    /// Awaiting internal approval
    PendingApproval,
    /// Approved and ready for publication
    Approved,
    /// Published to e-Tender portal
    Published,
    /// Open for bidding
    Open,
    /// Bidding period closed
    Closed,
    /// Under evaluation
    Evaluation,
    /// Evaluation complete, awaiting award
    Adjudication,
    /// Contract awarded
    Awarded,
    /// Tender cancelled
    Cancelled,
}

impl TenderStatus {
    pub fn label(&self) -> &'static str {
        match self {
            TenderStatus::Draft => "Draft",
            TenderStatus::PendingApproval => "Pending Approval",
            TenderStatus::Approved => "Approved",
            TenderStatus::Published => "Published",
            TenderStatus::Open => "Open",
            TenderStatus::Closed => "Closed",
            TenderStatus::Evaluation => "Evaluation",
            TenderStatus::Adjudication => "Adjudication",
            TenderStatus::Awarded => "Awarded",
            TenderStatus::Cancelled => "Cancelled",
        }
    }
}

impl Default for TenderStatus {
    fn default() -> Self {
        TenderStatus::Draft
    }
}

/// Deviation type for non-competitive procurement
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum DeviationType {
    /// Single source procurement
    SingleSource,
    /// Emergency procurement
    Emergency,
    /// Sole supplier available
    SoleSupplier,
    /// Strategic partnership
    Strategic,
}

impl DeviationType {
    pub fn label(&self) -> &'static str {
        match self {
            DeviationType::SingleSource => "Single Source",
            DeviationType::Emergency => "Emergency",
            DeviationType::SoleSupplier => "Sole Supplier",
            DeviationType::Strategic => "Strategic Partnership",
        }
    }
}

/// Evaluation criterion for tender scoring
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvaluationCriterion {
    pub id: String,
    pub name: String,
    pub description: String,
    pub weight: f64, // percentage weight
    pub max_score: u32,
}

/// Tender document attachment
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TenderDocument {
    pub id: String,
    pub name: String,
    pub file_type: String,
    pub size: u64,
    pub uploaded_at: String,
    pub category: String, // "specification", "terms", "evaluation", "annexure"
}

/// Bid submission from a supplier
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bid {
    pub id: String,
    pub tender_id: String,
    pub supplier_id: String,
    pub supplier_name: String,
    pub submitted_at: String,
    pub total_price: f64,
    pub bbbee_level: Option<u8>,
    pub status: BidStatus,
    pub technical_score: Option<f64>,
    pub price_score: Option<f64>,
    pub bbbee_score: Option<f64>,
    pub total_score: Option<f64>,
}

/// Bid status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum BidStatus {
    Received,
    UnderReview,
    Compliant,
    NonCompliant,
    Shortlisted,
    Awarded,
    Unsuccessful,
}

impl BidStatus {
    pub fn label(&self) -> &'static str {
        match self {
            BidStatus::Received => "Received",
            BidStatus::UnderReview => "Under Review",
            BidStatus::Compliant => "Compliant",
            BidStatus::NonCompliant => "Non-Compliant",
            BidStatus::Shortlisted => "Shortlisted",
            BidStatus::Awarded => "Awarded",
            BidStatus::Unsuccessful => "Unsuccessful",
        }
    }
}

/// Main Tender entity
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tender {
    pub id: String,
    pub reference_number: String,
    pub title: String,
    pub description: String,
    pub tender_type: TenderType,
    pub status: TenderStatus,
    pub estimated_value: f64,
    pub currency: String,

    // Dates
    pub created_at: String,
    pub publish_date: Option<String>,
    pub closing_date: Option<String>,
    pub briefing_date: Option<String>,
    pub award_date: Option<String>,

    // Scope
    pub category: String,
    pub department: String,
    pub cost_center: String,
    pub delivery_location: String,
    pub contract_duration: String,

    // Requirements
    pub scope_of_work: String,
    pub technical_requirements: String,
    pub mandatory_requirements: Vec<String>,

    // Evaluation
    pub evaluation_criteria: Vec<EvaluationCriterion>,
    pub price_weight: f64,
    pub bbbee_weight: f64,
    pub functionality_threshold: f64,

    // Documents
    pub documents: Vec<TenderDocument>,

    // Bids
    pub bids: Vec<Bid>,

    // Portal
    pub portal_reference: Option<String>,
    pub portal_url: Option<String>,

    // Deviation (if applicable)
    pub deviation_type: Option<DeviationType>,
    pub deviation_justification: Option<String>,
    pub deviation_approved_by: Option<String>,
    pub deviation_approved_at: Option<String>,

    // Audit
    pub created_by: String,
    pub last_modified_by: String,
    pub last_modified_at: String,
}

impl Default for Tender {
    fn default() -> Self {
        Self {
            id: String::new(),
            reference_number: String::new(),
            title: String::new(),
            description: String::new(),
            tender_type: TenderType::default(),
            status: TenderStatus::default(),
            estimated_value: 0.0,
            currency: "ZAR".to_string(),
            created_at: String::new(),
            publish_date: None,
            closing_date: None,
            briefing_date: None,
            award_date: None,
            category: String::new(),
            department: String::new(),
            cost_center: String::new(),
            delivery_location: String::new(),
            contract_duration: String::new(),
            scope_of_work: String::new(),
            technical_requirements: String::new(),
            mandatory_requirements: Vec::new(),
            evaluation_criteria: Vec::new(),
            price_weight: 80.0,
            bbbee_weight: 20.0,
            functionality_threshold: 70.0,
            documents: Vec::new(),
            bids: Vec::new(),
            portal_reference: None,
            portal_url: None,
            deviation_type: None,
            deviation_justification: None,
            deviation_approved_by: None,
            deviation_approved_at: None,
            created_by: String::new(),
            last_modified_by: String::new(),
            last_modified_at: String::new(),
        }
    }
}

/// Filter criteria for tender list
#[derive(Clone, Debug, Default)]
pub struct TenderFilter {
    pub tender_type: Option<TenderType>,
    pub status: Option<TenderStatus>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub search_query: Option<String>,
    pub department: Option<String>,
    pub category: Option<String>,
}

/// Pagination state
#[derive(Clone, Debug)]
pub struct PaginationState {
    pub current_page: u32,
    pub page_size: u32,
    pub total_items: u32,
    pub total_pages: u32,
}

impl Default for PaginationState {
    fn default() -> Self {
        Self {
            current_page: 1,
            page_size: 10,
            total_items: 0,
            total_pages: 0,
        }
    }
}
