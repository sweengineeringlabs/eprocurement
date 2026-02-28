//! Modal dialog component

use components::prelude::*;

/// Modal size variants
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ModalSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ModalSize {
    fn class(&self) -> &'static str {
        match self {
            ModalSize::Small => "modal-small",
            ModalSize::Medium => "modal-medium",
            ModalSize::Large => "modal-large",
        }
    }
}

/// Modal component
#[component]
pub fn modal(
    title: String,
    size: ModalSize,
    visible: Signal<bool>,
    on_close: Callback<()>,
    children: Vec<View>,
    footer_actions: Vec<View>,
) -> View {
    let overlay_class = if visible.get() { "modal-overlay show" } else { "modal-overlay" };
    let modal_class = format!("modal {}", size.class());

    let handle_overlay_click = Callback::<()>::new({
        let on_close = on_close.clone();
        move |_| {
            on_close.call(());
        }
    });

    let handle_close = Callback::<()>::new({
        let on_close = on_close.clone();
        move |_| {
            on_close.call(());
        }
    });

    view! {
        style {
            r#"
            .modal-overlay {
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background: #0d214599;
                backdrop-filter: blur(4px);
                z-index: 1000;
                display: flex;
                align-items: center;
                justify-content: center;
                opacity: 0;
                visibility: hidden;
                transition: all 0.3s ease;
            }
            .modal-overlay.show {
                opacity: 1;
                visibility: visible;
            }
            .modal {
                background: var(--surface);
                border-radius: var(--radius-lg);
                box-shadow: var(--shadow-lg);
                max-height: 90vh;
                overflow: hidden;
                transform: scale(0.9) translateY(-20px);
                transition: transform 0.3s ease;
            }
            .modal-overlay.show .modal {
                transform: scale(1) translateY(0);
            }
            .modal-small { width: 400px; max-width: 90vw; }
            .modal-medium { width: 600px; max-width: 90vw; }
            .modal-large { width: 900px; max-width: 90vw; }
            .modal-header {
                padding: 20px 24px;
                border-bottom: 1px solid var(--border);
                display: flex;
                align-items: center;
                justify-content: space-between;
            }
            .modal-title {
                font-family: Playfair Display, serif;
                font-size: 18px;
                font-weight: 600;
                color: var(--navy);
                margin: 0;
            }
            .modal-close {
                width: 32px;
                height: 32px;
                border: none;
                background: var(--bg);
                border-radius: var(--radius);
                cursor: pointer;
                font-size: 20px;
                color: var(--text-muted);
                display: flex;
                align-items: center;
                justify-content: center;
                transition: all 0.15s;
            }
            .modal-close:hover {
                background: var(--red-light);
                color: var(--red);
            }
            .modal-body {
                padding: 24px;
                overflow-y: auto;
                max-height: calc(90vh - 160px);
            }
            .modal-footer {
                padding: 16px 24px;
                border-top: 1px solid var(--border);
                display: flex;
                justify-content: flex-end;
                gap: 12px;
                background: var(--bg);
            }
            "#
        }

        <div class={overlay_class} data-testid="modal-overlay">
            <div class={modal_class} data-testid="modal">
                <div class="modal-header">
                    <h2 class="modal-title">{title}</h2>
                    <button class="modal-close" on:click={handle_close}>"×"</button>
                </div>
                <div class="modal-body">
                    for child in children {
                        {child}
                    }
                </div>
                if !footer_actions.is_empty() {
                    <div class="modal-footer">
                        for action in footer_actions {
                            {action}
                        }
                    </div>
                }
            </div>
        </div>
    }
}
