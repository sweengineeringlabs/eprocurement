//! Notice bar component for alerts and messages

use components::prelude::*;

/// Notice type variants
#[derive(Clone, Copy, PartialEq, Default)]
pub enum NoticeType {
    #[default]
    Info,
    Warning,
    Success,
    Error,
}

impl NoticeType {
    fn class(&self) -> &'static str {
        match self {
            NoticeType::Info => "info",
            NoticeType::Warning => "warning",
            NoticeType::Success => "success",
            NoticeType::Error => "error",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            NoticeType::Info => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>"#,
            NoticeType::Warning => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>"#,
            NoticeType::Success => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>"#,
            NoticeType::Error => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>"#,
        }
    }
}

/// Notice bar component
#[component]
pub fn notice_bar(message: String, notice_type: NoticeType, on_dismiss: Option<Callback<()>>) -> View {
    let class = format!("notice-bar {}", notice_type.class());
    let icon = notice_type.icon();
    let has_dismiss = on_dismiss.is_some();
    let handle_dismiss = Callback::new({
        let on_dismiss = on_dismiss.clone();
        move |_| {
            if let Some(cb) = &on_dismiss {
                cb.call(());
            }
        }
    });

    view! {
        style {
            r#"
            .notice-bar {
                display: flex;
                align-items: center;
                gap: 12px;
                padding: 12px 16px;
                border-radius: var(--radius);
                margin-bottom: 20px;
            }
            .notice-bar.info {
                background: var(--blue-light);
                color: var(--blue);
            }
            .notice-bar.warning {
                background: var(--orange-light);
                color: var(--orange);
            }
            .notice-bar.success {
                background: var(--green-light);
                color: var(--green);
            }
            .notice-bar.error {
                background: var(--red-light);
                color: var(--red);
            }
            .notice-bar svg {
                width: 18px;
                height: 18px;
                flex-shrink: 0;
            }
            .notice-bar-message {
                flex: 1;
                font-size: 13px;
            }
            .notice-bar-dismiss {
                background: none;
                border: none;
                cursor: pointer;
                padding: 4px;
                opacity: 0.7;
                color: currentColor;
            }
            .notice-bar-dismiss:hover {
                opacity: 1;
            }
            "#
        }

        <div class={class} data-testid="notice-bar">
            <span inner_html={icon}></span>
            <span class="notice-bar-message">{message}</span>
            if has_dismiss {
                <button class="notice-bar-dismiss" on:click={handle_dismiss}>
                    "×"
                </button>
            }
        </div>
    }
}
