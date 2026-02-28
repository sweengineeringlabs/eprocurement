//! B-BBEE level badge component

use components::prelude::*;

/// B-BBEE certification levels
#[derive(Clone, Copy, PartialEq, Default)]
pub enum BbbeeLevel {
    #[default]
    Level1,
    Level2,
    Level3,
    Level4,
    NonCompliant,
}

impl BbbeeLevel {
    fn class(&self) -> &'static str {
        match self {
            BbbeeLevel::Level1 => "bbbee-level-1",
            BbbeeLevel::Level2 => "bbbee-level-2",
            BbbeeLevel::Level3 => "bbbee-level-3",
            BbbeeLevel::Level4 => "bbbee-level-4",
            BbbeeLevel::NonCompliant => "bbbee-non",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            BbbeeLevel::Level1 => "Level 1",
            BbbeeLevel::Level2 => "Level 2",
            BbbeeLevel::Level3 => "Level 3",
            BbbeeLevel::Level4 => "Level 4",
            BbbeeLevel::NonCompliant => "Non-Compliant",
        }
    }
}

/// B-BBEE badge component
#[component]
pub fn bbbee_badge(level: BbbeeLevel) -> View {
    let class = format!("bbbee-badge {}", level.class());

    view! {
        style {
            r#"
            .bbbee-badge {
                display: inline-flex;
                align-items: center;
                gap: 4px;
                padding: 4px 8px;
                border-radius: var(--radius-sm);
                font-size: 11px;
                font-weight: 600;
            }
            .bbbee-level-1 { background: #e8f5e9; color: #2e7d32; }
            .bbbee-level-2 { background: #e3f2fd; color: #1565c0; }
            .bbbee-level-3 { background: #fff3e0; color: #ef6c00; }
            .bbbee-level-4 { background: #fce4ec; color: #c2185b; }
            .bbbee-non { background: var(--bg); color: var(--text-muted); }
            "#
        }

        <span class={class}>{level.label()}</span>
    }
}
