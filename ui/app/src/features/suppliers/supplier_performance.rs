//! Supplier performance dashboard with scores and trends

use components::prelude::*;
use wasm_bindgen::JsCast;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, data_table, DataTableColumn, DataTableRow,
    kpi_card, KpiColor,
};
use crate::shared::charts::{bar_chart, BarChartData, trend_chart, TrendChartData};
use crate::shared::forms::filter_bar;
use super::store::SuppliersStore;
use super::types::SupplierStatus;
use super::service;

/// Supplier performance dashboard page
#[component]
pub fn supplier_performance() -> View {
    let store = use_context::<SuppliersStore>();

    // Filter signals
    let performance_filter = signal(String::new()); // "all", "top", "underperforming"
    let category_filter = signal(String::new());
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
    let _kpis = store.kpis.clone();
    let loading = store.loading.clone();

    // Handle filter changes
    let handle_performance_filter = Callback::new({
        let performance_filter = performance_filter.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            performance_filter.set(select.value());
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

    // Filter suppliers
    let all_suppliers = suppliers.get();
    let active_suppliers: Vec<_> = all_suppliers.iter()
        .filter(|s| s.status == SupplierStatus::Active)
        .cloned()
        .collect();

    let filtered_suppliers: Vec<_> = active_suppliers.iter()
        .filter(|s| {
            let perf_match = match performance_filter.get().as_str() {
                "top" => s.performance_score.overall >= 85.0,
                "underperforming" => s.performance_score.overall < 70.0,
                _ => true,
            };

            let category_match = if category_filter.get().is_empty() {
                true
            } else {
                s.categories.iter().any(|c| c.code.starts_with(&category_filter.get()))
            };

            let search_match = if search_query.get().is_empty() {
                true
            } else {
                let q = search_query.get().to_lowercase();
                s.name.to_lowercase().contains(&q)
            };

            perf_match && category_match && search_match
        })
        .cloned()
        .collect();

    // Calculate performance metrics
    let total_active = active_suppliers.len() as f64;
    let avg_performance = if total_active > 0.0 {
        active_suppliers.iter().map(|s| s.performance_score.overall).sum::<f64>() / total_active
    } else {
        0.0
    };
    let top_performers = active_suppliers.iter().filter(|s| s.performance_score.overall >= 85.0).count();
    let underperformers = active_suppliers.iter().filter(|s| s.performance_score.overall < 70.0).count();

    // Performance distribution for chart
    let perf_distribution = vec![
        BarChartData { label: "90-100%".to_string(), value: active_suppliers.iter().filter(|s| s.performance_score.overall >= 90.0).count() as f64, color: Some("var(--green)".to_string()) },
        BarChartData { label: "80-89%".to_string(), value: active_suppliers.iter().filter(|s| s.performance_score.overall >= 80.0 && s.performance_score.overall < 90.0).count() as f64, color: Some("var(--blue)".to_string()) },
        BarChartData { label: "70-79%".to_string(), value: active_suppliers.iter().filter(|s| s.performance_score.overall >= 70.0 && s.performance_score.overall < 80.0).count() as f64, color: Some("var(--orange)".to_string()) },
        BarChartData { label: "60-69%".to_string(), value: active_suppliers.iter().filter(|s| s.performance_score.overall >= 60.0 && s.performance_score.overall < 70.0).count() as f64, color: Some("var(--orange)".to_string()) },
        BarChartData { label: "<60%".to_string(), value: active_suppliers.iter().filter(|s| s.performance_score.overall < 60.0).count() as f64, color: Some("var(--red)".to_string()) },
    ];

    // Category performance
    let category_performance = vec![
        BarChartData { label: "IT Services".to_string(), value: 89.0, color: Some("var(--blue)".to_string()) },
        BarChartData { label: "Construction".to_string(), value: 82.0, color: Some("var(--blue)".to_string()) },
        BarChartData { label: "Security".to_string(), value: 86.0, color: Some("var(--blue)".to_string()) },
        BarChartData { label: "Catering".to_string(), value: 91.0, color: Some("var(--blue)".to_string()) },
        BarChartData { label: "Transport".to_string(), value: 84.0, color: Some("var(--blue)".to_string()) },
        BarChartData { label: "Professional".to_string(), value: 78.0, color: Some("var(--orange)".to_string()) },
    ];

    // Table columns
    let columns = vec![
        DataTableColumn { key: "supplier".to_string(), label: "Supplier".to_string(), width: None, align: None, cell_class: None },
        DataTableColumn { key: "overall".to_string(), label: "Overall".to_string(), width: Some("90px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "quality".to_string(), label: "Quality".to_string(), width: Some("80px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "delivery".to_string(), label: "Delivery".to_string(), width: Some("80px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "price".to_string(), label: "Price".to_string(), width: Some("80px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "responsive".to_string(), label: "Response".to_string(), width: Some("80px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "trend".to_string(), label: "Trend".to_string(), width: Some("120px".to_string()), align: None, cell_class: None },
    ];

    // Transform suppliers to table rows
    let rows: Vec<DataTableRow> = filtered_suppliers.iter().map(|supplier| {
        let overall_color = score_color(supplier.performance_score.overall);
        let quality_color = score_color(supplier.performance_score.quality);
        let delivery_color = score_color(supplier.performance_score.delivery);
        let price_color = score_color(supplier.performance_score.price);
        let responsive_color = score_color(supplier.performance_score.responsiveness);

        // Calculate trend direction
        let trend_data: Vec<TrendChartData> = supplier.performance_history.iter()
            .map(|t| TrendChartData { label: t.period.clone(), value: t.score })
            .collect();

        let trend_direction = if supplier.performance_history.len() >= 2 {
            let last = supplier.performance_history.last().map(|t| t.score).unwrap_or(0.0);
            let prev = supplier.performance_history.get(supplier.performance_history.len() - 2).map(|t| t.score).unwrap_or(0.0);
            if last > prev { "up" } else if last < prev { "down" } else { "stable" }
        } else {
            "stable"
        };

        // Pre-compute chart height to avoid type inference issues in view! macro
        let trend_height: u32 = 30;

        DataTableRow {
            id: supplier.id.clone(),
            cells: vec![
                view! {
                    <div class="supplier-info">
                        <span class="supplier-name">{supplier.name.clone()}</span>
                        <span class="supplier-category">{supplier.categories.first().map(|c| c.name.clone()).unwrap_or_default()}</span>
                    </div>
                },
                view! {
                    <span class="score-cell" style={format!("color: {}", overall_color)}>
                        {format!("{:.0}%", supplier.performance_score.overall)}
                    </span>
                },
                view! {
                    <span class="score-cell" style={format!("color: {}", quality_color)}>
                        {format!("{:.0}", supplier.performance_score.quality)}
                    </span>
                },
                view! {
                    <span class="score-cell" style={format!("color: {}", delivery_color)}>
                        {format!("{:.0}", supplier.performance_score.delivery)}
                    </span>
                },
                view! {
                    <span class="score-cell" style={format!("color: {}", price_color)}>
                        {format!("{:.0}", supplier.performance_score.price)}
                    </span>
                },
                view! {
                    <span class="score-cell" style={format!("color: {}", responsive_color)}>
                        {format!("{:.0}", supplier.performance_score.responsiveness)}
                    </span>
                },
                view! {
                    <div class="trend-indicator">
                        {trend_chart(trend_data, Some(trend_height), None)}
                        <span class={format!("trend-arrow {}", trend_direction)}>
                            {match trend_direction {
                                "up" => "^",
                                "down" => "v",
                                _ => "-",
                            }}
                        </span>
                    </div>
                },
            ],
        }
    }).collect();

    // Icons
    let icon_star = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>"#;
    let icon_trending = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="23 6 13.5 15.5 8.5 10.5 1 18"/><polyline points="17 6 23 6 23 12"/></svg>"#;
    let icon_alert = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>"#;
    let icon_check = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>"#;

    // Pre-compute chart heights to avoid type inference issues in view! macro
    let chart_height: u32 = 180;

    // Pre-compute loading state and table content before view! block
    let is_loading = loading.get();
    let suppliers_count = filtered_suppliers.len();
    let table_content = if is_loading {
        view! { <div class="loading-overlay">"Loading performance data..."</div> }
    } else if filtered_suppliers.is_empty() {
        view! {
            <div class="loading-overlay">
                "No suppliers match the selected criteria."
            </div>
        }
    } else {
        view! {
            <div>
                {data_table(columns, rows, Some(handle_row_click))}
            </div>
        }
    };

    // Pre-compute top performers list
    let top_five: Vec<_> = active_suppliers.iter()
        .filter(|s| s.performance_score.overall >= 85.0)
        .take(5)
        .cloned()
        .collect();

    let top_performers_view = view! {
        <div class="top-performers-list">
            for supplier in top_five.iter() {
                <div class="top-performer-item">
                    <div class="top-performer-info">
                        <span class="top-performer-name">{supplier.name.clone()}</span>
                        <span class="top-performer-category">
                            {supplier.categories.first().map(|c| c.name.clone()).unwrap_or_default()}
                        </span>
                    </div>
                    <span class="top-performer-score">
                        {format!("{:.0}%", supplier.performance_score.overall)}
                    </span>
                </div>
            }
        </div>
    };

    view! {
        style {
            r#"
            .supplier-performance { display: flex; flex-direction: column; gap: var(--space-4); }
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
            .supplier-category {
                font-size: 11px;
                color: var(--text-muted);
            }
            .score-cell {
                font-weight: 600;
                font-family: IBM Plex Mono, monospace;
            }
            .trend-indicator {
                display: flex;
                align-items: center;
                gap: 8px;
            }
            .trend-arrow {
                font-weight: 600;
                font-size: 14px;
            }
            .trend-arrow.up { color: var(--green); }
            .trend-arrow.down { color: var(--red); }
            .trend-arrow.stable { color: var(--text-muted); }
            .loading-overlay {
                display: flex;
                align-items: center;
                justify-content: center;
                padding: 40px;
                color: var(--text-muted);
            }
            .performance-breakdown {
                display: grid;
                grid-template-columns: repeat(5, 1fr);
                gap: 16px;
                padding: 16px 0;
            }
            .breakdown-item {
                text-align: center;
            }
            .breakdown-label {
                font-size: 11px;
                color: var(--text-muted);
                margin-bottom: 8px;
            }
            .breakdown-value {
                font-size: 24px;
                font-weight: 600;
                font-family: IBM Plex Mono, monospace;
            }
            .top-performers-list {
                display: flex;
                flex-direction: column;
                gap: 12px;
            }
            .top-performer-item {
                display: flex;
                align-items: center;
                justify-content: space-between;
                padding: 12px;
                background: var(--bg);
                border-radius: var(--radius);
            }
            .top-performer-info {
                display: flex;
                flex-direction: column;
            }
            .top-performer-name {
                font-weight: 500;
            }
            .top-performer-category {
                font-size: 12px;
                color: var(--text-muted);
            }
            .top-performer-score {
                font-size: 20px;
                font-weight: 600;
                color: var(--green);
                font-family: IBM Plex Mono, monospace;
            }
            "#
        }

        <div class="supplier-performance" data-testid="supplier-performance">
            {page_header(
                "Supplier Performance".to_string(),
                Some("Monitor and analyze supplier performance metrics".to_string()),
                vec![
                    view! { <button class="btn btn-secondary">"Export Report"</button> },
                ]
            )}

            // KPI summary
            <div class="kpi-grid">
                {kpi_card(
                    "Avg. Performance".to_string(),
                    format!("{:.1}%", avg_performance),
                    KpiColor::Blue,
                    icon_star.to_string(),
                    None,
                    None
                )}
                {kpi_card(
                    "Top Performers".to_string(),
                    format!("{}", top_performers),
                    KpiColor::Green,
                    icon_trending.to_string(),
                    None,
                    None
                )}
                {kpi_card(
                    "Underperforming".to_string(),
                    format!("{}", underperformers),
                    KpiColor::Orange,
                    icon_alert.to_string(),
                    None,
                    None
                )}
                {kpi_card(
                    "Evaluated".to_string(),
                    format!("{}", active_suppliers.len()),
                    KpiColor::Accent,
                    icon_check.to_string(),
                    None,
                    None
                )}
            </div>

            // Charts row
            <div class="charts-grid">
                {panel(
                    "Performance Distribution".to_string(),
                    vec![],
                    vec![bar_chart(perf_distribution, Some(chart_height))]
                )}
                {panel(
                    "Performance by Category".to_string(),
                    vec![],
                    vec![bar_chart(category_performance, Some(chart_height))]
                )}
            </div>

            // Top performers
            {panel(
                "Top Performing Suppliers".to_string(),
                vec![view! { <a href="#/suppliers?filter=top" class="btn btn-sm btn-secondary">"View All"</a> }],
                vec![top_performers_view]
            )}

            // Filter bar
            {filter_bar(vec![
                view! {
                    <div class="filter-group">
                        <label>"Performance"</label>
                        <select on:change={handle_performance_filter}>
                            <option value="">"All Suppliers"</option>
                            <option value="top">"Top Performers (85%+)"</option>
                            <option value="underperforming">"Underperforming (<70%)"</option>
                        </select>
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"Category"</label>
                        <select on:change={handle_category_filter}>
                            <option value="">"All Categories"</option>
                            <option value="IT">"IT Services"</option>
                            <option value="CON">"Construction"</option>
                            <option value="SEC">"Security"</option>
                            <option value="CAT">"Catering"</option>
                            <option value="TRN">"Transport"</option>
                            <option value="PRO">"Professional Services"</option>
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

            // Performance table
            {panel(
                format!("Supplier Performance Scores ({} suppliers)", suppliers_count),
                vec![],
                vec![table_content]
            )}
        </div>
    }
}

/// Get color based on score value
fn score_color(score: f64) -> &'static str {
    if score >= 80.0 {
        "var(--green)"
    } else if score >= 70.0 {
        "var(--blue)"
    } else if score >= 60.0 {
        "var(--orange)"
    } else {
        "var(--red)"
    }
}
