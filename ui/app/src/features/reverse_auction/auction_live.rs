//! Live auction page with real-time bidding UI, countdown timer, and bid history

use components::prelude::*;
use wasm_bindgen::JsCast;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, panel_with_footer,
    status_badge, StatusType,
    bbbee_badge, BbbeeLevel,
    notice_bar, NoticeType,
    progress_bar, ProgressColor,
};
use crate::util::format::{format_currency, format_currency_full, format_number};
use super::store::ReverseAuctionStore;
use super::types::{AuctionStatus, PlaceBidRequest, BidHistoryEntry};
use super::service;

/// Live auction page component
#[component]
pub fn auction_live() -> View {
    let store = use_context::<ReverseAuctionStore>();

    // Local state for bid input
    let bid_amount = signal(String::new());

    // Load auction data on mount
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                // In production, get auction ID from route params
                // For demo, load first live auction
                service::load_auctions(&store).await;
                if let Some(auction) = store.auctions.get().iter().find(|a| a.status == AuctionStatus::Live) {
                    service::get_auction(&store, &auction.id).await;
                    service::connect_websocket(&store, &auction.id);
                    service::start_countdown_timer(&store);
                }
            });
        }
    });

    let selected = store.selected.clone();
    let bid_history = store.bid_history.clone();
    let countdown = store.countdown.clone();
    let active_bidders = store.active_bidders.clone();
    let current_user_bid = store.current_user_bid.clone();
    let is_leading = store.is_leading.clone();
    let user_rank = store.user_rank.clone();
    let ws_connected = store.ws_connected.clone();
    let loading = store.loading.clone();
    let bid_submitting = store.bid_submitting.clone();
    let bid_error = store.bid_error.clone();

    // Handle bid input change
    let handle_bid_input = Callback::new({
        let bid_amount = bid_amount.clone();
        let store = store.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let value = input.value();
            bid_amount.set(value.clone());
            store.set_bid_input(value);
        }
    });

    // Handle place bid
    let handle_place_bid: Callback<()> = Callback::new({
        let store = store.clone();
        let bid_amount = bid_amount.clone();
        move |_| {
            let store = store.clone();
            let amount_str = bid_amount.get();
            let bid_amount = bid_amount.clone();

            // Parse bid amount
            let amount: f64 = match amount_str.replace(",", "").replace(" ", "").parse() {
                Ok(a) => a,
                Err(_) => {
                    store.bid_error.set(Some("Please enter a valid amount".to_string()));
                    return;
                }
            };

            spawn(async move {
                if let Some(auction) = store.selected.get() {
                    let request = PlaceBidRequest {
                        auction_id: auction.id.clone(),
                        amount,
                    };
                    let _ = service::place_bid(&store, request).await;
                    bid_amount.set(String::new());
                }
            });
        }
    });

    // Handle quick bid buttons
    let handle_quick_bid = {
        let store = store.clone();
        move |decrement: f64| {
            let store = store.clone();
            Callback::<()>::new(move |_| {
                let store = store.clone();
                spawn(async move {
                    if let Some(auction) = store.selected.get() {
                        let current = auction.current_bid.unwrap_or(auction.starting_price);
                        let new_amount = current - decrement;
                        let request = PlaceBidRequest {
                            auction_id: auction.id.clone(),
                            amount: new_amount,
                        };
                        let _ = service::place_bid(&store, request).await;
                    }
                });
            })
        }
    };

    // Handle back navigation
    let handle_back: Callback<()> = Callback::new({
        move |_| {
            web_sys::window()
                .unwrap()
                .location()
                .set_href("#/reverse-auctions")
                .ok();
        }
    });

    // Icons
    let icon_back = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M19 12H5"/><polyline points="12 19 5 12 12 5"/></svg>"#;
    let icon_clock = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>"#;
    let icon_users = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>"#;
    let icon_trending = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="23 6 13.5 15.5 8.5 10.5 1 18"/><polyline points="17 6 23 6 23 12"/></svg>"#;
    let icon_check = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>"#;
    let icon_alert = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>"#;
    let icon_wifi = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M5 12.55a11 11 0 0 1 14.08 0"/><path d="M1.42 9a16 16 0 0 1 21.16 0"/><path d="M8.53 16.11a6 6 0 0 1 6.95 0"/><line x1="12" y1="20" x2="12.01" y2="20"/></svg>"#;
    let icon_wifi_off = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="1" y1="1" x2="23" y2="23"/><path d="M16.72 11.06A10.94 10.94 0 0 1 19 12.55"/><path d="M5 12.55a10.94 10.94 0 0 1 5.17-2.39"/><path d="M10.71 5.05A16 16 0 0 1 22.58 9"/><path d="M1.42 9a15.91 15.91 0 0 1 4.7-2.88"/><path d="M8.53 16.11a6 6 0 0 1 6.95 0"/><line x1="12" y1="20" x2="12.01" y2="20"/></svg>"#;

    view! {
        style {
            r#"
            .auction-live {
                display: flex;
                flex-direction: column;
                gap: var(--space-4);
            }
            .back-link {
                display: inline-flex;
                align-items: center;
                gap: 8px;
                color: var(--text-muted);
                text-decoration: none;
                font-size: 13px;
                margin-bottom: 8px;
                cursor: pointer;
            }
            .back-link:hover {
                color: var(--blue);
            }
            .back-link svg {
                width: 16px;
                height: 16px;
            }
            .live-header {
                display: flex;
                justify-content: space-between;
                align-items: flex-start;
                gap: 24px;
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius-lg);
                padding: 24px;
            }
            .live-header.is-live {
                border-color: var(--red);
                box-shadow: 0 0 0 1px var(--red);
            }
            .auction-info {
                flex: 1;
            }
            .auction-info h1 {
                font-family: Playfair Display, serif;
                font-size: 24px;
                font-weight: 600;
                color: var(--navy);
                margin-bottom: 8px;
                display: flex;
                align-items: center;
                gap: 12px;
            }
            .live-badge {
                display: inline-flex;
                align-items: center;
                gap: 6px;
                padding: 4px 10px;
                background: var(--red);
                color: white;
                font-size: 11px;
                font-weight: 600;
                text-transform: uppercase;
                letter-spacing: 0.5px;
                border-radius: 12px;
            }
            .live-badge .dot {
                width: 6px;
                height: 6px;
                background: white;
                border-radius: 50%;
                animation: blink 1s infinite;
            }
            @keyframes blink {
                0%, 100% { opacity: 1; }
                50% { opacity: 0.3; }
            }
            .auction-meta {
                display: flex;
                flex-wrap: wrap;
                gap: 16px;
                color: var(--text-muted);
                font-size: 13px;
            }
            .auction-meta-item {
                display: flex;
                align-items: center;
                gap: 6px;
            }
            .auction-meta-item svg {
                width: 14px;
                height: 14px;
            }
            .countdown-display {
                text-align: center;
                min-width: 200px;
            }
            .countdown-label {
                font-size: 12px;
                color: var(--text-muted);
                text-transform: uppercase;
                letter-spacing: 0.5px;
                margin-bottom: 8px;
            }
            .countdown-time {
                font-family: IBM Plex Mono, monospace;
                font-size: 48px;
                font-weight: 600;
                color: var(--navy);
                line-height: 1;
            }
            .countdown-time.ending-soon {
                color: var(--red);
                animation: pulse-color 1s infinite;
            }
            @keyframes pulse-color {
                0%, 100% { color: var(--red); }
                50% { color: var(--orange); }
            }
            .countdown-time.ended {
                color: var(--text-muted);
            }
            .live-layout {
                display: grid;
                grid-template-columns: 1fr 400px;
                gap: 24px;
            }
            @media (max-width: 1200px) {
                .live-layout {
                    grid-template-columns: 1fr;
                }
            }
            .main-content {
                display: flex;
                flex-direction: column;
                gap: 24px;
            }
            .bidding-panel {
                background: var(--surface);
                border: 2px solid var(--border);
                border-radius: var(--radius-lg);
                padding: 24px;
            }
            .bidding-panel.is-leading {
                border-color: var(--green);
                background: linear-gradient(to bottom, #22c55e0D, var(--surface));
            }
            .current-bid-section {
                text-align: center;
                padding-bottom: 24px;
                border-bottom: 1px solid var(--border);
                margin-bottom: 24px;
            }
            .current-bid-label {
                font-size: 14px;
                color: var(--text-muted);
                margin-bottom: 8px;
            }
            .current-bid-amount {
                font-family: IBM Plex Mono, monospace;
                font-size: 42px;
                font-weight: 700;
                color: var(--green);
            }
            .current-bid-info {
                display: flex;
                justify-content: center;
                gap: 24px;
                margin-top: 12px;
                font-size: 13px;
                color: var(--text-muted);
            }
            .bid-input-section {
                margin-bottom: 24px;
            }
            .bid-input-row {
                display: flex;
                gap: 12px;
                margin-bottom: 16px;
            }
            .bid-input-wrapper {
                flex: 1;
                position: relative;
            }
            .bid-input-wrapper .currency-prefix {
                position: absolute;
                left: 16px;
                top: 50%;
                transform: translateY(-50%);
                font-size: 18px;
                color: var(--text-muted);
            }
            .bid-input {
                width: 100%;
                padding: 16px 16px 16px 48px;
                font-size: 24px;
                font-family: IBM Plex Mono, monospace;
                border: 2px solid var(--border);
                border-radius: var(--radius-md);
                background: var(--surface);
            }
            .bid-input:focus {
                outline: none;
                border-color: var(--blue);
            }
            .bid-input.error {
                border-color: var(--red);
            }
            .bid-input:disabled {
                background: var(--bg);
                cursor: not-allowed;
            }
            .place-bid-btn {
                padding: 16px 32px;
                font-size: 16px;
                font-weight: 600;
                white-space: nowrap;
            }
            .place-bid-btn:disabled {
                opacity: 0.5;
                cursor: not-allowed;
            }
            .bid-error {
                color: var(--red);
                font-size: 13px;
                margin-top: 8px;
            }
            .quick-bid-section {
                display: flex;
                flex-direction: column;
                gap: 12px;
            }
            .quick-bid-label {
                font-size: 12px;
                color: var(--text-muted);
                text-transform: uppercase;
                letter-spacing: 0.5px;
            }
            .quick-bid-buttons {
                display: flex;
                gap: 8px;
            }
            .quick-bid-btn {
                flex: 1;
                padding: 12px;
                font-size: 14px;
                font-weight: 500;
                background: var(--bg);
                border: 1px solid var(--border);
                border-radius: var(--radius-md);
                cursor: pointer;
                transition: all 0.15s;
            }
            .quick-bid-btn:hover {
                background: var(--blue-light);
                border-color: var(--blue);
                color: var(--blue);
            }
            .quick-bid-btn:disabled {
                opacity: 0.5;
                cursor: not-allowed;
            }
            .user-status {
                display: flex;
                align-items: center;
                justify-content: center;
                gap: 8px;
                padding: 16px;
                background: var(--bg);
                border-radius: var(--radius-md);
                margin-top: 24px;
            }
            .user-status.leading {
                background: var(--green-light);
                color: var(--green);
            }
            .user-status.outbid {
                background: var(--orange-light);
                color: var(--orange);
            }
            .user-status svg {
                width: 20px;
                height: 20px;
            }
            .sidebar {
                display: flex;
                flex-direction: column;
                gap: 24px;
            }
            .bid-history-list {
                display: flex;
                flex-direction: column;
                gap: 0;
                max-height: 400px;
                overflow-y: auto;
            }
            .bid-history-item {
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding: 12px 0;
                border-bottom: 1px solid var(--border-light);
            }
            .bid-history-item:last-child {
                border-bottom: none;
            }
            .bid-history-item.leading {
                background: var(--green-light);
                margin: 0 -16px;
                padding: 12px 16px;
                border-radius: var(--radius-sm);
            }
            .bid-history-info {
                display: flex;
                flex-direction: column;
                gap: 2px;
            }
            .bid-history-bidder {
                font-size: 13px;
                font-weight: 500;
                color: var(--navy);
            }
            .bid-history-time {
                font-size: 11px;
                color: var(--text-muted);
            }
            .bid-history-amount {
                text-align: right;
            }
            .bid-history-price {
                font-family: IBM Plex Mono, monospace;
                font-size: 14px;
                font-weight: 600;
                color: var(--navy);
            }
            .bid-history-delta {
                font-size: 11px;
                color: var(--green);
            }
            .item-details {
                display: flex;
                flex-direction: column;
                gap: 12px;
            }
            .item-detail-row {
                display: flex;
                justify-content: space-between;
                padding: 8px 0;
                border-bottom: 1px solid var(--border-light);
            }
            .item-detail-row:last-child {
                border-bottom: none;
            }
            .item-detail-label {
                font-size: 13px;
                color: var(--text-muted);
            }
            .item-detail-value {
                font-size: 13px;
                font-weight: 500;
                color: var(--navy);
            }
            .connection-status {
                display: flex;
                align-items: center;
                gap: 8px;
                padding: 8px 12px;
                border-radius: var(--radius-sm);
                font-size: 12px;
            }
            .connection-status.connected {
                background: var(--green-light);
                color: var(--green);
            }
            .connection-status.disconnected {
                background: var(--red-light);
                color: var(--red);
            }
            .connection-status svg {
                width: 14px;
                height: 14px;
            }
            .bidders-list {
                display: flex;
                flex-direction: column;
                gap: 8px;
            }
            .bidder-card {
                display: flex;
                align-items: center;
                gap: 12px;
                padding: 12px;
                background: var(--bg);
                border-radius: var(--radius-md);
            }
            .bidder-card.active {
                background: var(--blue-light);
            }
            .bidder-avatar {
                width: 36px;
                height: 36px;
                border-radius: 50%;
                background: var(--blue);
                color: white;
                display: flex;
                align-items: center;
                justify-content: center;
                font-size: 14px;
                font-weight: 600;
            }
            .bidder-info {
                flex: 1;
            }
            .bidder-name {
                font-size: 13px;
                font-weight: 500;
                color: var(--navy);
            }
            .bidder-last-bid {
                font-size: 11px;
                color: var(--text-muted);
            }
            .bidder-status {
                width: 8px;
                height: 8px;
                border-radius: 50%;
                background: var(--green);
            }
            .bidder-status.inactive {
                background: var(--text-muted);
            }
            .price-progress {
                padding: 16px;
                background: var(--bg);
                border-radius: var(--radius-md);
            }
            .price-labels {
                display: flex;
                justify-content: space-between;
                margin-bottom: 8px;
                font-size: 11px;
                color: var(--text-muted);
            }
            .savings-badge {
                display: inline-flex;
                align-items: center;
                gap: 4px;
                padding: 4px 8px;
                background: var(--green-light);
                color: var(--green);
                font-size: 12px;
                font-weight: 600;
                border-radius: var(--radius-sm);
            }
            .no-auction {
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                padding: 80px 20px;
                text-align: center;
                color: var(--text-muted);
            }
            .no-auction svg {
                width: 64px;
                height: 64px;
                margin-bottom: 24px;
                opacity: 0.5;
            }
            .no-auction h2 {
                font-size: 20px;
                font-weight: 500;
                color: var(--navy);
                margin-bottom: 8px;
            }
            .no-auction p {
                font-size: 14px;
                margin-bottom: 24px;
            }
            "#
        }

        <div class="auction-live" data-testid="auction-live">
            // Back link
            <a class="back-link" on:click={handle_back}>
                <span inner_html={icon_back}></span>
                "Back to Auction List"
            </a>

            if let Some(auction) = selected.get().as_ref() {
                // Live auction header
                <div class={if auction.status == AuctionStatus::Live { "live-header is-live" } else { "live-header" }}>
                    <div class="auction-info">
                        <h1>
                            {auction.title.clone()}
                            if auction.status == AuctionStatus::Live {
                                <span class="live-badge">
                                    <span class="dot"></span>
                                    "LIVE"
                                </span>
                            }
                        </h1>
                        <div class="auction-meta">
                            <span class="auction-meta-item">
                                <strong>"Ref:"</strong> {auction.reference_number.clone()}
                            </span>
                            <span class="auction-meta-item">
                                <span inner_html={icon_users}></span>
                                {format!("{} bidders", active_bidders.get())}
                            </span>
                            <span class="auction-meta-item">
                                {connection_status(ws_connected.get(), icon_wifi, icon_wifi_off)}
                            </span>
                        </div>
                    </div>
                    <div class="countdown-display">
                        <div class="countdown-label">"Time Remaining"</div>
                        <div class={countdown_class(&countdown.get())}>
                            {countdown.get().display()}
                        </div>
                    </div>
                </div>

                // Auto-extend notice
                if auction.auto_extend {
                    {notice_bar(
                        NoticeType::Info,
                        format!("Auto-extend: Auction extends by {} minutes if bid placed in last {} seconds",
                            auction.extension_minutes,
                            auction.extension_threshold_seconds
                        ),
                        None
                    )}
                }

                // Main content layout
                <div class="live-layout">
                    <div class="main-content">
                        // Bidding panel
                        <div class={if is_leading.get() { "bidding-panel is-leading" } else { "bidding-panel" }}>
                            <div class="current-bid-section">
                                <div class="current-bid-label">"Current Lowest Bid"</div>
                                <div class="current-bid-amount">
                                    {format_currency_full(auction.current_bid.unwrap_or(auction.starting_price))}
                                </div>
                                <div class="current-bid-info">
                                    <span>"Starting: " {format_currency(auction.starting_price)}</span>
                                    <span>"Min Decrement: " {format_currency(auction.min_decrement)}</span>
                                    {savings_badge(auction.starting_price, auction.current_bid)}
                                </div>
                            </div>

                            // Bid input
                            if auction.status == AuctionStatus::Live {
                                <div class="bid-input-section">
                                    <div class="bid-input-row">
                                        <div class="bid-input-wrapper">
                                            <span class="currency-prefix">"R"</span>
                                            <input
                                                type="text"
                                                class={if bid_error.get().is_some() { "bid-input error" } else { "bid-input" }}
                                                placeholder="Enter your bid"
                                                value={bid_amount.get()}
                                                on:input={handle_bid_input}
                                                disabled={bid_submitting.get()}
                                            />
                                        </div>
                                        <button
                                            class="btn btn-primary place-bid-btn"
                                            on:click={handle_place_bid}
                                            disabled={bid_submitting.get() || bid_amount.get().is_empty()}
                                        >
                                            if bid_submitting.get() {
                                                "Placing Bid..."
                                            } else {
                                                "Place Bid"
                                            }
                                        </button>
                                    </div>
                                    if let Some(error) = bid_error.get().as_ref() {
                                        <div class="bid-error">{error.clone()}</div>
                                    }
                                </div>

                                // Quick bid buttons
                                <div class="quick-bid-section">
                                    <div class="quick-bid-label">"Quick Bid"</div>
                                    <div class="quick-bid-buttons">
                                        <button
                                            class="quick-bid-btn"
                                            on:click={handle_quick_bid(auction.min_decrement)}
                                            disabled={bid_submitting.get()}
                                        >
                                            {format!("-R {}", format_number(auction.min_decrement as i64))}
                                        </button>
                                        <button
                                            class="quick-bid-btn"
                                            on:click={handle_quick_bid(auction.min_decrement * 2.0)}
                                            disabled={bid_submitting.get()}
                                        >
                                            {format!("-R {}", format_number((auction.min_decrement * 2.0) as i64))}
                                        </button>
                                        <button
                                            class="quick-bid-btn"
                                            on:click={handle_quick_bid(auction.min_decrement * 5.0)}
                                            disabled={bid_submitting.get()}
                                        >
                                            {format!("-R {}", format_number((auction.min_decrement * 5.0) as i64))}
                                        </button>
                                        <button
                                            class="quick-bid-btn"
                                            on:click={handle_quick_bid(auction.min_decrement * 10.0)}
                                            disabled={bid_submitting.get()}
                                        >
                                            {format!("-R {}", format_number((auction.min_decrement * 10.0) as i64))}
                                        </button>
                                    </div>
                                </div>

                                // User status
                                {user_status_display(is_leading.get(), user_rank.get(), current_user_bid.get(), icon_check, icon_alert)}
                            } else {
                                <div class="user-status">
                                    <span inner_html={icon_clock}></span>
                                    "Auction is not currently live"
                                </div>
                            }
                        </div>

                        // Price progress
                        {price_progress_panel(auction.starting_price, auction.reserve_price, auction.current_bid)}

                        // Item details
                        {panel(
                            "Item Details".to_string(),
                            vec![],
                            vec![view! {
                                <div class="item-details">
                                    <div class="item-detail-row">
                                        <span class="item-detail-label">"Item"</span>
                                        <span class="item-detail-value">{auction.item.name.clone()}</span>
                                    </div>
                                    <div class="item-detail-row">
                                        <span class="item-detail-label">"Quantity"</span>
                                        <span class="item-detail-value">
                                            {format!("{} {}", auction.item.quantity, auction.item.unit)}
                                        </span>
                                    </div>
                                    <div class="item-detail-row">
                                        <span class="item-detail-label">"Category"</span>
                                        <span class="item-detail-value">{auction.item.category.clone()}</span>
                                    </div>
                                    <div class="item-detail-row">
                                        <span class="item-detail-label">"Specifications"</span>
                                        <span class="item-detail-value">{auction.item.specifications.clone()}</span>
                                    </div>
                                    <div class="item-detail-row">
                                        <span class="item-detail-label">"Department"</span>
                                        <span class="item-detail-value">{auction.department.clone()}</span>
                                    </div>
                                </div>
                            }]
                        )}
                    </div>

                    // Sidebar
                    <div class="sidebar">
                        // Bid history
                        {panel(
                            format!("Bid History ({})", bid_history.get().len()),
                            vec![],
                            vec![view! {
                                <div class="bid-history-list">
                                    if bid_history.get().is_empty() {
                                        <div style="text-align: center; padding: 24px; color: var(--text-muted);">
                                            "No bids yet"
                                        </div>
                                    } else {
                                        for entry in bid_history.get().iter() {
                                            {bid_history_item(entry.clone(), auction.anonymous_bidding)}
                                        }
                                    }
                                </div>
                            }]
                        )}

                        // Active bidders
                        {panel(
                            format!("Active Bidders ({})", auction.bidders.iter().filter(|b| b.is_active).count()),
                            vec![],
                            vec![view! {
                                <div class="bidders-list">
                                    for bidder in auction.bidders.iter() {
                                        <div class={if bidder.is_active { "bidder-card active" } else { "bidder-card" }}>
                                            <div class="bidder-avatar">
                                                {bidder.supplier_name.chars().next().unwrap_or('?')}
                                            </div>
                                            <div class="bidder-info">
                                                <div class="bidder-name">
                                                    if auction.anonymous_bidding {
                                                        {format!("Bidder {}", bidder.id.chars().last().unwrap_or('?'))}
                                                    } else {
                                                        {bidder.supplier_name.clone()}
                                                    }
                                                </div>
                                                <div class="bidder-last-bid">
                                                    if let Some(ref last_bid) = bidder.last_bid_at {
                                                        {format!("Last bid: {}", format_time_short(last_bid))}
                                                    } else {
                                                        "No bids yet"
                                                    }
                                                </div>
                                            </div>
                                            <div class={if bidder.is_active { "bidder-status" } else { "bidder-status inactive" }}></div>
                                        </div>
                                    }
                                </div>
                            }]
                        )}
                    </div>
                </div>
            } else {
                // No auction selected
                <div class="no-auction">
                    <span inner_html={icon_clock}></span>
                    <h2>"No Live Auction"</h2>
                    <p>"There is no live auction currently available."</p>
                    <a href="#/reverse-auctions" class="btn btn-primary">"View All Auctions"</a>
                </div>
            }
        </div>
    }
}

/// Connection status indicator
fn connection_status(connected: bool, icon_on: &str, icon_off: &str) -> View {
    if connected {
        view! {
            <div class="connection-status connected">
                <span inner_html={icon_on}></span>
                "Connected"
            </div>
        }
    } else {
        view! {
            <div class="connection-status disconnected">
                <span inner_html={icon_off}></span>
                "Disconnected"
            </div>
        }
    }
}

/// Countdown time class based on state
fn countdown_class(state: &super::types::CountdownState) -> &'static str {
    if state.is_ended {
        "countdown-time ended"
    } else if state.is_ending_soon {
        "countdown-time ending-soon"
    } else {
        "countdown-time"
    }
}

/// Savings badge showing percentage saved
fn savings_badge(starting: f64, current: Option<f64>) -> View {
    if let Some(current_bid) = current {
        let savings = starting - current_bid;
        let percentage = (savings / starting) * 100.0;
        if savings > 0.0 {
            return view! {
                <span class="savings-badge">
                    {format!("{:.1}% saved", percentage)}
                </span>
            };
        }
    }
    view! { <span></span> }
}

/// User status display
fn user_status_display(
    is_leading: bool,
    rank: Option<u32>,
    current_bid: Option<f64>,
    icon_check: &str,
    icon_alert: &str,
) -> View {
    if is_leading {
        view! {
            <div class="user-status leading">
                <span inner_html={icon_check}></span>
                "You are the leading bidder!"
            </div>
        }
    } else if let Some(bid) = current_bid {
        view! {
            <div class="user-status outbid">
                <span inner_html={icon_alert}></span>
                {format!("You have been outbid. Your last bid: R {:.2}", bid)}
            </div>
        }
    } else {
        view! {
            <div class="user-status">
                "Place your first bid to participate"
            </div>
        }
    }
}

/// Bid history item component
fn bid_history_item(entry: BidHistoryEntry, anonymous: bool) -> View {
    let bidder_display = if anonymous {
        entry.bidder_masked.clone()
    } else {
        entry.bidder_name.clone()
    };

    view! {
        <div class={if entry.is_leading { "bid-history-item leading" } else { "bid-history-item" }}>
            <div class="bid-history-info">
                <span class="bid-history-bidder">{bidder_display}</span>
                <span class="bid-history-time">{format_time_short(&entry.timestamp)}</span>
            </div>
            <div class="bid-history-amount">
                <div class="bid-history-price">{format_currency(entry.amount)}</div>
                if let Some(delta) = entry.delta {
                    <div class="bid-history-delta">{format!("-R {}", format_number(delta as i64))}</div>
                }
            </div>
        </div>
    }
}

/// Price progress panel
fn price_progress_panel(starting: f64, reserve: Option<f64>, current: Option<f64>) -> View {
    let current_bid = current.unwrap_or(starting);
    let progress = if starting > 0.0 {
        ((starting - current_bid) / starting * 100.0).min(100.0).max(0.0)
    } else {
        0.0
    };

    let reserve_progress = reserve.map(|r| {
        ((starting - r) / starting * 100.0).min(100.0).max(0.0)
    });

    view! {
        <div class="price-progress">
            <div class="price-labels">
                <span>{format!("Starting: {}", format_currency(starting))}</span>
                if let Some(r) = reserve {
                    <span>{format!("Reserve: {}", format_currency(r))}</span>
                }
                <span>{format!("Current: {}", format_currency(current_bid))}</span>
            </div>
            {progress_bar(progress as u32, 100, ProgressColor::Green)}
            if let Some(rp) = reserve_progress {
                <div style="margin-top: 4px; font-size: 11px; color: var(--text-muted);">
                    {format!("Reserve threshold at {:.0}%", rp)}
                </div>
            }
        </div>
    }
}

/// Format time for short display (HH:MM:SS)
fn format_time_short(iso_datetime: &str) -> String {
    if iso_datetime.len() >= 19 {
        iso_datetime[11..19].to_string()
    } else if iso_datetime.len() >= 16 {
        format!("{}:00", &iso_datetime[11..16])
    } else {
        iso_datetime.to_string()
    }
}
