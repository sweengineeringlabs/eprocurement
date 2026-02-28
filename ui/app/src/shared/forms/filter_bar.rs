//! Filter bar component for list views

use components::prelude::*;

/// Filter bar component
#[component]
pub fn filter_bar(children: Vec<View>) -> View {
    view! {
        style {
            r#"
            .filter-bar {
                display: flex;
                align-items: center;
                gap: 12px;
                padding: 16px 20px;
                background: var(--bg);
                border-radius: var(--radius);
                margin-bottom: 20px;
                flex-wrap: wrap;
            }
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
            .filter-group select:focus,
            .filter-group input:focus {
                outline: none;
                border-color: var(--blue);
            }
            .filter-spacer {
                flex: 1;
            }
            "#
        }

        <div class="filter-bar" data-testid="filter-bar">
            for child in children {
                {child}
            }
        </div>
    }
}
