//! Evaluation scoring page - criteria grid, bid comparison, committee notes

use components::prelude::*;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, panel_with_footer,
    status_badge, StatusType,
    tag, TagType,
    bbbee_badge, BbbeeLevel,
    progress_bar, ProgressColor,
    timeline, TimelineItem, TimelineStatus,
    notice_bar, NoticeType,
};
use crate::util::format::{format_currency, format_currency_full, format_date, format_datetime};
use super::store::EvaluationStore;
use super::types::{EvaluationStatus, CriterionCategory, ScoreSubmission, CriterionScore};
use super::service;

/// Evaluation scoring page
#[component]
pub fn evaluation_scoring() -> View {
    let store = use_context::<EvaluationStore>();

    // Load data on mount if not already loaded
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                if store.evaluations.get().is_empty() {
                    service::load_evaluations(&store).await;
                }
                // Select first evaluation if none selected
                if store.selected.get().is_none() && !store.evaluations.get().is_empty() {
                    let first_id = store.evaluations.get()[0].id.clone();
                    store.select_evaluation(&first_id);
                }
            });
        }
    });

    let selected = store.selected.clone();
    let selected_bid = store.selected_bid.clone();
    let scoring_in_progress = store.scoring_in_progress.clone();
    let current_scores = store.current_scores.clone();
    let loading = store.loading.clone();

    // Handlers
    let handle_start_scoring = {
        let store = store.clone();
        Callback::<()>::new(move |_| {
            store.start_scoring();
        })
    };

    let handle_select_bid = {
        let store = store.clone();
        Callback::new(move |bid_id: String| {
            store.select_bid(&bid_id);
        })
    };

    let handle_submit_scores = {
        let store = store.clone();
        Callback::<()>::new(move |_| {
            let store = store.clone();
            spawn(async move {
                if let (Some(eval), Some(bid)) = (store.selected.get().as_ref(), store.selected_bid.get().as_ref()) {
                    let submission = ScoreSubmission {
                        evaluation_id: eval.id.clone(),
                        bid_id: bid.id.clone(),
                        scores: store.current_scores.get().clone(),
                        committee_member_id: "mem_003".to_string(), // Current user
                    };
                    let _ = service::submit_scores(&store, submission).await;
                }
            });
        })
    };

    let handle_back = {
        let store = store.clone();
        Callback::<()>::new(move |_| {
            store.clear_selection();
        })
    };

    // Icons
    let icon_back = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M19 12H5"/><polyline points="12 19 5 12 12 5"/></svg>"#;
    let icon_users = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>"#;
    let icon_check = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>"#;
    let icon_clock = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>"#;
    let icon_alert = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>"#;

    view! {
        style {
            r#"
            .evaluation-scoring {
                display: flex;
                flex-direction: column;
                gap: var(--space-6);
            }
            .back-link {
                display: inline-flex;
                align-items: center;
                gap: 8px;
                color: var(--text-muted);
                text-decoration: none;
                font-size: 13px;
                margin-bottom: 8px;
                cursor: pointer;
            }
            .back-link:hover {
                color: var(--blue);
            }
            .back-link svg {
                width: 16px;
                height: 16px;
            }
            .eval-header {
                display: flex;
                justify-content: space-between;
                align-items: flex-start;
                gap: 24px;
            }
            .eval-info {
                flex: 1;
            }
            .eval-info h1 {
                font-family: Playfair Display, serif;
                font-size: 24px;
                font-weight: 600;
                color: var(--navy);
                margin-bottom: 8px;
            }
            .eval-meta {
                display: flex;
                flex-wrap: wrap;
                gap: 16px;
                color: var(--text-muted);
                font-size: 13px;
            }
            .eval-meta-item {
                display: flex;
                align-items: center;
                gap: 6px;
            }
            .eval-meta-item svg {
                width: 14px;
                height: 14px;
            }
            .eval-actions {
                display: flex;
                gap: 12px;
            }
            .scoring-layout {
                display: grid;
                grid-template-columns: 300px 1fr 350px;
                gap: 24px;
            }
            @media (max-width: 1400px) {
                .scoring-layout {
                    grid-template-columns: 1fr;
                }
            }
            .bid-list {
                display: flex;
                flex-direction: column;
                gap: 8px;
            }
            .bid-card {
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius-md);
                padding: 16px;
                cursor: pointer;
                transition: all 0.15s;
            }
            .bid-card:hover {
                border-color: var(--blue);
            }
            .bid-card.selected {
                border-color: var(--blue);
                background: var(--blue-light);
            }
            .bid-card-header {
                display: flex;
                justify-content: space-between;
                align-items: flex-start;
                margin-bottom: 8px;
            }
            .bid-supplier {
                font-weight: 600;
                font-size: 14px;
                color: var(--navy);
            }
            .bid-rank {
                font-size: 11px;
                font-weight: 600;
                padding: 2px 8px;
                border-radius: 10px;
                background: var(--green-light);
                color: var(--green);
            }
            .bid-rank.rank-1 { background: var(--gold-light); color: var(--gold); }
            .bid-rank.rank-2 { background: var(--silver-light); color: var(--silver); }
            .bid-rank.rank-3 { background: var(--bronze-light); color: var(--bronze); }
            .bid-details {
                display: flex;
                flex-wrap: wrap;
                gap: 12px;
                font-size: 12px;
                color: var(--text-muted);
            }
            .bid-price {
                font-family: IBM Plex Mono, monospace;
                font-weight: 500;
                color: var(--navy);
            }
            .bid-score {
                font-weight: 600;
            }
            .bid-score.high { color: var(--green); }
            .bid-score.medium { color: var(--orange); }
            .bid-score.low { color: var(--red); }
            .criteria-grid {
                display: flex;
                flex-direction: column;
                gap: 16px;
            }
            .criterion-category {
                margin-bottom: 8px;
            }
            .criterion-category-header {
                font-size: 12px;
                font-weight: 600;
                text-transform: uppercase;
                letter-spacing: 0.5px;
                color: var(--text-muted);
                margin-bottom: 12px;
                padding-bottom: 8px;
                border-bottom: 1px solid var(--border);
            }
            .criterion-row {
                display: grid;
                grid-template-columns: 1fr 100px 120px;
                gap: 16px;
                align-items: center;
                padding: 12px 0;
                border-bottom: 1px solid var(--border-light);
            }
            .criterion-row:last-child {
                border-bottom: none;
            }
            .criterion-info h4 {
                font-size: 14px;
                font-weight: 500;
                color: var(--navy);
                margin-bottom: 4px;
            }
            .criterion-info p {
                font-size: 12px;
                color: var(--text-muted);
            }
            .criterion-weight {
                font-size: 12px;
                color: var(--text-muted);
                text-align: center;
            }
            .criterion-score {
                display: flex;
                gap: 4px;
            }
            .score-btn {
                width: 32px;
                height: 32px;
                border: 1px solid var(--border);
                border-radius: var(--radius-sm);
                background: var(--surface);
                color: var(--text-muted);
                font-size: 12px;
                font-weight: 500;
                cursor: pointer;
                transition: all 0.15s;
            }
            .score-btn:hover {
                border-color: var(--blue);
                color: var(--blue);
            }
            .score-btn.selected {
                background: var(--blue);
                border-color: var(--blue);
                color: white;
            }
            .score-btn:disabled {
                opacity: 0.5;
                cursor: not-allowed;
            }
            .sidebar-section {
                margin-bottom: 24px;
            }
            .sidebar-section-title {
                font-size: 12px;
                font-weight: 600;
                text-transform: uppercase;
                letter-spacing: 0.5px;
                color: var(--text-muted);
                margin-bottom: 12px;
            }
            .committee-list {
                display: flex;
                flex-direction: column;
                gap: 8px;
            }
            .committee-member {
                display: flex;
                align-items: center;
                gap: 12px;
                padding: 8px 0;
            }
            .member-avatar {
                width: 32px;
                height: 32px;
                border-radius: 50%;
                background: var(--blue-light);
                color: var(--blue);
                display: flex;
                align-items: center;
                justify-content: center;
                font-size: 12px;
                font-weight: 600;
            }
            .member-info {
                flex: 1;
            }
            .member-name {
                font-size: 13px;
                font-weight: 500;
                color: var(--navy);
            }
            .member-role {
                font-size: 11px;
                color: var(--text-muted);
            }
            .member-status {
                width: 20px;
                height: 20px;
                border-radius: 50%;
                display: flex;
                align-items: center;
                justify-content: center;
            }
            .member-status.scored {
                background: var(--green-light);
                color: var(--green);
            }
            .member-status.pending {
                background: var(--orange-light);
                color: var(--orange);
            }
            .member-status svg {
                width: 12px;
                height: 12px;
            }
            .notes-list {
                display: flex;
                flex-direction: column;
                gap: 12px;
            }
            .note-card {
                background: var(--bg);
                border-radius: var(--radius-md);
                padding: 12px;
            }
            .note-header {
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin-bottom: 8px;
            }
            .note-author {
                font-size: 12px;
                font-weight: 500;
                color: var(--navy);
            }
            .note-time {
                font-size: 11px;
                color: var(--text-muted);
            }
            .note-content {
                font-size: 13px;
                color: var(--text);
                line-height: 1.5;
            }
            .score-summary {
                background: var(--bg);
                border-radius: var(--radius-md);
                padding: 16px;
                margin-top: 16px;
            }
            .score-summary-row {
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding: 8px 0;
                border-bottom: 1px solid var(--border-light);
            }
            .score-summary-row:last-child {
                border-bottom: none;
                font-weight: 600;
            }
            .score-summary-label {
                font-size: 13px;
                color: var(--text-muted);
            }
            .score-summary-value {
                font-size: 14px;
                font-weight: 500;
                color: var(--navy);
                font-family: IBM Plex Mono, monospace;
            }
            .comparison-table {
                width: 100%;
                border-collapse: collapse;
            }
            .comparison-table th,
            .comparison-table td {
                padding: 12px;
                text-align: left;
                border-bottom: 1px solid var(--border-light);
            }
            .comparison-table th {
                background: var(--bg);
                font-size: 12px;
                font-weight: 600;
                text-transform: uppercase;
                letter-spacing: 0.5px;
                color: var(--text-muted);
            }
            .comparison-table td {
                font-size: 13px;
            }
            .comparison-table .highlight {
                background: var(--green-light);
            }
            .no-selection {
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                padding: 60px 20px;
                text-align: center;
                color: var(--text-muted);
            }
            .no-selection svg {
                width: 48px;
                height: 48px;
                margin-bottom: 16px;
                opacity: 0.5;
            }
            .no-selection h3 {
                font-size: 16px;
                font-weight: 500;
                margin-bottom: 8px;
                color: var(--navy);
            }
            .no-selection p {
                font-size: 13px;
            }
            "#
        }

        <div class="evaluation-scoring" data-testid="evaluation-scoring">
            // Back link
            <a class="back-link" on:click={handle_back}>
                <span inner_html={icon_back}></span>
                "Back to Evaluation List"
            </a>

            if let Some(eval) = selected.get().as_ref() {
                // Header
                <div class="eval-header">
                    <div class="eval-info">
                        <h1>{eval.tender_title.clone()}</h1>
                        <div class="eval-meta">
                            <span class="eval-meta-item">
                                <strong>"Ref:"</strong> {eval.tender_reference.clone()}
                            </span>
                            <span class="eval-meta-item">
                                <strong>"Value:"</strong> {format_currency(eval.tender_value)}
                            </span>
                            <span class="eval-meta-item">
                                <span inner_html={icon_clock}></span>
                                "Deadline: " {format_date(&eval.evaluation_deadline)}
                            </span>
                            <span class="eval-meta-item">
                                {match eval.status {
                                    EvaluationStatus::Pending => status_badge(StatusType::Pending),
                                    EvaluationStatus::InProgress => status_badge(StatusType::InProgress),
                                    EvaluationStatus::Completed => status_badge(StatusType::Complete),
                                    EvaluationStatus::Approved => status_badge(StatusType::Approved),
                                    EvaluationStatus::Rejected => status_badge(StatusType::Rejected),
                                }}
                            </span>
                        </div>
                    </div>
                    <div class="eval-actions">
                        if !scoring_in_progress.get() {
                            <button class="btn btn-primary" on:click={handle_start_scoring.clone()}>
                                "Start Scoring"
                            </button>
                        }
                    </div>
                </div>

                // Notice bar for scoring info
                {notice_bar(
                    NoticeType::Info,
                    format!("Scoring Method: {} | Min Technical Score: {}% | Price Weight: {}%",
                        eval.scoring_method.as_str(),
                        eval.min_technical_score,
                        eval.price_weight
                    ),
                    None
                )}

                // Main scoring layout
                <div class="scoring-layout">
                    // Left sidebar - Bid list
                    {panel(
                        format!("Bids ({})", eval.bids.len()),
                        vec![],
                        vec![view! {
                            <div class="bid-list">
                                for bid in eval.bids.iter() {
                                    {bid_card(
                                        bid.clone(),
                                        selected_bid.get().as_ref().map(|b| b.id.clone()),
                                        handle_select_bid.clone()
                                    )}
                                }
                            </div>
                        }]
                    )}

                    // Center - Criteria scoring grid
                    if let Some(bid) = selected_bid.get().as_ref() {
                        {panel_with_footer(
                            format!("Score: {}", bid.supplier_name),
                            vec![
                                bbbee_badge(match bid.bbbee_level {
                                    1 => BbbeeLevel::Level1,
                                    2 => BbbeeLevel::Level2,
                                    3 => BbbeeLevel::Level3,
                                    4 => BbbeeLevel::Level4,
                                    _ => BbbeeLevel::NonCompliant,
                                }),
                            ],
                            vec![
                                criteria_scoring_grid(
                                    eval.criteria.clone(),
                                    current_scores.get().clone(),
                                    scoring_in_progress.get(),
                                    store.clone()
                                ),
                                view! {
                                    <div class="score-summary">
                                        <div class="score-summary-row">
                                            <span class="score-summary-label">"Bid Price"</span>
                                            <span class="score-summary-value">{format_currency_full(bid.total_price)}</span>
                                        </div>
                                        <div class="score-summary-row">
                                            <span class="score-summary-label">"Technical Score"</span>
                                            <span class="score-summary-value">
                                                {bid.technical_score.map(|s| format!("{:.1}%", s)).unwrap_or("-".to_string())}
                                            </span>
                                        </div>
                                        <div class="score-summary-row">
                                            <span class="score-summary-label">"Financial Score"</span>
                                            <span class="score-summary-value">
                                                {bid.financial_score.map(|s| format!("{:.1}%", s)).unwrap_or("-".to_string())}
                                            </span>
                                        </div>
                                        <div class="score-summary-row">
                                            <span class="score-summary-label">"Total Score"</span>
                                            <span class="score-summary-value">
                                                {bid.total_score.map(|s| format!("{:.1}%", s)).unwrap_or("-".to_string())}
                                            </span>
                                        </div>
                                    </div>
                                },
                            ],
                            vec![
                                view! { <button class="btn btn-secondary">"Save Draft"</button> },
                                view! {
                                    <button
                                        class="btn btn-primary"
                                        on:click={handle_submit_scores.clone()}
                                        disabled={!scoring_in_progress.get() || loading.get()}
                                    >
                                        "Submit Scores"
                                    </button>
                                },
                            ]
                        )}
                    } else {
                        {panel(
                            "Scoring".to_string(),
                            vec![],
                            vec![view! {
                                <div class="no-selection">
                                    <span inner_html={icon_alert}></span>
                                    <h3>"Select a Bid to Score"</h3>
                                    <p>"Choose a bid from the list on the left to begin scoring."</p>
                                </div>
                            }]
                        )}
                    }

                    // Right sidebar - Committee & Notes
                    <div class="sidebar">
                        // Committee members
                        {panel(
                            "Committee".to_string(),
                            vec![],
                            vec![view! {
                                <div class="committee-list">
                                    for member in eval.committee_members.iter() {
                                        <div class="committee-member">
                                            <div class="member-avatar">
                                                {member.name.chars().next().unwrap_or('?')}
                                            </div>
                                            <div class="member-info">
                                                <div class="member-name">{member.name.clone()}</div>
                                                <div class="member-role">{member.role.clone()}</div>
                                            </div>
                                            <div class={if member.has_scored { "member-status scored" } else { "member-status pending" }}>
                                                if member.has_scored {
                                                    <span inner_html={icon_check}></span>
                                                } else {
                                                    <span inner_html={icon_clock}></span>
                                                }
                                            </div>
                                        </div>
                                    }
                                </div>
                            }]
                        )}

                        // Committee notes
                        {panel(
                            "Committee Notes".to_string(),
                            vec![view! { <button class="btn btn-sm btn-secondary">"Add Note"</button> }],
                            vec![view! {
                                <div class="notes-list">
                                    for note in eval.notes.iter() {
                                        <div class="note-card">
                                            <div class="note-header">
                                                <span class="note-author">{note.author_name.clone()}</span>
                                                <span class="note-time">{format_datetime(&note.created_at)}</span>
                                            </div>
                                            <div class="note-content">{note.content.clone()}</div>
                                        </div>
                                    }
                                    if eval.notes.is_empty() {
                                        <p style="color: var(--text-muted); font-size: 13px; text-align: center; padding: 20px;">
                                            "No committee notes yet."
                                        </p>
                                    }
                                </div>
                            }]
                        )}
                    </div>
                </div>

                // Bid comparison table
                {panel(
                    "Bid Comparison".to_string(),
                    vec![view! { <button class="btn btn-sm btn-secondary">"Export Comparison"</button> }],
                    vec![bid_comparison_table(eval.bids.clone(), eval.criteria.clone())]
                )}
            } else {
                // No evaluation selected
                <div class="no-selection">
                    <span inner_html={icon_alert}></span>
                    <h3>"No Evaluation Selected"</h3>
                    <p>"Please select an evaluation from the list to begin scoring."</p>
                </div>
            }
        </div>
    }
}

/// Bid card component
fn bid_card(
    bid: super::types::Bid,
    selected_id: Option<String>,
    on_click: Callback<String>,
) -> View {
    let is_selected = selected_id.as_ref().map(|id| id == &bid.id).unwrap_or(false);
    let bid_id = bid.id.clone();

    let handle_click: Callback<()> = Callback::new({
        let on_click = on_click.clone();
        let bid_id = bid_id.clone();
        move |_| {
            on_click.call(bid_id.clone());
        }
    });

    let score_class = match bid.total_score {
        Some(s) if s >= 80.0 => "bid-score high",
        Some(s) if s >= 60.0 => "bid-score medium",
        Some(_) => "bid-score low",
        None => "bid-score",
    };

    let rank_class = match bid.rank {
        Some(1) => "bid-rank rank-1",
        Some(2) => "bid-rank rank-2",
        Some(3) => "bid-rank rank-3",
        Some(_) => "bid-rank",
        None => "bid-rank",
    };

    view! {
        <div
            class={if is_selected { "bid-card selected" } else { "bid-card" }}
            on:click={handle_click}
        >
            <div class="bid-card-header">
                <span class="bid-supplier">{bid.supplier_name}</span>
                if let Some(rank) = bid.rank {
                    <span class={rank_class}>{format!("#{}", rank)}</span>
                }
            </div>
            <div class="bid-details">
                <span class="bid-price">{format_currency(bid.total_price)}</span>
                <span>{format!("B-BBEE L{}", bid.bbbee_level)}</span>
                if let Some(score) = bid.total_score {
                    <span class={score_class}>{format!("{:.1}%", score)}</span>
                }
            </div>
        </div>
    }
}

/// Criteria scoring grid component
fn criteria_scoring_grid(
    criteria: Vec<super::types::EvaluationCriterion>,
    current_scores: Vec<CriterionScore>,
    scoring_enabled: bool,
    store: EvaluationStore,
) -> View {
    // Group criteria by category
    let technical: Vec<_> = criteria.iter()
        .filter(|c| c.category == CriterionCategory::Technical)
        .cloned()
        .collect();
    let experience: Vec<_> = criteria.iter()
        .filter(|c| c.category == CriterionCategory::Experience)
        .cloned()
        .collect();
    let bbbee: Vec<_> = criteria.iter()
        .filter(|c| c.category == CriterionCategory::Bbbee)
        .cloned()
        .collect();
    let local_content: Vec<_> = criteria.iter()
        .filter(|c| c.category == CriterionCategory::LocalContent)
        .cloned()
        .collect();
    let financial: Vec<_> = criteria.iter()
        .filter(|c| c.category == CriterionCategory::Financial)
        .cloned()
        .collect();

    view! {
        <div class="criteria-grid">
            if !technical.is_empty() {
                {criterion_category("Technical", technical, current_scores.clone(), scoring_enabled, store.clone())}
            }
            if !experience.is_empty() {
                {criterion_category("Experience", experience, current_scores.clone(), scoring_enabled, store.clone())}
            }
            if !bbbee.is_empty() {
                {criterion_category("B-BBEE", bbbee, current_scores.clone(), scoring_enabled, store.clone())}
            }
            if !local_content.is_empty() {
                {criterion_category("Local Content", local_content, current_scores.clone(), scoring_enabled, store.clone())}
            }
            if !financial.is_empty() {
                {criterion_category("Financial", financial, current_scores.clone(), scoring_enabled, store.clone())}
            }
        </div>
    }
}

/// Criterion category section
fn criterion_category(
    category_name: &str,
    criteria: Vec<super::types::EvaluationCriterion>,
    current_scores: Vec<CriterionScore>,
    scoring_enabled: bool,
    store: EvaluationStore,
) -> View {
    view! {
        <div class="criterion-category">
            <div class="criterion-category-header">{category_name}</div>
            for criterion in criteria.iter() {
                {criterion_row(criterion.clone(), current_scores.clone(), scoring_enabled, store.clone())}
            }
        </div>
    }
}

/// Single criterion row with score buttons
fn criterion_row(
    criterion: super::types::EvaluationCriterion,
    current_scores: Vec<CriterionScore>,
    scoring_enabled: bool,
    store: EvaluationStore,
) -> View {
    let current_score = current_scores.iter()
        .find(|s| s.criterion_id == criterion.id)
        .map(|s| s.score);

    view! {
        <div class="criterion-row">
            <div class="criterion-info">
                <h4>{criterion.name.clone()}</h4>
                <p>{criterion.description.clone()}</p>
            </div>
            <div class="criterion-weight">
                {format!("Weight: {}%", criterion.weight)}
            </div>
            <div class="criterion-score">
                for score in 0..=criterion.max_score {
                    {score_button(
                        criterion.id.clone(),
                        score,
                        current_score,
                        scoring_enabled,
                        store.clone()
                    )}
                }
            </div>
        </div>
    }
}

/// Score button component
fn score_button(
    criterion_id: String,
    score: u32,
    current_score: Option<u32>,
    enabled: bool,
    store: EvaluationStore,
) -> View {
    let is_selected = current_score == Some(score);
    let class = if is_selected { "score-btn selected" } else { "score-btn" };

    let handle_click = Callback::<()>::new({
        let criterion_id = criterion_id.clone();
        let store = store.clone();
        move |_| {
            store.update_score(&criterion_id, score, None);
        }
    });

    view! {
        <button
            class={class}
            on:click={handle_click}
            disabled={!enabled}
        >
            {score.to_string()}
        </button>
    }
}

/// Bid comparison table
fn bid_comparison_table(
    bids: Vec<super::types::Bid>,
    criteria: Vec<super::types::EvaluationCriterion>,
) -> View {
    // Sort bids by rank
    let mut sorted_bids = bids.clone();
    sorted_bids.sort_by(|a, b| {
        a.rank.unwrap_or(999).cmp(&b.rank.unwrap_or(999))
    });

    // Find lowest price for highlighting
    let lowest_price = sorted_bids.iter()
        .map(|b| b.total_price)
        .fold(f64::INFINITY, f64::min);

    // Find highest score for highlighting
    let highest_score = sorted_bids.iter()
        .filter_map(|b| b.total_score)
        .fold(0.0_f64, f64::max);

    view! {
        <table class="comparison-table">
            <thead>
                <tr>
                    <th>"Supplier"</th>
                    <th>"B-BBEE"</th>
                    <th>"Price"</th>
                    <th>"Technical"</th>
                    <th>"Financial"</th>
                    <th>"Total"</th>
                    <th>"Rank"</th>
                </tr>
            </thead>
            <tbody>
                for bid in sorted_bids.iter() {
                    <tr>
                        <td>
                            <strong>{bid.supplier_name.clone()}</strong>
                        </td>
                        <td>{format!("Level {}", bid.bbbee_level)}</td>
                        <td class={if bid.total_price == lowest_price { "highlight" } else { "" }}>
                            {format_currency(bid.total_price)}
                        </td>
                        <td>{bid.technical_score.map(|s| format!("{:.1}%", s)).unwrap_or("-".to_string())}</td>
                        <td>{bid.financial_score.map(|s| format!("{:.1}%", s)).unwrap_or("-".to_string())}</td>
                        <td class={if bid.total_score == Some(highest_score) { "highlight" } else { "" }}>
                            <strong>{bid.total_score.map(|s| format!("{:.1}%", s)).unwrap_or("-".to_string())}</strong>
                        </td>
                        <td>
                            {bid.rank.map(|r| format!("#{}", r)).unwrap_or("-".to_string())}
                        </td>
                    </tr>
                }
            </tbody>
        </table>
    }
}
