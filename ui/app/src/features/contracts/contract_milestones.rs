//! Contract milestones tracking page

use components::prelude::*;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, panel_with_footer,
    status_badge, StatusType,
    progress_bar, ProgressColor,
    timeline, TimelineItem, TimelineStatus,
    notice_bar, NoticeType,
    data_table, DataTableColumn, DataTableRow,
};
use crate::util::format::{format_currency, format_currency_full, format_date};
use super::types::{Contract, ContractMilestone, MilestoneStatus};
use super::store::ContractsStore;
use super::service;

/// Contract milestones tracking page
#[component]
pub fn contract_milestones(contract_id: String) -> View {
    let store = use_context::<ContractsStore>();

    // Load contract on mount
    effect({
        let store = store.clone();
        let contract_id = contract_id.clone();
        move || {
            let store = store.clone();
            let contract_id = contract_id.clone();
            spawn(async move {
                service::load_contract(&store, &contract_id).await;
            });
        }
    });

    let loading = store.loading.get();
    let contract = store.selected.get();

    view! {
        style {
            r#"
            .milestones-page { display: flex; flex-direction: column; gap: var(--space-6); }

            .milestone-summary {
                display: grid;
                grid-template-columns: repeat(4, 1fr);
                gap: 16px;
                margin-bottom: 24px;
            }
            .summary-card {
                padding: 20px;
                background: var(--surface);
                border-radius: var(--radius);
                border: 1px solid var(--border);
            }
            .summary-card .label {
                font-size: 12px;
                color: var(--text-muted);
                margin-bottom: 8px;
            }
            .summary-card .value {
                font-size: 24px;
                font-weight: 600;
                color: var(--navy);
            }
            .summary-card .value.currency {
                font-family: IBM Plex Mono, monospace;
            }
            .summary-card .sub {
                font-size: 12px;
                color: var(--text-muted);
                margin-top: 4px;
            }

            .milestones-grid {
                display: grid;
                grid-template-columns: 2fr 1fr;
                gap: 24px;
            }

            .milestone-timeline {
                display: flex;
                flex-direction: column;
                gap: 0;
            }

            .milestone-card {
                position: relative;
                padding: 20px;
                padding-left: 40px;
                border-left: 3px solid var(--border);
                margin-left: 12px;
            }
            .milestone-card:last-child { border-left-color: transparent; }

            .milestone-card::before {
                content: "";
                position: absolute;
                left: -9px;
                top: 24px;
                width: 16px;
                height: 16px;
                border-radius: 50%;
                background: var(--surface);
                border: 3px solid var(--border);
            }
            .milestone-card.completed::before { border-color: var(--green); background: var(--green); }
            .milestone-card.in-progress::before { border-color: var(--blue); background: var(--blue); }
            .milestone-card.overdue::before { border-color: var(--red); background: var(--red); }
            .milestone-card.pending::before { border-color: var(--border); }

            .milestone-card-inner {
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius);
                padding: 20px;
            }
            .milestone-card.completed .milestone-card-inner { border-left: 3px solid var(--green); }
            .milestone-card.in-progress .milestone-card-inner { border-left: 3px solid var(--blue); }
            .milestone-card.overdue .milestone-card-inner { border-left: 3px solid var(--red); }

            .milestone-header {
                display: flex;
                justify-content: space-between;
                align-items: flex-start;
                margin-bottom: 12px;
            }
            .milestone-title {
                font-size: 15px;
                font-weight: 600;
                color: var(--navy);
            }
            .milestone-date {
                font-size: 12px;
                color: var(--text-muted);
                margin-top: 4px;
            }
            .milestone-description {
                font-size: 13px;
                color: var(--text-muted);
                margin-bottom: 16px;
            }
            .milestone-payment {
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding: 12px;
                background: var(--bg);
                border-radius: var(--radius-sm);
            }
            .milestone-payment .amount {
                font-family: IBM Plex Mono, monospace;
                font-size: 14px;
                font-weight: 600;
            }
            .milestone-payment .percentage {
                font-size: 12px;
                color: var(--text-muted);
            }

            .milestone-deliverables {
                margin-top: 16px;
                padding-top: 16px;
                border-top: 1px solid var(--border);
            }
            .milestone-deliverables h5 {
                font-size: 12px;
                font-weight: 600;
                color: var(--text-muted);
                margin-bottom: 8px;
            }
            .deliverable-item {
                display: flex;
                align-items: center;
                gap: 8px;
                font-size: 13px;
                padding: 4px 0;
            }
            .deliverable-item svg { width: 14px; height: 14px; color: var(--green); }
            .deliverable-item.pending svg { color: var(--text-muted); }

            .milestone-notes {
                margin-top: 12px;
                padding: 12px;
                background: var(--blue-light);
                border-radius: var(--radius-sm);
                font-size: 12px;
                color: var(--blue);
            }

            .payment-schedule {
                display: flex;
                flex-direction: column;
                gap: 0;
            }
            .payment-item {
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding: 16px 20px;
                border-bottom: 1px solid var(--border);
            }
            .payment-item:last-child { border-bottom: none; }
            .payment-item.completed { background: var(--green-light); }
            .payment-item.pending { background: transparent; }
            .payment-info { display: flex; flex-direction: column; gap: 4px; }
            .payment-info .name { font-weight: 500; font-size: 13px; }
            .payment-info .date { font-size: 12px; color: var(--text-muted); }
            .payment-amount {
                font-family: IBM Plex Mono, monospace;
                font-size: 14px;
                font-weight: 600;
            }
            .payment-amount.paid { color: var(--green); }

            .progress-overview {
                padding: 20px;
                background: var(--bg);
                border-radius: var(--radius);
                margin-bottom: 16px;
            }
            .progress-stats {
                display: grid;
                grid-template-columns: repeat(2, 1fr);
                gap: 16px;
                margin-top: 16px;
            }
            .progress-stat {
                display: flex;
                flex-direction: column;
                gap: 4px;
            }
            .progress-stat .label { font-size: 11px; color: var(--text-muted); }
            .progress-stat .value { font-size: 14px; font-weight: 600; }

            @media (max-width: 1024px) {
                .milestones-grid { grid-template-columns: 1fr; }
                .milestone-summary { grid-template-columns: repeat(2, 1fr); }
            }
            "#
        }

        <div class="milestones-page" data-testid="contract-milestones">
            if loading {
                <div class="loading-state">"Loading contract..."</div>
            } else if let Some(contract) = contract {
                {render_milestones_content(contract)}
            } else {
                {notice_bar(
                    "Contract not found".to_string(),
                    NoticeType::Error,
                    None
                )}
            }
        </div>
    }
}

fn render_milestones_content(contract: Contract) -> View {
    let total_milestones = contract.milestones.len();
    let completed_milestones = contract.milestones.iter()
        .filter(|m| m.status == MilestoneStatus::Completed)
        .count();
    let overdue_milestones = contract.milestones.iter()
        .filter(|m| m.status == MilestoneStatus::Overdue)
        .count();

    let total_payments = contract.total_milestone_payments();
    let completed_payments = contract.completed_milestone_payments();
    let progress = contract.milestone_progress();

    let payment_progress = if total_payments > 0.0 {
        (completed_payments / total_payments) * 100.0
    } else {
        0.0
    };

    view! {
        {page_header(
            format!("Milestones: {}", contract.title),
            Some(format!("Contract {} - {}", contract.id, contract.supplier_name)),
            vec![
                view! { <a href={format!("/contracts/{}", contract.id)} class="btn btn-secondary">"View Contract"</a> },
                view! { <button class="btn btn-primary">"Add Milestone"</button> },
            ]
        )}

        // Overdue warning
        if overdue_milestones > 0 {
            {notice_bar(
                format!("{} milestone(s) are overdue. Please review and take action.", overdue_milestones),
                NoticeType::Warning,
                None
            )}
        }

        // Summary cards
        <div class="milestone-summary">
            <div class="summary-card">
                <div class="label">"Total Milestones"</div>
                <div class="value">{total_milestones.to_string()}</div>
                <div class="sub">{format!("{} completed", completed_milestones)}</div>
            </div>
            <div class="summary-card">
                <div class="label">"Completion Progress"</div>
                <div class="value">{format!("{:.0}%", progress)}</div>
                <div class="sub">{format!("{} of {} milestones", completed_milestones, total_milestones)}</div>
            </div>
            <div class="summary-card">
                <div class="label">"Total Contract Value"</div>
                <div class="value currency">{format_currency(contract.value)}</div>
                <div class="sub">"Including all milestones"</div>
            </div>
            <div class="summary-card">
                <div class="label">"Payments Made"</div>
                <div class="value currency">{format_currency(completed_payments)}</div>
                <div class="sub">{format!("{:.0}% of total", payment_progress)}</div>
            </div>
        </div>

        // Main content grid
        <div class="milestones-grid">
            // Timeline
            {panel(
                "Milestone Timeline".to_string(),
                vec![],
                vec![
                    view! {
                        <div class="milestone-timeline">
                            for milestone in contract.milestones.iter() {
                                {render_milestone_card(milestone.clone())}
                            }
                        </div>
                    }
                ]
            )}

            // Payment schedule sidebar
            <div>
                // Progress overview
                <div class="progress-overview">
                    <div style="font-size: 13px; font-weight: 600; margin-bottom: 12px;">"Overall Progress"</div>
                    {progress_bar(progress, ProgressColor::Blue, true, None)}
                    <div class="progress-stats">
                        <div class="progress-stat">
                            <span class="label">"Paid"</span>
                            <span class="value">{format_currency(completed_payments)}</span>
                        </div>
                        <div class="progress-stat">
                            <span class="label">"Remaining"</span>
                            <span class="value">{format_currency(total_payments - completed_payments)}</span>
                        </div>
                    </div>
                </div>

                {panel(
                    "Payment Schedule".to_string(),
                    vec![],
                    vec![
                        view! {
                            <div class="payment-schedule">
                                for milestone in contract.milestones.iter() {
                                    {render_payment_item(milestone.clone())}
                                }
                            </div>
                        }
                    ]
                )}
            </div>
        </div>
    }
}

fn render_milestone_card(milestone: ContractMilestone) -> View {
    let status_class = match milestone.status {
        MilestoneStatus::Completed => "completed",
        MilestoneStatus::InProgress => "in-progress",
        MilestoneStatus::Overdue => "overdue",
        MilestoneStatus::Pending => "pending",
        MilestoneStatus::Cancelled => "cancelled",
    };

    let status_badge_type = match milestone.status {
        MilestoneStatus::Completed => StatusType::Complete,
        MilestoneStatus::InProgress => StatusType::InProgress,
        MilestoneStatus::Overdue => StatusType::Failed,
        MilestoneStatus::Pending => StatusType::Pending,
        MilestoneStatus::Cancelled => StatusType::Cancelled,
    };

    let date_display = if let Some(completed) = &milestone.completed_date {
        format!("Completed: {}", format_date(completed))
    } else {
        format!("Due: {}", format_date(&milestone.due_date))
    };

    view! {
        <div class={format!("milestone-card {}", status_class)}>
            <div class="milestone-card-inner">
                <div class="milestone-header">
                    <div>
                        <div class="milestone-title">{milestone.title.clone()}</div>
                        <div class="milestone-date">{date_display}</div>
                    </div>
                    {status_badge(status_badge_type)}
                </div>

                <div class="milestone-description">{milestone.description.clone()}</div>

                <div class="milestone-payment">
                    <div>
                        <span class="amount">{format_currency_full(milestone.payment_amount)}</span>
                    </div>
                    <span class="percentage">{format!("{:.0}% of contract", milestone.payment_percentage)}</span>
                </div>

                if !milestone.deliverables.is_empty() {
                    <div class="milestone-deliverables">
                        <h5>"Deliverables"</h5>
                        for deliverable in milestone.deliverables.iter() {
                            <div class={if milestone.status == MilestoneStatus::Completed { "deliverable-item" } else { "deliverable-item pending" }}>
                                if milestone.status == MilestoneStatus::Completed {
                                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <polyline points="20 6 9 17 4 12"/>
                                    </svg>
                                } else {
                                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <circle cx="12" cy="12" r="10"/>
                                    </svg>
                                }
                                <span>{deliverable.clone()}</span>
                            </div>
                        }
                    </div>
                }

                if let Some(notes) = &milestone.notes {
                    <div class="milestone-notes">
                        <strong>"Note: "</strong>{notes.clone()}
                    </div>
                }
            </div>
        </div>
    }
}

fn render_payment_item(milestone: ContractMilestone) -> View {
    let is_completed = milestone.status == MilestoneStatus::Completed;
    let class = if is_completed { "payment-item completed" } else { "payment-item pending" };

    let date_str = if let Some(completed) = &milestone.completed_date {
        format!("Paid: {}", format_date(completed))
    } else {
        format!("Due: {}", format_date(&milestone.due_date))
    };

    view! {
        <div class={class}>
            <div class="payment-info">
                <span class="name">{milestone.title.clone()}</span>
                <span class="date">{date_str}</span>
            </div>
            <span class={if is_completed { "payment-amount paid" } else { "payment-amount" }}>
                {format_currency_full(milestone.payment_amount)}
            </span>
        </div>
    }
}

/// Milestone detail modal/page component
#[component]
pub fn milestone_detail(contract_id: String, milestone_id: String) -> View {
    let store = use_context::<ContractsStore>();

    // Load contract on mount
    effect({
        let store = store.clone();
        let contract_id = contract_id.clone();
        move || {
            let store = store.clone();
            let contract_id = contract_id.clone();
            spawn(async move {
                service::load_contract(&store, &contract_id).await;
            });
        }
    });

    let loading = store.loading.get();
    let contract = store.selected.get();

    let milestone = contract.as_ref().and_then(|c| {
        c.milestones.iter().find(|m| m.id == milestone_id).cloned()
    });

    view! {
        <div class="milestone-detail" data-testid="milestone-detail">
            if loading {
                <div class="loading-state">"Loading..."</div>
            } else if let Some(milestone) = milestone {
                {page_header(
                    milestone.title.clone(),
                    Some(format!("Milestone {} - Due: {}", milestone.id, format_date(&milestone.due_date))),
                    vec![
                        view! { <a href={format!("/contracts/{}/milestones", contract_id)} class="btn btn-secondary">"Back to Milestones"</a> },
                        if milestone.status == MilestoneStatus::InProgress {
                            view! { <button class="btn btn-primary">"Mark Complete"</button> }
                        } else {
                            view! { <span></span> }
                        },
                    ]
                )}

                {panel(
                    "Milestone Details".to_string(),
                    vec![],
                    vec![
                        view! {
                            <div class="form-grid">
                                <div class="detail-row">
                                    <span class="label">"Description"</span>
                                    <span class="value">{milestone.description.clone()}</span>
                                </div>
                                <div class="detail-row">
                                    <span class="label">"Payment Amount"</span>
                                    <span class="value">{format_currency_full(milestone.payment_amount)}</span>
                                </div>
                                <div class="detail-row">
                                    <span class="label">"Percentage"</span>
                                    <span class="value">{format!("{:.0}%", milestone.payment_percentage)}</span>
                                </div>
                                <div class="detail-row">
                                    <span class="label">"Status"</span>
                                    <span class="value">{milestone.status.as_str()}</span>
                                </div>
                            </div>
                        }
                    ]
                )}
            } else {
                {notice_bar(
                    "Milestone not found".to_string(),
                    NoticeType::Error,
                    None
                )}
            }
        </div>
    }
}
