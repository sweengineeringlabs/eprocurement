use e2e_test::{BrowserTestContext, assertions::PageAssertions};

pub async fn reviews_render(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='agsa-reviews']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='agsa-reviews']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn findings_visible(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.assert_element_exists("[data-testid='agsa-findings']").await.map_err(|e| e.to_string())?;
    Ok(())
}
