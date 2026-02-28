//! AI Assistant service - API calls for chat completions

use super::store::{
    AiAssistantStore, set_loading, set_streaming, set_error,
    add_assistant_response, save_conversation,
};
use super::types::{
    ChatCompletionRequest, ChatCompletionResponse, AnalysisRequest,
    AnalysisResult, AnalysisType, AnalysisParameters, Message, MessageRole,
    MessageMetadata, ResponseType, Finding, Recommendation, Severity,
    Priority, ActionType, RiskLevel, ConversationContext, DetailLevel,
};

/// Send a chat completion request
pub async fn send_message(store: &AiAssistantStore, message: String) {
    set_loading(store, true);
    set_error(store, None);

    // In production, this would call the AI API:
    // let request = ChatCompletionRequest {
    //     conversation_id: store.current_conversation.get().id,
    //     message: message.clone(),
    //     context: store.context.get(),
    //     include_context_data: true,
    // };
    // let response = api::post("/api/ai/chat/completions", &request).await;

    // For demo, generate mock response based on message content
    let response = generate_mock_response(&message, &store.context.get());

    add_assistant_response(store, response.message.content, response.message.metadata);
    save_conversation(store);

    set_loading(store, false);
}

/// Stream a chat completion (for long responses)
pub async fn stream_message(store: &AiAssistantStore, message: String) {
    set_loading(store, true);
    set_streaming(store, true);
    set_error(store, None);

    // In production, this would use Server-Sent Events or WebSocket:
    // let stream = api::stream_post("/api/ai/chat/completions/stream", &request).await;
    // while let Some(chunk) = stream.next().await { ... }

    // For demo, simulate streaming delay then add full response
    let response = generate_mock_response(&message, &store.context.get());
    add_assistant_response(store, response.message.content, response.message.metadata);
    save_conversation(store);

    set_streaming(store, false);
    set_loading(store, false);
}

/// Request a specific analysis
pub async fn request_analysis(
    store: &AiAssistantStore,
    analysis_type: AnalysisType,
    entity_id: String,
    entity_type: String,
) -> Result<AnalysisResult, String> {
    set_loading(store, true);
    set_error(store, None);

    // In production:
    // let request = AnalysisRequest {
    //     analysis_type,
    //     entity_id: entity_id.clone(),
    //     entity_type: entity_type.clone(),
    //     parameters: Some(AnalysisParameters::default()),
    // };
    // let response = api::post("/api/ai/analysis", &request).await;

    // Generate mock analysis result
    let result = generate_mock_analysis(analysis_type, &entity_id, &entity_type);

    // Add to analyses list
    let mut analyses = store.analyses.get();
    analyses.insert(0, result.clone());
    store.analyses.set(analyses);

    set_loading(store, false);
    Ok(result)
}

/// Analyze a tender document
pub async fn analyze_tender(store: &AiAssistantStore, tender_id: &str) -> Result<AnalysisResult, String> {
    request_analysis(store, AnalysisType::TenderDocument, tender_id.to_string(), "tender".to_string()).await
}

/// Check compliance for an entity
pub async fn check_compliance(
    store: &AiAssistantStore,
    entity_id: &str,
    entity_type: &str,
) -> Result<AnalysisResult, String> {
    request_analysis(store, AnalysisType::Compliance, entity_id.to_string(), entity_type.to_string()).await
}

/// Compare bids for a tender
pub async fn compare_bids(store: &AiAssistantStore, tender_id: &str) -> Result<AnalysisResult, String> {
    request_analysis(store, AnalysisType::BidComparison, tender_id.to_string(), "tender".to_string()).await
}

/// Assess supplier risk
pub async fn assess_supplier_risk(store: &AiAssistantStore, supplier_id: &str) -> Result<AnalysisResult, String> {
    request_analysis(store, AnalysisType::SupplierRisk, supplier_id.to_string(), "supplier".to_string()).await
}

/// Review a contract
pub async fn review_contract(store: &AiAssistantStore, contract_id: &str) -> Result<AnalysisResult, String> {
    request_analysis(store, AnalysisType::ContractReview, contract_id.to_string(), "contract".to_string()).await
}

/// Get regulatory guidance
pub async fn get_regulatory_guidance(store: &AiAssistantStore, topic: &str) {
    let prompt = format!("Provide guidance on: {}", topic);
    send_message(store, prompt).await;
}

/// Generate mock response based on message content
fn generate_mock_response(message: &str, context: &ConversationContext) -> ChatCompletionResponse {
    let message_lower = message.to_lowercase();

    let (content, response_type, confidence) = if message_lower.contains("analyze") || message_lower.contains("analysis") {
        (
            generate_analysis_response(&message_lower, context),
            ResponseType::TenderAnalysis,
            92.5,
        )
    } else if message_lower.contains("compliance") || message_lower.contains("pfma") || message_lower.contains("pppfa") {
        (
            generate_compliance_response(&message_lower),
            ResponseType::ComplianceCheck,
            95.0,
        )
    } else if message_lower.contains("bbbee") || message_lower.contains("b-bbee") || message_lower.contains("preferential") {
        (
            generate_bbbee_response(&message_lower),
            ResponseType::RegulatoryGuidance,
            98.0,
        )
    } else if message_lower.contains("compare") || message_lower.contains("bid") || message_lower.contains("evaluation") {
        (
            generate_bid_response(&message_lower),
            ResponseType::BidComparison,
            88.0,
        )
    } else if message_lower.contains("risk") {
        (
            generate_risk_response(&message_lower),
            ResponseType::RiskAssessment,
            85.0,
        )
    } else if message_lower.contains("deviation") || message_lower.contains("single source") || message_lower.contains("emergency") {
        (
            generate_deviation_response(&message_lower),
            ResponseType::ProcessRecommendation,
            90.0,
        )
    } else {
        (
            generate_general_response(&message_lower, context),
            ResponseType::General,
            80.0,
        )
    };

    let metadata = if response_type != ResponseType::General {
        Some(MessageMetadata {
            response_type,
            entity_id: extract_entity_id(context),
            confidence: Some(confidence),
            sources: get_relevant_sources(&response_type),
        })
    } else {
        None
    };

    ChatCompletionResponse {
        message: Message {
            id: "msg-response".to_string(),
            role: MessageRole::Assistant,
            content,
            timestamp: "Now".to_string(),
            metadata,
        },
        suggestions: vec![
            "Would you like more details on any specific area?".to_string(),
            "Should I generate a formal report?".to_string(),
        ],
        related_analyses: vec![],
    }
}

fn generate_analysis_response(message: &str, context: &ConversationContext) -> String {
    let tender_ref = match context {
        ConversationContext::Tender { tender_id, tender_title } => {
            format!("{} ({})", tender_id, tender_title)
        }
        _ => "the selected tender".to_string(),
    };

    format!(r#"I've completed the analysis of {}. Here's my assessment:

**Overall Status: Satisfactory with Minor Observations**

**Document Completeness: 95%**
- All mandatory sections present
- Technical specifications well-defined
- Evaluation criteria clearly weighted

**Compliance Assessment:**
- PFMA requirements: Compliant
- PPPFA requirements: Compliant
- Treasury regulations: Compliant
- B-BBEE requirements: Properly specified

**Key Observations:**

1. **Strengths:**
   - Clear scope of work definition
   - Appropriate evaluation methodology
   - Adequate contract duration specified
   - Proper risk allocation in terms

2. **Areas for Improvement:**
   - Consider extending the bid validity period from 90 to 120 days
   - Clarify the penalty clause for late delivery
   - Add specific SLA metrics for service components

3. **Risk Factors:**
   - Single delivery location may limit competition
   - High technical requirements could reduce bidder pool

**Recommendations:**
1. Review and potentially relax the ISO certification requirement
2. Consider site briefing to clarify technical requirements
3. Ensure adequate time for bid preparation (minimum 21 days)

Would you like me to generate a detailed compliance report or focus on any specific aspect?"#, tender_ref)
}

fn generate_compliance_response(message: &str) -> String {
    r#"**Compliance Check Results**

I've assessed the procurement against key regulatory frameworks:

**PFMA Compliance (Public Finance Management Act):**
- Section 38(1)(a)(iii): Appropriate financial controls in place
- Section 51(1)(a)(iii): Proper authorization levels defined
- Section 76(4)(c): Competitive bidding process followed

**PPPFA Compliance (Preferential Procurement Policy Framework Act):**
- Correct preference point system selected (80/20 for values under R50m)
- B-BBEE requirements properly incorporated
- Evaluation criteria weights comply with regulations

**National Treasury Regulations:**
- SCM regulations followed (TR 16A)
- Bid documentation meets minimum requirements
- Proper record-keeping provisions included

**Areas Requiring Attention:**
1. Ensure conflict of interest declarations are obtained from all committee members
2. Verify that the procurement plan was approved before initiation
3. Confirm budget availability certificate is on file

**Compliance Score: 94%**

The procurement is substantially compliant. Minor administrative items should be addressed before proceeding to award.

Would you like me to generate a compliance checklist or provide specific regulatory references?"#.to_string()
}

fn generate_bbbee_response(message: &str) -> String {
    r#"**B-BBEE Preferential Points Guide**

Based on your query, here's the B-BBEE preference point allocation:

**For tenders valued at R50 million or less (80/20 system):**

| B-BBEE Level | Points |
|--------------|--------|
| Level 1 | 20 |
| Level 2 | 18 |
| Level 3 | 14 |
| Level 4 | 12 |
| Level 5 | 8 |
| Level 6 | 6 |
| Level 7 | 4 |
| Level 8 | 2 |
| Non-compliant | 0 |

**For tenders above R50 million (90/10 system):**

| B-BBEE Level | Points |
|--------------|--------|
| Level 1 | 10 |
| Level 2 | 9 |
| Level 3 | 6 |
| Level 4 | 5 |
| Level 5 | 4 |
| Level 6 | 3 |
| Level 7 | 2 |
| Level 8 | 1 |
| Non-compliant | 0 |

**Important Notes:**
1. Only valid B-BBEE certificates or sworn affidavits (for EMEs/QSEs) are acceptable
2. Certificates must be valid at the closing date of the tender
3. Joint ventures must submit a consolidated B-BBEE certificate

**Price Points Formula:**
```
Ps = X(1 - (Pt - Pmin)/Pmin)
```
Where X = 80 (or 90 for larger tenders)

Would you like me to calculate specific scores for your bidders?"#.to_string()
}

fn generate_bid_response(message: &str) -> String {
    r#"**Bid Comparison Summary**

I've analyzed the submitted bids. Here's a comparative overview:

**Bids Received: 3**

| Criterion | Bidder A | Bidder B | Bidder C |
|-----------|----------|----------|----------|
| Price (ZAR) | R14.25M | R15.80M | R16.20M |
| B-BBEE Level | 1 | 2 | 3 |
| Technical Score | 85/100 | 78/100 | 82/100 |
| Experience | 8 years | 12 years | 5 years |
| Local Content | 65% | 72% | 58% |

**Preliminary Point Allocation:**

| Component | Bidder A | Bidder B | Bidder C |
|-----------|----------|----------|----------|
| Price Points (80) | 80.00 | 72.15 | 70.37 |
| B-BBEE Points (20) | 20.00 | 18.00 | 14.00 |
| **Total** | **100.00** | **90.15** | **84.37** |

**Key Observations:**

1. **Bidder A** offers the lowest price and highest B-BBEE rating
2. **Bidder B** has the most experience but higher pricing
3. **Bidder C** shows strong technical approach but limited track record

**Compliance Check:**
- All bidders submitted required documentation
- Bidder C: Tax clearance expires in 30 days - request updated certificate

**Recommendation:**
Based on the 80/20 preference point system, **Bidder A** achieves the highest score. However, verify:
1. Financial capability for a project of this magnitude
2. Reference checks for similar projects
3. Capacity to deliver within required timeframes

Would you like a detailed breakdown of the technical evaluation or price analysis?"#.to_string()
}

fn generate_risk_response(message: &str) -> String {
    r#"**Risk Assessment Summary**

I've conducted a comprehensive risk analysis:

**Overall Risk Level: Medium**

**Risk Categories:**

1. **Financial Risk: Low**
   - Adequate budget allocation confirmed
   - Payment terms are standard (30 days)
   - No currency exposure for local procurement

2. **Supplier Risk: Medium**
   - Financial health indicators are satisfactory
   - Track record shows 2 late deliveries in past 3 years
   - Current workload capacity needs verification

3. **Technical Risk: Low-Medium**
   - Specifications are well-defined
   - Technology is proven and mature
   - Integration complexity requires careful project management

4. **Compliance Risk: Low**
   - All regulatory requirements addressed
   - Proper governance structures in place
   - Audit trail documentation adequate

5. **Schedule Risk: Medium**
   - Timeline is aggressive but achievable
   - Dependencies on third-party inputs identified
   - Recommend including buffer for critical milestones

**Mitigation Recommendations:**

| Risk | Mitigation | Owner |
|------|------------|-------|
| Late delivery | Performance guarantee (10%) | Contract Manager |
| Quality issues | Detailed SLA with penalties | Project Manager |
| Scope creep | Change control process | Project Manager |
| Supplier default | Parent company guarantee | Legal |

**Risk Register Actions:**
1. Establish regular progress monitoring (weekly)
2. Define clear escalation procedures
3. Maintain relationship with alternative suppliers
4. Include termination clauses for material breach

Would you like me to elaborate on any specific risk category or generate a formal risk register?"#.to_string()
}

fn generate_deviation_response(message: &str) -> String {
    r#"**Deviation from Competitive Bidding - Guidance**

A deviation from normal procurement processes requires special justification and approval. Here's what you need to know:

**Types of Deviations:**

1. **Single Source Procurement**
   - Only one supplier can provide the goods/services
   - Proprietary technology or exclusive rights
   - Requires market research evidence

2. **Emergency Procurement**
   - Unforeseen circumstances requiring urgent action
   - Risk of harm to persons, property, or operations
   - Cannot wait for normal bidding process

3. **Sole Supplier**
   - Supplier holds exclusive rights
   - OEM requirements for warranty maintenance
   - No reasonable alternatives exist

**Required Documentation:**

1. Written motivation specifying:
   - Nature of goods/services required
   - Reason why competitive bidding is not possible
   - Market research conducted
   - Risk assessment of proposed approach
   - Value for money analysis

2. Supporting evidence:
   - Supplier capability confirmation
   - Price reasonableness assessment (benchmarks, historical data)
   - B-BBEE status of supplier

**Approval Authority:**

| Value | Approving Authority |
|-------|-------------------|
| Up to R500,000 | CFO |
| R500,001 - R10M | Accounting Officer |
| Above R10M | National Treasury |

**Process Timeline:**
1. Prepare motivation with supporting documents
2. Submit to SCM for review
3. Route through appropriate approval authority
4. Obtain National Treasury concurrence (if required)
5. Document in procurement file

**Important:** All deviations must be reported to National Treasury within 10 working days of approval.

Would you like me to help draft a deviation motivation letter or provide specific guidance for your situation?"#.to_string()
}

fn generate_general_response(message: &str, context: &ConversationContext) -> String {
    let context_info = match context {
        ConversationContext::Tender { tender_title, .. } => {
            format!("I see you're working on the tender: {}. ", tender_title)
        }
        ConversationContext::Contract { contract_title, .. } => {
            format!("I see you're working on the contract: {}. ", contract_title)
        }
        ConversationContext::Supplier { supplier_name, .. } => {
            format!("I see you're reviewing supplier: {}. ", supplier_name)
        }
        ConversationContext::BidEvaluation { tender_id } => {
            format!("I see you're evaluating bids for tender: {}. ", tender_id)
        }
        ConversationContext::General => String::new(),
    };

    format!(r#"{}I'm here to help with your procurement needs. I can assist you with:

**Tender Management:**
- Analyze tender documents for completeness and compliance
- Review specifications and evaluation criteria
- Check regulatory requirements (PFMA, PPPFA, Treasury regulations)

**Bid Evaluation:**
- Compare and score submitted bids
- Calculate preference points (80/20 or 90/10 system)
- Identify compliance issues and anomalies

**Compliance Support:**
- PFMA compliance verification
- B-BBEE requirements and calculations
- SCM policy alignment checks

**Risk Assessment:**
- Supplier risk analysis
- Contract risk review
- Procurement process risks

**Process Guidance:**
- Deviation procedures and approvals
- NBAC preparation support
- Award and contract management

What would you like help with? You can also use the quick action buttons for common tasks."#, context_info)
}

/// Generate mock analysis result
fn generate_mock_analysis(
    analysis_type: AnalysisType,
    entity_id: &str,
    entity_type: &str,
) -> AnalysisResult {
    AnalysisResult {
        id: format!("analysis-{}", entity_id),
        analysis_type,
        entity_id: entity_id.to_string(),
        entity_type: entity_type.to_string(),
        summary: format!("{} completed for {} {}", analysis_type.label(), entity_type, entity_id),
        findings: vec![
            Finding {
                id: "f-001".to_string(),
                category: "Documentation".to_string(),
                title: "Complete documentation set".to_string(),
                description: "All required documents have been submitted and are properly formatted.".to_string(),
                severity: Severity::Info,
                reference: Some("SCM Policy 4.2".to_string()),
            },
            Finding {
                id: "f-002".to_string(),
                category: "Compliance".to_string(),
                title: "Minor compliance observation".to_string(),
                description: "Bid validity period should be extended to match evaluation timeline.".to_string(),
                severity: Severity::Low,
                reference: Some("Treasury Regulation 16A6.3".to_string()),
            },
            Finding {
                id: "f-003".to_string(),
                category: "Risk".to_string(),
                title: "Supplier capacity verification needed".to_string(),
                description: "Recommend verifying supplier capacity for concurrent projects.".to_string(),
                severity: Severity::Medium,
                reference: None,
            },
        ],
        recommendations: vec![
            Recommendation {
                id: "r-001".to_string(),
                title: "Extend bid validity".to_string(),
                description: "Extend the bid validity period from 90 to 120 days to accommodate the evaluation process.".to_string(),
                priority: Priority::Medium,
                action_type: ActionType::Update,
                estimated_impact: Some("Reduces risk of bid expiry during evaluation".to_string()),
            },
            Recommendation {
                id: "r-002".to_string(),
                title: "Conduct reference checks".to_string(),
                description: "Contact provided references to verify supplier performance on similar projects.".to_string(),
                priority: Priority::High,
                action_type: ActionType::Investigate,
                estimated_impact: Some("Confirms supplier capability and reliability".to_string()),
            },
        ],
        risk_level: RiskLevel::Medium,
        compliance_score: Some(94.0),
        created_at: "2025-02-15".to_string(),
    }
}

/// Extract entity ID from context
fn extract_entity_id(context: &ConversationContext) -> Option<String> {
    match context {
        ConversationContext::Tender { tender_id, .. } => Some(tender_id.clone()),
        ConversationContext::Contract { contract_id, .. } => Some(contract_id.clone()),
        ConversationContext::Supplier { supplier_id, .. } => Some(supplier_id.clone()),
        ConversationContext::BidEvaluation { tender_id } => Some(tender_id.clone()),
        ConversationContext::General => None,
    }
}

/// Get relevant regulatory sources based on response type
fn get_relevant_sources(response_type: &ResponseType) -> Vec<String> {
    match response_type {
        ResponseType::TenderAnalysis => vec![
            "PFMA Section 38".to_string(),
            "Treasury Regulation 16A".to_string(),
            "SCM Policy Framework".to_string(),
        ],
        ResponseType::ComplianceCheck => vec![
            "PFMA 1999".to_string(),
            "PPPFA 2000".to_string(),
            "Treasury Regulations 2005".to_string(),
            "NT Instruction Notes".to_string(),
        ],
        ResponseType::BidComparison => vec![
            "PPPFA Regulations 2017".to_string(),
            "B-BBEE Act 2003".to_string(),
            "CIDB Regulations".to_string(),
        ],
        ResponseType::RegulatoryGuidance => vec![
            "PFMA 1999".to_string(),
            "PPPFA 2000".to_string(),
            "B-BBEE Codes of Good Practice".to_string(),
            "National Treasury Guidelines".to_string(),
        ],
        ResponseType::RiskAssessment => vec![
            "King IV Report".to_string(),
            "ISO 31000".to_string(),
            "NT Risk Management Framework".to_string(),
        ],
        ResponseType::ProcessRecommendation => vec![
            "SCM Policy Framework".to_string(),
            "NT Practice Notes".to_string(),
            "PFMA Section 76(4)(c)".to_string(),
        ],
        ResponseType::General => vec![],
    }
}
