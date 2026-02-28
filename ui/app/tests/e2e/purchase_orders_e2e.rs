use e2e_test::{BrowserTestContext, assertions::PageAssertions};

pub async fn landing_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='purchase-orders-landing']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='purchase-orders-landing']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn list_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='purchase-orders-list']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='purchase-orders-list']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn create_btn_visible(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.assert_element_exists("[data-testid='create-po-btn']").await.map_err(|e| e.to_string())?;
    Ok(())
}
