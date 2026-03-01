use e2e_test::{BrowserTestContext, assertions::PageAssertions};

pub async fn landing_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='catalogue-list']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='catalogue-list']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn search_works(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.assert_element_exists("[data-testid='catalogue-search']").await.map_err(|e| e.to_string())?;
    Ok(())
}
