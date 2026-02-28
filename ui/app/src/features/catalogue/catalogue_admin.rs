//! Catalogue admin page - Manage catalogue items

use components::prelude::*;
use wasm_bindgen::JsCast;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, data_table, DataTableColumn, DataTableRow,
    status_badge, StatusType,
    bbbee_badge, BbbeeLevel,
    tag, TagType,
    pagination,
    kpi_card, KpiColor,
    notice_bar, NoticeType,
    empty_state,
    modal, ModalSize,
};
use crate::shared::forms::filter_bar;
use crate::util::format::{format_currency, format_number};
use super::store::CatalogueStore;
use super::types::{CatalogueItem, CatalogueItemStatus, CatalogueCategory};
use super::service;

/// Catalogue admin page
#[component]
pub fn catalogue_admin() -> View {
    let store = use_context::<CatalogueStore>();

    // Filter signals
    let search_query = signal(String::new());
    let status_filter = signal(String::new());
    let category_filter = signal(String::new());

    // Modal state
    let show_add_modal = signal(false);
    let show_edit_modal = signal(false);
    let show_delete_modal = signal(false);
    let selected_item_id = signal(String::new());

    // Form state for new/edit item
    let form_name = signal(String::new());
    let form_code = signal(String::new());
    let form_description = signal(String::new());
    let form_category = signal(String::new());
    let form_price = signal(String::new());
    let form_supplier = signal(String::new());

    // Load data on mount
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                service::load_catalogue(&store).await;
            });
        }
    });

    let categories = store.categories.clone();
    let kpis = store.kpis.clone();
    let pagination_state = store.pagination.clone();
    let loading = store.loading.clone();

    // Handle search
    let handle_search = Callback::new({
        let store = store.clone();
        let search_query = search_query.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let value = input.value();
            search_query.set(value.clone());
            store.set_search(if value.is_empty() { None } else { Some(value) });
        }
    });

    // Handle status filter
    let handle_status_change = Callback::new({
        let store = store.clone();
        let status_filter = status_filter.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            let value = select.value();
            status_filter.set(value.clone());
            store.set_status(if value.is_empty() { None } else { Some(CatalogueItemStatus::from_str(&value)) });
        }
    });

    // Handle category filter
    let handle_category_change = Callback::new({
        let store = store.clone();
        let category_filter = category_filter.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            let value = select.value();
            category_filter.set(value.clone());
            store.set_category(if value.is_empty() { None } else { Some(value) });
        }
    });

    // Clear filters
    let handle_clear_filters = Callback::<()>::new({
        let store = store.clone();
        let search_query = search_query.clone();
        let status_filter = status_filter.clone();
        let category_filter = category_filter.clone();
        move |_| {
            search_query.set(String::new());
            status_filter.set(String::new());
            category_filter.set(String::new());
            store.clear_filters();
        }
    });

    // Handle row click - open edit modal
    let handle_row_click = Callback::new({
        let selected_item_id = selected_item_id.clone();
        let show_edit_modal = show_edit_modal.clone();
        let store = store.clone();
        let form_name = form_name.clone();
        let form_code = form_code.clone();
        let form_description = form_description.clone();
        let form_category = form_category.clone();
        let form_price = form_price.clone();
        let form_supplier = form_supplier.clone();
        move |item_id: String| {
            // Find and populate form
            if let Some(item) = store.items.get().iter().find(|i| i.id == item_id) {
                form_name.set(item.name.clone());
                form_code.set(item.item_code.clone());
                form_description.set(item.description.clone());
                form_category.set(item.category_id.clone());
                form_price.set(item.unit_price.to_string());
                form_supplier.set(item.supplier.name.clone());
            }
            selected_item_id.set(item_id);
            show_edit_modal.set(true);
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

    // Open add modal
    let handle_add_click = Callback::<()>::new({
        let show_add_modal = show_add_modal.clone();
        let form_name = form_name.clone();
        let form_code = form_code.clone();
        let form_description = form_description.clone();
        let form_category = form_category.clone();
        let form_price = form_price.clone();
        let form_supplier = form_supplier.clone();
        move |_| {
            // Clear form
            form_name.set(String::new());
            form_code.set(String::new());
            form_description.set(String::new());
            form_category.set(String::new());
            form_price.set(String::new());
            form_supplier.set(String::new());
            show_add_modal.set(true);
        }
    });

    // Close modals
    let handle_close_add = Callback::<()>::new({
        let show_add_modal = show_add_modal.clone();
        move |_| {
            show_add_modal.set(false);
        }
    });

    let handle_close_edit = Callback::<()>::new({
        let show_edit_modal = show_edit_modal.clone();
        move |_| {
            show_edit_modal.set(false);
        }
    });

    let handle_close_delete = Callback::<()>::new({
        let show_delete_modal = show_delete_modal.clone();
        move |_| {
            show_delete_modal.set(false);
        }
    });

    // Handle activate item
    let handle_activate = Callback::<()>::new({
        let store = store.clone();
        let selected_item_id = selected_item_id.clone();
        let show_edit_modal = show_edit_modal.clone();
        move |_| {
            let store = store.clone();
            let item_id = selected_item_id.get();
            let show_edit_modal = show_edit_modal.clone();
            spawn(async move {
                let _ = service::activate_item(&store, &item_id).await;
                show_edit_modal.set(false);
            });
        }
    });

    // Handle deactivate item
    let handle_deactivate = Callback::<()>::new({
        let store = store.clone();
        let selected_item_id = selected_item_id.clone();
        let show_edit_modal = show_edit_modal.clone();
        move |_| {
            let store = store.clone();
            let item_id = selected_item_id.get();
            let show_edit_modal = show_edit_modal.clone();
            spawn(async move {
                let _ = service::deactivate_item(&store, &item_id).await;
                show_edit_modal.set(false);
            });
        }
    });

    // Handle delete item
    let handle_delete = Callback::<()>::new({
        let show_edit_modal = show_edit_modal.clone();
        let show_delete_modal = show_delete_modal.clone();
        move |_| {
            show_edit_modal.set(false);
            show_delete_modal.set(true);
        }
    });

    let handle_confirm_delete = Callback::<()>::new({
        let store = store.clone();
        let selected_item_id = selected_item_id.clone();
        let show_delete_modal = show_delete_modal.clone();
        move |_| {
            let store = store.clone();
            let item_id = selected_item_id.get();
            let show_delete_modal = show_delete_modal.clone();
            spawn(async move {
                let _ = service::delete_item(&store, &item_id).await;
                show_delete_modal.set(false);
            });
        }
    });

    // Handle approve item
    let handle_approve = Callback::<()>::new({
        let store = store.clone();
        let selected_item_id = selected_item_id.clone();
        let show_edit_modal = show_edit_modal.clone();
        move |_| {
            let store = store.clone();
            let item_id = selected_item_id.get();
            let show_edit_modal = show_edit_modal.clone();
            spawn(async move {
                let _ = service::approve_item(&store, &item_id).await;
                show_edit_modal.set(false);
            });
        }
    });

    // Get filtered items
    let filtered_items = store.get_filtered_items();
    let pag_state = pagination_state.get();
    let kpis_data = kpis.get();
    let cats = categories.get();

    // Count pending items
    let pending_count = store.get_count_by_status(CatalogueItemStatus::PendingApproval);

    // Table columns
    let columns = vec![
        DataTableColumn { key: "code".to_string(), label: "Item Code".to_string(), width: Some("110px".to_string()), align: None, cell_class: Some("id-cell".to_string()) },
        DataTableColumn { key: "name".to_string(), label: "Item Name".to_string(), width: None, align: None, cell_class: None },
        DataTableColumn { key: "category".to_string(), label: "Category".to_string(), width: Some("140px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "supplier".to_string(), label: "Supplier".to_string(), width: Some("180px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "price".to_string(), label: "Unit Price".to_string(), width: Some("120px".to_string()), align: Some("right".to_string()), cell_class: Some("amount-cell".to_string()) },
        DataTableColumn { key: "stock".to_string(), label: "Stock".to_string(), width: Some("100px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "status".to_string(), label: "Status".to_string(), width: Some("110px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "actions".to_string(), label: "".to_string(), width: Some("80px".to_string()), align: Some("right".to_string()), cell_class: None },
    ];

    // Transform items to table rows
    let rows: Vec<DataTableRow> = filtered_items.iter().map(|item| {
        let status_view = get_status_badge(&item.status);
        let bbbee_view = get_bbbee_badge(item.supplier.bbbee_level);
        let stock_view = if item.in_stock {
            tag("In Stock".to_string(), TagType::Green)
        } else {
            tag("Out".to_string(), TagType::Red)
        };

        let item_id = item.id.clone();

        DataTableRow {
            id: item.id.clone(),
            cells: vec![
                view! {
                    <span class="item-code">{item.item_code.clone()}</span>
                },
                view! {
                    <div class="item-info">
                        <span class="item-name">{item.name.clone()}</span>
                        if item.featured {
                            <span class="featured-tag">"Featured"</span>
                        }
                    </div>
                },
                view! {
                    <span class="category-cell">{item.category_name.clone()}</span>
                },
                view! {
                    <div class="supplier-cell">
                        <span class="supplier-name">{item.supplier.name.clone()}</span>
                        {bbbee_view}
                    </div>
                },
                view! {
                    <span class="price-cell">{format_currency(item.unit_price)}</span>
                },
                stock_view,
                status_view,
                view! {
                    <div class="row-actions">
                        <button class="btn btn-sm btn-secondary">"Edit"</button>
                    </div>
                },
            ],
        }
    }).collect();

    // Icons
    let icon_package = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M16.5 9.4l-9-5.19"/><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/><polyline points="3.27 6.96 12 12.01 20.73 6.96"/><line x1="12" y1="22.08" x2="12" y2="12"/></svg>"#;
    let icon_check = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>"#;
    let icon_clock = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>"#;
    let icon_alert = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>"#;

    view! {
        style {
            r#"
            .catalogue-admin { display: flex; flex-direction: column; gap: var(--space-4); }
            .kpi-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; margin-bottom: 8px; }
            @media (max-width: 1200px) { .kpi-grid { grid-template-columns: repeat(2, 1fr); } }
            @media (max-width: 768px) { .kpi-grid { grid-template-columns: 1fr; } }

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
            .filter-group input:focus { outline: none; border-color: var(--blue); }
            .filter-spacer { flex: 1; }
            .search-input { min-width: 220px; }

            .item-code {
                font-family: IBM Plex Mono, monospace;
                font-size: 12px;
                color: var(--blue);
                font-weight: 500;
            }
            .item-info { display: flex; align-items: center; gap: 8px; }
            .item-name { font-weight: 500; color: var(--text); }
            .featured-tag {
                font-size: 10px;
                background: var(--orange-bg);
                color: var(--orange);
                padding: 2px 6px;
                border-radius: 4px;
                font-weight: 500;
            }
            .category-cell { font-size: 13px; color: var(--text-muted); }
            .supplier-cell { display: flex; flex-direction: column; gap: 4px; }
            .supplier-name { font-size: 13px; }
            .price-cell {
                font-family: IBM Plex Mono, monospace;
                font-size: 12px;
                font-weight: 600;
            }
            .row-actions { display: flex; gap: 8px; justify-content: flex-end; }

            .loading-state {
                display: flex;
                align-items: center;
                justify-content: center;
                padding: 60px;
                color: var(--text-muted);
            }

            .modal-form { display: flex; flex-direction: column; gap: 16px; }
            .form-row { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; }
            @media (max-width: 600px) { .form-row { grid-template-columns: 1fr; } }
            .form-field { display: flex; flex-direction: column; gap: 6px; }
            .form-field label {
                font-size: 12px;
                font-weight: 500;
                color: var(--text-muted);
            }
            .form-field input,
            .form-field select,
            .form-field textarea {
                padding: 10px 12px;
                border: 1px solid var(--border);
                border-radius: var(--radius-sm);
                font-size: 14px;
                background: var(--surface);
            }
            .form-field input:focus,
            .form-field select:focus,
            .form-field textarea:focus {
                outline: none;
                border-color: var(--blue);
            }
            .form-field textarea { min-height: 80px; resize: vertical; }
            .form-actions {
                display: flex;
                justify-content: flex-end;
                gap: 12px;
                padding-top: 16px;
                border-top: 1px solid var(--border);
            }

            .item-detail { padding: 16px 0; }
            .detail-row {
                display: flex;
                justify-content: space-between;
                padding: 8px 0;
                border-bottom: 1px solid var(--border);
            }
            .detail-row:last-child { border-bottom: none; }
            .detail-label { color: var(--text-muted); font-size: 13px; }
            .detail-value { font-weight: 500; }

            .status-actions {
                display: flex;
                gap: 8px;
                padding: 16px 0;
                border-bottom: 1px solid var(--border);
                margin-bottom: 16px;
            }

            .delete-confirm {
                text-align: center;
                padding: 24px 0;
            }
            .delete-confirm p { margin-bottom: 24px; color: var(--text-muted); }
            .delete-actions { display: flex; justify-content: center; gap: 12px; }
            "#
        }

        <div class="catalogue-admin" data-testid="catalogue-admin">
            {page_header(
                "Catalogue Management".to_string(),
                Some("Add, edit, and manage catalogue items".to_string()),
                vec![
                    view! { <button class="btn btn-secondary">"Import CSV"</button> },
                    view! { <button class="btn btn-secondary">"Export"</button> },
                    view! { <button class="btn btn-primary" on:click={handle_add_click.clone()}>"Add Item"</button> },
                ]
            )}

            // Pending approval notice
            if pending_count > 0 {
                {notice_bar(
                    format!("{} item(s) pending approval. Review and approve to make them available.", pending_count),
                    NoticeType::Warning,
                    None
                )}
            }

            // KPI summary
            <div class="kpi-grid">
                {kpi_card(
                    "Total Items".to_string(),
                    format_number(kpis_data.total_items),
                    KpiColor::Blue,
                    icon_package.to_string(),
                    None,
                    None
                )}
                {kpi_card(
                    "Active Items".to_string(),
                    format_number(kpis_data.active_items),
                    KpiColor::Green,
                    icon_check.to_string(),
                    None,
                    None
                )}
                {kpi_card(
                    "Pending Approval".to_string(),
                    kpis_data.pending_approval.to_string(),
                    KpiColor::Orange,
                    icon_clock.to_string(),
                    None,
                    None
                )}
                {kpi_card(
                    "Out of Stock".to_string(),
                    kpis_data.out_of_stock.to_string(),
                    KpiColor::Red,
                    icon_alert.to_string(),
                    None,
                    None
                )}
            </div>

            // Filters
            {filter_bar(vec![
                view! {
                    <div class="filter-group">
                        <label>"Status:"</label>
                        <select on:change={handle_status_change} value={status_filter.get()}>
                            <option value="">"All Statuses"</option>
                            <option value="active">"Active"</option>
                            <option value="inactive">"Inactive"</option>
                            <option value="discontinued">"Discontinued"</option>
                            <option value="pending_approval">"Pending Approval"</option>
                        </select>
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"Category:"</label>
                        <select on:change={handle_category_change} value={category_filter.get()}>
                            <option value="">"All Categories"</option>
                            for cat in cats.iter() {
                                <option value={cat.id.clone()}>{cat.name.clone()}</option>
                            }
                        </select>
                    </div>
                },
                view! { <div class="filter-spacer"></div> },
                view! {
                    <div class="filter-group">
                        <input
                            type="text"
                            class="search-input"
                            placeholder="Search items..."
                            value={search_query.get()}
                            on:input={handle_search}
                        />
                    </div>
                },
                view! {
                    <button class="btn btn-sm btn-secondary" on:click={handle_clear_filters}>"Clear"</button>
                },
            ])}

            // Items table
            {panel(
                format!("Catalogue Items ({})", filtered_items.len()),
                vec![],
                vec![
                    if loading.get() {
                        view! { <div class="loading-state">"Loading items..."</div> }
                    } else if rows.is_empty() {
                        {empty_state(
                            "No items found".to_string(),
                            Some("Try adjusting your filters or add a new item".to_string()),
                            None,
                            Some(view! { <button class="btn btn-primary" on:click={handle_add_click.clone()}>"Add Item"</button> })
                        )}
                    } else {
                        view! {
                            <div>
                                {data_table(columns, rows, Some(handle_row_click))}
                                {pagination(pag_state.current_page, pag_state.total_pages.max(1), handle_page_change)}
                            </div>
                        }
                    }
                ]
            )}
        </div>

        // Add Item Modal
        if show_add_modal.get() {
            {modal(
                "Add Catalogue Item".to_string(),
                ModalSize::Medium,
                show_add_modal.clone(),
                handle_close_add.clone(),
                vec![view! {
                    <div class="modal-form">
                        <div class="form-row">
                            <div class="form-field">
                                <label>"Item Code"</label>
                                <input
                                    type="text"
                                    placeholder="e.g., IT-LAP-001"
                                    value={form_code.get()}
                                    on:input={Callback::new({
                                        let form_code = form_code.clone();
                                        move |e: web_sys::Event| {
                                            let target = e.target().unwrap();
                                            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
                                            form_code.set(input.value());
                                        }
                                    })}
                                />
                            </div>
                            <div class="form-field">
                                <label>"Category"</label>
                                <select
                                    value={form_category.get()}
                                    on:change={Callback::new({
                                        let form_category = form_category.clone();
                                        move |e: web_sys::Event| {
                                            let target = e.target().unwrap();
                                            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
                                            form_category.set(select.value());
                                        }
                                    })}
                                >
                                    <option value="">"Select category..."</option>
                                    for cat in cats.iter() {
                                        <option value={cat.id.clone()}>{cat.name.clone()}</option>
                                    }
                                </select>
                            </div>
                        </div>
                        <div class="form-field">
                            <label>"Item Name"</label>
                            <input
                                type="text"
                                placeholder="Enter item name"
                                value={form_name.get()}
                                on:input={Callback::new({
                                    let form_name = form_name.clone();
                                    move |e: web_sys::Event| {
                                        let target = e.target().unwrap();
                                        let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
                                        form_name.set(input.value());
                                    }
                                })}
                            />
                        </div>
                        <div class="form-field">
                            <label>"Description"</label>
                            <textarea
                                placeholder="Enter item description"
                                value={form_description.get()}
                                on:input={Callback::new({
                                    let form_description = form_description.clone();
                                    move |e: web_sys::Event| {
                                        let target = e.target().unwrap();
                                        let input: web_sys::HtmlTextAreaElement = target.dyn_into().unwrap();
                                        form_description.set(input.value());
                                    }
                                })}
                            ></textarea>
                        </div>
                        <div class="form-row">
                            <div class="form-field">
                                <label>"Unit Price (ZAR)"</label>
                                <input
                                    type="number"
                                    step="0.01"
                                    placeholder="0.00"
                                    value={form_price.get()}
                                    on:input={Callback::new({
                                        let form_price = form_price.clone();
                                        move |e: web_sys::Event| {
                                            let target = e.target().unwrap();
                                            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
                                            form_price.set(input.value());
                                        }
                                    })}
                                />
                            </div>
                            <div class="form-field">
                                <label>"Supplier"</label>
                                <select
                                    value={form_supplier.get()}
                                    on:change={Callback::new({
                                        let form_supplier = form_supplier.clone();
                                        move |e: web_sys::Event| {
                                            let target = e.target().unwrap();
                                            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
                                            form_supplier.set(select.value());
                                        }
                                    })}
                                >
                                    <option value="">"Select supplier..."</option>
                                    <option value="SUP-001">"TechSolutions SA (Pty) Ltd"</option>
                                    <option value="SUP-002">"Office Essentials SA"</option>
                                    <option value="SUP-004">"SecureGuard Holdings"</option>
                                    <option value="SUP-009">"Limpopo Cleaning Services"</option>
                                    <option value="SUP-010">"Free State Furniture Factory"</option>
                                </select>
                            </div>
                        </div>
                    </div>
                }],
                vec![
                    view! { <button class="btn btn-secondary" on:click={handle_close_add}>"Cancel"</button> },
                    view! { <button class="btn btn-primary">"Add Item"</button> },
                ]
            )}
        }

        // Edit Item Modal
        if show_edit_modal.get() {
            {modal(
                "Edit Catalogue Item".to_string(),
                ModalSize::Medium,
                show_edit_modal.clone(),
                handle_close_edit.clone(),
                vec![view! {
                    <div>
                        // Status actions
                        <div class="status-actions">
                            <button class="btn btn-sm btn-success" on:click={handle_approve}>"Approve"</button>
                            <button class="btn btn-sm btn-secondary" on:click={handle_activate}>"Activate"</button>
                            <button class="btn btn-sm btn-warning" on:click={handle_deactivate}>"Deactivate"</button>
                            <button class="btn btn-sm btn-danger" on:click={handle_delete}>"Delete"</button>
                        </div>

                        <div class="modal-form">
                            <div class="form-row">
                                <div class="form-field">
                                    <label>"Item Code"</label>
                                    <input type="text" value={form_code.get()} disabled={true} />
                                </div>
                                <div class="form-field">
                                    <label>"Category"</label>
                                    <select value={form_category.get()}>
                                        for cat in cats.iter() {
                                            <option value={cat.id.clone()}>{cat.name.clone()}</option>
                                        }
                                    </select>
                                </div>
                            </div>
                            <div class="form-field">
                                <label>"Item Name"</label>
                                <input type="text" value={form_name.get()} />
                            </div>
                            <div class="form-field">
                                <label>"Description"</label>
                                <textarea value={form_description.get()}></textarea>
                            </div>
                            <div class="form-row">
                                <div class="form-field">
                                    <label>"Unit Price (ZAR)"</label>
                                    <input type="number" step="0.01" value={form_price.get()} />
                                </div>
                                <div class="form-field">
                                    <label>"Supplier"</label>
                                    <input type="text" value={form_supplier.get()} disabled={true} />
                                </div>
                            </div>
                        </div>
                    </div>
                }],
                vec![
                    view! { <button class="btn btn-secondary" on:click={handle_close_edit}>"Cancel"</button> },
                    view! { <button class="btn btn-primary">"Save Changes"</button> },
                ]
            )}
        }

        // Delete Confirmation Modal
        if show_delete_modal.get() {
            {modal(
                "Delete Item".to_string(),
                ModalSize::Small,
                show_delete_modal.clone(),
                handle_close_delete.clone(),
                vec![view! {
                    <div class="delete-confirm">
                        <p>"Are you sure you want to delete this catalogue item? This action cannot be undone."</p>
                    </div>
                }],
                vec![
                    view! { <button class="btn btn-secondary" on:click={handle_close_delete}>"Cancel"</button> },
                    view! { <button class="btn btn-danger" on:click={handle_confirm_delete}>"Delete Item"</button> },
                ]
            )}
        }
    }
}

/// Get status badge view
fn get_status_badge(status: &CatalogueItemStatus) -> View {
    let status_type = match status {
        CatalogueItemStatus::Active => StatusType::Active,
        CatalogueItemStatus::Inactive => StatusType::Draft,
        CatalogueItemStatus::Discontinued => StatusType::Cancelled,
        CatalogueItemStatus::PendingApproval => StatusType::Pending,
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
