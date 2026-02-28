use e2e_test::{BrowserTestContext, assertions::PageAssertions};

pub async fn reviews_render(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='nbac-reviews']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='nbac-reviews']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn approval_workflow_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.assert_element_exists("[data-testid='approval-workflow']").await.map_err(|e| e.to_string())?;
    Ok(())
}
