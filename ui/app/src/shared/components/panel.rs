//! Panel container component

use components::prelude::*;

/// Panel with header and body
#[component]
pub fn panel(
    title: String,
    header_actions: Vec<View>,
    children: Vec<View>,
) -> View {
    view! {
        style {
            r#"
            .panel {
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius-lg);
                box-shadow: var(--shadow-sm);
                margin-bottom: 20px;
            }
            .panel-header {
                padding: 16px 20px;
                border-bottom: 1px solid var(--border);
                display: flex;
                align-items: center;
                justify-content: space-between;
            }
            .panel-title {
                font-weight: 600;
                font-size: 15px;
                color: var(--navy);
            }
            .panel-body {
                padding: 20px;
            }
            .panel-header-actions {
                display: flex;
                gap: 8px;
            }
            "#
        }

        <div class="panel" data-testid="panel">
            <div class="panel-header">
                <span class="panel-title">{title}</span>
                <div class="panel-header-actions">
                    for action in header_actions {
                        {action}
                    }
                </div>
            </div>
            <div class="panel-body">
                for child in children {
                    {child}
                }
            </div>
        </div>
    }
}

/// Panel with header, body, and footer
#[component]
pub fn panel_with_footer(
    title: String,
    header_actions: Vec<View>,
    children: Vec<View>,
    footer_actions: Vec<View>,
) -> View {
    view! {
        style {
            r#"
            .panel {
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius-lg);
                box-shadow: var(--shadow-sm);
                margin-bottom: 20px;
            }
            .panel-header {
                padding: 16px 20px;
                border-bottom: 1px solid var(--border);
                display: flex;
                align-items: center;
                justify-content: space-between;
            }
            .panel-title {
                font-weight: 600;
                font-size: 15px;
                color: var(--navy);
            }
            .panel-body {
                padding: 20px;
            }
            .panel-footer {
                padding: 16px 20px;
                border-top: 1px solid var(--border);
                display: flex;
                justify-content: flex-end;
                gap: 12px;
                background: var(--bg);
                border-radius: 0 0 var(--radius-lg) var(--radius-lg);
            }
            "#
        }

        <div class="panel" data-testid="panel">
            <div class="panel-header">
                <span class="panel-title">{title}</span>
                <div class="panel-header-actions">
                    for action in header_actions {
                        {action}
                    }
                </div>
            </div>
            <div class="panel-body">
                for child in children {
                    {child}
                }
            </div>
            <div class="panel-footer">
                for action in footer_actions {
                    {action}
                }
            </div>
        </div>
    }
}
