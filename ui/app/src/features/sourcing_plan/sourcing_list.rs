//! Sourcing Plan list view

use components::prelude::*;
use wasm_bindgen::JsCast;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, data_table, DataTableColumn, DataTableRow,
    status_badge, StatusType,
    pagination,
    progress_bar,
};
use crate::shared::forms::filter_bar;
use crate::util::format::{format_currency, format_date};
use super::store::{SourcingPlanStore, get_paginated_plans, get_filtered_plans};
use super::types::{SourcingPlanStatus, SourcingPlanFilter};
use super::service;

/// Sourcing plan list page
#[component]
pub fn sourcing_list() -> View {
    let store = use_context::<SourcingPlanStore>();

    // Load data on mount
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                service::load_sourcing_plans(&store).await;
            });
        }
    });

    // Filter signals
    let search = signal(String::new());
    let status_filter = signal(String::new());
    let fiscal_year_filter = signal(String::new());
    let department_filter = signal(String::new());

    // Update store filter when inputs change
    let update_filter = {
        let store = store.clone();
        let search = search.clone();
        let status_filter = status_filter.clone();
        let fiscal_year_filter = fiscal_year_filter.clone();
        let department_filter = department_filter.clone();
        Callback::new(move |_: ()| {
            let mut filter = SourcingPlanFilter::default();
            filter.search = search.get();
            filter.status = if status_filter.get().is_empty() {
                None
            } else {
                Some(SourcingPlanStatus::from_str(&status_filter.get()))
            };
            filter.fiscal_year = if fiscal_year_filter.get().is_empty() {
                None
            } else {
                Some(fiscal_year_filter.get())
            };
            filter.department = if department_filter.get().is_empty() {
                None
            } else {
                Some(department_filter.get())
            };
            store.filter.set(filter);

            // Update pagination totals
            let mut pagination = store.pagination.get();
            pagination.update_totals(get_filtered_plans(&store).len() as u32);
            pagination.current_page = 1;
            store.pagination.set(pagination);
        })
    };

    // Handle search input
    let on_search = {
        let search = search.clone();
        let update_filter = update_filter.clone();
        Callback::new(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            search.set(input.value());
            update_filter.call(());
        })
    };

    // Handle status filter
    let on_status_change = {
        let status_filter = status_filter.clone();
        let update_filter = update_filter.clone();
        Callback::new(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            status_filter.set(select.value());
            update_filter.call(());
        })
    };

    // Handle fiscal year filter
    let on_fiscal_year_change = {
        let fiscal_year_filter = fiscal_year_filter.clone();
        let update_filter = update_filter.clone();
        Callback::new(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            fiscal_year_filter.set(select.value());
            update_filter.call(());
        })
    };

    // Handle department filter
    let on_department_change = {
        let department_filter = department_filter.clone();
        let update_filter = update_filter.clone();
        Callback::new(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            department_filter.set(select.value());
            update_filter.call(());
        })
    };

    // Handle clear filters
    let on_clear_filters = {
        let search = search.clone();
        let status_filter = status_filter.clone();
        let fiscal_year_filter = fiscal_year_filter.clone();
        let department_filter = department_filter.clone();
        let store = store.clone();
        Callback::<()>::new(move |_| {
            search.set(String::new());
            status_filter.set(String::new());
            fiscal_year_filter.set(String::new());
            department_filter.set(String::new());
            store.filter.set(SourcingPlanFilter::default());

            let mut pagination = store.pagination.get();
            pagination.update_totals(store.plans.get().len() as u32);
            pagination.current_page = 1;
            store.pagination.set(pagination);
        })
    };

    // Handle page change
    let on_page_change = {
        let store = store.clone();
        Callback::new(move |page: u32| {
            let mut pagination = store.pagination.get();
            pagination.current_page = page;
            store.pagination.set(pagination);
        })
    };

    // Handle row click
    let on_row_click = {
        let store = store.clone();
        Callback::new(move |id: String| {
            store.set_selected(&id);
            // In production, navigate to detail view
            web_sys::console::log_1(&format!("Selected sourcing plan: {}", id).into());
        })
    };

    // Table columns
    let columns = vec![
        DataTableColumn { key: "id".to_string(), label: "Plan ID".to_string(), width: Some("110px".to_string()), align: None, cell_class: Some("id-cell".to_string()) },
        DataTableColumn { key: "title".to_string(), label: "Title".to_string(), width: None, align: None, cell_class: None },
        DataTableColumn { key: "fiscal_year".to_string(), label: "Fiscal Year".to_string(), width: Some("100px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "department".to_string(), label: "Department".to_string(), width: Some("150px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "budget".to_string(), label: "Total Budget".to_string(), width: Some("140px".to_string()), align: Some("right".to_string()), cell_class: Some("amount-cell".to_string()) },
        DataTableColumn { key: "progress".to_string(), label: "Progress".to_string(), width: Some("130px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "status".to_string(), label: "Status".to_string(), width: Some("120px".to_string()), align: None, cell_class: None },
    ];

    // Transform sourcing plans to table rows
    let plans = get_paginated_plans(&store);
    let rows: Vec<DataTableRow> = plans.iter().map(|plan| {
        let status = match plan.status {
            SourcingPlanStatus::Draft => status_badge(StatusType::Draft),
            SourcingPlanStatus::UnderReview => status_badge(StatusType::Pending),
            SourcingPlanStatus::Approved => status_badge(StatusType::Approved),
            SourcingPlanStatus::Active => status_badge(StatusType::InProgress),
            SourcingPlanStatus::Completed => status_badge(StatusType::Complete),
            SourcingPlanStatus::Cancelled => status_badge(StatusType::Cancelled),
        };

        let completion = plan.completion_percentage();
        let budget_util = plan.budget.utilization_percentage();

        DataTableRow {
            id: plan.id.clone(),
            cells: vec![
                view! { <span class="id-cell">{plan.id.clone()}</span> },
                view! {
                    <div class="title-cell">
                        <span class="plan-title">{plan.title.clone()}</span>
                        <span class="plan-owner">{format!("Owner: {}", plan.owner)}</span>
                    </div>
                },
                view! { <span class="fiscal-year">{plan.fiscal_year.clone()}</span> },
                view! { <span>{plan.department.clone()}</span> },
                view! { <span class="amount-cell">{format_currency(plan.budget.total_budget)}</span> },
                view! {
                    <div class="progress-cell">
                        {progress_bar(completion as u32, 100, Some(format!("{:.0}%", completion)))}
                        <span class="progress-label">{format!("{}/{} tenders", plan.total_completed_tenders(), plan.total_planned_tenders())}</span>
                    </div>
                },
                status,
            ],
        }
    }).collect();

    let pagination_state = store.pagination.get();
    let total_filtered = get_filtered_plans(&store).len();

    // Calculate summary stats
    let all_plans = store.plans.get();
    let active_plans = all_plans.iter().filter(|p| p.status == SourcingPlanStatus::Active).count();
    let total_budget: f64 = all_plans.iter()
        .filter(|p| p.status == SourcingPlanStatus::Active)
        .map(|p| p.budget.total_budget)
        .sum();
    let total_spent: f64 = all_plans.iter()
        .filter(|p| p.status == SourcingPlanStatus::Active)
        .map(|p| p.budget.spent_amount)
        .sum();

    view! {
        style {
            r#"
            .sourcing-list { display: flex; flex-direction: column; gap: var(--space-4); }
            .title-cell {
                display: flex;
                flex-direction: column;
                gap: 2px;
            }
            .plan-title {
                font-weight: 500;
                max-width: 280px;
                overflow: hidden;
                text-overflow: ellipsis;
                white-space: nowrap;
            }
            .plan-owner {
                font-size: 12px;
                color: var(--text-muted);
            }
            .fiscal-year {
                font-family: IBM Plex Mono, monospace;
                font-size: 13px;
            }
            .progress-cell {
                display: flex;
                flex-direction: column;
                gap: 4px;
            }
            .progress-label {
                font-size: 11px;
                color: var(--text-muted);
            }
            .list-info {
                display: flex;
                align-items: center;
                justify-content: space-between;
                padding: 12px 0;
                font-size: 13px;
                color: var(--text-muted);
            }
            .export-buttons {
                display: flex;
                gap: 8px;
            }
            .summary-cards {
                display: grid;
                grid-template-columns: repeat(4, 1fr);
                gap: 16px;
                margin-bottom: 8px;
            }
            .summary-card {
                background: white;
                border: 1px solid var(--border);
                border-radius: var(--radius);
                padding: 16px;
            }
            .summary-card-label {
                font-size: 12px;
                color: var(--text-muted);
                margin-bottom: 4px;
            }
            .summary-card-value {
                font-size: 24px;
                font-weight: 600;
                color: var(--navy);
            }
            .summary-card-value.currency {
                font-family: IBM Plex Mono, monospace;
                font-size: 20px;
            }
            .summary-card-sub {
                font-size: 12px;
                color: var(--text-muted);
                margin-top: 4px;
            }
            "#
        }

        <div class="sourcing-list" data-testid="sourcing-list">
            {page_header(
                "Sourcing Plans".to_string(),
                Some("Annual procurement planning and budget management".to_string()),
                vec![
                    view! { <button class="btn btn-secondary">"Export"</button> },
                    view! { <a href="/sourcing-plans/new" class="btn btn-primary">"+ New Sourcing Plan"</a> },
                ]
            )}

            // Summary cards
            <div class="summary-cards">
                <div class="summary-card">
                    <div class="summary-card-label">"Active Plans"</div>
                    <div class="summary-card-value">{active_plans}</div>
                    <div class="summary-card-sub">{format!("{} total plans", all_plans.len())}</div>
                </div>
                <div class="summary-card">
                    <div class="summary-card-label">"Total Active Budget"</div>
                    <div class="summary-card-value currency">{format_currency(total_budget)}</div>
                    <div class="summary-card-sub">"FY 2025/26"</div>
                </div>
                <div class="summary-card">
                    <div class="summary-card-label">"Budget Utilized"</div>
                    <div class="summary-card-value currency">{format_currency(total_spent)}</div>
                    <div class="summary-card-sub">{format!("{:.1}% of total", if total_budget > 0.0 { (total_spent / total_budget) * 100.0 } else { 0.0 })}</div>
                </div>
                <div class="summary-card">
                    <div class="summary-card-label">"Available Budget"</div>
                    <div class="summary-card-value currency">{format_currency(total_budget - total_spent)}</div>
                    <div class="summary-card-sub">"Remaining"</div>
                </div>
            </div>

            // Filter bar
            {filter_bar(vec![
                view! {
                    <div class="filter-group">
                        <label>"Search"</label>
                        <input
                            type="text"
                            placeholder="Search by ID, title, owner..."
                            value={search.get()}
                            on:input={on_search}
                            style="width: 220px;"
                        />
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"Status"</label>
                        <select on:change={on_status_change}>
                            <option value="">"All Statuses"</option>
                            <option value="draft">"Draft"</option>
                            <option value="under_review">"Under Review"</option>
                            <option value="approved">"Approved"</option>
                            <option value="active">"Active"</option>
                            <option value="completed">"Completed"</option>
                            <option value="cancelled">"Cancelled"</option>
                        </select>
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"Fiscal Year"</label>
                        <select on:change={on_fiscal_year_change}>
                            <option value="">"All Years"</option>
                            <option value="2025/26">"2025/26"</option>
                            <option value="2024/25">"2024/25"</option>
                            <option value="2023/24">"2023/24"</option>
                        </select>
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"Department"</label>
                        <select on:change={on_department_change}>
                            <option value="">"All Departments"</option>
                            <option value="Procurement Division">"Procurement Division"</option>
                            <option value="Information Technology">"Information Technology"</option>
                            <option value="Facilities">"Facilities"</option>
                            <option value="Transport">"Transport"</option>
                            <option value="Human Resources">"Human Resources"</option>
                            <option value="Finance">"Finance"</option>
                        </select>
                    </div>
                },
                view! { <div class="filter-spacer"></div> },
                view! {
                    <button class="btn btn-sm btn-secondary" on:click={on_clear_filters}>"Clear Filters"</button>
                },
            ])}

            // Sourcing plans table
            {panel(
                "All Sourcing Plans".to_string(),
                vec![],
                vec![
                    view! {
                        <div class="list-info">
                            <span>{format!("Showing {} of {} plans", rows.len(), total_filtered)}</span>
                            <div class="export-buttons">
                                <button class="btn btn-sm btn-secondary">"CSV"</button>
                                <button class="btn btn-sm btn-secondary">"PDF"</button>
                            </div>
                        </div>
                    },
                    data_table(columns, rows, Some(on_row_click)),
                    view! {
                        <div>
                            {pagination(
                                pagination_state.current_page,
                                pagination_state.total_pages.max(1),
                                on_page_change
                            )}
                        </div>
                    },
                ]
            )}
        </div>
    }
}
