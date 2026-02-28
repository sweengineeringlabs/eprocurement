//! Button components

use components::prelude::*;

/// Button size variants
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ButtonSize {
    fn class(&self) -> &'static str {
        match self {
            ButtonSize::Small => "btn-sm",
            ButtonSize::Medium => "",
            ButtonSize::Large => "btn-lg",
        }
    }
}

/// Generic button component
#[component]
pub fn button(
    label: String,
    variant: String, // "primary", "secondary", "accent", "success", "danger"
    size: ButtonSize,
    icon: Option<String>,
    disabled: bool,
    on_click: Callback<()>,
) -> View {
    let class = format!("btn btn-{} {}", variant, size.class());

    let handle_click = Callback::<()>::new({
        let on_click = on_click.clone();
        let disabled = disabled;
        move |_| {
            if !disabled {
                on_click.call(());
            }
        }
    });

    view! {
        style {
            r#"
            .btn {
                display: inline-flex;
                align-items: center;
                gap: 8px;
                padding: 10px 16px;
                border-radius: var(--radius);
                font-size: 13px;
                font-weight: 500;
                cursor: pointer;
                transition: all 0.15s;
                border: none;
                text-decoration: none;
            }
            .btn svg {
                width: 16px;
                height: 16px;
            }
            .btn-primary {
                background: var(--blue);
                color: #fff;
            }
            .btn-primary:hover:not(:disabled) {
                background: #1550b0;
            }
            .btn-secondary {
                background: var(--surface);
                color: var(--text);
                border: 1px solid var(--border);
            }
            .btn-secondary:hover:not(:disabled) {
                background: var(--bg);
            }
            .btn-accent {
                background: var(--accent);
                color: #fff;
            }
            .btn-accent:hover:not(:disabled) {
                background: #b3862f;
            }
            .btn-success {
                background: var(--green);
                color: #fff;
            }
            .btn-success:hover:not(:disabled) {
                background: #156b3e;
            }
            .btn-danger {
                background: var(--red);
                color: #fff;
            }
            .btn-danger:hover:not(:disabled) {
                background: #a33025;
            }
            .btn-sm {
                padding: 6px 12px;
                font-size: 12px;
            }
            .btn-lg {
                padding: 14px 24px;
                font-size: 15px;
            }
            .btn:disabled {
                opacity: 0.5;
                cursor: not-allowed;
            }
            .btn:not(:disabled):active {
                transform: scale(0.98);
            }
            "#
        }

        <button class={class} disabled={disabled} on:click={handle_click}>
            if let Some(i) = icon {
                <span inner_html={i}></span>
            }
            {label}
        </button>
    }
}

/// Primary button shorthand
#[component]
pub fn btn_primary(
    label: String,
    icon: Option<String>,
    on_click: Callback<()>,
) -> View {
    button(label, "primary".to_string(), ButtonSize::Medium, icon, false, on_click)
}

/// Secondary button shorthand
#[component]
pub fn btn_secondary(
    label: String,
    icon: Option<String>,
    on_click: Callback<()>,
) -> View {
    button(label, "secondary".to_string(), ButtonSize::Medium, icon, false, on_click)
}

/// Accent button shorthand
#[component]
pub fn btn_accent(
    label: String,
    icon: Option<String>,
    on_click: Callback<()>,
) -> View {
    button(label, "accent".to_string(), ButtonSize::Medium, icon, false, on_click)
}

/// Danger button shorthand
#[component]
pub fn btn_danger(
    label: String,
    icon: Option<String>,
    on_click: Callback<()>,
) -> View {
    button(label, "danger".to_string(), ButtonSize::Medium, icon, false, on_click)
}
