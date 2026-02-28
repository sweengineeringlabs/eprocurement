//! Purchase Order list view

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
use super::types::{PurchaseOrderStatus, PurchaseOrderSummary};
use super::store::PurchaseOrdersStore;
use super::service;

/// Purchase Order list page
#[component]
pub fn po_list() -> View {
    let store = use_context::<PurchaseOrdersStore>();

    // Load data on mount
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                service::load_purchase_orders(&store).await;
            });
        }
    });

    // Filter signals
    let status_filter = signal(String::new());
    let supplier_filter = signal(String::new());
    let contract_filter = signal(String::new());
    let search_filter = signal(String::new());
    let show_pending_delivery = signal(false);

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
                Some(PurchaseOrderStatus::from_str(&value))
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

    let handle_contract_change = Callback::new({
        let store = store.clone();
        let contract_filter = contract_filter.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let value = input.value();
            contract_filter.set(value.clone());
            store.set_filter_contract(if value.is_empty() { None } else { Some(value) });
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

    let handle_pending_delivery_toggle: Callback<()> = Callback::new({
        let store = store.clone();
        let show_pending_delivery = show_pending_delivery.clone();
        move |_| {
            let new_value = !show_pending_delivery.get();
            show_pending_delivery.set(new_value);
            store.set_filter_pending_delivery(if new_value { Some(true) } else { None });
        }
    });

    let handle_clear_filters: Callback<()> = Callback::new({
        let store = store.clone();
        let status_filter = status_filter.clone();
        let supplier_filter = supplier_filter.clone();
        let contract_filter = contract_filter.clone();
        let search_filter = search_filter.clone();
        let show_pending_delivery = show_pending_delivery.clone();
        move |_| {
            status_filter.set(String::new());
            supplier_filter.set(String::new());
            contract_filter.set(String::new());
            search_filter.set(String::new());
            show_pending_delivery.set(false);
            store.clear_filters();
        }
    });

    // Get filtered purchase orders
    let purchase_orders = store.get_filtered_purchase_orders();
    let loading = store.loading.get();

    // Count pending delivery POs
    let pending_delivery_count = store.purchase_orders.get()
        .iter()
        .filter(|po| matches!(
            po.status,
            PurchaseOrderStatus::Sent
                | PurchaseOrderStatus::Acknowledged
                | PurchaseOrderStatus::PartiallyDelivered
        ))
        .count();

    // Calculate total value
    let total_value: f64 = purchase_orders.iter().map(|po| po.total_amount).sum();

    // Table columns
    let columns = vec![
        DataTableColumn { key: "po_number".to_string(), label: "PO Number".to_string(), width: Some("120px".to_string()), align: None, cell_class: Some("id-cell".to_string()) },
        DataTableColumn { key: "contract".to_string(), label: "Contract".to_string(), width: Some("120px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "supplier".to_string(), label: "Supplier".to_string(), width: Some("200px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "amount".to_string(), label: "Amount".to_string(), width: Some("130px".to_string()), align: Some("right".to_string()), cell_class: Some("amount-cell".to_string()) },
        DataTableColumn { key: "order_date".to_string(), label: "Order Date".to_string(), width: Some("100px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "delivery".to_string(), label: "Delivery".to_string(), width: Some("120px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "status".to_string(), label: "Status".to_string(), width: Some("130px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "actions".to_string(), label: "".to_string(), width: Some("80px".to_string()), align: Some("right".to_string()), cell_class: None },
    ];

    // Transform purchase orders to table rows
    let rows: Vec<DataTableRow> = purchase_orders.iter().map(|po| {
        let status_view = get_status_badge(&po.status);
        let bbbee_view = get_bbbee_badge(po.supplier_bbbee_level);
        let delivery_view = get_delivery_view(po);

        DataTableRow {
            id: po.id.clone(),
            cells: vec![
                view! {
                    <a href={format!("/purchase-orders/{}", po.id)} class="id-link">
                        {po.po_number.clone()}
                    </a>
                },
                view! {
                    <span class="contract-ref">
                        {po.contract_ref.clone().unwrap_or_else(|| "-".to_string())}
                    </span>
                },
                view! {
                    <div class="supplier-cell">
                        <span class="supplier-name">{po.supplier_name.clone()}</span>
                        {bbbee_view}
                    </div>
                },
                view! {
                    <div class="amount-cell">
                        <span class="amount">{format_currency(po.total_amount)}</span>
                        <span class="items-count">{format!("{} items", po.line_item_count)}</span>
                    </div>
                },
                view! { <span class="date-cell">{format_date(&po.order_date)}</span> },
                delivery_view,
                status_view,
                view! {
                    <div class="row-actions">
                        <a href={format!("/purchase-orders/{}", po.id)} class="btn btn-sm btn-secondary">"View"</a>
                    </div>
                },
            ],
        }
    }).collect();

    view! {
        style {
            r#"
            .purchase-orders-page { display: flex; flex-direction: column; gap: var(--space-6); }
            .id-link {
                color: var(--blue);
                text-decoration: none;
                font-weight: 500;
                font-family: IBM Plex Mono, monospace;
                font-size: 12px;
            }
            .id-link:hover { text-decoration: underline; }
            .contract-ref {
                font-family: IBM Plex Mono, monospace;
                font-size: 11px;
                color: var(--text-muted);
            }
            .supplier-cell { display: flex; flex-direction: column; gap: 4px; }
            .supplier-name { font-size: 13px; font-weight: 500; }
            .amount-cell {
                display: flex;
                flex-direction: column;
                align-items: flex-end;
                gap: 2px;
            }
            .amount-cell .amount {
                font-family: IBM Plex Mono, monospace;
                font-size: 12px;
                font-weight: 600;
            }
            .amount-cell .items-count {
                font-size: 11px;
                color: var(--text-muted);
            }
            .date-cell { font-size: 12px; color: var(--text-muted); }
            .delivery-cell { display: flex; flex-direction: column; gap: 4px; }
            .delivery-cell .delivery-date {
                font-size: 12px;
                color: var(--text-muted);
            }
            .delivery-cell .delivery-progress { min-width: 80px; }
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
            .stat-item .value {
                font-weight: 600;
                font-size: 14px;
                font-family: IBM Plex Mono, monospace;
            }
            "#
        }

        <div class="purchase-orders-page" data-testid="po-list">
            {page_header(
                "Purchase Orders".to_string(),
                Some("Manage purchase orders and track deliveries".to_string()),
                vec![
                    view! { <button class="btn btn-secondary">"Export"</button> },
                    view! { <a href="/purchase-orders/new" class="btn btn-primary">"New Purchase Order"</a> },
                ]
            )}

            // Pending delivery notice
            if pending_delivery_count > 0 {
                {notice_bar(
                    format!("{} purchase order(s) pending delivery. Track and confirm deliveries.", pending_delivery_count),
                    NoticeType::Info,
                    None
                )}
            }

            // Stats row
            <div class="stats-row">
                <div class="stat-item">
                    <span class="label">"Total POs:"</span>
                    <span class="count">{store.purchase_orders.get().len().to_string()}</span>
                </div>
                <div class={if pending_delivery_count > 0 { "stat-item warning" } else { "stat-item" }}>
                    <span class="label">"Pending Delivery:"</span>
                    <span class="count">{pending_delivery_count.to_string()}</span>
                </div>
                <div class="stat-item">
                    <span class="label">"Total Value:"</span>
                    <span class="value">{format_currency(total_value)}</span>
                </div>
                <div class="stat-item">
                    <span class="label">"Draft:"</span>
                    <span class="count">{store.get_count_by_status(PurchaseOrderStatus::Draft).to_string()}</span>
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
                            <option value="approved">"Approved"</option>
                            <option value="sent">"Sent to Supplier"</option>
                            <option value="acknowledged">"Acknowledged"</option>
                            <option value="partially_delivered">"Partially Delivered"</option>
                            <option value="delivered">"Delivered"</option>
                            <option value="invoiced">"Invoiced"</option>
                            <option value="closed">"Closed"</option>
                            <option value="cancelled">"Cancelled"</option>
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
                    <div class="filter-group">
                        <label>"Contract:"</label>
                        <input
                            type="text"
                            placeholder="Contract ref..."
                            value={contract_filter.get()}
                            on:input={handle_contract_change}
                        />
                    </div>
                },
                view! {
                    <label class="filter-checkbox">
                        <input
                            type="checkbox"
                            checked={show_pending_delivery.get()}
                            on:change={handle_pending_delivery_toggle}
                        />
                        "Pending delivery only"
                    </label>
                },
                view! { <div class="filter-spacer"></div> },
                view! {
                    <div class="filter-group">
                        <input
                            type="text"
                            placeholder="Search POs..."
                            value={search_filter.get()}
                            on:input={handle_search_change}
                        />
                    </div>
                },
                view! {
                    <button class="btn btn-sm btn-secondary" on:click={handle_clear_filters}>"Clear"</button>
                },
            ])}

            // Purchase orders table
            if loading {
                <div class="loading-state">"Loading purchase orders..."</div>
            } else if rows.is_empty() {
                {empty_state(
                    "No purchase orders found".to_string(),
                    Some("Try adjusting your filters or create a new purchase order".to_string()),
                    None,
                    Some(view! { <a href="/purchase-orders/new" class="btn btn-primary">"Create Purchase Order"</a> })
                )}
            } else {
                {panel(
                    format!("Purchase Orders ({})", rows.len()),
                    vec![],
                    vec![data_table(columns, rows, None)]
                )}
            }
        </div>
    }
}

/// Get status badge view
fn get_status_badge(status: &PurchaseOrderStatus) -> View {
    let status_type = match status {
        PurchaseOrderStatus::Draft => StatusType::Draft,
        PurchaseOrderStatus::PendingApproval => StatusType::Pending,
        PurchaseOrderStatus::Approved => StatusType::Approved,
        PurchaseOrderStatus::Sent => StatusType::InProgress,
        PurchaseOrderStatus::Acknowledged => StatusType::InProgress,
        PurchaseOrderStatus::PartiallyDelivered => StatusType::InProgress,
        PurchaseOrderStatus::Delivered => StatusType::Complete,
        PurchaseOrderStatus::Invoiced => StatusType::Active,
        PurchaseOrderStatus::Closed => StatusType::Complete,
        PurchaseOrderStatus::Cancelled => StatusType::Cancelled,
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

/// Get delivery progress view
fn get_delivery_view(po: &PurchaseOrderSummary) -> View {
    let show_progress = matches!(
        po.status,
        PurchaseOrderStatus::Acknowledged
            | PurchaseOrderStatus::PartiallyDelivered
            | PurchaseOrderStatus::Delivered
            | PurchaseOrderStatus::Invoiced
            | PurchaseOrderStatus::Closed
    );

    let color = if po.delivery_progress >= 100.0 {
        ProgressColor::Green
    } else if po.delivery_progress >= 50.0 {
        ProgressColor::Blue
    } else if po.delivery_progress > 0.0 {
        ProgressColor::Orange
    } else {
        ProgressColor::Red
    };

    view! {
        <div class="delivery-cell">
            <span class="delivery-date">{format_date(&po.expected_delivery_date)}</span>
            if show_progress {
                <div class="delivery-progress">
                    {progress_bar(po.delivery_progress, color, true, None)}
                </div>
            }
        </div>
    }
}
