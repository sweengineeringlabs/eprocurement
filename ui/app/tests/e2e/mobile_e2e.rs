use e2e_test::{BrowserTestContext, assertions::PageAssertions};

pub async fn app_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='mobile-app']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='mobile-app']").await.map_err(|e| e.to_string())?;
    Ok(())
}
