//! Date picker component

use components::prelude::*;
use wasm_bindgen::JsCast;

/// Date picker component
#[component]
pub fn date_picker(
    label: String,
    value: Signal<String>, // ISO format YYYY-MM-DD
    required: bool,
    disabled: bool,
    min_date: Option<String>,
    max_date: Option<String>,
    error: Option<String>,
) -> View {
    let has_error = error.is_some();
    let group_class = if has_error { "form-group has-error" } else { "form-group" };

    let handle_change = Callback::new({
        let value = value.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            value.set(input.value());
        }
    });

    view! {
        style {
            r#"
            .form-group {
                display: flex;
                flex-direction: column;
                gap: 6px;
            }
            .form-group label {
                font-size: 12px;
                font-weight: 500;
                color: var(--text-muted);
            }
            .required-mark {
                color: var(--red);
            }
            .form-group input[type="date"] {
                padding: 10px 12px;
                border: 1px solid var(--border);
                border-radius: var(--radius);
                font-size: 13px;
                font-family: inherit;
                transition: all 0.15s;
                background: var(--surface);
            }
            .form-group input[type="date"]:focus {
                outline: none;
                border-color: var(--blue);
                box-shadow: 0 0 0 3px var(--blue-light);
            }
            .form-group input[type="date"]:disabled {
                background: var(--bg);
                color: var(--text-muted);
                cursor: not-allowed;
            }
            .form-group.has-error input[type="date"] {
                border-color: var(--red);
            }
            .form-error {
                font-size: 11px;
                color: var(--red);
            }
            "#
        }

        <div class={group_class}>
            <label>
                {label}
                if required {
                    <span class="required-mark">" *"</span>
                }
            </label>
            <input
                type="date"
                value={value.get()}
                disabled={disabled}
                min={min_date.unwrap_or_default()}
                max={max_date.unwrap_or_default()}
                on:change={handle_change}
            />
            if let Some(err) = error {
                <span class="form-error">{err}</span>
            }
        </div>
    }
}
