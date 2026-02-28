//! Analytics Dashboard page

use components::prelude::*;
use crate::shared::layout::page_header;
use crate::shared::components::{
    kpi_card, KpiColor, KpiDelta,
    panel, data_table, DataTableColumn, DataTableRow,
    progress_bar, ProgressColor,
    tag, TagType,
};
use crate::shared::charts::{
    bar_chart, BarChartData,
    pie_chart, PieChartData,
    trend_chart, TrendChartData,
};
use crate::util::format::{format_currency, format_number, format_percentage};
use super::store::{AnalyticsStore, load_mock_data};
use super::service;

/// Analytics Dashboard page
#[component]
pub fn analytics_dashboard() -> View {
    let store = use_context::<AnalyticsStore>();

    // Load data on mount
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                service::load_analytics(&store).await;
            });
        }
    });

    let analytics = store.analytics.clone();
    let kpis = store.kpis.clone();

    // Icons
    let icon_dollar = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8"/><path d="M12 18V6"/></svg>"#;
    let icon_chart = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 20V10"/><path d="M12 20V4"/><path d="M6 20v-6"/></svg>"#;
    let icon_save = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/></svg>"#;
    let icon_users = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>"#;
    let icon_file = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><path d="M14 2v6h6"/><path d="M12 18v-6"/><path d="M9 15h6"/></svg>"#;
    let icon_clock = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>"#;
    let icon_star = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>"#;
    let icon_shield = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>"#;

    // Get data for charts
    let data = analytics.get();

    // Spend trend data
    let spend_trend: Vec<TrendChartData> = data.trends.monthly_spend.iter().map(|p| {
        TrendChartData {
            label: p.period.clone(),
            value: p.value,
        }
    }).collect();

    // Category breakdown bar chart
    let category_bars: Vec<BarChartData> = data.category_breakdown.iter().map(|c| {
        BarChartData {
            label: c.code.clone(),
            value: c.spend,
            color: Some(c.color.clone()),
        }
    }).collect();

    // Province distribution for bar chart
    let province_bars: Vec<BarChartData> = data.province_distribution.iter().take(5).map(|p| {
        BarChartData {
            label: p.province.chars().take(3).collect::<String>().to_uppercase(),
            value: p.spend,
            color: Some("var(--blue)".to_string()),
        }
    }).collect();

    // B-BBEE pie chart data
    let bbbee = &data.bbbee_metrics;
    let bbbee_pie: Vec<PieChartData> = vec![
        PieChartData { label: "Level 1".to_string(), value: bbbee.level_1_percent, color: "var(--green)".to_string() },
        PieChartData { label: "Level 2".to_string(), value: bbbee.level_2_percent, color: "var(--blue)".to_string() },
        PieChartData { label: "Level 3-4".to_string(), value: bbbee.level_3_4_percent, color: "var(--orange)".to_string() },
        PieChartData { label: "Level 5-8".to_string(), value: bbbee.level_5_8_percent, color: "var(--purple)".to_string() },
        PieChartData { label: "Non-Compliant".to_string(), value: bbbee.non_compliant_percent, color: "var(--text-muted)".to_string() },
    ];

    // Supplier table columns
    let supplier_columns = vec![
        DataTableColumn { key: "rank".to_string(), label: "#".to_string(), width: Some("40px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "supplier".to_string(), label: "Supplier".to_string(), width: None, align: None, cell_class: None },
        DataTableColumn { key: "spend".to_string(), label: "Total Spend".to_string(), width: None, align: Some("right".to_string()), cell_class: Some("amount-cell".to_string()) },
        DataTableColumn { key: "bbbee".to_string(), label: "B-BBEE".to_string(), width: Some("80px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "rating".to_string(), label: "Rating".to_string(), width: Some("80px".to_string()), align: Some("center".to_string()), cell_class: None },
    ];

    // Transform suppliers to table rows
    let supplier_rows: Vec<DataTableRow> = data.top_suppliers.iter().enumerate().map(|(idx, sup)| {
        let bbbee_tag = if sup.bbbee_level <= 2 {
            tag(format!("Level {}", sup.bbbee_level), TagType::Success)
        } else if sup.bbbee_level <= 4 {
            tag(format!("Level {}", sup.bbbee_level), TagType::Warning)
        } else {
            tag(format!("Level {}", sup.bbbee_level), TagType::Default)
        };

        let rating_color = if sup.overall_rating >= 90.0 {
            "var(--green)"
        } else if sup.overall_rating >= 80.0 {
            "var(--blue)"
        } else if sup.overall_rating >= 70.0 {
            "var(--orange)"
        } else {
            "var(--red)"
        };

        DataTableRow {
            id: sup.supplier_id.clone(),
            cells: vec![
                view! { <span class="rank">{(idx + 1).to_string()}</span> },
                view! { <span class="supplier-name">{sup.supplier_name.clone()}</span> },
                view! { <span class="amount-cell">{format_currency(sup.total_spend)}</span> },
                bbbee_tag,
                view! { <span style={format!("color: {}; font-weight: 600;", rating_color)}>{format!("{:.1}", sup.overall_rating)}</span> },
            ],
        }
    }).collect();

    // Category table columns
    let category_columns = vec![
        DataTableColumn { key: "category".to_string(), label: "Category".to_string(), width: None, align: None, cell_class: None },
        DataTableColumn { key: "spend".to_string(), label: "Spend".to_string(), width: None, align: Some("right".to_string()), cell_class: Some("amount-cell".to_string()) },
        DataTableColumn { key: "budget".to_string(), label: "Budget".to_string(), width: None, align: Some("right".to_string()), cell_class: Some("amount-cell".to_string()) },
        DataTableColumn { key: "variance".to_string(), label: "Variance".to_string(), width: None, align: Some("right".to_string()), cell_class: None },
        DataTableColumn { key: "percentage".to_string(), label: "% of Total".to_string(), width: Some("100px".to_string()), align: Some("center".to_string()), cell_class: None },
    ];

    // Category table rows
    let category_rows: Vec<DataTableRow> = data.category_breakdown.iter().map(|cat| {
        let variance_class = if cat.variance >= 0.0 { "positive" } else { "negative" };
        let variance_display = if cat.variance >= 0.0 {
            format!("+{}", format_currency(cat.variance))
        } else {
            format_currency(cat.variance)
        };

        DataTableRow {
            id: cat.code.clone(),
            cells: vec![
                view! {
                    <div class="category-cell">
                        <span class="color-dot" style={format!("background: {}", cat.color)}></span>
                        <span>{cat.name.clone()}</span>
                    </div>
                },
                view! { <span class="amount-cell">{format_currency(cat.spend)}</span> },
                view! { <span class="amount-cell">{format_currency(cat.budget)}</span> },
                view! { <span class={format!("variance {}", variance_class)}>{variance_display}</span> },
                view! { <span>{format_percentage(cat.percentage, 1)}</span> },
            ],
        }
    }).collect();

    view! {
        style {
            r#"
            .analytics-dashboard { display: flex; flex-direction: column; gap: var(--space-6); }
            .kpi-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; }
            @media (max-width: 1200px) { .kpi-grid { grid-template-columns: repeat(2, 1fr); } }
            @media (max-width: 768px) { .kpi-grid { grid-template-columns: 1fr; } }
            .charts-grid { display: grid; grid-template-columns: repeat(2, 1fr); gap: 24px; }
            @media (max-width: 1024px) { .charts-grid { grid-template-columns: 1fr; } }
            .category-cell { display: flex; align-items: center; gap: 8px; }
            .color-dot { width: 12px; height: 12px; border-radius: 50%; }
            .variance.positive { color: var(--green); }
            .variance.negative { color: var(--red); }
            .rank { color: var(--text-muted); font-weight: 600; }
            .supplier-name { font-weight: 500; }
            .amount-cell { font-family: IBM Plex Mono, monospace; }
            .bbbee-progress-section { margin-top: 24px; padding-top: 16px; border-top: 1px solid var(--border); }
            .bbbee-target-info { display: flex; justify-content: space-between; margin-bottom: 8px; font-size: 13px; }
            .bbbee-target-info .target { color: var(--text-muted); }
            .bbbee-target-info .current { font-weight: 600; color: var(--navy); }
            .filter-bar {
                display: flex;
                gap: 12px;
                padding: 12px 16px;
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius);
                margin-bottom: 8px;
            }
            .filter-bar select {
                padding: 8px 12px;
                border: 1px solid var(--border);
                border-radius: var(--radius-sm);
                background: var(--background);
                font-size: 13px;
                min-width: 150px;
            }
            .trend-section {
                display: flex;
                flex-direction: column;
                gap: 12px;
            }
            .trend-header {
                display: flex;
                justify-content: space-between;
                align-items: center;
            }
            .trend-legend {
                display: flex;
                gap: 16px;
                font-size: 12px;
            }
            .trend-legend-item {
                display: flex;
                align-items: center;
                gap: 6px;
            }
            .legend-color {
                width: 12px;
                height: 3px;
                border-radius: 2px;
            }
            .province-list {
                display: flex;
                flex-direction: column;
                gap: 8px;
            }
            .province-item {
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding: 8px 12px;
                background: var(--background);
                border-radius: var(--radius-sm);
            }
            .province-item .name { font-size: 13px; }
            .province-item .stats { display: flex; gap: 16px; font-size: 12px; color: var(--text-muted); }
            .province-item .amount { font-family: IBM Plex Mono, monospace; font-weight: 500; color: var(--navy); }
            "#
        }

        <div class="analytics-dashboard" data-testid="analytics-dashboard">
            {page_header(
                "Spend Analytics".to_string(),
                Some("Comprehensive procurement spend analysis for FY 2025/26".to_string()),
                vec![
                    view! { <button class="btn btn-secondary">"Export PDF"</button> },
                    view! { <button class="btn btn-secondary">"Export Excel"</button> },
                ]
            )}

            // Filter bar
            <div class="filter-bar">
                <select data-testid="period-filter">
                    <option value="year" selected>"FY 2025/26"</option>
                    <option value="quarter">"This Quarter"</option>
                    <option value="month">"This Month"</option>
                    <option value="custom">"Custom Range"</option>
                </select>
                <select data-testid="category-filter">
                    <option value="" selected>"All Categories"</option>
                    <option value="IT">"Information Technology"</option>
                    <option value="PRO">"Professional Services"</option>
                    <option value="FAC">"Facilities Management"</option>
                    <option value="SEC">"Security Services"</option>
                    <option value="FLT">"Fleet Management"</option>
                </select>
                <select data-testid="province-filter">
                    <option value="" selected>"All Provinces"</option>
                    <option value="GP">"Gauteng"</option>
                    <option value="WC">"Western Cape"</option>
                    <option value="KZN">"KwaZulu-Natal"</option>
                    <option value="EC">"Eastern Cape"</option>
                    <option value="MP">"Mpumalanga"</option>
                </select>
            </div>

            // KPI Row 1 - Financial Overview
            <div class="kpi-grid">
                {kpi_card(
                    "YTD Spend".to_string(),
                    format_currency(kpis.get().ytd_spend),
                    KpiColor::Blue,
                    icon_dollar.to_string(),
                    Some(KpiDelta { value: "3.2%".to_string(), is_positive: Some(false), suffix: "under budget".to_string() }),
                    None
                )}
                {kpi_card(
                    "Budget Utilization".to_string(),
                    format_percentage(kpis.get().budget_utilization, 1),
                    KpiColor::Green,
                    icon_chart.to_string(),
                    Some(KpiDelta { value: "R 153M".to_string(), is_positive: None, suffix: "remaining".to_string() }),
                    None
                )}
                {kpi_card(
                    "Cost Savings".to_string(),
                    format_currency(kpis.get().cost_savings),
                    KpiColor::Accent,
                    icon_save.to_string(),
                    Some(KpiDelta { value: "3.8%".to_string(), is_positive: Some(true), suffix: "of total spend".to_string() }),
                    None
                )}
                {kpi_card(
                    "Active Suppliers".to_string(),
                    format_number(data.active_suppliers),
                    KpiColor::Purple,
                    icon_users.to_string(),
                    Some(KpiDelta { value: "45".to_string(), is_positive: Some(true), suffix: "new this month".to_string() }),
                    None
                )}
            </div>

            // KPI Row 2 - Performance Metrics
            <div class="kpi-grid">
                {kpi_card(
                    "Active Contracts".to_string(),
                    data.active_contracts.to_string(),
                    KpiColor::Cyan,
                    icon_file.to_string(),
                    Some(KpiDelta { value: "12 expiring in 30 days".to_string(), is_positive: None, suffix: "".to_string() }),
                    None
                )}
                {kpi_card(
                    "Avg. Procurement Cycle".to_string(),
                    format!("{} days", kpis.get().avg_procurement_cycle),
                    KpiColor::Orange,
                    icon_clock.to_string(),
                    Some(KpiDelta { value: "5 days".to_string(), is_positive: Some(true), suffix: "faster than target".to_string() }),
                    None
                )}
                {kpi_card(
                    "Supplier Performance".to_string(),
                    format_percentage(kpis.get().supplier_performance, 1),
                    KpiColor::Green,
                    icon_star.to_string(),
                    Some(KpiDelta { value: "2.3%".to_string(), is_positive: Some(true), suffix: "from last quarter".to_string() }),
                    None
                )}
                {kpi_card(
                    "Compliance Score".to_string(),
                    format_percentage(kpis.get().compliance_score, 1),
                    KpiColor::Blue,
                    icon_shield.to_string(),
                    Some(KpiDelta { value: "7 issues".to_string(), is_positive: None, suffix: "require attention".to_string() }),
                    None
                )}
            </div>

            // Charts Row 1 - Spend Trends and Category Breakdown
            <div class="charts-grid">
                {panel(
                    "Monthly Spend Trend".to_string(),
                    vec![],
                    vec![
                        view! {
                            <div class="trend-section">
                                <div class="trend-header">
                                    <div class="trend-legend">
                                        <div class="trend-legend-item">
                                            <div class="legend-color" style="background: var(--blue);"></div>
                                            <span>"Actual Spend"</span>
                                        </div>
                                    </div>
                                </div>
                                {trend_chart(spend_trend, Some(180), Some("var(--blue)".to_string()))}
                            </div>
                        },
                    ]
                )}

                {panel(
                    "Spend by Category".to_string(),
                    vec![],
                    vec![bar_chart(category_bars, Some(200))]
                )}
            </div>

            // Charts Row 2 - B-BBEE and Province Distribution
            <div class="charts-grid">
                {panel(
                    "B-BBEE Spend Distribution".to_string(),
                    vec![view! { <a href="#" class="btn btn-sm btn-secondary">"View Details"</a> }],
                    vec![
                        pie_chart(bbbee_pie, None),
                        view! {
                            <div class="bbbee-progress-section">
                                <div class="bbbee-target-info">
                                    <span class="target">"Target: "{format_percentage(bbbee.bbbee_target, 0)}</span>
                                    <span class="current">"Current: "{format_percentage(bbbee.bbbee_actual, 1)}</span>
                                </div>
                                {progress_bar(bbbee.bbbee_actual, ProgressColor::Green, true, None)}
                            </div>
                        },
                    ]
                )}

                {panel(
                    "Provincial Distribution".to_string(),
                    vec![],
                    vec![
                        bar_chart(province_bars, Some(180)),
                        view! {
                            <div class="province-list" style="margin-top: 16px;">
                                for prov in data.province_distribution.iter().take(5) {
                                    <div class="province-item">
                                        <span class="name">{prov.province.clone()}</span>
                                        <div class="stats">
                                            <span class="amount">{format_currency(prov.spend)}</span>
                                            <span>{format!("{} suppliers", prov.supplier_count)}</span>
                                        </div>
                                    </div>
                                }
                            </div>
                        },
                    ]
                )}
            </div>

            // Tables Row - Category Details and Top Suppliers
            <div class="charts-grid">
                {panel(
                    "Category Breakdown".to_string(),
                    vec![view! { <a href="#" class="btn btn-sm btn-secondary">"View All"</a> }],
                    vec![data_table(category_columns, category_rows, None)]
                )}

                {panel(
                    "Top Suppliers by Spend".to_string(),
                    vec![view! { <a href="#" class="btn btn-sm btn-secondary">"View All"</a> }],
                    vec![data_table(supplier_columns, supplier_rows, None)]
                )}
            </div>
        </div>
    }
}
