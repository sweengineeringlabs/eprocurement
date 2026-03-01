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

impl Route {
    /// Convert route to URL path
    pub fn to_path(&self) -> String {
        match self {
            Route::Dashboard => "/".to_string(),
            // Requisitions
            Route::RequisitionsList => "/requisitions".to_string(),
            Route::RequisitionsCreate => "/requisitions/new".to_string(),
            Route::RequisitionsEdit(id) => format!("/requisitions/{}/edit", id),
            // Tenders
            Route::TendersList => "/tenders".to_string(),
            Route::TendersCreate => "/tenders/new".to_string(),
            Route::TendersEdit(id) => format!("/tenders/{}/edit", id),
            Route::TendersPublication(id) => format!("/tenders/{}/publication", id),
            Route::TendersDeviation(id) => format!("/tenders/{}/deviation", id),
            // Evaluation
            Route::EvaluationList => "/evaluation".to_string(),
            Route::EvaluationScoring(id) => format!("/evaluation/{}/scoring", id),
            // Contracts
            Route::ContractsList => "/contracts".to_string(),
            Route::ContractsCreate => "/contracts/new".to_string(),
            Route::ContractsEdit(id) => format!("/contracts/{}/edit", id),
            Route::ContractsMilestones(id) => format!("/contracts/{}/milestones", id),
            // Purchase Orders
            Route::PurchaseOrdersList => "/purchase-orders".to_string(),
            Route::PurchaseOrdersCreate => "/purchase-orders/new".to_string(),
            Route::PurchaseOrdersEdit(id) => format!("/purchase-orders/{}/edit", id),
            // Goods Receipt
            Route::GoodsReceiptList => "/goods-receipt".to_string(),
            // Suppliers
            Route::SuppliersRegistry => "/suppliers".to_string(),
            Route::SuppliersPerformance => "/suppliers/performance".to_string(),
            Route::SuppliersRisk => "/suppliers/risk".to_string(),
            // Supplier Portal
            Route::SupplierPortalDashboard => "/supplier-portal".to_string(),
            // Catalogue
            Route::CatalogueList => "/catalogue".to_string(),
            Route::CatalogueAdmin => "/catalogue/admin".to_string(),
            // Analytics
            Route::AnalyticsDashboard => "/analytics".to_string(),
            // GRC
            Route::GrcDashboard => "/grc".to_string(),
            // Audit
            Route::AuditTrail => "/audit".to_string(),
            // NBAC
            Route::NbacReviews => "/nbac".to_string(),
            // Reverse Auction
            Route::ReverseAuctionList => "/auctions".to_string(),
            Route::ReverseAuctionLive(id) => format!("/auctions/{}/live", id),
            // Documents
            Route::DocumentsLibrary => "/documents".to_string(),
            // AI Assistant
            Route::AiAssistantChat => "/ai-assistant".to_string(),
            // Sourcing Plan
            Route::SourcingPlanList => "/sourcing".to_string(),
            Route::SourcingPlanCreate => "/sourcing/new".to_string(),
            Route::SourcingPlanEdit(id) => format!("/sourcing/{}/edit", id),
            // B-BBEE
            Route::BbbeeGoals => "/bbbee".to_string(),
            // AGSA
            Route::AgsaReviews => "/agsa".to_string(),
            // Mobile
            Route::MobileSupplierApp => "/mobile".to_string(),
        }
    }

    /// Parse URL path to route
    pub fn from_path(path: &str) -> Self {
        let path = path.trim_start_matches("/app").trim_start_matches('/');
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        match segments.as_slice() {
            [] | [""] => Route::Dashboard,
            // Requisitions
            ["requisitions"] => Route::RequisitionsList,
            ["requisitions", "new"] => Route::RequisitionsCreate,
            ["requisitions", id, "edit"] => Route::RequisitionsEdit(id.to_string()),
            // Tenders
            ["tenders"] => Route::TendersList,
            ["tenders", "new"] => Route::TendersCreate,
            ["tenders", id, "edit"] => Route::TendersEdit(id.to_string()),
            ["tenders", id, "publication"] => Route::TendersPublication(id.to_string()),
            ["tenders", id, "deviation"] => Route::TendersDeviation(id.to_string()),
            // Evaluation
            ["evaluation"] => Route::EvaluationList,
            ["evaluation", id, "scoring"] => Route::EvaluationScoring(id.to_string()),
            // Contracts
            ["contracts"] => Route::ContractsList,
            ["contracts", "new"] => Route::ContractsCreate,
            ["contracts", id, "edit"] => Route::ContractsEdit(id.to_string()),
            ["contracts", id, "milestones"] => Route::ContractsMilestones(id.to_string()),
            // Purchase Orders
            ["purchase-orders"] => Route::PurchaseOrdersList,
            ["purchase-orders", "new"] => Route::PurchaseOrdersCreate,
            ["purchase-orders", id, "edit"] => Route::PurchaseOrdersEdit(id.to_string()),
            // Goods Receipt
            ["goods-receipt"] => Route::GoodsReceiptList,
            // Suppliers
            ["suppliers"] => Route::SuppliersRegistry,
            ["suppliers", "performance"] => Route::SuppliersPerformance,
            ["suppliers", "risk"] => Route::SuppliersRisk,
            // Supplier Portal
            ["supplier-portal"] => Route::SupplierPortalDashboard,
            // Catalogue
            ["catalogue"] => Route::CatalogueList,
            ["catalogue", "admin"] => Route::CatalogueAdmin,
            // Analytics
            ["analytics"] => Route::AnalyticsDashboard,
            // GRC
            ["grc"] => Route::GrcDashboard,
            // Audit
            ["audit"] => Route::AuditTrail,
            // NBAC
            ["nbac"] => Route::NbacReviews,
            // Reverse Auction
            ["auctions"] => Route::ReverseAuctionList,
            ["auctions", id, "live"] => Route::ReverseAuctionLive(id.to_string()),
            // Documents
            ["documents"] => Route::DocumentsLibrary,
            // AI Assistant
            ["ai-assistant"] => Route::AiAssistantChat,
            // Sourcing Plan
            ["sourcing"] => Route::SourcingPlanList,
            ["sourcing", "new"] => Route::SourcingPlanCreate,
            ["sourcing", id, "edit"] => Route::SourcingPlanEdit(id.to_string()),
            // B-BBEE
            ["bbbee"] => Route::BbbeeGoals,
            // AGSA
            ["agsa"] => Route::AgsaReviews,
            // Mobile
            ["mobile"] => Route::MobileSupplierApp,
            // Default
            _ => Route::Dashboard,
        }
    }
}
