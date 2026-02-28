//! Sourcing Plan service - API calls

use super::store::{SourcingPlanStore, load_mock_data};
use super::types::{SourcingPlan, SourcingPlanStatus, PlanApprovalStep};

/// Load sourcing plans data
pub async fn load_sourcing_plans(store: &SourcingPlanStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    // For now, load mock data
    load_mock_data(store);

    store.loading.set(false);
}

/// Create a new sourcing plan
pub async fn create_sourcing_plan(store: &SourcingPlanStore, mut plan: SourcingPlan) -> Result<SourcingPlan, String> {
    store.loading.set(true);
    store.error.set(None);

    // Generate ID
    let count = store.plans.get().len() + 1;
    let year = plan.fiscal_year.split('/').next().unwrap_or("2025");
    plan.id = format!("SP-{}-{:03}", year, count);
    plan.status = SourcingPlanStatus::Draft;
    plan.created_at = chrono_now();
    plan.updated_at = chrono_now();

    // Calculate totals
    plan.calculate_totals();

    // In production, this would POST to the API
    // For now, add to local store
    let mut plans = store.plans.get();
    plans.insert(0, plan.clone());
    store.plans.set(plans);

    // Update pagination
    let mut pagination = store.pagination.get();
    pagination.update_totals(store.plans.get().len() as u32);
    store.pagination.set(pagination);

    store.loading.set(false);

    Ok(plan)
}

/// Update an existing sourcing plan
pub async fn update_sourcing_plan(store: &SourcingPlanStore, plan: SourcingPlan) -> Result<SourcingPlan, String> {
    store.loading.set(true);
    store.error.set(None);

    let mut updated = plan.clone();
    updated.updated_at = chrono_now();
    updated.calculate_totals();

    // In production, this would PUT to the API
    // For now, update in local store
    let mut plans = store.plans.get();
    if let Some(pos) = plans.iter().position(|p| p.id == updated.id) {
        plans[pos] = updated.clone();
        store.plans.set(plans);
    } else {
        store.loading.set(false);
        store.error.set(Some("Sourcing plan not found".to_string()));
        return Err("Sourcing plan not found".to_string());
    }

    store.loading.set(false);

    Ok(updated)
}

/// Submit a sourcing plan for review
pub async fn submit_for_review(store: &SourcingPlanStore, id: &str) -> Result<SourcingPlan, String> {
    store.loading.set(true);
    store.error.set(None);

    let mut plans = store.plans.get();
    if let Some(pos) = plans.iter().position(|p| p.id == id) {
        if plans[pos].status != SourcingPlanStatus::Draft {
            store.loading.set(false);
            store.error.set(Some("Only draft plans can be submitted for review".to_string()));
            return Err("Only draft plans can be submitted for review".to_string());
        }

        plans[pos].status = SourcingPlanStatus::UnderReview;
        plans[pos].updated_at = chrono_now();

        // Add initial approval workflow step if empty
        if plans[pos].approval_workflow.is_empty() {
            plans[pos].approval_workflow.push(PlanApprovalStep {
                step: 1,
                role: "SCM Manager".to_string(),
                approver: None,
                status: "pending".to_string(),
                date: None,
                comments: None,
            });
            plans[pos].approval_workflow.push(PlanApprovalStep {
                step: 2,
                role: "CFO".to_string(),
                approver: None,
                status: "pending".to_string(),
                date: None,
                comments: None,
            });
            plans[pos].approval_workflow.push(PlanApprovalStep {
                step: 3,
                role: "Accounting Officer".to_string(),
                approver: None,
                status: "pending".to_string(),
                date: None,
                comments: None,
            });
        }

        let updated = plans[pos].clone();
        store.plans.set(plans);
        store.loading.set(false);

        Ok(updated)
    } else {
        store.loading.set(false);
        store.error.set(Some("Sourcing plan not found".to_string()));
        Err("Sourcing plan not found".to_string())
    }
}

/// Approve a sourcing plan
pub async fn approve_plan(store: &SourcingPlanStore, id: &str, approver: &str, comments: Option<String>) -> Result<SourcingPlan, String> {
    store.loading.set(true);
    store.error.set(None);

    let mut plans = store.plans.get();
    if let Some(pos) = plans.iter().position(|p| p.id == id) {
        // Find the next pending approval step
        if let Some(step_idx) = plans[pos].approval_workflow.iter().position(|s| s.status == "pending") {
            plans[pos].approval_workflow[step_idx].status = "approved".to_string();
            plans[pos].approval_workflow[step_idx].approver = Some(approver.to_string());
            plans[pos].approval_workflow[step_idx].date = Some(chrono_now());
            plans[pos].approval_workflow[step_idx].comments = comments;

            // Check if all steps are approved
            let all_approved = plans[pos].approval_workflow.iter().all(|s| s.status == "approved");
            if all_approved {
                plans[pos].status = SourcingPlanStatus::Approved;
                plans[pos].approved_at = Some(chrono_now());
            }
        }

        plans[pos].updated_at = chrono_now();

        let updated = plans[pos].clone();
        store.plans.set(plans);
        store.loading.set(false);

        Ok(updated)
    } else {
        store.loading.set(false);
        store.error.set(Some("Sourcing plan not found".to_string()));
        Err("Sourcing plan not found".to_string())
    }
}

/// Activate an approved sourcing plan
pub async fn activate_plan(store: &SourcingPlanStore, id: &str) -> Result<SourcingPlan, String> {
    store.loading.set(true);
    store.error.set(None);

    let mut plans = store.plans.get();
    if let Some(pos) = plans.iter().position(|p| p.id == id) {
        if plans[pos].status != SourcingPlanStatus::Approved {
            store.loading.set(false);
            store.error.set(Some("Only approved plans can be activated".to_string()));
            return Err("Only approved plans can be activated".to_string());
        }

        plans[pos].status = SourcingPlanStatus::Active;
        plans[pos].updated_at = chrono_now();

        let updated = plans[pos].clone();
        store.plans.set(plans);
        store.loading.set(false);

        Ok(updated)
    } else {
        store.loading.set(false);
        store.error.set(Some("Sourcing plan not found".to_string()));
        Err("Sourcing plan not found".to_string())
    }
}

/// Complete a sourcing plan
pub async fn complete_plan(store: &SourcingPlanStore, id: &str) -> Result<SourcingPlan, String> {
    store.loading.set(true);
    store.error.set(None);

    let mut plans = store.plans.get();
    if let Some(pos) = plans.iter().position(|p| p.id == id) {
        if plans[pos].status != SourcingPlanStatus::Active {
            store.loading.set(false);
            store.error.set(Some("Only active plans can be completed".to_string()));
            return Err("Only active plans can be completed".to_string());
        }

        plans[pos].status = SourcingPlanStatus::Completed;
        plans[pos].updated_at = chrono_now();

        let updated = plans[pos].clone();
        store.plans.set(plans);
        store.loading.set(false);

        Ok(updated)
    } else {
        store.loading.set(false);
        store.error.set(Some("Sourcing plan not found".to_string()));
        Err("Sourcing plan not found".to_string())
    }
}

/// Delete a sourcing plan (only drafts)
pub async fn delete_sourcing_plan(store: &SourcingPlanStore, id: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut plans = store.plans.get();
    if let Some(pos) = plans.iter().position(|p| p.id == id) {
        if plans[pos].status != SourcingPlanStatus::Draft {
            store.loading.set(false);
            store.error.set(Some("Only draft plans can be deleted".to_string()));
            return Err("Only draft plans can be deleted".to_string());
        }

        plans.remove(pos);
        store.plans.set(plans);

        // Update pagination
        let mut pagination = store.pagination.get();
        pagination.update_totals(store.plans.get().len() as u32);
        store.pagination.set(pagination);

        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Sourcing plan not found".to_string()));
        Err("Sourcing plan not found".to_string())
    }
}

/// Get a single sourcing plan by ID
pub async fn get_sourcing_plan(store: &SourcingPlanStore, id: &str) -> Option<SourcingPlan> {
    store.plans.get()
        .iter()
        .find(|p| p.id == id)
        .cloned()
}

/// Clone an existing plan for a new fiscal year
pub async fn clone_plan_for_new_year(store: &SourcingPlanStore, id: &str, new_fiscal_year: &str) -> Result<SourcingPlan, String> {
    store.loading.set(true);
    store.error.set(None);

    let plans = store.plans.get();
    if let Some(source) = plans.iter().find(|p| p.id == id) {
        let count = plans.len() + 1;
        let year = new_fiscal_year.split('/').next().unwrap_or("2025");

        let mut new_plan = source.clone();
        new_plan.id = format!("SP-{}-{:03}", year, count);
        new_plan.fiscal_year = new_fiscal_year.to_string();
        new_plan.status = SourcingPlanStatus::Draft;
        new_plan.created_at = chrono_now();
        new_plan.updated_at = chrono_now();
        new_plan.approved_at = None;
        new_plan.approval_workflow = Vec::new();

        // Reset spent amounts and completed tenders
        for category in &mut new_plan.categories {
            category.spent_amount = 0.0;
            category.completed_tenders = 0;
        }
        new_plan.budget.committed_amount = 0.0;
        new_plan.budget.spent_amount = 0.0;

        // Reset timeline milestones
        for milestone in &mut new_plan.timeline {
            milestone.actual_date = None;
            milestone.status = super::types::MilestoneStatus::Pending;
        }

        // Add to store
        let mut plans = store.plans.get();
        plans.insert(0, new_plan.clone());
        store.plans.set(plans);

        // Update pagination
        let mut pagination = store.pagination.get();
        pagination.update_totals(store.plans.get().len() as u32);
        store.pagination.set(pagination);

        store.loading.set(false);
        Ok(new_plan)
    } else {
        store.loading.set(false);
        store.error.set(Some("Source plan not found".to_string()));
        Err("Source plan not found".to_string())
    }
}

/// Helper to get current timestamp
fn chrono_now() -> String {
    // In production, use chrono crate
    // For now, return a fixed timestamp
    "2025-07-17T16:00:00Z".to_string()
}
