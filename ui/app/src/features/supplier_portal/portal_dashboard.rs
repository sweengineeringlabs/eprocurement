//! Supplier Portal Dashboard
//!
//! Main dashboard for suppliers showing opportunities, bid submissions, and contract awards.

use components::prelude::*;
use crate::shared::layout::page_header;
use crate::shared::components::{
    kpi_card, KpiColor, KpiDelta,
    panel, data_table, DataTableColumn, DataTableRow,
    status_badge, StatusType,
    timeline, TimelineItem, TimelineStatus,
    progress_bar, ProgressColor,
    notice_bar, NoticeType,
    empty_state,
};
use crate::util::format::{format_currency, format_date};
use super::store::{SupplierPortalStore, load_mock_portal_data};
use super::types::{OpportunityStatus, BidSubmissionStatus, ContractAwardStatus};
use super::service;

/// Portal Dashboard component
#[component]
pub fn portal_dashboard() -> View {
    let store = use_context::<SupplierPortalStore>();

    // Load data on mount
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                service::load_portal_data(&store).await;
            });
        }
    });

    let kpis = store.kpis.clone();
    let opportunities = store.opportunities.clone();
    let submissions = store.submissions.clone();
    let awards = store.awards.clone();
    let notifications = store.notifications.clone();
    let active_tab = store.active_tab.clone();
    let unread_count = store.unread_count.clone();

    // Tab handler
    let set_tab = {
        let store = store.clone();
        Callback::new(move |tab: String| {
            store.set_active_tab(&tab);
        })
    };

    // Icons
    let icon_briefcase = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="2" y="7" width="20" height="14" rx="2" ry="2"/><path d="M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16"/></svg>"#;
    let icon_clock = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>"#;
    let icon_file = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>"#;
    let icon_award = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="8" r="7"/><polyline points="8.21 13.89 7 23 12 20 17 23 15.79 13.88"/></svg>"#;
    let icon_dollar = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8"/><path d="M12 18V6"/></svg>"#;
    let icon_doc = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><path d="M14 2v6h6"/><path d="M12 18v-6"/><path d="M9 15h6"/></svg>"#;
    let icon_percent = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="19" y1="5" x2="5" y2="19"/><circle cx="6.5" cy="6.5" r="2.5"/><circle cx="17.5" cy="17.5" r="2.5"/></svg>"#;
    let icon_trending = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="23 6 13.5 15.5 8.5 10.5 1 18"/><polyline points="17 6 23 6 23 12"/></svg>"#;

    view! {
        style {
            r#"
            .portal-dashboard {
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
            @media (max-width: 768px) {
                .kpi-grid { grid-template-columns: 1fr; }
            }
            .portal-tabs {
                display: flex;
                gap: 8px;
                border-bottom: 1px solid var(--border);
                padding-bottom: 0;
                margin-bottom: 24px;
            }
            .portal-tab {
                padding: 12px 24px;
                background: none;
                border: none;
                color: var(--text-muted);
                font-size: 14px;
                font-weight: 500;
                cursor: pointer;
                border-bottom: 2px solid transparent;
                margin-bottom: -1px;
                transition: all 0.2s;
            }
            .portal-tab:hover {
                color: var(--text);
            }
            .portal-tab.active {
                color: var(--blue);
                border-bottom-color: var(--blue);
            }
            .portal-tab .badge {
                display: inline-flex;
                align-items: center;
                justify-content: center;
                min-width: 20px;
                height: 20px;
                padding: 0 6px;
                margin-left: 8px;
                background: var(--blue-light);
                color: var(--blue);
                border-radius: 10px;
                font-size: 11px;
                font-weight: 600;
            }
            .portal-tab.active .badge {
                background: var(--blue);
                color: white;
            }
            .tab-content {
                display: none;
            }
            .tab-content.active {
                display: block;
            }
            .opportunity-card {
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius-lg);
                padding: 20px;
                margin-bottom: 16px;
                transition: box-shadow 0.2s;
            }
            .opportunity-card:hover {
                box-shadow: var(--shadow);
            }
            .opportunity-header {
                display: flex;
                justify-content: space-between;
                align-items: flex-start;
                margin-bottom: 12px;
            }
            .opportunity-ref {
                font-size: 12px;
                color: var(--text-muted);
                margin-bottom: 4px;
            }
            .opportunity-title {
                font-size: 16px;
                font-weight: 600;
                color: var(--navy);
                margin-bottom: 8px;
            }
            .opportunity-meta {
                display: flex;
                gap: 24px;
                flex-wrap: wrap;
                margin-top: 12px;
                padding-top: 12px;
                border-top: 1px solid var(--border);
            }
            .opportunity-meta-item {
                display: flex;
                flex-direction: column;
                gap: 2px;
            }
            .opportunity-meta-label {
                font-size: 11px;
                color: var(--text-muted);
            }
            .opportunity-meta-value {
                font-size: 13px;
                font-weight: 500;
                color: var(--text);
            }
            .opportunity-actions {
                display: flex;
                gap: 8px;
                margin-top: 16px;
            }
            .days-badge {
                display: inline-flex;
                align-items: center;
                gap: 4px;
                padding: 4px 10px;
                border-radius: 12px;
                font-size: 12px;
                font-weight: 500;
            }
            .days-badge.urgent {
                background: var(--red-light);
                color: var(--red);
            }
            .days-badge.soon {
                background: var(--orange-light);
                color: var(--orange);
            }
            .days-badge.normal {
                background: var(--blue-light);
                color: var(--blue);
            }
            .submission-progress {
                display: flex;
                align-items: center;
                gap: 12px;
            }
            .submission-progress-bar {
                flex: 1;
                height: 6px;
                background: var(--bg);
                border-radius: 3px;
                overflow: hidden;
            }
            .submission-progress-fill {
                height: 100%;
                background: var(--blue);
                border-radius: 3px;
                transition: width 0.3s;
            }
            .submission-progress-text {
                font-size: 12px;
                color: var(--text-muted);
                white-space: nowrap;
            }
            .notification-list {
                display: flex;
                flex-direction: column;
                gap: 8px;
            }
            .notification-item {
                display: flex;
                gap: 12px;
                padding: 12px;
                background: var(--bg);
                border-radius: var(--radius);
                cursor: pointer;
                transition: background 0.2s;
            }
            .notification-item:hover {
                background: var(--border);
            }
            .notification-item.unread {
                background: var(--blue-light);
            }
            .notification-dot {
                width: 8px;
                height: 8px;
                border-radius: 50%;
                background: var(--blue);
                margin-top: 6px;
                flex-shrink: 0;
            }
            .notification-content {
                flex: 1;
            }
            .notification-title {
                font-size: 13px;
                font-weight: 500;
                color: var(--text);
                margin-bottom: 2px;
            }
            .notification-message {
                font-size: 12px;
                color: var(--text-muted);
            }
            .notification-time {
                font-size: 11px;
                color: var(--text-muted);
                white-space: nowrap;
            }
            .status-open { background: var(--blue-light); color: var(--blue); padding: 4px 10px; border-radius: 12px; font-size: 11px; font-weight: 500; }
            .status-closing-soon { background: var(--orange-light); color: var(--orange); padding: 4px 10px; border-radius: 12px; font-size: 11px; font-weight: 500; }
            .status-closed { background: var(--text-muted); color: white; padding: 4px 10px; border-radius: 12px; font-size: 11px; font-weight: 500; }
            .status-submitted { background: var(--blue-light); color: var(--blue); padding: 4px 10px; border-radius: 12px; font-size: 11px; font-weight: 500; }
            .status-shortlisted { background: var(--purple-light); color: var(--purple); padding: 4px 10px; border-radius: 12px; font-size: 11px; font-weight: 500; }
            .status-draft { background: var(--orange-light); color: var(--orange); padding: 4px 10px; border-radius: 12px; font-size: 11px; font-weight: 500; }
            .status-evaluation { background: var(--purple-light); color: var(--purple); padding: 4px 10px; border-radius: 12px; font-size: 11px; font-weight: 500; }
            .status-awarded { background: var(--green-light); color: var(--green); padding: 4px 10px; border-radius: 12px; font-size: 11px; font-weight: 500; }
            .status-unsuccessful { background: var(--red-light); color: var(--red); padding: 4px 10px; border-radius: 12px; font-size: 11px; font-weight: 500; }
            .status-active { background: var(--green-light); color: var(--green); padding: 4px 10px; border-radius: 12px; font-size: 11px; font-weight: 500; }
            .status-awaiting { background: var(--orange-light); color: var(--orange); padding: 4px 10px; border-radius: 12px; font-size: 11px; font-weight: 500; }
            .status-completed { background: var(--text-muted); color: white; padding: 4px 10px; border-radius: 12px; font-size: 11px; font-weight: 500; }
            "#
        }

        <div class="portal-dashboard" data-testid="portal-dashboard">
            {page_header(
                "Supplier Portal".to_string(),
                Some("Manage your tenders, bids, and contracts".to_string()),
                vec![
                    view! { <button class="btn btn-secondary">"My Profile"</button> },
                    view! { <button class="btn btn-primary">"Browse Opportunities"</button> },
                ]
            )}

            // Notification banner if there are urgent items
            if unread_count.get() > 0 {
                {notice_bar(
                    NoticeType::Info,
                    format!("You have {} unread notification{}", unread_count.get(), if unread_count.get() == 1 { "" } else { "s" }),
                    Some(vec![view! { <button class="btn btn-sm btn-secondary">"View All"</button> }])
                )}
            }

            // KPI Cards
            <div class="kpi-grid">
                {kpi_card(
                    "Open Opportunities".to_string(),
                    kpis.get().open_opportunities.to_string(),
                    KpiColor::Blue,
                    icon_briefcase.to_string(),
                    Some(KpiDelta { value: format!("{} closing soon", kpis.get().closing_soon), is_positive: None, suffix: "".to_string() }),
                    None
                )}
                {kpi_card(
                    "Active Bids".to_string(),
                    kpis.get().active_bids.to_string(),
                    KpiColor::Orange,
                    icon_file.to_string(),
                    Some(KpiDelta { value: "2 under evaluation".to_string(), is_positive: None, suffix: "".to_string() }),
                    None
                )}
                {kpi_card(
                    "Awarded Contracts".to_string(),
                    kpis.get().awarded_contracts.to_string(),
                    KpiColor::Green,
                    icon_award.to_string(),
                    Some(KpiDelta { value: format_currency(kpis.get().total_contract_value), is_positive: None, suffix: "total value".to_string() }),
                    None
                )}
                {kpi_card(
                    "Success Rate".to_string(),
                    format!("{:.1}%", kpis.get().success_rate),
                    KpiColor::Purple,
                    icon_percent.to_string(),
                    Some(KpiDelta { value: "5%".to_string(), is_positive: Some(true), suffix: "from last year".to_string() }),
                    None
                )}
            </div>

            // Tabs
            <div class="portal-tabs">
                <button
                    class={if active_tab.get() == "opportunities" { "portal-tab active" } else { "portal-tab" }}
                    on:click={let set_tab = set_tab.clone(); move || set_tab.call("opportunities".to_string())}
                >
                    "Opportunities"
                    <span class="badge">{opportunities.get().iter().filter(|o| o.status == OpportunityStatus::Open || o.status == OpportunityStatus::ClosingSoon).count()}</span>
                </button>
                <button
                    class={if active_tab.get() == "submissions" { "portal-tab active" } else { "portal-tab" }}
                    on:click={let set_tab = set_tab.clone(); move || set_tab.call("submissions".to_string())}
                >
                    "My Submissions"
                    <span class="badge">{submissions.get().len()}</span>
                </button>
                <button
                    class={if active_tab.get() == "awards" { "portal-tab active" } else { "portal-tab" }}
                    on:click={let set_tab = set_tab.clone(); move || set_tab.call("awards".to_string())}
                >
                    "Contract Awards"
                    <span class="badge">{awards.get().iter().filter(|a| a.status == ContractAwardStatus::Active || a.status == ContractAwardStatus::AwaitingSignature).count()}</span>
                </button>
            </div>

            // Tab Content - Opportunities
            <div class={if active_tab.get() == "opportunities" { "tab-content active" } else { "tab-content" }}>
                <div class="grid-3-2">
                    <div>
                        for opp in opportunities.get().iter().filter(|o| o.status != OpportunityStatus::Closed) {
                            <div class="opportunity-card">
                                <div class="opportunity-header">
                                    <div>
                                        <div class="opportunity-ref">{opp.reference_number.clone()}</div>
                                        <div class="opportunity-title">{opp.title.clone()}</div>
                                        <span class={opp.status.css_class()}>{opp.status.label()}</span>
                                    </div>
                                    <div class={format!("days-badge {}", if opp.days_remaining <= 3 { "urgent" } else if opp.days_remaining <= 7 { "soon" } else { "normal" })}>
                                        {format!("{} days left", opp.days_remaining)}
                                    </div>
                                </div>
                                <div class="opportunity-meta">
                                    <div class="opportunity-meta-item">
                                        <span class="opportunity-meta-label">"Category"</span>
                                        <span class="opportunity-meta-value">{opp.category.clone()}</span>
                                    </div>
                                    <div class="opportunity-meta-item">
                                        <span class="opportunity-meta-label">"Est. Value"</span>
                                        <span class="opportunity-meta-value">{format_currency(opp.estimated_value)}</span>
                                    </div>
                                    <div class="opportunity-meta-item">
                                        <span class="opportunity-meta-label">"Closing Date"</span>
                                        <span class="opportunity-meta-value">{format_date(&opp.closing_date)}</span>
                                    </div>
                                    <div class="opportunity-meta-item">
                                        <span class="opportunity-meta-label">"Documents"</span>
                                        <span class="opportunity-meta-value">{format!("{} files", opp.document_count)}</span>
                                    </div>
                                </div>
                                <div class="opportunity-actions">
                                    <button class="btn btn-sm btn-secondary">"View Details"</button>
                                    <button class="btn btn-sm btn-secondary">"Download Docs"</button>
                                    if opp.status != OpportunityStatus::BidSubmitted {
                                        <button class="btn btn-sm btn-primary">"Start Bid"</button>
                                    }
                                    if opp.mandatory_briefing && opp.briefing_date.is_some() {
                                        <button class="btn btn-sm btn-accent">"Register for Briefing"</button>
                                    }
                                </div>
                            </div>
                        }
                    </div>

                    // Notifications sidebar
                    {panel(
                        "Notifications".to_string(),
                        vec![view! { <button class="btn btn-sm btn-secondary">"Mark All Read"</button> }],
                        vec![view! {
                            <div class="notification-list">
                                for notif in notifications.get().iter().take(5) {
                                    <div class={if notif.read { "notification-item" } else { "notification-item unread" }}>
                                        if !notif.read {
                                            <div class="notification-dot"></div>
                                        }
                                        <div class="notification-content">
                                            <div class="notification-title">{notif.title.clone()}</div>
                                            <div class="notification-message">{notif.message.clone()}</div>
                                        </div>
                                        <div class="notification-time">{format_date(&notif.created_at)}</div>
                                    </div>
                                }
                            </div>
                        }]
                    )}
                </div>
            </div>

            // Tab Content - Submissions
            <div class={if active_tab.get() == "submissions" { "tab-content active" } else { "tab-content" }}>
                {panel(
                    "My Bid Submissions".to_string(),
                    vec![
                        view! { <button class="btn btn-sm btn-secondary">"Filter"</button> },
                        view! { <button class="btn btn-sm btn-secondary">"Export"</button> },
                    ],
                    vec![{
                        let columns = vec![
                            DataTableColumn { key: "ref".to_string(), label: "Reference".to_string(), width: None, align: None, cell_class: Some("id-cell".to_string()) },
                            DataTableColumn { key: "title".to_string(), label: "Tender Title".to_string(), width: None, align: None, cell_class: None },
                            DataTableColumn { key: "price".to_string(), label: "Bid Price".to_string(), width: None, align: Some("right".to_string()), cell_class: Some("amount-cell".to_string()) },
                            DataTableColumn { key: "status".to_string(), label: "Status".to_string(), width: None, align: None, cell_class: None },
                            DataTableColumn { key: "docs".to_string(), label: "Documents".to_string(), width: None, align: None, cell_class: None },
                            DataTableColumn { key: "score".to_string(), label: "Score".to_string(), width: None, align: Some("right".to_string()), cell_class: None },
                            DataTableColumn { key: "actions".to_string(), label: "Actions".to_string(), width: None, align: None, cell_class: None },
                        ];

                        let rows: Vec<DataTableRow> = submissions.get().iter().map(|sub| {
                            let status_class = sub.status.css_class();
                            let status_label = sub.status.label();
                            let doc_progress = if sub.documents_required > 0 {
                                (sub.documents_uploaded as f64 / sub.documents_required as f64) * 100.0
                            } else {
                                0.0
                            };

                            DataTableRow {
                                id: sub.id.clone(),
                                cells: vec![
                                    view! { <span class="id-cell">{sub.tender_reference.clone()}</span> },
                                    view! { <span>{sub.tender_title.clone()}</span> },
                                    view! { <span class="amount-cell">{if sub.total_price > 0.0 { format_currency(sub.total_price) } else { "-".to_string() }}</span> },
                                    view! { <span class={status_class}>{status_label}</span> },
                                    view! {
                                        <div class="submission-progress">
                                            <div class="submission-progress-bar">
                                                <div class="submission-progress-fill" style={format!("width: {}%", doc_progress)}></div>
                                            </div>
                                            <span class="submission-progress-text">{format!("{}/{}", sub.documents_uploaded, sub.documents_required)}</span>
                                        </div>
                                    },
                                    view! { <span>{if let Some(score) = sub.total_score { format!("{:.1}%", score) } else { "-".to_string() }}</span> },
                                    view! {
                                        <div style="display: flex; gap: 4px;">
                                            <button class="btn btn-sm btn-secondary">"View"</button>
                                            if sub.status == BidSubmissionStatus::Draft {
                                                <button class="btn btn-sm btn-primary">"Edit"</button>
                                            }
                                        </div>
                                    },
                                ],
                            }
                        }).collect();

                        data_table(columns, rows, None)
                    }]
                )}
            </div>

            // Tab Content - Awards
            <div class={if active_tab.get() == "awards" { "tab-content active" } else { "tab-content" }}>
                {panel(
                    "Contract Awards".to_string(),
                    vec![
                        view! { <button class="btn btn-sm btn-secondary">"Filter"</button> },
                        view! { <button class="btn btn-sm btn-secondary">"Export"</button> },
                    ],
                    vec![{
                        let columns = vec![
                            DataTableColumn { key: "contract".to_string(), label: "Contract No.".to_string(), width: None, align: None, cell_class: Some("id-cell".to_string()) },
                            DataTableColumn { key: "title".to_string(), label: "Title".to_string(), width: None, align: None, cell_class: None },
                            DataTableColumn { key: "value".to_string(), label: "Value".to_string(), width: None, align: Some("right".to_string()), cell_class: Some("amount-cell".to_string()) },
                            DataTableColumn { key: "period".to_string(), label: "Contract Period".to_string(), width: None, align: None, cell_class: None },
                            DataTableColumn { key: "status".to_string(), label: "Status".to_string(), width: None, align: None, cell_class: None },
                            DataTableColumn { key: "next".to_string(), label: "Next Milestone".to_string(), width: None, align: None, cell_class: None },
                            DataTableColumn { key: "actions".to_string(), label: "Actions".to_string(), width: None, align: None, cell_class: None },
                        ];

                        let rows: Vec<DataTableRow> = awards.get().iter().map(|award| {
                            let status_class = award.status.css_class();
                            let status_label = award.status.label();

                            DataTableRow {
                                id: award.id.clone(),
                                cells: vec![
                                    view! { <span class="id-cell">{award.contract_number.clone()}</span> },
                                    view! { <span>{award.title.clone()}</span> },
                                    view! { <span class="amount-cell">{format_currency(award.value)}</span> },
                                    view! { <span>{format!("{} - {}", format_date(&award.start_date), format_date(&award.end_date))}</span> },
                                    view! { <span class={status_class}>{status_label}</span> },
                                    view! { <span>{award.next_milestone.clone().unwrap_or("-".to_string())}</span> },
                                    view! {
                                        <div style="display: flex; gap: 4px;">
                                            <button class="btn btn-sm btn-secondary">"View"</button>
                                            <button class="btn btn-sm btn-secondary">"Documents"</button>
                                        </div>
                                    },
                                ],
                            }
                        }).collect();

                        data_table(columns, rows, None)
                    }]
                )}

                // Contract summary cards
                <div class="grid-2" style="margin-top: 24px;">
                    {panel(
                        "Revenue Summary".to_string(),
                        vec![],
                        vec![view! {
                            <div style="display: flex; flex-direction: column; gap: 16px;">
                                <div style="display: flex; justify-content: space-between; align-items: center;">
                                    <span style="color: var(--text-muted);">"YTD Revenue"</span>
                                    <span style="font-size: 24px; font-weight: 600; color: var(--green);">{format_currency(kpis.get().ytd_revenue)}</span>
                                </div>
                                <div style="display: flex; justify-content: space-between; align-items: center;">
                                    <span style="color: var(--text-muted);">"Total Contract Value"</span>
                                    <span style="font-size: 18px; font-weight: 500;">{format_currency(kpis.get().total_contract_value)}</span>
                                </div>
                                <div>
                                    <div style="display: flex; justify-content: space-between; margin-bottom: 8px;">
                                        <span style="font-size: 12px; color: var(--text-muted);">"Contract Utilization"</span>
                                        <span style="font-size: 12px; color: var(--text-muted);">"28%"</span>
                                    </div>
                                    {progress_bar(28.0, ProgressColor::Blue, false, None)}
                                </div>
                            </div>
                        }]
                    )}

                    {panel(
                        "Upcoming Milestones".to_string(),
                        vec![],
                        vec![{
                            let milestone_items: Vec<TimelineItem> = awards.get().iter()
                                .filter(|a| a.next_milestone.is_some() && a.status == ContractAwardStatus::Active)
                                .take(3)
                                .map(|a| TimelineItem {
                                    date: a.next_milestone_date.clone().unwrap_or_default(),
                                    title: a.next_milestone.clone().unwrap_or_default(),
                                    description: a.title.clone(),
                                    status: TimelineStatus::Pending,
                                })
                                .collect();

                            timeline(milestone_items, None)
                        }]
                    )}
                </div>
            </div>
        </div>
    }
}
