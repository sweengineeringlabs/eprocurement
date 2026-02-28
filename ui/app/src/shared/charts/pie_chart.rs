//! Pie/Donut chart component

use components::prelude::*;

/// Pie chart data segment
#[derive(Clone)]
pub struct PieChartData {
    pub label: String,
    pub value: f64,
    pub color: String,
}

/// Pie chart component (rendered as donut circles for simplicity)
#[component]
pub fn pie_chart(
    data: Vec<PieChartData>,
    title: Option<String>,
) -> View {
    let total: f64 = data.iter().map(|d| d.value).sum();

    view! {
        style {
            r#"
            .pie-chart {
                display: flex;
                flex-direction: column;
                gap: 16px;
            }
            .pie-chart-title {
                font-size: 14px;
                font-weight: 600;
                color: var(--navy);
                text-align: center;
            }
            .pie-chart-circles {
                display: flex;
                justify-content: space-around;
                text-align: center;
            }
            .pie-chart-item {
                display: flex;
                flex-direction: column;
                align-items: center;
                gap: 8px;
            }
            .pie-chart-circle {
                width: 80px;
                height: 80px;
                border-radius: 50%;
                border-width: 6px;
                border-style: solid;
                display: flex;
                align-items: center;
                justify-content: center;
            }
            .pie-chart-value {
                font-size: 20px;
                font-weight: 600;
            }
            .pie-chart-label {
                font-size: 12px;
                color: var(--text-muted);
            }
            "#
        }

        <div class="pie-chart" data-testid="pie-chart">
            if let Some(t) = title {
                <div class="pie-chart-title">{t}</div>
            }
            <div class="pie-chart-circles">
                for item in data.iter() {
                    {pie_item(item.clone(), total)}
                }
            </div>
        </div>
    }
}

fn pie_item(item: PieChartData, total: f64) -> View {
    let percentage = if total > 0.0 {
        (item.value / total) * 100.0
    } else {
        0.0
    };
    let style = format!("border-color: {}", item.color);

    view! {
        <div class="pie-chart-item">
            <div class="pie-chart-circle" style={style}>
                <span class="pie-chart-value">{format!("{:.0}%", percentage)}</span>
            </div>
            <span class="pie-chart-label">{item.label}</span>
        </div>
    }
}
