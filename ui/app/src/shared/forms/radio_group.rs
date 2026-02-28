//! Radio group component

use components::prelude::*;

/// Radio option
#[derive(Clone)]
pub struct RadioOption {
    pub value: String,
    pub label: String,
}

/// Radio group component
#[component]
pub fn radio_group(
    label: String,
    name: String,
    value: Signal<String>,
    options: Vec<RadioOption>,
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
            .form-group > label {
                font-size: 12px;
                font-weight: 500;
                color: var(--text-muted);
            }
            .required-mark {
                color: var(--red);
            }
            .radio-group {
                display: flex;
                flex-direction: column;
                gap: 8px;
            }
            .radio-item {
                display: flex;
                align-items: center;
                gap: 8px;
                font-size: 13px;
                cursor: pointer;
            }
            .radio-item input {
                width: 18px;
                height: 18px;
                cursor: pointer;
                accent-color: var(--blue);
            }
            .radio-item.disabled {
                opacity: 0.5;
                cursor: not-allowed;
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
            <div class="radio-group">
                for opt in options.iter() {
                    {radio_option(opt.clone(), name.clone(), value.clone(), disabled)}
                }
            </div>
            if let Some(err) = error {
                <span class="form-error">{err}</span>
            }
        </div>
    }
}

fn radio_option(opt: RadioOption, name: String, value: Signal<String>, disabled: bool) -> View {
    let is_checked = value.get() == opt.value;
    let opt_value = opt.value.clone();

    let handle_change = Callback::<()>::new({
        let value = value.clone();
        let opt_value = opt_value.clone();
        move |_| {
            value.set(opt_value.clone());
        }
    });

    view! {
        <label class={if disabled { "radio-item disabled" } else { "radio-item" }}>
            <input
                type="radio"
                name={name}
                value={opt.value}
                checked={is_checked}
                disabled={disabled}
                on:change={handle_change}
            />
            {opt.label}
        </label>
    }
}
