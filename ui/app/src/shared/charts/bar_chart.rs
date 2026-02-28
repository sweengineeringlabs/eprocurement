//! Bar chart component

use components::prelude::*;

/// Bar chart data point
#[derive(Clone)]
pub struct BarChartData {
    pub label: String,
    pub value: f64,
    pub color: Option<String>,
}

/// Bar chart component
#[component]
pub fn bar_chart(
    data: Vec<BarChartData>,
    height: Option<u32>,
) -> View {
    let height = height.unwrap_or(200);
    let max_value = data.iter().map(|d| d.value).fold(0.0f64, f64::max);

    view! {
        style {
            r#"
            .bar-chart {
                display: flex;
                flex-direction: column;
                gap: 12px;
            }
            .bar-chart-container {
                background: linear-gradient(180deg, var(--blue-light) 0%, transparent 100%);
                border-radius: var(--radius);
                display: flex;
                align-items: flex-end;
                justify-content: space-around;
                padding: 20px;
            }
            .bar-chart-bar {
                width: 32px;
                background: var(--blue);
                border-radius: var(--radius-sm) var(--radius-sm) 0 0;
                opacity: 0.8;
                transition: opacity 0.2s, height 0.3s;
            }
            .bar-chart-bar:hover {
                opacity: 1;
            }
            .bar-chart-labels {
                display: flex;
                justify-content: space-around;
                font-size: 11px;
                color: var(--text-muted);
            }
            "#
        }

        <div class="bar-chart" data-testid="bar-chart">
            <div class="bar-chart-container" style={format!("height: {}px", height)}>
                for item in data.iter() {
                    {bar_item(item.clone(), max_value, height)}
                }
            </div>
            <div class="bar-chart-labels">
                for item in data.iter() {
                    <span>{item.label.clone()}</span>
                }
            </div>
        </div>
    }
}

fn bar_item(item: BarChartData, max_value: f64, container_height: u32) -> View {
    let height_percent = if max_value > 0.0 {
        (item.value / max_value) * 100.0
    } else {
        0.0
    };
    let height_px = (height_percent / 100.0 * (container_height - 40) as f64) as u32;
    let style = format!(
        "height: {}px; background: {};",
        height_px,
        item.color.unwrap_or_else(|| "var(--blue)".to_string())
    );

    view! {
        <div class="bar-chart-bar" style={style} title={format!("{}: {:.0}", item.label, item.value)}></div>
    }
}
