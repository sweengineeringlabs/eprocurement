//! KPI Card component for dashboard metrics

use components::prelude::*;

/// KPI card color variants
#[derive(Clone, Copy, PartialEq, Default)]
pub enum KpiColor {
    #[default]
    Blue,
    Green,
    Orange,
    Accent,
    Purple,
    Red,
    Cyan,
}

impl KpiColor {
    fn class(&self) -> &'static str {
        match self {
            KpiColor::Blue => "",
            KpiColor::Green => "green",
            KpiColor::Orange => "orange",
            KpiColor::Accent => "accent",
            KpiColor::Purple => "purple",
            KpiColor::Red => "red",
            KpiColor::Cyan => "cyan",
        }
    }

    fn icon_class(&self) -> &'static str {
        match self {
            KpiColor::Blue => "blue",
            KpiColor::Green => "green",
            KpiColor::Orange => "orange",
            KpiColor::Accent => "accent",
            KpiColor::Purple => "purple",
            KpiColor::Red => "red",
            KpiColor::Cyan => "cyan",
        }
    }
}

/// Delta indicator for KPI changes
pub struct KpiDelta {
    pub value: String,
    pub is_positive: Option<bool>, // None for neutral
    pub suffix: String,
}

/// KPI Card displaying a metric with optional change indicator
#[component]
pub fn kpi_card(
    label: String,
    value: String,
    color: KpiColor,
    icon: String,
    delta: Option<KpiDelta>,
    on_click: Option<Callback<()>>,
) -> View {
    let card_class = format!("kpi-card {}", color.class());
    let icon_class = format!("kpi-icon {}", color.icon_class());

    let handle_click = Callback::new({
        let on_click = on_click.clone();
        move |_| {
            if let Some(cb) = &on_click {
                cb.call(());
            }
        }
    });

    view! {
        style {
            r#"
            .kpi-card {
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius-lg);
                padding: 20px;
                border-top: 3px solid var(--blue);
                cursor: pointer;
                transition: transform 0.2s, box-shadow 0.2s;
            }
            .kpi-card:hover {
                transform: translateY(-2px);
                box-shadow: var(--shadow);
            }
            .kpi-card.green { border-top-color: var(--green); }
            .kpi-card.orange { border-top-color: var(--orange); }
            .kpi-card.accent { border-top-color: var(--accent); }
            .kpi-card.purple { border-top-color: var(--purple); }
            .kpi-card.red { border-top-color: var(--red); }
            .kpi-card.cyan { border-top-color: var(--cyan); }
            .kpi-header {
                display: flex;
                align-items: flex-start;
                justify-content: space-between;
                margin-bottom: 12px;
            }
            .kpi-icon {
                width: 40px;
                height: 40px;
                border-radius: var(--radius);
                display: flex;
                align-items: center;
                justify-content: center;
            }
            .kpi-icon.blue { background: var(--blue-light); color: var(--blue); }
            .kpi-icon.green { background: var(--green-light); color: var(--green); }
            .kpi-icon.orange { background: var(--orange-light); color: var(--orange); }
            .kpi-icon.accent { background: var(--accent-light); color: var(--accent); }
            .kpi-icon.purple { background: var(--purple-light); color: var(--purple); }
            .kpi-icon.red { background: var(--red-light); color: var(--red); }
            .kpi-icon.cyan { background: var(--cyan-light); color: var(--cyan); }
            .kpi-icon svg {
                width: 20px;
                height: 20px;
            }
            .kpi-label {
                font-size: 12px;
                color: var(--text-muted);
                margin-bottom: 4px;
            }
            .kpi-value {
                font-size: 24px;
                font-weight: 600;
                color: var(--navy);
                font-family: IBM Plex Mono, monospace;
            }
            .kpi-delta {
                font-size: 11px;
                color: var(--text-muted);
                margin-top: 8px;
            }
            .kpi-delta .up { color: var(--green); }
            .kpi-delta .down { color: var(--red); }
            "#
        }

        <div class={card_class} on:click={handle_click} data-testid="kpi-card">
            <div class="kpi-header">
                <div class={icon_class}>
                    <span inner_html={icon}></span>
                </div>
            </div>
            <div class="kpi-label">{label}</div>
            <div class="kpi-value">{value}</div>
            if let Some(d) = delta {
                <div class="kpi-delta">
                    if let Some(positive) = d.is_positive {
                        if positive {
                            <span class="up">"↑ "{d.value}</span>
                        } else {
                            <span class="down">"↓ "{d.value}</span>
                        }
                    } else {
                        <span>{d.value}</span>
                    }
                    " "{d.suffix}
                </div>
            }
        </div>
    }
}
