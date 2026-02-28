use std::future::Future;
use std::path::PathBuf;
use std::time::Duration;

use e2e_test::{
    BrowserRunnerConfig, BrowserTestCase, BrowserTestContext, Viewport, WebServerConfig,
};

// ── Runner config ─────────────────────────────────────────────────────────

pub fn build_config() -> BrowserRunnerConfig {
    let port: u16 = std::env::var("RSC_TEST_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8082);

    let base_url = std::env::var("RSC_TEST_BASE_URL")
        .unwrap_or_else(|_| format!("http://localhost:{}", port));

    let headless = std::env::var("RSC_TEST_HEADLESS")
        .map(|v| v != "false" && v != "0")
        .unwrap_or(true);

    let timeout_secs: u64 = std::env::var("RSC_TEST_TIMEOUT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(120);

    // workspace root = two levels above CARGO_MANIFEST_DIR (ui/app → ui → eprocurement)
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    let dist_dir = workspace_root
        .join("ui/app/dist")
        .to_string_lossy()
        .into_owned();

    // cargo-webuild lives in the rustscript workspace (sibling repo).
    let rustscript_root = workspace_root
        .parent()
        .unwrap()
        .join("rustscript");

    let server_cmd = std::env::var("RSC_SERVER_CMD").unwrap_or_else(|_| {
        format!(
            "{} serve --dist {} --port {} --base-path /app",
            rustscript_root.join("target/debug/cargo-webuild").to_string_lossy(),
            dist_dir,
            port,
        )
    });

    BrowserRunnerConfig::new()
        .headless(headless)
        .verbose(true)
        .timeout(Duration::from_secs(timeout_secs))
        .retries(1)
        .viewport(Viewport::default())
        .base_url(base_url)
        .web_server(
            WebServerConfig::new(&server_cmd)
                .port(port)
                .cwd(&workspace_root),
        )
}

// ── Auth helper ───────────────────────────────────────────────────────────

/// Navigate to a feature route with auth tokens injected into localStorage.
pub async fn go_to(ctx: &BrowserTestContext, route: &str) -> Result<(), String> {
    ctx.navigate("/app/", Some("[data-testid='login-screen']"))
        .await
        .map_err(|e| e.to_string())?;
    ctx.set_local_storage("eproc_auth_token", "test-token-e2e")
        .await
        .map_err(|e| e.to_string())?;
    ctx.set_local_storage("eproc_auth_username", "admin")
        .await
        .map_err(|e| e.to_string())?;
    let full_route = if route == "/" {
        "/app/".to_string()
    } else {
        format!("/app{}", route)
    };
    ctx.navigate(&full_route, Some("[data-testid='app-shell']"))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

// ── make_test wrappers ────────────────────────────────────────────────────

/// Wrap a plain async test fn into a `BrowserTestCase`.
pub fn make_test<F, Fut>(name: &str, f: F) -> BrowserTestCase
where
    F: Fn(BrowserTestContext) -> Fut + Send + Sync + Copy + 'static,
    Fut: Future<Output = Result<(), String>> + Send + 'static,
{
    BrowserTestCase::new(name, move |ctx| async move { f(ctx).await })
}

/// Wrap a feature test fn: injects auth, navigates to `route`, then calls `f`.
pub fn make_auth_test<F, Fut>(name: &str, route: &'static str, f: F) -> BrowserTestCase
where
    F: Fn(BrowserTestContext) -> Fut + Send + Sync + Copy + 'static,
    Fut: Future<Output = Result<(), String>> + Send + 'static,
{
    BrowserTestCase::new(name, move |ctx| async move {
        go_to(&ctx, route).await?;
        f(ctx).await
    })
}
