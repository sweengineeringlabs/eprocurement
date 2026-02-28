//! Tab components for navigation

use components::prelude::*;

/// A single tab item
#[derive(Clone, Default)]
pub struct Tab {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub active: bool,
}

impl Tab {
    pub fn new(id: &str, label: &str) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            icon: None,
            active: false,
        }
    }

    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }

    pub fn with_active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

/// Tab bar component
#[component]
pub fn tab_bar(
    tabs: Vec<Tab>,
    active_id: String,
    on_change: Callback<String>,
) -> View {
    let tab_views: Vec<View> = tabs.iter().map(|tab| {
        tab_item(tab.clone(), active_id.clone(), on_change.clone())
    }).collect();

    view! {
        style {
            r#"
            .tab-bar {
                display: flex;
                gap: 4px;
                border-bottom: 1px solid var(--border);
                padding: 0 16px;
            }
            .tab-item {
                padding: 12px 16px;
                cursor: pointer;
                border-bottom: 2px solid transparent;
                color: var(--text-muted);
                font-size: 14px;
                transition: color 0.2s, border-color 0.2s;
                display: flex;
                align-items: center;
                gap: 8px;
            }
            .tab-item:hover {
                color: var(--text);
            }
            .tab-item.active {
                color: var(--accent);
                border-bottom-color: var(--accent);
            }
            .tab-icon {
                width: 16px;
                height: 16px;
            }
            .tab-icon svg {
                width: 100%;
                height: 100%;
            }
            "#
        }

        <div class="tab-bar" data-testid="tab-bar">
            for view in tab_views.iter() {
                {view.clone()}
            }
        </div>
    }
}

fn tab_item(tab: Tab, active_id: String, on_change: Callback<String>) -> View {
    let tab_id = tab.id.clone();
    let is_active = tab_id == active_id || tab.active;
    let class = if is_active { "tab-item active".to_string() } else { "tab-item".to_string() };
    let icon = tab.icon.clone();
    let label = tab.label.clone();
    let test_id = format!("tab-{}", tab.id);

    let handle_click = Callback::<()>::new({
        let on_change = on_change.clone();
        let tab_id = tab_id.clone();
        move |_| on_change.call(tab_id.clone())
    });

    view! {
        <div
            class={class}
            on:click={handle_click}
            data-testid={test_id}
        >
            if let Some(ref ic) = icon {
                <span class="tab-icon" inner_html={ic.clone()}></span>
            }
            {label}
        </div>
    }
}

/// Tabs container with content panels
#[component]
pub fn tabs(
    tabs_list: Vec<Tab>,
    active_id: Signal<String>,
    children: Vec<View>,
) -> View {
    let on_change = Callback::new({
        let active_id = active_id.clone();
        move |id: String| {
            active_id.set(id);
        }
    });

    view! {
        <div class="tabs-container">
            {tab_bar(tabs_list, active_id.get(), on_change)}
            <div class="tabs-content">
                for child in children {
                    {child}
                }
            </div>
        </div>
    }
}
