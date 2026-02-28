//! Pagination component

use components::prelude::*;

/// Pagination component
#[component]
pub fn pagination(
    current_page: u32,
    total_pages: u32,
    on_page_change: Callback<u32>,
) -> View {
    let pages: Vec<u32> = calculate_visible_pages(current_page, total_pages);

    let handle_prev = Callback::new({
        let on_page_change = on_page_change.clone();
        move |_| {
            if current_page > 1 {
                on_page_change.call(current_page - 1);
            }
        }
    });

    let handle_next = Callback::new({
        let on_page_change = on_page_change.clone();
        move |_| {
            if current_page < total_pages {
                on_page_change.call(current_page + 1);
            }
        }
    });

    view! {
        style {
            r#"
            .pagination {
                display: flex;
                align-items: center;
                justify-content: center;
                gap: 4px;
                margin-top: 20px;
            }
            .pagination-btn {
                min-width: 32px;
                height: 32px;
                padding: 0 8px;
                border: 1px solid var(--border);
                background: var(--surface);
                border-radius: var(--radius-sm);
                font-size: 13px;
                cursor: pointer;
                transition: all 0.15s;
                display: flex;
                align-items: center;
                justify-content: center;
            }
            .pagination-btn:hover:not(:disabled) {
                background: var(--bg);
                border-color: var(--blue);
            }
            .pagination-btn.active {
                background: var(--blue);
                border-color: var(--blue);
                color: #fff;
            }
            .pagination-btn:disabled {
                opacity: 0.5;
                cursor: not-allowed;
            }
            .pagination-ellipsis {
                padding: 0 8px;
                color: var(--text-muted);
            }
            "#
        }

        <div class="pagination" data-testid="pagination">
            // Previous button
            <button
                class="pagination-btn"
                disabled={current_page == 1}
                on:click={handle_prev}
            >
                "←"
            </button>

            // Page numbers
            for page in pages.iter() {
                if *page == 0 {
                    // Ellipsis
                    <span class="pagination-ellipsis">"..."</span>
                } else {
                    {page_button(*page, current_page, on_page_change.clone())}
                }
            }

            // Next button
            <button
                class="pagination-btn"
                disabled={current_page == total_pages}
                on:click={handle_next}
            >
                "→"
            </button>
        </div>
    }
}

fn page_button(page: u32, current: u32, on_click: Callback<u32>) -> View {
    let is_active = page == current;
    let class = if is_active { "pagination-btn active" } else { "pagination-btn" };

    let handle_click = Callback::new({
        let on_click = on_click.clone();
        move |_| on_click.call(page)
    });

    view! {
        <button
            class={class}
            on:click={handle_click}
        >
            {page.to_string()}
        </button>
    }
}

/// Calculate which page numbers to show
fn calculate_visible_pages(current: u32, total: u32) -> Vec<u32> {
    if total <= 7 {
        return (1..=total).collect();
    }

    let mut pages = Vec::new();

    // Always show first page
    pages.push(1);

    if current > 3 {
        pages.push(0); // Ellipsis
    }

    // Show pages around current
    let start = if current <= 3 { 2 } else { current - 1 };
    let end = if current >= total - 2 { total - 1 } else { current + 1 };

    for p in start..=end {
        if p > 1 && p < total {
            pages.push(p);
        }
    }

    if current < total - 2 {
        pages.push(0); // Ellipsis
    }

    // Always show last page
    pages.push(total);

    pages
}
