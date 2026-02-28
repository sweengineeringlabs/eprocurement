//! Contract list view

use components::prelude::*;
use wasm_bindgen::JsCast;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, data_table, DataTableColumn, DataTableRow,
    status_badge, StatusType,
    bbbee_badge, BbbeeLevel,
    notice_bar, NoticeType,
    progress_bar, ProgressColor,
    empty_state,
};
use crate::shared::forms::filter_bar;
use crate::util::format::{format_currency, format_date};
use super::types::{ContractStatus, ContractSummary};
use super::store::ContractsStore;
use super::service;

/// Contract list page
#[component]
pub fn contract_list() -> View {
    let store = use_context::<ContractsStore>();

    // Load data on mount
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                service::load_contracts(&store).await;
            });
        }
    });

    // Filter signals
    let status_filter = signal(String::new());
    let supplier_filter = signal(String::new());
    let search_filter = signal(String::new());
    let show_expiring = signal(false);

    // Handle filter changes
    let handle_status_change = Callback::new({
        let store = store.clone();
        let status_filter = status_filter.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            let value = select.value();
            status_filter.set(value.clone());

            let status = if value.is_empty() {
                None
            } else {
                Some(ContractStatus::from_str(&value))
            };
            store.set_filter_status(status);
        }
    });

    let handle_supplier_change = Callback::new({
        let store = store.clone();
        let supplier_filter = supplier_filter.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let value = input.value();
            supplier_filter.set(value.clone());
            store.set_filter_supplier(if value.is_empty() { None } else { Some(value) });
        }
    });

    let handle_search_change = Callback::new({
        let store = store.clone();
        let search_filter = search_filter.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let value = input.value();
            search_filter.set(value.clone());
            store.set_filter_search(if value.is_empty() { None } else { Some(value) });
        }
    });

    let handle_expiring_toggle: Callback<()> = Callback::new({
        let store = store.clone();
        let show_expiring = show_expiring.clone();
        move |_| {
            let new_value = !show_expiring.get();
            show_expiring.set(new_value);
            store.set_filter_expiring(if new_value { Some(30) } else { None });
        }
    });

    let handle_clear_filters: Callback<()> = Callback::new({
        let store = store.clone();
        let status_filter = status_filter.clone();
        let supplier_filter = supplier_filter.clone();
        let search_filter = search_filter.clone();
        let show_expiring = show_expiring.clone();
        move |_| {
            status_filter.set(String::new());
            supplier_filter.set(String::new());
            search_filter.set(String::new());
            show_expiring.set(false);
            store.clear_filters();
        }
    });

    // Get filtered contracts
    let contracts = store.get_filtered_contracts();
    let loading = store.loading.get();

    // Count expiring contracts (within 30 days)
    let expiring_count = store.contracts.get()
        .iter()
        .filter(|c| {
            c.status == ContractStatus::Active &&
            c.days_to_expiry.map(|d| d > 0 && d <= 30).unwrap_or(false)
        })
        .count();

    // Table columns
    let columns = vec![
        DataTableColumn { key: "id".to_string(), label: "Contract ID".to_string(), width: Some("120px".to_string()), align: None, cell_class: Some("id-cell".to_string()) },
        DataTableColumn { key: "title".to_string(), label: "Title".to_string(), width: None, align: None, cell_class: None },
        DataTableColumn { key: "supplier".to_string(), label: "Supplier".to_string(), width: Some("200px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "value".to_string(), label: "Value".to_string(), width: Some("120px".to_string()), align: Some("right".to_string()), cell_class: Some("amount-cell".to_string()) },
        DataTableColumn { key: "period".to_string(), label: "Period".to_string(), width: Some("180px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "progress".to_string(), label: "Progress".to_string(), width: Some("120px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "status".to_string(), label: "Status".to_string(), width: Some("120px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "actions".to_string(), label: "".to_string(), width: Some("80px".to_string()), align: Some("right".to_string()), cell_class: None },
    ];

    // Transform contracts to table rows
    let rows: Vec<DataTableRow> = contracts.iter().map(|contract| {
        let status_view = get_status_badge(&contract.status);
        let bbbee_view = get_bbbee_badge(contract.supplier_bbbee_level);
        let expiry_warning = get_expiry_warning(contract);
        let progress_view = get_progress_view(contract.milestone_progress);

        let period = format!("{} - {}", format_date(&contract.start_date), format_date(&contract.end_date));

        DataTableRow {
            id: contract.id.clone(),
            cells: vec![
                view! { <a href={format!("/contracts/{}", contract.id)} class="id-link">{contract.id.clone()}</a> },
                view! {
                    <div class="contract-title-cell">
                        <span class="title">{contract.title.clone()}</span>
                        {expiry_warning}
                    </div>
                },
                view! {
                    <div class="supplier-cell">
                        <span class="supplier-name">{contract.supplier_name.clone()}</span>
                        {bbbee_view}
                    </div>
                },
                view! { <span class="amount-cell">{format_currency(contract.value)}</span> },
                view! { <span class="period-cell">{period}</span> },
                progress_view,
                status_view,
                view! {
                    <div class="row-actions">
                        <a href={format!("/contracts/{}", contract.id)} class="btn btn-sm btn-secondary">"View"</a>
                    </div>
                },
            ],
        }
    }).collect();

    view! {
        style {
            r#"
            .contracts-page { display: flex; flex-direction: column; gap: var(--space-6); }
            .id-link {
                color: var(--blue);
                text-decoration: none;
                font-weight: 500;
                font-family: IBM Plex Mono, monospace;
                font-size: 12px;
            }
            .id-link:hover { text-decoration: underline; }
            .contract-title-cell { display: flex; flex-direction: column; gap: 4px; }
            .contract-title-cell .title { font-weight: 500; }
            .contract-title-cell .expiry-warning {
                display: inline-flex;
                align-items: center;
                gap: 4px;
                font-size: 11px;
                color: var(--orange);
            }
            .contract-title-cell .expiry-warning.critical { color: var(--red); }
            .contract-title-cell .expiry-warning svg { width: 12px; height: 12px; }
            .supplier-cell { display: flex; flex-direction: column; gap: 4px; }
            .supplier-name { font-size: 13px; }
            .amount-cell {
                font-family: IBM Plex Mono, monospace;
                font-size: 12px;
                font-weight: 500;
            }
            .period-cell { font-size: 12px; color: var(--text-muted); }
            .progress-cell { min-width: 100px; }
            .row-actions { display: flex; gap: 8px; justify-content: flex-end; }

            .filter-group { display: flex; align-items: center; gap: 8px; }
            .filter-group label { font-size: 12px; font-weight: 500; color: var(--text-muted); }
            .filter-group select,
            .filter-group input {
                padding: 6px 10px;
                border: 1px solid var(--border);
                border-radius: var(--radius-sm);
                font-size: 12px;
                background: var(--surface);
            }
            .filter-group select:focus,
            .filter-group input:focus {
                outline: none;
                border-color: var(--blue);
            }
            .filter-spacer { flex: 1; }
            .filter-checkbox {
                display: flex;
                align-items: center;
                gap: 6px;
                font-size: 12px;
                cursor: pointer;
            }
            .filter-checkbox input { cursor: pointer; }

            .stats-row {
                display: flex;
                gap: 16px;
                margin-bottom: 8px;
            }
            .stat-item {
                display: flex;
                align-items: center;
                gap: 8px;
                padding: 8px 12px;
                background: var(--bg);
                border-radius: var(--radius-sm);
                font-size: 12px;
            }
            .stat-item .count {
                font-weight: 600;
                font-size: 14px;
            }
            .stat-item.warning .count { color: var(--orange); }
            .stat-item.danger .count { color: var(--red); }
            "#
        }

        <div class="contracts-page" data-testid="contract-list">
            {page_header(
                "Contracts".to_string(),
                Some("Manage supplier contracts and agreements".to_string()),
                vec![
                    view! { <button class="btn btn-secondary">"Export"</button> },
                    view! { <a href="/contracts/new" class="btn btn-primary">"New Contract"</a> },
                ]
            )}

            // Expiry warning notice
            if expiring_count > 0 {
                {notice_bar(
                    format!("{} contract(s) expiring within 30 days. Review and initiate renewal process.", expiring_count),
                    NoticeType::Warning,
                    None
                )}
            }

            // Stats row
            <div class="stats-row">
                <div class="stat-item">
                    <span class="label">"Total Contracts:"</span>
                    <span class="count">{store.contracts.get().len().to_string()}</span>
                </div>
                <div class={if expiring_count > 0 { "stat-item warning" } else { "stat-item" }}>
                    <span class="label">"Expiring (30 days):"</span>
                    <span class="count">{expiring_count.to_string()}</span>
                </div>
                <div class="stat-item">
                    <span class="label">"Active:"</span>
                    <span class="count">{
                        store.contracts.get().iter().filter(|c| c.status == ContractStatus::Active).count().to_string()
                    }</span>
                </div>
            </div>

            // Filters
            {filter_bar(vec![
                view! {
                    <div class="filter-group">
                        <label>"Status:"</label>
                        <select on:change={handle_status_change}>
                            <option value="">"All Statuses"</option>
                            <option value="draft">"Draft"</option>
                            <option value="pending_approval">"Pending Approval"</option>
                            <option value="active">"Active"</option>
                            <option value="suspended">"Suspended"</option>
                            <option value="expired">"Expired"</option>
                            <option value="terminated">"Terminated"</option>
                            <option value="completed">"Completed"</option>
                        </select>
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"Supplier:"</label>
                        <input
                            type="text"
                            placeholder="Search supplier..."
                            value={supplier_filter.get()}
                            on:input={handle_supplier_change}
                        />
                    </div>
                },
                view! {
                    <label class="filter-checkbox">
                        <input
                            type="checkbox"
                            checked={show_expiring.get()}
                            on:change={handle_expiring_toggle}
                        />
                        "Expiring within 30 days"
                    </label>
                },
                view! { <div class="filter-spacer"></div> },
                view! {
                    <div class="filter-group">
                        <input
                            type="text"
                            placeholder="Search contracts..."
                            value={search_filter.get()}
                            on:input={handle_search_change}
                        />
                    </div>
                },
                view! {
                    <button class="btn btn-sm btn-secondary" on:click={handle_clear_filters}>"Clear"</button>
                },
            ])}

            // Contracts table
            if loading {
                <div class="loading-state">"Loading contracts..."</div>
            } else if rows.is_empty() {
                {empty_state(
                    "No contracts found".to_string(),
                    Some("Try adjusting your filters or create a new contract".to_string()),
                    Some(view! { <a href="/contracts/new" class="btn btn-primary">"Create Contract"</a> })
                )}
            } else {
                {panel(
                    format!("Contracts ({})", rows.len()),
                    vec![],
                    vec![data_table(columns, rows, None)]
                )}
            }
        </div>
    }
}

/// Get status badge view
fn get_status_badge(status: &ContractStatus) -> View {
    let status_type = match status {
        ContractStatus::Draft => StatusType::Draft,
        ContractStatus::PendingApproval => StatusType::Pending,
        ContractStatus::Active => StatusType::Active,
        ContractStatus::Suspended => StatusType::InProgress,
        ContractStatus::Expired => StatusType::Expired,
        ContractStatus::Terminated => StatusType::Cancelled,
        ContractStatus::Completed => StatusType::Complete,
    };
    status_badge(status_type)
}

/// Get B-BBEE badge view
fn get_bbbee_badge(level: u8) -> View {
    let bbbee_level = match level {
        1 => BbbeeLevel::Level1,
        2 => BbbeeLevel::Level2,
        3 => BbbeeLevel::Level3,
        4 => BbbeeLevel::Level4,
        _ => BbbeeLevel::NonCompliant,
    };
    bbbee_badge(bbbee_level)
}

/// Get expiry warning view
fn get_expiry_warning(contract: &ContractSummary) -> View {
    if contract.status != ContractStatus::Active {
        return view! { <span></span> };
    }

    match contract.days_to_expiry {
        Some(days) if days <= 0 => {
            view! {
                <span class="expiry-warning critical">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <circle cx="12" cy="12" r="10"/>
                        <line x1="12" y1="8" x2="12" y2="12"/>
                        <line x1="12" y1="16" x2="12.01" y2="16"/>
                    </svg>
                    "Expired"
                </span>
            }
        }
        Some(days) if days <= 7 => {
            view! {
                <span class="expiry-warning critical">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <circle cx="12" cy="12" r="10"/>
                        <line x1="12" y1="8" x2="12" y2="12"/>
                        <line x1="12" y1="16" x2="12.01" y2="16"/>
                    </svg>
                    {format!("Expires in {} days", days)}
                </span>
            }
        }
        Some(days) if days <= 30 => {
            view! {
                <span class="expiry-warning">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <circle cx="12" cy="12" r="10"/>
                        <polyline points="12 6 12 12 16 14"/>
                    </svg>
                    {format!("Expires in {} days", days)}
                </span>
            }
        }
        _ => view! { <span></span> },
    }
}

/// Get progress bar view
fn get_progress_view(progress: f64) -> View {
    let color = if progress >= 75.0 {
        ProgressColor::Green
    } else if progress >= 50.0 {
        ProgressColor::Blue
    } else if progress >= 25.0 {
        ProgressColor::Orange
    } else {
        ProgressColor::Red
    };

    view! {
        <div class="progress-cell">
            {progress_bar(progress, color, true, None)}
        </div>
    }
}
