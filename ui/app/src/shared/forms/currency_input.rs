//! Currency input component

use components::prelude::*;
use wasm_bindgen::JsCast;

/// Currency input component with ZAR formatting
#[component]
pub fn currency_input(
    label: String,
    value: Signal<f64>,
    required: bool,
    disabled: bool,
    error: Option<String>,
    hint: Option<String>,
) -> View {
    currency_input_with_testid(label, value, required, disabled, error, hint, None)
}

/// Currency input component with custom testid
#[component]
pub fn currency_input_with_testid(
    label: String,
    value: Signal<f64>,
    required: bool,
    disabled: bool,
    error: Option<String>,
    hint: Option<String>,
    testid: Option<String>,
) -> View {
    let input_testid = testid.unwrap_or_default();
    let has_error = error.is_some();
    let group_class = if has_error { "form-group has-error" } else { "form-group" };

    let formatted_value = format!("{:.2}", value.get());

    let handle_input = Callback::new({
        let value = value.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            if let Ok(num) = input.value().replace(",", "").parse::<f64>() {
                value.set(num);
            }
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
            .currency-input-wrapper {
                position: relative;
                display: flex;
                align-items: center;
            }
            .currency-prefix {
                position: absolute;
                left: 12px;
                font-size: 13px;
                color: var(--text-muted);
                font-weight: 500;
            }
            .currency-input {
                padding: 10px 12px 10px 28px;
                border: 1px solid var(--border);
                border-radius: var(--radius);
                font-size: 13px;
                font-family: IBM Plex Mono, monospace;
                transition: all 0.15s;
                background: var(--surface);
                width: 100%;
                text-align: right;
            }
            .currency-input:focus {
                outline: none;
                border-color: var(--blue);
                box-shadow: 0 0 0 3px var(--blue-light);
            }
            .currency-input:disabled {
                background: var(--bg);
                color: var(--text-muted);
                cursor: not-allowed;
            }
            .form-group.has-error .currency-input {
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
            <div class="currency-input-wrapper">
                <span class="currency-prefix">"R"</span>
                <input
                    type="text"
                    class="currency-input"
                    value={formatted_value}
                    disabled={disabled}
                    on:input={handle_input}
                    data-testid={input_testid}
                />
            </div>
            if let Some(err) = error {
                <span class="form-error">{err}</span>
            } else if let Some(h) = hint {
                <span class="form-hint">{h}</span>
            }
        </div>
    }
}
