//! Evaluation domain types

use serde::{Deserialize, Serialize};

/// Evaluation status enum
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EvaluationStatus {
    Pending,
    InProgress,
    Completed,
    Approved,
    Rejected,
}

impl EvaluationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            EvaluationStatus::Pending => "pending",
            EvaluationStatus::InProgress => "in_progress",
            EvaluationStatus::Completed => "completed",
            EvaluationStatus::Approved => "approved",
            EvaluationStatus::Rejected => "rejected",
        }
    }
}

/// Committee member for bid evaluation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommitteeMember {
    pub id: String,
    pub name: String,
    pub role: String,
    pub department: String,
    pub has_scored: bool,
    pub conflict_declared: bool,
}

/// Evaluation criterion definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvaluationCriterion {
    pub id: String,
    pub name: String,
    pub description: String,
    pub weight: f64,
    pub max_score: u32,
    pub category: CriterionCategory,
}

/// Criterion categories
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CriterionCategory {
    Technical,
    Financial,
    Experience,
    Bbbee,
    LocalContent,
}

impl CriterionCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            CriterionCategory::Technical => "Technical",
            CriterionCategory::Financial => "Financial",
            CriterionCategory::Experience => "Experience",
            CriterionCategory::Bbbee => "B-BBEE",
            CriterionCategory::LocalContent => "Local Content",
        }
    }
}

/// Individual score for a criterion on a bid
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CriterionScore {
    pub criterion_id: String,
    pub score: u32,
    pub comment: Option<String>,
    pub scored_by: String,
    pub scored_at: String,
}

/// Bid submission for evaluation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bid {
    pub id: String,
    pub supplier_id: String,
    pub supplier_name: String,
    pub bbbee_level: u8,
    pub submitted_at: String,
    pub total_price: f64,
    pub technical_score: Option<f64>,
    pub financial_score: Option<f64>,
    pub total_score: Option<f64>,
    pub scores: Vec<CriterionScore>,
    pub rank: Option<u32>,
    pub recommendation: Option<String>,
}

/// Committee note or comment
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommitteeNote {
    pub id: String,
    pub author_id: String,
    pub author_name: String,
    pub content: String,
    pub created_at: String,
    pub is_confidential: bool,
}

/// Tender evaluation summary
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TenderEvaluation {
    pub id: String,
    pub tender_id: String,
    pub tender_reference: String,
    pub tender_title: String,
    pub tender_value: f64,
    pub closing_date: String,
    pub evaluation_start: Option<String>,
    pub evaluation_deadline: String,
    pub status: EvaluationStatus,
    pub bids: Vec<Bid>,
    pub criteria: Vec<EvaluationCriterion>,
    pub committee_members: Vec<CommitteeMember>,
    pub notes: Vec<CommitteeNote>,
    pub scoring_method: ScoringMethod,
    pub min_technical_score: f64,
    pub price_weight: f64,
    pub functionality_weight: f64,
}

/// Scoring method enum
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ScoringMethod {
    /// 80/20 (80% functionality, 20% price)
    Functionality80Price20,
    /// 90/10 (90% functionality, 10% price)
    Functionality90Price10,
    /// Price only (lowest price wins if min functional score met)
    PriceOnly,
    /// Custom weighted scoring
    Custom,
}

impl ScoringMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            ScoringMethod::Functionality80Price20 => "80/20 (Functionality/Price)",
            ScoringMethod::Functionality90Price10 => "90/10 (Functionality/Price)",
            ScoringMethod::PriceOnly => "Price Only",
            ScoringMethod::Custom => "Custom Weighting",
        }
    }
}

/// Score submission request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScoreSubmission {
    pub evaluation_id: String,
    pub bid_id: String,
    pub scores: Vec<CriterionScore>,
    pub committee_member_id: String,
}

impl Default for TenderEvaluation {
    fn default() -> Self {
        Self {
            id: String::new(),
            tender_id: String::new(),
            tender_reference: String::new(),
            tender_title: String::new(),
            tender_value: 0.0,
            closing_date: String::new(),
            evaluation_start: None,
            evaluation_deadline: String::new(),
            status: EvaluationStatus::Pending,
            bids: Vec::new(),
            criteria: Vec::new(),
            committee_members: Vec::new(),
            notes: Vec::new(),
            scoring_method: ScoringMethod::Functionality80Price20,
            min_technical_score: 70.0,
            price_weight: 20.0,
            functionality_weight: 80.0,
        }
    }
}
