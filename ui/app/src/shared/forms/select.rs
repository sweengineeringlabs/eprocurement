//! Select dropdown component

use components::prelude::*;
use wasm_bindgen::JsCast;

/// Select option
#[derive(Clone)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

/// Select component
#[component]
pub fn select(
    label: String,
    value: Signal<String>,
    options: Vec<SelectOption>,
    placeholder: Option<String>,
    required: bool,
    disabled: bool,
    error: Option<String>,
) -> View {
    select_with_testid(label, value, options, placeholder, required, disabled, error, None)
}

/// Select component with custom testid
#[component]
pub fn select_with_testid(
    label: String,
    value: Signal<String>,
    options: Vec<SelectOption>,
    placeholder: Option<String>,
    required: bool,
    disabled: bool,
    error: Option<String>,
    testid: Option<String>,
) -> View {
    let select_testid = testid.unwrap_or_default();
    let has_error = error.is_some();
    let group_class = if has_error { "form-group has-error" } else { "form-group" };

    let handle_change = Callback::new({
        let value = value.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            value.set(select.value());
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
            .form-group select {
                padding: 10px 36px 10px 12px;
                border: 1px solid var(--border);
                border-radius: var(--radius);
                font-size: 13px;
                font-family: inherit;
                transition: all 0.15s;
                background: var(--surface);
                appearance: none;
                background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%236b7a94' stroke-width='2'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
                background-repeat: no-repeat;
                background-position: right 12px center;
                cursor: pointer;
            }
            .form-group select:focus {
                outline: none;
                border-color: var(--blue);
                box-shadow: 0 0 0 3px var(--blue-light);
            }
            .form-group select:disabled {
                background-color: var(--bg);
                color: var(--text-muted);
                cursor: not-allowed;
            }
            .form-group.has-error select {
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
            <select
                disabled={disabled}
                on:change={handle_change}
                data-testid={select_testid}
            >
                if let Some(ph) = placeholder {
                    <option value="" disabled={true} selected={value.get().is_empty()}>{ph}</option>
                }
                for opt in options.iter() {
                    <option
                        value={opt.value.clone()}
                        selected={value.get() == opt.value}
                    >
                        {opt.label.clone()}
                    </option>
                }
            </select>
            if let Some(err) = error {
                <span class="form-error">{err}</span>
            }
        </div>
    }
}
