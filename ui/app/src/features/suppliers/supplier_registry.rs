//! Supplier registry list with search, filters, and supplier cards

use components::prelude::*;
use wasm_bindgen::JsCast;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, data_table, DataTableColumn, DataTableRow,
    status_badge, StatusType,
    bbbee_badge, BbbeeLevel as BadgeLevel,
    tag, TagType,
    pagination,
    kpi_card, KpiColor,
};
use crate::shared::forms::filter_bar;
use crate::util::format::{format_currency, format_number};
use super::store::SuppliersStore;
use super::types::{SupplierStatus, BbbeeLevel, RiskRating, SupplierFilter};
use super::service;

/// Supplier registry page
#[component]
pub fn supplier_registry() -> View {
    let store = use_context::<SuppliersStore>();

    // Filter signals
    let filter_status = signal(String::new());
    let filter_bbbee = signal(String::new());
    let filter_risk = signal(String::new());
    let filter_province = signal(String::new());
    let search_query = signal(String::new());
    let view_mode = signal("table".to_string()); // "table" or "cards"

    // Load data on mount
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                service::load_suppliers(&store).await;
            });
        }
    });

    let suppliers = store.suppliers.clone();
    let kpis = store.kpis.clone();
    let pagination_state = store.pagination.clone();
    let loading = store.loading.clone();

    // Handle filter changes
    let handle_filter_status = Callback::new({
        let filter_status = filter_status.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            filter_status.set(select.value());
        }
    });

    let handle_filter_bbbee = Callback::new({
        let filter_bbbee = filter_bbbee.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            filter_bbbee.set(select.value());
        }
    });

    let handle_filter_risk = Callback::new({
        let filter_risk = filter_risk.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            filter_risk.set(select.value());
        }
    });

    let handle_filter_province = Callback::new({
        let filter_province = filter_province.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            filter_province.set(select.value());
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

    // Toggle view mode
    let handle_toggle_view = Callback::new({
        let view_mode = view_mode.clone();
        move |mode: String| {
            view_mode.set(mode);
        }
    });

    // Clear filters
    let handle_clear_filters = Callback::<()>::new({
        let filter_status = filter_status.clone();
        let filter_bbbee = filter_bbbee.clone();
        let filter_risk = filter_risk.clone();
        let filter_province = filter_province.clone();
        let search_query = search_query.clone();
        let store = store.clone();
        move |_| {
            filter_status.set(String::new());
            filter_bbbee.set(String::new());
            filter_risk.set(String::new());
            filter_province.set(String::new());
            search_query.set(String::new());
            store.filter.set(SupplierFilter::default());
        }
    });

    // Handle row click
    let handle_row_click = Callback::new({
        move |supplier_id: String| {
            web_sys::window()
                .unwrap()
                .location()
                .set_href(&format!("#/suppliers/{}", supplier_id))
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
        DataTableColumn { key: "name".to_string(), label: "Supplier".to_string(), width: None, align: None, cell_class: None },
        DataTableColumn { key: "registration".to_string(), label: "Reg. No.".to_string(), width: Some("140px".to_string()), align: None, cell_class: Some("id-cell".to_string()) },
        DataTableColumn { key: "bbbee".to_string(), label: "B-BBEE".to_string(), width: Some("100px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "categories".to_string(), label: "Categories".to_string(), width: Some("180px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "performance".to_string(), label: "Performance".to_string(), width: Some("100px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "risk".to_string(), label: "Risk".to_string(), width: Some("90px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "status".to_string(), label: "Status".to_string(), width: Some("100px".to_string()), align: None, cell_class: None },
    ];

    // Filter suppliers based on current filters
    let filter = store.filter.clone();
    let filtered_suppliers: Vec<_> = suppliers.get().iter()
        .filter(|s| {
            let status_match = if filter_status.get().is_empty() {
                true
            } else {
                match filter_status.get().as_str() {
                    "active" => s.status == SupplierStatus::Active,
                    "pending" => s.status == SupplierStatus::Pending,
                    "suspended" => s.status == SupplierStatus::Suspended,
                    "blacklisted" => s.status == SupplierStatus::Blacklisted,
                    _ => true,
                }
            };

            let bbbee_match = if filter_bbbee.get().is_empty() {
                true
            } else {
                match filter_bbbee.get().as_str() {
                    "1" => s.bbbee_level == BbbeeLevel::Level1,
                    "2" => s.bbbee_level == BbbeeLevel::Level2,
                    "3" => s.bbbee_level == BbbeeLevel::Level3,
                    "4" => s.bbbee_level == BbbeeLevel::Level4,
                    "non" => s.bbbee_level == BbbeeLevel::NonCompliant,
                    _ => true,
                }
            };

            let risk_match = if filter_risk.get().is_empty() {
                true
            } else {
                match filter_risk.get().as_str() {
                    "low" => s.risk_rating == RiskRating::Low,
                    "medium" => s.risk_rating == RiskRating::Medium,
                    "high" => s.risk_rating == RiskRating::High,
                    "critical" => s.risk_rating == RiskRating::Critical,
                    _ => true,
                }
            };

            let province_match = if filter_province.get().is_empty() {
                true
            } else {
                s.province.to_lowercase() == filter_province.get().to_lowercase()
            };

            let search_match = if search_query.get().is_empty() {
                true
            } else {
                let q = search_query.get().to_lowercase();
                s.name.to_lowercase().contains(&q) ||
                s.registration_number.to_lowercase().contains(&q) ||
                s.contact_person.to_lowercase().contains(&q)
            };

            status_match && bbbee_match && risk_match && province_match && search_match
        })
        .cloned()
        .collect();

    // Transform suppliers to table rows
    let rows: Vec<DataTableRow> = filtered_suppliers.iter().map(|supplier| {
        let bbbee = match supplier.bbbee_level {
            BbbeeLevel::Level1 => bbbee_badge(BadgeLevel::Level1),
            BbbeeLevel::Level2 => bbbee_badge(BadgeLevel::Level2),
            BbbeeLevel::Level3 | BbbeeLevel::Level4 => bbbee_badge(BadgeLevel::Level3),
            _ => bbbee_badge(BadgeLevel::NonCompliant),
        };

        let risk_tag = match supplier.risk_rating {
            RiskRating::Low => tag("Low".to_string(), TagType::Green),
            RiskRating::Medium => tag("Medium".to_string(), TagType::Orange),
            RiskRating::High => tag("High".to_string(), TagType::Red),
            RiskRating::Critical => tag("Critical".to_string(), TagType::Red),
        };

        let status = match supplier.status {
            SupplierStatus::Active => status_badge(StatusType::Active),
            SupplierStatus::Pending => status_badge(StatusType::Pending),
            SupplierStatus::Suspended => status_badge(StatusType::Cancelled),
            SupplierStatus::Blacklisted => status_badge(StatusType::Rejected),
            SupplierStatus::Expired => status_badge(StatusType::Draft),
            SupplierStatus::Inactive => status_badge(StatusType::Complete),
        };

        let categories = supplier.categories.iter()
            .take(2)
            .map(|c| c.name.clone())
            .collect::<Vec<_>>()
            .join(", ");

        let perf_color = if supplier.performance_score.overall >= 80.0 {
            "var(--green)"
        } else if supplier.performance_score.overall >= 60.0 {
            "var(--orange)"
        } else {
            "var(--red)"
        };

        DataTableRow {
            id: supplier.id.clone(),
            cells: vec![
                view! {
                    <div class="supplier-name">
                        <span class="name">{supplier.name.clone()}</span>
                        <span class="location">{format!("{}, {}", supplier.city.clone(), supplier.province.clone())}</span>
                    </div>
                },
                view! { <span class="id-cell">{supplier.registration_number.clone()}</span> },
                bbbee,
                view! { <span class="categories">{categories}</span> },
                view! {
                    <span class="performance-score" style={format!("color: {}", perf_color)}>
                        {format!("{:.0}%", supplier.performance_score.overall)}
                    </span>
                },
                risk_tag,
                status,
            ],
        }
    }).collect();

    // Icons
    let icon_users = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>"#;
    let icon_check = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>"#;
    let icon_layers = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/><path d="M2 12l10 5 10-5"/></svg>"#;
    let icon_alert = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>"#;

    let kpis_data = kpis.get();
    let pag_state = pagination_state.get();

    view! {
        style {
            r#"
            .supplier-registry { display: flex; flex-direction: column; gap: var(--space-4); }
            .kpi-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; margin-bottom: 8px; }
            @media (max-width: 1200px) { .kpi-grid { grid-template-columns: repeat(2, 1fr); } }
            @media (max-width: 768px) { .kpi-grid { grid-template-columns: 1fr; } }
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
            .filter-spacer { flex: 1; }
            .search-input { min-width: 200px; }
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
            .supplier-name {
                display: flex;
                flex-direction: column;
                gap: 2px;
            }
            .supplier-name .name {
                font-weight: 500;
                color: var(--text);
            }
            .supplier-name .location {
                font-size: 11px;
                color: var(--text-muted);
            }
            .categories {
                font-size: 12px;
                color: var(--text-muted);
            }
            .performance-score {
                font-weight: 600;
                font-family: IBM Plex Mono, monospace;
            }
            .loading-overlay {
                display: flex;
                align-items: center;
                justify-content: center;
                padding: 40px;
                color: var(--text-muted);
            }
            .supplier-cards {
                display: grid;
                grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
                gap: 16px;
            }
            .supplier-card {
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius-lg);
                padding: 20px;
                cursor: pointer;
                transition: transform 0.2s, box-shadow 0.2s;
            }
            .supplier-card:hover {
                transform: translateY(-2px);
                box-shadow: var(--shadow);
            }
            .supplier-card-header {
                display: flex;
                justify-content: space-between;
                align-items: flex-start;
                margin-bottom: 12px;
            }
            .supplier-card-title {
                font-weight: 600;
                color: var(--text);
                margin-bottom: 4px;
            }
            .supplier-card-subtitle {
                font-size: 12px;
                color: var(--text-muted);
            }
            .supplier-card-body {
                display: flex;
                flex-direction: column;
                gap: 12px;
            }
            .supplier-card-row {
                display: flex;
                justify-content: space-between;
                align-items: center;
                font-size: 13px;
            }
            .supplier-card-label {
                color: var(--text-muted);
            }
            .supplier-card-value {
                font-weight: 500;
            }
            .supplier-card-footer {
                display: flex;
                gap: 8px;
                margin-top: 12px;
                padding-top: 12px;
                border-top: 1px solid var(--border);
            }
            "#
        }

        <div class="supplier-registry" data-testid="supplier-registry">
            {page_header(
                "Supplier Registry".to_string(),
                Some("Manage and monitor registered suppliers".to_string()),
                vec![
                    view! { <button class="btn btn-secondary">"Export"</button> },
                    view! { <a href="#/suppliers/new" class="btn btn-primary">"Add Supplier"</a> },
                ]
            )}

            // KPI summary
            <div class="kpi-grid">
                {kpi_card(
                    "Total Suppliers".to_string(),
                    format_number(kpis_data.total_suppliers),
                    KpiColor::Blue,
                    icon_users.to_string(),
                    None,
                    None
                )}
                {kpi_card(
                    "Active Suppliers".to_string(),
                    format_number(kpis_data.active_suppliers),
                    KpiColor::Green,
                    icon_check.to_string(),
                    None,
                    None
                )}
                {kpi_card(
                    "B-BBEE Compliant".to_string(),
                    format!("{:.1}%", kpis_data.bbbee_compliant_percent),
                    KpiColor::Accent,
                    icon_layers.to_string(),
                    None,
                    None
                )}
                {kpi_card(
                    "High Risk".to_string(),
                    kpis_data.high_risk_count.to_string(),
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
                        <label>"Status"</label>
                        <select on:change={handle_filter_status}>
                            <option value="">"All Statuses"</option>
                            <option value="active">"Active"</option>
                            <option value="pending">"Pending"</option>
                            <option value="suspended">"Suspended"</option>
                            <option value="blacklisted">"Blacklisted"</option>
                        </select>
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"B-BBEE"</label>
                        <select on:change={handle_filter_bbbee}>
                            <option value="">"All Levels"</option>
                            <option value="1">"Level 1"</option>
                            <option value="2">"Level 2"</option>
                            <option value="3">"Level 3"</option>
                            <option value="4">"Level 4+"</option>
                            <option value="non">"Non-Compliant"</option>
                        </select>
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"Risk"</label>
                        <select on:change={handle_filter_risk}>
                            <option value="">"All Risk Levels"</option>
                            <option value="low">"Low"</option>
                            <option value="medium">"Medium"</option>
                            <option value="high">"High"</option>
                            <option value="critical">"Critical"</option>
                        </select>
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"Province"</label>
                        <select on:change={handle_filter_province}>
                            <option value="">"All Provinces"</option>
                            <option value="gauteng">"Gauteng"</option>
                            <option value="western cape">"Western Cape"</option>
                            <option value="kwazulu-natal">"KwaZulu-Natal"</option>
                            <option value="eastern cape">"Eastern Cape"</option>
                            <option value="free state">"Free State"</option>
                            <option value="mpumalanga">"Mpumalanga"</option>
                            <option value="limpopo">"Limpopo"</option>
                            <option value="north west">"North West"</option>
                            <option value="northern cape">"Northern Cape"</option>
                        </select>
                    </div>
                },
                view! { <div class="filter-spacer"></div> },
                view! {
                    <div class="filter-group">
                        <input
                            type="text"
                            class="search-input"
                            placeholder="Search suppliers..."
                            on:input={handle_search}
                        />
                    </div>
                },
                view! {
                    <div class="view-toggle">
                        <button
                            class={if view_mode.get() == "table" { "active" } else { "" }}
                            on:click={Callback::new({
                                let handle_toggle_view = handle_toggle_view.clone();
                                move |_| handle_toggle_view.call("table".to_string())
                            })}
                        >"Table"</button>
                        <button
                            class={if view_mode.get() == "cards" { "active" } else { "" }}
                            on:click={Callback::new({
                                let handle_toggle_view = handle_toggle_view.clone();
                                move |_| handle_toggle_view.call("cards".to_string())
                            })}
                        >"Cards"</button>
                    </div>
                },
                view! {
                    <button class="btn btn-sm btn-secondary" on:click={handle_clear_filters}>"Clear"</button>
                },
            ])}

            // Data display (table or cards)
            {panel(
                format!("Suppliers ({} total)", filtered_suppliers.len()),
                vec![],
                vec![
                    if loading.get() {
                        view! { <div class="loading-overlay">"Loading suppliers..."</div> }
                    } else if filtered_suppliers.is_empty() {
                        view! {
                            <div class="loading-overlay">
                                "No suppliers found. Adjust filters or add a new supplier."
                            </div>
                        }
                    } else if view_mode.get() == "cards" {
                        view! {
                            <div class="supplier-cards">
                                for supplier in filtered_suppliers.iter() {
                                    {supplier_card(supplier.clone(), handle_row_click.clone())}
                                }
                            </div>
                        }
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
    }
}

/// Supplier card component for card view
fn supplier_card(supplier: super::types::Supplier, on_click: Callback<String>) -> View {
    let supplier_id = supplier.id.clone();
    let handle_click = Callback::unit({
        let on_click = on_click.clone();
        let supplier_id = supplier_id.clone();
        move || {
            on_click.call(supplier_id.clone());
        }
    });

    let bbbee = match supplier.bbbee_level {
        BbbeeLevel::Level1 => bbbee_badge(BadgeLevel::Level1),
        BbbeeLevel::Level2 => bbbee_badge(BadgeLevel::Level2),
        BbbeeLevel::Level3 | BbbeeLevel::Level4 => bbbee_badge(BadgeLevel::Level3),
        _ => bbbee_badge(BadgeLevel::NonCompliant),
    };

    let risk_tag = match supplier.risk_rating {
        RiskRating::Low => tag("Low Risk".to_string(), TagType::Green),
        RiskRating::Medium => tag("Medium Risk".to_string(), TagType::Orange),
        RiskRating::High => tag("High Risk".to_string(), TagType::Red),
        RiskRating::Critical => tag("Critical Risk".to_string(), TagType::Red),
    };

    let perf_color = if supplier.performance_score.overall >= 80.0 {
        "var(--green)"
    } else if supplier.performance_score.overall >= 60.0 {
        "var(--orange)"
    } else {
        "var(--red)"
    };

    view! {
        <div class="supplier-card" on:click={handle_click}>
            <div class="supplier-card-header">
                <div>
                    <div class="supplier-card-title">{supplier.name}</div>
                    <div class="supplier-card-subtitle">{format!("{}, {}", supplier.city, supplier.province)}</div>
                </div>
                {bbbee}
            </div>
            <div class="supplier-card-body">
                <div class="supplier-card-row">
                    <span class="supplier-card-label">"Registration"</span>
                    <span class="supplier-card-value">{supplier.registration_number}</span>
                </div>
                <div class="supplier-card-row">
                    <span class="supplier-card-label">"Performance"</span>
                    <span class="supplier-card-value" style={format!("color: {}", perf_color)}>
                        {format!("{:.0}%", supplier.performance_score.overall)}
                    </span>
                </div>
                <div class="supplier-card-row">
                    <span class="supplier-card-label">"Active Contracts"</span>
                    <span class="supplier-card-value">{supplier.active_contracts.to_string()}</span>
                </div>
                <div class="supplier-card-row">
                    <span class="supplier-card-label">"Total Value"</span>
                    <span class="supplier-card-value">{format_currency(supplier.total_contract_value)}</span>
                </div>
            </div>
            <div class="supplier-card-footer">
                {risk_tag}
            </div>
        </div>
    }
}
