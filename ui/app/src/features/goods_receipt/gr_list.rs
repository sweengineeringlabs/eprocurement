//! Goods Receipt list view

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
    tab_bar, Tab,
};
use crate::shared::forms::filter_bar;
use crate::util::format::{format_currency, format_date};
use super::types::{GoodsReceiptStatus, InspectionStatus, GoodsReceiptSummary};
use super::store::GoodsReceiptStore;
use super::service;

/// Goods Receipt list page
#[component]
pub fn gr_list() -> View {
    let store = use_context::<GoodsReceiptStore>();

    // Load data on mount
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                service::load_receipts(&store).await;
            });
        }
    });

    // Tab state
    let active_tab = signal("pending".to_string());

    // Filter signals
    let status_filter = signal(String::new());
    let inspection_filter = signal(String::new());
    let search_filter = signal(String::new());

    // Handle tab change
    let handle_tab_change = Callback::new({
        let active_tab = active_tab.clone();
        move |tab: String| {
            active_tab.set(tab);
        }
    });

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
                Some(GoodsReceiptStatus::from_str(&value))
            };
            store.set_filter_status(status);
        }
    });

    let handle_inspection_change = Callback::new({
        let store = store.clone();
        let inspection_filter = inspection_filter.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            let value = select.value();
            inspection_filter.set(value.clone());

            let inspection = if value.is_empty() {
                None
            } else {
                Some(InspectionStatus::from_str(&value))
            };
            store.set_filter_inspection(inspection);
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

    let handle_clear_filters = Callback::new({
        let store = store.clone();
        let status_filter = status_filter.clone();
        let inspection_filter = inspection_filter.clone();
        let search_filter = search_filter.clone();
        move |_| {
            status_filter.set(String::new());
            inspection_filter.set(String::new());
            search_filter.set(String::new());
            store.clear_filters();
        }
    });

    // Get receipts based on active tab
    let pending_receipts = store.get_pending_receipts();
    let completed_receipts = store.get_completed_receipts();
    let filtered_receipts = store.get_filtered_receipts();

    let current_receipts = match active_tab.get().as_str() {
        "pending" => pending_receipts.clone(),
        "completed" => completed_receipts.clone(),
        _ => filtered_receipts.clone(),
    };

    let loading = store.loading.get();

    // Count statistics
    let pending_count = pending_receipts.len();
    let pending_inspection_count = store.receipts.get()
        .iter()
        .filter(|r| {
            r.inspection_status == InspectionStatus::Pending
                || r.inspection_status == InspectionStatus::InProgress
        })
        .count();

    // Tabs
    let tabs = vec![
        Tab {
            id: "pending".to_string(),
            label: format!("Pending Receipts ({})", pending_count),
            icon: None,
            active: active_tab.get() == "pending",
        },
        Tab {
            id: "completed".to_string(),
            label: format!("Completed ({})", completed_receipts.len()),
            icon: None,
            active: active_tab.get() == "completed",
        },
        Tab {
            id: "all".to_string(),
            label: format!("All Receipts ({})", store.receipts.get().len()),
            icon: None,
            active: active_tab.get() == "all",
        },
    ];

    // Table columns
    let columns = vec![
        DataTableColumn {
            key: "id".to_string(),
            label: "GR Number".to_string(),
            width: Some("120px".to_string()),
            align: None,
            cell_class: Some("id-cell".to_string()),
        },
        DataTableColumn {
            key: "po_number".to_string(),
            label: "PO Reference".to_string(),
            width: Some("120px".to_string()),
            align: None,
            cell_class: Some("id-cell".to_string()),
        },
        DataTableColumn {
            key: "supplier".to_string(),
            label: "Supplier".to_string(),
            width: Some("200px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "items".to_string(),
            label: "Items".to_string(),
            width: Some("80px".to_string()),
            align: Some("center".to_string()),
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
            key: "progress".to_string(),
            label: "Progress".to_string(),
            width: Some("120px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "inspection".to_string(),
            label: "Inspection".to_string(),
            width: Some("120px".to_string()),
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
        DataTableColumn {
            key: "received_by".to_string(),
            label: "Received By".to_string(),
            width: Some("120px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "actions".to_string(),
            label: "".to_string(),
            width: Some("80px".to_string()),
            align: Some("right".to_string()),
            cell_class: None,
        },
    ];

    // Transform receipts to table rows
    let rows: Vec<DataTableRow> = current_receipts
        .iter()
        .map(|receipt| {
            let status_view = get_status_badge(&receipt.status);
            let inspection_view = get_inspection_badge(&receipt.inspection_status);
            let bbbee_view = get_bbbee_badge(receipt.supplier_bbbee_level);
            let progress_view = get_progress_view(receipt.completion_percentage);

            DataTableRow {
                id: receipt.id.clone(),
                cells: vec![
                    view! {
                        <a href={format!("/goods-receipt/{}", receipt.id)} class="id-link">
                            {receipt.id.clone()}
                        </a>
                    },
                    view! {
                        <a href={format!("/purchase-orders/{}", receipt.po_number)} class="id-link">
                            {receipt.po_number.clone()}
                        </a>
                    },
                    view! {
                        <div class="supplier-cell">
                            <span class="supplier-name">{receipt.supplier_name.clone()}</span>
                            {bbbee_view}
                        </div>
                    },
                    view! {
                        <span class="items-count">{receipt.total_items.to_string()}</span>
                    },
                    view! {
                        <span class="amount-cell">{format_currency(receipt.total_value)}</span>
                    },
                    progress_view,
                    inspection_view,
                    status_view,
                    view! {
                        <span class="received-by">{receipt.received_by.clone()}</span>
                    },
                    view! {
                        <div class="row-actions">
                            <a href={format!("/goods-receipt/{}", receipt.id)} class="btn btn-sm btn-secondary">
                                {if receipt.status == GoodsReceiptStatus::Pending { "Process" } else { "View" }}
                            </a>
                        </div>
                    },
                ],
            }
        })
        .collect();

    view! {
        style {
            r#"
            .gr-page { display: flex; flex-direction: column; gap: var(--space-6); }
            .id-link {
                color: var(--blue);
                text-decoration: none;
                font-weight: 500;
                font-family: IBM Plex Mono, monospace;
                font-size: 12px;
            }
            .id-link:hover { text-decoration: underline; }
            .supplier-cell { display: flex; flex-direction: column; gap: 4px; }
            .supplier-name { font-size: 13px; }
            .items-count {
                font-family: IBM Plex Mono, monospace;
                font-size: 12px;
                font-weight: 500;
            }
            .amount-cell {
                font-family: IBM Plex Mono, monospace;
                font-size: 12px;
                font-weight: 500;
            }
            .progress-cell { min-width: 100px; }
            .received-by { font-size: 12px; color: var(--text-muted); }
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
            .stat-item.info .count { color: var(--blue); }

            .inspection-badge {
                display: inline-flex;
                align-items: center;
                gap: 4px;
                padding: 2px 8px;
                border-radius: var(--radius-sm);
                font-size: 11px;
                font-weight: 500;
            }
            .inspection-badge.pending {
                background: var(--gray-100);
                color: var(--gray-700);
            }
            .inspection-badge.in-progress {
                background: var(--blue-100);
                color: var(--blue-700);
            }
            .inspection-badge.passed {
                background: var(--green-100);
                color: var(--green-700);
            }
            .inspection-badge.failed {
                background: var(--red-100);
                color: var(--red-700);
            }
            .inspection-badge.partial {
                background: var(--orange-100);
                color: var(--orange-700);
            }
            .inspection-badge.waived {
                background: var(--purple-100);
                color: var(--purple-700);
            }
            "#
        }

        <div class="gr-page" data-testid="gr-list">
            {page_header(
                "Goods Receipt".to_string(),
                Some("Process and track goods received from suppliers".to_string()),
                vec![
                    view! { <button class="btn btn-secondary">"Export"</button> },
                    view! { <a href="/goods-receipt/new" class="btn btn-primary">"New Receipt"</a> },
                ]
            )}

            // Pending inspection warning
            if pending_inspection_count > 0 {
                {notice_bar(
                    format!("{} receipt(s) pending inspection. Complete quality checks before acceptance.", pending_inspection_count),
                    NoticeType::Warning,
                    None
                )}
            }

            // Stats row
            <div class="stats-row">
                <div class={if pending_count > 0 { "stat-item warning" } else { "stat-item" }}>
                    <span class="label">"Pending Processing:"</span>
                    <span class="count">{pending_count.to_string()}</span>
                </div>
                <div class={if pending_inspection_count > 0 { "stat-item info" } else { "stat-item" }}>
                    <span class="label">"Awaiting Inspection:"</span>
                    <span class="count">{pending_inspection_count.to_string()}</span>
                </div>
                <div class="stat-item">
                    <span class="label">"Completed Today:"</span>
                    <span class="count">{
                        store.receipts.get().iter()
                            .filter(|r| r.status == GoodsReceiptStatus::Completed && r.receipt_date == "2025-02-27")
                            .count().to_string()
                    }</span>
                </div>
            </div>

            // Tabs
            {tab_bar(tabs, handle_tab_change)}

            // Filters (only show on "all" tab)
            if active_tab.get() == "all" {
                {filter_bar(vec![
                    view! {
                        <div class="filter-group">
                            <label>"Status:"</label>
                            <select on:change={handle_status_change}>
                                <option value="">"All Statuses"</option>
                                <option value="draft">"Draft"</option>
                                <option value="pending">"Pending"</option>
                                <option value="partially_received">"Partially Received"</option>
                                <option value="completed">"Completed"</option>
                                <option value="rejected">"Rejected"</option>
                                <option value="cancelled">"Cancelled"</option>
                            </select>
                        </div>
                    },
                    view! {
                        <div class="filter-group">
                            <label>"Inspection:"</label>
                            <select on:change={handle_inspection_change}>
                                <option value="">"All"</option>
                                <option value="pending">"Pending"</option>
                                <option value="in_progress">"In Progress"</option>
                                <option value="passed">"Passed"</option>
                                <option value="failed">"Failed"</option>
                                <option value="partial_pass">"Partial Pass"</option>
                                <option value="waived">"Waived"</option>
                            </select>
                        </div>
                    },
                    view! { <div class="filter-spacer"></div> },
                    view! {
                        <div class="filter-group">
                            <input
                                type="text"
                                placeholder="Search GR, PO, supplier..."
                                value={search_filter.get()}
                                on:input={handle_search_change}
                            />
                        </div>
                    },
                    view! {
                        <button class="btn btn-sm btn-secondary" on:click={handle_clear_filters}>"Clear"</button>
                    },
                ])}
            }

            // Receipts table
            if loading {
                <div class="loading-state">"Loading goods receipts..."</div>
            } else if rows.is_empty() {
                {empty_state(
                    match active_tab.get().as_str() {
                        "pending" => "No pending receipts".to_string(),
                        "completed" => "No completed receipts".to_string(),
                        _ => "No goods receipts found".to_string()
                    },
                    Some(match active_tab.get().as_str() {
                        "pending" => "All incoming deliveries have been processed".to_string(),
                        "completed" => "No receipts have been completed yet".to_string(),
                        _ => "Try adjusting your filters or create a new goods receipt".to_string()
                    }),
                    Some(view! { <a href="/goods-receipt/new" class="btn btn-primary">"New Receipt"</a> })
                )}
            } else {
                {panel(
                    format!("{} ({})",
                        match active_tab.get().as_str() {
                            "pending" => "Pending Receipts",
                            "completed" => "Completed Receipts",
                            _ => "All Receipts"
                        },
                        rows.len()
                    ),
                    vec![],
                    vec![data_table(columns, rows, None)]
                )}
            }
        </div>
    }
}

/// Get status badge view
fn get_status_badge(status: &GoodsReceiptStatus) -> View {
    let status_type = match status {
        GoodsReceiptStatus::Draft => StatusType::Draft,
        GoodsReceiptStatus::Pending => StatusType::Pending,
        GoodsReceiptStatus::PartiallyReceived => StatusType::InProgress,
        GoodsReceiptStatus::Completed => StatusType::Complete,
        GoodsReceiptStatus::Rejected => StatusType::Cancelled,
        GoodsReceiptStatus::Cancelled => StatusType::Cancelled,
    };
    status_badge(status_type)
}

/// Get inspection badge view
fn get_inspection_badge(status: &InspectionStatus) -> View {
    let (class, label) = match status {
        InspectionStatus::Pending => ("inspection-badge pending", "Pending"),
        InspectionStatus::InProgress => ("inspection-badge in-progress", "In Progress"),
        InspectionStatus::Passed => ("inspection-badge passed", "Passed"),
        InspectionStatus::Failed => ("inspection-badge failed", "Failed"),
        InspectionStatus::PartialPass => ("inspection-badge partial", "Partial"),
        InspectionStatus::Waived => ("inspection-badge waived", "Waived"),
    };

    view! {
        <span class={class}>{label}</span>
    }
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

/// Get progress bar view
fn get_progress_view(progress: f64) -> View {
    let color = if progress >= 100.0 {
        ProgressColor::Green
    } else if progress >= 75.0 {
        ProgressColor::Blue
    } else if progress >= 50.0 {
        ProgressColor::Orange
    } else if progress > 0.0 {
        ProgressColor::Red
    } else {
        ProgressColor::Gray
    };

    view! {
        <div class="progress-cell">
            {progress_bar(progress, color, true, None)}
        </div>
    }
}
