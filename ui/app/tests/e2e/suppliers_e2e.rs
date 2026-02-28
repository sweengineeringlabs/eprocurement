use e2e_test::{BrowserTestContext, assertions::PageAssertions};

pub async fn registry_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='supplier-registry']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='supplier-registry']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn performance_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='supplier-performance']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='supplier-performance']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn risk_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='supplier-risk']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='supplier-risk']").await.map_err(|e| e.to_string())?;
    Ok(())
}
