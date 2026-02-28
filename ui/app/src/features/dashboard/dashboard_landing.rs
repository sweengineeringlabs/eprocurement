//! Dashboard landing page

use components::prelude::*;
use crate::shared::layout::page_header;
use crate::shared::components::{
    kpi_card, KpiColor, KpiDelta,
    panel, data_table, DataTableColumn, DataTableRow,
    status_badge, StatusType,
    timeline, TimelineItem, TimelineStatus,
    progress_bar, ProgressColor,
};
use crate::shared::charts::{bar_chart, BarChartData, pie_chart, PieChartData};
use crate::util::format::{format_currency, format_number};
use super::store::{DashboardStore, load_mock_data};
use super::service;

/// Dashboard landing page
#[component]
pub fn dashboard_landing() -> View {
    let store = use_context::<DashboardStore>();

    // Load data on mount
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                service::load_dashboard(&store).await;
            });
        }
    });

    let kpis = store.kpis.clone();
    let recent_reqs = store.recent_requisitions.clone();
    let activities = store.activities.clone();
    let spend = store.spend_by_category.clone();
    let bbbee = store.bbbee_breakdown.clone();

    // Table columns
    let columns = vec![
        DataTableColumn { key: "id".to_string(), label: "Req ID".to_string(), width: None, align: None, cell_class: Some("id-cell".to_string()) },
        DataTableColumn { key: "description".to_string(), label: "Description".to_string(), width: None, align: None, cell_class: None },
        DataTableColumn { key: "amount".to_string(), label: "Amount".to_string(), width: None, align: Some("right".to_string()), cell_class: Some("amount-cell".to_string()) },
        DataTableColumn { key: "status".to_string(), label: "Status".to_string(), width: None, align: None, cell_class: None },
    ];

    // Transform requisitions to table rows
    let rows: Vec<DataTableRow> = recent_reqs.get().iter().map(|req| {
        let status = match req.status.as_str() {
            "approved" => status_badge(StatusType::Approved),
            "pending" => status_badge(StatusType::Pending),
            "draft" => status_badge(StatusType::Draft),
            "evaluation" => status_badge(StatusType::Evaluation),
            _ => status_badge(StatusType::New),
        };
        DataTableRow {
            id: req.id.clone(),
            cells: vec![
                view! { <span class="id-cell">{req.id.clone()}</span> },
                view! { <span>{req.description.clone()}</span> },
                view! { <span class="amount-cell">{format_currency(req.amount)}</span> },
                status,
            ],
        }
    }).collect();

    // Transform activities to timeline items
    let timeline_items: Vec<TimelineItem> = activities.get().iter().map(|act| {
        TimelineItem {
            date: act.timestamp.clone(),
            title: act.title.clone(),
            description: act.description.clone(),
            status: if act.completed { TimelineStatus::Completed } else { TimelineStatus::Pending },
        }
    }).collect();

    // Spend chart data
    let spend_data: Vec<BarChartData> = spend.get().iter().map(|s| {
        BarChartData {
            label: s.name.clone(),
            value: s.amount,
            color: None,
        }
    }).collect();

    // BBBEE chart data
    let bbbee_data = bbbee.get();
    let bbbee_pie: Vec<PieChartData> = vec![
        PieChartData { label: "Level 1".to_string(), value: bbbee_data.level_1, color: "var(--green)".to_string() },
        PieChartData { label: "Level 2".to_string(), value: bbbee_data.level_2, color: "var(--blue)".to_string() },
        PieChartData { label: "Level 3-4".to_string(), value: bbbee_data.level_3_4, color: "var(--orange)".to_string() },
        PieChartData { label: "Other".to_string(), value: bbbee_data.other, color: "var(--text-muted)".to_string() },
    ];

    // Icons
    let icon_doc = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>"#;
    let icon_check = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>"#;
    let icon_dollar = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8"/><path d="M12 18V6"/></svg>"#;
    let icon_users = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>"#;
    let icon_clock = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>"#;
    let icon_file = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><path d="M14 2v6h6"/><path d="M12 18v-6"/><path d="M9 15h6"/></svg>"#;
    let icon_layers = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/><path d="M2 12l10 5 10-5"/></svg>"#;
    let icon_alert = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>"#;

    view! {
        style {
            r#"
            .dashboard { display: flex; flex-direction: column; gap: var(--space-6); }
            .kpi-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; }
            @media (max-width: 1200px) { .kpi-grid { grid-template-columns: repeat(2, 1fr); } }
            @media (max-width: 768px) { .kpi-grid { grid-template-columns: 1fr; } }
            "#
        }

        <div class="dashboard" data-testid="dashboard-landing">
            {page_header(
                "Procurement Dashboard".to_string(),
                Some("Overview of procurement activities for FY 2025/26".to_string()),
                vec![
                    view! { <button class="btn btn-secondary">"Export Report"</button> },
                    view! { <a href="#" class="btn btn-primary">"New Requisition"</a> },
                ]
            )}

            // KPI Row 1
            <div class="kpi-grid">
                {kpi_card(
                    "Active Requisitions".to_string(),
                    kpis.get().active_requisitions.to_string(),
                    KpiColor::Blue,
                    icon_doc.to_string(),
                    Some(KpiDelta { value: "12%".to_string(), is_positive: Some(true), suffix: "from last month".to_string() }),
                    None
                )}
                {kpi_card(
                    "Open Tenders".to_string(),
                    kpis.get().open_tenders.to_string(),
                    KpiColor::Green,
                    icon_check.to_string(),
                    Some(KpiDelta { value: "8%".to_string(), is_positive: Some(true), suffix: "from last month".to_string() }),
                    None
                )}
                {kpi_card(
                    "YTD Spend".to_string(),
                    format_currency(kpis.get().ytd_spend),
                    KpiColor::Accent,
                    icon_dollar.to_string(),
                    Some(KpiDelta { value: "3%".to_string(), is_positive: Some(false), suffix: "vs budget".to_string() }),
                    None
                )}
                {kpi_card(
                    "Active Suppliers".to_string(),
                    format_number(kpis.get().active_suppliers),
                    KpiColor::Purple,
                    icon_users.to_string(),
                    Some(KpiDelta { value: "45".to_string(), is_positive: Some(true), suffix: "new this month".to_string() }),
                    None
                )}
            </div>

            // KPI Row 2
            <div class="kpi-grid">
                {kpi_card(
                    "Pending Approvals".to_string(),
                    kpis.get().pending_approvals.to_string(),
                    KpiColor::Orange,
                    icon_clock.to_string(),
                    Some(KpiDelta { value: "8 urgent (>5 days)".to_string(), is_positive: None, suffix: "".to_string() }),
                    None
                )}
                {kpi_card(
                    "Active Contracts".to_string(),
                    kpis.get().active_contracts.to_string(),
                    KpiColor::Cyan,
                    icon_file.to_string(),
                    Some(KpiDelta { value: "12 expiring in 30 days".to_string(), is_positive: None, suffix: "".to_string() }),
                    None
                )}
                {kpi_card(
                    "B-BBEE Spend".to_string(),
                    format!("{:.1}%", kpis.get().bbbee_spend_percent),
                    KpiColor::Green,
                    icon_layers.to_string(),
                    Some(KpiDelta { value: "Target: 80%".to_string(), is_positive: None, suffix: "".to_string() }),
                    None
                )}
                {kpi_card(
                    "Compliance Issues".to_string(),
                    kpis.get().compliance_issues.to_string(),
                    KpiColor::Red,
                    icon_alert.to_string(),
                    Some(KpiDelta { value: "3 critical attention needed".to_string(), is_positive: None, suffix: "".to_string() }),
                    None
                )}
            </div>

            // Main content grid
            <div class="grid-3-2">
                // Recent Requisitions
                {panel(
                    "Recent Requisitions".to_string(),
                    vec![view! { <a href="#" class="btn btn-sm btn-secondary">"View All"</a> }],
                    vec![data_table(columns, rows, None)]
                )}

                // Activity Timeline
                {panel(
                    "Recent Activity".to_string(),
                    vec![],
                    vec![timeline(timeline_items, None)]
                )}
            </div>

            // Charts row
            <div class="grid-2">
                // Spend by Category
                {panel(
                    "Spend by Category".to_string(),
                    vec![],
                    vec![bar_chart(spend_data, Some(200))]
                )}

                // B-BBEE Performance
                {panel(
                    "B-BBEE Performance".to_string(),
                    vec![],
                    vec![
                        pie_chart(bbbee_pie, None),
                        view! {
                            <div style="margin-top: 24px; padding-top: 16px; border-top: 1px solid var(--border);">
                                {progress_bar(bbbee_data.current, ProgressColor::Green, true, None)}
                            </div>
                        },
                    ]
                )}
            </div>
        </div>
    }
}
