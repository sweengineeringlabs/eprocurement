//! Mobile App Preview and Configuration Page
//!
//! Allows administrators to preview and configure the mobile supplier app.

use components::prelude::*;
use crate::shared::layout::page_header;
use crate::shared::components::{
    kpi_card, KpiColor, KpiDelta,
    panel, data_table, DataTableColumn, DataTableRow,
    status_badge, StatusType,
    progress_bar, ProgressColor,
    notice_bar, NoticeType,
};
use crate::shared::forms::{text_input, select, SelectOption, checkbox};
use crate::util::format::format_date;
use super::store::{MobileStore, load_mock_mobile_data};
use super::types::{
    AppFeature, DeviceType, PreviewMode, PreviewScreen,
    NotificationCategory,
};
use super::service;

/// Mobile App Preview and Configuration component
#[component]
pub fn mobile_app() -> View {
    let store = use_context::<MobileStore>();

    // Load data on mount
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                service::load_mobile_data(&store).await;
            });
        }
    });

    let config = store.config.clone();
    let config_draft = store.config_draft.clone();
    let stats = store.stats.clone();
    let users = store.users.clone();
    let active_tab = store.active_tab.clone();
    let preview_mode = store.preview_mode.clone();
    let preview_screen = store.preview_screen.clone();
    let preview_dark_mode = store.preview_dark_mode.clone();
    let config_dirty = store.config_dirty.clone();
    let saving = store.saving.clone();

    // Tab handler
    let set_tab = {
        let store = store.clone();
        Callback::new(move |tab: String| {
            store.set_active_tab(&tab);
        })
    };

    // Preview mode handler
    let set_preview = {
        let store = store.clone();
        Callback::new(move |mode: PreviewMode| {
            store.set_preview_mode(mode);
        })
    };

    // Preview screen handler
    let set_screen = {
        let store = store.clone();
        Callback::new(move |screen: PreviewScreen| {
            store.set_preview_screen(screen);
        })
    };

    // Dark mode toggle
    let toggle_dark = {
        let store = store.clone();
        Callback::<()>::new(move |_| {
            store.toggle_preview_dark_mode();
        })
    };

    // Save config handler
    let save_config = {
        let store = store.clone();
        Callback::<()>::new(move |_| {
            let store = store.clone();
            spawn(async move {
                if let Err(e) = service::save_config(&store).await {
                    store.error.set(Some(e));
                }
            });
        })
    };

    // Reset config handler
    let reset_config = {
        let store = store.clone();
        Callback::<()>::new(move |_| {
            store.reset_config_draft();
        })
    };

    // Feature toggle handler
    let toggle_feature = {
        let store = store.clone();
        Callback::new(move |feature: AppFeature| {
            store.toggle_feature(feature);
        })
    };

    // Icons
    let icon_smartphone = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="5" y="2" width="14" height="20" rx="2" ry="2"/><line x1="12" y1="18" x2="12.01" y2="18"/></svg>"#;
    let icon_users = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>"#;
    let icon_bell = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"/><path d="M13.73 21a2 2 0 0 1-3.46 0"/></svg>"#;
    let icon_download = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>"#;
    let icon_activity = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="22 12 18 12 15 21 9 3 6 12 2 12"/></svg>"#;
    let icon_file = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>"#;
    let icon_camera = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z"/><circle cx="12" cy="13" r="4"/></svg>"#;
    let icon_check = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>"#;

    // Pre-compute percentage calculations for use in view! block
    let push_enabled_percent = if stats.get().total_users > 0 {
        (stats.get().push_enabled_users as f64 / stats.get().total_users as f64) * 100.0
    } else { 0.0 };

    let ios_percent = if stats.get().total_users > 0 {
        (stats.get().ios_users as f64 / stats.get().total_users as f64) * 100.0
    } else { 0.0 };

    let android_percent = if stats.get().total_users > 0 {
        (stats.get().android_users as f64 / stats.get().total_users as f64) * 100.0
    } else { 0.0 };

    let ios_users_count = stats.get().ios_users;
    let android_users_count = stats.get().android_users;

    view! {
        style {
            r#"
            .mobile-app-page {
                display: flex;
                flex-direction: column;
                gap: var(--space-6);
            }
            .kpi-grid {
                display: grid;
                grid-template-columns: repeat(4, 1fr);
                gap: 16px;
            }
            @media (max-width: 1200px) {
                .kpi-grid { grid-template-columns: repeat(2, 1fr); }
            }
            @media (max-width: 768px) {
                .kpi-grid { grid-template-columns: 1fr; }
            }
            .mobile-tabs {
                display: flex;
                gap: 8px;
                border-bottom: 1px solid var(--border);
                padding-bottom: 0;
                margin-bottom: 24px;
            }
            .mobile-tab {
                padding: 12px 24px;
                background: none;
                border: none;
                color: var(--text-muted);
                font-size: 14px;
                font-weight: 500;
                cursor: pointer;
                border-bottom: 2px solid transparent;
                margin-bottom: -1px;
                transition: all 0.2s;
            }
            .mobile-tab:hover {
                color: var(--text);
            }
            .mobile-tab.active {
                color: var(--blue);
                border-bottom-color: var(--blue);
            }
            .tab-content {
                display: none;
            }
            .tab-content.active {
                display: block;
            }
            .preview-container {
                display: flex;
                gap: 32px;
            }
            .preview-sidebar {
                width: 280px;
                flex-shrink: 0;
            }
            .preview-main {
                flex: 1;
                display: flex;
                justify-content: center;
                align-items: flex-start;
            }
            .device-frame {
                position: relative;
                background: #1a1a1a;
                border-radius: 40px;
                padding: 12px;
                box-shadow: 0 25px 50px -12px #00000066;
            }
            .device-frame.iphone {
                width: 375px;
            }
            .device-frame.android {
                width: 360px;
                border-radius: 24px;
            }
            .device-frame.ipad {
                width: 768px;
                border-radius: 24px;
            }
            .device-screen {
                background: var(--surface);
                border-radius: 28px;
                overflow: hidden;
                height: 700px;
            }
            .device-frame.ipad .device-screen {
                height: 900px;
            }
            .device-frame.dark .device-screen {
                background: #0f0f0f;
            }
            .device-notch {
                position: absolute;
                top: 12px;
                left: 50%;
                transform: translateX(-50%);
                width: 150px;
                height: 30px;
                background: #1a1a1a;
                border-radius: 0 0 20px 20px;
                z-index: 10;
            }
            .device-home-indicator {
                position: absolute;
                bottom: 8px;
                left: 50%;
                transform: translateX(-50%);
                width: 134px;
                height: 5px;
                background: #333;
                border-radius: 3px;
            }
            .app-header {
                background: var(--navy);
                color: white;
                padding: 60px 16px 16px;
            }
            .device-frame.dark .app-header {
                background: #1a1a2e;
            }
            .app-header-title {
                font-size: 20px;
                font-weight: 600;
            }
            .app-header-subtitle {
                font-size: 12px;
                opacity: 0.8;
                margin-top: 4px;
            }
            .app-content {
                padding: 16px;
            }
            .app-kpi-grid {
                display: grid;
                grid-template-columns: repeat(2, 1fr);
                gap: 12px;
                margin-bottom: 16px;
            }
            .app-kpi-card {
                background: var(--bg);
                border-radius: 12px;
                padding: 12px;
            }
            .device-frame.dark .app-kpi-card {
                background: #1a1a2e;
            }
            .app-kpi-value {
                font-size: 24px;
                font-weight: 600;
                color: var(--navy);
            }
            .device-frame.dark .app-kpi-value {
                color: white;
            }
            .app-kpi-label {
                font-size: 11px;
                color: var(--text-muted);
            }
            .app-section-title {
                font-size: 14px;
                font-weight: 600;
                color: var(--navy);
                margin-bottom: 12px;
            }
            .device-frame.dark .app-section-title {
                color: white;
            }
            .app-list-item {
                display: flex;
                align-items: center;
                gap: 12px;
                padding: 12px;
                background: var(--bg);
                border-radius: 8px;
                margin-bottom: 8px;
            }
            .device-frame.dark .app-list-item {
                background: #1a1a2e;
            }
            .app-list-icon {
                width: 40px;
                height: 40px;
                border-radius: 8px;
                background: var(--blue-light);
                display: flex;
                align-items: center;
                justify-content: center;
            }
            .app-list-content {
                flex: 1;
            }
            .app-list-title {
                font-size: 13px;
                font-weight: 500;
                color: var(--text);
            }
            .device-frame.dark .app-list-title {
                color: white;
            }
            .app-list-meta {
                font-size: 11px;
                color: var(--text-muted);
            }
            .app-nav-bar {
                position: absolute;
                bottom: 20px;
                left: 12px;
                right: 12px;
                display: flex;
                justify-content: space-around;
                background: var(--surface);
                border-radius: 16px;
                padding: 12px;
                box-shadow: 0 -4px 20px #0000001A;
            }
            .device-frame.dark .app-nav-bar {
                background: #1a1a2e;
            }
            .app-nav-item {
                display: flex;
                flex-direction: column;
                align-items: center;
                gap: 4px;
                color: var(--text-muted);
                font-size: 10px;
            }
            .app-nav-item.active {
                color: var(--blue);
            }
            .preview-controls {
                display: flex;
                flex-direction: column;
                gap: 16px;
            }
            .control-group {
                display: flex;
                flex-direction: column;
                gap: 8px;
            }
            .control-label {
                font-size: 12px;
                font-weight: 500;
                color: var(--text-muted);
            }
            .control-buttons {
                display: flex;
                gap: 4px;
                flex-wrap: wrap;
            }
            .control-btn {
                padding: 8px 12px;
                background: var(--bg);
                border: 1px solid var(--border);
                border-radius: var(--radius);
                font-size: 12px;
                cursor: pointer;
                transition: all 0.2s;
            }
            .control-btn:hover {
                background: var(--border);
            }
            .control-btn.active {
                background: var(--blue);
                border-color: var(--blue);
                color: white;
            }
            .screen-list {
                display: flex;
                flex-direction: column;
                gap: 4px;
                max-height: 400px;
                overflow-y: auto;
            }
            .screen-item {
                padding: 8px 12px;
                background: var(--bg);
                border-radius: var(--radius);
                font-size: 12px;
                cursor: pointer;
                transition: all 0.2s;
            }
            .screen-item:hover {
                background: var(--border);
            }
            .screen-item.active {
                background: var(--blue-light);
                color: var(--blue);
            }
            .config-section {
                margin-bottom: 24px;
            }
            .config-section-title {
                font-size: 14px;
                font-weight: 600;
                color: var(--navy);
                margin-bottom: 16px;
                padding-bottom: 8px;
                border-bottom: 1px solid var(--border);
            }
            .config-grid {
                display: grid;
                grid-template-columns: repeat(2, 1fr);
                gap: 16px;
            }
            .feature-grid {
                display: grid;
                grid-template-columns: repeat(2, 1fr);
                gap: 12px;
            }
            @media (max-width: 768px) {
                .config-grid, .feature-grid { grid-template-columns: 1fr; }
            }
            .feature-card {
                display: flex;
                align-items: flex-start;
                gap: 12px;
                padding: 12px;
                background: var(--bg);
                border: 1px solid var(--border);
                border-radius: var(--radius);
                cursor: pointer;
                transition: all 0.2s;
            }
            .feature-card:hover {
                border-color: var(--blue);
            }
            .feature-card.enabled {
                background: var(--blue-light);
                border-color: var(--blue);
            }
            .feature-checkbox {
                margin-top: 2px;
            }
            .feature-info {
                flex: 1;
            }
            .feature-name {
                font-size: 13px;
                font-weight: 500;
                color: var(--text);
            }
            .feature-description {
                font-size: 11px;
                color: var(--text-muted);
                margin-top: 2px;
            }
            .feature-category {
                font-size: 10px;
                color: var(--blue);
                text-transform: uppercase;
                letter-spacing: 0.5px;
                margin-top: 4px;
            }
            .color-picker-row {
                display: flex;
                align-items: center;
                gap: 12px;
            }
            .color-preview {
                width: 40px;
                height: 40px;
                border-radius: var(--radius);
                border: 2px solid var(--border);
            }
            .config-actions {
                display: flex;
                gap: 12px;
                padding-top: 24px;
                border-top: 1px solid var(--border);
            }
            .user-device-badge {
                display: inline-flex;
                align-items: center;
                gap: 4px;
                padding: 4px 8px;
                border-radius: 12px;
                font-size: 11px;
                font-weight: 500;
            }
            .user-device-badge.ios {
                background: var(--text-muted);
                color: white;
            }
            .user-device-badge.android {
                background: var(--green-light);
                color: var(--green);
            }
            "#
        }

        <div class="mobile-app-page" data-testid="mobile-app-page">
            {page_header(
                "Mobile Supplier App".to_string(),
                Some("Preview and configure the mobile app for suppliers".to_string()),
                vec![
                    view! { <button class="btn btn-secondary">"Download QR Code"</button> },
                    view! { <button class="btn btn-primary">"Publish Update"</button> },
                ]
            )}

            // KPI Cards
            <div class="kpi-grid">
                {kpi_card(
                    "Total App Users".to_string(),
                    stats.get().total_users.to_string(),
                    KpiColor::Blue,
                    icon_users.to_string(),
                    Some(KpiDelta { value: format!("{} active", stats.get().active_users_30d), is_positive: None, suffix: "in last 30 days".to_string() }),
                    None
                )}
                {kpi_card(
                    "Push Enabled".to_string(),
                    stats.get().push_enabled_users.to_string(),
                    KpiColor::Green,
                    icon_bell.to_string(),
                    Some(KpiDelta { value: format!("{:.0}%", push_enabled_percent), is_positive: Some(true), suffix: "of users".to_string() }),
                    None
                )}
                {kpi_card(
                    "Mobile Bids".to_string(),
                    stats.get().bids_submitted_mobile.to_string(),
                    KpiColor::Orange,
                    icon_file.to_string(),
                    Some(KpiDelta { value: "This month".to_string(), is_positive: None, suffix: "".to_string() }),
                    None
                )}
                {kpi_card(
                    "Docs Scanned".to_string(),
                    stats.get().documents_scanned.to_string(),
                    KpiColor::Purple,
                    icon_camera.to_string(),
                    Some(KpiDelta { value: "12%".to_string(), is_positive: Some(true), suffix: "from last month".to_string() }),
                    None
                )}
            </div>

            // Tabs
            <div class="mobile-tabs">
                <button
                    class={if active_tab.get() == "preview" { "mobile-tab active" } else { "mobile-tab" }}
                    on:click={let set_tab = set_tab.clone(); move || set_tab.call("preview".to_string())}
                >
                    "App Preview"
                </button>
                <button
                    class={if active_tab.get() == "config" { "mobile-tab active" } else { "mobile-tab" }}
                    on:click={let set_tab = set_tab.clone(); move || set_tab.call("config".to_string())}
                >
                    "Configuration"
                </button>
                <button
                    class={if active_tab.get() == "features" { "mobile-tab active" } else { "mobile-tab" }}
                    on:click={let set_tab = set_tab.clone(); move || set_tab.call("features".to_string())}
                >
                    "Features"
                </button>
                <button
                    class={if active_tab.get() == "users" { "mobile-tab active" } else { "mobile-tab" }}
                    on:click={let set_tab = set_tab.clone(); move || set_tab.call("users".to_string())}
                >
                    "Users"
                </button>
            </div>

            // Tab Content - Preview
            <div class={if active_tab.get() == "preview" { "tab-content active" } else { "tab-content" }}>
                <div class="preview-container">
                    <div class="preview-sidebar">
                        {panel(
                            "Preview Controls".to_string(),
                            vec![],
                            vec![view! {
                                <div class="preview-controls">
                                    <div class="control-group">
                                        <span class="control-label">"Device"</span>
                                        <div class="control-buttons">
                                            for mode in PreviewMode::all() {
                                                <button
                                                    class={if preview_mode.get() == mode { "control-btn active" } else { "control-btn" }}
                                                    on:click={let set_preview = set_preview.clone(); move || set_preview.call(mode)}
                                                >
                                                    {mode.label()}
                                                </button>
                                            }
                                        </div>
                                    </div>

                                    <div class="control-group">
                                        <span class="control-label">"Theme"</span>
                                        <div class="control-buttons">
                                            <button
                                                class={if !preview_dark_mode.get() { "control-btn active" } else { "control-btn" }}
                                                on:click={let toggle_dark = toggle_dark.clone(); move || if preview_dark_mode.get() { toggle_dark.call(()) }}
                                            >
                                                "Light"
                                            </button>
                                            <button
                                                class={if preview_dark_mode.get() { "control-btn active" } else { "control-btn" }}
                                                on:click={let toggle_dark = toggle_dark.clone(); move || if !preview_dark_mode.get() { toggle_dark.call(()) }}
                                            >
                                                "Dark"
                                            </button>
                                        </div>
                                    </div>

                                    <div class="control-group">
                                        <span class="control-label">"Screen"</span>
                                        <div class="screen-list">
                                            for screen in PreviewScreen::all() {
                                                <div
                                                    class={if preview_screen.get() == screen { "screen-item active" } else { "screen-item" }}
                                                    on:click={let set_screen = set_screen.clone(); move || set_screen.call(screen)}
                                                >
                                                    {screen.label()}
                                                </div>
                                            }
                                        </div>
                                    </div>
                                </div>
                            }]
                        )}
                    </div>

                    <div class="preview-main">
                        <div class={format!("device-frame {} {}",
                            if preview_mode.get() == PreviewMode::IPhone || preview_mode.get() == PreviewMode::IPad { "iphone" } else { "android" },
                            if preview_dark_mode.get() { "dark" } else { "" }
                        )}>
                            if preview_mode.get() == PreviewMode::IPhone {
                                <div class="device-notch"></div>
                            }

                            <div class="device-screen">
                                // App Header
                                <div class="app-header" style={format!("background: {};", config_draft.get().primary_color)}>
                                    <div class="app-header-title">{config_draft.get().app_name}</div>
                                    <div class="app-header-subtitle">"Welcome, TechSolutions SA"</div>
                                </div>

                                // App Content
                                <div class="app-content">
                                    <div class="app-kpi-grid">
                                        <div class="app-kpi-card">
                                            <div class="app-kpi-value" style={format!("color: {};", config_draft.get().secondary_color)}>"5"</div>
                                            <div class="app-kpi-label">"Open Tenders"</div>
                                        </div>
                                        <div class="app-kpi-card">
                                            <div class="app-kpi-value" style={format!("color: {};", config_draft.get().secondary_color)}>"3"</div>
                                            <div class="app-kpi-label">"Active Bids"</div>
                                        </div>
                                        <div class="app-kpi-card">
                                            <div class="app-kpi-value" style={format!("color: {};", config_draft.get().secondary_color)}>"2"</div>
                                            <div class="app-kpi-label">"Contracts"</div>
                                        </div>
                                        <div class="app-kpi-card">
                                            <div class="app-kpi-value" style={format!("color: {};", config_draft.get().secondary_color)}>"1"</div>
                                            <div class="app-kpi-label">"Notifications"</div>
                                        </div>
                                    </div>

                                    <div class="app-section-title">"Recent Opportunities"</div>

                                    <div class="app-list-item">
                                        <div class="app-list-icon" style={format!("background: {}20;", config_draft.get().secondary_color)}>
                                            <span style={format!("color: {};", config_draft.get().secondary_color)}>"IT"</span>
                                        </div>
                                        <div class="app-list-content">
                                            <div class="app-list-title">"IT Infrastructure Upgrade"</div>
                                            <div class="app-list-meta">"Closes in 5 days - R15M"</div>
                                        </div>
                                    </div>

                                    <div class="app-list-item">
                                        <div class="app-list-icon" style={format!("background: {}20;", config_draft.get().secondary_color)}>
                                            <span style={format!("color: {};", config_draft.get().secondary_color)}>"SEC"</span>
                                        </div>
                                        <div class="app-list-content">
                                            <div class="app-list-title">"Security Services Contract"</div>
                                            <div class="app-list-meta">"Closes in 12 days - R8M"</div>
                                        </div>
                                    </div>

                                    <div class="app-list-item">
                                        <div class="app-list-icon" style={format!("background: {}20;", config_draft.get().secondary_color)}>
                                            <span style={format!("color: {};", config_draft.get().secondary_color)}>"FAC"</span>
                                        </div>
                                        <div class="app-list-content">
                                            <div class="app-list-title">"Office Furniture Supply"</div>
                                            <div class="app-list-meta">"Closes in 3 days - R2.5M"</div>
                                        </div>
                                    </div>
                                </div>

                                // Bottom Navigation
                                <div class="app-nav-bar">
                                    <div class="app-nav-item active">
                                        <span inner_html={icon_smartphone}></span>
                                        "Home"
                                    </div>
                                    <div class="app-nav-item">
                                        <span inner_html={icon_file}></span>
                                        "Tenders"
                                    </div>
                                    <div class="app-nav-item">
                                        <span inner_html={icon_activity}></span>
                                        "Bids"
                                    </div>
                                    <div class="app-nav-item">
                                        <span inner_html={icon_bell}></span>
                                        "Alerts"
                                    </div>
                                </div>
                            </div>

                            <div class="device-home-indicator"></div>
                        </div>
                    </div>
                </div>
            </div>

            // Tab Content - Configuration
            <div class={if active_tab.get() == "config" { "tab-content active" } else { "tab-content" }}>
                {panel(
                    "App Configuration".to_string(),
                    vec![],
                    vec![view! {
                        <div>
                            if config_dirty.get() {
                                {notice_bar(
                                    "You have unsaved changes".to_string(),
                                    NoticeType::Warning,
                                    None
                                )}
                            }

                            <div class="config-section">
                                <div class="config-section-title">"Branding"</div>
                                <div class="config-grid">
                                    <div>
                                        {text_input(
                                            "App Name".to_string(),
                                            signal(config_draft.get().app_name.clone()),
                                            None,
                                            false,
                                            false,
                                            None,
                                            None,
                                            Some("text".to_string()),
                                        )}
                                    </div>
                                    <div>
                                        {text_input(
                                            "Organization Name".to_string(),
                                            signal(config_draft.get().organization_name.clone()),
                                            None,
                                            false,
                                            false,
                                            None,
                                            None,
                                            Some("text".to_string()),
                                        )}
                                    </div>
                                    <div>
                                        <label class="form-label">"Primary Color"</label>
                                        <div class="color-picker-row">
                                            <div class="color-preview" style={format!("background: {};", config_draft.get().primary_color)}></div>
                                            <input type="color" value={config_draft.get().primary_color} style="flex: 1; height: 40px;" />
                                        </div>
                                    </div>
                                    <div>
                                        <label class="form-label">"Secondary Color"</label>
                                        <div class="color-picker-row">
                                            <div class="color-preview" style={format!("background: {};", config_draft.get().secondary_color)}></div>
                                            <input type="color" value={config_draft.get().secondary_color} style="flex: 1; height: 40px;" />
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <div class="config-section">
                                <div class="config-section-title">"Security"</div>
                                <div class="config-grid">
                                    <div>
                                        {select(
                                            "Session Timeout".to_string(),
                                            signal(config_draft.get().session_timeout_minutes.to_string()),
                                            vec![
                                                SelectOption { value: "15".to_string(), label: "15 minutes".to_string() },
                                                SelectOption { value: "30".to_string(), label: "30 minutes".to_string() },
                                                SelectOption { value: "60".to_string(), label: "1 hour".to_string() },
                                                SelectOption { value: "120".to_string(), label: "2 hours".to_string() },
                                            ],
                                            None,
                                            false,
                                            false,
                                            None,
                                        )}
                                    </div>
                                    <div style="display: flex; flex-direction: column; justify-content: flex-end;">
                                        {checkbox(
                                            "Require Biometric Authentication".to_string(),
                                            signal(config_draft.get().require_biometric),
                                            false,
                                        )}
                                    </div>
                                </div>
                            </div>

                            <div class="config-section">
                                <div class="config-section-title">"Offline Mode"</div>
                                <div class="config-grid">
                                    <div>
                                        {checkbox(
                                            "Enable Offline Mode".to_string(),
                                            signal(config_draft.get().offline_mode_enabled),
                                            false,
                                        )}
                                    </div>
                                    <div>
                                        {select(
                                            "Max Offline Duration".to_string(),
                                            signal(config_draft.get().max_offline_days.to_string()),
                                            vec![
                                                SelectOption { value: "1".to_string(), label: "1 day".to_string() },
                                                SelectOption { value: "3".to_string(), label: "3 days".to_string() },
                                                SelectOption { value: "7".to_string(), label: "7 days".to_string() },
                                                SelectOption { value: "14".to_string(), label: "14 days".to_string() },
                                            ],
                                            None,
                                            false,
                                            false,
                                            None,
                                        )}
                                    </div>
                                </div>
                            </div>

                            <div class="config-section">
                                <div class="config-section-title">"Support"</div>
                                <div class="config-grid">
                                    <div>
                                        {text_input(
                                            "Support Email".to_string(),
                                            signal(config_draft.get().support_email.clone()),
                                            None,
                                            false,
                                            false,
                                            None,
                                            None,
                                            Some("email".to_string()),
                                        )}
                                    </div>
                                    <div>
                                        {text_input(
                                            "Support Phone".to_string(),
                                            signal(config_draft.get().support_phone.clone()),
                                            None,
                                            false,
                                            false,
                                            None,
                                            None,
                                            Some("tel".to_string()),
                                        )}
                                    </div>
                                </div>
                            </div>

                            <div class="config-actions">
                                <button class="btn btn-secondary" on:click={let reset_config = reset_config.clone(); move || reset_config.call(())}>"Reset"</button>
                                <button class="btn btn-primary" disabled={!config_dirty.get() || saving.get()} on:click={let save_config = save_config.clone(); move || save_config.call(())}>
                                    {if saving.get() { "Saving..." } else { "Save Configuration" }}
                                </button>
                            </div>
                        </div>
                    }]
                )}
            </div>

            // Tab Content - Features
            <div class={if active_tab.get() == "features" { "tab-content active" } else { "tab-content" }}>
                {panel(
                    "App Features".to_string(),
                    vec![
                        view! { <button class="btn btn-sm btn-secondary">"Enable All"</button> },
                        view! { <button class="btn btn-sm btn-secondary">"Disable All"</button> },
                    ],
                    vec![view! {
                        <div>
                            <p style="color: var(--text-muted); margin-bottom: 24px;">
                                "Enable or disable features available to suppliers in the mobile app."
                            </p>

                            <div class="feature-grid">
                                for feature in AppFeature::all() {
                                    <div
                                        class={if config_draft.get().features_enabled.contains(&feature) { "feature-card enabled" } else { "feature-card" }}
                                        on:click={let toggle_feature = toggle_feature.clone(); move || toggle_feature.call(feature)}
                                    >
                                        <input
                                            type="checkbox"
                                            class="feature-checkbox"
                                            checked={config_draft.get().features_enabled.contains(&feature)}
                                        />
                                        <div class="feature-info">
                                            <div class="feature-name">{feature.label()}</div>
                                            <div class="feature-description">{feature.description()}</div>
                                            <div class="feature-category">{feature.category()}</div>
                                        </div>
                                    </div>
                                }
                            </div>

                            <div class="config-actions">
                                <button class="btn btn-secondary" on:click={let reset_config = reset_config.clone(); move || reset_config.call(())}>"Reset"</button>
                                <button class="btn btn-primary" disabled={!config_dirty.get() || saving.get()} on:click={let save_config = save_config.clone(); move || save_config.call(())}>
                                    {if saving.get() { "Saving..." } else { "Save Features" }}
                                </button>
                            </div>
                        </div>
                    }]
                )}
            </div>

            // Tab Content - Users
            <div class={if active_tab.get() == "users" { "tab-content active" } else { "tab-content" }}>
                {panel(
                    "Mobile App Users".to_string(),
                    vec![
                        view! { <button class="btn btn-sm btn-secondary">"Export"</button> },
                        view! { <button class="btn btn-sm btn-secondary">"Send Notification"</button> },
                    ],
                    vec![{
                        let columns = vec![
                            DataTableColumn { key: "supplier".to_string(), label: "Supplier".to_string(), width: None, align: None, cell_class: None },
                            DataTableColumn { key: "email".to_string(), label: "Email".to_string(), width: None, align: None, cell_class: None },
                            DataTableColumn { key: "device".to_string(), label: "Device".to_string(), width: None, align: None, cell_class: None },
                            DataTableColumn { key: "version".to_string(), label: "App Version".to_string(), width: None, align: None, cell_class: None },
                            DataTableColumn { key: "last_active".to_string(), label: "Last Active".to_string(), width: None, align: None, cell_class: None },
                            DataTableColumn { key: "status".to_string(), label: "Status".to_string(), width: None, align: None, cell_class: None },
                            DataTableColumn { key: "actions".to_string(), label: "Actions".to_string(), width: None, align: None, cell_class: None },
                        ];

                        let rows: Vec<DataTableRow> = users.get().iter().map(|user| {
                            DataTableRow {
                                id: user.id.clone(),
                                cells: vec![
                                    view! { <span style="font-weight: 500;">{user.supplier_name.clone()}</span> },
                                    view! { <span>{user.email.clone()}</span> },
                                    view! {
                                        <span class={if user.device_type == DeviceType::IOS { "user-device-badge ios" } else { "user-device-badge android" }}>
                                            {user.device_type.label()}
                                            {
                                                if let Some(model) = user.device_model.clone() {
                                                    view! { <span style="opacity: 0.8;">{format!(" - {}", model)}</span> }
                                                } else {
                                                    view! {}
                                                }
                                            }
                                        </span>
                                    },
                                    view! { <span>{user.app_version.clone()}</span> },
                                    view! { <span>{format_date(&user.last_active_at)}</span> },
                                    view! {
                                        if user.is_active {
                                            {status_badge(StatusType::Approved)}
                                        } else {
                                            {status_badge(StatusType::Cancelled)}
                                        }
                                    },
                                    view! {
                                        <div style="display: flex; gap: 4px;">
                                            <button class="btn btn-sm btn-secondary">"View"</button>
                                            if user.push_token.is_some() {
                                                <button class="btn btn-sm btn-secondary">"Notify"</button>
                                            }
                                        </div>
                                    },
                                ],
                            }
                        }).collect();

                        data_table(columns, rows, None)
                    }]
                )}

                // Device breakdown
                <div class="grid-2" style="margin-top: 24px;">
                    {panel(
                        "Device Breakdown".to_string(),
                        vec![],
                        vec![view! {
                            <div style="display: flex; flex-direction: column; gap: 16px;">
                                <div>
                                    <div style="display: flex; justify-content: space-between; margin-bottom: 8px;">
                                        <span>"iOS"</span>
                                        <span>{format!("{} users ({:.0}%)", ios_users_count, ios_percent)}</span>
                                    </div>
                                    {progress_bar(ios_percent, ProgressColor::Gray, false, None)}
                                </div>
                                <div>
                                    <div style="display: flex; justify-content: space-between; margin-bottom: 8px;">
                                        <span>"Android"</span>
                                        <span>{format!("{} users ({:.0}%)", android_users_count, android_percent)}</span>
                                    </div>
                                    {progress_bar(android_percent, ProgressColor::Green, false, None)}
                                </div>
                            </div>
                        }]
                    )}

                    {panel(
                        "Feature Usage".to_string(),
                        vec![],
                        vec![view! {
                            <div style="display: flex; flex-direction: column; gap: 12px;">
                                <div style="display: flex; justify-content: space-between; align-items: center;">
                                    <span>"Bids Submitted"</span>
                                    <span style="font-weight: 600;">{stats.get().bids_submitted_mobile}</span>
                                </div>
                                <div style="display: flex; justify-content: space-between; align-items: center;">
                                    <span>"Documents Scanned"</span>
                                    <span style="font-weight: 600;">{stats.get().documents_scanned}</span>
                                </div>
                                <div style="display: flex; justify-content: space-between; align-items: center;">
                                    <span>"Briefing Check-ins"</span>
                                    <span style="font-weight: 600;">{stats.get().briefing_checkins}</span>
                                </div>
                                <div style="display: flex; justify-content: space-between; align-items: center;">
                                    <span>"Deliveries Confirmed"</span>
                                    <span style="font-weight: 600;">{stats.get().deliveries_confirmed}</span>
                                </div>
                                <div style="display: flex; justify-content: space-between; align-items: center;">
                                    <span>"Auction Participations"</span>
                                    <span style="font-weight: 600;">{stats.get().auction_participations}</span>
                                </div>
                            </div>
                        }]
                    )}
                </div>
            </div>
        </div>
    }
}
