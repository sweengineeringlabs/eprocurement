//! Empty state component

use components::prelude::*;

/// Empty state component
#[component]
pub fn empty_state(
    title: String,
    description: Option<String>,
    icon: Option<String>,
    action: Option<View>,
) -> View {
    let default_icon = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"/><line x1="9" y1="9" x2="15" y2="15"/><line x1="15" y1="9" x2="9" y2="15"/></svg>"#;
    let icon_html = icon.unwrap_or_else(|| default_icon.to_string());

    view! {
        style {
            r#"
            .empty-state {
                text-align: center;
                padding: 60px 20px;
                color: var(--text-muted);
            }
            .empty-state-icon {
                width: 64px;
                height: 64px;
                margin: 0 auto 16px;
                opacity: 0.3;
            }
            .empty-state-icon svg {
                width: 100%;
                height: 100%;
            }
            .empty-state-title {
                font-size: 16px;
                font-weight: 500;
                margin-bottom: 8px;
                color: var(--text);
            }
            .empty-state-description {
                font-size: 14px;
                color: var(--text-muted);
                margin-bottom: 16px;
            }
            "#
        }

        <div class="empty-state" data-testid="empty-state">
            <div class="empty-state-icon">
                <span inner_html={icon_html}></span>
            </div>
            <h3 class="empty-state-title">{title}</h3>
            if let Some(desc) = description {
                <p class="empty-state-description">{desc}</p>
            }
            if let Some(act) = action {
                {act}
            }
        </div>
    }
}
