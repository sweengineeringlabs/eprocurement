//! Supplier risk assessment page with risk indicators

use components::prelude::*;
use wasm_bindgen::JsCast;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, data_table, DataTableColumn, DataTableRow,
    status_badge, StatusType,
    bbbee_badge, BbbeeLevel as BadgeLevel,
    tag, TagType,
    kpi_card, KpiColor,
    progress_bar, ProgressColor,
};
use crate::shared::charts::{bar_chart, BarChartData, pie_chart, PieChartData};
use crate::shared::forms::filter_bar;
use crate::util::format::format_currency;
use super::store::SuppliersStore;
use super::types::{Supplier, SupplierStatus, BbbeeLevel, RiskRating, RiskIndicator, RiskIndicatorStatus};
use super::service;

/// Supplier risk assessment page
#[component]
pub fn supplier_risk() -> View {
    let store = use_context::<SuppliersStore>();

    // Filter signals
    let risk_filter = signal(String::new());
    let category_filter = signal(String::new());
    let indicator_status = signal(String::new());
    let search_query = signal(String::new());

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
    let loading = store.loading.clone();

    // Handle filter changes
    let handle_risk_filter = Callback::new({
        let risk_filter = risk_filter.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            risk_filter.set(select.value());
        }
    });

    let handle_category_filter = Callback::new({
        let category_filter = category_filter.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            category_filter.set(select.value());
        }
    });

    let handle_indicator_status = Callback::new({
        let indicator_status = indicator_status.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            indicator_status.set(select.value());
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

    // Calculate risk metrics
    let all_suppliers = suppliers.get();
    let active_suppliers: Vec<_> = all_suppliers.iter()
        .filter(|s| s.status == SupplierStatus::Active || s.status == SupplierStatus::Suspended)
        .cloned()
        .collect();

    let low_risk = active_suppliers.iter().filter(|s| s.risk_rating == RiskRating::Low).count();
    let medium_risk = active_suppliers.iter().filter(|s| s.risk_rating == RiskRating::Medium).count();
    let high_risk = active_suppliers.iter().filter(|s| s.risk_rating == RiskRating::High).count();
    let critical_risk = active_suppliers.iter().filter(|s| s.risk_rating == RiskRating::Critical).count();

    // Collect all risk indicators
    let all_indicators: Vec<(String, RiskIndicator)> = active_suppliers.iter()
        .flat_map(|s| s.risk_indicators.iter().map(|r| (s.id.clone(), r.clone())))
        .collect();

    let open_indicators = all_indicators.iter()
        .filter(|(_, r)| r.status == RiskIndicatorStatus::Open || r.status == RiskIndicatorStatus::UnderReview)
        .count();

    // Filter suppliers
    let filtered_suppliers: Vec<_> = active_suppliers.iter()
        .filter(|s| {
            let risk_match = match risk_filter.get().as_str() {
                "low" => s.risk_rating == RiskRating::Low,
                "medium" => s.risk_rating == RiskRating::Medium,
                "high" => s.risk_rating == RiskRating::High,
                "critical" => s.risk_rating == RiskRating::Critical,
                _ => true,
            };

            let search_match = if search_query.get().is_empty() {
                true
            } else {
                let q = search_query.get().to_lowercase();
                s.name.to_lowercase().contains(&q)
            };

            risk_match && search_match
        })
        .cloned()
        .collect();

    // Risk distribution for pie chart
    let risk_distribution = vec![
        PieChartData { label: "Low".to_string(), value: low_risk as f64, color: "var(--green)".to_string() },
        PieChartData { label: "Medium".to_string(), value: medium_risk as f64, color: "var(--orange)".to_string() },
        PieChartData { label: "High".to_string(), value: high_risk as f64, color: "var(--red)".to_string() },
        PieChartData { label: "Critical".to_string(), value: critical_risk as f64, color: "#8b0000".to_string() },
    ];

    // Risk by category
    let risk_by_category = vec![
        BarChartData { label: "Financial".to_string(), value: all_indicators.iter().filter(|(_, r)| r.category == "Financial").count() as f64, color: Some("var(--blue)".to_string()) },
        BarChartData { label: "Compliance".to_string(), value: all_indicators.iter().filter(|(_, r)| r.category == "Compliance").count() as f64, color: Some("var(--orange)".to_string()) },
        BarChartData { label: "Performance".to_string(), value: all_indicators.iter().filter(|(_, r)| r.category == "Performance").count() as f64, color: Some("var(--purple)".to_string()) },
        BarChartData { label: "Delivery".to_string(), value: all_indicators.iter().filter(|(_, r)| r.category == "Delivery").count() as f64, color: Some("var(--red)".to_string()) },
    ];

    // Table columns for suppliers at risk
    let risk_columns = vec![
        DataTableColumn { key: "supplier".to_string(), label: "Supplier".to_string(), width: None, align: None, cell_class: None },
        DataTableColumn { key: "risk_rating".to_string(), label: "Risk Rating".to_string(), width: Some("100px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "risk_score".to_string(), label: "Score".to_string(), width: Some("80px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "indicators".to_string(), label: "Open Issues".to_string(), width: Some("100px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "bbbee".to_string(), label: "B-BBEE".to_string(), width: Some("90px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "contract_value".to_string(), label: "Contract Value".to_string(), width: Some("140px".to_string()), align: Some("right".to_string()), cell_class: None },
    ];

    // Transform suppliers to risk table rows
    let risk_rows: Vec<DataTableRow> = filtered_suppliers.iter().map(|supplier| {
        let risk_tag = match supplier.risk_rating {
            RiskRating::Low => tag("Low".to_string(), TagType::Green),
            RiskRating::Medium => tag("Medium".to_string(), TagType::Orange),
            RiskRating::High => tag("High".to_string(), TagType::Red),
            RiskRating::Critical => tag("Critical".to_string(), TagType::Red),
        };

        let bbbee = match supplier.bbbee_level {
            BbbeeLevel::Level1 => bbbee_badge(BadgeLevel::Level1),
            BbbeeLevel::Level2 => bbbee_badge(BadgeLevel::Level2),
            BbbeeLevel::Level3 | BbbeeLevel::Level4 => bbbee_badge(BadgeLevel::Level3),
            _ => bbbee_badge(BadgeLevel::NonCompliant),
        };

        let open_count = supplier.risk_indicators.iter()
            .filter(|r| r.status == RiskIndicatorStatus::Open || r.status == RiskIndicatorStatus::UnderReview)
            .count();

        let score_color = if supplier.risk_score >= 70.0 {
            "var(--red)"
        } else if supplier.risk_score >= 40.0 {
            "var(--orange)"
        } else {
            "var(--green)"
        };

        DataTableRow {
            id: supplier.id.clone(),
            cells: vec![
                view! {
                    <div class="supplier-info">
                        <span class="supplier-name">{supplier.name.clone()}</span>
                        <span class="supplier-location">{format!("{}, {}", supplier.city.clone(), supplier.province.clone())}</span>
                    </div>
                },
                risk_tag,
                view! {
                    <span class="risk-score" style={format!("color: {}", score_color)}>
                        {format!("{:.0}", supplier.risk_score)}
                    </span>
                },
                view! {
                    <span class={if open_count > 0 { "open-issues has-issues" } else { "open-issues" }}>
                        {open_count.to_string()}
                    </span>
                },
                bbbee,
                view! {
                    <span class="contract-value">{format_currency(supplier.total_contract_value)}</span>
                },
            ],
        }
    }).collect();

    // Risk indicators table columns
    let indicator_columns = vec![
        DataTableColumn { key: "supplier".to_string(), label: "Supplier".to_string(), width: Some("200px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "category".to_string(), label: "Category".to_string(), width: Some("100px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "indicator".to_string(), label: "Risk Indicator".to_string(), width: None, align: None, cell_class: None },
        DataTableColumn { key: "severity".to_string(), label: "Severity".to_string(), width: Some("90px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "status".to_string(), label: "Status".to_string(), width: Some("110px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "detected".to_string(), label: "Detected".to_string(), width: Some("100px".to_string()), align: None, cell_class: None },
    ];

    // Filter indicators
    let filtered_indicators: Vec<_> = all_indicators.iter()
        .filter(|(_, r)| {
            let status_match = match indicator_status.get().as_str() {
                "open" => r.status == RiskIndicatorStatus::Open,
                "review" => r.status == RiskIndicatorStatus::UnderReview,
                "mitigated" => r.status == RiskIndicatorStatus::Mitigated,
                "closed" => r.status == RiskIndicatorStatus::Closed,
                _ => true,
            };

            let category_match = if category_filter.get().is_empty() {
                true
            } else {
                r.category.to_lowercase() == category_filter.get().to_lowercase()
            };

            status_match && category_match
        })
        .cloned()
        .collect();

    // Transform indicators to table rows
    let indicator_rows: Vec<DataTableRow> = filtered_indicators.iter().map(|(supplier_id, indicator)| {
        let supplier_name = active_suppliers.iter()
            .find(|s| &s.id == supplier_id)
            .map(|s| s.name.clone())
            .unwrap_or_default();

        let severity_tag = match indicator.severity {
            RiskRating::Low => tag("Low".to_string(), TagType::Green),
            RiskRating::Medium => tag("Medium".to_string(), TagType::Orange),
            RiskRating::High => tag("High".to_string(), TagType::Red),
            RiskRating::Critical => tag("Critical".to_string(), TagType::Red),
        };

        let status = match indicator.status {
            RiskIndicatorStatus::Open => status_badge(StatusType::Pending),
            RiskIndicatorStatus::UnderReview => status_badge(StatusType::Evaluation),
            RiskIndicatorStatus::Mitigated => status_badge(StatusType::Active),
            RiskIndicatorStatus::Accepted => status_badge(StatusType::Approved),
            RiskIndicatorStatus::Closed => status_badge(StatusType::Complete),
        };

        let category_tag = match indicator.category.as_str() {
            "Financial" => tag(indicator.category.clone(), TagType::Blue),
            "Compliance" => tag(indicator.category.clone(), TagType::Orange),
            "Performance" => tag(indicator.category.clone(), TagType::Purple),
            "Delivery" => tag(indicator.category.clone(), TagType::Red),
            _ => tag(indicator.category.clone(), TagType::Default),
        };

        DataTableRow {
            id: indicator.id.clone(),
            cells: vec![
                view! { <span>{supplier_name}</span> },
                category_tag,
                view! {
                    <div class="indicator-info">
                        <span class="indicator-name">{indicator.indicator.clone()}</span>
                        <span class="indicator-desc">{indicator.description.clone()}</span>
                    </div>
                },
                severity_tag,
                status,
                view! { <span class="detected-date">{indicator.detected_at.clone()}</span> },
            ],
        }
    }).collect();

    // Icons
    let icon_shield = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>"#;
    let icon_alert = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>"#;
    let icon_warning = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>"#;
    let icon_check = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>"#;

    view! {
        style {
            r#"
            .supplier-risk { display: flex; flex-direction: column; gap: var(--space-4); }
            .kpi-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; margin-bottom: 8px; }
            @media (max-width: 1200px) { .kpi-grid { grid-template-columns: repeat(2, 1fr); } }
            @media (max-width: 768px) { .kpi-grid { grid-template-columns: 1fr; } }
            .charts-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; }
            @media (max-width: 900px) { .charts-grid { grid-template-columns: 1fr; } }
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
            .filter-spacer { flex: 1; }
            .search-input { min-width: 200px; }
            .supplier-info {
                display: flex;
                flex-direction: column;
                gap: 2px;
            }
            .supplier-name {
                font-weight: 500;
                color: var(--text);
            }
            .supplier-location {
                font-size: 11px;
                color: var(--text-muted);
            }
            .risk-score {
                font-weight: 600;
                font-family: IBM Plex Mono, monospace;
            }
            .open-issues {
                font-weight: 500;
            }
            .open-issues.has-issues {
                color: var(--red);
            }
            .contract-value {
                font-family: IBM Plex Mono, monospace;
            }
            .indicator-info {
                display: flex;
                flex-direction: column;
                gap: 2px;
            }
            .indicator-name {
                font-weight: 500;
                color: var(--text);
            }
            .indicator-desc {
                font-size: 11px;
                color: var(--text-muted);
            }
            .detected-date {
                font-size: 12px;
                color: var(--text-muted);
            }
            .loading-overlay {
                display: flex;
                align-items: center;
                justify-content: center;
                padding: 40px;
                color: var(--text-muted);
            }
            .risk-summary {
                display: grid;
                grid-template-columns: repeat(4, 1fr);
                gap: 16px;
                padding: 16px;
                background: var(--bg);
                border-radius: var(--radius);
            }
            .risk-summary-item {
                text-align: center;
            }
            .risk-summary-count {
                font-size: 28px;
                font-weight: 600;
                font-family: IBM Plex Mono, monospace;
            }
            .risk-summary-label {
                font-size: 12px;
                color: var(--text-muted);
                margin-top: 4px;
            }
            .risk-low { color: var(--green); }
            .risk-medium { color: var(--orange); }
            .risk-high { color: var(--red); }
            .risk-critical { color: #8b0000; }
            .critical-suppliers {
                display: flex;
                flex-direction: column;
                gap: 12px;
            }
            .critical-supplier-item {
                display: flex;
                align-items: center;
                justify-content: space-between;
                padding: 12px;
                background: #fff5f5;
                border: 1px solid #ffcccc;
                border-radius: var(--radius);
            }
            .critical-supplier-info {
                display: flex;
                flex-direction: column;
            }
            .critical-supplier-name {
                font-weight: 500;
                color: var(--red);
            }
            .critical-supplier-issues {
                font-size: 12px;
                color: var(--text-muted);
            }
            "#
        }

        <div class="supplier-risk" data-testid="supplier-risk">
            {page_header(
                "Supplier Risk Assessment".to_string(),
                Some("Monitor and manage supplier risk exposure".to_string()),
                vec![
                    view! { <button class="btn btn-secondary">"Export Report"</button> },
                    view! { <button class="btn btn-primary">"Run Assessment"</button> },
                ]
            )}

            // KPI summary
            <div class="kpi-grid">
                {kpi_card(
                    "Low Risk".to_string(),
                    format!("{}", low_risk),
                    KpiColor::Green,
                    icon_shield.to_string(),
                    None,
                    None
                )}
                {kpi_card(
                    "Medium Risk".to_string(),
                    format!("{}", medium_risk),
                    KpiColor::Orange,
                    icon_warning.to_string(),
                    None,
                    None
                )}
                {kpi_card(
                    "High/Critical Risk".to_string(),
                    format!("{}", high_risk + critical_risk),
                    KpiColor::Red,
                    icon_alert.to_string(),
                    None,
                    None
                )}
                {kpi_card(
                    "Open Issues".to_string(),
                    format!("{}", open_indicators),
                    KpiColor::Purple,
                    icon_check.to_string(),
                    None,
                    None
                )}
            </div>

            // Charts row
            <div class="charts-grid">
                {panel(
                    "Risk Distribution".to_string(),
                    vec![],
                    vec![
                        pie_chart(risk_distribution, None),
                        view! {
                            <div class="risk-summary">
                                <div class="risk-summary-item">
                                    <div class="risk-summary-count risk-low">{low_risk.to_string()}</div>
                                    <div class="risk-summary-label">"Low Risk"</div>
                                </div>
                                <div class="risk-summary-item">
                                    <div class="risk-summary-count risk-medium">{medium_risk.to_string()}</div>
                                    <div class="risk-summary-label">"Medium Risk"</div>
                                </div>
                                <div class="risk-summary-item">
                                    <div class="risk-summary-count risk-high">{high_risk.to_string()}</div>
                                    <div class="risk-summary-label">"High Risk"</div>
                                </div>
                                <div class="risk-summary-item">
                                    <div class="risk-summary-count risk-critical">{critical_risk.to_string()}</div>
                                    <div class="risk-summary-label">"Critical"</div>
                                </div>
                            </div>
                        },
                    ]
                )}
                {panel(
                    "Risk Indicators by Category".to_string(),
                    vec![],
                    vec![bar_chart(risk_by_category, Some(180))]
                )}
            </div>

            // Critical suppliers
            if critical_risk > 0 {
                {panel(
                    "Critical Risk Suppliers - Immediate Attention Required".to_string(),
                    vec![],
                    vec![{
                        let critical_suppliers: Vec<_> = active_suppliers.iter()
                            .filter(|s| s.risk_rating == RiskRating::Critical)
                            .collect();

                        view! {
                            <div class="critical-suppliers">
                                for supplier in critical_suppliers.iter() {
                                    <div class="critical-supplier-item">
                                        <div class="critical-supplier-info">
                                            <span class="critical-supplier-name">{supplier.name.clone()}</span>
                                            <span class="critical-supplier-issues">
                                                {format!("{} open risk indicators - Risk Score: {:.0}",
                                                    supplier.risk_indicators.iter()
                                                        .filter(|r| r.status == RiskIndicatorStatus::Open)
                                                        .count(),
                                                    supplier.risk_score
                                                )}
                                            </span>
                                        </div>
                                        <a href={format!("#/suppliers/{}", supplier.id)} class="btn btn-sm btn-danger">"Review"</a>
                                    </div>
                                }
                            </div>
                        }
                    }]
                )}
            }

            // Filter bar for suppliers
            {filter_bar(vec![
                view! {
                    <div class="filter-group">
                        <label>"Risk Level"</label>
                        <select on:change={handle_risk_filter}>
                            <option value="">"All Risk Levels"</option>
                            <option value="low">"Low"</option>
                            <option value="medium">"Medium"</option>
                            <option value="high">"High"</option>
                            <option value="critical">"Critical"</option>
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
            ])}

            // Suppliers at risk table
            {panel(
                format!("Suppliers at Risk ({} total)", filtered_suppliers.len()),
                vec![],
                vec![
                    if loading.get() {
                        view! { <div class="loading-overlay">"Loading risk data..."</div> }
                    } else if filtered_suppliers.is_empty() {
                        view! {
                            <div class="loading-overlay">
                                "No suppliers match the selected criteria."
                            </div>
                        }
                    } else {
                        view! {
                            <div>
                                {data_table(risk_columns, risk_rows, Some(handle_row_click))}
                            </div>
                        }
                    }
                ]
            )}

            // Filter bar for indicators
            {filter_bar(vec![
                view! {
                    <div class="filter-group">
                        <label>"Indicator Status"</label>
                        <select on:change={handle_indicator_status}>
                            <option value="">"All Statuses"</option>
                            <option value="open">"Open"</option>
                            <option value="review">"Under Review"</option>
                            <option value="mitigated">"Mitigated"</option>
                            <option value="closed">"Closed"</option>
                        </select>
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"Category"</label>
                        <select on:change={handle_category_filter}>
                            <option value="">"All Categories"</option>
                            <option value="financial">"Financial"</option>
                            <option value="compliance">"Compliance"</option>
                            <option value="performance">"Performance"</option>
                            <option value="delivery">"Delivery"</option>
                        </select>
                    </div>
                },
                view! { <div class="filter-spacer"></div> },
            ])}

            // Risk indicators table
            {panel(
                format!("Risk Indicators ({} total)", filtered_indicators.len()),
                vec![],
                vec![
                    if filtered_indicators.is_empty() {
                        view! {
                            <div class="loading-overlay">
                                "No risk indicators found."
                            </div>
                        }
                    } else {
                        view! {
                            <div>
                                {data_table(indicator_columns, indicator_rows, None)}
                            </div>
                        }
                    }
                ]
            )}
        </div>
    }
}
