//! Progress chart component for goal tracking

use components::prelude::*;

/// Progress chart data
#[derive(Clone)]
pub struct ProgressChartData {
    pub label: String,
    pub current: f64,
    pub target: f64,
    pub color: String,
}

/// Progress chart component
#[component]
pub fn progress_chart(data: ProgressChartData) -> View {
    let percentage = if data.target > 0.0 {
        (data.current / data.target) * 100.0
    } else {
        0.0
    };
    let width_style = format!("width: {}%", percentage.min(100.0));
    let fill_style = format!("background: {}", data.color);

    view! {
        style {
            r#"
            .progress-chart {
                display: flex;
                flex-direction: column;
                gap: 8px;
            }
            .progress-chart-header {
                display: flex;
                justify-content: space-between;
                align-items: center;
            }
            .progress-chart-label {
                font-size: 12px;
                color: var(--text);
            }
            .progress-chart-values {
                font-size: 12px;
                font-weight: 600;
            }
            .progress-chart-track {
                height: 8px;
                background: var(--bg);
                border-radius: 4px;
                overflow: hidden;
            }
            .progress-chart-fill {
                height: 100%;
                border-radius: 4px;
                transition: width 0.3s ease;
            }
            "#
        }

        <div class="progress-chart" data-testid="progress-chart">
            <div class="progress-chart-header">
                <span class="progress-chart-label">{data.label}</span>
                <span class="progress-chart-values">
                    {format!("{:.1}% / {:.0}%", data.current, data.target)}
                </span>
            </div>
            <div class="progress-chart-track">
                <div class="progress-chart-fill" style={format!("{} {}", width_style, fill_style)}></div>
            </div>
        </div>
    }
}
