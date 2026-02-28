use e2e_test::{BrowserTestContext, assertions::PageAssertions};

pub async fn sidebar_layout(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='sidebar']").await.map_err(|e| e.to_string())?;
    let style = ctx.evaluate(
        "getComputedStyle(document.querySelector('[data-testid=\"sidebar\"]')).display"
    ).await.map_err(|e| e.to_string())?;
    if style.as_str() != Some("flex") {
        return Err(format!("Expected sidebar display: flex, got: {:?}", style));
    }
    Ok(())
}

pub async fn button_styles(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for(".btn-primary").await.map_err(|e| e.to_string())?;
    let cursor = ctx.evaluate(
        "getComputedStyle(document.querySelector('.btn-primary')).cursor"
    ).await.map_err(|e| e.to_string())?;
    if cursor.as_str() != Some("pointer") {
        return Err(format!("Expected button cursor: pointer, got: {:?}", cursor));
    }
    Ok(())
}

pub async fn input_styles(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.assert_element_exists("input").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn table_styles(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.assert_element_exists("[data-testid='data-table']").await.map_err(|e| e.to_string())?;
    Ok(())
}
