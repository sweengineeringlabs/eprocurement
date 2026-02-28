//! Top navigation bar component

use components::prelude::*;

/// Topbar with search, notifications, and user menu
#[component]
pub fn topbar() -> View {
    view! {
        style {
            r#"
            .topbar {
                position: sticky;
                top: 0;
                height: var(--topbar-height);
                background: var(--surface);
                border-bottom: 1px solid var(--border);
                display: flex;
                align-items: center;
                justify-content: space-between;
                padding: 0 24px;
                z-index: 50;
            }
            .topbar-left {
                display: flex;
                align-items: center;
                gap: 16px;
            }
            .search-box {
                display: flex;
                align-items: center;
                gap: 8px;
                background: var(--bg);
                border: 1px solid var(--border);
                border-radius: var(--radius);
                padding: 8px 12px;
                width: 280px;
            }
            .search-box svg {
                width: 16px;
                height: 16px;
                color: var(--text-muted);
            }
            .search-box input {
                border: none;
                background: transparent;
                outline: none;
                font-size: 13px;
                width: 100%;
            }
            .topbar-right {
                display: flex;
                align-items: center;
                gap: 16px;
            }
            .topbar-btn {
                width: 36px;
                height: 36px;
                border-radius: var(--radius);
                border: 1px solid var(--border);
                background: var(--surface);
                display: flex;
                align-items: center;
                justify-content: center;
                cursor: pointer;
                transition: all 0.15s;
                position: relative;
            }
            .topbar-btn:hover {
                background: var(--bg);
            }
            .topbar-btn svg {
                width: 18px;
                height: 18px;
                color: var(--text-muted);
            }
            .notification-dot {
                position: absolute;
                top: 6px;
                right: 6px;
                width: 8px;
                height: 8px;
                background: var(--red);
                border-radius: 50%;
                border: 2px solid var(--surface);
            }
            .user-menu {
                display: flex;
                align-items: center;
                gap: 10px;
                cursor: pointer;
            }
            .user-avatar {
                width: 36px;
                height: 36px;
                background: var(--blue);
                border-radius: 50%;
                display: flex;
                align-items: center;
                justify-content: center;
                color: #fff;
                font-weight: 600;
                font-size: 14px;
            }
            .user-info {
                text-align: right;
            }
            .user-name {
                font-weight: 500;
                font-size: 13px;
            }
            .user-role {
                font-size: 11px;
                color: var(--text-muted);
            }
            "#
        }

        <header class="topbar" data-testid="topbar">
            <div class="topbar-left">
                <div class="search-box">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <circle cx="11" cy="11" r="8"/>
                        <line x1="21" y1="21" x2="16.65" y2="16.65"/>
                    </svg>
                    <input type="text" placeholder="Search requisitions, tenders, suppliers..." />
                </div>
            </div>
            <div class="topbar-right">
                <button class="topbar-btn" title="Notifications">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"/>
                        <path d="M13.73 21a2 2 0 0 1-3.46 0"/>
                    </svg>
                    <span class="notification-dot"></span>
                </button>
                <button class="topbar-btn" title="Help">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <circle cx="12" cy="12" r="10"/>
                        <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"/>
                        <line x1="12" y1="17" x2="12.01" y2="17"/>
                    </svg>
                </button>
                <div class="user-menu">
                    <div class="user-info">
                        <div class="user-name">"Thabo Mokoena"</div>
                        <div class="user-role">"Procurement Manager"</div>
                    </div>
                    <div class="user-avatar">"TM"</div>
                </div>
            </div>
        </header>
    }
}
