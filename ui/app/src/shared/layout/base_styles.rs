//! Global CSS design tokens and base styles
//! Configuration: theme.yaml, styles.yaml
//! Migrated from mockups/css/styles.css

/// Design tokens from theme.yaml
pub mod tokens {
    // Colors - light theme
    pub const NAVY: &str = "#0d2145";
    pub const NAVY_LIGHT: &str = "#1a3a6e";
    pub const BLUE: &str = "#1a5fce";
    pub const BLUE_LIGHT: &str = "#e8f0fe";
    pub const ACCENT: &str = "#c8973a";
    pub const ACCENT_LIGHT: &str = "#fef6e8";
    pub const GREEN: &str = "#1a7a4a";
    pub const GREEN_LIGHT: &str = "#e6f4ed";
    pub const ORANGE: &str = "#c26b12";
    pub const ORANGE_LIGHT: &str = "#fef3e6";
    pub const RED: &str = "#c0392b";
    pub const RED_LIGHT: &str = "#fdeaea";
    pub const PURPLE: &str = "#6b4c9a";
    pub const PURPLE_LIGHT: &str = "#f3effa";
    pub const CYAN: &str = "#0891b2";
    pub const CYAN_LIGHT: &str = "#e6f7fa";
    pub const BG: &str = "#f4f5f7";
    pub const SURFACE: &str = "#ffffff";
    pub const BORDER: &str = "#e2e6ed";
    pub const TEXT: &str = "#0d1b2e";
    pub const TEXT_MUTED: &str = "#6b7a94";

    // Shadows
    pub const SHADOW_SM: &str = "0 1px 2px #0d21450F";
    pub const SHADOW_MD: &str = "0 2px 8px #0d214514";
    pub const SHADOW_LG: &str = "0 8px 24px #0d21451F";

    // Spacing (px)
    pub const SPACE_1: u32 = 4;
    pub const SPACE_2: u32 = 8;
    pub const SPACE_3: u32 = 12;
    pub const SPACE_4: u32 = 16;
    pub const SPACE_5: u32 = 20;
    pub const SPACE_6: u32 = 24;
    pub const SPACE_8: u32 = 32;

    // Border radius (px)
    pub const RADIUS_SM: u32 = 4;
    pub const RADIUS_MD: u32 = 6;
    pub const RADIUS_LG: u32 = 8;

    // Layout (px)
    pub const SIDEBAR_WIDTH: u32 = 260;
    pub const TOPBAR_HEIGHT: u32 = 64;

    // Typography
    pub const FONT_FAMILY: &str = "'IBM Plex Sans', -apple-system, BlinkMacSystemFont, sans-serif";
    pub const FONT_FAMILY_HEADING: &str = "'Playfair Display', serif";
    pub const FONT_FAMILY_MONO: &str = "'IBM Plex Mono', monospace";
}

/// Returns the global stylesheet with design tokens
/// Generated from theme.yaml and styles.yaml
pub fn base_styles() -> &'static str {
    r#"
/* Google Fonts - must be first */
@import url('https://fonts.googleapis.com/css2?family=Playfair+Display:wght@500;600;700&family=IBM+Plex+Sans:wght@400;500;600&family=IBM+Plex+Mono:wght@400;500&display=swap');

/* SARS eProcurement System - Design Tokens */
/* RFP 33/2025 */
/* Source: theme.yaml, styles.yaml */

*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

:root {
    /* Color Palette */
    --navy: #0d2145;
    --navy-light: #1a3a6e;
    --blue: #1a5fce;
    --blue-light: #e8f0fe;
    --accent: #c8973a;
    --accent-light: #fef6e8;
    --green: #1a7a4a;
    --green-light: #e6f4ed;
    --orange: #c26b12;
    --orange-light: #fef3e6;
    --red: #c0392b;
    --red-light: #fdeaea;
    --purple: #6b4c9a;
    --purple-light: #f3effa;
    --cyan: #0891b2;
    --cyan-light: #e6f7fa;

    /* Surfaces & Typography */
    --bg: #f4f5f7;
    --surface: #ffffff;
    --border: #e2e6ed;
    --text: #0d1b2e;
    --text-muted: #6b7a94;

    /* Shadows */
    --shadow-sm: 0 1px 2px #0d21450F;
    --shadow: 0 2px 8px #0d214514;
    --shadow-lg: 0 8px 24px #0d21451F;

    /* Border Radius */
    --radius-sm: 4px;
    --radius: 6px;
    --radius-lg: 8px;

    /* Layout */
    --sidebar-width: 260px;
    --topbar-height: 64px;

    /* Spacing */
    --space-1: 4px;
    --space-2: 8px;
    --space-3: 12px;
    --space-4: 16px;
    --space-5: 20px;
    --space-6: 24px;
    --space-8: 32px;

    /* Typography */
    --font-family: 'IBM Plex Sans', -apple-system, BlinkMacSystemFont, sans-serif;
    --font-family-heading: 'Playfair Display', serif;
    --font-family-mono: 'IBM Plex Mono', monospace;
}

body {
    font-family: var(--font-family);
    background: var(--bg);
    color: var(--text);
    font-size: 14px;
    line-height: 1.5;
}

/* Typography */
h1, h2, h3, .heading {
    font-family: var(--font-family-heading);
    color: var(--navy);
}

.font-mono {
    font-family: var(--font-family-mono);
}

/* Main Layout */
.main {
    margin-left: var(--sidebar-width);
    min-height: 100vh;
}

.content {
    padding: var(--space-6);
}

/* Animations */
@keyframes fadeIn {
    from { opacity: 0; transform: translateY(6px); }
    to { opacity: 1; transform: translateY(0); }
}

.page-enter {
    animation: fadeIn 0.2s ease;
}

@keyframes spin {
    to { transform: rotate(360deg); }
}

@keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.6; }
}

/* Grid Utilities */
.grid-2 { display: grid; grid-template-columns: repeat(2, 1fr); gap: var(--space-5); }
.grid-3 { display: grid; grid-template-columns: repeat(3, 1fr); gap: var(--space-5); }
.grid-4 { display: grid; grid-template-columns: repeat(4, 1fr); gap: var(--space-5); }
.grid-3-2 { display: grid; grid-template-columns: 3fr 2fr; gap: var(--space-5); }
.grid-2-1 { display: grid; grid-template-columns: 2fr 1fr; gap: var(--space-5); }
.grid-1-2 { display: grid; grid-template-columns: 1fr 2fr; gap: var(--space-5); }

/* Flex Utilities */
.flex { display: flex; }
.items-center { align-items: center; }
.justify-between { justify-content: space-between; }
.gap-2 { gap: var(--space-2); }
.gap-3 { gap: var(--space-3); }
.gap-4 { gap: var(--space-4); }

/* Text Utilities */
.text-muted { color: var(--text-muted); }
.text-blue { color: var(--blue); }
.text-green { color: var(--green); }
.text-orange { color: var(--orange); }
.text-red { color: var(--red); }
.text-right { text-align: right; }
.text-center { text-align: center; }
.font-bold { font-weight: 600; }

/* Spacing Utilities */
.mb-0 { margin-bottom: 0; }
.mb-2 { margin-bottom: var(--space-2); }
.mb-4 { margin-bottom: var(--space-4); }
.mb-6 { margin-bottom: var(--space-6); }
.mt-4 { margin-top: var(--space-4); }
.mt-6 { margin-top: var(--space-6); }

/* Buttons */
.btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 10px 16px;
    border-radius: var(--radius);
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s;
    border: none;
    text-decoration: none;
}
.btn svg { width: 16px; height: 16px; }
.btn-primary { background: var(--blue); color: #fff; }
.btn-primary:hover { background: #1550b0; }
.btn-secondary { background: var(--surface); color: var(--text); border: 1px solid var(--border); }
.btn-secondary:hover { background: var(--bg); }
.btn-accent { background: var(--accent); color: #fff; }
.btn-accent:hover { background: #b3862f; }
.btn-success { background: var(--green); color: #fff; }
.btn-success:hover { background: #156b3e; }
.btn-danger { background: var(--red); color: #fff; }
.btn-danger:hover { background: #a33025; }
.btn-sm { padding: 6px 12px; font-size: 12px; }
.btn-xs { padding: 4px 8px; font-size: 11px; }
.btn-lg { padding: 14px 24px; font-size: 15px; }
.btn:disabled { opacity: 0.5; cursor: not-allowed; }

/* Panels */
.panel { background: var(--surface); border: 1px solid var(--border); border-radius: var(--radius-lg); box-shadow: var(--shadow-sm); }
.panel-header { padding: 16px 20px; border-bottom: 1px solid var(--border); display: flex; align-items: center; justify-content: space-between; }
.panel-title { font-weight: 600; font-size: 15px; color: var(--navy); }
.panel-body { padding: 20px; }
.panel-footer { padding: 16px 20px; border-top: 1px solid var(--border); display: flex; justify-content: flex-end; gap: 12px; background: var(--bg); border-radius: 0 0 var(--radius-lg) var(--radius-lg); }

/* Forms */
.form-group { display: flex; flex-direction: column; gap: 6px; }
.form-group label { font-size: 12px; font-weight: 500; color: var(--text-muted); }
.form-group input, .form-group select, .form-group textarea {
    padding: 10px 12px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    font-size: 13px;
    font-family: inherit;
    transition: all 0.15s;
    background: var(--surface);
}
.form-group input:focus, .form-group select:focus, .form-group textarea:focus {
    outline: none;
    border-color: var(--blue);
    box-shadow: 0 0 0 3px var(--blue-light);
}

/* Data Tables */
.data-table { width: 100%; border-collapse: collapse; }
.data-table th, .data-table td { padding: 12px 16px; text-align: left; border-bottom: 1px solid var(--border); }
.data-table th { font-weight: 600; color: var(--text-muted); font-size: 12px; text-transform: uppercase; letter-spacing: 0.5px; }
.data-table tbody tr:hover { background: var(--bg); }

/* App Shell */
.app-shell {
    display: flex;
    min-height: 100vh;
}
.app-main {
    flex: 1;
    margin-left: var(--sidebar-width);
    display: flex;
    flex-direction: column;
}
.app-content {
    flex: 1;
    padding: var(--space-6);
}

/* Sidebar */
.sidebar {
    position: fixed;
    top: 0;
    left: 0;
    width: var(--sidebar-width);
    height: 100vh;
    background: var(--navy);
    color: #fff;
    display: flex;
    flex-direction: column;
    z-index: 100;
}
.sidebar-header {
    padding: 20px;
    border-bottom: 1px solid rgba(255,255,255,0.1);
}
.logo {
    display: flex;
    align-items: center;
    gap: 12px;
    text-decoration: none;
    color: #fff;
}
.logo-icon {
    width: 40px;
    height: 40px;
    background: var(--accent);
    border-radius: var(--radius);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 700;
    font-size: 16px;
}
.logo-text {
    font-family: 'Playfair Display', serif;
    font-size: 18px;
    font-weight: 600;
}
.logo-sub {
    font-size: 11px;
    opacity: 0.7;
    margin-top: 2px;
}
.sidebar-nav {
    flex: 1;
    overflow-y: auto;
    padding: 16px 0;
}
.nav-section {
    margin-bottom: 8px;
}
.nav-section-title {
    padding: 8px 20px;
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 1px;
    opacity: 0.5;
    font-weight: 600;
}
.nav-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 20px;
    color: rgba(255,255,255,0.7);
    text-decoration: none;
    cursor: pointer;
    transition: all 0.15s;
    border-left: 3px solid transparent;
    font-size: 13px;
}
.nav-item:hover {
    background: rgba(255,255,255,0.05);
    color: #fff;
}
.nav-item.active {
    background: rgba(255,255,255,0.1);
    color: #fff;
    border-left-color: var(--accent);
}
.nav-item svg {
    width: 18px;
    height: 18px;
    opacity: 0.8;
}
.nav-badge {
    margin-left: auto;
    background: var(--accent);
    color: var(--navy);
    font-size: 10px;
    font-weight: 600;
    padding: 2px 6px;
    border-radius: 10px;
}

/* Topbar */
.topbar {
    position: sticky;
    top: 0;
    height: var(--topbar-height);
    background: var(--surface);
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 24px;
    z-index: 50;
}
.topbar-left {
    display: flex;
    align-items: center;
    gap: 16px;
}
.search-box {
    display: flex;
    align-items: center;
    gap: 8px;
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 8px 12px;
    width: 280px;
}
.search-box svg {
    width: 16px;
    height: 16px;
    color: var(--text-muted);
}
.search-box input {
    border: none;
    background: transparent;
    outline: none;
    font-size: 13px;
    width: 100%;
}
.topbar-right {
    display: flex;
    align-items: center;
    gap: 16px;
}
.topbar-btn {
    width: 36px;
    height: 36px;
    border-radius: var(--radius);
    border: 1px solid var(--border);
    background: var(--surface);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.15s;
    position: relative;
}
.topbar-btn:hover {
    background: var(--bg);
}
.topbar-btn svg {
    width: 18px;
    height: 18px;
    color: var(--text-muted);
}
.notification-dot {
    position: absolute;
    top: 6px;
    right: 6px;
    width: 8px;
    height: 8px;
    background: var(--red);
    border-radius: 50%;
    border: 2px solid var(--surface);
}
.user-menu {
    display: flex;
    align-items: center;
    gap: 10px;
    cursor: pointer;
}
.user-avatar {
    width: 36px;
    height: 36px;
    background: var(--blue);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #fff;
    font-weight: 600;
    font-size: 14px;
}
.user-info {
    text-align: right;
}
.user-name {
    font-weight: 500;
    font-size: 13px;
}
.user-role {
    font-size: 11px;
    color: var(--text-muted);
}

/* Page Header */
.page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 24px;
}
.page-header-left h1 {
    font-family: 'Playfair Display', serif;
    font-size: 24px;
    font-weight: 600;
    color: var(--navy);
    margin-bottom: 4px;
}
.page-header-left p {
    color: var(--text-muted);
    font-size: 13px;
}
.page-header-actions {
    display: flex;
    gap: 12px;
}

/* KPI Cards */
.kpi-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 16px;
    margin-bottom: 24px;
}
.kpi-card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: 20px;
    border-top: 3px solid var(--blue);
    transition: transform 0.2s, box-shadow 0.2s;
}
.kpi-card:hover {
    transform: translateY(-2px);
    box-shadow: var(--shadow);
}
.kpi-card.green { border-top-color: var(--green); }
.kpi-card.orange { border-top-color: var(--orange); }
.kpi-card.accent { border-top-color: var(--accent); }
.kpi-card.purple { border-top-color: var(--purple); }
.kpi-card.red { border-top-color: var(--red); }
.kpi-card.cyan { border-top-color: var(--cyan); }
.kpi-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    margin-bottom: 12px;
}
.kpi-icon {
    width: 40px;
    height: 40px;
    border-radius: var(--radius);
    display: flex;
    align-items: center;
    justify-content: center;
}
.kpi-icon.blue { background: var(--blue-light); color: var(--blue); }
.kpi-icon.green { background: var(--green-light); color: var(--green); }
.kpi-icon.orange { background: var(--orange-light); color: var(--orange); }
.kpi-icon.accent { background: var(--accent-light); color: var(--accent); }
.kpi-icon.purple { background: var(--purple-light); color: var(--purple); }
.kpi-icon.red { background: var(--red-light); color: var(--red); }
.kpi-icon.cyan { background: var(--cyan-light); color: var(--cyan); }
.kpi-icon svg {
    width: 20px;
    height: 20px;
}
.kpi-label {
    font-size: 12px;
    color: var(--text-muted);
    margin-bottom: 4px;
}
.kpi-value {
    font-size: 24px;
    font-weight: 600;
    color: var(--navy);
    font-family: 'IBM Plex Mono', monospace;
}
.kpi-delta {
    font-size: 11px;
    color: var(--text-muted);
    margin-top: 8px;
}
.kpi-delta .up { color: var(--green); }
.kpi-delta .down { color: var(--red); }

/* Status Badges */
.status {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 10px;
    border-radius: 12px;
    font-size: 11px;
    font-weight: 500;
}
.status::before {
    content: '';
    width: 6px;
    height: 6px;
    border-radius: 50%;
}
.status-approved, .status-active, .status-published, .status-complete {
    background: var(--green-light);
    color: var(--green);
}
.status-approved::before, .status-active::before, .status-published::before, .status-complete::before {
    background: var(--green);
}
.status-pending, .status-draft, .status-in-progress {
    background: var(--orange-light);
    color: var(--orange);
}
.status-pending::before, .status-draft::before, .status-in-progress::before {
    background: var(--orange);
}
.status-rejected, .status-cancelled, .status-expired, .status-failed {
    background: var(--red-light);
    color: var(--red);
}
.status-rejected::before, .status-cancelled::before, .status-expired::before, .status-failed::before {
    background: var(--red);
}
.status-new, .status-submitted, .status-open {
    background: var(--blue-light);
    color: var(--blue);
}
.status-new::before, .status-submitted::before, .status-open::before {
    background: var(--blue);
}

/* Progress Bars */
.progress-container {
    display: flex;
    align-items: center;
    gap: 12px;
}
.progress-track {
    flex: 1;
    height: 8px;
    background: var(--bg);
    border-radius: 4px;
    overflow: hidden;
}
.progress-fill {
    height: 100%;
    border-radius: 4px;
    transition: width 0.3s ease;
}
.progress-fill.blue { background: var(--blue); }
.progress-fill.green { background: var(--green); }
.progress-fill.orange { background: var(--orange); }
.progress-fill.accent { background: var(--accent); }
.progress-fill.red { background: var(--red); }
.progress-label {
    font-size: 12px;
    font-weight: 600;
    color: var(--navy);
    min-width: 45px;
}

/* Pie Chart */
.pie-chart-circles {
    display: flex;
    justify-content: space-around;
    gap: 16px;
}
.pie-chart-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
}
.pie-chart-circle {
    width: 60px;
    height: 60px;
    border-radius: 50%;
    border: 4px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: center;
}
.pie-chart-value {
    font-size: 14px;
    font-weight: 600;
    color: var(--navy);
}
.pie-chart-label {
    font-size: 11px;
    color: var(--text-muted);
}

/* Bar Chart */
.bar-chart-container {
    display: flex;
    align-items: flex-end;
    justify-content: space-around;
    padding: 20px;
    background: linear-gradient(180deg, var(--blue-light) 0%, transparent 100%);
    border-radius: var(--radius);
}

/* Timeline */
.timeline {
    position: relative;
    padding-left: 24px;
}
.timeline::before {
    content: '';
    position: absolute;
    left: 6px;
    top: 0;
    bottom: 0;
    width: 2px;
    background: var(--border);
}
.timeline-item {
    position: relative;
    padding-bottom: 20px;
}
.timeline-item::before {
    content: '';
    position: absolute;
    left: -24px;
    top: 4px;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: var(--blue);
    border: 3px solid var(--surface);
}

/* Responsive */
@media (max-width: 768px) {
    .sidebar { transform: translateX(-100%); }
    .app-main { margin-left: 0; }
    .main { margin-left: 0; }
    .kpi-grid { grid-template-columns: repeat(2, 1fr); }
    .grid-2, .grid-3, .grid-3-2, .grid-2-1, .grid-1-2 {
        grid-template-columns: 1fr;
    }
}

"#
}
