//! AGSA service - API calls

use super::store::{AgsaStore, load_mock_data};
use super::types::{
    AuditFinding, AuditReport, ActionItem, FindingStatus, ActionStatus,
    ActionNote, AgsaComment, CommentType,
};

/// Load all AGSA data
pub async fn load_agsa_data(store: &AgsaStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    // For now, load mock data
    load_mock_data(store);

    store.loading.set(false);
}

/// Load findings list
pub async fn load_findings(store: &AgsaStore) {
    store.loading.set(true);
    store.error.set(None);

    // Ensure data is loaded
    if store.findings.get().is_empty() {
        load_mock_data(store);
    }

    store.loading.set(false);
}

/// Load a single finding by ID
pub async fn load_finding(store: &AgsaStore, finding_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // Ensure findings are loaded
    if store.findings.get().is_empty() {
        load_mock_data(store);
    }

    // Select the finding
    store.select_finding(finding_id);

    store.loading.set(false);
}

/// Load audit reports
pub async fn load_audit_reports(store: &AgsaStore) {
    store.loading.set(true);
    store.error.set(None);

    // Ensure data is loaded
    if store.audit_reports.get().is_empty() {
        load_mock_data(store);
    }

    store.loading.set(false);
}

/// Load a single audit report by ID
pub async fn load_audit_report(store: &AgsaStore, report_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // Ensure reports are loaded
    if store.audit_reports.get().is_empty() {
        load_mock_data(store);
    }

    // Select the report
    store.select_report(report_id);

    store.loading.set(false);
}

/// Update finding status
pub async fn update_finding_status(
    store: &AgsaStore,
    finding_id: &str,
    status: FindingStatus,
) -> Result<(), String> {
    store.loading.set(true);

    let mut findings = store.findings.get().clone();

    if let Some(finding) = findings.iter_mut().find(|f| f.id == finding_id) {
        finding.status = status;
        finding.updated_at = "2025-02-27T12:00:00Z".to_string();

        if matches!(status, FindingStatus::Resolved | FindingStatus::Closed) {
            finding.actual_resolution_date = Some("2025-02-27".to_string());
        }
    } else {
        store.loading.set(false);
        return Err("Finding not found".to_string());
    }

    store.findings.set(findings);

    // Refresh selected finding if applicable
    if let Some(selected) = store.selected_finding.get().as_ref() {
        if selected.id == finding_id {
            store.select_finding(finding_id);
        }
    }

    store.loading.set(false);
    Ok(())
}

/// Update action item status
pub async fn update_action_status(
    store: &AgsaStore,
    action_id: &str,
    status: ActionStatus,
    progress_percent: Option<u8>,
) -> Result<(), String> {
    store.loading.set(true);

    let mut actions = store.action_items.get().clone();

    if let Some(action) = actions.iter_mut().find(|a| a.id == action_id) {
        action.status = status.clone();
        action.updated_at = "2025-02-27T12:00:00Z".to_string();

        if let Some(progress) = progress_percent {
            action.progress_percent = progress;
        }

        if matches!(status, ActionStatus::Completed) {
            action.completion_date = Some("2025-02-27".to_string());
            action.progress_percent = 100;
        }

        if matches!(status, ActionStatus::Verified) {
            action.verification_date = Some("2025-02-27".to_string());
            action.verified_by = Some("AGSA Auditor".to_string());
        }
    } else {
        store.loading.set(false);
        return Err("Action item not found".to_string());
    }

    store.action_items.set(actions);
    store.loading.set(false);
    Ok(())
}

/// Add a note to an action item
pub async fn add_action_note(
    store: &AgsaStore,
    action_id: &str,
    author_id: &str,
    author_name: &str,
    content: &str,
) -> Result<(), String> {
    store.loading.set(true);

    let mut actions = store.action_items.get().clone();

    if let Some(action) = actions.iter_mut().find(|a| a.id == action_id) {
        let note = ActionNote {
            id: format!("note_{:03}", action.notes.len() + 1),
            author_id: author_id.to_string(),
            author_name: author_name.to_string(),
            content: content.to_string(),
            created_at: "2025-02-27T12:00:00Z".to_string(),
        };
        action.notes.push(note);
        action.updated_at = "2025-02-27T12:00:00Z".to_string();
    } else {
        store.loading.set(false);
        return Err("Action item not found".to_string());
    }

    store.action_items.set(actions);
    store.loading.set(false);
    Ok(())
}

/// Add AGSA comment to a finding
pub async fn add_agsa_comment(
    store: &AgsaStore,
    finding_id: &str,
    author: &str,
    content: &str,
    comment_type: CommentType,
) -> Result<(), String> {
    store.loading.set(true);

    let mut findings = store.findings.get().clone();

    if let Some(finding) = findings.iter_mut().find(|f| f.id == finding_id) {
        let comment = AgsaComment {
            id: format!("cmt_{:03}", finding.agsa_comments.len() + 1),
            author: author.to_string(),
            content: content.to_string(),
            comment_type,
            created_at: "2025-02-27T12:00:00Z".to_string(),
        };
        finding.agsa_comments.push(comment);
        finding.updated_at = "2025-02-27T12:00:00Z".to_string();
    } else {
        store.loading.set(false);
        return Err("Finding not found".to_string());
    }

    store.findings.set(findings);

    // Refresh selected finding
    if let Some(selected) = store.selected_finding.get().as_ref() {
        if selected.id == finding_id {
            store.select_finding(finding_id);
        }
    }

    store.loading.set(false);
    Ok(())
}

/// Create a new action item for a finding
pub async fn create_action_item(
    store: &AgsaStore,
    finding_id: &str,
    description: &str,
    assigned_to: &str,
    assigned_department: &str,
    due_date: &str,
    priority: super::types::ActionPriority,
) -> Result<ActionItem, String> {
    store.loading.set(true);

    let mut actions = store.action_items.get().clone();
    let action_count = actions.len();

    let action = ActionItem {
        id: format!("act_{:03}", action_count + 1),
        finding_id: finding_id.to_string(),
        reference_number: format!("ACT/2024-25/{:03}", action_count + 1),
        description: description.to_string(),
        priority,
        status: ActionStatus::NotStarted,
        assigned_to: assigned_to.to_string(),
        assigned_department: assigned_department.to_string(),
        due_date: due_date.to_string(),
        completion_date: None,
        verification_date: None,
        verified_by: None,
        progress_percent: 0,
        notes: Vec::new(),
        evidence_documents: Vec::new(),
        blockers: Vec::new(),
        created_at: "2025-02-27T12:00:00Z".to_string(),
        updated_at: "2025-02-27T12:00:00Z".to_string(),
    };

    actions.push(action.clone());
    store.action_items.set(actions);

    // Also add to the finding's action_items
    let mut findings = store.findings.get().clone();
    if let Some(finding) = findings.iter_mut().find(|f| f.id == finding_id) {
        finding.action_items.push(action.clone());
        finding.updated_at = "2025-02-27T12:00:00Z".to_string();
    }
    store.findings.set(findings);

    store.loading.set(false);
    Ok(action)
}

/// Upload evidence document for an action item
pub async fn upload_action_evidence(
    store: &AgsaStore,
    action_id: &str,
    document_path: &str,
) -> Result<(), String> {
    store.loading.set(true);

    let mut actions = store.action_items.get().clone();

    if let Some(action) = actions.iter_mut().find(|a| a.id == action_id) {
        action.evidence_documents.push(document_path.to_string());
        action.updated_at = "2025-02-27T12:00:00Z".to_string();
    } else {
        store.loading.set(false);
        return Err("Action item not found".to_string());
    }

    store.action_items.set(actions);
    store.loading.set(false);
    Ok(())
}

/// Get finding statistics
pub fn get_finding_stats(store: &AgsaStore) -> FindingStats {
    let findings = store.findings.get();

    let total = findings.len();
    let open = findings.iter().filter(|f| matches!(f.status, FindingStatus::Open)).count();
    let in_progress = findings.iter().filter(|f| matches!(f.status, FindingStatus::InProgress)).count();
    let resolved = findings.iter().filter(|f| matches!(f.status, FindingStatus::Resolved)).count();
    let closed = findings.iter().filter(|f| matches!(f.status, FindingStatus::Closed)).count();
    let overdue = findings.iter().filter(|f| matches!(f.status, FindingStatus::Overdue)).count();
    let recurring = findings.iter().filter(|f| f.is_repeat_finding).count();

    let material = findings.iter().filter(|f| matches!(f.severity, super::types::FindingSeverity::Material)).count();
    let significant = findings.iter().filter(|f| matches!(f.severity, super::types::FindingSeverity::Significant)).count();

    let financial_impact: f64 = findings.iter()
        .filter_map(|f| f.financial_impact)
        .sum();

    FindingStats {
        total,
        open,
        in_progress,
        resolved,
        closed,
        overdue,
        recurring,
        material,
        significant,
        financial_impact,
    }
}

/// Get action item statistics
pub fn get_action_stats(store: &AgsaStore) -> ActionStats {
    let actions = store.action_items.get();

    let total = actions.len();
    let not_started = actions.iter().filter(|a| matches!(a.status, ActionStatus::NotStarted)).count();
    let in_progress = actions.iter().filter(|a| matches!(a.status, ActionStatus::InProgress)).count();
    let completed = actions.iter().filter(|a| matches!(a.status, ActionStatus::Completed)).count();
    let verified = actions.iter().filter(|a| matches!(a.status, ActionStatus::Verified)).count();
    let overdue = actions.iter().filter(|a| matches!(a.status, ActionStatus::Overdue)).count();

    let avg_progress: f64 = if total > 0 {
        actions.iter().map(|a| a.progress_percent as f64).sum::<f64>() / total as f64
    } else {
        0.0
    };

    let critical = actions.iter().filter(|a| matches!(a.priority, super::types::ActionPriority::Critical)).count();
    let high = actions.iter().filter(|a| matches!(a.priority, super::types::ActionPriority::High)).count();

    ActionStats {
        total,
        not_started,
        in_progress,
        completed,
        verified,
        overdue,
        average_progress: avg_progress,
        critical_priority: critical,
        high_priority: high,
    }
}

/// Finding statistics
pub struct FindingStats {
    pub total: usize,
    pub open: usize,
    pub in_progress: usize,
    pub resolved: usize,
    pub closed: usize,
    pub overdue: usize,
    pub recurring: usize,
    pub material: usize,
    pub significant: usize,
    pub financial_impact: f64,
}

/// Action item statistics
pub struct ActionStats {
    pub total: usize,
    pub not_started: usize,
    pub in_progress: usize,
    pub completed: usize,
    pub verified: usize,
    pub overdue: usize,
    pub average_progress: f64,
    pub critical_priority: usize,
    pub high_priority: usize,
}
