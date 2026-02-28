//! Global CSS design tokens and base styles
//! Migrated from mockups/css/styles.css

/// Returns the global stylesheet with design tokens
pub fn base_styles() -> &'static str {
    r#"
/* SARS eProcurement System - Design Tokens */
/* RFP 33/2025 */

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
}

body {
    font-family: IBM Plex Sans, -apple-system, BlinkMacSystemFont, sans-serif;
    background: var(--bg);
    color: var(--text);
    font-size: 14px;
    line-height: 1.5;
}

/* Typography */
h1, h2, h3, .heading {
    font-family: Playfair Display, serif;
    color: var(--navy);
}

.font-mono {
    font-family: IBM Plex Mono, monospace;
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

/* Responsive */
@media (max-width: 768px) {
    .sidebar { transform: translateX(-100%); }
    .main { margin-left: 0; }
    .grid-2, .grid-3, .grid-3-2, .grid-2-1, .grid-1-2 {
        grid-template-columns: 1fr;
    }
}

/* Google Fonts (injected via link in index.html) */
@import url('https://fonts.googleapis.com/css2?family=Playfair+Display:wght@500;600;700&family=IBM+Plex+Sans:wght@400;500;600&family=IBM+Plex+Mono:wght@400;500&display=swap');
"#
}
