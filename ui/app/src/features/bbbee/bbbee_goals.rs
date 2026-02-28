//! B-BBEE Goals Dashboard
//!
//! Dashboard showing B-BBEE spend targets vs actuals, level distribution,
//! supplier breakdown, and compliance metrics per South African B-BBEE Codes.

use components::prelude::*;
use crate::shared::layout::page_header;
use crate::shared::components::{
    kpi_card, KpiColor, KpiDelta,
    panel, data_table, DataTableColumn, DataTableRow,
    progress_bar, ProgressColor,
    tag, TagType,
    status_badge, StatusType,
};
use crate::shared::charts::{
    bar_chart, BarChartData,
    pie_chart, PieChartData,
    trend_chart, TrendChartData,
};
use crate::util::format::{format_currency, format_number, format_percentage};
use super::store::BbbeeStore;
use super::types::{
    BbbeeLevel, OwnershipClassification, TargetStatus, Trend,
    ComplianceStatus, VerificationStatus, EnterpriseSize,
};
use super::service;

/// B-BBEE Goals Dashboard page
#[component]
pub fn bbbee_goals() -> View {
    let store = use_context::<BbbeeStore>();

    // Tab state
    let active_tab = signal("overview".to_string());

    // Load data on mount
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                service::load_bbbee_data(&store).await;
            });
        }
    });

    let kpis = store.kpis.clone();
    let spend_targets = store.spend_targets.clone();
    let level_breakdown = store.level_breakdown.clone();
    let compliance_metrics = store.compliance_metrics.clone();
    let supplier_classifications = store.supplier_classifications.clone();
    let trend_data = store.trend_data.clone();
    let designated_group_spend = store.designated_group_spend.clone();
    let provincial_distribution = store.provincial_distribution.clone();
    let loading = store.loading.clone();

    // Tab handlers
    let set_tab_overview = Callback::new({
        let active_tab = active_tab.clone();
        move |_| active_tab.set("overview".to_string())
    });
    let set_tab_targets = Callback::new({
        let active_tab = active_tab.clone();
        move |_| active_tab.set("targets".to_string())
    });
    let set_tab_suppliers = Callback::new({
        let active_tab = active_tab.clone();
        move |_| active_tab.set("suppliers".to_string())
    });
    let set_tab_metrics = Callback::new({
        let active_tab = active_tab.clone();
        move |_| active_tab.set("metrics".to_string())
    });

    // Get computed values
    let kpi_data = kpis.get();
    let targets = spend_targets.get();
    let levels = level_breakdown.get();
    let metrics = compliance_metrics.get();
    let suppliers = supplier_classifications.get();
    let trends = trend_data.get();
    let designated = designated_group_spend.get();
    let provinces = provincial_distribution.get();

    // B-BBEE spend trend for chart
    let spend_trend: Vec<TrendChartData> = trends.iter().map(|t| {
        TrendChartData {
            label: t.period.clone(),
            value: t.bbbee_percent,
        }
    }).collect();

    // Level distribution pie chart
    let level_pie: Vec<PieChartData> = levels.iter()
        .filter(|l| l.spend_percentage > 0.5)
        .map(|l| {
            PieChartData {
                label: l.level.label().to_string(),
                value: l.spend_percentage,
                color: l.level.color().to_string(),
            }
        }).collect();

    // Level distribution bar chart
    let level_bars: Vec<BarChartData> = levels.iter()
        .filter(|l| l.level != BbbeeLevel::NonCompliant)
        .take(5)
        .map(|l| {
            BarChartData {
                label: format!("L{}", l.level.to_u8().unwrap_or(0)),
                value: l.spend_amount,
                color: Some(l.level.color().to_string()),
            }
        }).collect();

    // Designated group bar chart
    let designated_bars: Vec<BarChartData> = designated.iter().take(6).map(|d| {
        let label = match d.classification {
            OwnershipClassification::BlackOwned => "BO".to_string(),
            OwnershipClassification::BlackWomenOwned => "BWO".to_string(),
            OwnershipClassification::EME => "EME".to_string(),
            OwnershipClassification::QSE => "QSE".to_string(),
            OwnershipClassification::YouthOwned => "Youth".to_string(),
            OwnershipClassification::RuralTownship => "Rural".to_string(),
            _ => "Other".to_string(),
        };
        BarChartData {
            label,
            value: d.spend_percentage,
            color: Some(if d.variance >= 0.0 { "var(--green)".to_string() } else { "var(--orange)".to_string() }),
        }
    }).collect();

    // Spend targets table columns
    let target_columns = vec![
        DataTableColumn { key: "name".to_string(), label: "Target".to_string(), width: None, align: None, cell_class: None },
        DataTableColumn { key: "target".to_string(), label: "Target".to_string(), width: Some("80px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "actual".to_string(), label: "Actual".to_string(), width: Some("80px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "variance".to_string(), label: "Variance".to_string(), width: Some("90px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "status".to_string(), label: "Status".to_string(), width: Some("110px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "progress".to_string(), label: "Progress".to_string(), width: Some("150px".to_string()), align: None, cell_class: None },
    ];

    // Transform targets to table rows
    let target_rows: Vec<DataTableRow> = targets.iter().map(|target| {
        let status_tag = match target.status {
            TargetStatus::Exceeding => tag("Exceeding".to_string(), TagType::Green),
            TargetStatus::OnTrack => tag("On Track".to_string(), TagType::Blue),
            TargetStatus::AtRisk => tag("At Risk".to_string(), TagType::Orange),
            TargetStatus::BelowTarget => tag("Below".to_string(), TagType::Red),
            TargetStatus::NotApplicable => tag("N/A".to_string(), TagType::Default),
        };

        let variance_class = if target.variance >= 0.0 { "positive" } else { "negative" };
        let variance_text = if target.variance >= 0.0 {
            format!("+{:.1}%", target.variance)
        } else {
            format!("{:.1}%", target.variance)
        };

        let progress_color = match target.status {
            TargetStatus::Exceeding => ProgressColor::Green,
            TargetStatus::OnTrack => ProgressColor::Blue,
            TargetStatus::AtRisk => ProgressColor::Orange,
            TargetStatus::BelowTarget => ProgressColor::Red,
            TargetStatus::NotApplicable => ProgressColor::Gray,
        };

        let progress_value: f64 = (target.actual_percentage / target.target_percentage * 100.0).min(100.0);

        DataTableRow {
            id: target.id.clone(),
            cells: vec![
                view! {
                    <div class="target-info">
                        <span class="target-name">{target.name.clone()}</span>
                        <span class="target-category">{target.category.clone()}</span>
                    </div>
                },
                view! { <span class="percentage-cell">{format_percentage(target.target_percentage, 1)}</span> },
                view! { <span class="percentage-cell">{format_percentage(target.actual_percentage, 1)}</span> },
                view! { <span class={format!("variance {}", variance_class)}>{variance_text}</span> },
                status_tag,
                view! { <div class="progress-cell">{progress_bar(progress_value, progress_color, false, None)}</div> },
            ],
        }
    }).collect();

    // Level breakdown table columns
    let level_columns = vec![
        DataTableColumn { key: "level".to_string(), label: "B-BBEE Level".to_string(), width: Some("120px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "suppliers".to_string(), label: "Suppliers".to_string(), width: Some("90px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "spend".to_string(), label: "Spend".to_string(), width: None, align: Some("right".to_string()), cell_class: Some("amount-cell".to_string()) },
        DataTableColumn { key: "percent".to_string(), label: "% of Total".to_string(), width: Some("90px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "recognition".to_string(), label: "Recognition".to_string(), width: Some("100px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "recognized".to_string(), label: "Recognized Spend".to_string(), width: None, align: Some("right".to_string()), cell_class: Some("amount-cell".to_string()) },
    ];

    // Transform level breakdown to table rows
    let level_rows: Vec<DataTableRow> = levels.iter().map(|level| {
        let level_tag = match level.level {
            BbbeeLevel::Level1 | BbbeeLevel::Level2 => tag(level.level.label().to_string(), TagType::Green),
            BbbeeLevel::Level3 | BbbeeLevel::Level4 => tag(level.level.label().to_string(), TagType::Blue),
            BbbeeLevel::Level5 | BbbeeLevel::Level6 => tag(level.level.label().to_string(), TagType::Orange),
            BbbeeLevel::Level7 | BbbeeLevel::Level8 => tag(level.level.label().to_string(), TagType::Red),
            BbbeeLevel::NonCompliant => tag(level.level.label().to_string(), TagType::Default),
        };

        DataTableRow {
            id: format!("{:?}", level.level),
            cells: vec![
                level_tag,
                view! { <span>{level.supplier_count.to_string()}</span> },
                view! { <span class="amount-cell">{format_currency(level.spend_amount)}</span> },
                view! { <span>{format_percentage(level.spend_percentage, 1)}</span> },
                view! { <span class="recognition-level">{format!("{:.0}%", level.level.recognition_level())}</span> },
                view! { <span class="amount-cell">{format_currency(level.recognized_spend)}</span> },
            ],
        }
    }).collect();

    // Supplier table columns
    let supplier_columns = vec![
        DataTableColumn { key: "name".to_string(), label: "Supplier".to_string(), width: None, align: None, cell_class: None },
        DataTableColumn { key: "level".to_string(), label: "B-BBEE".to_string(), width: Some("90px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "ownership".to_string(), label: "Ownership".to_string(), width: Some("120px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "size".to_string(), label: "Size".to_string(), width: Some("100px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "spend".to_string(), label: "Spend".to_string(), width: None, align: Some("right".to_string()), cell_class: Some("amount-cell".to_string()) },
        DataTableColumn { key: "status".to_string(), label: "Verified".to_string(), width: Some("100px".to_string()), align: None, cell_class: None },
    ];

    // Transform suppliers to table rows
    let supplier_rows: Vec<DataTableRow> = suppliers.iter().map(|sup| {
        let level_tag = match sup.bbbee_level {
            BbbeeLevel::Level1 | BbbeeLevel::Level2 => tag(sup.bbbee_level.label().to_string(), TagType::Green),
            BbbeeLevel::Level3 | BbbeeLevel::Level4 => tag(sup.bbbee_level.label().to_string(), TagType::Blue),
            BbbeeLevel::Level5 | BbbeeLevel::Level6 => tag(sup.bbbee_level.label().to_string(), TagType::Orange),
            _ => tag(sup.bbbee_level.label().to_string(), TagType::Default),
        };

        let ownership_text = if sup.ownership_classifications.contains(&OwnershipClassification::BlackWomenOwned) {
            "BWO".to_string()
        } else if sup.ownership_classifications.contains(&OwnershipClassification::BlackOwned) {
            "BO".to_string()
        } else {
            "-".to_string()
        };

        let size_tag = match sup.enterprise_size {
            EnterpriseSize::EME => tag("EME".to_string(), TagType::Purple),
            EnterpriseSize::QSE => tag("QSE".to_string(), TagType::Cyan),
            EnterpriseSize::Generic => tag("Generic".to_string(), TagType::Default),
        };

        let status = match sup.verification_status {
            VerificationStatus::Verified => status_badge(StatusType::Approved),
            VerificationStatus::Expired => status_badge(StatusType::Rejected),
            VerificationStatus::Pending => status_badge(StatusType::Pending),
            VerificationStatus::SwornAffidavit => status_badge(StatusType::Active),
            VerificationStatus::Unverified => status_badge(StatusType::Draft),
        };

        DataTableRow {
            id: sup.supplier_id.clone(),
            cells: vec![
                view! {
                    <div class="supplier-info">
                        <span class="supplier-name">{sup.supplier_name.clone()}</span>
                        <span class="supplier-province">{sup.province.clone()}</span>
                    </div>
                },
                level_tag,
                view! { <span class="ownership-badge">{ownership_text}</span> },
                size_tag,
                view! { <span class="amount-cell">{format_currency(sup.total_spend)}</span> },
                status,
            ],
        }
    }).collect();

    // Compliance metrics table columns
    let metric_columns = vec![
        DataTableColumn { key: "name".to_string(), label: "Metric".to_string(), width: None, align: None, cell_class: None },
        DataTableColumn { key: "element".to_string(), label: "Element".to_string(), width: Some("120px".to_string()), align: None, cell_class: None },
        DataTableColumn { key: "target".to_string(), label: "Target".to_string(), width: Some("80px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "actual".to_string(), label: "Actual".to_string(), width: Some("80px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "points".to_string(), label: "Points".to_string(), width: Some("80px".to_string()), align: Some("center".to_string()), cell_class: None },
        DataTableColumn { key: "status".to_string(), label: "Status".to_string(), width: Some("110px".to_string()), align: None, cell_class: None },
    ];

    // Transform metrics to table rows
    let metric_rows: Vec<DataTableRow> = metrics.iter().map(|metric| {
        let status_tag = match metric.status {
            ComplianceStatus::Compliant => tag("Compliant".to_string(), TagType::Green),
            ComplianceStatus::PartiallyCompliant => tag("Partial".to_string(), TagType::Orange),
            ComplianceStatus::NonCompliant => tag("Non-Compliant".to_string(), TagType::Red),
            ComplianceStatus::PendingReview => tag("Pending".to_string(), TagType::Blue),
            ComplianceStatus::NotApplicable => tag("N/A".to_string(), TagType::Default),
        };

        let points_color = if metric.points_achieved >= metric.weighting_points * 0.9 {
            "var(--green)"
        } else if metric.points_achieved >= metric.weighting_points * 0.7 {
            "var(--orange)"
        } else {
            "var(--red)"
        };

        DataTableRow {
            id: metric.id.clone(),
            cells: vec![
                view! {
                    <div class="metric-info">
                        <span class="metric-name">{metric.name.clone()}</span>
                        <span class="metric-desc">{metric.description.chars().take(50).collect::<String>()}{"..."}</span>
                    </div>
                },
                view! { <span class="element-label">{metric.scorecard_element.label()}</span> },
                view! { <span>{format!("{:.0}{}", metric.target_value, metric.unit)}</span> },
                view! { <span>{format!("{:.1}{}", metric.actual_value, metric.unit)}</span> },
                view! {
                    <span style={format!("color: {}; font-weight: 600;", points_color)}>
                        {format!("{:.1}/{:.0}", metric.points_achieved, metric.weighting_points)}
                    </span>
                },
                status_tag,
            ],
        }
    }).collect();

    // Icons
    let icon_target = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><circle cx="12" cy="12" r="6"/><circle cx="12" cy="12" r="2"/></svg>"#;
    let icon_users = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>"#;
    let icon_chart = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 20V10"/><path d="M12 20V4"/><path d="M6 20v-6"/></svg>"#;
    let icon_check = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>"#;
    let icon_alert = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>"#;
    let icon_star = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>"#;
    let icon_shield = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>"#;
    let icon_trending = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="23 6 13.5 15.5 8.5 10.5 1 18"/><polyline points="17 6 23 6 23 12"/></svg>"#;

    // Pre-compute values for type inference in view! macro
    let level_1_2_spend: f64 = kpi_data.total_bbbee_spend * kpi_data.level_1_2_percent / 100.0;
    let scorecard_percentage: f64 = if kpi_data.max_scorecard_points > 0.0 {
        kpi_data.scorecard_points / kpi_data.max_scorecard_points * 100.0
    } else {
        0.0
    };

    // Pre-compute target card data for the first 3 targets
    let target_cards: Vec<(String, TargetStatus, f64, f64, f64)> = targets.iter().take(3).map(|t| {
        let progress: f64 = if t.target_percentage > 0.0 {
            (t.actual_percentage / t.target_percentage * 100.0).min(100.0)
        } else {
            0.0
        };
        (t.name.clone(), t.status.clone(), t.target_percentage, t.actual_percentage, progress)
    }).collect();

    view! {
        style {
            r#"
            .bbbee-goals { display: flex; flex-direction: column; gap: var(--space-4); }
            .kpi-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; }
            @media (max-width: 1200px) { .kpi-grid { grid-template-columns: repeat(2, 1fr); } }
            @media (max-width: 768px) { .kpi-grid { grid-template-columns: 1fr; } }
            .charts-grid { display: grid; grid-template-columns: repeat(2, 1fr); gap: 24px; }
            @media (max-width: 1024px) { .charts-grid { grid-template-columns: 1fr; } }
            .grid-3 { display: grid; grid-template-columns: repeat(3, 1fr); gap: 24px; }
            @media (max-width: 1200px) { .grid-3 { grid-template-columns: 1fr 1fr; } }
            @media (max-width: 768px) { .grid-3 { grid-template-columns: 1fr; } }
            .tab-bar {
                display: flex;
                gap: 4px;
                border-bottom: 1px solid var(--border);
                padding: 0 4px;
                margin-bottom: 16px;
            }
            .tab-button {
                padding: 12px 20px;
                border: none;
                background: transparent;
                color: var(--text-muted);
                font-size: 14px;
                font-weight: 500;
                cursor: pointer;
                border-bottom: 2px solid transparent;
                margin-bottom: -1px;
                transition: all 0.2s;
            }
            .tab-button:hover {
                color: var(--text);
                background: var(--bg);
            }
            .tab-button.active {
                color: var(--blue);
                border-bottom-color: var(--blue);
            }
            .target-info, .supplier-info, .metric-info {
                display: flex;
                flex-direction: column;
                gap: 2px;
            }
            .target-name, .supplier-name, .metric-name {
                font-weight: 500;
                color: var(--text);
            }
            .target-category, .supplier-province, .metric-desc {
                font-size: 11px;
                color: var(--text-muted);
            }
            .percentage-cell {
                font-family: IBM Plex Mono, monospace;
                font-weight: 500;
            }
            .variance.positive { color: var(--green); font-weight: 600; }
            .variance.negative { color: var(--red); font-weight: 600; }
            .amount-cell {
                font-family: IBM Plex Mono, monospace;
            }
            .recognition-level {
                font-family: IBM Plex Mono, monospace;
                font-weight: 500;
                color: var(--blue);
            }
            .progress-cell { min-width: 120px; }
            .ownership-badge {
                padding: 2px 8px;
                background: var(--blue-light);
                color: var(--blue);
                border-radius: 4px;
                font-size: 11px;
                font-weight: 600;
            }
            .element-label {
                font-size: 11px;
                color: var(--text-muted);
            }
            .loading-overlay {
                display: flex;
                align-items: center;
                justify-content: center;
                padding: 40px;
                color: var(--text-muted);
            }
            .scorecard-summary {
                display: grid;
                grid-template-columns: auto 1fr auto;
                gap: 24px;
                align-items: center;
                padding: 20px;
                background: linear-gradient(135deg, var(--blue-light) 0%, var(--green-light) 100%);
                border-radius: var(--radius);
                margin-bottom: 16px;
            }
            .scorecard-level {
                display: flex;
                flex-direction: column;
                align-items: center;
                padding: 20px;
                background: white;
                border-radius: var(--radius);
                box-shadow: 0 2px 8px #0000001A;
            }
            .level-badge {
                font-size: 32px;
                font-weight: 700;
                color: var(--green);
            }
            .level-label {
                font-size: 12px;
                color: var(--text-muted);
                margin-top: 4px;
            }
            .scorecard-info {
                display: flex;
                flex-direction: column;
                gap: 8px;
            }
            .scorecard-title {
                font-size: 18px;
                font-weight: 600;
                color: var(--navy);
            }
            .scorecard-subtitle {
                font-size: 13px;
                color: var(--text-muted);
            }
            .scorecard-points {
                display: flex;
                flex-direction: column;
                align-items: flex-end;
            }
            .points-value {
                font-size: 28px;
                font-weight: 700;
                color: var(--navy);
            }
            .points-max {
                font-size: 12px;
                color: var(--text-muted);
            }
            .target-progress-card {
                display: flex;
                flex-direction: column;
                gap: 16px;
                padding: 16px;
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius);
            }
            .target-header {
                display: flex;
                justify-content: space-between;
                align-items: center;
            }
            .target-title { font-weight: 600; color: var(--text); }
            .target-values {
                display: flex;
                gap: 16px;
                font-size: 13px;
            }
            .target-values .target { color: var(--text-muted); }
            .target-values .actual { font-weight: 600; }
            .provincial-list {
                display: flex;
                flex-direction: column;
                gap: 8px;
            }
            .provincial-item {
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding: 10px 12px;
                background: var(--background);
                border-radius: var(--radius-sm);
            }
            .provincial-name { font-size: 13px; font-weight: 500; }
            .provincial-stats {
                display: flex;
                gap: 16px;
                font-size: 12px;
                color: var(--text-muted);
            }
            .provincial-stats .bbbee-percent {
                font-weight: 600;
                color: var(--green);
            }
            .alert-banner {
                display: flex;
                align-items: center;
                gap: 12px;
                padding: 12px 16px;
                background: #fff3cd;
                border: 1px solid #ffc107;
                border-radius: var(--radius);
                margin-bottom: 16px;
            }
            .alert-banner.success {
                background: #d4edda;
                border-color: #28a745;
            }
            .alert-banner .icon {
                width: 24px;
                height: 24px;
                color: #856404;
            }
            .alert-banner.success .icon { color: #155724; }
            .alert-banner .message { flex: 1; font-size: 13px; }
            .alert-banner .action {
                padding: 6px 12px;
                background: white;
                border: 1px solid var(--border);
                border-radius: var(--radius-sm);
                font-size: 12px;
                cursor: pointer;
            }
            "#
        }

        <div class="bbbee-goals" data-testid="bbbee-goals">
            {page_header(
                "B-BBEE Goals".to_string(),
                Some("Broad-Based Black Economic Empowerment compliance tracking for FY 2025/26".to_string()),
                vec![
                    view! { <button class="btn btn-secondary">"Export DTI Report"</button> },
                    view! { <button class="btn btn-secondary">"Export Excel"</button> },
                ]
            )}

            // Alert banner for expiring certificates
            if kpi_data.expiring_certificates > 0 {
                <div class="alert-banner">
                    <span class="icon" inner_html={icon_alert}></span>
                    <span class="message">
                        <strong>{kpi_data.expiring_certificates.to_string()}</strong>
                        " supplier B-BBEE certificates expiring within 90 days"
                    </span>
                    <button class="action">"View List"</button>
                </div>
            }

            // Scorecard summary banner
            <div class="scorecard-summary">
                <div class="scorecard-level">
                    <span class="level-badge">{kpi_data.projected_level.label()}</span>
                    <span class="level-label">"Projected Level"</span>
                </div>
                <div class="scorecard-info">
                    <span class="scorecard-title">"B-BBEE Preferential Procurement Scorecard"</span>
                    <span class="scorecard-subtitle">"Enterprise & Supplier Development Element - Code 400"</span>
                </div>
                <div class="scorecard-points">
                    <span class="points-value">{format!("{:.1}", kpi_data.scorecard_points)}</span>
                    <span class="points-max">{format!("of {:.0} points", kpi_data.max_scorecard_points)}</span>
                </div>
            </div>

            // Tab navigation
            <div class="tab-bar">
                <button
                    class={if active_tab.get() == "overview" { "tab-button active" } else { "tab-button" }}
                    on:click={set_tab_overview}
                >"Overview"</button>
                <button
                    class={if active_tab.get() == "targets" { "tab-button active" } else { "tab-button" }}
                    on:click={set_tab_targets}
                >"Spend Targets"</button>
                <button
                    class={if active_tab.get() == "suppliers" { "tab-button active" } else { "tab-button" }}
                    on:click={set_tab_suppliers}
                >"Suppliers"</button>
                <button
                    class={if active_tab.get() == "metrics" { "tab-button active" } else { "tab-button" }}
                    on:click={set_tab_metrics}
                >"Compliance Metrics"</button>
            </div>

            // Overview tab content
            if active_tab.get() == "overview" {
                <div class="tab-content">
                    // KPI Row
                    <div class="kpi-grid">
                        {kpi_card(
                            "B-BBEE Spend".to_string(),
                            format_percentage(kpi_data.overall_bbbee_percent, 1),
                            if kpi_data.overall_bbbee_percent >= kpi_data.target_bbbee_percent { KpiColor::Green } else { KpiColor::Orange },
                            icon_target.to_string(),
                            Some(KpiDelta {
                                value: format!("{}%", kpi_data.target_bbbee_percent),
                                is_positive: None,
                                suffix: "target".to_string(),
                            }),
                            None
                        )}
                        {kpi_card(
                            "Compliant Suppliers".to_string(),
                            format_number(kpi_data.compliant_supplier_count),
                            KpiColor::Blue,
                            icon_users.to_string(),
                            Some(KpiDelta {
                                value: kpi_data.non_compliant_supplier_count.to_string(),
                                is_positive: if kpi_data.non_compliant_supplier_count > 0 { Some(false) } else { None },
                                suffix: "non-compliant".to_string(),
                            }),
                            None
                        )}
                        {kpi_card(
                            "Level 1-2 Spend".to_string(),
                            format_percentage(kpi_data.level_1_2_percent, 1),
                            KpiColor::Green,
                            icon_star.to_string(),
                            Some(KpiDelta {
                                value: format_currency(level_1_2_spend),
                                is_positive: None,
                                suffix: "".to_string(),
                            }),
                            None
                        )}
                        {kpi_card(
                            "Black Owned Spend".to_string(),
                            format_percentage(kpi_data.black_owned_percent, 1),
                            if kpi_data.black_owned_percent >= 40.0 { KpiColor::Green } else { KpiColor::Orange },
                            icon_chart.to_string(),
                            Some(KpiDelta {
                                value: "40%".to_string(),
                                is_positive: None,
                                suffix: "target".to_string(),
                            }),
                            None
                        )}
                    </div>

                    // Second KPI Row
                    <div class="kpi-grid">
                        {kpi_card(
                            "Black Women Owned".to_string(),
                            format_percentage(kpi_data.black_women_owned_percent, 1),
                            if kpi_data.black_women_owned_percent >= 12.0 { KpiColor::Green } else { KpiColor::Orange },
                            icon_users.to_string(),
                            Some(KpiDelta {
                                value: "12%".to_string(),
                                is_positive: Some(kpi_data.black_women_owned_percent >= 12.0),
                                suffix: "target".to_string(),
                            }),
                            None
                        )}
                        {kpi_card(
                            "EME/QSE Spend".to_string(),
                            format_percentage(kpi_data.eme_qse_percent, 1),
                            if kpi_data.eme_qse_percent >= 30.0 { KpiColor::Green } else { KpiColor::Orange },
                            icon_trending.to_string(),
                            Some(KpiDelta {
                                value: "30%".to_string(),
                                is_positive: None,
                                suffix: "target".to_string(),
                            }),
                            None
                        )}
                        {kpi_card(
                            "Designated Groups".to_string(),
                            format_percentage(kpi_data.designated_group_percent, 1),
                            KpiColor::Accent,
                            icon_shield.to_string(),
                            Some(KpiDelta {
                                value: "2%".to_string(),
                                is_positive: Some(kpi_data.designated_group_percent >= 2.0),
                                suffix: "target".to_string(),
                            }),
                            None
                        )}
                        {kpi_card(
                            "Recognized Spend".to_string(),
                            format_currency(kpi_data.total_recognized_spend),
                            KpiColor::Purple,
                            icon_check.to_string(),
                            Some(KpiDelta {
                                value: format!("+{}", format_currency(kpi_data.total_recognized_spend - kpi_data.total_bbbee_spend)),
                                is_positive: Some(true),
                                suffix: "bonus".to_string(),
                            }),
                            None
                        )}
                    </div>

                    // Charts Row
                    <div class="charts-grid">
                        {panel(
                            "B-BBEE Spend Trend".to_string(),
                            vec![],
                            vec![
                                view! {
                                    <div class="target-values" style="margin-bottom: 12px;">
                                        <span class="target">"Target: "{format_percentage(80.0, 0)}</span>
                                        <span class="actual">"Current: "{format_percentage(kpi_data.overall_bbbee_percent, 1)}</span>
                                    </div>
                                },
                                trend_chart(spend_trend, Some(180), Some("var(--green)".to_string())),
                            ]
                        )}

                        {panel(
                            "Spend by B-BBEE Level".to_string(),
                            vec![],
                            vec![pie_chart(level_pie.clone(), Some(180))]
                        )}
                    </div>

                    // Second Charts Row
                    <div class="charts-grid">
                        {panel(
                            "Level Distribution".to_string(),
                            vec![view! { <a href="#" class="btn btn-sm btn-secondary">"View Details"</a> }],
                            vec![bar_chart(level_bars, Some(180))]
                        )}

                        {panel(
                            "Designated Group Spend (%)".to_string(),
                            vec![],
                            vec![bar_chart(designated_bars, Some(180))]
                        )}
                    </div>

                    // Provincial Distribution
                    {panel(
                        "Provincial B-BBEE Distribution".to_string(),
                        vec![],
                        vec![view! {
                            <div class="provincial-list">
                                for prov in provinces.iter().take(5) {
                                    <div class="provincial-item">
                                        <span class="provincial-name">{prov.province.clone()}</span>
                                        <div class="provincial-stats">
                                            <span class="bbbee-percent">{format_percentage(prov.bbbee_percentage, 0)}" B-BBEE"</span>
                                            <span>{format!("{} suppliers", prov.supplier_count)}</span>
                                            <span>{format_currency(prov.total_spend)}</span>
                                        </div>
                                    </div>
                                }
                            </div>
                        }]
                    )}
                </div>
            }

            // Spend Targets tab content
            if active_tab.get() == "targets" {
                <div class="tab-content">
                    // Target progress cards
                    <div class="grid-3">
                        for (name, status, target_pct, actual_pct, progress) in target_cards.iter() {
                            <div class="target-progress-card">
                                <div class="target-header">
                                    <span class="target-title">{name.clone()}</span>
                                    {match status {
                                        TargetStatus::Exceeding => tag("Exceeding".to_string(), TagType::Green),
                                        TargetStatus::OnTrack => tag("On Track".to_string(), TagType::Blue),
                                        TargetStatus::AtRisk => tag("At Risk".to_string(), TagType::Orange),
                                        TargetStatus::BelowTarget => tag("Below".to_string(), TagType::Red),
                                        TargetStatus::NotApplicable => tag("N/A".to_string(), TagType::Default),
                                    }}
                                </div>
                                <div class="target-values">
                                    <span class="target">"Target: "{format_percentage(*target_pct, 0)}</span>
                                    <span class="actual">"Actual: "{format_percentage(*actual_pct, 1)}</span>
                                </div>
                                {progress_bar(
                                    *progress,
                                    match status {
                                        TargetStatus::Exceeding => ProgressColor::Green,
                                        TargetStatus::OnTrack => ProgressColor::Blue,
                                        TargetStatus::AtRisk => ProgressColor::Orange,
                                        _ => ProgressColor::Red,
                                    },
                                    true,
                                    None
                                )}
                            </div>
                        }
                    </div>

                    // Full targets table
                    {panel(
                        "All Spend Targets".to_string(),
                        vec![view! { <button class="btn btn-sm btn-primary">"Add Target"</button> }],
                        vec![
                            if loading.get() {
                                view! { <div class="loading-overlay">"Loading targets..."</div> }
                            } else {
                                view! { <div>{data_table(target_columns.clone(), target_rows.clone(), None)}</div> }
                            }
                        ]
                    )}

                    // Level breakdown table
                    {panel(
                        "B-BBEE Level Breakdown".to_string(),
                        vec![],
                        vec![
                            if loading.get() {
                                view! { <div class="loading-overlay">"Loading level data..."</div> }
                            } else {
                                view! { <div>{data_table(level_columns.clone(), level_rows.clone(), None)}</div> }
                            }
                        ]
                    )}
                </div>
            }

            // Suppliers tab content
            if active_tab.get() == "suppliers" {
                <div class="tab-content">
                    // Supplier KPIs
                    <div class="kpi-grid">
                        {kpi_card(
                            "Total Suppliers".to_string(),
                            format_number(kpi_data.compliant_supplier_count + kpi_data.non_compliant_supplier_count),
                            KpiColor::Blue,
                            icon_users.to_string(),
                            None,
                            None
                        )}
                        {kpi_card(
                            "Level 1-2".to_string(),
                            format!("{}", levels.iter().filter(|l| l.level == BbbeeLevel::Level1 || l.level == BbbeeLevel::Level2).map(|l| l.supplier_count).sum::<u32>()),
                            KpiColor::Green,
                            icon_star.to_string(),
                            None,
                            None
                        )}
                        {kpi_card(
                            "EME Suppliers".to_string(),
                            format!("{}", suppliers.iter().filter(|s| s.is_eme).count()),
                            KpiColor::Purple,
                            icon_trending.to_string(),
                            None,
                            None
                        )}
                        {kpi_card(
                            "Expiring Certs".to_string(),
                            format!("{}", kpi_data.expiring_certificates),
                            if kpi_data.expiring_certificates > 0 { KpiColor::Orange } else { KpiColor::Green },
                            icon_alert.to_string(),
                            None,
                            None
                        )}
                    </div>

                    // Suppliers table
                    {panel(
                        format!("B-BBEE Supplier Classifications ({} total)", suppliers.len()),
                        vec![
                            view! { <button class="btn btn-sm btn-secondary">"Filter"</button> },
                            view! { <button class="btn btn-sm btn-primary">"Export"</button> },
                        ],
                        vec![
                            if loading.get() {
                                view! { <div class="loading-overlay">"Loading suppliers..."</div> }
                            } else {
                                view! { <div>{data_table(supplier_columns.clone(), supplier_rows.clone(), None)}</div> }
                            }
                        ]
                    )}
                </div>
            }

            // Compliance Metrics tab content
            if active_tab.get() == "metrics" {
                <div class="tab-content">
                    // Metrics summary
                    <div class="kpi-grid">
                        {kpi_card(
                            "Scorecard Points".to_string(),
                            format!("{:.1}/{:.0}", kpi_data.scorecard_points, kpi_data.max_scorecard_points),
                            if kpi_data.scorecard_points >= kpi_data.max_scorecard_points * 0.9 { KpiColor::Green } else { KpiColor::Orange },
                            icon_star.to_string(),
                            Some(KpiDelta {
                                value: format!("{:.0}%", scorecard_percentage),
                                is_positive: None,
                                suffix: "achieved".to_string(),
                            }),
                            None
                        )}
                        {kpi_card(
                            "Compliant Metrics".to_string(),
                            format!("{}", metrics.iter().filter(|m| m.status == ComplianceStatus::Compliant).count()),
                            KpiColor::Green,
                            icon_check.to_string(),
                            None,
                            None
                        )}
                        {kpi_card(
                            "Partial Compliance".to_string(),
                            format!("{}", metrics.iter().filter(|m| m.status == ComplianceStatus::PartiallyCompliant).count()),
                            KpiColor::Orange,
                            icon_alert.to_string(),
                            None,
                            None
                        )}
                        {kpi_card(
                            "Non-Compliant".to_string(),
                            format!("{}", metrics.iter().filter(|m| m.status == ComplianceStatus::NonCompliant).count()),
                            if metrics.iter().any(|m| m.status == ComplianceStatus::NonCompliant) { KpiColor::Red } else { KpiColor::Green },
                            icon_shield.to_string(),
                            None,
                            None
                        )}
                    </div>

                    // Compliance metrics table
                    {panel(
                        "B-BBEE Compliance Metrics".to_string(),
                        vec![view! { <button class="btn btn-sm btn-primary">"Run Assessment"</button> }],
                        vec![
                            if loading.get() {
                                view! { <div class="loading-overlay">"Loading metrics..."</div> }
                            } else {
                                view! { <div>{data_table(metric_columns.clone(), metric_rows.clone(), None)}</div> }
                            }
                        ]
                    )}
                </div>
            }
        </div>
    }
}
