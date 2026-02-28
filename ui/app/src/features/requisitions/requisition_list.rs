//! Requisition list view

use components::prelude::*;
use wasm_bindgen::JsCast;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, data_table, DataTableColumn, DataTableRow,
    status_badge, StatusType,
    pagination,
};
use crate::shared::forms::filter_bar;
use crate::util::format::{format_currency, format_date};
use super::store::{RequisitionsStore, get_paginated_requisitions, get_filtered_requisitions};
use super::types::{RequisitionStatus, RequisitionFilter};
use super::service;

/// Requisition list page
#[component]
pub fn requisition_list() -> View {
    let store = use_context::<RequisitionsStore>();

    // Load data on mount
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                service::load_requisitions(&store).await;
            });
        }
    });

    // Filter signals
    let search = signal(String::new());
    let status_filter = signal(String::new());
    let department_filter = signal(String::new());

    // Update store filter when inputs change
    let update_filter = {
        let store = store.clone();
        let search = search.clone();
        let status_filter = status_filter.clone();
        let department_filter = department_filter.clone();
        Callback::new(move |_: ()| {
            let mut filter = RequisitionFilter::default();
            filter.search = search.get();
            filter.status = if status_filter.get().is_empty() {
                None
            } else {
                Some(RequisitionStatus::from_str(&status_filter.get()))
            };
            filter.department = if department_filter.get().is_empty() {
                None
            } else {
                Some(department_filter.get())
            };
            store.filter.set(filter);

            // Update pagination totals
            let mut pagination = store.pagination.get();
            pagination.update_totals(get_filtered_requisitions(&store).len() as u32);
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
            web_sys::console::log_1(&format!("Selected requisition: {}", id).into());
        })
    };

    // Table columns
    let columns = vec![
        DataTableColumn { key: "id".to_string(), label: "Req ID".to_string(), width: Some("120px".to_string()), align: None, cell_class: Some("id-cell".to_string()) },
        DataTableColumn { key: "description".to_string(), label: "Description".to_string(), width: None, align: None, cell_class: None },
        DataTableColumn { key: "department".to_string(), label: "Department".to_string(), width: Some("150px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "requester".to_string(), label: "Requester".to_string(), width: Some("140px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "amount".to_string(), label: "Amount".to_string(), width: Some("130px".to_string()), align: Some("right".to_string()), cell_class: Some("amount-cell".to_string()) },
        DataTableColumn { key: "date".to_string(), label: "Date".to_string(), width: Some("100px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "status".to_string(), label: "Status".to_string(), width: Some("130px".to_string()), align: None, cell_class: None },
    ];

    // Transform requisitions to table rows
    let requisitions = get_paginated_requisitions(&store);
    let rows: Vec<DataTableRow> = requisitions.iter().map(|req| {
        let status = match req.status {
            RequisitionStatus::Draft => status_badge(StatusType::Draft),
            RequisitionStatus::Submitted => status_badge(StatusType::Submitted),
            RequisitionStatus::PendingApproval => status_badge(StatusType::Pending),
            RequisitionStatus::Approved => status_badge(StatusType::Approved),
            RequisitionStatus::Rejected => status_badge(StatusType::Rejected),
            RequisitionStatus::Cancelled => status_badge(StatusType::Cancelled),
            RequisitionStatus::InProgress => status_badge(StatusType::InProgress),
            RequisitionStatus::Complete => status_badge(StatusType::Complete),
        };
        DataTableRow {
            id: req.id.clone(),
            cells: vec![
                view! { <span class="id-cell">{req.id.clone()}</span> },
                view! { <span class="description-cell">{req.description.clone()}</span> },
                view! { <span>{req.department.clone()}</span> },
                view! { <span>{req.requester.clone()}</span> },
                view! { <span class="amount-cell">{format_currency(req.amount)}</span> },
                view! { <span>{format_date(&req.created_at)}</span> },
                status,
            ],
        }
    }).collect();

    let pagination_state = store.pagination.get();
    let total_filtered = get_filtered_requisitions(&store).len();

    view! {
        style {
            r#"
            .requisition-list { display: flex; flex-direction: column; gap: var(--space-4); }
            .description-cell {
                max-width: 300px;
                overflow: hidden;
                text-overflow: ellipsis;
                white-space: nowrap;
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
            "#
        }

        <div class="requisition-list" data-testid="requisition-list">
            {page_header(
                "Requisitions".to_string(),
                Some("Manage purchase requisitions and approvals".to_string()),
                vec![
                    view! { <button class="btn btn-secondary">"Export"</button> },
                    view! { <a href="/requisitions/new" class="btn btn-primary">"+ New Requisition"</a> },
                ]
            )}

            // Filter bar
            {filter_bar(vec![
                view! {
                    <div class="filter-group">
                        <label>"Search"</label>
                        <input
                            type="text"
                            placeholder="Search by ID, description, requester..."
                            value={search.get()}
                            on:input={on_search}
                            style="width: 250px;"
                        />
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"Status"</label>
                        <select on:change={on_status_change}>
                            <option value="">"All Statuses"</option>
                            <option value="draft">"Draft"</option>
                            <option value="submitted">"Submitted"</option>
                            <option value="pending">"Pending Approval"</option>
                            <option value="approved">"Approved"</option>
                            <option value="rejected">"Rejected"</option>
                            <option value="in_progress">"In Progress"</option>
                            <option value="complete">"Complete"</option>
                        </select>
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"Department"</label>
                        <select on:change={on_department_change}>
                            <option value="">"All Departments"</option>
                            <option value="Information Technology">"Information Technology"</option>
                            <option value="Administration">"Administration"</option>
                            <option value="Facilities">"Facilities"</option>
                            <option value="Transport">"Transport"</option>
                            <option value="Human Resources">"Human Resources"</option>
                            <option value="Health Services">"Health Services"</option>
                            <option value="Finance">"Finance"</option>
                        </select>
                    </div>
                },
                view! { <div class="filter-spacer"></div> },
                view! {
                    <button class="btn btn-sm btn-secondary">"Clear Filters"</button>
                },
            ])}

            // Requisitions table
            {panel(
                "All Requisitions".to_string(),
                vec![],
                vec![
                    view! {
                        <div class="list-info">
                            <span>{format!("Showing {} of {} requisitions", rows.len(), total_filtered)}</span>
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
