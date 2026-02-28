//! Evaluation service - API calls

use super::store::{EvaluationStore, load_mock_data};
use super::types::{ScoreSubmission, CriterionScore};

/// Load all evaluations
pub async fn load_evaluations(store: &EvaluationStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    // For now, load mock data
    load_mock_data(store);

    store.loading.set(false);
}

/// Load a single evaluation by ID
pub async fn load_evaluation(store: &EvaluationStore, evaluation_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // Ensure evaluations are loaded
    if store.evaluations.get().is_empty() {
        load_mock_data(store);
    }

    // Select the evaluation
    store.select_evaluation(evaluation_id);

    store.loading.set(false);
}

/// Submit scores for a bid
pub async fn submit_scores(
    store: &EvaluationStore,
    submission: ScoreSubmission,
) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    // Validate scores
    if submission.scores.is_empty() {
        store.loading.set(false);
        return Err("No scores to submit".to_string());
    }

    // In production, this would POST to the API
    // For now, update local state
    let mut evaluations = store.evaluations.get().clone();

    if let Some(eval) = evaluations.iter_mut().find(|e| e.id == submission.evaluation_id) {
        if let Some(bid) = eval.bids.iter_mut().find(|b| b.id == submission.bid_id) {
            // Update scores
            for new_score in &submission.scores {
                if let Some(existing) = bid.scores.iter_mut()
                    .find(|s| s.criterion_id == new_score.criterion_id && s.scored_by == new_score.scored_by)
                {
                    *existing = new_score.clone();
                } else {
                    bid.scores.push(new_score.clone());
                }
            }

            // Recalculate technical score (simplified)
            let total_weight: f64 = eval.criteria.iter().map(|c| c.weight).sum();
            let weighted_score: f64 = bid.scores.iter()
                .filter_map(|s| {
                    eval.criteria.iter()
                        .find(|c| c.id == s.criterion_id)
                        .map(|c| (s.score as f64 / c.max_score as f64) * c.weight)
                })
                .sum();

            bid.technical_score = Some((weighted_score / total_weight) * 100.0);
        }

        // Update committee member status
        if let Some(member) = eval.committee_members.iter_mut()
            .find(|m| m.id == submission.committee_member_id)
        {
            member.has_scored = true;
        }
    }

    store.evaluations.set(evaluations);

    // Refresh selected evaluation
    if let Some(selected) = store.selected.get().as_ref() {
        store.select_evaluation(&selected.id);
    }

    store.scoring_in_progress.set(false);
    store.loading.set(false);

    Ok(())
}

/// Add a committee note
pub async fn add_committee_note(
    store: &EvaluationStore,
    evaluation_id: &str,
    author_id: &str,
    author_name: &str,
    content: &str,
    is_confidential: bool,
) -> Result<(), String> {
    store.loading.set(true);

    let mut evaluations = store.evaluations.get().clone();

    if let Some(eval) = evaluations.iter_mut().find(|e| e.id == evaluation_id) {
        let note = super::types::CommitteeNote {
            id: format!("note_{:03}", eval.notes.len() + 1),
            author_id: author_id.to_string(),
            author_name: author_name.to_string(),
            content: content.to_string(),
            created_at: "2025-02-27T12:00:00Z".to_string(),
            is_confidential,
        };
        eval.notes.push(note);
    }

    store.evaluations.set(evaluations);

    // Refresh selected evaluation
    if let Some(selected) = store.selected.get().as_ref() {
        store.select_evaluation(&selected.id);
    }

    store.loading.set(false);
    Ok(())
}

/// Finalize evaluation and submit for approval
pub async fn finalize_evaluation(
    store: &EvaluationStore,
    evaluation_id: &str,
) -> Result<(), String> {
    store.loading.set(true);

    let mut evaluations = store.evaluations.get().clone();

    if let Some(eval) = evaluations.iter_mut().find(|e| e.id == evaluation_id) {
        // Check if all committee members have scored
        let all_scored = eval.committee_members.iter()
            .filter(|m| !m.conflict_declared)
            .all(|m| m.has_scored);

        if !all_scored {
            store.loading.set(false);
            return Err("Not all committee members have completed scoring".to_string());
        }

        // Update status to completed
        eval.status = super::types::EvaluationStatus::Completed;

        // Rank bids by total score
        // First, collect bid IDs with their scores to determine ranking
        let mut bid_scores: Vec<(String, f64)> = eval.bids.iter()
            .filter_map(|b| b.total_score.map(|score| (b.id.clone(), score)))
            .collect();
        bid_scores.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Now assign ranks using the sorted bid IDs
        for (i, (bid_id, _)) in bid_scores.iter().enumerate() {
            if let Some(bid) = eval.bids.iter_mut().find(|b| &b.id == bid_id) {
                bid.rank = Some((i + 1) as u32);
            }
        }
    }

    store.evaluations.set(evaluations);

    // Refresh selected evaluation
    if let Some(selected) = store.selected.get().as_ref() {
        store.select_evaluation(&selected.id);
    }

    store.loading.set(false);
    Ok(())
}

/// Calculate scores for display
pub fn calculate_bid_scores(scores: &[CriterionScore], max_score: u32) -> f64 {
    if scores.is_empty() {
        return 0.0;
    }

    let total: u32 = scores.iter().map(|s| s.score).sum();
    let max_total = (max_score as usize * scores.len()) as f64;

    if max_total > 0.0 {
        (total as f64 / max_total) * 100.0
    } else {
        0.0
    }
}
