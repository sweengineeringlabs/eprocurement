use e2e_test::{BrowserTestContext, assertions::PageAssertions};

pub async fn goals_render(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='bbbee-goals']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='bbbee-goals']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn targets_visible(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.assert_element_exists("[data-testid='bbbee-targets']").await.map_err(|e| e.to_string())?;
    Ok(())
}
