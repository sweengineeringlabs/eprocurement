//! Sidebar navigation component

use components::prelude::*;
use crate::Route;

/// Navigation section with title and items
struct NavSection {
    title: &'static str,
    items: Vec<NavItem>,
}

/// Single navigation item
struct NavItem {
    label: &'static str,
    route: Route,
    icon: &'static str,
    badge: Option<u32>,
    sub_items: Vec<NavItem>,
}

/// Sidebar component with navigation
#[component]
pub fn sidebar(current_route: Signal<Route>, on_navigate: Callback<Route>) -> View {
    let sections = nav_sections();

    view! {
        style {
            r#"
            .sidebar {
                position: fixed;
                top: 0;
                left: 0;
                width: var(--sidebar-width);
                height: 100vh;
                background: var(--navy);
                color: #fff;
                display: flex;
                flex-direction: column;
                z-index: 100;
            }
            .sidebar-header {
                padding: 20px;
                border-bottom: 1px solid #ffffff1A;
            }
            .logo {
                display: flex;
                align-items: center;
                gap: 12px;
                text-decoration: none;
                color: #fff;
            }
            .logo-icon {
                width: 40px;
                height: 40px;
                background: var(--accent);
                border-radius: var(--radius);
                display: flex;
                align-items: center;
                justify-content: center;
                font-weight: 700;
                font-size: 16px;
            }
            .logo-text {
                font-family: 'Playfair Display', serif;
                font-size: 18px;
                font-weight: 600;
            }
            .logo-sub {
                font-size: 11px;
                opacity: 0.7;
                margin-top: 2px;
            }
            .sidebar-nav {
                flex: 1;
                overflow-y: auto;
                padding: 16px 0;
            }
            .nav-section {
                margin-bottom: 8px;
            }
            .nav-section-title {
                padding: 8px 20px;
                font-size: 10px;
                text-transform: uppercase;
                letter-spacing: 1px;
                opacity: 0.5;
                font-weight: 600;
            }
            .nav-item {
                display: flex;
                align-items: center;
                gap: 12px;
                padding: 10px 20px;
                color: #ffffffB3;
                text-decoration: none;
                cursor: pointer;
                transition: all 0.15s;
                border-left: 3px solid transparent;
                font-size: 13px;
            }
            .nav-item:hover {
                background: #ffffff0D;
                color: #fff;
            }
            .nav-item.active {
                background: #ffffff1A;
                color: #fff;
                border-left-color: var(--accent);
            }
            .nav-item svg {
                width: 18px;
                height: 18px;
                opacity: 0.8;
            }
            .nav-badge {
                margin-left: auto;
                background: var(--accent);
                color: var(--navy);
                font-size: 10px;
                font-weight: 600;
                padding: 2px 6px;
                border-radius: 10px;
            }
            .nav-sub {
                padding-left: 50px;
                font-size: 12px;
                padding-top: 6px;
                padding-bottom: 6px;
            }
            "#
        }

        <aside class="sidebar" data-testid="sidebar">
            <div class="sidebar-header">
                <a href="#" class="logo">
                    <div class="logo-icon">"eP"</div>
                    <div>
                        <div class="logo-text">"eProcurement"</div>
                        <div class="logo-sub">"SARS"</div>
                    </div>
                </a>
            </div>
            <nav class="sidebar-nav" data-testid="nav-links">
                for section in sections.iter() {
                    {nav_section_view(section, current_route.clone(), on_navigate.clone())}
                }
            </nav>
        </aside>
    }
}

fn nav_section_view(
    section: &NavSection,
    current_route: Signal<Route>,
    on_navigate: Callback<Route>,
) -> View {
    view! {
        <div class="nav-section">
            <div class="nav-section-title">{section.title}</div>
            for item in section.items.iter() {
                {nav_item_view(item, current_route.clone(), on_navigate.clone())}
            }
        </div>
    }
}

fn nav_item_view(
    item: &NavItem,
    current_route: Signal<Route>,
    on_navigate: Callback<Route>,
) -> View {
    let is_active = current_route.get() == item.route;
    let class = if is_active { "nav-item active" } else { "nav-item" };
    let route = item.route.clone();
    let testid = format!("nav-{}", item.label.to_lowercase().replace(' ', "-"));

    // Use simple Fn() closure - no Event param needed since <a> has no href
    let handle_click = {
        let on_navigate = on_navigate.clone();
        let route = route.clone();
        move || {
            on_navigate.call(route.clone());
        }
    };

    view! {
        <a class={class} data-testid={testid} on:click={handle_click}>
            <span inner_html={item.icon}></span>
            <span>{item.label}</span>
            if let Some(count) = item.badge {
                <span class="nav-badge">{count.to_string()}</span>
            }
        </a>
        for sub in item.sub_items.iter() {
            {nav_sub_item_view(sub, current_route.clone(), on_navigate.clone())}
        }
    }
}

fn nav_sub_item_view(
    item: &NavItem,
    current_route: Signal<Route>,
    on_navigate: Callback<Route>,
) -> View {
    let is_active = current_route.get() == item.route;
    let class = if is_active { "nav-item nav-sub active" } else { "nav-item nav-sub" };
    let route = item.route.clone();

    // Use simple Fn() closure
    let handle_click = {
        let on_navigate = on_navigate.clone();
        move || {
            on_navigate.call(route.clone());
        }
    };

    view! {
        <a class={class} on:click={handle_click}>
            <span>{item.label}</span>
        </a>
    }
}

/// Returns all navigation sections
fn nav_sections() -> Vec<NavSection> {
    vec![
        NavSection {
            title: "Overview",
            items: vec![
                NavItem {
                    label: "Dashboard",
                    route: Route::Dashboard,
                    icon: icon_dashboard(),
                    badge: None,
                    sub_items: vec![],
                },
            ],
        },
        NavSection {
            title: "Procurement",
            items: vec![
                NavItem {
                    label: "Requisitions",
                    route: Route::RequisitionsList,
                    icon: icon_document(),
                    badge: Some(127),
                    sub_items: vec![],
                },
                NavItem {
                    label: "Tenders",
                    route: Route::TendersList,
                    icon: icon_briefcase(),
                    badge: Some(23),
                    sub_items: vec![],
                },
                NavItem {
                    label: "Evaluation",
                    route: Route::EvaluationList,
                    icon: icon_clipboard(),
                    badge: None,
                    sub_items: vec![],
                },
                NavItem {
                    label: "Contracts",
                    route: Route::ContractsList,
                    icon: icon_file_text(),
                    badge: None,
                    sub_items: vec![],
                },
                NavItem {
                    label: "Purchase Orders",
                    route: Route::PurchaseOrdersList,
                    icon: icon_shopping_cart(),
                    badge: None,
                    sub_items: vec![],
                },
                NavItem {
                    label: "Goods Receipt",
                    route: Route::GoodsReceiptList,
                    icon: icon_package(),
                    badge: None,
                    sub_items: vec![],
                },
            ],
        },
        NavSection {
            title: "Suppliers",
            items: vec![
                NavItem {
                    label: "Registry",
                    route: Route::SuppliersRegistry,
                    icon: icon_users(),
                    badge: None,
                    sub_items: vec![],
                },
                NavItem {
                    label: "Performance",
                    route: Route::SuppliersPerformance,
                    icon: icon_bar_chart(),
                    badge: None,
                    sub_items: vec![],
                },
                NavItem {
                    label: "Risk",
                    route: Route::SuppliersRisk,
                    icon: icon_alert_triangle(),
                    badge: None,
                    sub_items: vec![],
                },
                NavItem {
                    label: "Supplier Portal",
                    route: Route::SupplierPortalDashboard,
                    icon: icon_external_link(),
                    badge: None,
                    sub_items: vec![],
                },
            ],
        },
        NavSection {
            title: "Catalogue",
            items: vec![
                NavItem {
                    label: "Browse",
                    route: Route::CatalogueList,
                    icon: icon_grid(),
                    badge: None,
                    sub_items: vec![],
                },
                NavItem {
                    label: "Admin",
                    route: Route::CatalogueAdmin,
                    icon: icon_settings(),
                    badge: None,
                    sub_items: vec![],
                },
            ],
        },
        NavSection {
            title: "Special Sourcing",
            items: vec![
                NavItem {
                    label: "Reverse Auction",
                    route: Route::ReverseAuctionList,
                    icon: icon_trending_down(),
                    badge: None,
                    sub_items: vec![],
                },
                NavItem {
                    label: "Sourcing Plans",
                    route: Route::SourcingPlanList,
                    icon: icon_calendar(),
                    badge: None,
                    sub_items: vec![],
                },
            ],
        },
        NavSection {
            title: "Compliance",
            items: vec![
                NavItem {
                    label: "B-BBEE Goals",
                    route: Route::BbbeeGoals,
                    icon: icon_target(),
                    badge: None,
                    sub_items: vec![],
                },
                NavItem {
                    label: "GRC Dashboard",
                    route: Route::GrcDashboard,
                    icon: icon_shield(),
                    badge: None,
                    sub_items: vec![],
                },
                NavItem {
                    label: "Audit Trail",
                    route: Route::AuditTrail,
                    icon: icon_search(),
                    badge: None,
                    sub_items: vec![],
                },
                NavItem {
                    label: "NBAC Reviews",
                    route: Route::NbacReviews,
                    icon: icon_check_square(),
                    badge: None,
                    sub_items: vec![],
                },
                NavItem {
                    label: "AGSA Reviews",
                    route: Route::AgsaReviews,
                    icon: icon_file_check(),
                    badge: None,
                    sub_items: vec![],
                },
            ],
        },
        NavSection {
            title: "Tools",
            items: vec![
                NavItem {
                    label: "Analytics",
                    route: Route::AnalyticsDashboard,
                    icon: icon_pie_chart(),
                    badge: None,
                    sub_items: vec![],
                },
                NavItem {
                    label: "Documents",
                    route: Route::DocumentsLibrary,
                    icon: icon_folder(),
                    badge: None,
                    sub_items: vec![],
                },
                NavItem {
                    label: "AI Assistant",
                    route: Route::AiAssistantChat,
                    icon: icon_message_circle(),
                    badge: None,
                    sub_items: vec![],
                },
            ],
        },
    ]
}

// SVG Icons
fn icon_dashboard() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/><rect x="14" y="14" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/></svg>"#
}

fn icon_document() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>"#
}

fn icon_briefcase() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="2" y="7" width="20" height="14" rx="2" ry="2"/><path d="M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16"/></svg>"#
}

fn icon_clipboard() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/><rect x="8" y="2" width="8" height="4" rx="1" ry="1"/></svg>"#
}

fn icon_file_text() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><path d="M14 2v6h6"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/></svg>"#
}

fn icon_shopping_cart() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="9" cy="21" r="1"/><circle cx="20" cy="21" r="1"/><path d="M1 1h4l2.68 13.39a2 2 0 0 0 2 1.61h9.72a2 2 0 0 0 2-1.61L23 6H6"/></svg>"#
}

fn icon_package() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12.89 1.45l8 4A2 2 0 0 1 22 7.24v9.53a2 2 0 0 1-1.11 1.79l-8 4a2 2 0 0 1-1.79 0l-8-4a2 2 0 0 1-1.1-1.8V7.24a2 2 0 0 1 1.11-1.79l8-4a2 2 0 0 1 1.78 0z"/><polyline points="2.32 6.16 12 11 21.68 6.16"/><line x1="12" y1="22.76" x2="12" y2="11"/></svg>"#
}

fn icon_users() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>"#
}

fn icon_bar_chart() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" y1="20" x2="12" y2="10"/><line x1="18" y1="20" x2="18" y2="4"/><line x1="6" y1="20" x2="6" y2="16"/></svg>"#
}

fn icon_alert_triangle() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>"#
}

fn icon_external_link() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/></svg>"#
}

fn icon_grid() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/><rect x="14" y="14" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/></svg>"#
}

fn icon_settings() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>"#
}

fn icon_trending_down() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="23 18 13.5 8.5 8.5 13.5 1 6"/><polyline points="17 18 23 18 23 12"/></svg>"#
}

fn icon_calendar() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="4" width="18" height="18" rx="2" ry="2"/><line x1="16" y1="2" x2="16" y2="6"/><line x1="8" y1="2" x2="8" y2="6"/><line x1="3" y1="10" x2="21" y2="10"/></svg>"#
}

fn icon_target() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><circle cx="12" cy="12" r="6"/><circle cx="12" cy="12" r="2"/></svg>"#
}

fn icon_shield() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>"#
}

fn icon_search() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>"#
}

fn icon_check_square() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="9 11 12 14 22 4"/><path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"/></svg>"#
}

fn icon_file_check() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><path d="M14 2v6h6"/><path d="M9 15l2 2 4-4"/></svg>"#
}

fn icon_pie_chart() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21.21 15.89A10 10 0 1 1 8 2.83"/><path d="M22 12A10 10 0 0 0 12 2v10z"/></svg>"#
}

fn icon_folder() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>"#
}

fn icon_message_circle() -> &'static str {
    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"/></svg>"#
}
