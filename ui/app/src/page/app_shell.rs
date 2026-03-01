//! Main application shell with sidebar and router

use components::prelude::*;
use wasm_bindgen::prelude::*;
use crate::Route;
use crate::shared::layout::{sidebar, topbar};

// Feature imports
use crate::features::dashboard::dashboard_landing::dashboard_landing;
use crate::features::requisitions::requisition_list::requisition_list;
use crate::features::requisitions::requisition_form::requisition_form;
use crate::features::tenders::tender_list::tender_list;
use crate::features::tenders::tender_form::tender_form;
use crate::features::tenders::tender_publication::tender_publication;
use crate::features::tenders::tender_deviation::tender_deviation;
use crate::features::evaluation::evaluation_list::evaluation_list;
use crate::features::evaluation::evaluation_scoring::evaluation_scoring;
use crate::features::contracts::contract_list::contract_list;
use crate::features::contracts::contract_form::contract_form;
use crate::features::contracts::contract_milestones::contract_milestones;
use crate::features::purchase_orders::po_list::po_list;
use crate::features::purchase_orders::po_form::po_form;
use crate::features::goods_receipt::gr_list::gr_list;
use crate::features::suppliers::supplier_registry::supplier_registry;
use crate::features::suppliers::supplier_performance::supplier_performance;
use crate::features::suppliers::supplier_risk::supplier_risk;
use crate::features::supplier_portal::portal_dashboard::portal_dashboard;
use crate::features::catalogue::catalogue_list::catalogue_list;
use crate::features::catalogue::catalogue_admin::catalogue_admin;
use crate::features::analytics::analytics_dashboard::analytics_dashboard;
use crate::features::grc::grc_dashboard::grc_dashboard;
use crate::features::audit::audit_trail::audit_trail;
use crate::features::nbac::nbac_reviews::nbac_reviews;
use crate::features::reverse_auction::auction_list::auction_list;
use crate::features::reverse_auction::auction_live::auction_live;
use crate::features::documents::documents_library::documents_library;
use crate::features::ai_assistant::ai_chat::ai_chat_panel;
use crate::features::sourcing_plan::sourcing_list::sourcing_list;
use crate::features::sourcing_plan::sourcing_form::sourcing_form;
use crate::features::bbbee::bbbee_goals::bbbee_goals;
use crate::features::agsa::agsa_reviews::agsa_reviews;
use crate::features::mobile::mobile_app::mobile_app;

/// Get initial route from browser URL
fn get_initial_route() -> Route {
    let window = web_sys::window().expect("no window");
    let location = window.location();
    let pathname = location.pathname().unwrap_or_default();
    Route::from_path(&pathname)
}

/// Push route to browser history
fn push_history(route: &Route) {
    let window = web_sys::window().expect("no window");
    let history = window.history().expect("no history");
    let path = format!("/app{}", route.to_path());
    let _ = history.push_state_with_url(&JsValue::NULL, "", Some(&path));
}

/// Main application shell
#[component]
pub fn app_shell() -> View {
    // Initialize route from current URL
    let route = signal(get_initial_route());

    // Handle navigation: update signal AND push to history
    let handle_navigate = Callback::<Route>::new({
        let route = route.clone();
        move |new_route: Route| {
            push_history(&new_route);
            route.set(new_route);
        }
    });

    // Listen for popstate (back/forward buttons)
    effect({
        let route = route.clone();
        move || {
            let route = route.clone();
            let window = web_sys::window().expect("no window");
            let closure = Closure::<dyn Fn()>::new(move || {
                let window = web_sys::window().expect("no window");
                let location = window.location();
                let pathname = location.pathname().unwrap_or_default();
                let new_route = Route::from_path(&pathname);
                route.set(new_route);
            });
            window
                .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
                .expect("failed to add popstate listener");
            closure.forget(); // Leak closure to keep it alive
        }
    });

    view! {
        style {
            r#"
            .app-shell {
                display: flex;
                min-height: 100vh;
            }
            .app-main {
                flex: 1;
                margin-left: var(--sidebar-width);
                display: flex;
                flex-direction: column;
            }
            .app-content {
                flex: 1;
                padding: var(--space-6);
            }
            "#
        }

        <div class="app-shell" data-testid="app-shell">
            {sidebar(route.clone(), handle_navigate)}
            <div class="app-main">
                {topbar()}
                <main class="app-content page-enter">
                    {route_view(route.clone())}
                </main>
            </div>
        </div>
    }
}

/// Route to view mapping
fn route_view(route: Signal<Route>) -> View {
    match route.get() {
        Route::Dashboard => dashboard_landing(),

        // Requisitions
        Route::RequisitionsList => requisition_list(),
        Route::RequisitionsCreate => requisition_form(),
        Route::RequisitionsEdit(_id) => requisition_form(),

        // Tenders
        Route::TendersList => tender_list(),
        Route::TendersCreate => tender_form(None),
        Route::TendersEdit(id) => tender_form(Some(id)),
        Route::TendersPublication(id) => tender_publication(id),
        Route::TendersDeviation(_id) => tender_deviation(),

        // Evaluation
        Route::EvaluationList => evaluation_list(),
        Route::EvaluationScoring(_id) => evaluation_scoring(),

        // Contracts
        Route::ContractsList => contract_list(),
        Route::ContractsCreate => contract_form(None),
        Route::ContractsEdit(id) => contract_form(Some(id)),
        Route::ContractsMilestones(id) => contract_milestones(id),

        // Purchase Orders
        Route::PurchaseOrdersList => po_list(),
        Route::PurchaseOrdersCreate => po_form(None),
        Route::PurchaseOrdersEdit(id) => po_form(Some(id)),

        // Goods Receipt
        Route::GoodsReceiptList => gr_list(),

        // Suppliers
        Route::SuppliersRegistry => supplier_registry(),
        Route::SuppliersPerformance => supplier_performance(),
        Route::SuppliersRisk => supplier_risk(),

        // Supplier Portal
        Route::SupplierPortalDashboard => portal_dashboard(),

        // Catalogue
        Route::CatalogueList => catalogue_list(),
        Route::CatalogueAdmin => catalogue_admin(),

        // Analytics
        Route::AnalyticsDashboard => analytics_dashboard(),

        // GRC
        Route::GrcDashboard => grc_dashboard(),

        // Audit
        Route::AuditTrail => audit_trail(),

        // NBAC
        Route::NbacReviews => nbac_reviews(),

        // Reverse Auction
        Route::ReverseAuctionList => auction_list(),
        Route::ReverseAuctionLive(_id) => auction_live(),

        // Documents
        Route::DocumentsLibrary => documents_library(),

        // AI Assistant
        Route::AiAssistantChat => ai_chat_panel(),

        // Sourcing Plan
        Route::SourcingPlanList => sourcing_list(),
        Route::SourcingPlanCreate => sourcing_form(),
        Route::SourcingPlanEdit(_id) => sourcing_form(),

        // B-BBEE
        Route::BbbeeGoals => bbbee_goals(),

        // AGSA
        Route::AgsaReviews => agsa_reviews(),

        // Mobile
        Route::MobileSupplierApp => mobile_app(),
    }
}
