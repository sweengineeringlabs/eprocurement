//! Evaluation list page - tenders pending evaluation

use components::prelude::*;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, data_table, DataTableColumn, DataTableRow,
    status_badge, StatusType,
    tag, TagType,
};
use crate::util::format::{format_currency, format_date};
use super::store::EvaluationStore;
use super::types::EvaluationStatus;
use super::service;

/// Evaluation list page
#[component]
pub fn evaluation_list() -> View {
    let store = use_context::<EvaluationStore>();

    // Load data on mount
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                service::load_evaluations(&store).await;
            });
        }
    });

    let evaluations = store.evaluations.clone();
    let filter_status = store.filter_status.clone();

    // Filter evaluations based on selected status
    let filtered_evaluations = move |_| {
        let all_evals = evaluations.get();
        match filter_status.get().as_ref() {
            Some(status) => all_evals.into_iter()
                .filter(|e| &e.status == status)
                .collect::<Vec<_>>(),
            None => all_evals,
        }
    };

    // Summary counts
    let pending_count = move |_| evaluations.get().iter()
        .filter(|e| matches!(e.status, EvaluationStatus::Pending))
        .count();
    let in_progress_count = move |_| evaluations.get().iter()
        .filter(|e| matches!(e.status, EvaluationStatus::InProgress))
        .count();
    let completed_count = move |_| evaluations.get().iter()
        .filter(|e| matches!(e.status, EvaluationStatus::Completed))
        .count();
    let approved_count = move |_| evaluations.get().iter()
        .filter(|e| matches!(e.status, EvaluationStatus::Approved))
        .count();

    // Table columns
    let columns = vec![
        DataTableColumn {
            key: "reference".to_string(),
            label: "Tender Ref".to_string(),
            width: Some("120px".to_string()),
            align: None,
            cell_class: Some("id-cell".to_string())
        },
        DataTableColumn {
            key: "title".to_string(),
            label: "Tender Title".to_string(),
            width: None,
            align: None,
            cell_class: None
        },
        DataTableColumn {
            key: "value".to_string(),
            label: "Value".to_string(),
            width: Some("120px".to_string()),
            align: Some("right".to_string()),
            cell_class: Some("amount-cell".to_string())
        },
        DataTableColumn {
            key: "bids".to_string(),
            label: "Bids".to_string(),
            width: Some("80px".to_string()),
            align: Some("center".to_string()),
            cell_class: None
        },
        DataTableColumn {
            key: "deadline".to_string(),
            label: "Eval Deadline".to_string(),
            width: Some("120px".to_string()),
            align: None,
            cell_class: None
        },
        DataTableColumn {
            key: "progress".to_string(),
            label: "Scoring Progress".to_string(),
            width: Some("150px".to_string()),
            align: None,
            cell_class: None
        },
        DataTableColumn {
            key: "status".to_string(),
            label: "Status".to_string(),
            width: Some("120px".to_string()),
            align: None,
            cell_class: None
        },
    ];

    // Transform evaluations to table rows
    let rows = move |_| -> Vec<DataTableRow> {
        filtered_evaluations().iter().map(|eval| {
            let status = match eval.status {
                EvaluationStatus::Pending => status_badge(StatusType::Pending),
                EvaluationStatus::InProgress => status_badge(StatusType::InProgress),
                EvaluationStatus::Completed => status_badge(StatusType::Complete),
                EvaluationStatus::Approved => status_badge(StatusType::Approved),
                EvaluationStatus::Rejected => status_badge(StatusType::Rejected),
            };

            // Calculate scoring progress
            let total_members = eval.committee_members.iter()
                .filter(|m| !m.conflict_declared)
                .count();
            let scored_members = eval.committee_members.iter()
                .filter(|m| m.has_scored && !m.conflict_declared)
                .count();
            let progress_pct = if total_members > 0 {
                (scored_members as f64 / total_members as f64) * 100.0
            } else {
                0.0
            };

            // Deadline urgency
            let deadline_class = if eval.status == EvaluationStatus::InProgress {
                "deadline-urgent"
            } else {
                ""
            };

            DataTableRow {
                id: eval.id.clone(),
                cells: vec![
                    view! { <span class="id-cell">{eval.tender_reference.clone()}</span> },
                    view! { <span class="tender-title">{eval.tender_title.clone()}</span> },
                    view! { <span class="amount-cell">{format_currency(eval.tender_value)}</span> },
                    view! {
                        <span class="bid-count">
                            {tag(eval.bids.len().to_string(), TagType::Info)}
                        </span>
                    },
                    view! { <span class={deadline_class}>{format_date(&eval.evaluation_deadline)}</span> },
                    view! {
                        <div class="progress-cell">
                            <div class="progress-bar-mini">
                                <div class="progress-fill" style={format!("width: {}%", progress_pct)}></div>
                            </div>
                            <span class="progress-text">{format!("{}/{}", scored_members, total_members)}</span>
                        </div>
                    },
                    status,
                ],
            }
        }).collect()
    };

    // Handle row click - navigate to evaluation scoring
    let handle_row_click = Callback::new({
        let store = store.clone();
        move |eval_id: String| {
            store.select_evaluation(&eval_id);
            // In production, would use router navigation
            // router.navigate(&format!("/evaluation/{}", eval_id));
        }
    });

    // Filter button handlers
    let set_filter_all = {
        let filter_status = filter_status.clone();
        Callback::<()>::new(move |_| filter_status.set(None))
    };
    let set_filter_pending = {
        let filter_status = filter_status.clone();
        Callback::<()>::new(move |_| filter_status.set(Some(EvaluationStatus::Pending)))
    };
    let set_filter_in_progress = {
        let filter_status = filter_status.clone();
        Callback::<()>::new(move |_| filter_status.set(Some(EvaluationStatus::InProgress)))
    };
    let set_filter_completed = {
        let filter_status = filter_status.clone();
        Callback::<()>::new(move |_| filter_status.set(Some(EvaluationStatus::Completed)))
    };

    // Icons
    let icon_clipboard = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/><rect x="8" y="2" width="8" height="4" rx="1" ry="1"/></svg>"#;
    let icon_clock = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>"#;
    let icon_check = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>"#;
    let icon_award = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="8" r="7"/><polyline points="8.21 13.89 7 23 12 20 17 23 15.79 13.88"/></svg>"#;

    view! {
        style {
            r#"
            .evaluation-list {
                display: flex;
                flex-direction: column;
                gap: var(--space-6);
            }
            .summary-cards {
                display: grid;
                grid-template-columns: repeat(4, 1fr);
                gap: 16px;
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
            .summary-icon.in-progress { background: var(--blue-light); color: var(--blue); }
            .summary-icon.completed { background: var(--green-light); color: var(--green); }
            .summary-icon.approved { background: var(--purple-light); color: var(--purple); }
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
            .filter-bar {
                display: flex;
                gap: 8px;
                margin-bottom: 16px;
            }
            .filter-btn {
                padding: 8px 16px;
                border: 1px solid var(--border);
                border-radius: var(--radius-md);
                background: var(--surface);
                color: var(--text);
                font-size: 13px;
                cursor: pointer;
                transition: all 0.15s;
            }
            .filter-btn:hover {
                border-color: var(--blue);
                color: var(--blue);
            }
            .filter-btn.active {
                background: var(--blue);
                border-color: var(--blue);
                color: white;
            }
            .progress-cell {
                display: flex;
                align-items: center;
                gap: 8px;
            }
            .progress-bar-mini {
                flex: 1;
                height: 6px;
                background: var(--border);
                border-radius: 3px;
                overflow: hidden;
            }
            .progress-fill {
                height: 100%;
                background: var(--green);
                border-radius: 3px;
                transition: width 0.3s;
            }
            .progress-text {
                font-size: 12px;
                color: var(--text-muted);
                font-family: IBM Plex Mono, monospace;
                min-width: 40px;
            }
            .deadline-urgent {
                color: var(--orange);
                font-weight: 500;
            }
            .tender-title {
                max-width: 300px;
                white-space: nowrap;
                overflow: hidden;
                text-overflow: ellipsis;
            }
            "#
        }

        <div class="evaluation-list" data-testid="evaluation-list">
            {page_header(
                "Bid Evaluation".to_string(),
                Some("Review and score bids for open tenders".to_string()),
                vec![
                    view! { <button class="btn btn-secondary">"Export Report"</button> },
                ]
            )}

            // Summary cards
            <div class="summary-cards">
                <div
                    class={move |_| if filter_status.get().is_none() { "summary-card active" } else { "summary-card" }}
                    on:click={set_filter_all.clone()}
                >
                    <div class="summary-icon pending" inner_html={icon_clipboard}></div>
                    <div class="summary-content">
                        <h3>{move |_| evaluations.get().len()}</h3>
                        <p>"Total Evaluations"</p>
                    </div>
                </div>
                <div
                    class={move |_| if matches!(filter_status.get().as_ref(), Some(EvaluationStatus::Pending)) { "summary-card active" } else { "summary-card" }}
                    on:click={set_filter_pending}
                >
                    <div class="summary-icon pending" inner_html={icon_clock}></div>
                    <div class="summary-content">
                        <h3>{pending_count}</h3>
                        <p>"Pending Start"</p>
                    </div>
                </div>
                <div
                    class={move |_| if matches!(filter_status.get().as_ref(), Some(EvaluationStatus::InProgress)) { "summary-card active" } else { "summary-card" }}
                    on:click={set_filter_in_progress}
                >
                    <div class="summary-icon in-progress" inner_html={icon_clipboard}></div>
                    <div class="summary-content">
                        <h3>{in_progress_count}</h3>
                        <p>"In Progress"</p>
                    </div>
                </div>
                <div
                    class={move |_| if matches!(filter_status.get().as_ref(), Some(EvaluationStatus::Completed)) { "summary-card active" } else { "summary-card" }}
                    on:click={set_filter_completed}
                >
                    <div class="summary-icon completed" inner_html={icon_check}></div>
                    <div class="summary-content">
                        <h3>{completed_count}</h3>
                        <p>"Completed"</p>
                    </div>
                </div>
            </div>

            // Evaluations table
            {panel(
                "Tenders Pending Evaluation".to_string(),
                vec![],
                vec![data_table(columns, rows(), Some(handle_row_click))]
            )}
        </div>
    }
}
