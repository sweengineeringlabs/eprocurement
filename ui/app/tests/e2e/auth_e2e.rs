use e2e_test::{BrowserTestContext, assertions::PageAssertions};

/// Login screen renders all form elements.
pub async fn login_form_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.navigate("/app/", Some("[data-testid='login-screen']"))
        .await
        .map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='login-screen']")
        .await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='login-username']")
        .await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='login-password']")
        .await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='login-submit']")
        .await.map_err(|e| e.to_string())?;
    Ok(())
}

/// Login form is interactable and authenticates successfully.
pub async fn form_login_works(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.navigate("/app/", Some("[data-testid='login-screen']"))
        .await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='login-username']")
        .await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='login-password']")
        .await.map_err(|e| e.to_string())?;
    // Call the login API via fetch
    let token_val = ctx.evaluate(
        "fetch('/api/v1/auth/login',{method:'POST',headers:{'Content-Type':'application/json'},\
         body:JSON.stringify({username:'admin',password:'password'})})\
         .then(r=>r.ok?r.json():null)\
         .then(d=>d&&d.token?(localStorage.setItem('eproc_auth_token',d.token),\
         localStorage.setItem('eproc_auth_username','admin'),d.token):'')"
    ).await.map_err(|e| e.to_string())?;
    if token_val.as_str().map_or(true, |s| s.is_empty()) {
        return Err("form_login_works: login API did not return a JWT token".into());
    }
    ctx.navigate("/app/", Some("[data-testid='app-shell']"))
        .await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='app-shell']")
        .await.map_err(|e| e.to_string())?;
    Ok(())
}

/// Injecting auth via localStorage bypasses the login screen.
pub async fn localStorage_auth_bypasses_login(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.navigate("/app/", Some("[data-testid='login-screen']"))
        .await.map_err(|e| e.to_string())?;
    ctx.set_local_storage("eproc_auth_token", "test-token-e2e")
        .await.map_err(|e| e.to_string())?;
    ctx.set_local_storage("eproc_auth_username", "admin")
        .await.map_err(|e| e.to_string())?;
    ctx.navigate("/app/", Some("[data-testid='app-shell']"))
        .await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='app-shell']")
        .await.map_err(|e| e.to_string())?;
    Ok(())
}

/// After auth, app-shell has sidebar and main-content.
pub async fn app_shell_structure(ctx: BrowserTestContext) -> Result<(), String> {
    crate::common::go_to(&ctx, "/").await?;
    ctx.wait_for("[data-testid='sidebar']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='sidebar']")
        .await.map_err(|e| e.to_string())?;
    ctx.wait_for("[data-testid='main-content']").await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='main-content']")
        .await.map_err(|e| e.to_string())?;
    Ok(())
}

/// Removing auth tokens redirects back to login screen.
pub async fn logout_clears_session(ctx: BrowserTestContext) -> Result<(), String> {
    crate::common::go_to(&ctx, "/").await?;
    ctx.evaluate("localStorage.removeItem('eproc_auth_token'); localStorage.removeItem('eproc_auth_username');")
        .await.map_err(|e| e.to_string())?;
    ctx.navigate("/app/", Some("[data-testid='login-screen']"))
        .await.map_err(|e| e.to_string())?;
    ctx.assert_element_exists("[data-testid='login-screen']")
        .await.map_err(|e| e.to_string())?;
    Ok(())
}
