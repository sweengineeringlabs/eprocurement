//! Toast notification component

use components::prelude::*;

/// Toast type variants
#[derive(Clone, Copy, PartialEq)]
pub enum ToastType {
    Success,
    Error,
    Warning,
    Info,
}

impl ToastType {
    fn class(&self) -> &'static str {
        match self {
            ToastType::Success => "toast-success",
            ToastType::Error => "toast-error",
            ToastType::Warning => "toast-warning",
            ToastType::Info => "toast-info",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            ToastType::Success => "✓",
            ToastType::Error => "×",
            ToastType::Warning => "!",
            ToastType::Info => "i",
        }
    }
}

/// Toast data
#[derive(Clone)]
pub struct Toast {
    pub id: String,
    pub message: String,
    pub toast_type: ToastType,
    pub visible: bool,
}

/// Toast container component
#[component]
pub fn toast_container(toasts: Signal<Vec<Toast>>, on_dismiss: Callback<String>) -> View {
    view! {
        style {
            r#"
            .toast-container {
                position: fixed;
                top: 24px;
                right: 24px;
                z-index: 2000;
                display: flex;
                flex-direction: column;
                gap: 12px;
                pointer-events: none;
            }
            .toast {
                display: flex;
                align-items: center;
                gap: 12px;
                padding: 14px 20px;
                background: var(--surface);
                border-radius: var(--radius);
                box-shadow: var(--shadow-lg);
                border-left: 4px solid var(--blue);
                min-width: 300px;
                max-width: 450px;
                transform: translateX(120%);
                transition: transform 0.3s ease;
                pointer-events: auto;
            }
            .toast.show {
                transform: translateX(0);
            }
            .toast-success { border-left-color: var(--green); }
            .toast-error { border-left-color: var(--red); }
            .toast-warning { border-left-color: var(--orange); }
            .toast-info { border-left-color: var(--blue); }
            .toast-icon {
                width: 24px;
                height: 24px;
                border-radius: 50%;
                display: flex;
                align-items: center;
                justify-content: center;
                font-size: 14px;
                font-weight: 700;
                flex-shrink: 0;
            }
            .toast-success .toast-icon { background: var(--green-light); color: var(--green); }
            .toast-error .toast-icon { background: var(--red-light); color: var(--red); }
            .toast-warning .toast-icon { background: var(--orange-light); color: var(--orange); }
            .toast-info .toast-icon { background: var(--blue-light); color: var(--blue); }
            .toast-message {
                flex: 1;
                font-size: 14px;
                color: var(--text);
            }
            .toast-close {
                width: 24px;
                height: 24px;
                border: none;
                background: transparent;
                color: var(--text-muted);
                cursor: pointer;
                font-size: 18px;
                opacity: 0.5;
                transition: opacity 0.15s;
            }
            .toast-close:hover {
                opacity: 1;
            }
            "#
        }

        <div class="toast-container" data-testid="toast-container">
            for toast in toasts.get().iter() {
                {toast_item(toast.clone(), on_dismiss.clone())}
            }
        </div>
    }
}

fn toast_item(toast: Toast, on_dismiss: Callback<String>) -> View {
    let class = format!("toast {} {}", toast.toast_type.class(), if toast.visible { "show" } else { "" });
    let icon = toast.toast_type.icon();
    let id = toast.id.clone();

    let handle_close = Callback::<()>::new({
        let on_dismiss = on_dismiss.clone();
        let id = id.clone();
        move |_| {
            on_dismiss.call(id.clone());
        }
    });

    view! {
        <div class={class}>
            <span class="toast-icon">{icon}</span>
            <span class="toast-message">{toast.message}</span>
            <button class="toast-close" on:click={handle_close}>"×"</button>
        </div>
    }
}
