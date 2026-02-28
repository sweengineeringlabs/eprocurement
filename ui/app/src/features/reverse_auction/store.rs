//! Reverse Auction store

use components::prelude::*;
use super::types::{
    ReverseAuction, AuctionStatus, AuctionFilter, PaginationState,
    AuctionItem, Bidder, AuctionBid, BidHistoryEntry, CountdownState,
};

/// Reverse Auction state store
#[derive(Clone)]
pub struct ReverseAuctionStore {
    // List view state
    pub auctions: Signal<Vec<ReverseAuction>>,
    pub filter: Signal<AuctionFilter>,
    pub pagination: Signal<PaginationState>,

    // Detail/Live view state
    pub selected: Signal<Option<ReverseAuction>>,
    pub bid_history: Signal<Vec<BidHistoryEntry>>,
    pub countdown: Signal<CountdownState>,
    pub active_bidders: Signal<u32>,

    // Bidding state
    pub current_user_bid: Signal<Option<f64>>,
    pub bid_input: Signal<String>,
    pub is_leading: Signal<bool>,
    pub user_rank: Signal<Option<u32>>,

    // WebSocket state
    pub ws_connected: Signal<bool>,
    pub ws_reconnecting: Signal<bool>,

    // UI state
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
    pub bid_submitting: Signal<bool>,
    pub bid_error: Signal<Option<String>>,
}

impl ReverseAuctionStore {
    pub fn new() -> Self {
        Self {
            auctions: signal(Vec::new()),
            filter: signal(AuctionFilter::default()),
            pagination: signal(PaginationState::default()),
            selected: signal(None),
            bid_history: signal(Vec::new()),
            countdown: signal(CountdownState::default()),
            active_bidders: signal(0),
            current_user_bid: signal(None),
            bid_input: signal(String::new()),
            is_leading: signal(false),
            user_rank: signal(None),
            ws_connected: signal(false),
            ws_reconnecting: signal(false),
            loading: signal(false),
            error: signal(None),
            bid_submitting: signal(false),
            bid_error: signal(None),
        }
    }

    /// Update countdown from remaining seconds
    pub fn update_countdown(&self, total_seconds: i64) {
        self.countdown.set(CountdownState::from_seconds(total_seconds));
    }

    /// Update bid input value
    pub fn set_bid_input(&self, value: String) {
        self.bid_input.set(value);
        self.bid_error.set(None);
    }

    /// Validate bid amount
    pub fn validate_bid(&self, amount: f64) -> Result<(), String> {
        let auction = self.selected.get();
        if auction.is_none() {
            return Err("No auction selected".to_string());
        }
        let auction = auction.as_ref().unwrap();

        if auction.status != AuctionStatus::Live {
            return Err("Auction is not currently live".to_string());
        }

        let current_bid = auction.current_bid.unwrap_or(auction.starting_price);
        let max_allowed = current_bid - auction.min_decrement;

        if amount >= current_bid {
            return Err(format!(
                "Bid must be lower than current bid of R {:.2}",
                current_bid
            ));
        }

        if amount > max_allowed {
            return Err(format!(
                "Bid must be at least R {:.2} lower than current bid",
                auction.min_decrement
            ));
        }

        if let Some(reserve) = auction.reserve_price {
            if amount < reserve {
                // Allow bid but warn
            }
        }

        Ok(())
    }

    /// Clear bid error
    pub fn clear_bid_error(&self) {
        self.bid_error.set(None);
    }

    /// Handle new bid from WebSocket
    pub fn handle_new_bid(&self, bid: AuctionBid) {
        // Update current bid on selected auction
        if let Some(mut auction) = self.selected.get() {
            if auction.id == bid.auction_id {
                let previous_bid = auction.current_bid;
                auction.current_bid = Some(bid.amount);
                auction.total_bids += 1;
                self.selected.set(Some(auction));

                // Add to bid history
                let mut history = self.bid_history.get();
                let entry = BidHistoryEntry {
                    id: bid.id.clone(),
                    bidder_name: bid.bidder_name.clone(),
                    bidder_masked: format!("Bidder {}", (history.len() % 26) as u8 + b'A'),
                    amount: bid.amount,
                    timestamp: bid.timestamp.clone(),
                    delta: previous_bid.map(|p| p - bid.amount),
                    is_leading: true,
                };

                // Mark previous leading bid as not leading
                for h in history.iter_mut() {
                    h.is_leading = false;
                }

                history.insert(0, entry);
                self.bid_history.set(history);

                // Update user's leading status
                // In production, compare with current user's bidder ID
                self.is_leading.set(false);
            }
        }
    }

    /// Handle auction time extension
    pub fn handle_time_extended(&self, new_end_time: String) {
        if let Some(mut auction) = self.selected.get() {
            auction.extended_end_time = Some(new_end_time);
            self.selected.set(Some(auction));
        }
    }

    /// Handle auction ended
    pub fn handle_auction_ended(&self, _winning_bid: Option<AuctionBid>) {
        if let Some(mut auction) = self.selected.get() {
            auction.status = AuctionStatus::Ended;
            self.selected.set(Some(auction));
        }
        self.countdown.set(CountdownState {
            hours: 0,
            minutes: 0,
            seconds: 0,
            total_seconds: 0,
            is_ending_soon: false,
            is_ended: true,
        });
    }

    /// Update active bidder count
    pub fn update_active_bidders(&self, count: u32) {
        self.active_bidders.set(count);
    }
}

/// Load mock data for demo
pub fn load_mock_data(store: &ReverseAuctionStore) {
    let mock_auctions = vec![
        ReverseAuction {
            id: "AUC-2025-0001".to_string(),
            reference_number: "RA-2025-0001".to_string(),
            title: "Server Hardware Procurement - Dell PowerEdge R750".to_string(),
            description: "Reverse auction for the procurement of 20 Dell PowerEdge R750 servers for data centre expansion.".to_string(),
            status: AuctionStatus::Live,
            item: AuctionItem {
                id: "ITEM-001".to_string(),
                name: "Dell PowerEdge R750 Server".to_string(),
                description: "Enterprise-grade rack server with Intel Xeon processors".to_string(),
                quantity: 20,
                unit: "units".to_string(),
                specifications: "Intel Xeon Gold 6330, 128GB RAM, 4x 1.92TB SSD".to_string(),
                category: "IT Hardware".to_string(),
            },
            start_time: "2025-02-27T09:00:00Z".to_string(),
            end_time: "2025-02-27T15:00:00Z".to_string(),
            extended_end_time: None,
            created_at: "2025-02-20".to_string(),
            starting_price: 2_500_000.0,
            reserve_price: Some(1_800_000.0),
            current_bid: Some(2_150_000.0),
            min_decrement: 5_000.0,
            currency: "ZAR".to_string(),
            bidders: vec![
                Bidder {
                    id: "BDR-001".to_string(),
                    supplier_id: "SUP-001".to_string(),
                    supplier_name: "TechSolutions SA (Pty) Ltd".to_string(),
                    bbbee_level: Some(1),
                    is_qualified: true,
                    qualification_date: Some("2025-02-22".to_string()),
                    is_active: true,
                    last_bid_at: Some("2025-02-27T10:45:00Z".to_string()),
                },
                Bidder {
                    id: "BDR-002".to_string(),
                    supplier_id: "SUP-002".to_string(),
                    supplier_name: "DataCore Systems".to_string(),
                    bbbee_level: Some(2),
                    is_qualified: true,
                    qualification_date: Some("2025-02-22".to_string()),
                    is_active: true,
                    last_bid_at: Some("2025-02-27T10:30:00Z".to_string()),
                },
                Bidder {
                    id: "BDR-003".to_string(),
                    supplier_id: "SUP-003".to_string(),
                    supplier_name: "Enterprise IT Solutions".to_string(),
                    bbbee_level: Some(3),
                    is_qualified: true,
                    qualification_date: Some("2025-02-23".to_string()),
                    is_active: false,
                    last_bid_at: Some("2025-02-27T09:15:00Z".to_string()),
                },
            ],
            total_bids: 12,
            auto_extend: true,
            extension_minutes: 5,
            extension_threshold_seconds: 120,
            anonymous_bidding: true,
            show_rank_only: true,
            tender_id: Some("TND-2025-0089".to_string()),
            tender_reference: Some("RFP-2025-0089".to_string()),
            department: "Corporate Services".to_string(),
            cost_center: "CC-IT-001".to_string(),
            created_by: "John Smith".to_string(),
            last_modified_by: "John Smith".to_string(),
            last_modified_at: "2025-02-27".to_string(),
        },
        ReverseAuction {
            id: "AUC-2025-0002".to_string(),
            reference_number: "RA-2025-0002".to_string(),
            title: "Office Furniture - Ergonomic Desks and Chairs".to_string(),
            description: "Procurement of ergonomic office furniture for new wing.".to_string(),
            status: AuctionStatus::Scheduled,
            item: AuctionItem {
                id: "ITEM-002".to_string(),
                name: "Ergonomic Office Furniture Set".to_string(),
                description: "Height-adjustable desks and ergonomic chairs".to_string(),
                quantity: 150,
                unit: "sets".to_string(),
                specifications: "Electric sit-stand desks, mesh ergonomic chairs".to_string(),
                category: "Furniture".to_string(),
            },
            start_time: "2025-03-01T10:00:00Z".to_string(),
            end_time: "2025-03-01T14:00:00Z".to_string(),
            extended_end_time: None,
            created_at: "2025-02-15".to_string(),
            starting_price: 750_000.0,
            reserve_price: Some(550_000.0),
            current_bid: None,
            min_decrement: 2_500.0,
            currency: "ZAR".to_string(),
            bidders: vec![
                Bidder {
                    id: "BDR-004".to_string(),
                    supplier_id: "SUP-004".to_string(),
                    supplier_name: "Office Pro Suppliers".to_string(),
                    bbbee_level: Some(2),
                    is_qualified: true,
                    qualification_date: Some("2025-02-25".to_string()),
                    is_active: false,
                    last_bid_at: None,
                },
                Bidder {
                    id: "BDR-005".to_string(),
                    supplier_id: "SUP-005".to_string(),
                    supplier_name: "Workspace Solutions".to_string(),
                    bbbee_level: Some(1),
                    is_qualified: true,
                    qualification_date: Some("2025-02-26".to_string()),
                    is_active: false,
                    last_bid_at: None,
                },
            ],
            total_bids: 0,
            auto_extend: true,
            extension_minutes: 3,
            extension_threshold_seconds: 60,
            anonymous_bidding: true,
            show_rank_only: true,
            tender_id: None,
            tender_reference: None,
            department: "Facilities Management".to_string(),
            cost_center: "CC-FAC-002".to_string(),
            created_by: "Jane Doe".to_string(),
            last_modified_by: "Jane Doe".to_string(),
            last_modified_at: "2025-02-25".to_string(),
        },
        ReverseAuction {
            id: "AUC-2025-0003".to_string(),
            reference_number: "RA-2025-0003".to_string(),
            title: "Fleet Vehicle Maintenance Services".to_string(),
            description: "Annual contract for fleet vehicle maintenance.".to_string(),
            status: AuctionStatus::Ended,
            item: AuctionItem {
                id: "ITEM-003".to_string(),
                name: "Fleet Maintenance Service Contract".to_string(),
                description: "12-month maintenance contract for 50 vehicles".to_string(),
                quantity: 1,
                unit: "contract".to_string(),
                specifications: "Includes scheduled maintenance, repairs, and roadside assistance".to_string(),
                category: "Services".to_string(),
            },
            start_time: "2025-02-20T09:00:00Z".to_string(),
            end_time: "2025-02-20T13:00:00Z".to_string(),
            extended_end_time: Some("2025-02-20T13:15:00Z".to_string()),
            created_at: "2025-02-10".to_string(),
            starting_price: 1_200_000.0,
            reserve_price: Some(900_000.0),
            current_bid: Some(950_000.0),
            min_decrement: 5_000.0,
            currency: "ZAR".to_string(),
            bidders: vec![
                Bidder {
                    id: "BDR-006".to_string(),
                    supplier_id: "SUP-006".to_string(),
                    supplier_name: "AutoCare Fleet Services".to_string(),
                    bbbee_level: Some(2),
                    is_qualified: true,
                    qualification_date: Some("2025-02-15".to_string()),
                    is_active: false,
                    last_bid_at: Some("2025-02-20T13:12:00Z".to_string()),
                },
            ],
            total_bids: 18,
            auto_extend: true,
            extension_minutes: 5,
            extension_threshold_seconds: 120,
            anonymous_bidding: true,
            show_rank_only: true,
            tender_id: None,
            tender_reference: None,
            department: "Operations".to_string(),
            cost_center: "CC-OPS-003".to_string(),
            created_by: "Mike Brown".to_string(),
            last_modified_by: "Mike Brown".to_string(),
            last_modified_at: "2025-02-20".to_string(),
        },
        ReverseAuction {
            id: "AUC-2025-0004".to_string(),
            reference_number: "RA-2025-0004".to_string(),
            title: "Network Equipment - Cisco Switches".to_string(),
            description: "Procurement of network switches for campus upgrade.".to_string(),
            status: AuctionStatus::Awarded,
            item: AuctionItem {
                id: "ITEM-004".to_string(),
                name: "Cisco Catalyst 9300 Switches".to_string(),
                description: "48-port PoE+ network switches".to_string(),
                quantity: 30,
                unit: "units".to_string(),
                specifications: "Cisco Catalyst 9300-48P-E with DNA license".to_string(),
                category: "IT Hardware".to_string(),
            },
            start_time: "2025-02-15T09:00:00Z".to_string(),
            end_time: "2025-02-15T12:00:00Z".to_string(),
            extended_end_time: None,
            created_at: "2025-02-05".to_string(),
            starting_price: 1_800_000.0,
            reserve_price: Some(1_400_000.0),
            current_bid: Some(1_425_000.0),
            min_decrement: 5_000.0,
            currency: "ZAR".to_string(),
            bidders: vec![],
            total_bids: 15,
            auto_extend: true,
            extension_minutes: 5,
            extension_threshold_seconds: 120,
            anonymous_bidding: true,
            show_rank_only: true,
            tender_id: Some("TND-2025-0088".to_string()),
            tender_reference: Some("RFQ-2025-0088".to_string()),
            department: "Corporate Services".to_string(),
            cost_center: "CC-IT-001".to_string(),
            created_by: "John Smith".to_string(),
            last_modified_by: "John Smith".to_string(),
            last_modified_at: "2025-02-15".to_string(),
        },
        ReverseAuction {
            id: "AUC-2025-0005".to_string(),
            reference_number: "RA-2025-0005".to_string(),
            title: "Security Services - CCTV Installation".to_string(),
            description: "Installation of CCTV system at new premises.".to_string(),
            status: AuctionStatus::Draft,
            item: AuctionItem {
                id: "ITEM-005".to_string(),
                name: "CCTV Installation Project".to_string(),
                description: "Supply and installation of 100-camera CCTV system".to_string(),
                quantity: 1,
                unit: "project".to_string(),
                specifications: "IP cameras, NVR, monitoring station, cabling".to_string(),
                category: "Security".to_string(),
            },
            start_time: String::new(),
            end_time: String::new(),
            extended_end_time: None,
            created_at: "2025-02-25".to_string(),
            starting_price: 850_000.0,
            reserve_price: None,
            current_bid: None,
            min_decrement: 2_500.0,
            currency: "ZAR".to_string(),
            bidders: vec![],
            total_bids: 0,
            auto_extend: true,
            extension_minutes: 5,
            extension_threshold_seconds: 120,
            anonymous_bidding: true,
            show_rank_only: true,
            tender_id: None,
            tender_reference: None,
            department: "Corporate Services".to_string(),
            cost_center: "CC-SEC-001".to_string(),
            created_by: "Peter Jones".to_string(),
            last_modified_by: "Peter Jones".to_string(),
            last_modified_at: "2025-02-25".to_string(),
        },
    ];

    store.auctions.set(mock_auctions);
    store.pagination.set(PaginationState {
        current_page: 1,
        page_size: 10,
        total_items: 5,
        total_pages: 1,
    });
}

/// Load mock bid history for a live auction
pub fn load_mock_bid_history(store: &ReverseAuctionStore) {
    let history = vec![
        BidHistoryEntry {
            id: "BH-012".to_string(),
            bidder_name: "TechSolutions SA (Pty) Ltd".to_string(),
            bidder_masked: "Bidder A".to_string(),
            amount: 2_150_000.0,
            timestamp: "2025-02-27T10:45:23Z".to_string(),
            delta: Some(25_000.0),
            is_leading: true,
        },
        BidHistoryEntry {
            id: "BH-011".to_string(),
            bidder_name: "DataCore Systems".to_string(),
            bidder_masked: "Bidder B".to_string(),
            amount: 2_175_000.0,
            timestamp: "2025-02-27T10:42:15Z".to_string(),
            delta: Some(15_000.0),
            is_leading: false,
        },
        BidHistoryEntry {
            id: "BH-010".to_string(),
            bidder_name: "TechSolutions SA (Pty) Ltd".to_string(),
            bidder_masked: "Bidder A".to_string(),
            amount: 2_190_000.0,
            timestamp: "2025-02-27T10:38:45Z".to_string(),
            delta: Some(30_000.0),
            is_leading: false,
        },
        BidHistoryEntry {
            id: "BH-009".to_string(),
            bidder_name: "DataCore Systems".to_string(),
            bidder_masked: "Bidder B".to_string(),
            amount: 2_220_000.0,
            timestamp: "2025-02-27T10:35:12Z".to_string(),
            delta: Some(20_000.0),
            is_leading: false,
        },
        BidHistoryEntry {
            id: "BH-008".to_string(),
            bidder_name: "Enterprise IT Solutions".to_string(),
            bidder_masked: "Bidder C".to_string(),
            amount: 2_240_000.0,
            timestamp: "2025-02-27T10:30:00Z".to_string(),
            delta: Some(10_000.0),
            is_leading: false,
        },
        BidHistoryEntry {
            id: "BH-007".to_string(),
            bidder_name: "TechSolutions SA (Pty) Ltd".to_string(),
            bidder_masked: "Bidder A".to_string(),
            amount: 2_250_000.0,
            timestamp: "2025-02-27T10:25:30Z".to_string(),
            delta: Some(50_000.0),
            is_leading: false,
        },
        BidHistoryEntry {
            id: "BH-006".to_string(),
            bidder_name: "DataCore Systems".to_string(),
            bidder_masked: "Bidder B".to_string(),
            amount: 2_300_000.0,
            timestamp: "2025-02-27T10:20:15Z".to_string(),
            delta: Some(25_000.0),
            is_leading: false,
        },
        BidHistoryEntry {
            id: "BH-005".to_string(),
            bidder_name: "Enterprise IT Solutions".to_string(),
            bidder_masked: "Bidder C".to_string(),
            amount: 2_325_000.0,
            timestamp: "2025-02-27T10:15:00Z".to_string(),
            delta: Some(50_000.0),
            is_leading: false,
        },
    ];

    store.bid_history.set(history);
    store.active_bidders.set(3);
}

/// Select an auction
pub fn select_auction(store: &ReverseAuctionStore, auction_id: &str) {
    let auction = store.auctions.get().iter()
        .find(|a| a.id == auction_id)
        .cloned();
    store.selected.set(auction);
}

/// Clear selected auction
pub fn clear_selection(store: &ReverseAuctionStore) {
    store.selected.set(None);
    store.bid_history.set(Vec::new());
    store.countdown.set(CountdownState::default());
    store.active_bidders.set(0);
    store.current_user_bid.set(None);
    store.bid_input.set(String::new());
    store.is_leading.set(false);
    store.user_rank.set(None);
}
