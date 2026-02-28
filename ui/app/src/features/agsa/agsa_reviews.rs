//! AGSA Reviews page - audit findings, action items, compliance status

use components::prelude::*;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, data_table, DataTableColumn, DataTableRow,
    status_badge, StatusType,
    tag, TagType,
    kpi_card, KpiColor, KpiDelta,
    timeline, TimelineItem, TimelineStatus,
    tabs, Tab,
    progress_bar,
};
use crate::util::format::{format_currency, format_date};
use super::store::AgsaStore;
use super::types::{
    FindingStatus, FindingCategory, FindingSeverity, ActionStatus, ActionPriority,
    ComplianceStatus,
};
use super::service;

/// AGSA Reviews page
#[component]
pub fn agsa_reviews() -> View {
    let store = use_context::<AgsaStore>();

    // Load data on mount
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                service::load_agsa_data(&store).await;
            });
        }
    });

    let findings = store.findings.clone();
    let action_items = store.action_items.clone();
    let audit_reports = store.audit_reports.clone();
    let kpis = store.kpis.clone();
    let filter = store.filter.clone();

    // Active tab state
    let active_tab = signal("findings".to_string());

    // Summary counts
    let open_count = move |_| findings.get().iter()
        .filter(|f| matches!(f.status, FindingStatus::Open))
        .count();
    let in_progress_count = move |_| findings.get().iter()
        .filter(|f| matches!(f.status, FindingStatus::InProgress))
        .count();
    let resolved_count = move |_| findings.get().iter()
        .filter(|f| matches!(f.status, FindingStatus::Resolved | FindingStatus::Closed))
        .count();
    let repeat_count = move |_| findings.get().iter()
        .filter(|f| f.is_repeat_finding)
        .count();

    // Filter findings
    let filtered_findings = {
        let store = store.clone();
        move |_| store.filtered_findings()
    };

    // Overdue actions
    let overdue_actions = {
        let store = store.clone();
        move |_| store.overdue_action_items()
    };

    // Findings table columns
    let finding_columns = vec![
        DataTableColumn {
            key: "reference".to_string(),
            label: "Reference".to_string(),
            width: Some("160px".to_string()),
            align: None,
            cell_class: Some("id-cell".to_string()),
        },
        DataTableColumn {
            key: "title".to_string(),
            label: "Finding".to_string(),
            width: None,
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "category".to_string(),
            label: "Category".to_string(),
            width: Some("140px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "severity".to_string(),
            label: "Severity".to_string(),
            width: Some("100px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "department".to_string(),
            label: "Department".to_string(),
            width: Some("150px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "target".to_string(),
            label: "Target Date".to_string(),
            width: Some("100px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "status".to_string(),
            label: "Status".to_string(),
            width: Some("120px".to_string()),
            align: None,
            cell_class: None,
        },
    ];

    // Transform findings to table rows
    let finding_rows = move |_| -> Vec<DataTableRow> {
        filtered_findings().iter().map(|finding| {
            let status = match finding.status {
                FindingStatus::Open => status_badge(StatusType::Pending),
                FindingStatus::InProgress => status_badge(StatusType::InProgress),
                FindingStatus::Resolved => status_badge(StatusType::Complete),
                FindingStatus::Closed => status_badge(StatusType::Approved),
                FindingStatus::Recurring => status_badge(StatusType::OnHold),
                FindingStatus::Overdue => status_badge(StatusType::Rejected),
            };

            let severity_tag = match finding.severity {
                FindingSeverity::Material => tag("Material".to_string(), TagType::Danger),
                FindingSeverity::Significant => tag("Significant".to_string(), TagType::Warning),
                FindingSeverity::Minor => tag("Minor".to_string(), TagType::Info),
                FindingSeverity::Observation => tag("Observation".to_string(), TagType::Default),
            };

            let category_display = finding.category.label();

            let title_with_repeat = if finding.is_repeat_finding {
                view! {
                    <span class="title-cell">
                        {finding.title.clone()}
                        <span class="repeat-badge" title="Repeat Finding">" (R)"</span>
                    </span>
                }
            } else {
                view! { <span class="title-cell">{finding.title.clone()}</span> }
            };

            DataTableRow {
                id: finding.id.clone(),
                cells: vec![
                    view! { <span class="id-cell">{finding.reference_number.clone()}</span> },
                    title_with_repeat,
                    view! { <span>{category_display}</span> },
                    severity_tag,
                    view! { <span>{finding.responsible_department.clone()}</span> },
                    view! { <span>{format_date(&finding.target_date)}</span> },
                    status,
                ],
            }
        }).collect()
    };

    // Action items table columns
    let action_columns = vec![
        DataTableColumn {
            key: "reference".to_string(),
            label: "Reference".to_string(),
            width: Some("140px".to_string()),
            align: None,
            cell_class: Some("id-cell".to_string()),
        },
        DataTableColumn {
            key: "description".to_string(),
            label: "Action".to_string(),
            width: None,
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "assigned".to_string(),
            label: "Assigned To".to_string(),
            width: Some("150px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "priority".to_string(),
            label: "Priority".to_string(),
            width: Some("90px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "due".to_string(),
            label: "Due Date".to_string(),
            width: Some("100px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "progress".to_string(),
            label: "Progress".to_string(),
            width: Some("120px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "status".to_string(),
            label: "Status".to_string(),
            width: Some("110px".to_string()),
            align: None,
            cell_class: None,
        },
    ];

    // Transform action items to table rows
    let action_rows = move |_| -> Vec<DataTableRow> {
        action_items.get().iter().map(|action| {
            let status = match action.status {
                ActionStatus::NotStarted => status_badge(StatusType::Pending),
                ActionStatus::InProgress => status_badge(StatusType::InProgress),
                ActionStatus::Completed => status_badge(StatusType::Complete),
                ActionStatus::Verified => status_badge(StatusType::Approved),
                ActionStatus::Overdue => status_badge(StatusType::Rejected),
                ActionStatus::Cancelled => status_badge(StatusType::Cancelled),
            };

            let priority_tag = match action.priority {
                ActionPriority::Critical => tag("Critical".to_string(), TagType::Danger),
                ActionPriority::High => tag("High".to_string(), TagType::Warning),
                ActionPriority::Medium => tag("Medium".to_string(), TagType::Info),
                ActionPriority::Low => tag("Low".to_string(), TagType::Default),
            };

            let progress_color = if action.progress_percent >= 75 {
                "var(--green)"
            } else if action.progress_percent >= 50 {
                "var(--blue)"
            } else if action.progress_percent >= 25 {
                "var(--orange)"
            } else {
                "var(--text-muted)"
            };

            DataTableRow {
                id: action.id.clone(),
                cells: vec![
                    view! { <span class="id-cell">{action.reference_number.clone()}</span> },
                    view! { <span class="description-cell">{action.description.clone()}</span> },
                    view! { <span>{action.assigned_to.clone()}</span> },
                    priority_tag,
                    view! { <span>{format_date(&action.due_date)}</span> },
                    view! {
                        <div class="progress-cell">
                            {progress_bar(action.progress_percent as f64, Some(progress_color.to_string()))}
                            <span class="progress-text">{format!("{}%", action.progress_percent)}</span>
                        </div>
                    },
                    status,
                ],
            }
        }).collect()
    };

    // Audit reports table columns
    let report_columns = vec![
        DataTableColumn {
            key: "reference".to_string(),
            label: "Report No.".to_string(),
            width: Some("160px".to_string()),
            align: None,
            cell_class: Some("id-cell".to_string()),
        },
        DataTableColumn {
            key: "year".to_string(),
            label: "Financial Year".to_string(),
            width: Some("110px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "type".to_string(),
            label: "Type".to_string(),
            width: Some("130px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "findings".to_string(),
            label: "Findings".to_string(),
            width: Some("90px".to_string()),
            align: Some("center".to_string()),
            cell_class: None,
        },
        DataTableColumn {
            key: "resolved".to_string(),
            label: "Resolved".to_string(),
            width: Some("90px".to_string()),
            align: Some("center".to_string()),
            cell_class: None,
        },
        DataTableColumn {
            key: "status".to_string(),
            label: "Opinion".to_string(),
            width: Some("180px".to_string()),
            align: None,
            cell_class: None,
        },
    ];

    // Transform reports to table rows
    let report_rows = move |_| -> Vec<DataTableRow> {
        audit_reports.get().iter().map(|report| {
            let status_tag = match report.compliance_status {
                ComplianceStatus::Clean => tag("Clean Audit".to_string(), TagType::Success),
                ComplianceStatus::UnqualifiedWithFindings => tag("Unqualified w/Findings".to_string(), TagType::Info),
                ComplianceStatus::Qualified => tag("Qualified".to_string(), TagType::Warning),
                ComplianceStatus::Adverse => tag("Adverse".to_string(), TagType::Danger),
                ComplianceStatus::Disclaimer => tag("Disclaimer".to_string(), TagType::Danger),
                ComplianceStatus::PendingAudit => tag("Pending".to_string(), TagType::Default),
            };

            let type_display = report.audit_type.label();

            DataTableRow {
                id: report.id.clone(),
                cells: vec![
                    view! { <span class="id-cell">{report.reference_number.clone()}</span> },
                    view! { <span>{report.financial_year.clone()}</span> },
                    view! { <span>{type_display}</span> },
                    view! { <span class="count-cell">{report.total_findings}</span> },
                    view! { <span class="count-cell">{report.resolved_findings}</span> },
                    status_tag,
                ],
            }
        }).collect()
    };

    // Handle row clicks
    let handle_finding_click = Callback::new({
        let store = store.clone();
        move |finding_id: String| {
            store.select_finding(&finding_id);
            // In production, would navigate to detail view
        }
    });

    let handle_action_click = Callback::new({
        move |_action_id: String| {
            // In production, would open action detail modal
        }
    });

    let handle_report_click = Callback::new({
        let store = store.clone();
        move |report_id: String| {
            store.select_report(&report_id);
            // In production, would navigate to report detail view
        }
    });

    // Filter handlers
    let set_filter_all = {
        let store = store.clone();
        Callback::<()>::new(move |_| store.set_status_filter(None))
    };
    let set_filter_open = {
        let store = store.clone();
        Callback::<()>::new(move |_| store.set_status_filter(Some(FindingStatus::Open)))
    };
    let set_filter_in_progress = {
        let store = store.clone();
        Callback::<()>::new(move |_| store.set_status_filter(Some(FindingStatus::InProgress)))
    };
    let set_filter_resolved = {
        let store = store.clone();
        Callback::<()>::new(move |_| store.set_status_filter(Some(FindingStatus::Resolved)))
    };

    // Tab change handler
    let on_tab_change = {
        let active_tab = active_tab.clone();
        Callback::new(move |tab: String| {
            active_tab.set(tab);
        })
    };

    // Icons
    let icon_alert = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>"#;
    let icon_clock = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>"#;
    let icon_check = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>"#;
    let icon_refresh = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="23 4 23 10 17 10"/><polyline points="1 20 1 14 7 14"/><path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/></svg>"#;
    let icon_dollar = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8"/><path d="M12 18V6"/></svg>"#;
    let icon_trending = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="23 6 13.5 15.5 8.5 10.5 1 18"/><polyline points="17 6 23 6 23 12"/></svg>"#;
    let icon_file = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/><polyline points="10 9 9 9 8 9"/></svg>"#;
    let icon_target = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><circle cx="12" cy="12" r="6"/><circle cx="12" cy="12" r="2"/></svg>"#;

    // Timeline for upcoming deadlines
    let deadline_timeline = move |_| -> Vec<TimelineItem> {
        let mut items: Vec<TimelineItem> = action_items.get().iter()
            .filter(|a| !matches!(a.status, ActionStatus::Completed | ActionStatus::Verified | ActionStatus::Cancelled))
            .take(5)
            .map(|action| {
                let status = if matches!(action.status, ActionStatus::Overdue) {
                    TimelineStatus::Error
                } else if action.progress_percent >= 75 {
                    TimelineStatus::Complete
                } else {
                    TimelineStatus::Pending
                };
                TimelineItem {
                    date: format_date(&action.due_date),
                    title: action.reference_number.clone(),
                    description: format!("{} ({}%)", action.description.chars().take(40).collect::<String>(), action.progress_percent),
                    status,
                }
            })
            .collect();
        items.sort_by(|a, b| a.date.cmp(&b.date));
        items
    };

    // Get compliance status display
    let compliance_display = move |_| {
        let status = kpis.get().current_compliance_status;
        match status {
            ComplianceStatus::Clean => ("Clean Audit", "var(--green)"),
            ComplianceStatus::UnqualifiedWithFindings => ("Unqualified w/Findings", "var(--blue)"),
            ComplianceStatus::Qualified => ("Qualified Opinion", "var(--orange)"),
            ComplianceStatus::Adverse => ("Adverse Opinion", "var(--red)"),
            ComplianceStatus::Disclaimer => ("Disclaimer", "#8b0000"),
            ComplianceStatus::PendingAudit => ("Pending Audit", "var(--text-muted)"),
        }
    };

    view! {
        style {
            r#"
            .agsa-reviews {
                display: flex;
                flex-direction: column;
                gap: var(--space-6);
            }
            .kpi-grid {
                display: grid;
                grid-template-columns: repeat(4, 1fr);
                gap: 16px;
            }
            @media (max-width: 1200px) {
                .kpi-grid { grid-template-columns: repeat(2, 1fr); }
            }
            @media (max-width: 600px) {
                .kpi-grid { grid-template-columns: 1fr; }
            }
            .summary-cards {
                display: grid;
                grid-template-columns: repeat(4, 1fr);
                gap: 16px;
                margin-bottom: 16px;
            }
            @media (max-width: 1024px) {
                .summary-cards { grid-template-columns: repeat(2, 1fr); }
            }
            @media (max-width: 600px) {
                .summary-cards { grid-template-columns: 1fr; }
            }
            .summary-card {
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius-lg);
                padding: 20px;
                display: flex;
                align-items: center;
                gap: 16px;
                cursor: pointer;
                transition: all 0.2s;
            }
            .summary-card:hover {
                border-color: var(--blue);
                box-shadow: var(--shadow-md);
            }
            .summary-card.active {
                border-color: var(--blue);
                background: var(--blue-light);
            }
            .summary-icon {
                width: 48px;
                height: 48px;
                border-radius: var(--radius-md);
                display: flex;
                align-items: center;
                justify-content: center;
            }
            .summary-icon svg {
                width: 24px;
                height: 24px;
            }
            .summary-icon.open { background: var(--red-light); color: var(--red); }
            .summary-icon.progress { background: var(--cyan-light); color: var(--cyan); }
            .summary-icon.resolved { background: var(--green-light); color: var(--green); }
            .summary-icon.repeat { background: var(--purple-light); color: var(--purple); }
            .summary-content h3 {
                font-size: 24px;
                font-weight: 700;
                color: var(--navy);
                margin-bottom: 4px;
            }
            .summary-content p {
                font-size: 13px;
                color: var(--text-muted);
            }
            .content-grid {
                display: grid;
                grid-template-columns: 1fr 350px;
                gap: 24px;
            }
            @media (max-width: 1200px) {
                .content-grid { grid-template-columns: 1fr; }
            }
            .title-cell {
                max-width: 280px;
                white-space: nowrap;
                overflow: hidden;
                text-overflow: ellipsis;
            }
            .description-cell {
                max-width: 250px;
                white-space: nowrap;
                overflow: hidden;
                text-overflow: ellipsis;
            }
            .repeat-badge {
                color: var(--purple);
                font-weight: 600;
                margin-left: 4px;
            }
            .count-cell {
                text-align: center;
                font-weight: 600;
            }
            .progress-cell {
                display: flex;
                align-items: center;
                gap: 8px;
            }
            .progress-text {
                font-size: 12px;
                font-weight: 500;
                color: var(--text-muted);
                min-width: 35px;
            }
            .sidebar {
                display: flex;
                flex-direction: column;
                gap: 24px;
            }
            .compliance-status {
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius-lg);
                padding: 20px;
                text-align: center;
            }
            .compliance-status h4 {
                font-size: 13px;
                font-weight: 500;
                color: var(--text-muted);
                margin-bottom: 12px;
            }
            .compliance-badge {
                display: inline-block;
                padding: 8px 16px;
                border-radius: var(--radius-md);
                font-size: 14px;
                font-weight: 600;
            }
            .quick-stats {
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius-lg);
                padding: 20px;
            }
            .quick-stats h4 {
                font-size: 14px;
                font-weight: 600;
                color: var(--navy);
                margin-bottom: 16px;
            }
            .stat-row {
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding: 8px 0;
                border-bottom: 1px solid var(--border);
            }
            .stat-row:last-child {
                border-bottom: none;
            }
            .stat-label {
                font-size: 13px;
                color: var(--text-muted);
            }
            .stat-value {
                font-size: 14px;
                font-weight: 600;
                color: var(--navy);
            }
            .stat-value.highlight {
                color: var(--green);
            }
            .stat-value.warning {
                color: var(--orange);
            }
            .stat-value.danger {
                color: var(--red);
            }
            "#
        }

        <div class="agsa-reviews" data-testid="agsa-reviews">
            {page_header(
                "AGSA - Audit Findings".to_string(),
                Some("Auditor-General South Africa findings, action items, and compliance tracking".to_string()),
                vec![
                    view! { <button class="btn btn-secondary">"Export Report"</button> },
                    view! { <button class="btn btn-primary">"Submit Action Plan"</button> },
                ]
            )}

            // KPI Row
            <div class="kpi-grid">
                {kpi_card(
                    "Total Findings".to_string(),
                    kpis.get().total_findings.to_string(),
                    KpiColor::Orange,
                    icon_alert.to_string(),
                    Some(KpiDelta { value: format!("{} material", kpis.get().open_findings), is_positive: None, suffix: "".to_string() }),
                    None
                )}
                {kpi_card(
                    "Action Items".to_string(),
                    kpis.get().total_action_items.to_string(),
                    KpiColor::Blue,
                    icon_target.to_string(),
                    Some(KpiDelta { value: format!("{} overdue", kpis.get().overdue_actions), is_positive: Some(false), suffix: "".to_string() }),
                    None
                )}
                {kpi_card(
                    "Resolution Rate".to_string(),
                    format!("{:.1}%", kpis.get().resolution_rate),
                    KpiColor::Green,
                    icon_trending.to_string(),
                    Some(KpiDelta { value: "Target: 100%".to_string(), is_positive: None, suffix: "".to_string() }),
                    None
                )}
                {kpi_card(
                    "Financial Impact".to_string(),
                    format_currency(kpis.get().financial_impact_total),
                    KpiColor::Red,
                    icon_dollar.to_string(),
                    Some(KpiDelta { value: "Identified exposure".to_string(), is_positive: None, suffix: "".to_string() }),
                    None
                )}
            </div>

            // Summary filter cards
            <div class="summary-cards">
                <div
                    class={move |_| if filter.get().status.is_none() { "summary-card active" } else { "summary-card" }}
                    on:click={set_filter_all.clone()}
                >
                    <div class="summary-icon open" inner_html={icon_file}></div>
                    <div class="summary-content">
                        <h3>{move |_| findings.get().len()}</h3>
                        <p>"Total Findings"</p>
                    </div>
                </div>
                <div
                    class={move |_| if matches!(filter.get().status, Some(FindingStatus::Open)) { "summary-card active" } else { "summary-card" }}
                    on:click={set_filter_open}
                >
                    <div class="summary-icon open" inner_html={icon_alert}></div>
                    <div class="summary-content">
                        <h3>{open_count}</h3>
                        <p>"Open"</p>
                    </div>
                </div>
                <div
                    class={move |_| if matches!(filter.get().status, Some(FindingStatus::InProgress)) { "summary-card active" } else { "summary-card" }}
                    on:click={set_filter_in_progress}
                >
                    <div class="summary-icon progress" inner_html={icon_clock}></div>
                    <div class="summary-content">
                        <h3>{in_progress_count}</h3>
                        <p>"In Progress"</p>
                    </div>
                </div>
                <div
                    class={move |_| if matches!(filter.get().status, Some(FindingStatus::Resolved)) { "summary-card active" } else { "summary-card" }}
                    on:click={set_filter_resolved}
                >
                    <div class="summary-icon resolved" inner_html={icon_check}></div>
                    <div class="summary-content">
                        <h3>{resolved_count}</h3>
                        <p>"Resolved"</p>
                    </div>
                </div>
            </div>

            // Main content with sidebar
            <div class="content-grid">
                // Main content area with tabs
                <div class="main-content">
                    {tabs(
                        vec![
                            Tab { id: "findings".to_string(), label: "Audit Findings".to_string() },
                            Tab { id: "actions".to_string(), label: "Action Items".to_string() },
                            Tab { id: "reports".to_string(), label: "Audit Reports".to_string() },
                        ],
                        active_tab.get(),
                        on_tab_change.clone()
                    )}

                    // Tab content
                    {move |_| match active_tab.get().as_str() {
                        "findings" => view! {
                            {panel(
                                "AGSA Audit Findings".to_string(),
                                vec![
                                    view! { <button class="btn btn-sm btn-secondary">"Filter"</button> },
                                    view! { <button class="btn btn-sm btn-secondary">"Export"</button> },
                                ],
                                vec![data_table(finding_columns.clone(), finding_rows(), Some(handle_finding_click.clone()))]
                            )}
                        },
                        "actions" => view! {
                            {panel(
                                "Corrective Action Items".to_string(),
                                vec![
                                    view! { <button class="btn btn-sm btn-primary">"+ New Action"</button> },
                                ],
                                vec![data_table(action_columns.clone(), action_rows(), Some(handle_action_click.clone()))]
                            )}
                        },
                        "reports" => view! {
                            {panel(
                                "Audit Report History".to_string(),
                                vec![
                                    view! { <button class="btn btn-sm btn-secondary">"Download"</button> },
                                ],
                                vec![data_table(report_columns.clone(), report_rows(), Some(handle_report_click.clone()))]
                            )}
                        },
                        _ => view! { <div></div> },
                    }}
                </div>

                // Sidebar
                <div class="sidebar">
                    // Current compliance status
                    <div class="compliance-status">
                        <h4>"Current Audit Opinion"</h4>
                        <div
                            class="compliance-badge"
                            style={move |_| format!("background: {}20; color: {}", compliance_display().1, compliance_display().1)}
                        >
                            {move |_| compliance_display().0}
                        </div>
                    </div>

                    // Upcoming deadlines
                    {panel(
                        "Upcoming Deadlines".to_string(),
                        vec![],
                        vec![timeline(deadline_timeline(), None)]
                    )}

                    // Quick stats
                    <div class="quick-stats">
                        <h4>"Finding Statistics"</h4>
                        <div class="stat-row">
                            <span class="stat-label">"Repeat Findings"</span>
                            <span class="stat-value warning">{repeat_count}</span>
                        </div>
                        <div class="stat-row">
                            <span class="stat-label">"Avg Resolution Days"</span>
                            <span class="stat-value">{move |_| format!("{:.0}", kpis.get().average_resolution_days)}</span>
                        </div>
                        <div class="stat-row">
                            <span class="stat-label">"Actions Completed"</span>
                            <span class="stat-value highlight">{move |_| kpis.get().completed_actions}</span>
                        </div>
                        <div class="stat-row">
                            <span class="stat-label">"Actions Overdue"</span>
                            <span class="stat-value danger">{move |_| kpis.get().overdue_actions}</span>
                        </div>
                    </div>

                    // Severity breakdown
                    {panel(
                        "Findings by Severity".to_string(),
                        vec![],
                        vec![view! {
                            <div class="severity-breakdown">
                                <div class="stat-row">
                                    <span class="stat-label">"Material"</span>
                                    <span class="stat-value danger">
                                        {move |_| findings.get().iter().filter(|f| matches!(f.severity, FindingSeverity::Material)).count()}
                                    </span>
                                </div>
                                <div class="stat-row">
                                    <span class="stat-label">"Significant"</span>
                                    <span class="stat-value warning">
                                        {move |_| findings.get().iter().filter(|f| matches!(f.severity, FindingSeverity::Significant)).count()}
                                    </span>
                                </div>
                                <div class="stat-row">
                                    <span class="stat-label">"Minor"</span>
                                    <span class="stat-value">
                                        {move |_| findings.get().iter().filter(|f| matches!(f.severity, FindingSeverity::Minor)).count()}
                                    </span>
                                </div>
                                <div class="stat-row">
                                    <span class="stat-label">"Observation"</span>
                                    <span class="stat-value">
                                        {move |_| findings.get().iter().filter(|f| matches!(f.severity, FindingSeverity::Observation)).count()}
                                    </span>
                                </div>
                            </div>
                        }]
                    )}
                </div>
            </div>
        </div>
    }
}
