use e2e_test::{BrowserTestContext, assertions::PageAssertions};

pub async fn landing_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='evaluation-landing']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='evaluation-landing']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn list_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='evaluation-list']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='evaluation-list']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn scoring_panel_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.assert_element_exists("[data-testid='scoring-panel']").await.map_err(|e| e.to_string())?;
    Ok(())
}
