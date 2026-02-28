use e2e_test::{BrowserTestContext, assertions::PageAssertions};

pub async fn dashboard_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='analytics-dashboard']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='analytics-dashboard']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn charts_visible(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.assert_element_exists("[data-testid='analytics-charts']").await.map_err(|e| e.to_string())?;
    Ok(())
}
