//! Form group layout component

use components::prelude::*;

/// Form group for organizing form fields
#[component]
pub fn form_group(
    title: Option<String>,
    columns: u32, // 1, 2, or 3
    children: Vec<View>,
) -> View {
    let grid_class = match columns {
        1 => "form-grid-1",
        2 => "form-grid",
        3 => "form-grid-3",
        _ => "form-grid",
    };

    view! {
        style {
            r#"
            .form-section {
                margin-bottom: 24px;
            }
            .form-section-title {
                font-weight: 600;
                font-size: 14px;
                color: var(--navy);
                margin-bottom: 16px;
                padding-bottom: 8px;
                border-bottom: 1px solid var(--border);
            }
            .form-grid {
                display: grid;
                grid-template-columns: repeat(2, 1fr);
                gap: 16px;
            }
            .form-grid-1 {
                display: grid;
                grid-template-columns: 1fr;
                gap: 16px;
            }
            .form-grid-3 {
                display: grid;
                grid-template-columns: repeat(3, 1fr);
                gap: 16px;
            }
            @media (max-width: 768px) {
                .form-grid, .form-grid-3 {
                    grid-template-columns: 1fr;
                }
            }
            "#
        }

        <div class="form-section">
            if let Some(t) = title {
                <div class="form-section-title">{t}</div>
            }
            <div class={grid_class}>
                for child in children {
                    {child}
                }
            </div>
        </div>
    }
}
