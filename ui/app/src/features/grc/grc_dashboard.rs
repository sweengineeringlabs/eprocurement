//! GRC Dashboard page showing compliance status, risk matrix, policy violations, control effectiveness

use components::prelude::*;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, data_table, DataTableColumn, DataTableRow,
    status_badge, StatusType,
    tag, TagType,
    kpi_card, KpiColor, KpiDelta,
};
use crate::shared::charts::{bar_chart, BarChartData, pie_chart, PieChartData};
use crate::util::format::format_currency;
use super::store::GrcStore;
use super::types::{
    ComplianceStatus, ComplianceCategory, RiskLevel, RiskCategory, RiskTrend,
    ViolationStatus, ViolationType, Severity, Likelihood, Impact,
    ControlEffectiveness, ControlOperatingStatus,
};
use super::service;

/// GRC Dashboard page
#[component]
pub fn grc_dashboard() -> View {
    let store = use_context::<GrcStore>();

    // Tab state
    let active_tab = signal("overview".to_string());

    // Load data on mount
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                service::load_grc_data(&store).await;
            });
        }
    });

    let kpis = store.kpis.clone();
    let compliance_checks = store.compliance_checks.clone();
    let risk_assessments = store.risk_assessments.clone();
    let policy_violations = store.policy_violations.clone();
    let controls = store.controls.clone();
    let loading = store.loading.clone();

    // Tab handlers
    let set_tab_overview = Callback::<()>::new({
        let active_tab = active_tab.clone();
        move |_| active_tab.set("overview".to_string())
    });
    let set_tab_compliance = Callback::<()>::new({
        let active_tab = active_tab.clone();
        move |_| active_tab.set("compliance".to_string())
    });
    let set_tab_risks = Callback::<()>::new({
        let active_tab = active_tab.clone();
        move |_| active_tab.set("risks".to_string())
    });
    let set_tab_violations = Callback::<()>::new({
        let active_tab = active_tab.clone();
        move |_| active_tab.set("violations".to_string())
    });
    let set_tab_controls = Callback::<()>::new({
        let active_tab = active_tab.clone();
        move |_| active_tab.set("controls".to_string())
    });

    // Get computed values
    let kpi_data = kpis.get();
    let checks = compliance_checks.get();
    let risks = risk_assessments.get();
    let violations = policy_violations.get();
    let ctrl_list = controls.get();

    // Compliance by category for chart
    let compliance_by_category: Vec<BarChartData> = vec![
        BarChartData {
            label: "PFMA".to_string(),
            value: checks.iter().filter(|c| c.category == ComplianceCategory::PFMA && c.status == ComplianceStatus::Compliant).count() as f64,
            color: Some("var(--green)".to_string()),
        },
        BarChartData {
            label: "SCM".to_string(),
            value: checks.iter().filter(|c| c.category == ComplianceCategory::SCM && c.status == ComplianceStatus::Compliant).count() as f64,
            color: Some("var(--green)".to_string()),
        },
        BarChartData {
            label: "B-BBEE".to_string(),
            value: checks.iter().filter(|c| c.category == ComplianceCategory::BBBEE && c.status == ComplianceStatus::Compliant).count() as f64,
            color: Some("var(--green)".to_string()),
        },
        BarChartData {
            label: "Treasury".to_string(),
            value: checks.iter().filter(|c| c.category == ComplianceCategory::Treasury && c.status == ComplianceStatus::Compliant).count() as f64,
            color: Some("var(--green)".to_string()),
        },
        BarChartData {
            label: "POPIA".to_string(),
            value: checks.iter().filter(|c| c.category == ComplianceCategory::DataProtection && c.status == ComplianceStatus::Compliant).count() as f64,
            color: Some("var(--green)".to_string()),
        },
    ];

    // Risk distribution pie chart
    let risk_distribution: Vec<PieChartData> = vec![
        PieChartData { label: "Low".to_string(), value: kpi_data.low_risks as f64, color: "var(--green)".to_string() },
        PieChartData { label: "Medium".to_string(), value: kpi_data.medium_risks as f64, color: "var(--orange)".to_string() },
        PieChartData { label: "High".to_string(), value: kpi_data.high_risks as f64, color: "var(--red)".to_string() },
        PieChartData { label: "Extreme".to_string(), value: kpi_data.extreme_risks as f64, color: "#8b0000".to_string() },
    ];

    // Control effectiveness pie chart
    let control_effectiveness: Vec<PieChartData> = vec![
        PieChartData {
            label: "Effective".to_string(),
            value: ctrl_list.iter().filter(|c| c.effectiveness == ControlEffectiveness::Effective).count() as f64,
            color: "var(--green)".to_string(),
        },
        PieChartData {
            label: "Partially".to_string(),
            value: ctrl_list.iter().filter(|c| c.effectiveness == ControlEffectiveness::PartiallyEffective).count() as f64,
            color: "var(--orange)".to_string(),
        },
        PieChartData {
            label: "Ineffective".to_string(),
            value: ctrl_list.iter().filter(|c| c.effectiveness == ControlEffectiveness::Ineffective).count() as f64,
            color: "var(--red)".to_string(),
        },
        PieChartData {
            label: "Not Tested".to_string(),
            value: ctrl_list.iter().filter(|c| c.effectiveness == ControlEffectiveness::NotTested).count() as f64,
            color: "var(--text-muted)".to_string(),
        },
    ];

    // Violations by type
    let violations_by_type: Vec<BarChartData> = vec![
        BarChartData {
            label: "Threshold".to_string(),
            value: violations.iter().filter(|v| v.violation_type == ViolationType::Threshold).count() as f64,
            color: Some("var(--red)".to_string()),
        },
        BarChartData {
            label: "Conflict".to_string(),
            value: violations.iter().filter(|v| v.violation_type == ViolationType::Conflict).count() as f64,
            color: Some("var(--red)".to_string()),
        },
        BarChartData {
            label: "Documentation".to_string(),
            value: violations.iter().filter(|v| v.violation_type == ViolationType::Documentation).count() as f64,
            color: Some("var(--orange)".to_string()),
        },
        BarChartData {
            label: "Authorization".to_string(),
            value: violations.iter().filter(|v| v.violation_type == ViolationType::Authorization).count() as f64,
            color: Some("var(--orange)".to_string()),
        },
        BarChartData {
            label: "Segregation".to_string(),
            value: violations.iter().filter(|v| v.violation_type == ViolationType::Segregation).count() as f64,
            color: Some("var(--purple)".to_string()),
        },
    ];

    // Compliance table columns
    let compliance_columns = vec![
        DataTableColumn { key: "name".to_string(), label: "Compliance Check".to_string(), width: None, align: None, cell_class: None },
        DataTableColumn { key: "category".to_string(), label: "Category".to_string(), width: Some("100px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "status".to_string(), label: "Status".to_string(), width: Some("130px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "score".to_string(), label: "Score".to_string(), width: Some("80px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "next_review".to_string(), label: "Next Review".to_string(), width: Some("110px".to_string()), align: None, cell_class: None },
    ];

    // Transform compliance to table rows
    let compliance_rows: Vec<DataTableRow> = checks.iter().map(|check| {
        let status = match check.status {
            ComplianceStatus::Compliant => status_badge(StatusType::Approved),
            ComplianceStatus::NonCompliant => status_badge(StatusType::Rejected),
            ComplianceStatus::PartiallyCompliant => status_badge(StatusType::Pending),
            ComplianceStatus::PendingReview => status_badge(StatusType::Evaluation),
            ComplianceStatus::NotApplicable => status_badge(StatusType::Draft),
        };

        let category_tag = match check.category {
            ComplianceCategory::PFMA => tag("PFMA".to_string(), TagType::Blue),
            ComplianceCategory::SCM => tag("SCM".to_string(), TagType::Green),
            ComplianceCategory::BBBEE => tag("B-BBEE".to_string(), TagType::Orange),
            ComplianceCategory::Treasury => tag("Treasury".to_string(), TagType::Purple),
            ComplianceCategory::DataProtection => tag("POPIA".to_string(), TagType::Cyan),
            _ => tag(check.category.label().to_string(), TagType::Default),
        };

        let score_color = if check.score >= 80.0 {
            "var(--green)"
        } else if check.score >= 60.0 {
            "var(--orange)"
        } else if check.score > 0.0 {
            "var(--red)"
        } else {
            "var(--text-muted)"
        };

        // Pre-compute score text before view! block
        let score_text = if check.score > 0.0 {
            format!("{:.0}%", check.score)
        } else {
            "-".to_string()
        };

        DataTableRow {
            id: check.id.clone(),
            cells: vec![
                view! {
                    <div class="check-info">
                        <span class="check-name">{check.name.clone()}</span>
                        <span class="check-regulation">{check.regulation.clone()}</span>
                    </div>
                },
                category_tag,
                status,
                view! {
                    <span class="score-value" style={format!("color: {}", score_color)}>
                        {score_text}
                    </span>
                },
                view! { <span class="date-cell">{check.next_review.clone()}</span> },
            ],
        }
    }).collect();

    // Risk table columns
    let risk_columns = vec![
        DataTableColumn { key: "name".to_string(), label: "Risk".to_string(), width: None, align: None, cell_class: None },
        DataTableColumn { key: "category".to_string(), label: "Category".to_string(), width: Some("100px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "inherent".to_string(), label: "Inherent".to_string(), width: Some("90px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "residual".to_string(), label: "Residual".to_string(), width: Some("90px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "trend".to_string(), label: "Trend".to_string(), width: Some("90px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "owner".to_string(), label: "Owner".to_string(), width: Some("130px".to_string()), align: None, cell_class: None },
    ];

    // Transform risks to table rows
    let risk_rows: Vec<DataTableRow> = risks.iter().map(|risk| {
        let category_tag = match risk.risk_category {
            RiskCategory::Financial => tag("Financial".to_string(), TagType::Blue),
            RiskCategory::Fraud => tag("Fraud".to_string(), TagType::Red),
            RiskCategory::Compliance => tag("Compliance".to_string(), TagType::Orange),
            RiskCategory::SupplyChain => tag("Supply Chain".to_string(), TagType::Green),
            RiskCategory::Cybersecurity => tag("Cyber".to_string(), TagType::Purple),
            RiskCategory::Reputational => tag("Reputation".to_string(), TagType::Cyan),
            _ => tag(risk.risk_category.label().to_string(), TagType::Default),
        };

        let inherent_tag = match risk.inherent_risk {
            RiskLevel::Extreme => tag("Extreme".to_string(), TagType::Red),
            RiskLevel::High => tag("High".to_string(), TagType::Red),
            RiskLevel::Medium => tag("Medium".to_string(), TagType::Orange),
            RiskLevel::Low => tag("Low".to_string(), TagType::Green),
        };

        let residual_tag = match risk.residual_risk {
            RiskLevel::Extreme => tag("Extreme".to_string(), TagType::Red),
            RiskLevel::High => tag("High".to_string(), TagType::Red),
            RiskLevel::Medium => tag("Medium".to_string(), TagType::Orange),
            RiskLevel::Low => tag("Low".to_string(), TagType::Green),
        };

        let trend_icon = match risk.trend {
            RiskTrend::Increasing => view! { <span class="trend-up" title="Increasing">"↑"</span> },
            RiskTrend::Stable => view! { <span class="trend-stable" title="Stable">"→"</span> },
            RiskTrend::Decreasing => view! { <span class="trend-down" title="Decreasing">"↓"</span> },
        };

        DataTableRow {
            id: risk.id.clone(),
            cells: vec![
                view! {
                    <div class="risk-info">
                        <span class="risk-name">{risk.name.clone()}</span>
                        <span class="risk-desc">{risk.description.chars().take(60).collect::<String>()}{"..."}</span>
                    </div>
                },
                category_tag,
                inherent_tag,
                residual_tag,
                trend_icon,
                view! { <span class="owner-cell">{risk.owner.clone()}</span> },
            ],
        }
    }).collect();

    // Violations table columns
    let violation_columns = vec![
        DataTableColumn { key: "id".to_string(), label: "ID".to_string(), width: Some("90px".to_string()), align: None, cell_class: Some("id-cell".to_string()) },
        DataTableColumn { key: "policy".to_string(), label: "Policy".to_string(), width: None, align: None, cell_class: None },
        DataTableColumn { key: "type".to_string(), label: "Type".to_string(), width: Some("100px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "severity".to_string(), label: "Severity".to_string(), width: Some("90px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "status".to_string(), label: "Status".to_string(), width: Some("120px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "impact".to_string(), label: "Financial Impact".to_string(), width: Some("130px".to_string()), align: Some("right".to_string()), cell_class: None },
    ];

    // Transform violations to table rows
    let violation_rows: Vec<DataTableRow> = violations.iter().map(|violation| {
        let type_tag = match violation.violation_type {
            ViolationType::Threshold => tag("Threshold".to_string(), TagType::Red),
            ViolationType::Conflict => tag("Conflict".to_string(), TagType::Red),
            ViolationType::Documentation => tag("Docs".to_string(), TagType::Orange),
            ViolationType::Authorization => tag("Auth".to_string(), TagType::Orange),
            ViolationType::Segregation => tag("SoD".to_string(), TagType::Purple),
            _ => tag(violation.violation_type.label().to_string(), TagType::Default),
        };

        let severity_tag = match violation.severity {
            Severity::Critical => tag("Critical".to_string(), TagType::Red),
            Severity::High => tag("High".to_string(), TagType::Red),
            Severity::Medium => tag("Medium".to_string(), TagType::Orange),
            Severity::Low => tag("Low".to_string(), TagType::Blue),
            Severity::Informational => tag("Info".to_string(), TagType::Default),
        };

        let status = match violation.status {
            ViolationStatus::Open => status_badge(StatusType::Pending),
            ViolationStatus::UnderInvestigation => status_badge(StatusType::Evaluation),
            ViolationStatus::PendingAction => status_badge(StatusType::Pending),
            ViolationStatus::Resolved => status_badge(StatusType::Approved),
            ViolationStatus::Closed => status_badge(StatusType::Complete),
            ViolationStatus::Escalated => status_badge(StatusType::Rejected),
        };

        DataTableRow {
            id: violation.id.clone(),
            cells: vec![
                view! { <span class="id-cell">{violation.id.clone()}</span> },
                view! {
                    <div class="violation-info">
                        <span class="violation-policy">{violation.policy_name.clone()}</span>
                        <span class="violation-entity">{format!("{}: {}", violation.entity_type.label(), violation.affected_entity.clone())}</span>
                    </div>
                },
                type_tag,
                severity_tag,
                status,
                view! {
                    <span class="amount-cell">
                        {violation.financial_impact.map(|i| format_currency(i)).unwrap_or_else(|| "-".to_string())}
                    </span>
                },
            ],
        }
    }).collect();

    // Controls table columns
    let control_columns = vec![
        DataTableColumn { key: "name".to_string(), label: "Control".to_string(), width: None, align: None, cell_class: None },
        DataTableColumn { key: "type".to_string(), label: "Type".to_string(), width: Some("100px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "effectiveness".to_string(), label: "Effectiveness".to_string(), width: Some("130px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "score".to_string(), label: "Score".to_string(), width: Some("80px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "status".to_string(), label: "Operating".to_string(), width: Some("120px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "last_tested".to_string(), label: "Last Test".to_string(), width: Some("100px".to_string()), align: None, cell_class: None },
    ];

    // Transform controls to table rows
    let control_rows: Vec<DataTableRow> = ctrl_list.iter().map(|control| {
        let type_tag = match control.control_type {
            crate::features::grc::types::ControlType::Preventive => tag("Preventive".to_string(), TagType::Green),
            crate::features::grc::types::ControlType::Detective => tag("Detective".to_string(), TagType::Blue),
            crate::features::grc::types::ControlType::Corrective => tag("Corrective".to_string(), TagType::Orange),
            crate::features::grc::types::ControlType::Directive => tag("Directive".to_string(), TagType::Purple),
        };

        let effectiveness_tag = match control.effectiveness {
            ControlEffectiveness::Effective => tag("Effective".to_string(), TagType::Green),
            ControlEffectiveness::PartiallyEffective => tag("Partial".to_string(), TagType::Orange),
            ControlEffectiveness::Ineffective => tag("Ineffective".to_string(), TagType::Red),
            ControlEffectiveness::NotTested => tag("Not Tested".to_string(), TagType::Default),
        };

        let status = match control.status {
            ControlOperatingStatus::Operating => status_badge(StatusType::Active),
            ControlOperatingStatus::PartiallyOperating => status_badge(StatusType::Pending),
            ControlOperatingStatus::NotOperating => status_badge(StatusType::Rejected),
            ControlOperatingStatus::UnderReview => status_badge(StatusType::Evaluation),
            ControlOperatingStatus::Retired => status_badge(StatusType::Draft),
        };

        let score_color = if control.effectiveness_score >= 80 {
            "var(--green)"
        } else if control.effectiveness_score >= 60 {
            "var(--orange)"
        } else {
            "var(--red)"
        };

        DataTableRow {
            id: control.id.clone(),
            cells: vec![
                view! {
                    <div class="control-info">
                        <span class="control-name">{control.name.clone()}</span>
                        <span class="control-owner">{format!("Owner: {}", control.owner.clone())}</span>
                    </div>
                },
                type_tag,
                effectiveness_tag,
                view! {
                    <span class="score-value" style={format!("color: {}", score_color)}>
                        {format!("{}%", control.effectiveness_score)}
                    </span>
                },
                status,
                view! { <span class="date-cell">{control.last_tested.clone()}</span> },
            ],
        }
    }).collect();

    // Icons
    let icon_shield = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>"#;
    let icon_alert = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>"#;
    let icon_check = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>"#;
    let icon_settings = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>"#;
    let icon_warning = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>"#;

    // Pre-compute values used in the view! block
    let resolved_violations_count = violations.iter()
        .filter(|v| v.status == ViolationStatus::Resolved || v.status == ViolationStatus::Closed)
        .count();
    let partially_effective_controls_count = ctrl_list.iter()
        .filter(|c| c.effectiveness == ControlEffectiveness::PartiallyEffective)
        .count();

    // Pre-compute KPI conditional values
    let compliance_kpi_color = if kpi_data.compliance_score >= 80.0 {
        KpiColor::Green
    } else if kpi_data.compliance_score >= 60.0 {
        KpiColor::Orange
    } else {
        KpiColor::Red
    };

    let high_extreme_risks = kpi_data.high_risks + kpi_data.extreme_risks;
    let risks_kpi_color = if high_extreme_risks == 0 { KpiColor::Green } else { KpiColor::Orange };
    let risks_is_positive = if high_extreme_risks > 0 { Some(false) } else { None };
    let high_extreme_risks_str = format!("{} high/extreme", high_extreme_risks);

    let violations_kpi_color = if kpi_data.critical_violations > 0 {
        KpiColor::Red
    } else if kpi_data.open_violations > 0 {
        KpiColor::Orange
    } else {
        KpiColor::Green
    };
    let violations_is_positive = if kpi_data.critical_violations > 0 { Some(false) } else { None };

    let controls_kpi_color = if kpi_data.control_coverage >= 80.0 {
        KpiColor::Green
    } else if kpi_data.control_coverage >= 60.0 {
        KpiColor::Orange
    } else {
        KpiColor::Red
    };
    let controls_is_positive = if kpi_data.ineffective_controls > 0 { Some(false) } else { None };

    view! {
        style {
            r#"
            .grc-dashboard { display: flex; flex-direction: column; gap: var(--space-4); }
            .kpi-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; }
            @media (max-width: 1200px) { .kpi-grid { grid-template-columns: repeat(2, 1fr); } }
            @media (max-width: 768px) { .kpi-grid { grid-template-columns: 1fr; } }
            .charts-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px; }
            @media (max-width: 1200px) { .charts-grid { grid-template-columns: 1fr 1fr; } }
            @media (max-width: 768px) { .charts-grid { grid-template-columns: 1fr; } }
            .tab-bar {
                display: flex;
                gap: 4px;
                border-bottom: 1px solid var(--border);
                padding: 0 4px;
                margin-bottom: 16px;
            }
            .tab-button {
                padding: 12px 20px;
                border: none;
                background: transparent;
                color: var(--text-muted);
                font-size: 14px;
                font-weight: 500;
                cursor: pointer;
                border-bottom: 2px solid transparent;
                margin-bottom: -1px;
                transition: all 0.2s;
            }
            .tab-button:hover {
                color: var(--text);
                background: var(--bg);
            }
            .tab-button.active {
                color: var(--blue);
                border-bottom-color: var(--blue);
            }
            .check-info, .risk-info, .violation-info, .control-info {
                display: flex;
                flex-direction: column;
                gap: 2px;
            }
            .check-name, .risk-name, .violation-policy, .control-name {
                font-weight: 500;
                color: var(--text);
            }
            .check-regulation, .risk-desc, .violation-entity, .control-owner {
                font-size: 11px;
                color: var(--text-muted);
            }
            .score-value {
                font-weight: 600;
                font-family: IBM Plex Mono, monospace;
            }
            .date-cell, .owner-cell {
                font-size: 12px;
                color: var(--text-muted);
            }
            .id-cell {
                font-family: IBM Plex Mono, monospace;
                font-size: 12px;
            }
            .amount-cell {
                font-family: IBM Plex Mono, monospace;
            }
            .trend-up { color: var(--red); font-size: 18px; }
            .trend-stable { color: var(--orange); font-size: 18px; }
            .trend-down { color: var(--green); font-size: 18px; }
            .loading-overlay {
                display: flex;
                align-items: center;
                justify-content: center;
                padding: 40px;
                color: var(--text-muted);
            }
            .risk-matrix {
                display: grid;
                grid-template-columns: auto repeat(5, 1fr);
                gap: 2px;
                background: var(--border);
                border: 1px solid var(--border);
                border-radius: var(--radius);
                overflow: hidden;
            }
            .matrix-header {
                background: var(--bg);
                padding: 8px;
                font-size: 11px;
                font-weight: 500;
                text-align: center;
                color: var(--text-muted);
            }
            .matrix-row-header {
                background: var(--bg);
                padding: 8px;
                font-size: 11px;
                font-weight: 500;
                text-align: right;
                color: var(--text-muted);
            }
            .matrix-cell {
                background: var(--surface);
                padding: 12px;
                text-align: center;
                font-weight: 600;
                font-size: 14px;
            }
            .matrix-cell.extreme { background: #dc143c; color: white; }
            .matrix-cell.high { background: #ff6b6b; color: white; }
            .matrix-cell.medium { background: #ffd93d; color: #333; }
            .matrix-cell.low { background: #6bcb77; color: white; }
            .critical-alerts {
                display: flex;
                flex-direction: column;
                gap: 12px;
            }
            .alert-item {
                display: flex;
                align-items: center;
                gap: 12px;
                padding: 12px;
                background: #fff5f5;
                border: 1px solid #ffcccc;
                border-radius: var(--radius);
            }
            .alert-icon {
                width: 32px;
                height: 32px;
                border-radius: 50%;
                background: var(--red);
                color: white;
                display: flex;
                align-items: center;
                justify-content: center;
            }
            .alert-icon svg { width: 16px; height: 16px; }
            .alert-content { flex: 1; }
            .alert-title { font-weight: 500; color: var(--red); }
            .alert-desc { font-size: 12px; color: var(--text-muted); }
            .compliance-score-card {
                display: flex;
                flex-direction: column;
                align-items: center;
                padding: 24px;
                text-align: center;
            }
            .score-circle {
                width: 120px;
                height: 120px;
                border-radius: 50%;
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                font-size: 32px;
                font-weight: 700;
                margin-bottom: 12px;
            }
            .score-circle.good { background: var(--green-light); color: var(--green); border: 3px solid var(--green); }
            .score-circle.warning { background: var(--orange-light); color: var(--orange); border: 3px solid var(--orange); }
            .score-circle.bad { background: var(--red-light); color: var(--red); border: 3px solid var(--red); }
            .score-label { font-size: 14px; color: var(--text-muted); }
            "#
        }

        <div class="grc-dashboard" data-testid="grc-dashboard">
            {page_header(
                "GRC Dashboard".to_string(),
                Some("Governance, Risk Management, and Compliance Overview".to_string()),
                vec![
                    view! { <button class="btn btn-secondary">"Export Report"</button> },
                    view! { <button class="btn btn-primary">"Run Assessment"</button> },
                ]
            )}

            // Tab navigation
            <div class="tab-bar">
                <button
                    class={if active_tab.get() == "overview" { "tab-button active" } else { "tab-button" }}
                    on:click={set_tab_overview}
                >"Overview"</button>
                <button
                    class={if active_tab.get() == "compliance" { "tab-button active" } else { "tab-button" }}
                    on:click={set_tab_compliance}
                >"Compliance"</button>
                <button
                    class={if active_tab.get() == "risks" { "tab-button active" } else { "tab-button" }}
                    on:click={set_tab_risks}
                >"Risks"</button>
                <button
                    class={if active_tab.get() == "violations" { "tab-button active" } else { "tab-button" }}
                    on:click={set_tab_violations}
                >"Violations"</button>
                <button
                    class={if active_tab.get() == "controls" { "tab-button active" } else { "tab-button" }}
                    on:click={set_tab_controls}
                >"Controls"</button>
            </div>

            // Overview tab content
            if active_tab.get() == "overview" {
                <div class="tab-content">
                    // KPI summary
                    <div class="kpi-grid">
                        {kpi_card(
                            "Compliance Score".to_string(),
                            format!("{:.0}%", kpi_data.compliance_score),
                            compliance_kpi_color,
                            icon_check.to_string(),
                            Some(KpiDelta {
                                value: format!("{} checks", kpi_data.total_checks),
                                is_positive: None,
                                suffix: "assessed".to_string(),
                            }),
                            None
                        )}
                        {kpi_card(
                            "Active Risks".to_string(),
                            format!("{}", kpi_data.total_risks),
                            risks_kpi_color,
                            icon_alert.to_string(),
                            Some(KpiDelta {
                                value: high_extreme_risks_str.clone(),
                                is_positive: risks_is_positive,
                                suffix: "".to_string(),
                            }),
                            None
                        )}
                        {kpi_card(
                            "Open Violations".to_string(),
                            format!("{}", kpi_data.open_violations),
                            violations_kpi_color,
                            icon_warning.to_string(),
                            Some(KpiDelta {
                                value: format!("{} critical", kpi_data.critical_violations),
                                is_positive: violations_is_positive,
                                suffix: "".to_string(),
                            }),
                            None
                        )}
                        {kpi_card(
                            "Control Effectiveness".to_string(),
                            format!("{:.0}%", kpi_data.control_coverage),
                            controls_kpi_color,
                            icon_settings.to_string(),
                            Some(KpiDelta {
                                value: format!("{} ineffective", kpi_data.ineffective_controls),
                                is_positive: controls_is_positive,
                                suffix: "".to_string(),
                            }),
                            None
                        )}
                    </div>

                    // Charts row
                    <div class="charts-grid">
                        // Risk Distribution
                        {panel(
                            "Risk Distribution".to_string(),
                            vec![],
                            vec![pie_chart(risk_distribution.clone(), None)]
                        )}

                        // Control Effectiveness
                        {panel(
                            "Control Effectiveness".to_string(),
                            vec![],
                            vec![pie_chart(control_effectiveness.clone(), None)]
                        )}

                        // Violations by Type
                        {panel(
                            "Violations by Type".to_string(),
                            vec![],
                            vec![bar_chart(violations_by_type.clone(), Some(180))]
                        )}
                    </div>

                    // Risk Matrix
                    <div class="grid-2">
                        {panel(
                            "Risk Heat Map".to_string(),
                            vec![],
                            vec![{
                                // Pre-compute all matrix cell values before the view! block
                                let matrix_risks = risks.clone();

                                // Almost Certain row counts
                                let ac_insig = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::AlmostCertain && r.impact == Impact::Insignificant).count().to_string();
                                let ac_minor = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::AlmostCertain && r.impact == Impact::Minor).count().to_string();
                                let ac_mod = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::AlmostCertain && r.impact == Impact::Moderate).count().to_string();
                                let ac_major = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::AlmostCertain && r.impact == Impact::Major).count().to_string();
                                let ac_cat = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::AlmostCertain && r.impact == Impact::Catastrophic).count().to_string();

                                // Likely row counts
                                let li_insig = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::Likely && r.impact == Impact::Insignificant).count().to_string();
                                let li_minor = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::Likely && r.impact == Impact::Minor).count().to_string();
                                let li_mod = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::Likely && r.impact == Impact::Moderate).count().to_string();
                                let li_major = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::Likely && r.impact == Impact::Major).count().to_string();
                                let li_cat = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::Likely && r.impact == Impact::Catastrophic).count().to_string();

                                // Possible row counts
                                let po_insig = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::Possible && r.impact == Impact::Insignificant).count().to_string();
                                let po_minor = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::Possible && r.impact == Impact::Minor).count().to_string();
                                let po_mod = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::Possible && r.impact == Impact::Moderate).count().to_string();
                                let po_major = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::Possible && r.impact == Impact::Major).count().to_string();
                                let po_cat = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::Possible && r.impact == Impact::Catastrophic).count().to_string();

                                // Unlikely row counts
                                let un_insig = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::Unlikely && r.impact == Impact::Insignificant).count().to_string();
                                let un_minor = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::Unlikely && r.impact == Impact::Minor).count().to_string();
                                let un_mod = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::Unlikely && r.impact == Impact::Moderate).count().to_string();
                                let un_major = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::Unlikely && r.impact == Impact::Major).count().to_string();
                                let un_cat = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::Unlikely && r.impact == Impact::Catastrophic).count().to_string();

                                // Rare row counts
                                let ra_insig = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::Rare && r.impact == Impact::Insignificant).count().to_string();
                                let ra_minor = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::Rare && r.impact == Impact::Minor).count().to_string();
                                let ra_mod = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::Rare && r.impact == Impact::Moderate).count().to_string();
                                let ra_major = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::Rare && r.impact == Impact::Major).count().to_string();
                                let ra_cat = matrix_risks.iter().filter(|r| r.likelihood == Likelihood::Rare && r.impact == Impact::Catastrophic).count().to_string();

                                view! {
                                    <div class="risk-matrix">
                                        // Header row
                                        <div class="matrix-header"></div>
                                        <div class="matrix-header">"Insignificant"</div>
                                        <div class="matrix-header">"Minor"</div>
                                        <div class="matrix-header">"Moderate"</div>
                                        <div class="matrix-header">"Major"</div>
                                        <div class="matrix-header">"Catastrophic"</div>

                                        // Almost Certain row
                                        <div class="matrix-row-header">"Almost Certain"</div>
                                        <div class="matrix-cell medium">{ac_insig}</div>
                                        <div class="matrix-cell high">{ac_minor}</div>
                                        <div class="matrix-cell high">{ac_mod}</div>
                                        <div class="matrix-cell extreme">{ac_major}</div>
                                        <div class="matrix-cell extreme">{ac_cat}</div>

                                        // Likely row
                                        <div class="matrix-row-header">"Likely"</div>
                                        <div class="matrix-cell low">{li_insig}</div>
                                        <div class="matrix-cell medium">{li_minor}</div>
                                        <div class="matrix-cell high">{li_mod}</div>
                                        <div class="matrix-cell high">{li_major}</div>
                                        <div class="matrix-cell extreme">{li_cat}</div>

                                        // Possible row
                                        <div class="matrix-row-header">"Possible"</div>
                                        <div class="matrix-cell low">{po_insig}</div>
                                        <div class="matrix-cell medium">{po_minor}</div>
                                        <div class="matrix-cell medium">{po_mod}</div>
                                        <div class="matrix-cell high">{po_major}</div>
                                        <div class="matrix-cell high">{po_cat}</div>

                                        // Unlikely row
                                        <div class="matrix-row-header">"Unlikely"</div>
                                        <div class="matrix-cell low">{un_insig}</div>
                                        <div class="matrix-cell low">{un_minor}</div>
                                        <div class="matrix-cell medium">{un_mod}</div>
                                        <div class="matrix-cell medium">{un_major}</div>
                                        <div class="matrix-cell high">{un_cat}</div>

                                        // Rare row
                                        <div class="matrix-row-header">"Rare"</div>
                                        <div class="matrix-cell low">{ra_insig}</div>
                                        <div class="matrix-cell low">{ra_minor}</div>
                                        <div class="matrix-cell low">{ra_mod}</div>
                                        <div class="matrix-cell medium">{ra_major}</div>
                                        <div class="matrix-cell medium">{ra_cat}</div>
                                    </div>
                                }
                            }]
                        )}

                        // Critical Alerts
                        {panel(
                            "Critical Alerts".to_string(),
                            vec![],
                            vec![{
                                let critical_items: Vec<_> = violations.iter()
                                    .filter(|v| v.severity == Severity::Critical && v.status != ViolationStatus::Closed)
                                    .take(3)
                                    .collect();
                                let non_compliant: Vec<_> = checks.iter()
                                    .filter(|c| c.status == ComplianceStatus::NonCompliant)
                                    .take(2)
                                    .collect();

                                view! {
                                    <div class="critical-alerts">
                                        for violation in critical_items.iter() {
                                            <div class="alert-item">
                                                <div class="alert-icon">
                                                    <span inner_html={icon_alert}></span>
                                                </div>
                                                <div class="alert-content">
                                                    <div class="alert-title">{violation.policy_name.clone()}</div>
                                                    <div class="alert-desc">{violation.description.chars().take(80).collect::<String>()}{"..."}</div>
                                                </div>
                                            </div>
                                        }
                                        for check in non_compliant.iter() {
                                            <div class="alert-item">
                                                <div class="alert-icon">
                                                    <span inner_html={icon_warning}></span>
                                                </div>
                                                <div class="alert-content">
                                                    <div class="alert-title">{check.name.clone()}</div>
                                                    <div class="alert-desc">"Non-compliant - requires immediate attention"</div>
                                                </div>
                                            </div>
                                        }
                                        if critical_items.is_empty() && non_compliant.is_empty() {
                                            <div class="loading-overlay">"No critical alerts at this time"</div>
                                        }
                                    </div>
                                }
                            }]
                        )}
                    </div>
                </div>
            }

            // Compliance tab content
            if active_tab.get() == "compliance" {
                <div class="tab-content">
                    // Compliance overview cards
                    <div class="kpi-grid">
                        {kpi_card(
                            "Total Checks".to_string(),
                            format!("{}", kpi_data.total_checks),
                            KpiColor::Blue,
                            icon_check.to_string(),
                            None,
                            None
                        )}
                        {kpi_card(
                            "Compliant".to_string(),
                            format!("{}", kpi_data.compliant_checks),
                            KpiColor::Green,
                            icon_shield.to_string(),
                            None,
                            None
                        )}
                        {kpi_card(
                            "Non-Compliant".to_string(),
                            format!("{}", kpi_data.non_compliant_checks),
                            KpiColor::Red,
                            icon_alert.to_string(),
                            None,
                            None
                        )}
                        {kpi_card(
                            "Pending Review".to_string(),
                            format!("{}", kpi_data.pending_reviews),
                            KpiColor::Orange,
                            icon_warning.to_string(),
                            None,
                            None
                        )}
                    </div>

                    // Compliance table
                    {panel(
                        format!("Compliance Checks ({} total)", checks.len()),
                        vec![view! { <button class="btn btn-sm btn-primary">"New Assessment"</button> }],
                        vec![
                            if loading.get() {
                                view! { <div class="loading-overlay">"Loading compliance data..."</div> }
                            } else {
                                view! {
                                    <div>
                                        {data_table(compliance_columns.clone(), compliance_rows.clone(), None)}
                                    </div>
                                }
                            }
                        ]
                    )}
                </div>
            }

            // Risks tab content
            if active_tab.get() == "risks" {
                <div class="tab-content">
                    // Risk overview cards
                    <div class="kpi-grid">
                        {kpi_card(
                            "Total Risks".to_string(),
                            format!("{}", kpi_data.total_risks),
                            KpiColor::Blue,
                            icon_alert.to_string(),
                            None,
                            None
                        )}
                        {kpi_card(
                            "Extreme/High".to_string(),
                            format!("{}", kpi_data.extreme_risks + kpi_data.high_risks),
                            KpiColor::Red,
                            icon_alert.to_string(),
                            None,
                            None
                        )}
                        {kpi_card(
                            "Medium".to_string(),
                            format!("{}", kpi_data.medium_risks),
                            KpiColor::Orange,
                            icon_warning.to_string(),
                            None,
                            None
                        )}
                        {kpi_card(
                            "Low".to_string(),
                            format!("{}", kpi_data.low_risks),
                            KpiColor::Green,
                            icon_shield.to_string(),
                            None,
                            None
                        )}
                    </div>

                    // Risk table
                    {panel(
                        format!("Risk Register ({} total)", risks.len()),
                        vec![view! { <button class="btn btn-sm btn-primary">"Add Risk"</button> }],
                        vec![
                            if loading.get() {
                                view! { <div class="loading-overlay">"Loading risk data..."</div> }
                            } else {
                                view! {
                                    <div>
                                        {data_table(risk_columns.clone(), risk_rows.clone(), None)}
                                    </div>
                                }
                            }
                        ]
                    )}
                </div>
            }

            // Violations tab content
            if active_tab.get() == "violations" {
                <div class="tab-content">
                    // Violations overview cards
                    <div class="kpi-grid">
                        {kpi_card(
                            "Total Violations".to_string(),
                            format!("{}", violations.len()),
                            KpiColor::Blue,
                            icon_warning.to_string(),
                            None,
                            None
                        )}
                        {kpi_card(
                            "Open".to_string(),
                            format!("{}", kpi_data.open_violations),
                            KpiColor::Orange,
                            icon_alert.to_string(),
                            None,
                            None
                        )}
                        {kpi_card(
                            "Critical".to_string(),
                            format!("{}", kpi_data.critical_violations),
                            KpiColor::Red,
                            icon_alert.to_string(),
                            None,
                            None
                        )}
                        {kpi_card(
                            "Resolved".to_string(),
                            format!("{}", resolved_violations_count),
                            KpiColor::Green,
                            icon_check.to_string(),
                            None,
                            None
                        )}
                    </div>

                    // Violations table
                    {panel(
                        format!("Policy Violations ({} total)", violations.len()),
                        vec![view! { <button class="btn btn-sm btn-primary">"Report Violation"</button> }],
                        vec![
                            if loading.get() {
                                view! { <div class="loading-overlay">"Loading violations data..."</div> }
                            } else {
                                view! {
                                    <div>
                                        {data_table(violation_columns.clone(), violation_rows.clone(), None)}
                                    </div>
                                }
                            }
                        ]
                    )}
                </div>
            }

            // Controls tab content
            if active_tab.get() == "controls" {
                <div class="tab-content">
                    // Controls overview cards
                    <div class="kpi-grid">
                        {kpi_card(
                            "Total Controls".to_string(),
                            format!("{}", kpi_data.total_controls),
                            KpiColor::Blue,
                            icon_settings.to_string(),
                            None,
                            None
                        )}
                        {kpi_card(
                            "Effective".to_string(),
                            format!("{}", kpi_data.effective_controls),
                            KpiColor::Green,
                            icon_check.to_string(),
                            None,
                            None
                        )}
                        {kpi_card(
                            "Partially Effective".to_string(),
                            format!("{}", partially_effective_controls_count),
                            KpiColor::Orange,
                            icon_warning.to_string(),
                            None,
                            None
                        )}
                        {kpi_card(
                            "Ineffective".to_string(),
                            format!("{}", kpi_data.ineffective_controls),
                            KpiColor::Red,
                            icon_alert.to_string(),
                            None,
                            None
                        )}
                    </div>

                    // Controls table
                    {panel(
                        format!("Internal Controls ({} total)", ctrl_list.len()),
                        vec![view! { <button class="btn btn-sm btn-primary">"Add Control"</button> }],
                        vec![
                            if loading.get() {
                                view! { <div class="loading-overlay">"Loading controls data..."</div> }
                            } else {
                                view! {
                                    <div>
                                        {data_table(control_columns.clone(), control_rows.clone(), None)}
                                    </div>
                                }
                            }
                        ]
                    )}
                </div>
            }
        </div>
    }
}
