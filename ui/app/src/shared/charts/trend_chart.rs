//! Trend chart component (sparkline style)

use components::prelude::*;

/// Trend chart data point
#[derive(Clone)]
pub struct TrendChartData {
    pub label: String,
    pub value: f64,
}

/// Trend chart component (simplified bar visualization)
#[component]
pub fn trend_chart(
    data: Vec<TrendChartData>,
    height: Option<u32>,
    color: Option<String>,
) -> View {
    let height = height.unwrap_or(100);
    let color = color.unwrap_or_else(|| "var(--blue)".to_string());
    let max_value = data.iter().map(|d| d.value).fold(0.0f64, f64::max);

    view! {
        style {
            r#"
            .trend-chart {
                display: flex;
                flex-direction: column;
                gap: 8px;
            }
            .trend-chart-container {
                display: flex;
                align-items: flex-end;
                gap: 4px;
            }
            .trend-chart-bar {
                flex: 1;
                border-radius: 2px 2px 0 0;
                transition: height 0.3s;
                min-width: 8px;
            }
            .trend-chart-labels {
                display: flex;
                justify-content: space-between;
                font-size: 10px;
                color: var(--text-muted);
            }
            "#
        }

        <div class="trend-chart" data-testid="trend-chart">
            <div class="trend-chart-container" style={format!("height: {}px", height)}>
                for item in data.iter() {
                    {trend_bar(item.clone(), max_value, height, color.clone())}
                }
            </div>
            <div class="trend-chart-labels">
                if let Some(first) = data.first() {
                    <span>{first.label.clone()}</span>
                }
                if let Some(last) = data.last() {
                    <span>{last.label.clone()}</span>
                }
            </div>
        </div>
    }
}

fn trend_bar(item: TrendChartData, max_value: f64, container_height: u32, color: String) -> View {
    let height_percent = if max_value > 0.0 {
        (item.value / max_value) * 100.0
    } else {
        0.0
    };
    let height_px = (height_percent / 100.0 * container_height as f64) as u32;
    let style = format!("height: {}px; background: {};", height_px, color);

    view! {
        <div class="trend-chart-bar" style={style} title={format!("{}: {:.0}", item.label, item.value)}></div>
    }
}
