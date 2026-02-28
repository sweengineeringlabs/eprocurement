//! NBAC service - API calls

use super::store::{NbacStore, load_mock_data};
use super::types::{
    ReviewItem, Decision, Meeting, ReviewStatus, DecisionType, VoteType, Vote,
    ReviewNote, AgendaItem, AgendaItemStatus,
};

/// Load all NBAC data
pub async fn load_nbac_data(store: &NbacStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    // For now, load mock data
    load_mock_data(store);

    store.loading.set(false);
}

/// Load reviews list
pub async fn load_reviews(store: &NbacStore) {
    store.loading.set(true);
    store.error.set(None);

    // Ensure data is loaded
    if store.reviews.get().is_empty() {
        load_mock_data(store);
    }

    store.loading.set(false);
}

/// Load a single review by ID
pub async fn load_review(store: &NbacStore, review_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // Ensure reviews are loaded
    if store.reviews.get().is_empty() {
        load_mock_data(store);
    }

    // Select the review
    store.select_review(review_id);

    store.loading.set(false);
}

/// Load meetings list
pub async fn load_meetings(store: &NbacStore) {
    store.loading.set(true);
    store.error.set(None);

    // Ensure data is loaded
    if store.meetings.get().is_empty() {
        load_mock_data(store);
    }

    store.loading.set(false);
}

/// Load a single meeting by ID
pub async fn load_meeting(store: &NbacStore, meeting_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // Ensure meetings are loaded
    if store.meetings.get().is_empty() {
        load_mock_data(store);
    }

    // Select the meeting
    store.select_meeting(meeting_id);

    store.loading.set(false);
}

/// Schedule a review for a meeting
pub async fn schedule_review(
    store: &NbacStore,
    review_id: &str,
    meeting_id: &str,
) -> Result<(), String> {
    store.loading.set(true);

    let mut reviews = store.reviews.get().clone();
    let meetings = store.meetings.get();

    // Find the meeting
    let meeting = meetings.iter().find(|m| m.id == meeting_id)
        .ok_or("Meeting not found")?;

    // Update the review
    if let Some(review) = reviews.iter_mut().find(|r| r.id == review_id) {
        review.scheduled_meeting_id = Some(meeting_id.to_string());
        review.scheduled_meeting_date = Some(meeting.date.clone());
        review.status = ReviewStatus::Scheduled;
    } else {
        store.loading.set(false);
        return Err("Review not found".to_string());
    }

    store.reviews.set(reviews);

    // Refresh selected review if applicable
    if let Some(selected) = store.selected_review.get().as_ref() {
        if selected.id == review_id {
            store.select_review(review_id);
        }
    }

    store.loading.set(false);
    Ok(())
}

/// Record a decision for a review
pub async fn record_decision(
    store: &NbacStore,
    review_id: &str,
    decision_type: DecisionType,
    summary: &str,
    rationale: &str,
    votes: Vec<Vote>,
    approved_value: Option<f64>,
    awarded_supplier_id: Option<String>,
    awarded_supplier_name: Option<String>,
    conditions: Vec<String>,
) -> Result<Decision, String> {
    store.loading.set(true);

    let mut reviews = store.reviews.get().clone();
    let mut decisions = store.decisions.get().clone();

    // Calculate vote counts
    let votes_for = votes.iter().filter(|v| matches!(v.vote, VoteType::For)).count() as u32;
    let votes_against = votes.iter().filter(|v| matches!(v.vote, VoteType::Against)).count() as u32;
    let votes_abstain = votes.iter().filter(|v| matches!(v.vote, VoteType::Abstain)).count() as u32;
    let is_unanimous = votes_against == 0 && votes_abstain == 0;

    // Create decision
    let decision = Decision {
        id: format!("dec_{:03}", decisions.len() + 1),
        review_id: review_id.to_string(),
        decision_type,
        decision_date: "2025-02-27".to_string(),
        meeting_id: "mtg_001".to_string(),
        resolution_number: format!("NBAC/RES/2025/{:03}", decisions.len() + 24),
        summary: summary.to_string(),
        rationale: rationale.to_string(),
        conditions,
        votes: votes.clone(),
        votes_for,
        votes_against,
        votes_abstain,
        is_unanimous,
        effective_date: "2025-02-27".to_string(),
        approved_value,
        awarded_supplier_id,
        awarded_supplier_name,
        attachments: Vec::new(),
        recorded_by: "Ms. Sarah van Wyk".to_string(),
        confirmed_by: None,
        confirmed_at: None,
    };

    // Update review status based on decision type
    if let Some(review) = reviews.iter_mut().find(|r| r.id == review_id) {
        review.status = match decision_type {
            DecisionType::Award | DecisionType::Extension | DecisionType::Variation => ReviewStatus::Approved,
            DecisionType::Reject | DecisionType::Cancellation => ReviewStatus::Rejected,
            DecisionType::Defer | DecisionType::RequestInfo => ReviewStatus::Deferred,
        };
        review.decision = Some(decision.clone());
    }

    // Add to decisions list
    decisions.push(decision.clone());

    store.reviews.set(reviews);
    store.decisions.set(decisions);

    // Refresh selected review
    if let Some(selected) = store.selected_review.get().as_ref() {
        if selected.id == review_id {
            store.select_review(review_id);
        }
    }

    store.loading.set(false);
    Ok(decision)
}

/// Add a note to a review
pub async fn add_review_note(
    store: &NbacStore,
    review_id: &str,
    author_id: &str,
    author_name: &str,
    content: &str,
    is_internal: bool,
) -> Result<(), String> {
    store.loading.set(true);

    let mut reviews = store.reviews.get().clone();

    if let Some(review) = reviews.iter_mut().find(|r| r.id == review_id) {
        let note = ReviewNote {
            id: format!("note_{:03}", review.notes.len() + 1),
            author_id: author_id.to_string(),
            author_name: author_name.to_string(),
            content: content.to_string(),
            created_at: "2025-02-27T12:00:00Z".to_string(),
            is_internal,
        };
        review.notes.push(note);
    } else {
        store.loading.set(false);
        return Err("Review not found".to_string());
    }

    store.reviews.set(reviews);

    // Refresh selected review
    if let Some(selected) = store.selected_review.get().as_ref() {
        if selected.id == review_id {
            store.select_review(review_id);
        }
    }

    store.loading.set(false);
    Ok(())
}

/// Update review status
pub async fn update_review_status(
    store: &NbacStore,
    review_id: &str,
    status: ReviewStatus,
) -> Result<(), String> {
    store.loading.set(true);

    let mut reviews = store.reviews.get().clone();

    if let Some(review) = reviews.iter_mut().find(|r| r.id == review_id) {
        review.status = status;
    } else {
        store.loading.set(false);
        return Err("Review not found".to_string());
    }

    store.reviews.set(reviews);

    // Refresh selected review
    if let Some(selected) = store.selected_review.get().as_ref() {
        if selected.id == review_id {
            store.select_review(review_id);
        }
    }

    store.loading.set(false);
    Ok(())
}

/// Update agenda item status
pub async fn update_agenda_item(
    store: &NbacStore,
    meeting_id: &str,
    item_id: &str,
    status: AgendaItemStatus,
    outcome: Option<String>,
    resolution_number: Option<String>,
) -> Result<(), String> {
    store.loading.set(true);

    let mut meetings = store.meetings.get().clone();

    if let Some(meeting) = meetings.iter_mut().find(|m| m.id == meeting_id) {
        if let Some(item) = meeting.agenda.items.iter_mut().find(|i| i.id == item_id) {
            item.status = status;
            item.outcome = outcome;
            item.resolution_number = resolution_number;
        } else {
            store.loading.set(false);
            return Err("Agenda item not found".to_string());
        }
    } else {
        store.loading.set(false);
        return Err("Meeting not found".to_string());
    }

    store.meetings.set(meetings);

    // Refresh selected meeting
    if let Some(selected) = store.selected_meeting.get().as_ref() {
        if selected.id == meeting_id {
            store.select_meeting(meeting_id);
        }
    }

    store.loading.set(false);
    Ok(())
}

/// Confirm decision (chairperson sign-off)
pub async fn confirm_decision(
    store: &NbacStore,
    decision_id: &str,
    confirmed_by: &str,
) -> Result<(), String> {
    store.loading.set(true);

    let mut decisions = store.decisions.get().clone();

    if let Some(decision) = decisions.iter_mut().find(|d| d.id == decision_id) {
        decision.confirmed_by = Some(confirmed_by.to_string());
        decision.confirmed_at = Some("2025-02-27T14:00:00Z".to_string());
    } else {
        store.loading.set(false);
        return Err("Decision not found".to_string());
    }

    // Also update the decision in the review
    let mut reviews = store.reviews.get().clone();
    for review in reviews.iter_mut() {
        if let Some(ref mut dec) = review.decision {
            if dec.id == decision_id {
                dec.confirmed_by = Some(confirmed_by.to_string());
                dec.confirmed_at = Some("2025-02-27T14:00:00Z".to_string());
            }
        }
    }

    store.decisions.set(decisions);
    store.reviews.set(reviews);

    store.loading.set(false);
    Ok(())
}

/// Get decision statistics
pub fn get_decision_stats(store: &NbacStore) -> DecisionStats {
    let decisions = store.decisions.get();

    let total = decisions.len();
    let awards = decisions.iter().filter(|d| matches!(d.decision_type, DecisionType::Award)).count();
    let rejections = decisions.iter().filter(|d| matches!(d.decision_type, DecisionType::Reject)).count();
    let deferrals = decisions.iter().filter(|d| matches!(d.decision_type, DecisionType::Defer)).count();
    let variations = decisions.iter().filter(|d| matches!(d.decision_type, DecisionType::Variation)).count();
    let extensions = decisions.iter().filter(|d| matches!(d.decision_type, DecisionType::Extension)).count();

    let total_value: f64 = decisions.iter()
        .filter_map(|d| d.approved_value)
        .sum();

    let unanimous = decisions.iter().filter(|d| d.is_unanimous).count();

    DecisionStats {
        total,
        awards,
        rejections,
        deferrals,
        variations,
        extensions,
        total_value,
        unanimous_count: unanimous,
        unanimous_rate: if total > 0 { (unanimous as f64 / total as f64) * 100.0 } else { 0.0 },
    }
}

/// Decision statistics
pub struct DecisionStats {
    pub total: usize,
    pub awards: usize,
    pub rejections: usize,
    pub deferrals: usize,
    pub variations: usize,
    pub extensions: usize,
    pub total_value: f64,
    pub unanimous_count: usize,
    pub unanimous_rate: f64,
}
