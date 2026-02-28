use e2e_test::{BrowserTestContext, assertions::PageAssertions};

pub async fn list_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='reverse-auction-list']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='reverse-auction-list']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn live_panel_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.assert_element_exists("[data-testid='live-auction-panel']").await.map_err(|e| e.to_string())?;
    Ok(())
}
