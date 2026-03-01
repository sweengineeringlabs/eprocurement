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
/// Uses sidebar navigation since the app uses internal signal-based routing.
pub async fn go_to(ctx: &BrowserTestContext, route: &str) -> Result<(), String> {
    // First navigate to app root and wait for the app-shell to render
    ctx.navigate("/app/", Some("[data-testid='app-shell']"))
        .await
        .map_err(|e| e.to_string())?;

    // Inject auth tokens into localStorage
    ctx.set_local_storage("eproc_auth_token", "test-token-e2e")
        .await
        .map_err(|e| e.to_string())?;
    ctx.set_local_storage("eproc_auth_username", "admin")
        .await
        .map_err(|e| e.to_string())?;

    // Map route path to sidebar nav testid
    // testids are generated from label: nav-{label.to_lowercase().replace(' ', '-')}
    let nav_testid = match route {
        "/" | "/dashboard" => None, // Dashboard is default
        "/tenders" => Some("nav-tenders"),
        "/requisitions" => Some("nav-requisitions"),
        "/evaluation" => Some("nav-evaluation"),
        "/contracts" => Some("nav-contracts"),
        "/purchase-orders" => Some("nav-purchase-orders"),
        "/goods-receipt" => Some("nav-goods-receipt"),
        "/suppliers" | "/suppliers/registry" => Some("nav-registry"),
        "/suppliers/performance" => Some("nav-performance"),
        "/suppliers/risk" => Some("nav-risk"),
        "/supplier-portal" => Some("nav-supplier-portal"),
        "/catalogue" => Some("nav-browse"),
        "/catalogue/admin" => Some("nav-admin"),
        "/analytics" => Some("nav-analytics"),
        "/grc" => Some("nav-grc-dashboard"),
        "/audit" => Some("nav-audit-trail"),
        "/nbac" => Some("nav-nbac-reviews"),
        "/auctions" | "/reverse-auction" => Some("nav-reverse-auction"),
        "/sourcing" | "/sourcing-plan" => Some("nav-sourcing-plans"),
        "/bbbee" => Some("nav-b-bbee-goals"),
        "/agsa" => Some("nav-agsa-reviews"),
        "/documents" => Some("nav-documents"),
        "/ai-assistant" => Some("nav-ai-assistant"),
        "/mobile" => None, // No direct nav for mobile
        _ => None,
    };

    // Click sidebar nav link to navigate (app uses internal routing, not URL)
    if let Some(testid) = nav_testid {
        let selector = format!("[data-testid='{}']", testid);
        ctx.wait_for(&selector).await.map_err(|e| e.to_string())?;

        // Scroll element into view before clicking (needed for items lower in sidebar)
        let scroll_js = format!(
            r#"document.querySelector("{}").scrollIntoView({{ behavior: 'instant', block: 'center' }})"#,
            selector.replace('"', "\\\"")
        );
        ctx.evaluate(&scroll_js).await.map_err(|e| e.to_string())?;

        ctx.click(&selector).await.map_err(|e| e.to_string())?;
    }

    Ok(())
}

// ── CSS assertion helpers ─────────────────────────────────────────────────
// These helpers are provided for future visual regression tests

/// Assert computed CSS property matches expected value
#[allow(dead_code)]
pub async fn assert_css(
    ctx: &BrowserTestContext,
    selector: &str,
    property: &str,
    expected: &str,
) -> Result<(), String> {
    // Convert CSS property name (kebab-case) to JS camelCase for property access
    // e.g., "z-index" -> "zIndex", "background-color" -> "backgroundColor"
    let js_property = property.split('-')
        .enumerate()
        .map(|(i, part)| {
            if i == 0 {
                part.to_string()
            } else {
                let mut chars = part.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().chain(chars).collect(),
                }
            }
        })
        .collect::<String>();

    // Use double quotes for selector in JS (matching existing working tests)
    // Escape any double quotes in the selector
    let escaped_selector = selector.replace('"', "\\\"");
    let js = format!(
        r#"(() => {{
            const el = document.querySelector("{}");
            if (!el) return {{ error: "Element not found: {}" }};
            return {{ value: getComputedStyle(el).{} }};
        }})()"#,
        escaped_selector, escaped_selector, js_property
    );

    let result = ctx.evaluate(&js).await.map_err(|e| e.to_string())?;

    if let Some(err) = result.get("error") {
        return Err(err.as_str().unwrap_or("Unknown error").to_string());
    }

    let actual = result.get("value")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if !css_values_match(actual, expected) {
        return Err(format!(
            "CSS mismatch for '{}' on '{}': expected '{}', got '{}'",
            property, selector, expected, actual
        ));
    }

    Ok(())
}

/// Compare CSS values with tolerance for browser normalization
#[allow(dead_code)]
fn css_values_match(actual: &str, expected: &str) -> bool {
    // Exact match
    if actual == expected {
        return true;
    }

    // Numeric comparison with 1px tolerance
    if let (Some(a), Some(e)) = (parse_px(actual), parse_px(expected)) {
        return (a - e).abs() < 1.5;
    }

    // Color comparison (hex vs rgb)
    if let (Some(a), Some(e)) = (parse_color(actual), parse_color(expected)) {
        return a == e;
    }

    false
}

#[allow(dead_code)]
fn parse_px(s: &str) -> Option<f64> {
    s.trim().trim_end_matches("px").parse().ok()
}

#[allow(dead_code)]
fn parse_color(s: &str) -> Option<(u8, u8, u8)> {
    // Handle rgb(r, g, b) format
    if s.starts_with("rgb(") {
        let inner = s.trim_start_matches("rgb(").trim_end_matches(")");
        let parts: Vec<u8> = inner.split(',')
            .filter_map(|p| p.trim().parse::<u8>().ok())
            .collect();
        if parts.len() == 3 {
            return Some((parts[0], parts[1], parts[2]));
        }
    }
    // Handle #RRGGBB format
    if s.starts_with('#') && s.len() == 7 {
        let r = u8::from_str_radix(&s[1..3], 16).ok()?;
        let g = u8::from_str_radix(&s[3..5], 16).ok()?;
        let b = u8::from_str_radix(&s[5..7], 16).ok()?;
        return Some((r, g, b));
    }
    None
}

/// Assert element is truly hidden (not just display:none)
#[allow(dead_code)]
pub async fn assert_hidden(
    ctx: &BrowserTestContext,
    selector: &str,
) -> Result<(), String> {
    // Escape single quotes in selector for JS string
    let escaped_selector = selector.replace('\'', "\\'");
    let js = format!(
        r#"(() => {{
            const el = document.querySelector('{}');
            if (!el) return {{ hidden: true, reason: 'not_found' }};
            const style = getComputedStyle(el);
            const rect = el.getBoundingClientRect();
            return {{
                hidden: style.display === 'none'
                    || style.visibility === 'hidden'
                    || style.opacity === '0'
                    || (rect.width === 0 && rect.height === 0),
                display: style.display,
                visibility: style.visibility,
                opacity: style.opacity,
                width: rect.width,
                height: rect.height
            }};
        }})()"#,
        escaped_selector
    );

    let result = ctx.evaluate(&js).await.map_err(|e| e.to_string())?;

    let is_hidden = result.get("hidden")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if !is_hidden {
        return Err(format!(
            "Element '{}' should be hidden but is visible: {:?}",
            selector, result
        ));
    }

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
