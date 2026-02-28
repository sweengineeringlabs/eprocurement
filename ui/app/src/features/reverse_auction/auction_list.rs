//! Auction list view with status filtering and pagination

use components::prelude::*;
use wasm_bindgen::JsCast;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, data_table, DataTableColumn, DataTableRow,
    status_badge, StatusType,
    pagination,
};
use crate::shared::forms::filter_bar;
use crate::util::format::{format_currency, format_date};
use super::store::ReverseAuctionStore;
use super::types::{AuctionStatus, AuctionFilter};
use super::service;

/// Auction list page
#[component]
pub fn auction_list() -> View {
    let store = use_context::<ReverseAuctionStore>();

    // Filter signals
    let filter_status = signal(String::new());
    let filter_date_from = signal(String::new());
    let filter_date_to = signal(String::new());
    let search_query = signal(String::new());

    // Load data on mount
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                service::load_auctions(&store).await;
            });
        }
    });

    let auctions = store.auctions.clone();
    let pagination_state = store.pagination.clone();
    let loading = store.loading.clone();

    // Handle filter changes
    let handle_filter_status = Callback::new({
        let filter_status = filter_status.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            filter_status.set(select.value());
        }
    });

    let handle_date_from = Callback::new({
        let filter_date_from = filter_date_from.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            filter_date_from.set(input.value());
        }
    });

    let handle_date_to = Callback::new({
        let filter_date_to = filter_date_to.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            filter_date_to.set(input.value());
        }
    });

    let handle_search = Callback::new({
        let search_query = search_query.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            search_query.set(input.value());
        }
    });

    // Apply filters
    let handle_apply_filters: Callback<()> = Callback::new({
        let store = store.clone();
        let filter_status = filter_status.clone();
        let filter_date_from = filter_date_from.clone();
        let filter_date_to = filter_date_to.clone();
        let search_query = search_query.clone();
        move |_| {
            let filter = AuctionFilter {
                status: match filter_status.get().as_str() {
                    "draft" => Some(AuctionStatus::Draft),
                    "scheduled" => Some(AuctionStatus::Scheduled),
                    "live" => Some(AuctionStatus::Live),
                    "ended" => Some(AuctionStatus::Ended),
                    "awarded" => Some(AuctionStatus::Awarded),
                    "cancelled" => Some(AuctionStatus::Cancelled),
                    _ => None,
                },
                date_from: if filter_date_from.get().is_empty() { None } else { Some(filter_date_from.get()) },
                date_to: if filter_date_to.get().is_empty() { None } else { Some(filter_date_to.get()) },
                search_query: if search_query.get().is_empty() { None } else { Some(search_query.get()) },
                department: None,
                category: None,
            };
            store.filter.set(filter);
        }
    });

    // Clear filters
    let handle_clear_filters: Callback<()> = Callback::new({
        let filter_status = filter_status.clone();
        let filter_date_from = filter_date_from.clone();
        let filter_date_to = filter_date_to.clone();
        let search_query = search_query.clone();
        let store = store.clone();
        move |_| {
            filter_status.set(String::new());
            filter_date_from.set(String::new());
            filter_date_to.set(String::new());
            search_query.set(String::new());
            store.filter.set(AuctionFilter::default());
        }
    });

    // Handle row click - navigate to live auction page
    let handle_row_click = Callback::new({
        move |auction_id: String| {
            web_sys::window()
                .unwrap()
                .location()
                .set_href(&format!("#/reverse-auctions/{}", auction_id))
                .ok();
        }
    });

    // Handle page change
    let handle_page_change = Callback::new({
        let pagination_state = pagination_state.clone();
        move |page: u32| {
            let mut state = pagination_state.get();
            state.current_page = page;
            pagination_state.set(state);
        }
    });

    // Table columns
    let columns = vec![
        DataTableColumn {
            key: "reference".to_string(),
            label: "Reference".to_string(),
            width: Some("130px".to_string()),
            align: None,
            cell_class: Some("id-cell".to_string()),
        },
        DataTableColumn {
            key: "title".to_string(),
            label: "Title".to_string(),
            width: None,
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "starting_price".to_string(),
            label: "Starting Price".to_string(),
            width: Some("140px".to_string()),
            align: Some("right".to_string()),
            cell_class: Some("amount-cell".to_string()),
        },
        DataTableColumn {
            key: "current_bid".to_string(),
            label: "Current Bid".to_string(),
            width: Some("140px".to_string()),
            align: Some("right".to_string()),
            cell_class: Some("amount-cell".to_string()),
        },
        DataTableColumn {
            key: "bidders".to_string(),
            label: "Bidders".to_string(),
            width: Some("80px".to_string()),
            align: Some("center".to_string()),
            cell_class: None,
        },
        DataTableColumn {
            key: "end_time".to_string(),
            label: "End Time".to_string(),
            width: Some("140px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "status".to_string(),
            label: "Status".to_string(),
            width: Some("120px".to_string()),
            align: None,
            cell_class: None,
        },
    ];

    // Filter auctions based on current filter
    let filter = store.filter.clone();
    let filtered_auctions: Vec<_> = auctions.get().iter()
        .filter(|a| {
            let f = filter.get();
            let status_match = f.status.map_or(true, |fs| a.status == fs);
            let search_match = f.search_query.as_ref().map_or(true, |q| {
                a.title.to_lowercase().contains(&q.to_lowercase()) ||
                a.reference_number.to_lowercase().contains(&q.to_lowercase())
            });
            status_match && search_match
        })
        .cloned()
        .collect();

    // Transform auctions to table rows
    let rows: Vec<DataTableRow> = filtered_auctions.iter().map(|auction| {
        let status = match auction.status {
            AuctionStatus::Draft => status_badge(StatusType::Draft),
            AuctionStatus::Scheduled => status_badge(StatusType::Pending),
            AuctionStatus::Live => status_badge(StatusType::InProgress),
            AuctionStatus::Ended => status_badge(StatusType::Complete),
            AuctionStatus::Awarded => status_badge(StatusType::Active),
            AuctionStatus::Cancelled => status_badge(StatusType::Cancelled),
        };

        let current_bid = auction.current_bid
            .map(|b| format_currency(b))
            .unwrap_or_else(|| "-".to_string());

        let end_time = if auction.end_time.is_empty() {
            "-".to_string()
        } else {
            format_datetime_short(&auction.end_time)
        };

        let bidder_count = auction.bidders.len().to_string();

        // Add live indicator for live auctions
        let status_cell = if auction.status == AuctionStatus::Live {
            view! {
                <div class="status-with-indicator">
                    <span class="live-indicator"></span>
                    {status}
                </div>
            }
        } else {
            status
        };

        DataTableRow {
            id: auction.id.clone(),
            cells: vec![
                view! { <span class="id-cell">{auction.reference_number.clone()}</span> },
                view! {
                    <div class="auction-title-cell">
                        <span class="title">{auction.title.clone()}</span>
                        <span class="item-name">{auction.item.name.clone()}</span>
                    </div>
                },
                view! { <span class="amount-cell">{format_currency(auction.starting_price)}</span> },
                view! { <span class="amount-cell current-bid">{current_bid}</span> },
                view! { <span class="text-center">{bidder_count}</span> },
                view! { <span>{end_time}</span> },
                status_cell,
            ],
        }
    }).collect();

    let pag_state = pagination_state.get();

    // Icons
    let icon_live = r#"<svg viewBox="0 0 24 24" fill="currentColor"><circle cx="12" cy="12" r="4"/></svg>"#;

    view! {
        style {
            r#"
            .auction-list { display: flex; flex-direction: column; gap: var(--space-4); }
            .filter-group {
                display: flex;
                align-items: center;
                gap: 8px;
            }
            .filter-group label {
                font-size: 12px;
                font-weight: 500;
                color: var(--text-muted);
            }
            .filter-group select,
            .filter-group input {
                padding: 6px 10px;
                border: 1px solid var(--border);
                border-radius: var(--radius-sm);
                font-size: 12px;
                background: var(--surface);
            }
            .filter-group select:focus,
            .filter-group input:focus {
                outline: none;
                border-color: var(--blue);
            }
            .filter-spacer {
                flex: 1;
            }
            .search-input {
                min-width: 200px;
            }
            .loading-overlay {
                display: flex;
                align-items: center;
                justify-content: center;
                padding: 40px;
                color: var(--text-muted);
            }
            .text-center { text-align: center; }
            .auction-title-cell {
                display: flex;
                flex-direction: column;
                gap: 2px;
            }
            .auction-title-cell .title {
                font-weight: 500;
                color: var(--navy);
            }
            .auction-title-cell .item-name {
                font-size: 12px;
                color: var(--text-muted);
            }
            .current-bid {
                color: var(--green);
                font-weight: 600;
            }
            .status-with-indicator {
                display: flex;
                align-items: center;
                gap: 8px;
            }
            .live-indicator {
                width: 8px;
                height: 8px;
                border-radius: 50%;
                background: var(--red);
                animation: pulse 1.5s infinite;
            }
            @keyframes pulse {
                0%, 100% { opacity: 1; transform: scale(1); }
                50% { opacity: 0.5; transform: scale(1.2); }
            }
            .kpi-row {
                display: grid;
                grid-template-columns: repeat(4, 1fr);
                gap: 16px;
                margin-bottom: 16px;
            }
            .kpi-card {
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius-md);
                padding: 16px 20px;
                display: flex;
                flex-direction: column;
                gap: 4px;
            }
            .kpi-card.live {
                border-color: var(--red);
                background: linear-gradient(to right, #ef44440D, var(--surface));
            }
            .kpi-label {
                font-size: 12px;
                color: var(--text-muted);
                text-transform: uppercase;
                letter-spacing: 0.5px;
            }
            .kpi-value {
                font-size: 24px;
                font-weight: 600;
                color: var(--navy);
                font-family: IBM Plex Mono, monospace;
            }
            .kpi-value.live {
                color: var(--red);
            }
            "#
        }

        <div class="auction-list" data-testid="auction-list">
            {page_header(
                "Reverse Auctions".to_string(),
                Some("Manage live reverse auctions for competitive bidding".to_string()),
                vec![
                    view! { <button class="btn btn-secondary">"Export"</button> },
                    view! { <a href="#/reverse-auctions/new" class="btn btn-primary">"New Auction"</a> },
                ]
            )}

            // KPI summary row
            <div class="kpi-row">
                {kpi_card("Live Auctions", live_count(&filtered_auctions), true)}
                {kpi_card("Scheduled", scheduled_count(&filtered_auctions), false)}
                {kpi_card("Total Savings", total_savings(&filtered_auctions), false)}
                {kpi_card("Active Bidders", active_bidders(&filtered_auctions), false)}
            </div>

            // Filter bar
            {filter_bar(vec![
                view! {
                    <div class="filter-group">
                        <label>"Status"</label>
                        <select on:change={handle_filter_status}>
                            <option value="">"All Statuses"</option>
                            <option value="draft">"Draft"</option>
                            <option value="scheduled">"Scheduled"</option>
                            <option value="live">"Live"</option>
                            <option value="ended">"Ended"</option>
                            <option value="awarded">"Awarded"</option>
                            <option value="cancelled">"Cancelled"</option>
                        </select>
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"From"</label>
                        <input type="date" on:change={handle_date_from} />
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"To"</label>
                        <input type="date" on:change={handle_date_to} />
                    </div>
                },
                view! { <div class="filter-spacer"></div> },
                view! {
                    <div class="filter-group">
                        <input
                            type="text"
                            class="search-input"
                            placeholder="Search auctions..."
                            on:input={handle_search}
                        />
                    </div>
                },
                view! {
                    <button class="btn btn-sm btn-secondary" on:click={handle_clear_filters}>"Clear"</button>
                },
                view! {
                    <button class="btn btn-sm btn-primary" on:click={handle_apply_filters}>"Apply"</button>
                },
            ])}

            // Data table
            {panel(
                format!("Auctions ({} total)", filtered_auctions.len()),
                vec![],
                vec![
                    if loading.get() {
                        view! { <div class="loading-overlay">"Loading auctions..."</div> }
                    } else if rows.is_empty() {
                        view! {
                            <div class="loading-overlay">
                                "No auctions found. Create a new auction to get started."
                            </div>
                        }
                    } else {
                        view! {
                            <div>
                                {data_table(columns, rows, Some(handle_row_click))}
                                {pagination(pag_state.current_page, pag_state.total_pages.max(1), handle_page_change)}
                            </div>
                        }
                    }
                ]
            )}
        </div>
    }
}

/// KPI card component
fn kpi_card(label: &str, value: String, is_live: bool) -> View {
    let card_class = if is_live { "kpi-card live" } else { "kpi-card" };
    let value_class = if is_live { "kpi-value live" } else { "kpi-value" };

    view! {
        <div class={card_class}>
            <span class="kpi-label">{label}</span>
            <span class={value_class}>{value}</span>
        </div>
    }
}

/// Count live auctions
fn live_count(auctions: &[super::types::ReverseAuction]) -> String {
    auctions.iter()
        .filter(|a| a.status == AuctionStatus::Live)
        .count()
        .to_string()
}

/// Count scheduled auctions
fn scheduled_count(auctions: &[super::types::ReverseAuction]) -> String {
    auctions.iter()
        .filter(|a| a.status == AuctionStatus::Scheduled)
        .count()
        .to_string()
}

/// Calculate total savings from completed auctions
fn total_savings(auctions: &[super::types::ReverseAuction]) -> String {
    let savings: f64 = auctions.iter()
        .filter(|a| a.status == AuctionStatus::Awarded || a.status == AuctionStatus::Ended)
        .filter_map(|a| {
            a.current_bid.map(|bid| a.starting_price - bid)
        })
        .sum();

    format_currency(savings)
}

/// Count active bidders across live auctions
fn active_bidders(auctions: &[super::types::ReverseAuction]) -> String {
    auctions.iter()
        .filter(|a| a.status == AuctionStatus::Live)
        .flat_map(|a| a.bidders.iter())
        .filter(|b| b.is_active)
        .count()
        .to_string()
}

/// Format datetime for display (short format)
fn format_datetime_short(iso_datetime: &str) -> String {
    // Parse ISO datetime and format as "DD MMM HH:MM"
    if iso_datetime.len() >= 16 {
        let date_part = &iso_datetime[..10];
        let time_part = &iso_datetime[11..16];
        let parts: Vec<&str> = date_part.split('-').collect();
        if parts.len() == 3 {
            let month = match parts[1] {
                "01" => "Jan",
                "02" => "Feb",
                "03" => "Mar",
                "04" => "Apr",
                "05" => "May",
                "06" => "Jun",
                "07" => "Jul",
                "08" => "Aug",
                "09" => "Sep",
                "10" => "Oct",
                "11" => "Nov",
                "12" => "Dec",
                _ => parts[1],
            };
            return format!("{} {} {}", parts[2], month, time_part);
        }
    }
    iso_datetime.to_string()
}
