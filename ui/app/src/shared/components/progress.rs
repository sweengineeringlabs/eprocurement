//! Progress bar component

use components::prelude::*;

/// Progress bar color variants
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ProgressColor {
    #[default]
    Blue,
    Green,
    Orange,
    Accent,
    Red,
    Gray,
}

impl ProgressColor {
    fn class(&self) -> &'static str {
        match self {
            ProgressColor::Blue => "blue",
            ProgressColor::Green => "green",
            ProgressColor::Orange => "orange",
            ProgressColor::Accent => "accent",
            ProgressColor::Red => "red",
            ProgressColor::Gray => "gray",
        }
    }
}

/// Progress bar component
#[component]
pub fn progress_bar(
    value: f64, // 0.0 to 100.0
    color: ProgressColor,
    show_label: bool,
    height: Option<u32>,
) -> View {
    let fill_class = format!("progress-fill {}", color.class());
    let width_style = format!("width: {}%", value.min(100.0).max(0.0));
    let height_style = height.map(|h| format!("height: {}px", h)).unwrap_or_default();

    view! {
        style {
            r#"
            .progress-container {
                display: flex;
                align-items: center;
                gap: 12px;
            }
            .progress-track {
                flex: 1;
                height: 8px;
                background: var(--bg);
                border-radius: 4px;
                overflow: hidden;
            }
            .progress-fill {
                height: 100%;
                border-radius: 4px;
                transition: width 0.3s ease;
            }
            .progress-fill.blue { background: var(--blue); }
            .progress-fill.green { background: var(--green); }
            .progress-fill.orange { background: var(--orange); }
            .progress-fill.accent { background: var(--accent); }
            .progress-fill.red { background: var(--red); }
            .progress-fill.gray { background: var(--text-muted); }
            .progress-label {
                font-size: 12px;
                font-weight: 500;
                color: var(--text);
                min-width: 40px;
                text-align: right;
            }
            "#
        }

        <div class="progress-container" data-testid="progress-bar">
            <div class="progress-track" style={height_style}>
                <div class={fill_class} style={width_style}></div>
            </div>
            if show_label {
                <span class="progress-label">{format!("{:.1}%", value)}</span>
            }
        </div>
    }
}
