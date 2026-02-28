//! AI Assistant domain types

use serde::{Deserialize, Serialize};

/// Message role in conversation
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageRole {
    /// User message
    User,
    /// AI assistant response
    Assistant,
    /// System instruction (not displayed)
    System,
}

impl MessageRole {
    pub fn label(&self) -> &'static str {
        match self {
            MessageRole::User => "You",
            MessageRole::Assistant => "AI Assistant",
            MessageRole::System => "System",
        }
    }
}

/// Chat message
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: String,
    /// Optional structured data (e.g., analysis results)
    pub metadata: Option<MessageMetadata>,
}

/// Message metadata for structured responses
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageMetadata {
    /// Type of response
    pub response_type: ResponseType,
    /// Related entity ID (tender, contract, etc.)
    pub entity_id: Option<String>,
    /// Confidence score (0-100)
    pub confidence: Option<f64>,
    /// Source references
    pub sources: Vec<String>,
}

/// Type of AI response
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseType {
    /// General conversational response
    General,
    /// Tender analysis
    TenderAnalysis,
    /// Compliance check result
    ComplianceCheck,
    /// Bid comparison
    BidComparison,
    /// Regulatory guidance
    RegulatoryGuidance,
    /// Risk assessment
    RiskAssessment,
    /// Process recommendation
    ProcessRecommendation,
}

impl ResponseType {
    pub fn label(&self) -> &'static str {
        match self {
            ResponseType::General => "General",
            ResponseType::TenderAnalysis => "Tender Analysis",
            ResponseType::ComplianceCheck => "Compliance Check",
            ResponseType::BidComparison => "Bid Comparison",
            ResponseType::RegulatoryGuidance => "Regulatory Guidance",
            ResponseType::RiskAssessment => "Risk Assessment",
            ResponseType::ProcessRecommendation => "Process Recommendation",
        }
    }
}

/// Conversation session
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub messages: Vec<Message>,
    pub created_at: String,
    pub updated_at: String,
    /// Context type for the conversation
    pub context: ConversationContext,
}

impl Default for Conversation {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: "New Conversation".to_string(),
            messages: Vec::new(),
            created_at: String::new(),
            updated_at: String::new(),
            context: ConversationContext::General,
        }
    }
}

/// Conversation context type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConversationContext {
    /// General procurement assistance
    General,
    /// Tender-specific context
    Tender { tender_id: String, tender_title: String },
    /// Contract-specific context
    Contract { contract_id: String, contract_title: String },
    /// Supplier-specific context
    Supplier { supplier_id: String, supplier_name: String },
    /// Bid evaluation context
    BidEvaluation { tender_id: String },
}

impl ConversationContext {
    pub fn label(&self) -> String {
        match self {
            ConversationContext::General => "General Assistance".to_string(),
            ConversationContext::Tender { tender_title, .. } => format!("Tender: {}", tender_title),
            ConversationContext::Contract { contract_title, .. } => format!("Contract: {}", contract_title),
            ConversationContext::Supplier { supplier_name, .. } => format!("Supplier: {}", supplier_name),
            ConversationContext::BidEvaluation { tender_id } => format!("Bid Evaluation: {}", tender_id),
        }
    }
}

/// Quick action suggestion
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Suggestion {
    pub id: String,
    pub label: String,
    pub prompt: String,
    pub icon: String,
    pub category: SuggestionCategory,
}

/// Suggestion category
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum SuggestionCategory {
    TenderAnalysis,
    Compliance,
    BidEvaluation,
    RiskAssessment,
    Regulatory,
    Process,
}

impl SuggestionCategory {
    pub fn label(&self) -> &'static str {
        match self {
            SuggestionCategory::TenderAnalysis => "Tender Analysis",
            SuggestionCategory::Compliance => "Compliance",
            SuggestionCategory::BidEvaluation => "Bid Evaluation",
            SuggestionCategory::RiskAssessment => "Risk Assessment",
            SuggestionCategory::Regulatory => "Regulatory",
            SuggestionCategory::Process => "Process",
        }
    }
}

/// Analysis result from AI
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub id: String,
    pub analysis_type: AnalysisType,
    pub entity_id: String,
    pub entity_type: String,
    pub summary: String,
    pub findings: Vec<Finding>,
    pub recommendations: Vec<Recommendation>,
    pub risk_level: RiskLevel,
    pub compliance_score: Option<f64>,
    pub created_at: String,
}

/// Type of analysis performed
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum AnalysisType {
    /// Full tender document analysis
    TenderDocument,
    /// Compliance verification
    Compliance,
    /// Bid comparison and evaluation
    BidComparison,
    /// Supplier risk assessment
    SupplierRisk,
    /// Contract review
    ContractReview,
    /// Spend analysis
    SpendAnalysis,
}

impl AnalysisType {
    pub fn label(&self) -> &'static str {
        match self {
            AnalysisType::TenderDocument => "Tender Document Analysis",
            AnalysisType::Compliance => "Compliance Check",
            AnalysisType::BidComparison => "Bid Comparison",
            AnalysisType::SupplierRisk => "Supplier Risk Assessment",
            AnalysisType::ContractReview => "Contract Review",
            AnalysisType::SpendAnalysis => "Spend Analysis",
        }
    }
}

/// Analysis finding
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub category: String,
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub reference: Option<String>,
}

/// Severity level
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    pub fn label(&self) -> &'static str {
        match self {
            Severity::Info => "Info",
            Severity::Low => "Low",
            Severity::Medium => "Medium",
            Severity::High => "High",
            Severity::Critical => "Critical",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            Severity::Info => "var(--blue)",
            Severity::Low => "var(--green)",
            Severity::Medium => "var(--orange)",
            Severity::High => "var(--red)",
            Severity::Critical => "var(--red-dark)",
        }
    }
}

/// Recommendation from analysis
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: String,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub action_type: ActionType,
    pub estimated_impact: Option<String>,
}

/// Priority level
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

impl Priority {
    pub fn label(&self) -> &'static str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
            Priority::Urgent => "Urgent",
        }
    }
}

/// Type of recommended action
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ActionType {
    Review,
    Update,
    Escalate,
    Approve,
    Reject,
    Investigate,
    Document,
}

impl ActionType {
    pub fn label(&self) -> &'static str {
        match self {
            ActionType::Review => "Review Required",
            ActionType::Update => "Update Needed",
            ActionType::Escalate => "Escalate",
            ActionType::Approve => "Approve",
            ActionType::Reject => "Reject",
            ActionType::Investigate => "Investigate",
            ActionType::Document => "Document",
        }
    }
}

/// Overall risk level
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl RiskLevel {
    pub fn label(&self) -> &'static str {
        match self {
            RiskLevel::Low => "Low Risk",
            RiskLevel::Medium => "Medium Risk",
            RiskLevel::High => "High Risk",
            RiskLevel::Critical => "Critical Risk",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            RiskLevel::Low => "var(--green)",
            RiskLevel::Medium => "var(--orange)",
            RiskLevel::High => "var(--red)",
            RiskLevel::Critical => "var(--red-dark)",
        }
    }
}

/// Chat completion request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub conversation_id: String,
    pub message: String,
    pub context: ConversationContext,
    /// Include related entity data
    pub include_context_data: bool,
}

/// Chat completion response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub message: Message,
    /// Suggested follow-up questions
    pub suggestions: Vec<String>,
    /// Related analyses if applicable
    pub related_analyses: Vec<String>,
}

/// Analysis request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnalysisRequest {
    pub analysis_type: AnalysisType,
    pub entity_id: String,
    pub entity_type: String,
    /// Additional parameters
    pub parameters: Option<AnalysisParameters>,
}

/// Analysis parameters
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnalysisParameters {
    /// Include regulatory compliance checks
    pub include_compliance: bool,
    /// Specific regulations to check against
    pub regulations: Vec<String>,
    /// Compare against similar entities
    pub include_benchmarks: bool,
    /// Depth of analysis
    pub detail_level: DetailLevel,
}

impl Default for AnalysisParameters {
    fn default() -> Self {
        Self {
            include_compliance: true,
            regulations: vec!["PFMA".to_string(), "PPPFA".to_string(), "B-BBEE".to_string()],
            include_benchmarks: false,
            detail_level: DetailLevel::Standard,
        }
    }
}

/// Level of detail for analysis
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum DetailLevel {
    Summary,
    Standard,
    Detailed,
    Comprehensive,
}

impl DetailLevel {
    pub fn label(&self) -> &'static str {
        match self {
            DetailLevel::Summary => "Summary",
            DetailLevel::Standard => "Standard",
            DetailLevel::Detailed => "Detailed",
            DetailLevel::Comprehensive => "Comprehensive",
        }
    }
}
