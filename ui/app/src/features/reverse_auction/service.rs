//! Reverse Auction service - API calls and WebSocket handling

use super::store::{ReverseAuctionStore, load_mock_data, load_mock_bid_history, select_auction, clear_selection};
use super::types::{
    ReverseAuction, AuctionFilter, AuctionStatus, AuctionBid, WsMessage,
    PlaceBidRequest, PlaceBidResponse,
};

/// Load auctions list
pub async fn load_auctions(store: &ReverseAuctionStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API:
    // let response = api::get("/api/reverse-auctions", &store.filter.get()).await;
    // For now, load mock data
    load_mock_data(store);

    store.loading.set(false);
}

/// Load auctions with filter
pub async fn load_auctions_filtered(store: &ReverseAuctionStore, filter: AuctionFilter) {
    store.filter.set(filter);
    load_auctions(store).await;
}

/// Get single auction by ID
pub async fn get_auction(store: &ReverseAuctionStore, auction_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::get(&format!("/api/reverse-auctions/{}", auction_id)).await;
    // For now, find in mock data
    select_auction(store, auction_id);

    // Load bid history for live auctions
    if let Some(auction) = store.selected.get() {
        if auction.status == AuctionStatus::Live {
            load_mock_bid_history(store);
        }
    }

    store.loading.set(false);
}

/// Create new auction
pub async fn create_auction(store: &ReverseAuctionStore, auction: ReverseAuction) -> Result<ReverseAuction, String> {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::post("/api/reverse-auctions", &auction).await;

    // Generate a new ID for mock
    let mut new_auction = auction;
    let count = store.auctions.get().len() + 1;
    new_auction.id = format!("AUC-2025-{:04}", count);
    new_auction.reference_number = format!("RA-2025-{:04}", count);
    new_auction.status = AuctionStatus::Draft;
    new_auction.created_at = "2025-02-27".to_string();
    new_auction.last_modified_at = "2025-02-27".to_string();

    // Add to list
    let mut auctions = store.auctions.get();
    auctions.push(new_auction.clone());
    store.auctions.set(auctions);

    store.loading.set(false);
    Ok(new_auction)
}

/// Update existing auction
pub async fn update_auction(store: &ReverseAuctionStore, auction: ReverseAuction) -> Result<ReverseAuction, String> {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::put(&format!("/api/reverse-auctions/{}", auction.id), &auction).await;

    // Update in mock list
    let mut auctions = store.auctions.get();
    if let Some(pos) = auctions.iter().position(|a| a.id == auction.id) {
        auctions[pos] = auction.clone();
        store.auctions.set(auctions);
        store.selected.set(Some(auction.clone()));
        store.loading.set(false);
        Ok(auction)
    } else {
        store.loading.set(false);
        store.error.set(Some("Auction not found".to_string()));
        Err("Auction not found".to_string())
    }
}

/// Schedule auction (set start and end times)
pub async fn schedule_auction(
    store: &ReverseAuctionStore,
    auction_id: &str,
    start_time: &str,
    end_time: &str,
) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::post(&format!("/api/reverse-auctions/{}/schedule", auction_id), &ScheduleRequest {
    //     start_time,
    //     end_time,
    // }).await;

    let mut auctions = store.auctions.get();
    if let Some(pos) = auctions.iter().position(|a| a.id == auction_id) {
        auctions[pos].status = AuctionStatus::Scheduled;
        auctions[pos].start_time = start_time.to_string();
        auctions[pos].end_time = end_time.to_string();
        auctions[pos].last_modified_at = "2025-02-27".to_string();
        store.auctions.set(auctions.clone());
        store.selected.set(Some(auctions[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Auction not found".to_string()));
        Err("Auction not found".to_string())
    }
}

/// Start auction (transition to Live status)
pub async fn start_auction(store: &ReverseAuctionStore, auction_id: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut auctions = store.auctions.get();
    if let Some(pos) = auctions.iter().position(|a| a.id == auction_id) {
        if auctions[pos].status != AuctionStatus::Scheduled {
            store.loading.set(false);
            store.error.set(Some("Auction must be scheduled to start".to_string()));
            return Err("Auction must be scheduled to start".to_string());
        }

        auctions[pos].status = AuctionStatus::Live;
        auctions[pos].last_modified_at = "2025-02-27".to_string();
        store.auctions.set(auctions.clone());
        store.selected.set(Some(auctions[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Auction not found".to_string()));
        Err("Auction not found".to_string())
    }
}

/// End auction manually
pub async fn end_auction(store: &ReverseAuctionStore, auction_id: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut auctions = store.auctions.get();
    if let Some(pos) = auctions.iter().position(|a| a.id == auction_id) {
        auctions[pos].status = AuctionStatus::Ended;
        auctions[pos].last_modified_at = "2025-02-27".to_string();
        store.auctions.set(auctions.clone());
        store.selected.set(Some(auctions[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Auction not found".to_string()));
        Err("Auction not found".to_string())
    }
}

/// Award auction to winning bidder
pub async fn award_auction(
    store: &ReverseAuctionStore,
    auction_id: &str,
    winning_bidder_id: &str,
) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut auctions = store.auctions.get();
    if let Some(pos) = auctions.iter().position(|a| a.id == auction_id) {
        if auctions[pos].status != AuctionStatus::Ended {
            store.loading.set(false);
            store.error.set(Some("Auction must be ended before awarding".to_string()));
            return Err("Auction must be ended before awarding".to_string());
        }

        auctions[pos].status = AuctionStatus::Awarded;
        auctions[pos].last_modified_at = "2025-02-27".to_string();
        store.auctions.set(auctions.clone());
        store.selected.set(Some(auctions[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Auction not found".to_string()));
        Err("Auction not found".to_string())
    }
}

/// Cancel auction
pub async fn cancel_auction(store: &ReverseAuctionStore, auction_id: &str, reason: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut auctions = store.auctions.get();
    if let Some(pos) = auctions.iter().position(|a| a.id == auction_id) {
        auctions[pos].status = AuctionStatus::Cancelled;
        auctions[pos].last_modified_at = "2025-02-27".to_string();
        store.auctions.set(auctions.clone());
        store.selected.set(Some(auctions[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Auction not found".to_string()));
        Err("Auction not found".to_string())
    }
}

/// Delete auction (draft only)
pub async fn delete_auction(store: &ReverseAuctionStore, auction_id: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let auctions = store.auctions.get();
    if let Some(auction) = auctions.iter().find(|a| a.id == auction_id) {
        if auction.status != AuctionStatus::Draft {
            store.loading.set(false);
            store.error.set(Some("Only draft auctions can be deleted".to_string()));
            return Err("Only draft auctions can be deleted".to_string());
        }
    }

    let new_auctions: Vec<_> = auctions.into_iter()
        .filter(|a| a.id != auction_id)
        .collect();
    store.auctions.set(new_auctions);
    clear_selection(store);

    store.loading.set(false);
    Ok(())
}

/// Place a bid in live auction
pub async fn place_bid(store: &ReverseAuctionStore, request: PlaceBidRequest) -> Result<PlaceBidResponse, String> {
    store.bid_submitting.set(true);
    store.bid_error.set(None);

    // Validate bid
    if let Err(e) = store.validate_bid(request.amount) {
        store.bid_submitting.set(false);
        store.bid_error.set(Some(e.clone()));
        return Err(e);
    }

    // In production, this would send via WebSocket or API:
    // let response = ws::send(WsMessage::PlaceBid(request)).await;
    // or
    // let response = api::post(&format!("/api/reverse-auctions/{}/bids", request.auction_id), &request).await;

    // Simulate successful bid for demo
    let bid = AuctionBid {
        id: format!("BID-{:06}", rand_id()),
        auction_id: request.auction_id.clone(),
        bidder_id: "current-user".to_string(),
        bidder_name: "Current User".to_string(),
        amount: request.amount,
        timestamp: "2025-02-27T10:50:00Z".to_string(),
        is_leading: true,
        rank: 1,
    };

    // Update store with new bid
    store.handle_new_bid(bid.clone());
    store.current_user_bid.set(Some(request.amount));
    store.is_leading.set(true);
    store.user_rank.set(Some(1));
    store.bid_input.set(String::new());

    store.bid_submitting.set(false);

    Ok(PlaceBidResponse {
        success: true,
        bid: Some(bid),
        error: None,
        new_end_time: None,
    })
}

/// Connect to WebSocket for live auction updates
pub fn connect_websocket(store: &ReverseAuctionStore, auction_id: &str) {
    store.ws_connected.set(false);
    store.ws_reconnecting.set(true);

    // In production, this would establish a WebSocket connection:
    // let ws = WebSocket::new(&format!("wss://api.example.com/auctions/{}/ws", auction_id));
    //
    // ws.on_open(|| {
    //     store.ws_connected.set(true);
    //     store.ws_reconnecting.set(false);
    // });
    //
    // ws.on_message(|msg| {
    //     match msg {
    //         WsMessage::NewBid { bid, .. } => store.handle_new_bid(bid),
    //         WsMessage::TimeExtended { new_end_time, .. } => store.handle_time_extended(new_end_time),
    //         WsMessage::AuctionEnded { winning_bid, .. } => store.handle_auction_ended(winning_bid),
    //         WsMessage::BidderJoined { bidder_count, .. } => store.update_active_bidders(bidder_count),
    //         WsMessage::BidderLeft { bidder_count, .. } => store.update_active_bidders(bidder_count),
    //         _ => {}
    //     }
    // });
    //
    // ws.on_close(|| {
    //     store.ws_connected.set(false);
    //     // Attempt reconnection
    // });
    //
    // ws.on_error(|e| {
    //     store.error.set(Some(format!("WebSocket error: {}", e)));
    // });

    // For demo, simulate connected state
    store.ws_connected.set(true);
    store.ws_reconnecting.set(false);
}

/// Disconnect from WebSocket
pub fn disconnect_websocket(store: &ReverseAuctionStore) {
    // In production, close the WebSocket connection
    store.ws_connected.set(false);
    store.ws_reconnecting.set(false);
}

/// Send heartbeat/ping to keep connection alive
pub fn send_heartbeat(_store: &ReverseAuctionStore) {
    // In production:
    // ws::send(WsMessage::Ping);
}

/// Load bid history for an auction
pub async fn load_bid_history(store: &ReverseAuctionStore, auction_id: &str) {
    store.loading.set(true);

    // In production:
    // let response = api::get(&format!("/api/reverse-auctions/{}/bids", auction_id)).await;

    // Load mock data
    load_mock_bid_history(store);

    store.loading.set(false);
}

/// Helper function to generate random ID
fn rand_id() -> u32 {
    // In production, use proper random generation
    // For demo, return a fixed value
    123456
}

/// Calculate time remaining until auction end
pub fn calculate_time_remaining(end_time: &str) -> i64 {
    // In production, parse ISO timestamp and calculate difference
    // For demo, return mock value (4 hours 15 minutes 30 seconds)
    15330
}

/// Start countdown timer
pub fn start_countdown_timer(store: &ReverseAuctionStore) {
    // In production, this would use setInterval to update countdown every second
    //
    // let interval = setInterval(move |_| {
    //     if let Some(auction) = store.selected.get() {
    //         let end_time = auction.extended_end_time.as_ref().unwrap_or(&auction.end_time);
    //         let remaining = calculate_time_remaining(end_time);
    //         store.update_countdown(remaining);
    //
    //         if remaining <= 0 {
    //             clearInterval(interval);
    //         }
    //     }
    // }, 1000);

    // For demo, set initial countdown value
    store.update_countdown(15330); // 4h 15m 30s
}

/// Stop countdown timer
pub fn stop_countdown_timer() {
    // In production, clear the interval
}

/// Format remaining time for display
pub fn format_time_remaining(seconds: i64) -> String {
    if seconds <= 0 {
        return "Ended".to_string();
    }

    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{:02}:{:02}", minutes, secs)
    }
}
