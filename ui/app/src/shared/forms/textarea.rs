//! Textarea component

use components::prelude::*;
use wasm_bindgen::JsCast;

/// Textarea component
#[component]
pub fn textarea(
    label: String,
    value: Signal<String>,
    placeholder: Option<String>,
    required: bool,
    disabled: bool,
    rows: Option<u32>,
    error: Option<String>,
    hint: Option<String>,
) -> View {
    let has_error = error.is_some();
    let group_class = if has_error { "form-group has-error" } else { "form-group" };
    let rows = rows.unwrap_or(4);

    let handle_input = Callback::new({
        let value = value.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let textarea: web_sys::HtmlTextAreaElement = target.dyn_into().unwrap();
            value.set(textarea.value());
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
            .form-group textarea {
                padding: 10px 12px;
                border: 1px solid var(--border);
                border-radius: var(--radius);
                font-size: 13px;
                font-family: inherit;
                transition: all 0.15s;
                background: var(--surface);
                min-height: 100px;
                resize: vertical;
            }
            .form-group textarea:focus {
                outline: none;
                border-color: var(--blue);
                box-shadow: 0 0 0 3px var(--blue-light);
            }
            .form-group textarea:disabled {
                background: var(--bg);
                color: var(--text-muted);
                cursor: not-allowed;
            }
            .form-group.has-error textarea {
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
            <textarea
                rows={rows.to_string()}
                placeholder={placeholder.unwrap_or_default()}
                disabled={disabled}
                on:input={handle_input}
            >
                {value.get()}
            </textarea>
            if let Some(err) = error {
                <span class="form-error">{err}</span>
            } else if let Some(h) = hint {
                <span class="form-hint">{h}</span>
            }
        </div>
    }
}
