//! Multi-select component with checkboxes

use components::prelude::*;

/// Multi-select option
#[derive(Clone)]
pub struct MultiSelectOption {
    pub value: String,
    pub label: String,
}

/// Multi-select component
#[component]
pub fn multi_select(
    label: String,
    selected: Signal<Vec<String>>,
    options: Vec<MultiSelectOption>,
    required: bool,
    disabled: bool,
    error: Option<String>,
) -> View {
    let has_error = error.is_some();
    let group_class = if has_error { "form-group has-error" } else { "form-group" };

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
            .multi-select-options {
                display: flex;
                flex-direction: column;
                gap: 8px;
                padding: 12px;
                border: 1px solid var(--border);
                border-radius: var(--radius);
                background: var(--surface);
                max-height: 200px;
                overflow-y: auto;
            }
            .multi-select-option {
                display: flex;
                align-items: center;
                gap: 8px;
                font-size: 13px;
                cursor: pointer;
            }
            .multi-select-option input {
                width: 16px;
                height: 16px;
                cursor: pointer;
                accent-color: var(--blue);
            }
            .multi-select-option:hover {
                color: var(--blue);
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
            <div class="multi-select-options">
                for opt in options.iter() {
                    {multi_select_option(opt.clone(), selected.clone(), disabled)}
                }
            </div>
            if let Some(err) = error {
                <span class="form-error">{err}</span>
            }
        </div>
    }
}

fn multi_select_option(opt: MultiSelectOption, selected: Signal<Vec<String>>, disabled: bool) -> View {
    let is_checked = selected.get().contains(&opt.value);
    let value = opt.value.clone();

    let handle_change = Callback::<()>::new({
        let selected = selected.clone();
        let value = value.clone();
        move |_| {
            let mut current = selected.get();
            if current.contains(&value) {
                current.retain(|v| v != &value);
            } else {
                current.push(value.clone());
            }
            selected.set(current);
        }
    });

    view! {
        <label class="multi-select-option">
            <input
                type="checkbox"
                checked={is_checked}
                disabled={disabled}
                on:change={handle_change}
            />
            {opt.label}
        </label>
    }
}
