//! Reverse Auction domain types

use serde::{Deserialize, Serialize};

/// Auction status enumeration
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum AuctionStatus {
    /// Initial draft state - auction being configured
    Draft,
    /// Scheduled for future start
    Scheduled,
    /// Currently accepting bids
    Live,
    /// Auction ended, awaiting finalization
    Ended,
    /// Winner selected, auction complete
    Awarded,
    /// Auction cancelled
    Cancelled,
}

impl AuctionStatus {
    pub fn label(&self) -> &'static str {
        match self {
            AuctionStatus::Draft => "Draft",
            AuctionStatus::Scheduled => "Scheduled",
            AuctionStatus::Live => "Live",
            AuctionStatus::Ended => "Ended",
            AuctionStatus::Awarded => "Awarded",
            AuctionStatus::Cancelled => "Cancelled",
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self, AuctionStatus::Live)
    }

    pub fn is_complete(&self) -> bool {
        matches!(self, AuctionStatus::Ended | AuctionStatus::Awarded | AuctionStatus::Cancelled)
    }
}

impl Default for AuctionStatus {
    fn default() -> Self {
        AuctionStatus::Draft
    }
}

/// Auction item being procured
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuctionItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub quantity: u32,
    pub unit: String,
    pub specifications: String,
    pub category: String,
}

impl Default for AuctionItem {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            description: String::new(),
            quantity: 1,
            unit: "unit".to_string(),
            specifications: String::new(),
            category: String::new(),
        }
    }
}

/// Bidder/supplier in the auction
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bidder {
    pub id: String,
    pub supplier_id: String,
    pub supplier_name: String,
    pub bbbee_level: Option<u8>,
    pub is_qualified: bool,
    pub qualification_date: Option<String>,
    pub is_active: bool,
    pub last_bid_at: Option<String>,
}

/// Individual bid in the auction
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuctionBid {
    pub id: String,
    pub auction_id: String,
    pub bidder_id: String,
    pub bidder_name: String,
    pub amount: f64,
    pub timestamp: String,
    pub is_leading: bool,
    pub rank: u32,
}

/// Bid history entry for timeline display
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BidHistoryEntry {
    pub id: String,
    pub bidder_name: String,
    pub bidder_masked: String, // e.g., "Bidder A" for anonymous display
    pub amount: f64,
    pub timestamp: String,
    pub delta: Option<f64>, // difference from previous bid
    pub is_leading: bool,
}

/// Main Reverse Auction entity
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReverseAuction {
    pub id: String,
    pub reference_number: String,
    pub title: String,
    pub description: String,
    pub status: AuctionStatus,

    // Item being auctioned
    pub item: AuctionItem,

    // Timing
    pub start_time: String,
    pub end_time: String,
    pub extended_end_time: Option<String>,
    pub created_at: String,

    // Pricing
    pub starting_price: f64,
    pub reserve_price: Option<f64>,
    pub current_bid: Option<f64>,
    pub min_decrement: f64,
    pub currency: String,

    // Participants
    pub bidders: Vec<Bidder>,
    pub total_bids: u32,

    // Configuration
    pub auto_extend: bool,
    pub extension_minutes: u32,
    pub extension_threshold_seconds: u32,
    pub anonymous_bidding: bool,
    pub show_rank_only: bool,

    // Related
    pub tender_id: Option<String>,
    pub tender_reference: Option<String>,
    pub department: String,
    pub cost_center: String,

    // Audit
    pub created_by: String,
    pub last_modified_by: String,
    pub last_modified_at: String,
}

impl Default for ReverseAuction {
    fn default() -> Self {
        Self {
            id: String::new(),
            reference_number: String::new(),
            title: String::new(),
            description: String::new(),
            status: AuctionStatus::default(),
            item: AuctionItem::default(),
            start_time: String::new(),
            end_time: String::new(),
            extended_end_time: None,
            created_at: String::new(),
            starting_price: 0.0,
            reserve_price: None,
            current_bid: None,
            min_decrement: 100.0,
            currency: "ZAR".to_string(),
            bidders: Vec::new(),
            total_bids: 0,
            auto_extend: true,
            extension_minutes: 5,
            extension_threshold_seconds: 120,
            anonymous_bidding: true,
            show_rank_only: true,
            tender_id: None,
            tender_reference: None,
            department: String::new(),
            cost_center: String::new(),
            created_by: String::new(),
            last_modified_by: String::new(),
            last_modified_at: String::new(),
        }
    }
}

/// Filter criteria for auction list
#[derive(Clone, Debug, Default)]
pub struct AuctionFilter {
    pub status: Option<AuctionStatus>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub search_query: Option<String>,
    pub department: Option<String>,
    pub category: Option<String>,
}

/// Pagination state
#[derive(Clone, Debug)]
pub struct PaginationState {
    pub current_page: u32,
    pub page_size: u32,
    pub total_items: u32,
    pub total_pages: u32,
}

impl Default for PaginationState {
    fn default() -> Self {
        Self {
            current_page: 1,
            page_size: 10,
            total_items: 0,
            total_pages: 0,
        }
    }
}

/// WebSocket message types for live auction
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    /// New bid placed
    NewBid {
        auction_id: String,
        bid: AuctionBid,
    },
    /// Auction time extended
    TimeExtended {
        auction_id: String,
        new_end_time: String,
    },
    /// Auction started
    AuctionStarted {
        auction_id: String,
    },
    /// Auction ended
    AuctionEnded {
        auction_id: String,
        winning_bid: Option<AuctionBid>,
    },
    /// Bidder joined
    BidderJoined {
        auction_id: String,
        bidder_count: u32,
    },
    /// Bidder left
    BidderLeft {
        auction_id: String,
        bidder_count: u32,
    },
    /// Heartbeat/ping
    Ping,
    /// Error message
    Error {
        message: String,
    },
}

/// Countdown timer state
#[derive(Clone, Debug, Default)]
pub struct CountdownState {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub total_seconds: i64,
    pub is_ending_soon: bool,
    pub is_ended: bool,
}

impl CountdownState {
    pub fn from_seconds(total: i64) -> Self {
        if total <= 0 {
            return Self {
                hours: 0,
                minutes: 0,
                seconds: 0,
                total_seconds: 0,
                is_ending_soon: false,
                is_ended: true,
            };
        }

        let hours = (total / 3600) as u32;
        let minutes = ((total % 3600) / 60) as u32;
        let seconds = (total % 60) as u32;

        Self {
            hours,
            minutes,
            seconds,
            total_seconds: total,
            is_ending_soon: total <= 120, // 2 minutes
            is_ended: false,
        }
    }

    pub fn display(&self) -> String {
        if self.is_ended {
            "Ended".to_string()
        } else if self.hours > 0 {
            format!("{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds)
        } else {
            format!("{:02}:{:02}", self.minutes, self.seconds)
        }
    }
}

/// Bid placement request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaceBidRequest {
    pub auction_id: String,
    pub amount: f64,
}

/// Bid placement response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaceBidResponse {
    pub success: bool,
    pub bid: Option<AuctionBid>,
    pub error: Option<String>,
    pub new_end_time: Option<String>,
}
