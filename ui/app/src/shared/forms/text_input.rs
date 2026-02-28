//! Text input component

use components::prelude::*;
use wasm_bindgen::JsCast;

/// Text input component
#[component]
pub fn text_input(
    label: String,
    value: Signal<String>,
    placeholder: Option<String>,
    required: bool,
    disabled: bool,
    error: Option<String>,
    hint: Option<String>,
    input_type: Option<String>, // "text", "email", "password", "tel", etc.
) -> View {
    let input_type = input_type.unwrap_or_else(|| "text".to_string());
    let has_error = error.is_some();
    let group_class = if has_error { "form-group has-error" } else { "form-group" };

    let handle_input = Callback::new({
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
            .form-group.span-2 {
                grid-column: span 2;
            }
            .form-group label {
                font-size: 12px;
                font-weight: 500;
                color: var(--text-muted);
            }
            .required-mark {
                color: var(--red);
            }
            .form-group input {
                padding: 10px 12px;
                border: 1px solid var(--border);
                border-radius: var(--radius);
                font-size: 13px;
                font-family: inherit;
                transition: all 0.15s;
                background: var(--surface);
            }
            .form-group input:focus {
                outline: none;
                border-color: var(--blue);
                box-shadow: 0 0 0 3px var(--blue-light);
            }
            .form-group input:disabled {
                background: var(--bg);
                color: var(--text-muted);
                cursor: not-allowed;
            }
            .form-group.has-error input {
                border-color: var(--red);
            }
            .form-hint {
                font-size: 11px;
                color: var(--text-muted);
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
                type={input_type}
                value={value.get()}
                placeholder={placeholder.unwrap_or_default()}
                disabled={disabled}
                on:input={handle_input}
            />
            if let Some(err) = error {
                <span class="form-error">{err}</span>
            } else if let Some(h) = hint {
                <span class="form-hint">{h}</span>
            }
        </div>
    }
}
