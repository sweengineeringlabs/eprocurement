use e2e_test::{BrowserTestContext, assertions::PageAssertions};

pub async fn landing_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='goods-receipt-landing']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='goods-receipt-landing']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn list_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='goods-receipt-list']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='goods-receipt-list']").await.map_err(|e| e.to_string())?;
    Ok(())
}
