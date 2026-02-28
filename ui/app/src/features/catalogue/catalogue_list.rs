//! Catalogue list view - Browse catalogue with categories, search, and item cards

use components::prelude::*;
use wasm_bindgen::JsCast;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, status_badge, StatusType,
    bbbee_badge, BbbeeLevel,
    tag, TagType,
    pagination,
    kpi_card, KpiColor,
    empty_state,
};
use crate::shared::forms::filter_bar;
use crate::util::format::{format_currency, format_number};
use super::store::CatalogueStore;
use super::types::{CatalogueItemStatus, CatalogueCategory, CatalogueSortBy};
use super::service;

/// Catalogue browse page
#[component]
pub fn catalogue_list() -> View {
    let store = use_context::<CatalogueStore>();

    // Filter signals
    let search_query = signal(String::new());
    let selected_category = signal(String::new());
    let sort_by = signal("name_asc".to_string());
    let in_stock_only = signal(false);
    let selected_bbbee = signal(String::new());

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
    let view_mode = store.view_mode.clone();

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

    // Handle category filter
    let handle_category_change = Callback::new({
        let store = store.clone();
        let selected_category = selected_category.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            let value = select.value();
            selected_category.set(value.clone());
            store.set_category(if value.is_empty() { None } else { Some(value) });
        }
    });

    // Handle sort change
    let handle_sort_change = Callback::new({
        let store = store.clone();
        let sort_by = sort_by.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            let value = select.value();
            sort_by.set(value.clone());
            store.sort_by.set(CatalogueSortBy::from_str(&value));
        }
    });

    // Handle stock filter
    let handle_stock_toggle = Callback::new({
        let store = store.clone();
        let in_stock_only = in_stock_only.clone();
        move |_| {
            let new_value = !in_stock_only.get();
            in_stock_only.set(new_value);
            store.set_in_stock_only(new_value);
        }
    });

    // Handle B-BBEE filter
    let handle_bbbee_change = Callback::new({
        let store = store.clone();
        let selected_bbbee = selected_bbbee.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            let value = select.value();
            selected_bbbee.set(value.clone());
            let mut filter = store.filter.get();
            filter.bbbee_level = if value.is_empty() { None } else { value.parse().ok() };
            store.filter.set(filter);
        }
    });

    // Toggle view mode
    let handle_toggle_view = Callback::new({
        let view_mode = view_mode.clone();
        move |mode: String| {
            view_mode.set(mode);
        }
    });

    // Clear filters
    let handle_clear_filters = Callback::new({
        let store = store.clone();
        let search_query = search_query.clone();
        let selected_category = selected_category.clone();
        let sort_by = sort_by.clone();
        let in_stock_only = in_stock_only.clone();
        let selected_bbbee = selected_bbbee.clone();
        move |_| {
            search_query.set(String::new());
            selected_category.set(String::new());
            sort_by.set("name_asc".to_string());
            in_stock_only.set(false);
            selected_bbbee.set(String::new());
            store.clear_filters();
            store.sort_by.set(CatalogueSortBy::default());
        }
    });

    // Handle item click
    let handle_item_click = Callback::new({
        move |item_id: String| {
            web_sys::window()
                .unwrap()
                .location()
                .set_href(&format!("#/catalogue/{}", item_id))
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

    // Get filtered items
    let filtered_items = store.get_filtered_items();
    let pag_state = pagination_state.get();
    let kpis_data = kpis.get();
    let cats = categories.get();

    // Icons
    let icon_package = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M16.5 9.4l-9-5.19"/><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/><polyline points="3.27 6.96 12 12.01 20.73 6.96"/><line x1="12" y1="22.08" x2="12" y2="12"/></svg>"#;
    let icon_layers = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 17 12 22 22 17"/><polyline points="2 12 12 17 22 12"/></svg>"#;
    let icon_users = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>"#;
    let icon_alert = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>"#;

    view! {
        style {
            r#"
            .catalogue-page { display: flex; flex-direction: column; gap: var(--space-4); }
            .kpi-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; margin-bottom: 8px; }
            @media (max-width: 1200px) { .kpi-grid { grid-template-columns: repeat(2, 1fr); } }
            @media (max-width: 768px) { .kpi-grid { grid-template-columns: 1fr; } }

            .catalogue-layout { display: grid; grid-template-columns: 240px 1fr; gap: 24px; }
            @media (max-width: 992px) { .catalogue-layout { grid-template-columns: 1fr; } }

            .categories-sidebar {
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius-lg);
                padding: 16px;
                height: fit-content;
                position: sticky;
                top: 16px;
            }
            .categories-title {
                font-size: 14px;
                font-weight: 600;
                color: var(--text);
                margin-bottom: 12px;
                padding-bottom: 8px;
                border-bottom: 1px solid var(--border);
            }
            .category-list { display: flex; flex-direction: column; gap: 4px; }
            .category-item {
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding: 8px 12px;
                border-radius: var(--radius-sm);
                cursor: pointer;
                font-size: 13px;
                color: var(--text-muted);
                transition: all 0.15s;
            }
            .category-item:hover { background: var(--bg); color: var(--text); }
            .category-item.active { background: var(--blue-bg); color: var(--blue); font-weight: 500; }
            .category-count {
                font-size: 11px;
                background: var(--bg);
                padding: 2px 6px;
                border-radius: 10px;
            }
            .category-item.active .category-count { background: var(--blue); color: white; }

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
            .search-input { min-width: 220px; }
            .filter-checkbox {
                display: flex;
                align-items: center;
                gap: 6px;
                font-size: 12px;
                cursor: pointer;
            }

            .view-toggle {
                display: flex;
                gap: 4px;
                background: var(--bg);
                padding: 4px;
                border-radius: var(--radius-sm);
            }
            .view-toggle button {
                padding: 4px 12px;
                border: none;
                background: transparent;
                border-radius: var(--radius-sm);
                cursor: pointer;
                font-size: 12px;
                color: var(--text-muted);
            }
            .view-toggle button.active {
                background: var(--surface);
                color: var(--text);
                box-shadow: var(--shadow-sm);
            }

            .item-grid {
                display: grid;
                grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
                gap: 16px;
            }
            .item-card {
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius-lg);
                overflow: hidden;
                cursor: pointer;
                transition: transform 0.2s, box-shadow 0.2s;
            }
            .item-card:hover {
                transform: translateY(-2px);
                box-shadow: var(--shadow);
            }
            .item-card.out-of-stock { opacity: 0.7; }
            .item-image {
                height: 160px;
                background: var(--bg);
                display: flex;
                align-items: center;
                justify-content: center;
                color: var(--text-muted);
                font-size: 48px;
                position: relative;
            }
            .item-image img {
                width: 100%;
                height: 100%;
                object-fit: cover;
            }
            .featured-badge {
                position: absolute;
                top: 8px;
                left: 8px;
                background: var(--orange);
                color: white;
                font-size: 10px;
                font-weight: 600;
                padding: 2px 8px;
                border-radius: 4px;
                text-transform: uppercase;
            }
            .out-of-stock-badge {
                position: absolute;
                top: 8px;
                right: 8px;
                background: var(--red);
                color: white;
                font-size: 10px;
                font-weight: 600;
                padding: 2px 8px;
                border-radius: 4px;
            }
            .item-content { padding: 16px; }
            .item-category {
                font-size: 11px;
                color: var(--text-muted);
                text-transform: uppercase;
                letter-spacing: 0.5px;
                margin-bottom: 4px;
            }
            .item-name {
                font-size: 14px;
                font-weight: 600;
                color: var(--text);
                margin-bottom: 8px;
                line-height: 1.3;
                max-height: 2.6em;
                overflow: hidden;
                text-overflow: ellipsis;
            }
            .item-code {
                font-size: 11px;
                font-family: IBM Plex Mono, monospace;
                color: var(--text-muted);
                margin-bottom: 12px;
            }
            .item-price {
                font-size: 18px;
                font-weight: 700;
                color: var(--text);
                margin-bottom: 8px;
            }
            .item-price .currency { font-size: 14px; font-weight: 500; }
            .item-price .vat { font-size: 11px; font-weight: 400; color: var(--text-muted); }
            .item-footer {
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding-top: 12px;
                border-top: 1px solid var(--border);
            }
            .item-supplier {
                display: flex;
                align-items: center;
                gap: 6px;
                font-size: 12px;
                color: var(--text-muted);
            }

            .item-list {
                display: flex;
                flex-direction: column;
                gap: 8px;
            }
            .item-row {
                display: grid;
                grid-template-columns: 80px 1fr 120px 140px 100px 100px;
                align-items: center;
                gap: 16px;
                padding: 12px 16px;
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius-md);
                cursor: pointer;
                transition: background 0.15s;
            }
            .item-row:hover { background: var(--bg); }
            .item-row-image {
                width: 60px;
                height: 60px;
                background: var(--bg);
                border-radius: var(--radius-sm);
                display: flex;
                align-items: center;
                justify-content: center;
                color: var(--text-muted);
                font-size: 24px;
            }
            .item-row-info { min-width: 0; }
            .item-row-name {
                font-weight: 500;
                color: var(--text);
                margin-bottom: 4px;
                white-space: nowrap;
                overflow: hidden;
                text-overflow: ellipsis;
            }
            .item-row-category { font-size: 12px; color: var(--text-muted); }
            .item-row-code { font-family: IBM Plex Mono, monospace; font-size: 11px; color: var(--text-muted); }
            .item-row-price {
                font-weight: 600;
                font-family: IBM Plex Mono, monospace;
                text-align: right;
            }
            .item-row-supplier { font-size: 12px; color: var(--text-muted); }
            .item-row-stock { text-align: center; }

            .loading-state {
                display: flex;
                align-items: center;
                justify-content: center;
                padding: 60px;
                color: var(--text-muted);
            }
            "#
        }

        <div class="catalogue-page" data-testid="catalogue-list">
            {page_header(
                "Catalogue".to_string(),
                Some("Browse and search available items".to_string()),
                vec![
                    view! { <button class="btn btn-secondary">"Export"</button> },
                ]
            )}

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
                    "Categories".to_string(),
                    kpis_data.categories_count.to_string(),
                    KpiColor::Accent,
                    icon_layers.to_string(),
                    None,
                    None
                )}
                {kpi_card(
                    "Suppliers".to_string(),
                    kpis_data.suppliers_count.to_string(),
                    KpiColor::Green,
                    icon_users.to_string(),
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

            // Filter bar
            {filter_bar(vec![
                view! {
                    <div class="filter-group">
                        <label>"Category"</label>
                        <select on:change={handle_category_change}>
                            <option value="">"All Categories"</option>
                            for cat in cats.iter() {
                                <option value={cat.id.clone()}>{cat.name.clone()}</option>
                            }
                        </select>
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"B-BBEE Level"</label>
                        <select on:change={handle_bbbee_change}>
                            <option value="">"All Levels"</option>
                            <option value="1">"Level 1"</option>
                            <option value="2">"Level 1-2"</option>
                            <option value="3">"Level 1-3"</option>
                            <option value="4">"Level 1-4"</option>
                        </select>
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"Sort By"</label>
                        <select on:change={handle_sort_change}>
                            <option value="name_asc">"Name (A-Z)"</option>
                            <option value="name_desc">"Name (Z-A)"</option>
                            <option value="price_asc">"Price (Low to High)"</option>
                            <option value="price_desc">"Price (High to Low)"</option>
                            <option value="category">"Category"</option>
                            <option value="recent">"Recently Added"</option>
                        </select>
                    </div>
                },
                view! {
                    <label class="filter-checkbox">
                        <input
                            type="checkbox"
                            checked={in_stock_only.get()}
                            on:change={handle_stock_toggle}
                        />
                        "In stock only"
                    </label>
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
                    <div class="view-toggle">
                        <button
                            class={if view_mode.get() == "grid" { "active" } else { "" }}
                            on:click={Callback::new({
                                let handle_toggle_view = handle_toggle_view.clone();
                                move |_| handle_toggle_view.call("grid".to_string())
                            })}
                        >"Grid"</button>
                        <button
                            class={if view_mode.get() == "list" { "active" } else { "" }}
                            on:click={Callback::new({
                                let handle_toggle_view = handle_toggle_view.clone();
                                move |_| handle_toggle_view.call("list".to_string())
                            })}
                        >"List"</button>
                    </div>
                },
                view! {
                    <button class="btn btn-sm btn-secondary" on:click={handle_clear_filters}>"Clear"</button>
                },
            ])}

            // Main content
            {panel(
                format!("Items ({} found)", filtered_items.len()),
                vec![],
                vec![
                    if loading.get() {
                        view! { <div class="loading-state">"Loading catalogue..."</div> }
                    } else if filtered_items.is_empty() {
                        {empty_state(
                            "No items found".to_string(),
                            Some("Try adjusting your search or filters".to_string()),
                            None
                        )}
                    } else if view_mode.get() == "list" {
                        view! {
                            <div class="item-list">
                                for item in filtered_items.iter() {
                                    {item_row(item.clone(), handle_item_click.clone())}
                                }
                            </div>
                        }
                    } else {
                        view! {
                            <div class="item-grid">
                                for item in filtered_items.iter() {
                                    {item_card(item.clone(), handle_item_click.clone())}
                                }
                            </div>
                        }
                    },
                    view! {
                        <div style="margin-top: 16px;">
                            {pagination(pag_state.current_page, pag_state.total_pages.max(1), handle_page_change)}
                        </div>
                    }
                ]
            )}
        </div>
    }
}

/// Item card component for grid view
fn item_card(item: super::types::CatalogueItem, on_click: Callback<String>) -> View {
    let item_id = item.id.clone();
    let handle_click = Callback::unit({
        let on_click = on_click.clone();
        let item_id = item_id.clone();
        move || {
            on_click.call(item_id.clone());
        }
    });

    let bbbee_level = match item.supplier.bbbee_level {
        1 => BbbeeLevel::Level1,
        2 => BbbeeLevel::Level2,
        3 => BbbeeLevel::Level3,
        4 => BbbeeLevel::Level4,
        _ => BbbeeLevel::NonCompliant,
    };

    let card_class = if item.in_stock { "item-card" } else { "item-card out-of-stock" };

    view! {
        <div class={card_class} on:click={handle_click}>
            <div class="item-image">
                if item.featured {
                    <span class="featured-badge">"Featured"</span>
                }
                if !item.in_stock {
                    <span class="out-of-stock-badge">"Out of Stock"</span>
                }
                // Placeholder icon if no image
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="48" height="48">
                    <path d="M16.5 9.4l-9-5.19"/>
                    <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/>
                    <polyline points="3.27 6.96 12 12.01 20.73 6.96"/>
                    <line x1="12" y1="22.08" x2="12" y2="12"/>
                </svg>
            </div>
            <div class="item-content">
                <div class="item-category">{item.category_name.clone()}</div>
                <div class="item-name">{item.name.clone()}</div>
                <div class="item-code">{item.item_code.clone()}</div>
                <div class="item-price">
                    <span class="currency">"R"</span>
                    {format!("{:.2}", item.unit_price)}
                    <span class="vat">" incl. VAT"</span>
                </div>
                <div class="item-footer">
                    <div class="item-supplier">
                        {bbbee_badge(bbbee_level)}
                        <span>{item.supplier.name.clone()}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Item row component for list view
fn item_row(item: super::types::CatalogueItem, on_click: Callback<String>) -> View {
    let item_id = item.id.clone();
    let handle_click = Callback::unit({
        let on_click = on_click.clone();
        let item_id = item_id.clone();
        move || {
            on_click.call(item_id.clone());
        }
    });

    let bbbee_level = match item.supplier.bbbee_level {
        1 => BbbeeLevel::Level1,
        2 => BbbeeLevel::Level2,
        3 => BbbeeLevel::Level3,
        4 => BbbeeLevel::Level4,
        _ => BbbeeLevel::NonCompliant,
    };

    let stock_tag = if item.in_stock {
        tag("In Stock".to_string(), TagType::Green)
    } else {
        tag("Out of Stock".to_string(), TagType::Red)
    };

    view! {
        <div class="item-row" on:click={handle_click}>
            <div class="item-row-image">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="28" height="28">
                    <path d="M16.5 9.4l-9-5.19"/>
                    <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/>
                </svg>
            </div>
            <div class="item-row-info">
                <div class="item-row-name">{item.name.clone()}</div>
                <div class="item-row-category">{item.category_name.clone()}</div>
            </div>
            <div class="item-row-code">{item.item_code.clone()}</div>
            <div class="item-row-price">{format_currency(item.unit_price)}</div>
            <div class="item-row-supplier">
                {bbbee_badge(bbbee_level)}
            </div>
            <div class="item-row-stock">
                {stock_tag}
            </div>
        </div>
    }
}
