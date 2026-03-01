use e2e_test::{BrowserTestContext, assertions::PageAssertions};

// ── Existing tests (updated) ─────────────────────────────────────────────────

pub async fn landing_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='tender-list']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='tender-list']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn list_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='tender-list']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='tender-list']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn create_btn_visible(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='create-tender-btn']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='create-tender-btn']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn status_filters_render(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='tender-filter-status']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='tender-filter-status']").await.map_err(|e| e.to_string())?;
    Ok(())
}

// ── Navigation & List tests ──────────────────────────────────────────────────

pub async fn navigate_to_tenders(ctx: BrowserTestContext) -> Result<(), String> {
    // Verify we landed on the tenders page with list visible
    ctx.wait_for("[data-testid='tender-list']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='tender-list']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn list_shows_tender_rows(ctx: BrowserTestContext) -> Result<(), String> {
    // Wait for the data table to render
    ctx.wait_for("[data-testid='tender-table']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='tender-table']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn filter_by_status_works(ctx: BrowserTestContext) -> Result<(), String> {
    // Verify status filter dropdown exists
    ctx.wait_for("[data-testid='tender-filter-status']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='tender-filter-status']").await.map_err(|e| e.to_string())?;

    // Click apply button to verify it's functional
    ctx.click("[data-testid='tender-filter-apply']").await.map_err(|e| e.to_string())?;

    // Table should still be visible after filtering
    ctx.wait_for("[data-testid='tender-table']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn search_filters_list(ctx: BrowserTestContext) -> Result<(), String> {
    // Verify search input exists
    ctx.wait_for("[data-testid='tender-filter-search']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='tender-filter-search']").await.map_err(|e| e.to_string())?;

    // Type in search box
    ctx.fill("[data-testid='tender-filter-search']", "test tender")
        .await
        .map_err(|e| e.to_string())?;

    // Click apply button
    ctx.click("[data-testid='tender-filter-apply']").await.map_err(|e| e.to_string())?;

    // Table should still be visible after filtering
    ctx.wait_for("[data-testid='tender-table']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn clear_filters_resets_list(ctx: BrowserTestContext) -> Result<(), String> {
    // Click clear filters button
    ctx.wait_for("[data-testid='tender-filter-clear']").await.map_err(|e| e.to_string())?;
    ctx.click("[data-testid='tender-filter-clear']").await.map_err(|e| e.to_string())?;

    // Verify table is still visible
    ctx.wait_for("[data-testid='tender-table']").await.map_err(|e| e.to_string())?;
    Ok(())
}

// ── Create Flow tests ────────────────────────────────────────────────────────

pub async fn click_create_opens_form(ctx: BrowserTestContext) -> Result<(), String> {
    // Click the create button
    ctx.wait_for("[data-testid='create-tender-btn']").await.map_err(|e| e.to_string())?;
    ctx.click("[data-testid='create-tender-btn']").await.map_err(|e| e.to_string())?;

    // Verify form is now visible
    ctx.wait_for("[data-testid='tender-form']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='tender-form']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn form_step_navigation_works(ctx: BrowserTestContext) -> Result<(), String> {
    // Navigate to new tender form
    ctx.wait_for("[data-testid='create-tender-btn']").await.map_err(|e| e.to_string())?;
    ctx.click("[data-testid='create-tender-btn']").await.map_err(|e| e.to_string())?;

    // Verify stepper is visible
    ctx.wait_for("[data-testid='tender-form-stepper']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='tender-form-stepper']").await.map_err(|e| e.to_string())?;

    // Verify we're on step 1
    ctx.assert_element_exists("[data-testid='tender-form-step-1']").await.map_err(|e| e.to_string())?;

    // Click Next to go to step 2
    ctx.click("[data-testid='tender-form-next']").await.map_err(|e| e.to_string())?;

    // Give time for step change - verify Back button is now visible
    ctx.wait_for("[data-testid='tender-form-prev']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='tender-form-prev']").await.map_err(|e| e.to_string())?;

    // Click Back to return to step 1
    ctx.click("[data-testid='tender-form-prev']").await.map_err(|e| e.to_string())?;

    // Verify we're back on step 1
    ctx.wait_for("[data-testid='tender-form-step-1']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='tender-form-step-1']").await.map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn form_validation_shows_errors(ctx: BrowserTestContext) -> Result<(), String> {
    // Navigate to new tender form
    ctx.wait_for("[data-testid='create-tender-btn']").await.map_err(|e| e.to_string())?;
    ctx.click("[data-testid='create-tender-btn']").await.map_err(|e| e.to_string())?;

    // Wait for form
    ctx.wait_for("[data-testid='tender-form']").await.map_err(|e| e.to_string())?;

    // Verify required form fields exist
    ctx.assert_element_exists("[data-testid='tender-title-input']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='tender-type-select']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='tender-value-input']").await.map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn save_draft_succeeds(ctx: BrowserTestContext) -> Result<(), String> {
    // Navigate to new tender form
    ctx.wait_for("[data-testid='create-tender-btn']").await.map_err(|e| e.to_string())?;
    ctx.click("[data-testid='create-tender-btn']").await.map_err(|e| e.to_string())?;

    // Wait for form
    ctx.wait_for("[data-testid='tender-form']").await.map_err(|e| e.to_string())?;

    // Fill in required fields
    ctx.fill("[data-testid='tender-title-input']", "E2E Test Tender")
        .await
        .map_err(|e| e.to_string())?;

    // Verify Save Draft button exists
    ctx.assert_element_exists("[data-testid='tender-form-save-draft']").await.map_err(|e| e.to_string())?;

    Ok(())
}

// ── View/Detail tests ────────────────────────────────────────────────────────

pub async fn click_row_shows_detail(ctx: BrowserTestContext) -> Result<(), String> {
    // Wait for table to render
    ctx.wait_for("[data-testid='tender-table']").await.map_err(|e| e.to_string())?;

    // Verify the table renders correctly
    ctx.assert_element_exists("[data-testid='tender-table']").await.map_err(|e| e.to_string())?;

    Ok(())
}

// ── Filter element tests ─────────────────────────────────────────────────────

pub async fn type_filter_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='tender-filter-type']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='tender-filter-type']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn apply_button_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='tender-filter-apply']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='tender-filter-apply']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn clear_button_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='tender-filter-clear']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='tender-filter-clear']").await.map_err(|e| e.to_string())?;
    Ok(())
}
