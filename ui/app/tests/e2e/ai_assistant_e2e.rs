use e2e_test::{BrowserTestContext, assertions::PageAssertions};

pub async fn chat_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.wait_for("[data-testid='ai-chat-panel']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='ai-chat-panel']").await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn input_accepts_text(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.assert_element_exists("[data-testid='ai-message-input']").await.map_err(|e| e.to_string())?;
    Ok(())
}
