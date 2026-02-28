//! AI Assistant store

use components::prelude::*;
use super::types::{
    Conversation, Message, MessageRole, MessageMetadata, ResponseType,
    Suggestion, SuggestionCategory, AnalysisResult, ConversationContext,
};

/// AI Assistant state store
#[derive(Clone)]
pub struct AiAssistantStore {
    /// Current active conversation
    pub current_conversation: Signal<Conversation>,
    /// List of past conversations
    pub conversations: Signal<Vec<Conversation>>,
    /// Available quick action suggestions
    pub suggestions: Signal<Vec<Suggestion>>,
    /// Recent analysis results
    pub analyses: Signal<Vec<AnalysisResult>>,
    /// Whether chat panel is open
    pub is_open: Signal<bool>,
    /// Whether AI is processing a request
    pub is_loading: Signal<bool>,
    /// Whether AI is streaming a response
    pub is_streaming: Signal<bool>,
    /// Current input message
    pub input_message: Signal<String>,
    /// Error message if any
    pub error: Signal<Option<String>>,
    /// Conversation context
    pub context: Signal<ConversationContext>,
}

impl AiAssistantStore {
    pub fn new() -> Self {
        Self {
            current_conversation: signal(Conversation::default()),
            conversations: signal(Vec::new()),
            suggestions: signal(Vec::new()),
            analyses: signal(Vec::new()),
            is_open: signal(false),
            is_loading: signal(false),
            is_streaming: signal(false),
            input_message: signal(String::new()),
            error: signal(None),
            context: signal(ConversationContext::General),
        }
    }
}

/// Initialize store with default suggestions
pub fn init_store(store: &AiAssistantStore) {
    load_default_suggestions(store);
    load_conversation_history(store);
}

/// Load default quick action suggestions
pub fn load_default_suggestions(store: &AiAssistantStore) {
    let suggestions = vec![
        // Tender Analysis
        Suggestion {
            id: "sug-001".to_string(),
            label: "Analyze Tender".to_string(),
            prompt: "Analyze the current tender for completeness, compliance, and potential risks.".to_string(),
            icon: "document-search".to_string(),
            category: SuggestionCategory::TenderAnalysis,
        },
        Suggestion {
            id: "sug-002".to_string(),
            label: "Review Specifications".to_string(),
            prompt: "Review the technical specifications for clarity and completeness.".to_string(),
            icon: "clipboard-check".to_string(),
            category: SuggestionCategory::TenderAnalysis,
        },
        // Compliance
        Suggestion {
            id: "sug-003".to_string(),
            label: "Check PFMA Compliance".to_string(),
            prompt: "Check this procurement against PFMA requirements and highlight any compliance gaps.".to_string(),
            icon: "shield-check".to_string(),
            category: SuggestionCategory::Compliance,
        },
        Suggestion {
            id: "sug-004".to_string(),
            label: "Verify B-BBEE".to_string(),
            prompt: "Verify B-BBEE compliance and calculate the preferential points allocation.".to_string(),
            icon: "badge-check".to_string(),
            category: SuggestionCategory::Compliance,
        },
        Suggestion {
            id: "sug-005".to_string(),
            label: "PPPFA Assessment".to_string(),
            prompt: "Assess compliance with the Preferential Procurement Policy Framework Act.".to_string(),
            icon: "scale".to_string(),
            category: SuggestionCategory::Compliance,
        },
        // Bid Evaluation
        Suggestion {
            id: "sug-006".to_string(),
            label: "Compare Bids".to_string(),
            prompt: "Compare all submitted bids and provide a summary of key differences.".to_string(),
            icon: "chart-bar".to_string(),
            category: SuggestionCategory::BidEvaluation,
        },
        Suggestion {
            id: "sug-007".to_string(),
            label: "Evaluate Pricing".to_string(),
            prompt: "Evaluate the pricing proposals and identify any anomalies or concerns.".to_string(),
            icon: "currency-dollar".to_string(),
            category: SuggestionCategory::BidEvaluation,
        },
        Suggestion {
            id: "sug-008".to_string(),
            label: "Score Functionality".to_string(),
            prompt: "Help score the functionality criteria based on the submitted proposals.".to_string(),
            icon: "star".to_string(),
            category: SuggestionCategory::BidEvaluation,
        },
        // Risk Assessment
        Suggestion {
            id: "sug-009".to_string(),
            label: "Assess Supplier Risk".to_string(),
            prompt: "Assess the risk profile of the selected supplier based on available data.".to_string(),
            icon: "exclamation-triangle".to_string(),
            category: SuggestionCategory::RiskAssessment,
        },
        Suggestion {
            id: "sug-010".to_string(),
            label: "Contract Risk Review".to_string(),
            prompt: "Review the contract terms and identify potential risks and mitigation strategies.".to_string(),
            icon: "document-text".to_string(),
            category: SuggestionCategory::RiskAssessment,
        },
        // Regulatory
        Suggestion {
            id: "sug-011".to_string(),
            label: "Treasury Regulations".to_string(),
            prompt: "Explain the relevant National Treasury regulations for this procurement.".to_string(),
            icon: "library".to_string(),
            category: SuggestionCategory::Regulatory,
        },
        Suggestion {
            id: "sug-012".to_string(),
            label: "SCM Policy Check".to_string(),
            prompt: "Check alignment with Supply Chain Management policy requirements.".to_string(),
            icon: "clipboard-list".to_string(),
            category: SuggestionCategory::Regulatory,
        },
        // Process
        Suggestion {
            id: "sug-013".to_string(),
            label: "Deviation Guidance".to_string(),
            prompt: "Provide guidance on the deviation process and required approvals.".to_string(),
            icon: "arrow-right".to_string(),
            category: SuggestionCategory::Process,
        },
        Suggestion {
            id: "sug-014".to_string(),
            label: "NBAC Preparation".to_string(),
            prompt: "Help prepare documentation for the National Bid Adjudication Committee.".to_string(),
            icon: "users".to_string(),
            category: SuggestionCategory::Process,
        },
        Suggestion {
            id: "sug-015".to_string(),
            label: "Award Checklist".to_string(),
            prompt: "Generate a checklist for contract award and supplier onboarding.".to_string(),
            icon: "check-circle".to_string(),
            category: SuggestionCategory::Process,
        },
    ];

    store.suggestions.set(suggestions);
}

/// Load mock conversation history
pub fn load_conversation_history(store: &AiAssistantStore) {
    let conversations = vec![
        Conversation {
            id: "conv-001".to_string(),
            title: "IT Infrastructure Tender Analysis".to_string(),
            messages: vec![
                Message {
                    id: "msg-001".to_string(),
                    role: MessageRole::User,
                    content: "Analyze the IT Infrastructure tender TND-2025-0089 for compliance issues.".to_string(),
                    timestamp: "2025-02-15 09:30".to_string(),
                    metadata: None,
                },
                Message {
                    id: "msg-002".to_string(),
                    role: MessageRole::Assistant,
                    content: "I've analyzed tender TND-2025-0089 (IT Infrastructure Upgrade - Data Centre Modernization). Here's my assessment:\n\n**Compliance Status: Generally Compliant**\n\n**Key Findings:**\n1. All mandatory PFMA requirements are met\n2. B-BBEE requirements clearly specified (Level 1-4 preferred)\n3. Evaluation criteria weights comply with PPPFA (80/20 split)\n\n**Areas of Attention:**\n- Functionality threshold of 70% is appropriate for this complexity\n- Briefing session scheduled - ensure minutes are documented\n- ISO 27001 requirement may limit competition\n\n**Recommendations:**\n1. Consider accepting equivalent international certifications to ISO 27001\n2. Ensure bid validity period aligns with expected evaluation timeline\n3. Document market analysis to justify specifications".to_string(),
                    timestamp: "2025-02-15 09:31".to_string(),
                    metadata: Some(MessageMetadata {
                        response_type: ResponseType::TenderAnalysis,
                        entity_id: Some("TND-2025-0089".to_string()),
                        confidence: Some(92.5),
                        sources: vec![
                            "PFMA Section 38".to_string(),
                            "PPPFA Regulations 2017".to_string(),
                            "NT Instruction Note 3 of 2021".to_string(),
                        ],
                    }),
                },
            ],
            created_at: "2025-02-15".to_string(),
            updated_at: "2025-02-15".to_string(),
            context: ConversationContext::Tender {
                tender_id: "TND-2025-0089".to_string(),
                tender_title: "IT Infrastructure Upgrade".to_string(),
            },
        },
        Conversation {
            id: "conv-002".to_string(),
            title: "B-BBEE Calculation Help".to_string(),
            messages: vec![
                Message {
                    id: "msg-003".to_string(),
                    role: MessageRole::User,
                    content: "How do I calculate preferential points for a B-BBEE Level 2 supplier on an R5 million tender?".to_string(),
                    timestamp: "2025-02-14 14:15".to_string(),
                    metadata: None,
                },
                Message {
                    id: "msg-004".to_string(),
                    role: MessageRole::Assistant,
                    content: "For a tender value of R5 million, the **80/20 preference point system** applies (as the value is below R50 million).\n\n**B-BBEE Points Allocation:**\n- Level 1: 20 points\n- Level 2: 18 points\n- Level 3: 14 points\n- Level 4: 12 points\n- Level 5: 8 points\n- Level 6: 6 points\n- Level 7: 4 points\n- Level 8: 2 points\n- Non-compliant: 0 points\n\n**For your Level 2 supplier: 18 preference points**\n\n**Total Score Calculation:**\n```\nTotal = Price Points (max 80) + B-BBEE Points (18)\n```\n\n**Price Points Formula:**\n```\nPs = 80(1 - (Pt - Pmin)/Pmin)\n```\nWhere Pt = Price of tender under consideration, Pmin = Lowest acceptable tender price\n\nWould you like me to help calculate the complete score once you have the pricing information?".to_string(),
                    timestamp: "2025-02-14 14:16".to_string(),
                    metadata: Some(MessageMetadata {
                        response_type: ResponseType::RegulatoryGuidance,
                        entity_id: None,
                        confidence: Some(98.0),
                        sources: vec![
                            "PPPFA Regulations 2017".to_string(),
                            "B-BBEE Codes of Good Practice".to_string(),
                        ],
                    }),
                },
            ],
            created_at: "2025-02-14".to_string(),
            updated_at: "2025-02-14".to_string(),
            context: ConversationContext::General,
        },
    ];

    store.conversations.set(conversations);
}

/// Start a new conversation
pub fn start_new_conversation(store: &AiAssistantStore, context: ConversationContext) {
    let conv_id = format!("conv-{}", chrono_now());
    let title = match &context {
        ConversationContext::General => "New Conversation".to_string(),
        ConversationContext::Tender { tender_title, .. } => format!("Tender: {}", tender_title),
        ConversationContext::Contract { contract_title, .. } => format!("Contract: {}", contract_title),
        ConversationContext::Supplier { supplier_name, .. } => format!("Supplier: {}", supplier_name),
        ConversationContext::BidEvaluation { tender_id } => format!("Bid Evaluation: {}", tender_id),
    };

    let conversation = Conversation {
        id: conv_id,
        title,
        messages: Vec::new(),
        created_at: chrono_now(),
        updated_at: chrono_now(),
        context: context.clone(),
    };

    store.current_conversation.set(conversation);
    store.context.set(context);
    store.error.set(None);
}

/// Add message to current conversation
pub fn add_message(store: &AiAssistantStore, message: Message) {
    let mut conversation = store.current_conversation.get();
    conversation.messages.push(message);
    conversation.updated_at = chrono_now();
    store.current_conversation.set(conversation);
}

/// Add user message and prepare for response
pub fn send_user_message(store: &AiAssistantStore, content: String) {
    let message = Message {
        id: format!("msg-{}", chrono_now()),
        role: MessageRole::User,
        content,
        timestamp: format_timestamp(),
        metadata: None,
    };
    add_message(store, message);
    store.input_message.set(String::new());
}

/// Add assistant response
pub fn add_assistant_response(store: &AiAssistantStore, content: String, metadata: Option<MessageMetadata>) {
    let message = Message {
        id: format!("msg-{}", chrono_now()),
        role: MessageRole::Assistant,
        content,
        timestamp: format_timestamp(),
        metadata,
    };
    add_message(store, message);
}

/// Toggle chat panel visibility
pub fn toggle_chat(store: &AiAssistantStore) {
    let current = store.is_open.get();
    store.is_open.set(!current);
}

/// Open chat panel
pub fn open_chat(store: &AiAssistantStore) {
    store.is_open.set(true);
}

/// Close chat panel
pub fn close_chat(store: &AiAssistantStore) {
    store.is_open.set(false);
}

/// Set loading state
pub fn set_loading(store: &AiAssistantStore, loading: bool) {
    store.is_loading.set(loading);
}

/// Set streaming state
pub fn set_streaming(store: &AiAssistantStore, streaming: bool) {
    store.is_streaming.set(streaming);
}

/// Set error
pub fn set_error(store: &AiAssistantStore, error: Option<String>) {
    store.error.set(error);
}

/// Select a past conversation
pub fn select_conversation(store: &AiAssistantStore, conversation_id: &str) {
    if let Some(conv) = store.conversations.get().iter().find(|c| c.id == conversation_id).cloned() {
        store.current_conversation.set(conv.clone());
        store.context.set(conv.context);
    }
}

/// Save current conversation to history
pub fn save_conversation(store: &AiAssistantStore) {
    let current = store.current_conversation.get();
    if current.messages.is_empty() {
        return;
    }

    let mut conversations = store.conversations.get();

    // Update if exists, otherwise add
    if let Some(pos) = conversations.iter().position(|c| c.id == current.id) {
        conversations[pos] = current;
    } else {
        conversations.insert(0, current);
    }

    store.conversations.set(conversations);
}

/// Delete a conversation from history
pub fn delete_conversation(store: &AiAssistantStore, conversation_id: &str) {
    let conversations: Vec<_> = store.conversations.get()
        .into_iter()
        .filter(|c| c.id != conversation_id)
        .collect();
    store.conversations.set(conversations);
}

/// Get suggestions by category
pub fn get_suggestions_by_category(store: &AiAssistantStore, category: SuggestionCategory) -> Vec<Suggestion> {
    store.suggestions.get()
        .into_iter()
        .filter(|s| s.category == category)
        .collect()
}

/// Helper: Generate timestamp string
fn chrono_now() -> String {
    // In production, use proper timestamp
    "2025-02-15T10:30:00Z".to_string()
}

/// Helper: Format timestamp for display
fn format_timestamp() -> String {
    "Today, 10:30".to_string()
}
