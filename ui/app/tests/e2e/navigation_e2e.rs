use e2e_test::{BrowserTestContext, assertions::PageAssertions};
use crate::common::go_to;

pub async fn app_shell_renders(ctx: BrowserTestContext) -> Result<(), String> {
    go_to(&ctx, "/").await?;
    ctx.assert_element_exists("[data-testid='app-shell']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn sidebar_visible(ctx: BrowserTestContext) -> Result<(), String> {
    go_to(&ctx, "/").await?;
    ctx.wait_for("[data-testid='sidebar']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='sidebar']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn nav_links_present(ctx: BrowserTestContext) -> Result<(), String> {
    go_to(&ctx, "/").await?;
    ctx.assert_element_exists("[data-testid='nav-links']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn route_dashboard(ctx: BrowserTestContext) -> Result<(), String> {
    go_to(&ctx, "/dashboard").await?;
    ctx.wait_for("[data-testid='dashboard-landing']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='dashboard-landing']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn route_requisitions(ctx: BrowserTestContext) -> Result<(), String> {
    go_to(&ctx, "/requisitions").await?;
    ctx.wait_for("[data-testid='requisitions-landing']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='requisitions-landing']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn route_tenders(ctx: BrowserTestContext) -> Result<(), String> {
    go_to(&ctx, "/tenders").await?;
    ctx.wait_for("[data-testid='tenders-landing']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='tenders-landing']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn route_evaluation(ctx: BrowserTestContext) -> Result<(), String> {
    go_to(&ctx, "/evaluation").await?;
    ctx.wait_for("[data-testid='evaluation-landing']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='evaluation-landing']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn route_contracts(ctx: BrowserTestContext) -> Result<(), String> {
    go_to(&ctx, "/contracts").await?;
    ctx.wait_for("[data-testid='contracts-landing']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='contracts-landing']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn route_purchase_orders(ctx: BrowserTestContext) -> Result<(), String> {
    go_to(&ctx, "/purchase-orders").await?;
    ctx.wait_for("[data-testid='purchase-orders-landing']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='purchase-orders-landing']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn route_goods_receipt(ctx: BrowserTestContext) -> Result<(), String> {
    go_to(&ctx, "/goods-receipt").await?;
    ctx.wait_for("[data-testid='goods-receipt-landing']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='goods-receipt-landing']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn route_suppliers(ctx: BrowserTestContext) -> Result<(), String> {
    go_to(&ctx, "/suppliers/registry").await?;
    ctx.wait_for("[data-testid='suppliers-landing']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='suppliers-landing']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn route_supplier_portal(ctx: BrowserTestContext) -> Result<(), String> {
    go_to(&ctx, "/supplier-portal").await?;
    ctx.wait_for("[data-testid='supplier-portal-landing']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='supplier-portal-landing']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn route_catalogue(ctx: BrowserTestContext) -> Result<(), String> {
    go_to(&ctx, "/catalogue").await?;
    ctx.wait_for("[data-testid='catalogue-landing']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='catalogue-landing']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn route_analytics(ctx: BrowserTestContext) -> Result<(), String> {
    go_to(&ctx, "/analytics").await?;
    ctx.wait_for("[data-testid='analytics-landing']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='analytics-landing']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn route_grc(ctx: BrowserTestContext) -> Result<(), String> {
    go_to(&ctx, "/grc").await?;
    ctx.wait_for("[data-testid='grc-landing']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='grc-landing']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn route_audit(ctx: BrowserTestContext) -> Result<(), String> {
    go_to(&ctx, "/audit").await?;
    ctx.wait_for("[data-testid='audit-landing']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='audit-landing']").await.map_err(|e| e.to_string())?;
    Ok(())
}
