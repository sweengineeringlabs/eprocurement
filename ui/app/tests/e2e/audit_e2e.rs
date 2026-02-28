use e2e_test::{BrowserTestContext, assertions::PageAssertions};

pub async fn trail_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='audit-trail']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='audit-trail']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn filters_work(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.assert_element_exists("[data-testid='audit-filters']").await.map_err(|e| e.to_string())?;
    Ok(())
}
