# E2E Test Architecture

## Overview

The eProcurement frontend uses a Rust-based end-to-end (E2E) testing framework that runs browser tests via WebAssembly. Tests are organized into **suites** by feature, with built-in filtering to run one or more suites at a time.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         E2E Test Architecture                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│   ┌──────────────────────┐                                                  │
│   │   RSC_TEST_SUITE     │  Environment Variable (optional)                 │
│   │   e.g. "tenders"     │  Set to filter which suites run                  │
│   └──────────┬───────────┘                                                  │
│              │                                                              │
│              ▼                                                              │
│   ┌──────────────────────────────────────────────────────────────────────┐  │
│   │                        main.rs                                        │  │
│   │  ┌────────────────────────────────────────────────────────────────┐  │  │
│   │  │  fn should_run_suite(name: &str) -> bool                       │  │  │
│   │  │    - Checks RSC_TEST_SUITE env var                             │  │  │
│   │  │    - Returns true if suite should run                          │  │  │
│   │  └────────────────────────────────────────────────────────────────┘  │  │
│   │                              │                                        │  │
│   │                              ▼                                        │  │
│   │  ┌────────────────────────────────────────────────────────────────┐  │  │
│   │  │  fn run_eprocurement_e2e()                                     │  │  │
│   │  │                                                                 │  │  │
│   │  │    if should_run_suite("auth")       → auth_e2e.rs             │  │  │
│   │  │    if should_run_suite("navigation") → navigation_e2e.rs       │  │  │
│   │  │    if should_run_suite("dashboard")  → dashboard_e2e.rs        │  │  │
│   │  │    if should_run_suite("tenders")    → tenders_e2e.rs          │  │  │
│   │  │    ...                                                          │  │  │
│   │  └────────────────────────────────────────────────────────────────┘  │  │
│   └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Directory Structure

```
app/tests/e2e/
├── main.rs                 # Test runner + suite filtering logic
├── common.rs               # Shared helpers (config, auth, test wrappers)
│
├── auth_e2e.rs             # Suite: "auth"
├── navigation_e2e.rs       # Suite: "navigation"
├── dashboard_e2e.rs        # Suite: "dashboard"
├── requisitions_e2e.rs     # Suite: "requisitions"
├── tenders_e2e.rs          # Suite: "tenders"
├── evaluation_e2e.rs       # Suite: "evaluation"
├── contracts_e2e.rs        # Suite: "contracts"
├── purchase_orders_e2e.rs  # Suite: "purchase_orders"
├── goods_receipt_e2e.rs    # Suite: "goods_receipt"
├── suppliers_e2e.rs        # Suite: "suppliers"
├── supplier_portal_e2e.rs  # Suite: "supplier_portal"
├── catalogue_e2e.rs        # Suite: "catalogue"
├── analytics_e2e.rs        # Suite: "analytics"
├── grc_e2e.rs              # Suite: "grc"
├── audit_e2e.rs            # Suite: "audit"
├── nbac_e2e.rs             # Suite: "nbac"
├── reverse_auction_e2e.rs  # Suite: "reverse_auction"
├── documents_e2e.rs        # Suite: "documents"
├── ai_assistant_e2e.rs     # Suite: "ai_assistant"
├── sourcing_plan_e2e.rs    # Suite: "sourcing_plan"
├── bbbee_e2e.rs            # Suite: "bbbee"
├── agsa_e2e.rs             # Suite: "agsa"
├── mobile_e2e.rs           # Suite: "mobile"
└── visual_e2e.rs           # Suite: "visual"
```

## Core Components

### 1. Suite Filter (`main.rs`)

```rust
fn should_run_suite(name: &str) -> bool {
    match std::env::var("RSC_TEST_SUITE") {
        Ok(filter) => filter.split(',').any(|s| s.trim() == name),
        Err(_) => true,  // No filter = run all
    }
}
```

### 2. Test Runner (`main.rs`)

```rust
#[tokio::test(flavor = "multi_thread")]
async fn run_eprocurement_e2e() {
    let mut runner = BrowserTestRunner::new(build_config());

    if should_run_suite("auth") {
        let mut suite = BrowserTestSuite::new("auth");
        suite.add_test(make_test("login_form_renders", auth_e2e::login_form_renders));
        // ... more tests
        runner.add_suite(suite);
    }

    // ... more suites

    let (_, summary) = runner.run().await;
    assert_eq!(summary.failed + summary.timed_out, 0);
}
```

### 3. Common Helpers (`common.rs`)

| Function | Purpose |
|----------|---------|
| `build_config()` | Creates browser runner config (port, headless, timeout) |
| `go_to(ctx, route)` | Navigates to route with auth tokens injected |
| `make_test(name, fn)` | Wraps async test function into `BrowserTestCase` |
| `make_auth_test(name, route, fn)` | Same as above, but navigates to route first |

### 4. Suite Test File (e.g., `tenders_e2e.rs`)

```rust
use e2e_test::BrowserTestContext;

pub async fn landing_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.query_selector("[data-testid='tenders-landing']")
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn list_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.query_selector("[data-testid='tenders-list']")
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
```

## Data Flow

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│  common.rs  │    │ Suite File  │    │   main.rs   │    │   Runner    │
│             │    │             │    │             │    │             │
│ build_config│◄───│test functions│◄───│ add to suite│───►│  execute    │
│ make_test   │    │             │    │ if enabled  │    │  browser    │
│ go_to       │    │             │    │             │    │  tests      │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
```

## Available Suites

### Core
| Suite | Description |
|-------|-------------|
| `auth` | Login, logout, session management |
| `navigation` | Routing, sidebar, nav links |
| `dashboard` | KPI cards, charts, activity feed |
| `visual` | CSS/layout verification |

### Procurement Workflow
| Suite | Description |
|-------|-------------|
| `requisitions` | Purchase requisition CRUD |
| `tenders` | Tender management, publication |
| `evaluation` | Bid evaluation, scoring |
| `contracts` | Contract lifecycle, milestones |
| `purchase_orders` | PO creation, management |
| `goods_receipt` | GRN processing |

### Supplier Management
| Suite | Description |
|-------|-------------|
| `suppliers` | Registry, performance, risk |
| `supplier_portal` | Supplier-facing dashboard |
| `catalogue` | Product/service catalogue |

### Compliance & Governance
| Suite | Description |
|-------|-------------|
| `grc` | Governance, Risk, Compliance |
| `audit` | Audit trail, logging |
| `nbac` | National Bid Adjudication |
| `bbbee` | B-BBEE compliance goals |
| `agsa` | Auditor-General reviews |

### Advanced Features
| Suite | Description |
|-------|-------------|
| `analytics` | Dashboards, reports |
| `reverse_auction` | Live auction interface |
| `documents` | Document library |
| `ai_assistant` | AI chat interface |
| `sourcing_plan` | Sourcing strategy |
| `mobile` | Mobile supplier app |

## Usage

### Run All Tests
```bash
cargo test e2e --features browser
```

### Run Single Suite
```bash
RSC_TEST_SUITE=tenders cargo test e2e --features browser
```

### Run Multiple Suites
```bash
RSC_TEST_SUITE=auth,dashboard,tenders cargo test e2e --features browser
```

### Run with Visible Browser (Non-Headless)
```bash
RSC_TEST_HEADLESS=false RSC_TEST_SUITE=auth cargo test e2e --features browser
```

### Custom Port
```bash
RSC_TEST_PORT=9000 cargo test e2e --features browser
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `RSC_TEST_SUITE` | (none) | Comma-separated list of suites to run |
| `RSC_TEST_PORT` | `8082` | Dev server port |
| `RSC_TEST_BASE_URL` | `http://localhost:{port}` | Base URL for tests |
| `RSC_TEST_HEADLESS` | `true` | Run browser in headless mode |
| `RSC_TEST_TIMEOUT` | `120` | Test timeout in seconds |

## Adding a New Suite

1. **Create test file**: `app/tests/e2e/{feature}_e2e.rs`

```rust
use e2e_test::BrowserTestContext;

pub async fn landing_renders(ctx: BrowserTestContext) -> Result<(), String> {
    ctx.query_selector("[data-testid='feature-landing']")
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
```

2. **Register module in `main.rs`**:

```rust
mod feature_e2e;
```

3. **Add suite block in `run_eprocurement_e2e()`**:

```rust
if should_run_suite("feature") {
    let mut suite = BrowserTestSuite::new("feature");
    suite.add_test(make_auth_test("landing_renders", "/feature", feature_e2e::landing_renders));
    runner.add_suite(suite);
}
```

## Test Conventions

1. **Test IDs**: Use `data-testid` attributes in components for reliable selectors
2. **Naming**: Test functions use snake_case describing what they verify
3. **Auth**: Use `make_auth_test` for routes requiring authentication
4. **Errors**: Return `Result<(), String>` with descriptive error messages
