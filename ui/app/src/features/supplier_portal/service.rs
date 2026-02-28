//! Supplier Portal service - API calls

use super::store::{
    SupplierPortalStore, load_mock_portal_data,
    get_mock_opportunity, get_mock_submission, get_mock_award,
};
use super::types::{
    TenderOpportunity, BidSubmission, ContractAward, PortalDocument,
    BidSubmissionStatus, OpportunityStatus,
};

/// Load all portal data (dashboard)
pub async fn load_portal_data(store: &SupplierPortalStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    // For now, load mock data
    load_mock_portal_data(store);

    store.loading.set(false);
}

/// Load tender opportunities
pub async fn load_opportunities(store: &SupplierPortalStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API with filters
    // For now, mock data is already loaded
    load_mock_portal_data(store);

    store.loading.set(false);
}

/// Load single opportunity details
pub async fn load_opportunity(store: &SupplierPortalStore, id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    if let Some(opportunity) = get_mock_opportunity(id) {
        store.selected_opportunity.set(Some(opportunity));
    } else {
        store.error.set(Some(format!("Opportunity {} not found", id)));
    }

    store.loading.set(false);
}

/// Load bid submissions
pub async fn load_submissions(store: &SupplierPortalStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    // Mock data is already loaded via load_portal_data
    if store.submissions.get().is_empty() {
        load_mock_portal_data(store);
    }

    store.loading.set(false);
}

/// Load single submission details
pub async fn load_submission(store: &SupplierPortalStore, id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    if let Some(submission) = get_mock_submission(id) {
        store.selected_submission.set(Some(submission));
    } else {
        store.error.set(Some(format!("Submission {} not found", id)));
    }

    store.loading.set(false);
}

/// Create a new bid submission (draft)
pub async fn create_bid_submission(
    store: &SupplierPortalStore,
    tender_id: &str,
) -> Result<String, String> {
    store.saving.set(true);
    store.error.set(None);

    // Validate tender exists and is open - extract needed values upfront
    let opportunities = store.opportunities.get();
    let opportunity = opportunities.iter().find(|o| o.id == tender_id);

    if opportunity.is_none() {
        store.saving.set(false);
        return Err("Tender not found".to_string());
    }

    let opp = opportunity.unwrap();
    if opp.status == OpportunityStatus::Closed {
        store.saving.set(false);
        return Err("Tender is closed for bidding".to_string());
    }

    // Extract values from opportunity before dropping the borrow
    let opp_reference_number = opp.reference_number.clone();
    let opp_title = opp.title.clone();
    let opp_currency = opp.currency.clone();
    let opp_document_count = opp.document_count;

    // Check if already submitted
    let submissions = store.submissions.get();
    if submissions.iter().any(|s| s.tender_id == tender_id && s.status != BidSubmissionStatus::Withdrawn) {
        store.saving.set(false);
        return Err("You have already created a bid for this tender".to_string());
    }

    // Generate new bid ID
    let new_id = format!("BID-2025-{:04}", rand_id());

    // Create draft submission using extracted values
    let new_submission = BidSubmission {
        id: new_id.clone(),
        tender_id: tender_id.to_string(),
        tender_reference: opp_reference_number,
        tender_title: opp_title,
        submitted_at: None,
        total_price: 0.0,
        currency: opp_currency,
        status: BidSubmissionStatus::Draft,
        technical_compliance: None,
        price_score: None,
        total_score: None,
        rank: None,
        documents_uploaded: 0,
        documents_required: opp_document_count,
        notes: None,
        created_at: chrono_now(),
        updated_at: chrono_now(),
    };

    // Add to submissions
    let mut submissions = store.submissions.get();
    submissions.push(new_submission.clone());
    store.submissions.set(submissions);
    store.selected_submission.set(Some(new_submission));

    store.saving.set(false);
    Ok(new_id)
}

/// Update bid submission
pub async fn update_bid_submission(
    store: &SupplierPortalStore,
    submission: BidSubmission,
) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    // Validate
    if submission.id.is_empty() {
        store.saving.set(false);
        return Err("Submission ID is required".to_string());
    }

    // Can only update draft submissions
    let submissions = store.submissions.get();
    let existing = submissions.iter().find(|s| s.id == submission.id);

    if let Some(existing_sub) = existing {
        if existing_sub.status != BidSubmissionStatus::Draft {
            store.saving.set(false);
            return Err("Can only update draft submissions".to_string());
        }
    } else {
        store.saving.set(false);
        return Err("Submission not found".to_string());
    }

    // Update submission
    let mut submissions = store.submissions.get();
    if let Some(idx) = submissions.iter().position(|s| s.id == submission.id) {
        submissions[idx] = submission.clone();
        store.submissions.set(submissions);
        store.selected_submission.set(Some(submission));
    }

    store.saving.set(false);
    Ok(())
}

/// Submit bid for evaluation
pub async fn submit_bid(store: &SupplierPortalStore, submission_id: &str) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    let mut submissions = store.submissions.get();
    let idx = submissions.iter().position(|s| s.id == submission_id);

    if idx.is_none() {
        store.saving.set(false);
        return Err("Submission not found".to_string());
    }

    let idx = idx.unwrap();

    // Extract values we need before mutating submissions
    let tender_id = submissions[idx].tender_id.clone();
    let status = submissions[idx].status.clone();
    let total_price = submissions[idx].total_price;
    let documents_uploaded = submissions[idx].documents_uploaded;
    let documents_required = submissions[idx].documents_required;

    // Validate
    if status != BidSubmissionStatus::Draft {
        store.saving.set(false);
        return Err("Can only submit draft bids".to_string());
    }

    if total_price <= 0.0 {
        store.saving.set(false);
        return Err("Total price must be greater than zero".to_string());
    }

    if documents_uploaded < documents_required {
        store.saving.set(false);
        return Err(format!(
            "Please upload all required documents ({}/{})",
            documents_uploaded, documents_required
        ));
    }

    // Submit
    let mut updated = submissions[idx].clone();
    updated.status = BidSubmissionStatus::Submitted;
    updated.submitted_at = Some(chrono_now());
    updated.updated_at = chrono_now();
    submissions[idx] = updated.clone();

    store.submissions.set(submissions);
    store.selected_submission.set(Some(updated));

    // Update opportunity status
    let mut opportunities = store.opportunities.get();
    if let Some(opp_idx) = opportunities.iter().position(|o| o.id == tender_id) {
        opportunities[opp_idx].status = OpportunityStatus::BidSubmitted;
        store.opportunities.set(opportunities);
    }

    store.saving.set(false);
    Ok(())
}

/// Withdraw bid submission
pub async fn withdraw_bid(store: &SupplierPortalStore, submission_id: &str, reason: &str) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    if reason.is_empty() {
        store.saving.set(false);
        return Err("Withdrawal reason is required".to_string());
    }

    let mut submissions = store.submissions.get();
    let idx = submissions.iter().position(|s| s.id == submission_id);

    if idx.is_none() {
        store.saving.set(false);
        return Err("Submission not found".to_string());
    }

    let idx = idx.unwrap();

    // Extract status before mutating
    let status = submissions[idx].status.clone();

    // Can only withdraw submitted bids that haven't been evaluated
    if status != BidSubmissionStatus::Submitted {
        store.saving.set(false);
        return Err("Can only withdraw submitted bids that are not yet under evaluation".to_string());
    }

    // Withdraw
    let mut updated = submissions[idx].clone();
    updated.status = BidSubmissionStatus::Withdrawn;
    updated.notes = Some(format!("Withdrawn: {}", reason));
    updated.updated_at = chrono_now();
    submissions[idx] = updated.clone();

    store.submissions.set(submissions);
    store.selected_submission.set(Some(updated));

    store.saving.set(false);
    Ok(())
}

/// Load contract awards
pub async fn load_awards(store: &SupplierPortalStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    if store.awards.get().is_empty() {
        load_mock_portal_data(store);
    }

    store.loading.set(false);
}

/// Load single award details
pub async fn load_award(store: &SupplierPortalStore, id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    if let Some(award) = get_mock_award(id) {
        store.selected_award.set(Some(award));
    } else {
        store.error.set(Some(format!("Contract award {} not found", id)));
    }

    store.loading.set(false);
}

/// Upload document for bid submission
pub async fn upload_document(
    store: &SupplierPortalStore,
    submission_id: &str,
    document: PortalDocument,
) -> Result<String, String> {
    store.saving.set(true);
    store.error.set(None);

    // Validate submission exists and is draft
    let submissions = store.submissions.get();
    let submission = submissions.iter().find(|s| s.id == submission_id);

    if submission.is_none() {
        store.saving.set(false);
        return Err("Submission not found".to_string());
    }

    let sub = submission.unwrap();
    if sub.status != BidSubmissionStatus::Draft {
        store.saving.set(false);
        return Err("Can only upload documents to draft submissions".to_string());
    }

    // Validate document
    if document.name.is_empty() {
        store.saving.set(false);
        return Err("Document name is required".to_string());
    }

    // Generate document ID
    let doc_id = format!("DOC-{:06}", rand_id());

    // Create document record
    let mut new_doc = document;
    new_doc.id = doc_id.clone();
    new_doc.uploaded_at = chrono_now();
    new_doc.is_uploaded = true;
    new_doc.reference_id = submission_id.to_string();
    new_doc.reference_type = "bid".to_string();

    // Add to documents
    let mut documents = store.documents.get();
    documents.push(new_doc);
    store.documents.set(documents);

    // Update submission document count
    let mut submissions = store.submissions.get();
    if let Some(idx) = submissions.iter().position(|s| s.id == submission_id) {
        submissions[idx].documents_uploaded += 1;
        submissions[idx].updated_at = chrono_now();
        store.submissions.set(submissions);
    }

    store.saving.set(false);
    Ok(doc_id)
}

/// Delete document from bid submission
pub async fn delete_document(
    store: &SupplierPortalStore,
    document_id: &str,
) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    let documents = store.documents.get();
    let document = documents.iter().find(|d| d.id == document_id);

    if document.is_none() {
        store.saving.set(false);
        return Err("Document not found".to_string());
    }

    let doc = document.unwrap();

    // Check if parent submission is still draft
    if doc.reference_type == "bid" {
        let submissions = store.submissions.get();
        let submission = submissions.iter().find(|s| s.id == doc.reference_id);

        if let Some(sub) = submission {
            if sub.status != BidSubmissionStatus::Draft {
                store.saving.set(false);
                return Err("Cannot delete documents from submitted bids".to_string());
            }
        }
    }

    // Remove document
    let reference_id = doc.reference_id.clone();
    let reference_type = doc.reference_type.clone();
    let mut documents = store.documents.get();
    documents.retain(|d| d.id != document_id);
    store.documents.set(documents);

    // Update submission document count if applicable
    if reference_type == "bid" {
        let mut submissions = store.submissions.get();
        if let Some(idx) = submissions.iter().position(|s| s.id == reference_id) {
            if submissions[idx].documents_uploaded > 0 {
                submissions[idx].documents_uploaded -= 1;
            }
            submissions[idx].updated_at = chrono_now();
            store.submissions.set(submissions);
        }
    }

    store.saving.set(false);
    Ok(())
}

/// Download tender documents
pub async fn download_tender_documents(tender_id: &str) -> Result<Vec<u8>, String> {
    // In production, this would call the API to get the document package
    // For now, return empty bytes
    Ok(Vec::new())
}

/// Download contract document
pub async fn download_contract_document(contract_id: &str, document_id: &str) -> Result<Vec<u8>, String> {
    // In production, this would call the API
    Ok(Vec::new())
}

/// Register for tender briefing
pub async fn register_for_briefing(
    store: &SupplierPortalStore,
    tender_id: &str,
) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    // Validate tender exists
    let opportunities = store.opportunities.get();
    let opportunity = opportunities.iter().find(|o| o.id == tender_id);

    if opportunity.is_none() {
        store.saving.set(false);
        return Err("Tender not found".to_string());
    }

    let opp = opportunity.unwrap();
    if opp.briefing_date.is_none() {
        store.saving.set(false);
        return Err("This tender does not have a briefing session".to_string());
    }

    // In production, this would call the API to register
    // For now, just simulate success

    store.saving.set(false);
    Ok(())
}

/// Mark notifications as read
pub async fn mark_notifications_read(store: &SupplierPortalStore, notification_ids: Vec<String>) {
    for id in notification_ids {
        store.mark_notification_read(&id);
    }
}

/// Refresh portal data
pub async fn refresh_portal_data(store: &SupplierPortalStore) {
    load_portal_data(store).await;
}

// Helper functions
fn rand_id() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    (duration.as_millis() % 1_000_000) as u32
}

fn chrono_now() -> String {
    // In production, use chrono crate
    "2025-02-27T10:00:00Z".to_string()
}
