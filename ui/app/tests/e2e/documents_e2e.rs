use e2e_test::{BrowserTestContext, assertions::PageAssertions};

pub async fn library_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='documents-library']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='documents-library']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn upload_visible(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.assert_element_exists("[data-testid='upload-btn']").await.map_err(|e| e.to_string())?;
    Ok(())
}
