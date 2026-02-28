//! GRC service - API calls

use super::store::{GrcStore, load_mock_data, select_compliance, select_risk, select_violation, select_control};
use super::types::{
    ComplianceCheck, ComplianceStatus, RiskAssessment, RiskStatus, RiskLevel,
    PolicyViolation, ViolationStatus, ControlStatus, ControlEffectiveness, GrcFilter,
};

/// Load all GRC data
pub async fn load_grc_data(store: &GrcStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API:
    // let response = api::get("/api/grc/dashboard").await;
    // For now, load mock data
    load_mock_data(store);

    store.loading.set(false);
}

/// Load compliance checks with optional filter
pub async fn load_compliance_checks(store: &GrcStore, filter: Option<GrcFilter>) {
    store.loading.set(true);
    store.error.set(None);

    if let Some(f) = filter {
        store.filter.set(f);
    }

    // In production:
    // let response = api::get("/api/grc/compliance", &store.filter.get()).await;
    load_mock_data(store);

    store.loading.set(false);
}

/// Get single compliance check by ID
pub async fn get_compliance_check(store: &GrcStore, compliance_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::get(&format!("/api/grc/compliance/{}", compliance_id)).await;
    select_compliance(store, compliance_id);

    store.loading.set(false);
}

/// Update compliance check status
pub async fn update_compliance_status(
    store: &GrcStore,
    compliance_id: &str,
    status: ComplianceStatus,
    score: f64,
) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::put(&format!("/api/grc/compliance/{}", compliance_id), &{ status, score }).await;

    let mut checks = store.compliance_checks.get();
    if let Some(pos) = checks.iter().position(|c| c.id == compliance_id) {
        checks[pos].status = status;
        checks[pos].score = score;
        store.compliance_checks.set(checks.clone());
        store.selected_compliance.set(Some(checks[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Compliance check not found".to_string()));
        Err("Compliance check not found".to_string())
    }
}

/// Load risk assessments with optional filter
pub async fn load_risk_assessments(store: &GrcStore, filter: Option<GrcFilter>) {
    store.loading.set(true);
    store.error.set(None);

    if let Some(f) = filter {
        store.filter.set(f);
    }

    // In production:
    // let response = api::get("/api/grc/risks", &store.filter.get()).await;
    load_mock_data(store);

    store.loading.set(false);
}

/// Get single risk assessment by ID
pub async fn get_risk_assessment(store: &GrcStore, risk_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::get(&format!("/api/grc/risks/{}", risk_id)).await;
    select_risk(store, risk_id);

    store.loading.set(false);
}

/// Update risk assessment
pub async fn update_risk_assessment(
    store: &GrcStore,
    risk_id: &str,
    residual_risk: RiskLevel,
    residual_score: u8,
    status: RiskStatus,
) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut risks = store.risk_assessments.get();
    if let Some(pos) = risks.iter().position(|r| r.id == risk_id) {
        risks[pos].residual_risk = residual_risk;
        risks[pos].residual_score = residual_score;
        risks[pos].status = status;
        store.risk_assessments.set(risks.clone());
        store.selected_risk.set(Some(risks[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Risk assessment not found".to_string()));
        Err("Risk assessment not found".to_string())
    }
}

/// Create new risk assessment
pub async fn create_risk_assessment(
    store: &GrcStore,
    risk: RiskAssessment,
) -> Result<RiskAssessment, String> {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::post("/api/grc/risks", &risk).await;

    let mut new_risk = risk;
    let count = store.risk_assessments.get().len() + 1;
    new_risk.id = format!("RISK-{:03}", count);

    let mut risks = store.risk_assessments.get();
    risks.push(new_risk.clone());
    store.risk_assessments.set(risks);

    store.loading.set(false);
    Ok(new_risk)
}

/// Load policy violations with optional filter
pub async fn load_policy_violations(store: &GrcStore, filter: Option<GrcFilter>) {
    store.loading.set(true);
    store.error.set(None);

    if let Some(f) = filter {
        store.filter.set(f);
    }

    // In production:
    // let response = api::get("/api/grc/violations", &store.filter.get()).await;
    load_mock_data(store);

    store.loading.set(false);
}

/// Get single policy violation by ID
pub async fn get_policy_violation(store: &GrcStore, violation_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::get(&format!("/api/grc/violations/{}", violation_id)).await;
    select_violation(store, violation_id);

    store.loading.set(false);
}

/// Update policy violation status
pub async fn update_violation_status(
    store: &GrcStore,
    violation_id: &str,
    status: ViolationStatus,
    resolution: Option<String>,
) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut violations = store.policy_violations.get();
    if let Some(pos) = violations.iter().position(|v| v.id == violation_id) {
        violations[pos].status = status;
        if let Some(res) = resolution {
            violations[pos].resolution = Some(res);
            if status == ViolationStatus::Resolved || status == ViolationStatus::Closed {
                violations[pos].resolution_date = Some("2025-02-15".to_string()); // Mock date
            }
        }
        store.policy_violations.set(violations.clone());
        store.selected_violation.set(Some(violations[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Policy violation not found".to_string()));
        Err("Policy violation not found".to_string())
    }
}

/// Report new policy violation
pub async fn report_violation(
    store: &GrcStore,
    violation: PolicyViolation,
) -> Result<PolicyViolation, String> {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::post("/api/grc/violations", &violation).await;

    let mut new_violation = violation;
    let count = store.policy_violations.get().len() + 1;
    new_violation.id = format!("VIO-{:03}", count);
    new_violation.status = ViolationStatus::Open;
    new_violation.detected_date = "2025-02-15".to_string(); // Mock date

    let mut violations = store.policy_violations.get();
    violations.push(new_violation.clone());
    store.policy_violations.set(violations);

    store.loading.set(false);
    Ok(new_violation)
}

/// Load controls with optional filter
pub async fn load_controls(store: &GrcStore, filter: Option<GrcFilter>) {
    store.loading.set(true);
    store.error.set(None);

    if let Some(f) = filter {
        store.filter.set(f);
    }

    // In production:
    // let response = api::get("/api/grc/controls", &store.filter.get()).await;
    load_mock_data(store);

    store.loading.set(false);
}

/// Get single control by ID
pub async fn get_control(store: &GrcStore, control_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::get(&format!("/api/grc/controls/{}", control_id)).await;
    select_control(store, control_id);

    store.loading.set(false);
}

/// Update control effectiveness
pub async fn update_control_effectiveness(
    store: &GrcStore,
    control_id: &str,
    effectiveness: ControlEffectiveness,
    effectiveness_score: u8,
) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut controls = store.controls.get();
    if let Some(pos) = controls.iter().position(|c| c.id == control_id) {
        controls[pos].effectiveness = effectiveness;
        controls[pos].effectiveness_score = effectiveness_score;
        store.controls.set(controls.clone());
        store.selected_control.set(Some(controls[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Control not found".to_string()));
        Err("Control not found".to_string())
    }
}

/// Get high-priority compliance issues
pub async fn get_compliance_issues(store: &GrcStore) {
    let filter = GrcFilter {
        status: Some("non_compliant".to_string()),
        ..Default::default()
    };
    load_compliance_checks(store, Some(filter)).await;
}

/// Get escalated risks
pub async fn get_escalated_risks(store: &GrcStore) {
    let filter = GrcFilter {
        status: Some("escalated".to_string()),
        ..Default::default()
    };
    load_risk_assessments(store, Some(filter)).await;
}

/// Get open violations
pub async fn get_open_violations(store: &GrcStore) {
    let filter = GrcFilter {
        status: Some("open".to_string()),
        ..Default::default()
    };
    load_policy_violations(store, Some(filter)).await;
}

/// Get ineffective controls
pub async fn get_ineffective_controls(store: &GrcStore) {
    let filter = GrcFilter {
        status: Some("ineffective".to_string()),
        ..Default::default()
    };
    load_controls(store, Some(filter)).await;
}

/// Export GRC report
pub async fn export_grc_report(store: &GrcStore, report_type: &str) -> Result<String, String> {
    store.loading.set(true);

    // In production:
    // let response = api::get(&format!("/api/grc/export?type={}", report_type)).await;

    // Mock: return a download URL
    let url = format!("/downloads/grc_report_{}.pdf", report_type);

    store.loading.set(false);
    Ok(url)
}

/// Run compliance assessment
pub async fn run_compliance_assessment(
    store: &GrcStore,
    compliance_id: &str,
) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::post(&format!("/api/grc/compliance/{}/assess", compliance_id), &{}).await;

    // Mock: update last assessed date
    let mut checks = store.compliance_checks.get();
    if let Some(pos) = checks.iter().position(|c| c.id == compliance_id) {
        checks[pos].last_assessed = "2025-02-15".to_string();
        store.compliance_checks.set(checks.clone());
        store.selected_compliance.set(Some(checks[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Compliance check not found".to_string()));
        Err("Compliance check not found".to_string())
    }
}

/// Test control effectiveness
pub async fn test_control(
    store: &GrcStore,
    control_id: &str,
) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::post(&format!("/api/grc/controls/{}/test", control_id), &{}).await;

    // Mock: update last tested date
    let mut controls = store.controls.get();
    if let Some(pos) = controls.iter().position(|c| c.id == control_id) {
        controls[pos].last_tested = "2025-02-15".to_string();
        store.controls.set(controls.clone());
        store.selected_control.set(Some(controls[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Control not found".to_string()));
        Err("Control not found".to_string())
    }
}
