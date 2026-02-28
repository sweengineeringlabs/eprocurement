use e2e_test::{BrowserTestContext, assertions::PageAssertions};

pub async fn landing_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='tenders-landing']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='tenders-landing']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn list_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='tenders-list']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='tenders-list']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn create_btn_visible(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.assert_element_exists("[data-testid='create-tender-btn']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn status_filters_render(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.assert_element_exists("[data-testid='status-filters']").await.map_err(|e| e.to_string())?;
    Ok(())
}
