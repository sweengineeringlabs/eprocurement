//! NBAC Reviews page - pending adjudications, meeting schedule, decision history

use components::prelude::*;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, data_table, DataTableColumn, DataTableRow,
    status_badge, StatusType,
    tag, TagType,
    kpi_card, KpiColor, KpiDelta,
    timeline, TimelineItem, TimelineStatus,
    tabs, Tab,
};
use crate::util::format::{format_currency, format_date};
use super::store::NbacStore;
use super::types::{ReviewStatus, ReviewCategory, Priority, MeetingStatus, DecisionType};
use super::service;

/// NBAC Reviews page
#[component]
pub fn nbac_reviews() -> View {
    let store = use_context::<NbacStore>();

    // Load data on mount
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                service::load_nbac_data(&store).await;
            });
        }
    });

    let reviews = store.reviews.clone();
    let meetings = store.meetings.clone();
    let decisions = store.decisions.clone();
    let kpis = store.kpis.clone();
    let filter = store.filter.clone();

    // Active tab state
    let active_tab = signal("reviews".to_string());

    // Summary counts
    let pending_count = move |_| reviews.get().iter()
        .filter(|r| matches!(r.status, ReviewStatus::Pending))
        .count();
    let scheduled_count = move |_| reviews.get().iter()
        .filter(|r| matches!(r.status, ReviewStatus::Scheduled))
        .count();
    let in_review_count = move |_| reviews.get().iter()
        .filter(|r| matches!(r.status, ReviewStatus::InReview))
        .count();
    let approved_count = move |_| reviews.get().iter()
        .filter(|r| matches!(r.status, ReviewStatus::Approved))
        .count();

    // Filter reviews
    let filtered_reviews = {
        let store = store.clone();
        move |_| store.filtered_reviews()
    };

    // Upcoming meetings
    let upcoming_meetings = {
        let store = store.clone();
        move |_| store.upcoming_meetings()
    };

    // Recent decisions
    let recent_decisions = {
        let store = store.clone();
        move |_| store.recent_decisions()
    };

    // Reviews table columns
    let review_columns = vec![
        DataTableColumn {
            key: "reference".to_string(),
            label: "Reference".to_string(),
            width: Some("140px".to_string()),
            align: None,
            cell_class: Some("id-cell".to_string()),
        },
        DataTableColumn {
            key: "title".to_string(),
            label: "Title".to_string(),
            width: None,
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "category".to_string(),
            label: "Category".to_string(),
            width: Some("130px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "value".to_string(),
            label: "Value".to_string(),
            width: Some("120px".to_string()),
            align: Some("right".to_string()),
            cell_class: Some("amount-cell".to_string()),
        },
        DataTableColumn {
            key: "department".to_string(),
            label: "Department".to_string(),
            width: Some("150px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "priority".to_string(),
            label: "Priority".to_string(),
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

    // Transform reviews to table rows
    let review_rows = move |_| -> Vec<DataTableRow> {
        filtered_reviews().iter().map(|review| {
            let status = match review.status {
                ReviewStatus::Pending => status_badge(StatusType::Pending),
                ReviewStatus::Scheduled => status_badge(StatusType::Scheduled),
                ReviewStatus::InReview => status_badge(StatusType::InProgress),
                ReviewStatus::Approved => status_badge(StatusType::Approved),
                ReviewStatus::Rejected => status_badge(StatusType::Rejected),
                ReviewStatus::Deferred => status_badge(StatusType::OnHold),
                ReviewStatus::RequiresInfo => status_badge(StatusType::Pending),
            };

            let priority_tag = match review.priority {
                Priority::Critical => tag("Critical".to_string(), TagType::Danger),
                Priority::High => tag("High".to_string(), TagType::Warning),
                Priority::Medium => tag("Medium".to_string(), TagType::Info),
                Priority::Low => tag("Low".to_string(), TagType::Default),
            };

            let category_tag = match review.category {
                ReviewCategory::BidAward => tag("Bid Award".to_string(), TagType::Primary),
                ReviewCategory::Deviation => tag("Deviation".to_string(), TagType::Warning),
                ReviewCategory::ContractVariation => tag("Variation".to_string(), TagType::Info),
                ReviewCategory::ContractExtension => tag("Extension".to_string(), TagType::Info),
                ReviewCategory::Cancellation => tag("Cancellation".to_string(), TagType::Danger),
                ReviewCategory::EmergencyProcurement => tag("Emergency".to_string(), TagType::Danger),
                ReviewCategory::SingleSource => tag("Single Source".to_string(), TagType::Warning),
                ReviewCategory::Confinement => tag("Confinement".to_string(), TagType::Warning),
            };

            DataTableRow {
                id: review.id.clone(),
                cells: vec![
                    view! { <span class="id-cell">{review.reference_number.clone()}</span> },
                    view! { <span class="title-cell">{review.tender_title.clone()}</span> },
                    category_tag,
                    view! { <span class="amount-cell">{format_currency(review.tender_value)}</span> },
                    view! { <span>{review.department.clone()}</span> },
                    priority_tag,
                    status,
                ],
            }
        }).collect()
    };

    // Meetings table columns
    let meeting_columns = vec![
        DataTableColumn {
            key: "number".to_string(),
            label: "Meeting No.".to_string(),
            width: Some("160px".to_string()),
            align: None,
            cell_class: Some("id-cell".to_string()),
        },
        DataTableColumn {
            key: "date".to_string(),
            label: "Date".to_string(),
            width: Some("120px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "time".to_string(),
            label: "Time".to_string(),
            width: Some("80px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "venue".to_string(),
            label: "Venue".to_string(),
            width: None,
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "items".to_string(),
            label: "Agenda Items".to_string(),
            width: Some("100px".to_string()),
            align: Some("center".to_string()),
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

    // Transform meetings to table rows
    let meeting_rows = move |_| -> Vec<DataTableRow> {
        meetings.get().iter().map(|meeting| {
            let status = match meeting.status {
                MeetingStatus::Scheduled => status_badge(StatusType::Scheduled),
                MeetingStatus::InProgress => status_badge(StatusType::InProgress),
                MeetingStatus::Completed => status_badge(StatusType::Complete),
                MeetingStatus::Cancelled => status_badge(StatusType::Rejected),
                MeetingStatus::Postponed => status_badge(StatusType::OnHold),
            };

            DataTableRow {
                id: meeting.id.clone(),
                cells: vec![
                    view! { <span class="id-cell">{meeting.meeting_number.clone()}</span> },
                    view! { <span>{format_date(&meeting.date)}</span> },
                    view! { <span>{meeting.time.clone()}</span> },
                    view! { <span>{meeting.venue.clone()}</span> },
                    view! {
                        <span class="agenda-count">
                            {tag(meeting.agenda.items.len().to_string(), TagType::Info)}
                        </span>
                    },
                    status,
                ],
            }
        }).collect()
    };

    // Decisions table columns
    let decision_columns = vec![
        DataTableColumn {
            key: "resolution".to_string(),
            label: "Resolution No.".to_string(),
            width: Some("160px".to_string()),
            align: None,
            cell_class: Some("id-cell".to_string()),
        },
        DataTableColumn {
            key: "date".to_string(),
            label: "Date".to_string(),
            width: Some("100px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "summary".to_string(),
            label: "Summary".to_string(),
            width: None,
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "type".to_string(),
            label: "Type".to_string(),
            width: Some("100px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "value".to_string(),
            label: "Value".to_string(),
            width: Some("120px".to_string()),
            align: Some("right".to_string()),
            cell_class: Some("amount-cell".to_string()),
        },
        DataTableColumn {
            key: "votes".to_string(),
            label: "Votes".to_string(),
            width: Some("100px".to_string()),
            align: Some("center".to_string()),
            cell_class: None,
        },
    ];

    // Transform decisions to table rows
    let decision_rows = move |_| -> Vec<DataTableRow> {
        decisions.get().iter().map(|decision| {
            let type_tag = match decision.decision_type {
                DecisionType::Award => tag("Award".to_string(), TagType::Success),
                DecisionType::Reject => tag("Reject".to_string(), TagType::Danger),
                DecisionType::Defer => tag("Defer".to_string(), TagType::Warning),
                DecisionType::RequestInfo => tag("Info Req".to_string(), TagType::Info),
                DecisionType::Cancellation => tag("Cancel".to_string(), TagType::Danger),
                DecisionType::Variation => tag("Variation".to_string(), TagType::Info),
                DecisionType::Extension => tag("Extension".to_string(), TagType::Info),
            };

            let vote_display = if decision.is_unanimous {
                view! { <span class="vote-unanimous">"Unanimous"</span> }
            } else {
                view! {
                    <span class="vote-count">
                        <span class="vote-for">{decision.votes_for}</span>
                        "/"
                        <span class="vote-against">{decision.votes_against}</span>
                        "/"
                        <span class="vote-abstain">{decision.votes_abstain}</span>
                    </span>
                }
            };

            let value_display = decision.approved_value
                .map(|v| format_currency(v))
                .unwrap_or_else(|| "-".to_string());

            DataTableRow {
                id: decision.id.clone(),
                cells: vec![
                    view! { <span class="id-cell">{decision.resolution_number.clone()}</span> },
                    view! { <span>{format_date(&decision.decision_date)}</span> },
                    view! { <span class="summary-cell">{decision.summary.clone()}</span> },
                    type_tag,
                    view! { <span class="amount-cell">{value_display}</span> },
                    vote_display,
                ],
            }
        }).collect()
    };

    // Handle row clicks
    let handle_review_click = Callback::new({
        let store = store.clone();
        move |review_id: String| {
            store.select_review(&review_id);
            // In production, would navigate to detail view
        }
    });

    let handle_meeting_click = Callback::new({
        let store = store.clone();
        move |meeting_id: String| {
            store.select_meeting(&meeting_id);
            // In production, would navigate to detail view
        }
    });

    // Filter handlers
    let set_filter_all = {
        let store = store.clone();
        Callback::<()>::new(move |_| store.set_status_filter(None))
    };
    let set_filter_pending = {
        let store = store.clone();
        Callback::<()>::new(move |_| store.set_status_filter(Some(ReviewStatus::Pending)))
    };
    let set_filter_scheduled = {
        let store = store.clone();
        Callback::<()>::new(move |_| store.set_status_filter(Some(ReviewStatus::Scheduled)))
    };
    let set_filter_approved = {
        let store = store.clone();
        Callback::<()>::new(move |_| store.set_status_filter(Some(ReviewStatus::Approved)))
    };

    // Tab change handler
    let on_tab_change = {
        let active_tab = active_tab.clone();
        Callback::new(move |tab: String| {
            active_tab.set(tab);
        })
    };

    // Icons
    let icon_clipboard = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/><rect x="8" y="2" width="8" height="4" rx="1" ry="1"/></svg>"#;
    let icon_clock = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>"#;
    let icon_check = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>"#;
    let icon_calendar = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="4" width="18" height="18" rx="2" ry="2"/><line x1="16" y1="2" x2="16" y2="6"/><line x1="8" y1="2" x2="8" y2="6"/><line x1="3" y1="10" x2="21" y2="10"/></svg>"#;
    let icon_dollar = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8"/><path d="M12 18V6"/></svg>"#;
    let icon_trending = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="23 6 13.5 15.5 8.5 10.5 1 18"/><polyline points="17 6 23 6 23 12"/></svg>"#;
    let icon_alert = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>"#;
    let icon_users = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>"#;

    // Timeline items for upcoming meetings
    let meeting_timeline = move |_| -> Vec<TimelineItem> {
        upcoming_meetings().iter().map(|meeting| {
            TimelineItem {
                date: format!("{} at {}", format_date(&meeting.date), meeting.time),
                title: meeting.meeting_number.clone(),
                description: format!("{} - {} items", meeting.venue, meeting.agenda.items.len()),
                status: TimelineStatus::Pending,
            }
        }).collect()
    };

    view! {
        style {
            r#"
            .nbac-reviews {
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
            .summary-icon.pending { background: var(--orange-light); color: var(--orange); }
            .summary-icon.scheduled { background: var(--blue-light); color: var(--blue); }
            .summary-icon.in-review { background: var(--cyan-light); color: var(--cyan); }
            .summary-icon.approved { background: var(--green-light); color: var(--green); }
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
                max-width: 250px;
                white-space: nowrap;
                overflow: hidden;
                text-overflow: ellipsis;
            }
            .summary-cell {
                max-width: 300px;
                white-space: nowrap;
                overflow: hidden;
                text-overflow: ellipsis;
            }
            .vote-unanimous {
                color: var(--green);
                font-weight: 500;
            }
            .vote-count {
                font-family: IBM Plex Mono, monospace;
                font-size: 12px;
            }
            .vote-for { color: var(--green); }
            .vote-against { color: var(--red); }
            .vote-abstain { color: var(--text-muted); }
            .sidebar {
                display: flex;
                flex-direction: column;
                gap: 24px;
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
            "#
        }

        <div class="nbac-reviews" data-testid="nbac-reviews">
            {page_header(
                "NBAC - Bid Adjudication".to_string(),
                Some("National Bid Adjudication Committee reviews and decisions".to_string()),
                vec![
                    view! { <button class="btn btn-secondary">"Export Report"</button> },
                    view! { <button class="btn btn-primary">"Schedule Meeting"</button> },
                ]
            )}

            // KPI Row
            <div class="kpi-grid">
                {kpi_card(
                    "Pending Reviews".to_string(),
                    kpis.get().pending_reviews.to_string(),
                    KpiColor::Orange,
                    icon_clipboard.to_string(),
                    Some(KpiDelta { value: "3 urgent".to_string(), is_positive: None, suffix: "".to_string() }),
                    None
                )}
                {kpi_card(
                    "Decisions YTD".to_string(),
                    kpis.get().decisions_ytd.to_string(),
                    KpiColor::Green,
                    icon_check.to_string(),
                    Some(KpiDelta { value: format!("{:.0}% approval rate", kpis.get().approval_rate), is_positive: Some(true), suffix: "".to_string() }),
                    None
                )}
                {kpi_card(
                    "Total Approved YTD".to_string(),
                    format_currency(kpis.get().total_value_approved_ytd),
                    KpiColor::Blue,
                    icon_dollar.to_string(),
                    Some(KpiDelta { value: "4 this month".to_string(), is_positive: Some(true), suffix: "".to_string() }),
                    None
                )}
                {kpi_card(
                    "Avg Turnaround".to_string(),
                    format!("{:.1} days", kpis.get().average_turnaround_days),
                    KpiColor::Cyan,
                    icon_trending.to_string(),
                    Some(KpiDelta { value: "Target: 14 days".to_string(), is_positive: Some(true), suffix: "".to_string() }),
                    None
                )}
            </div>

            // Summary filter cards
            <div class="summary-cards">
                <div
                    class={move |_| if filter.get().status.is_none() { "summary-card active" } else { "summary-card" }}
                    on:click={set_filter_all.clone()}
                >
                    <div class="summary-icon pending" inner_html={icon_clipboard}></div>
                    <div class="summary-content">
                        <h3>{move |_| reviews.get().len()}</h3>
                        <p>"Total Reviews"</p>
                    </div>
                </div>
                <div
                    class={move |_| if matches!(filter.get().status, Some(ReviewStatus::Pending)) { "summary-card active" } else { "summary-card" }}
                    on:click={set_filter_pending}
                >
                    <div class="summary-icon pending" inner_html={icon_clock}></div>
                    <div class="summary-content">
                        <h3>{pending_count}</h3>
                        <p>"Pending"</p>
                    </div>
                </div>
                <div
                    class={move |_| if matches!(filter.get().status, Some(ReviewStatus::Scheduled)) { "summary-card active" } else { "summary-card" }}
                    on:click={set_filter_scheduled}
                >
                    <div class="summary-icon scheduled" inner_html={icon_calendar}></div>
                    <div class="summary-content">
                        <h3>{scheduled_count}</h3>
                        <p>"Scheduled"</p>
                    </div>
                </div>
                <div
                    class={move |_| if matches!(filter.get().status, Some(ReviewStatus::Approved)) { "summary-card active" } else { "summary-card" }}
                    on:click={set_filter_approved}
                >
                    <div class="summary-icon approved" inner_html={icon_check}></div>
                    <div class="summary-content">
                        <h3>{approved_count}</h3>
                        <p>"Approved"</p>
                    </div>
                </div>
            </div>

            // Main content with sidebar
            <div class="content-grid">
                // Main content area with tabs
                <div class="main-content">
                    {tabs(
                        vec![
                            Tab { id: "reviews".to_string(), label: "Pending Reviews".to_string() },
                            Tab { id: "meetings".to_string(), label: "Meeting Schedule".to_string() },
                            Tab { id: "decisions".to_string(), label: "Decision History".to_string() },
                        ],
                        active_tab.get(),
                        on_tab_change.clone()
                    )}

                    // Tab content
                    {move |_| match active_tab.get().as_str() {
                        "reviews" => view! {
                            {panel(
                                "Reviews Pending Adjudication".to_string(),
                                vec![
                                    view! { <button class="btn btn-sm btn-secondary">"Filter"</button> },
                                ],
                                vec![data_table(review_columns.clone(), review_rows(), Some(handle_review_click.clone()))]
                            )}
                        },
                        "meetings" => view! {
                            {panel(
                                "NBAC Meeting Schedule".to_string(),
                                vec![
                                    view! { <button class="btn btn-sm btn-primary">"+ New Meeting"</button> },
                                ],
                                vec![data_table(meeting_columns.clone(), meeting_rows(), Some(handle_meeting_click.clone()))]
                            )}
                        },
                        "decisions" => view! {
                            {panel(
                                "Decision History".to_string(),
                                vec![
                                    view! { <button class="btn btn-sm btn-secondary">"Export"</button> },
                                ],
                                vec![data_table(decision_columns.clone(), decision_rows(), None)]
                            )}
                        },
                        _ => view! { <div></div> },
                    }}
                </div>

                // Sidebar
                <div class="sidebar">
                    // Upcoming meetings
                    {panel(
                        "Upcoming Meetings".to_string(),
                        vec![],
                        vec![timeline(meeting_timeline(), None)]
                    )}

                    // Quick stats
                    <div class="quick-stats">
                        <h4>"Committee Statistics"</h4>
                        <div class="stat-row">
                            <span class="stat-label">"Decisions This Month"</span>
                            <span class="stat-value">{move |_| kpis.get().decisions_this_month}</span>
                        </div>
                        <div class="stat-row">
                            <span class="stat-label">"Approval Rate"</span>
                            <span class="stat-value highlight">{move |_| format!("{:.0}%", kpis.get().approval_rate)}</span>
                        </div>
                        <div class="stat-row">
                            <span class="stat-label">"Overdue Reviews"</span>
                            <span class="stat-value warning">{move |_| kpis.get().overdue_reviews}</span>
                        </div>
                        <div class="stat-row">
                            <span class="stat-label">"Upcoming Meetings"</span>
                            <span class="stat-value">{move |_| kpis.get().upcoming_meetings}</span>
                        </div>
                    </div>

                    // Committee members quick view
                    {panel(
                        "Committee Members".to_string(),
                        vec![view! { <a href="#" class="btn btn-sm btn-secondary">"View All"</a> }],
                        vec![view! {
                            <div class="committee-quick">
                                <div class="stat-row">
                                    <span class="stat-label">"Active Members"</span>
                                    <span class="stat-value">"7"</span>
                                </div>
                                <div class="stat-row">
                                    <span class="stat-label">"Quorum Required"</span>
                                    <span class="stat-value">"5"</span>
                                </div>
                                <div class="stat-row">
                                    <span class="stat-label">"Avg Attendance"</span>
                                    <span class="stat-value highlight">"93%"</span>
                                </div>
                            </div>
                        }]
                    )}
                </div>
            </div>
        </div>
    }
}
