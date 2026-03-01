mod common;
mod auth_e2e;
mod navigation_e2e;
mod dashboard_e2e;
mod requisitions_e2e;
mod tenders_e2e;
mod evaluation_e2e;
mod contracts_e2e;
mod purchase_orders_e2e;
mod goods_receipt_e2e;
mod suppliers_e2e;
mod supplier_portal_e2e;
mod catalogue_e2e;
mod analytics_e2e;
mod grc_e2e;
mod audit_e2e;
mod nbac_e2e;
mod reverse_auction_e2e;
mod documents_e2e;
mod ai_assistant_e2e;
mod sourcing_plan_e2e;
mod bbbee_e2e;
mod agsa_e2e;
mod mobile_e2e;
mod visual_e2e;

use common::{build_config, make_test, make_auth_test};
use e2e_test::{BrowserTestRunner, BrowserTestSuite};

/// Check if a suite should run based on RSC_TEST_SUITE env var.
/// If not set, all suites run. If set, only matching suite(s) run.
/// Supports comma-separated values: RSC_TEST_SUITE=auth,dashboard
fn should_run_suite(name: &str) -> bool {
    match std::env::var("RSC_TEST_SUITE") {
        Ok(filter) => filter.split(',').any(|s| s.trim() == name),
        Err(_) => true,
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn run_eprocurement_e2e() {
    let mut runner = BrowserTestRunner::new(build_config());

    // ── auth ────────────────────────────────────────────────────────────
    if should_run_suite("auth") {
        let mut suite = BrowserTestSuite::new("auth");
        suite.add_test(make_test("login_form_renders",              auth_e2e::login_form_renders));
        suite.add_test(make_test("form_login_works",                auth_e2e::form_login_works));
        suite.add_test(make_test("localStorage_auth_bypasses_login", auth_e2e::localStorage_auth_bypasses_login));
        suite.add_test(make_test("app_shell_structure",             auth_e2e::app_shell_structure));
        suite.add_test(make_test("logout_clears_session",           auth_e2e::logout_clears_session));
        runner.add_suite(suite);
    }

    // ── navigation ──────────────────────────────────────────────────────
    if should_run_suite("navigation") {
        let mut suite = BrowserTestSuite::new("navigation");
        suite.add_test(make_test("app_shell_renders",           navigation_e2e::app_shell_renders));
        suite.add_test(make_test("sidebar_visible",             navigation_e2e::sidebar_visible));
        suite.add_test(make_test("nav_links_present",           navigation_e2e::nav_links_present));
        suite.add_test(make_test("route_dashboard",             navigation_e2e::route_dashboard));
        suite.add_test(make_test("route_requisitions",          navigation_e2e::route_requisitions));
        suite.add_test(make_test("route_tenders",               navigation_e2e::route_tenders));
        suite.add_test(make_test("route_evaluation",            navigation_e2e::route_evaluation));
        suite.add_test(make_test("route_contracts",             navigation_e2e::route_contracts));
        suite.add_test(make_test("route_purchase_orders",       navigation_e2e::route_purchase_orders));
        suite.add_test(make_test("route_goods_receipt",         navigation_e2e::route_goods_receipt));
        suite.add_test(make_test("route_suppliers",             navigation_e2e::route_suppliers));
        suite.add_test(make_test("route_supplier_portal",       navigation_e2e::route_supplier_portal));
        suite.add_test(make_test("route_catalogue",             navigation_e2e::route_catalogue));
        suite.add_test(make_test("route_analytics",             navigation_e2e::route_analytics));
        suite.add_test(make_test("route_grc",                   navigation_e2e::route_grc));
        suite.add_test(make_test("route_audit",                 navigation_e2e::route_audit));
        runner.add_suite(suite);
    }

    // ── dashboard ───────────────────────────────────────────────────────
    if should_run_suite("dashboard") {
        let mut suite = BrowserTestSuite::new("dashboard");
        suite.add_test(make_auth_test("landing_renders",           "/dashboard", dashboard_e2e::landing_renders));
        suite.add_test(make_auth_test("page_header_renders",       "/dashboard", dashboard_e2e::page_header_renders));
        suite.add_test(make_auth_test("kpi_cards_render",          "/dashboard", dashboard_e2e::kpi_cards_render));
        suite.add_test(make_auth_test("panels_render",             "/dashboard", dashboard_e2e::panels_render));
        suite.add_test(make_auth_test("requisitions_table_renders", "/dashboard", dashboard_e2e::requisitions_table_renders));
        suite.add_test(make_auth_test("recent_activity_visible",   "/dashboard", dashboard_e2e::recent_activity_visible));
        suite.add_test(make_auth_test("charts_render",             "/dashboard", dashboard_e2e::charts_render));
        suite.add_test(make_auth_test("action_buttons_visible",    "/dashboard", dashboard_e2e::action_buttons_visible));
        runner.add_suite(suite);
    }

    // ── requisitions ────────────────────────────────────────────────────
    if should_run_suite("requisitions") {
        let mut suite = BrowserTestSuite::new("requisitions");
        suite.add_test(make_auth_test("landing_renders",        "/requisitions", requisitions_e2e::landing_renders));
        suite.add_test(make_auth_test("list_renders",           "/requisitions", requisitions_e2e::list_renders));
        suite.add_test(make_auth_test("create_btn_visible",     "/requisitions", requisitions_e2e::create_btn_visible));
        suite.add_test(make_auth_test("filter_bar_renders",     "/requisitions", requisitions_e2e::filter_bar_renders));
        runner.add_suite(suite);
    }

    // ── tenders ─────────────────────────────────────────────────────────
    if should_run_suite("tenders") {
        let mut suite = BrowserTestSuite::new("tenders");
        // Basic rendering tests
        suite.add_test(make_auth_test("landing_renders",          "/tenders", tenders_e2e::landing_renders));
        suite.add_test(make_auth_test("list_renders",             "/tenders", tenders_e2e::list_renders));
        suite.add_test(make_auth_test("create_btn_visible",       "/tenders", tenders_e2e::create_btn_visible));
        suite.add_test(make_auth_test("status_filters_render",    "/tenders", tenders_e2e::status_filters_render));
        // Navigation & List tests
        suite.add_test(make_auth_test("navigate_to_tenders",      "/tenders", tenders_e2e::navigate_to_tenders));
        suite.add_test(make_auth_test("list_shows_tender_rows",   "/tenders", tenders_e2e::list_shows_tender_rows));
        suite.add_test(make_auth_test("filter_by_status_works",   "/tenders", tenders_e2e::filter_by_status_works));
        suite.add_test(make_auth_test("search_filters_list",      "/tenders", tenders_e2e::search_filters_list));
        suite.add_test(make_auth_test("clear_filters_resets_list", "/tenders", tenders_e2e::clear_filters_resets_list));
        // Create Flow tests
        suite.add_test(make_auth_test("click_create_opens_form",  "/tenders", tenders_e2e::click_create_opens_form));
        suite.add_test(make_auth_test("form_step_navigation_works", "/tenders", tenders_e2e::form_step_navigation_works));
        suite.add_test(make_auth_test("form_validation_shows_errors", "/tenders", tenders_e2e::form_validation_shows_errors));
        suite.add_test(make_auth_test("save_draft_succeeds",      "/tenders", tenders_e2e::save_draft_succeeds));
        // View/Detail tests
        suite.add_test(make_auth_test("click_row_shows_detail",   "/tenders", tenders_e2e::click_row_shows_detail));
        // Filter element tests
        suite.add_test(make_auth_test("type_filter_renders",      "/tenders", tenders_e2e::type_filter_renders));
        suite.add_test(make_auth_test("apply_button_renders",     "/tenders", tenders_e2e::apply_button_renders));
        suite.add_test(make_auth_test("clear_button_renders",     "/tenders", tenders_e2e::clear_button_renders));
        runner.add_suite(suite);
    }

    // ── evaluation ──────────────────────────────────────────────────────
    if should_run_suite("evaluation") {
        let mut suite = BrowserTestSuite::new("evaluation");
        suite.add_test(make_auth_test("landing_renders",        "/evaluation", evaluation_e2e::landing_renders));
        suite.add_test(make_auth_test("list_renders",           "/evaluation", evaluation_e2e::list_renders));
        suite.add_test(make_auth_test("scoring_panel_renders",  "/evaluation", evaluation_e2e::scoring_panel_renders));
        runner.add_suite(suite);
    }

    // ── contracts ───────────────────────────────────────────────────────
    if should_run_suite("contracts") {
        let mut suite = BrowserTestSuite::new("contracts");
        suite.add_test(make_auth_test("landing_renders",        "/contracts", contracts_e2e::landing_renders));
        suite.add_test(make_auth_test("list_renders",           "/contracts", contracts_e2e::list_renders));
        suite.add_test(make_auth_test("create_btn_visible",     "/contracts", contracts_e2e::create_btn_visible));
        suite.add_test(make_auth_test("milestones_accessible",  "/contracts", contracts_e2e::milestones_accessible));
        runner.add_suite(suite);
    }

    // ── purchase_orders ─────────────────────────────────────────────────
    if should_run_suite("purchase_orders") {
        let mut suite = BrowserTestSuite::new("purchase_orders");
        suite.add_test(make_auth_test("landing_renders",        "/purchase-orders", purchase_orders_e2e::landing_renders));
        suite.add_test(make_auth_test("list_renders",           "/purchase-orders", purchase_orders_e2e::list_renders));
        suite.add_test(make_auth_test("create_btn_visible",     "/purchase-orders", purchase_orders_e2e::create_btn_visible));
        runner.add_suite(suite);
    }

    // ── goods_receipt ───────────────────────────────────────────────────
    if should_run_suite("goods_receipt") {
        let mut suite = BrowserTestSuite::new("goods_receipt");
        suite.add_test(make_auth_test("landing_renders",        "/goods-receipt", goods_receipt_e2e::landing_renders));
        suite.add_test(make_auth_test("list_renders",           "/goods-receipt", goods_receipt_e2e::list_renders));
        runner.add_suite(suite);
    }

    // ── suppliers ───────────────────────────────────────────────────────
    if should_run_suite("suppliers") {
        let mut suite = BrowserTestSuite::new("suppliers");
        suite.add_test(make_auth_test("registry_renders",       "/suppliers/registry", suppliers_e2e::registry_renders));
        suite.add_test(make_auth_test("performance_renders",    "/suppliers/performance", suppliers_e2e::performance_renders));
        suite.add_test(make_auth_test("risk_renders",           "/suppliers/risk", suppliers_e2e::risk_renders));
        runner.add_suite(suite);
    }

    // ── supplier_portal ─────────────────────────────────────────────────
    if should_run_suite("supplier_portal") {
        let mut suite = BrowserTestSuite::new("supplier_portal");
        suite.add_test(make_auth_test("dashboard_renders",      "/supplier-portal", supplier_portal_e2e::dashboard_renders));
        suite.add_test(make_auth_test("opportunities_visible",  "/supplier-portal", supplier_portal_e2e::opportunities_visible));
        runner.add_suite(suite);
    }

    // ── catalogue ───────────────────────────────────────────────────────
    if should_run_suite("catalogue") {
        let mut suite = BrowserTestSuite::new("catalogue");
        suite.add_test(make_auth_test("landing_renders",        "/catalogue", catalogue_e2e::landing_renders));
        suite.add_test(make_auth_test("search_works",           "/catalogue", catalogue_e2e::search_works));
        runner.add_suite(suite);
    }

    // ── analytics ───────────────────────────────────────────────────────
    if should_run_suite("analytics") {
        let mut suite = BrowserTestSuite::new("analytics");
        suite.add_test(make_auth_test("dashboard_renders",      "/analytics", analytics_e2e::dashboard_renders));
        suite.add_test(make_auth_test("charts_visible",         "/analytics", analytics_e2e::charts_visible));
        runner.add_suite(suite);
    }

    // ── grc ─────────────────────────────────────────────────────────────
    if should_run_suite("grc") {
        let mut suite = BrowserTestSuite::new("grc");
        suite.add_test(make_auth_test("dashboard_renders",      "/grc", grc_e2e::dashboard_renders));
        suite.add_test(make_auth_test("compliance_panel",       "/grc", grc_e2e::compliance_panel_renders));
        runner.add_suite(suite);
    }

    // ── audit ───────────────────────────────────────────────────────────
    if should_run_suite("audit") {
        let mut suite = BrowserTestSuite::new("audit");
        suite.add_test(make_auth_test("trail_renders",          "/audit", audit_e2e::trail_renders));
        suite.add_test(make_auth_test("filters_work",           "/audit", audit_e2e::filters_work));
        runner.add_suite(suite);
    }

    // ── nbac ────────────────────────────────────────────────────────────
    if should_run_suite("nbac") {
        let mut suite = BrowserTestSuite::new("nbac");
        suite.add_test(make_auth_test("reviews_render",         "/nbac", nbac_e2e::reviews_render));
        suite.add_test(make_auth_test("approval_workflow",      "/nbac", nbac_e2e::approval_workflow_renders));
        runner.add_suite(suite);
    }

    // ── reverse_auction ─────────────────────────────────────────────────
    if should_run_suite("reverse_auction") {
        let mut suite = BrowserTestSuite::new("reverse_auction");
        suite.add_test(make_auth_test("list_renders",           "/reverse-auction", reverse_auction_e2e::list_renders));
        suite.add_test(make_auth_test("live_panel_renders",     "/reverse-auction", reverse_auction_e2e::live_panel_renders));
        runner.add_suite(suite);
    }

    // ── documents ───────────────────────────────────────────────────────
    if should_run_suite("documents") {
        let mut suite = BrowserTestSuite::new("documents");
        suite.add_test(make_auth_test("library_renders",        "/documents", documents_e2e::library_renders));
        suite.add_test(make_auth_test("upload_visible",         "/documents", documents_e2e::upload_visible));
        runner.add_suite(suite);
    }

    // ── ai_assistant ────────────────────────────────────────────────────
    if should_run_suite("ai_assistant") {
        let mut suite = BrowserTestSuite::new("ai_assistant");
        suite.add_test(make_auth_test("chat_renders",           "/ai-assistant", ai_assistant_e2e::chat_renders));
        suite.add_test(make_auth_test("input_accepts_text",     "/ai-assistant", ai_assistant_e2e::input_accepts_text));
        runner.add_suite(suite);
    }

    // ── sourcing_plan ───────────────────────────────────────────────────
    if should_run_suite("sourcing_plan") {
        let mut suite = BrowserTestSuite::new("sourcing_plan");
        suite.add_test(make_auth_test("list_renders",           "/sourcing-plan", sourcing_plan_e2e::list_renders));
        suite.add_test(make_auth_test("create_btn_visible",     "/sourcing-plan", sourcing_plan_e2e::create_btn_visible));
        runner.add_suite(suite);
    }

    // ── bbbee ───────────────────────────────────────────────────────────
    if should_run_suite("bbbee") {
        let mut suite = BrowserTestSuite::new("bbbee");
        suite.add_test(make_auth_test("goals_render",           "/bbbee", bbbee_e2e::goals_render));
        suite.add_test(make_auth_test("targets_visible",        "/bbbee", bbbee_e2e::targets_visible));
        runner.add_suite(suite);
    }

    // ── agsa ────────────────────────────────────────────────────────────
    if should_run_suite("agsa") {
        let mut suite = BrowserTestSuite::new("agsa");
        suite.add_test(make_auth_test("reviews_render",         "/agsa", agsa_e2e::reviews_render));
        suite.add_test(make_auth_test("findings_visible",       "/agsa", agsa_e2e::findings_visible));
        runner.add_suite(suite);
    }

    // ── mobile ──────────────────────────────────────────────────────────
    if should_run_suite("mobile") {
        let mut suite = BrowserTestSuite::new("mobile");
        suite.add_test(make_auth_test("app_renders",            "/mobile", mobile_e2e::app_renders));
        runner.add_suite(suite);
    }

    // ── visual ──────────────────────────────────────────────────────────
    // 26 comprehensive visual tests for CSS styles, layout, and design tokens
    if should_run_suite("visual") {
        let mut suite = BrowserTestSuite::new("visual");

        // Layout tests (3)
        suite.add_test(make_auth_test("layout/sidebar",              "/dashboard", visual_e2e::sidebar_layout));
        suite.add_test(make_auth_test("layout/topbar",               "/dashboard", visual_e2e::topbar_layout));
        suite.add_test(make_auth_test("layout/main_content",         "/dashboard", visual_e2e::main_content_alignment));

        // Button tests (4)
        suite.add_test(make_auth_test("buttons/primary",             "/dashboard", visual_e2e::button_primary_styles));
        suite.add_test(make_auth_test("buttons/secondary",           "/dashboard", visual_e2e::button_secondary_styles));
        suite.add_test(make_auth_test("buttons/sizes",               "/dashboard", visual_e2e::button_sizes));
        suite.add_test(make_auth_test("buttons/disabled",            "/dashboard", visual_e2e::button_disabled_state));

        // Panel tests (2)
        suite.add_test(make_auth_test("panels/base",                 "/dashboard", visual_e2e::panel_styles));
        suite.add_test(make_auth_test("panels/header",               "/dashboard", visual_e2e::panel_header_styles));

        // Form tests (2)
        suite.add_test(make_auth_test("forms/input",                 "/requisitions", visual_e2e::input_styles));
        suite.add_test(make_auth_test("forms/label",                 "/requisitions", visual_e2e::label_styles));

        // Color tests (1)
        suite.add_test(make_auth_test("colors/tokens",               "/dashboard", visual_e2e::color_tokens));

        // Typography tests (1)
        suite.add_test(make_auth_test("typography/styles",           "/dashboard", visual_e2e::typography_styles));

        // Grid/Flex tests (2)
        suite.add_test(make_auth_test("grid/utilities",              "/dashboard", visual_e2e::grid_utilities));
        suite.add_test(make_auth_test("flex/utilities",              "/dashboard", visual_e2e::flex_utilities));

        // Responsive tests (1)
        suite.add_test(make_auth_test("responsive/mobile",           "/dashboard", visual_e2e::responsive_mobile));

        // Screenshot tests (3)
        suite.add_test(make_auth_test("screenshots/dashboard",       "/dashboard", visual_e2e::screenshot_dashboard));
        suite.add_test(make_auth_test("screenshots/sidebar",         "/dashboard", visual_e2e::screenshot_sidebar));
        suite.add_test(make_auth_test("screenshots/buttons",         "/dashboard", visual_e2e::screenshot_buttons));

        // Visibility tests (2)
        suite.add_test(make_auth_test("visibility/critical",         "/dashboard", visual_e2e::critical_elements_visible));
        suite.add_test(make_auth_test("visibility/viewport",         "/dashboard", visual_e2e::elements_within_viewport));

        // Table tests (1)
        suite.add_test(make_auth_test("tables/styles",               "/tenders", visual_e2e::table_styles));

        // Schema validation tests (4)
        suite.add_test(make_auth_test("schema/button",               "/dashboard", visual_e2e::schema_button_component));
        suite.add_test(make_auth_test("schema/panel",                "/dashboard", visual_e2e::schema_panel_component));
        suite.add_test(make_auth_test("schema/form_group",           "/requisitions", visual_e2e::schema_form_group_component));
        suite.add_test(make_auth_test("schema/spacing",              "/dashboard", visual_e2e::schema_spacing_tokens));

        runner.add_suite(suite);
    }

    let (_, summary) = runner.run().await;
    assert_eq!(
        summary.failed + summary.timed_out,
        0,
        "{} test(s) failed or timed out",
        summary.failed + summary.timed_out,
    );
}
