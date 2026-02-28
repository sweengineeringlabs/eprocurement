//! eProcurement Frontend - SARS RFP 33/2025
//! Main WASM entry point with context providers

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub mod features;
pub mod page;
pub mod shared;
pub mod util;

use components::prelude::*;
use page::app_shell::app_shell;
use shared::layout::base_styles;
use util::auth::{auth_provider, AuthState};

// Feature stores
use features::dashboard::store::DashboardStore;
use features::requisitions::store::RequisitionsStore;
use features::tenders::store::TendersStore;
use features::evaluation::store::EvaluationStore;
use features::contracts::store::ContractsStore;
use features::purchase_orders::store::PurchaseOrdersStore;
use features::goods_receipt::store::GoodsReceiptStore;
use features::suppliers::store::SuppliersStore;
use features::supplier_portal::store::SupplierPortalStore;
use features::catalogue::store::CatalogueStore;
use features::analytics::store::AnalyticsStore;
use features::grc::store::GrcStore;
use features::audit::store::AuditStore;
use features::nbac::store::NbacStore;
use features::reverse_auction::store::ReverseAuctionStore;
use features::documents::store::DocumentsStore;
use features::ai_assistant::store::AiAssistantStore;
use features::sourcing_plan::store::SourcingPlanStore;
use features::bbbee::store::BbbeeStore;
use features::agsa::store::AgsaStore;
use features::mobile::store::MobileStore;

/// WASM entry point
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    inject_style(base_styles());
    let _ = crate::util::api::api_base();

    let window = web_sys::window().expect("no global window");
    let document = window.document().expect("no document");
    let body = document.body().expect("no body element");

    let root_view = app();
    root_view.mount(&body.into());
}

/// Root application component with context providers
#[component]
pub fn app() -> View {
    let auth_state = signal(AuthState::default());
    provide_context(auth_state.clone());

    // Provide all feature stores
    provide_context(DashboardStore::new());
    provide_context(RequisitionsStore::new());
    provide_context(TendersStore::new());
    provide_context(EvaluationStore::new());
    provide_context(ContractsStore::new());
    provide_context(PurchaseOrdersStore::new());
    provide_context(GoodsReceiptStore::new());
    provide_context(SuppliersStore::new());
    provide_context(SupplierPortalStore::new());
    provide_context(CatalogueStore::new());
    provide_context(AnalyticsStore::new());
    provide_context(GrcStore::new());
    provide_context(AuditStore::new());
    provide_context(NbacStore::new());
    provide_context(ReverseAuctionStore::new());
    provide_context(DocumentsStore::new());
    provide_context(AiAssistantStore::new());
    provide_context(SourcingPlanStore::new());
    provide_context(BbbeeStore::new());
    provide_context(AgsaStore::new());
    provide_context(MobileStore::new());

    auth_provider(auth_state, vec![app_shell()])
}

/// Application routes
#[derive(Clone, PartialEq, Debug)]
pub enum Route {
    Dashboard,
    // Requisitions
    RequisitionsList,
    RequisitionsCreate,
    RequisitionsEdit(String),
    // Tenders
    TendersList,
    TendersCreate,
    TendersEdit(String),
    TendersPublication(String),
    TendersDeviation(String),
    // Evaluation
    EvaluationList,
    EvaluationScoring(String),
    // Contracts
    ContractsList,
    ContractsCreate,
    ContractsEdit(String),
    ContractsMilestones(String),
    // Purchase Orders
    PurchaseOrdersList,
    PurchaseOrdersCreate,
    PurchaseOrdersEdit(String),
    // Goods Receipt
    GoodsReceiptList,
    // Suppliers
    SuppliersRegistry,
    SuppliersPerformance,
    SuppliersRisk,
    // Supplier Portal
    SupplierPortalDashboard,
    // Catalogue
    CatalogueList,
    CatalogueAdmin,
    // Analytics
    AnalyticsDashboard,
    // GRC
    GrcDashboard,
    // Audit
    AuditTrail,
    // NBAC
    NbacReviews,
    // Reverse Auction
    ReverseAuctionList,
    ReverseAuctionLive(String),
    // Documents
    DocumentsLibrary,
    // AI Assistant
    AiAssistantChat,
    // Sourcing Plan
    SourcingPlanList,
    SourcingPlanCreate,
    SourcingPlanEdit(String),
    // B-BBEE
    BbbeeGoals,
    // AGSA
    AgsaReviews,
    // Mobile
    MobileSupplierApp,
}

impl Default for Route {
    fn default() -> Self {
        Route::Dashboard
    }
}
