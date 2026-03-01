//! Tender list view with filtering and pagination

use components::prelude::*;
use wasm_bindgen::JsCast;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, data_table_with_testid, DataTableColumn, DataTableRow,
    status_badge, StatusType,
    tag, TagType,
    pagination,
};
use crate::shared::forms::filter_bar;
use crate::util::format::format_currency;
use super::store::TendersStore;
use super::types::{TenderType, TenderStatus, TenderFilter};
use super::service;

/// Tender list page
#[component]
pub fn tender_list() -> View {
    let store = use_context::<TendersStore>();

    // Filter signals
    let filter_type = signal(String::new());
    let filter_status = signal(String::new());
    let filter_date_from = signal(String::new());
    let filter_date_to = signal(String::new());
    let search_query = signal(String::new());

    // Load data on mount
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                service::load_tenders(&store).await;
            });
        }
    });

    let tenders = store.tenders.clone();
    let pagination_state = store.pagination.clone();
    let loading = store.loading.clone();

    // Handle filter changes
    let handle_filter_type = Callback::new({
        let filter_type = filter_type.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            filter_type.set(select.value());
        }
    });

    let handle_filter_status = Callback::new({
        let filter_status = filter_status.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            filter_status.set(select.value());
        }
    });

    let handle_date_from = Callback::new({
        let filter_date_from = filter_date_from.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            filter_date_from.set(input.value());
        }
    });

    let handle_date_to = Callback::new({
        let filter_date_to = filter_date_to.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            filter_date_to.set(input.value());
        }
    });

    let handle_search = Callback::new({
        let search_query = search_query.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            search_query.set(input.value());
        }
    });

    // Apply filters
    let handle_apply_filters = Callback::<()>::new({
        let store = store.clone();
        let filter_type = filter_type.clone();
        let filter_status = filter_status.clone();
        let filter_date_from = filter_date_from.clone();
        let filter_date_to = filter_date_to.clone();
        let search_query = search_query.clone();
        move |_| {
            let filter = TenderFilter {
                tender_type: match filter_type.get().as_str() {
                    "rfq" => Some(TenderType::Rfq),
                    "rfp" => Some(TenderType::Rfp),
                    "rft" => Some(TenderType::Rft),
                    _ => None,
                },
                status: match filter_status.get().as_str() {
                    "draft" => Some(TenderStatus::Draft),
                    "pending" => Some(TenderStatus::PendingApproval),
                    "approved" => Some(TenderStatus::Approved),
                    "published" => Some(TenderStatus::Published),
                    "open" => Some(TenderStatus::Open),
                    "closed" => Some(TenderStatus::Closed),
                    "evaluation" => Some(TenderStatus::Evaluation),
                    "awarded" => Some(TenderStatus::Awarded),
                    "cancelled" => Some(TenderStatus::Cancelled),
                    _ => None,
                },
                date_from: if filter_date_from.get().is_empty() { None } else { Some(filter_date_from.get()) },
                date_to: if filter_date_to.get().is_empty() { None } else { Some(filter_date_to.get()) },
                search_query: if search_query.get().is_empty() { None } else { Some(search_query.get()) },
                department: None,
                category: None,
            };
            store.filter.set(filter);
        }
    });

    // Clear filters
    let handle_clear_filters = Callback::<()>::new({
        let filter_type = filter_type.clone();
        let filter_status = filter_status.clone();
        let filter_date_from = filter_date_from.clone();
        let filter_date_to = filter_date_to.clone();
        let search_query = search_query.clone();
        let store = store.clone();
        move |_| {
            filter_type.set(String::new());
            filter_status.set(String::new());
            filter_date_from.set(String::new());
            filter_date_to.set(String::new());
            search_query.set(String::new());
            store.filter.set(TenderFilter::default());
        }
    });

    // Handle row click
    let handle_row_click = Callback::new({
        move |tender_id: String| {
            // Navigate to tender detail
            web_sys::window()
                .unwrap()
                .location()
                .set_href(&format!("#/tenders/{}", tender_id))
                .ok();
        }
    });

    // Handle page change
    let handle_page_change = Callback::new({
        let pagination_state = pagination_state.clone();
        move |page: u32| {
            let mut state = pagination_state.get();
            state.current_page = page;
            pagination_state.set(state);
        }
    });

    // Table columns
    let columns = vec![
        DataTableColumn { key: "reference".to_string(), label: "Reference".to_string(), width: Some("120px".to_string()), align: None, cell_class: Some("id-cell".to_string()) },
        DataTableColumn { key: "title".to_string(), label: "Title".to_string(), width: None, align: None, cell_class: None },
        DataTableColumn { key: "type".to_string(), label: "Type".to_string(), width: Some("80px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "value".to_string(), label: "Est. Value".to_string(), width: Some("140px".to_string()), align: Some("right".to_string()), cell_class: Some("amount-cell".to_string()) },
        DataTableColumn { key: "closing".to_string(), label: "Closing Date".to_string(), width: Some("120px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "bids".to_string(), label: "Bids".to_string(), width: Some("60px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "status".to_string(), label: "Status".to_string(), width: Some("120px".to_string()), align: None, cell_class: None },
    ];

    // Filter tenders based on current filter
    let filter = store.filter.clone();
    let filtered_tenders: Vec<_> = tenders.get().iter()
        .filter(|t| {
            let f = filter.get();
            let type_match = f.tender_type.map_or(true, |ft| t.tender_type == ft);
            let status_match = f.status.map_or(true, |fs| t.status == fs);
            let search_match = f.search_query.as_ref().map_or(true, |q| {
                t.title.to_lowercase().contains(&q.to_lowercase()) ||
                t.reference_number.to_lowercase().contains(&q.to_lowercase())
            });
            type_match && status_match && search_match
        })
        .cloned()
        .collect();

    // Transform tenders to table rows
    let rows: Vec<DataTableRow> = filtered_tenders.iter().map(|tender| {
        let type_tag = match tender.tender_type {
            TenderType::Rfq => tag(tender.tender_type.label().to_string(), TagType::Rfq),
            TenderType::Rfp => tag(tender.tender_type.label().to_string(), TagType::Rfp),
            TenderType::Rft => tag(tender.tender_type.label().to_string(), TagType::Tender),
        };

        let status = match tender.status {
            TenderStatus::Draft => status_badge(StatusType::Draft),
            TenderStatus::PendingApproval => status_badge(StatusType::Pending),
            TenderStatus::Approved => status_badge(StatusType::Approved),
            TenderStatus::Published => status_badge(StatusType::Published),
            TenderStatus::Open => status_badge(StatusType::Open),
            TenderStatus::Closed => status_badge(StatusType::Complete),
            TenderStatus::Evaluation => status_badge(StatusType::Evaluation),
            TenderStatus::Adjudication => status_badge(StatusType::Review),
            TenderStatus::Awarded => status_badge(StatusType::Active),
            TenderStatus::Cancelled => status_badge(StatusType::Cancelled),
        };

        let closing = tender.closing_date.clone().unwrap_or_else(|| "-".to_string());
        let bid_count = tender.bids.len().to_string();

        DataTableRow {
            id: tender.id.clone(),
            cells: vec![
                view! { <span class="id-cell">{tender.reference_number.clone()}</span> },
                view! { <span>{tender.title.clone()}</span> },
                type_tag,
                view! { <span class="amount-cell">{format_currency(tender.estimated_value)}</span> },
                view! { <span>{closing}</span> },
                view! { <span class="text-center">{bid_count}</span> },
                status,
            ],
        }
    }).collect();

    let pag_state = pagination_state.get();

    view! {
        style {
            r#"
            .tender-list { display: flex; flex-direction: column; gap: var(--space-4); }
            .filter-group {
                display: flex;
                align-items: center;
                gap: 8px;
            }
            .filter-group label {
                font-size: 12px;
                font-weight: 500;
                color: var(--text-muted);
            }
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
            .filter-spacer {
                flex: 1;
            }
            .search-input {
                min-width: 200px;
            }
            .loading-overlay {
                display: flex;
                align-items: center;
                justify-content: center;
                padding: 40px;
                color: var(--text-muted);
            }
            .text-center { text-align: center; }
            "#
        }

        <div class="tender-list" data-testid="tender-list">
            {page_header(
                "Tenders".to_string(),
                Some("Manage procurement tenders and bids".to_string()),
                vec![
                    view! { <button class="btn btn-secondary">"Export"</button> },
                    view! { <a href="#/tenders/new" class="btn btn-primary" data-testid="create-tender-btn">"New Tender"</a> },
                ]
            )}

            // Filter bar
            {filter_bar(vec![
                view! {
                    <div class="filter-group">
                        <label>"Type"</label>
                        <select on:change={handle_filter_type} data-testid="tender-filter-type">
                            <option value="">"All Types"</option>
                            <option value="rfq">"RFQ"</option>
                            <option value="rfp">"RFP"</option>
                            <option value="rft">"RFT"</option>
                        </select>
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"Status"</label>
                        <select on:change={handle_filter_status} data-testid="tender-filter-status">
                            <option value="">"All Statuses"</option>
                            <option value="draft">"Draft"</option>
                            <option value="pending">"Pending Approval"</option>
                            <option value="approved">"Approved"</option>
                            <option value="published">"Published"</option>
                            <option value="open">"Open"</option>
                            <option value="closed">"Closed"</option>
                            <option value="evaluation">"Evaluation"</option>
                            <option value="awarded">"Awarded"</option>
                            <option value="cancelled">"Cancelled"</option>
                        </select>
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"From"</label>
                        <input type="date" on:change={handle_date_from} />
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"To"</label>
                        <input type="date" on:change={handle_date_to} />
                    </div>
                },
                view! { <div class="filter-spacer"></div> },
                view! {
                    <div class="filter-group">
                        <input
                            type="text"
                            class="search-input"
                            placeholder="Search tenders..."
                            on:input={handle_search}
                            data-testid="tender-filter-search"
                        />
                    </div>
                },
                view! {
                    <button class="btn btn-sm btn-secondary" on:click={handle_clear_filters} data-testid="tender-filter-clear">"Clear"</button>
                },
                view! {
                    <button class="btn btn-sm btn-primary" on:click={handle_apply_filters} data-testid="tender-filter-apply">"Apply"</button>
                },
            ])}

            // Data table
            {panel(
                format!("Tenders ({} total)", filtered_tenders.len()),
                vec![],
                vec![
                    if loading.get() {
                        view! { <div class="loading-overlay">"Loading tenders..."</div> }
                    } else if rows.is_empty() {
                        view! {
                            <div class="loading-overlay">
                                "No tenders found. Create a new tender to get started."
                            </div>
                        }
                    } else {
                        view! {
                            <div>
                                {data_table_with_testid(columns, rows, Some(handle_row_click), Some("tender-table".to_string()), Some("tender-row".to_string()))}
                                {pagination(pag_state.current_page, pag_state.total_pages.max(1), handle_page_change)}
                            </div>
                        }
                    }
                ]
            )}
        </div>
    }
}
