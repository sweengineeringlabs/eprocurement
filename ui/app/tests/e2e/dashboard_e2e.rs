use e2e_test::{BrowserTestContext, assertions::PageAssertions};

/// Verify dashboard landing page renders
pub async fn landing_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='dashboard-landing']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='dashboard-landing']").await.map_err(|e| e.to_string())?;
    Ok(())
}

/// Verify page header renders with correct title
pub async fn page_header_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for(".page-header").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists(".page-header").await.map_err(|e| e.to_string())?;
    let title = ctx.evaluate(
        "document.querySelector('.page-header h1')?.textContent || ''"
    ).await.map_err(|e| e.to_string())?;
    if !title.as_str().map_or(false, |s| s.contains("Procurement Dashboard")) {
        return Err(format!("Expected 'Procurement Dashboard' in title, got: {:?}", title));
    }
    Ok(())
}

/// Verify KPI cards grid renders
pub async fn kpi_cards_render(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for(".kpi-grid").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists(".kpi-grid").await.map_err(|e| e.to_string())?;
    let count = ctx.evaluate(
        "document.querySelectorAll('.kpi-card').length"
    ).await.map_err(|e| e.to_string())?;
    if count.as_u64().unwrap_or(0) < 4 {
        return Err(format!("Expected at least 4 KPI cards, got: {:?}", count));
    }
    Ok(())
}

/// Verify panels render correctly
pub async fn panels_render(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for(".panel").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists(".panel").await.map_err(|e| e.to_string())?;
    Ok(())
}

/// Verify recent requisitions table renders
pub async fn requisitions_table_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for(".data-table").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists(".data-table").await.map_err(|e| e.to_string())?;
    Ok(())
}

/// Verify recent activity timeline is visible
pub async fn recent_activity_visible(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for(".timeline").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists(".timeline").await.map_err(|e| e.to_string())?;
    Ok(())
}

/// Verify charts section renders (bar chart + pie chart)
pub async fn charts_render(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for(".grid-2").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists(".grid-2").await.map_err(|e| e.to_string())?;
    Ok(())
}

/// Verify action buttons are visible (Export Report, New Requisition)
pub async fn action_buttons_visible(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.assert_element_exists(".btn-primary").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists(".btn-secondary").await.map_err(|e| e.to_string())?;
    Ok(())
}
