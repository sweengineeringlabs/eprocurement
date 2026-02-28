//! Checkbox component

use components::prelude::*;

/// Checkbox component
#[component]
pub fn checkbox(
    label: String,
    checked: Signal<bool>,
    disabled: bool,
) -> View {
    let handle_change = Callback::<()>::new({
        let checked = checked.clone();
        move |_| {
            checked.set(!checked.get());
        }
    });

    view! {
        style {
            r#"
            .checkbox-item {
                display: flex;
                align-items: center;
                gap: 8px;
                font-size: 13px;
                cursor: pointer;
            }
            .checkbox-item input {
                width: 18px;
                height: 18px;
                cursor: pointer;
                accent-color: var(--blue);
            }
            .checkbox-item.disabled {
                opacity: 0.5;
                cursor: not-allowed;
            }
            "#
        }

        <label class={if disabled { "checkbox-item disabled" } else { "checkbox-item" }}>
            <input
                type="checkbox"
                checked={checked.get()}
                disabled={disabled}
                on:change={handle_change}
            />
            {label}
        </label>
    }
}
