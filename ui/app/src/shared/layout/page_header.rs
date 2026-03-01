//! Page header component with title and actions

use components::prelude::*;

/// Page header with title, subtitle, and action buttons
#[component]
pub fn page_header(
    title: String,
    subtitle: Option<String>,
    children: Vec<View>,
) -> View {
    view! {
        style {
            r#"
            .page-header {
                display: flex;
                align-items: center;
                justify-content: space-between;
                margin-bottom: 24px;
            }
            .page-header-left h1 {
                font-family: 'Playfair Display', serif;
                font-size: 24px;
                font-weight: 600;
                color: var(--navy);
                margin-bottom: 4px;
            }
            .page-header-left p {
                color: var(--text-muted);
                font-size: 13px;
            }
            .page-header-actions {
                display: flex;
                gap: 12px;
            }
            "#
        }

        <div class="page-header" data-testid="page-header">
            <div class="page-header-left">
                <h1>{title}</h1>
                if let Some(sub) = subtitle {
                    <p>{sub}</p>
                }
            </div>
            <div class="page-header-actions">
                for child in children {
                    {child}
                }
            </div>
        </div>
    }
}
