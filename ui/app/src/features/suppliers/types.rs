//! Supplier domain types

use serde::{Deserialize, Serialize};

/// B-BBEE certification levels
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum BbbeeLevel {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
    NonCompliant,
}

impl BbbeeLevel {
    pub fn label(&self) -> &'static str {
        match self {
            BbbeeLevel::Level1 => "Level 1",
            BbbeeLevel::Level2 => "Level 2",
            BbbeeLevel::Level3 => "Level 3",
            BbbeeLevel::Level4 => "Level 4",
            BbbeeLevel::Level5 => "Level 5",
            BbbeeLevel::Level6 => "Level 6",
            BbbeeLevel::Level7 => "Level 7",
            BbbeeLevel::Level8 => "Level 8",
            BbbeeLevel::NonCompliant => "Non-Compliant",
        }
    }

    pub fn from_u8(level: u8) -> Self {
        match level {
            1 => BbbeeLevel::Level1,
            2 => BbbeeLevel::Level2,
            3 => BbbeeLevel::Level3,
            4 => BbbeeLevel::Level4,
            5 => BbbeeLevel::Level5,
            6 => BbbeeLevel::Level6,
            7 => BbbeeLevel::Level7,
            8 => BbbeeLevel::Level8,
            _ => BbbeeLevel::NonCompliant,
        }
    }

    pub fn to_u8(&self) -> Option<u8> {
        match self {
            BbbeeLevel::Level1 => Some(1),
            BbbeeLevel::Level2 => Some(2),
            BbbeeLevel::Level3 => Some(3),
            BbbeeLevel::Level4 => Some(4),
            BbbeeLevel::Level5 => Some(5),
            BbbeeLevel::Level6 => Some(6),
            BbbeeLevel::Level7 => Some(7),
            BbbeeLevel::Level8 => Some(8),
            BbbeeLevel::NonCompliant => None,
        }
    }
}

impl Default for BbbeeLevel {
    fn default() -> Self {
        BbbeeLevel::NonCompliant
    }
}

/// Risk rating levels
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum RiskRating {
    Low,
    Medium,
    High,
    Critical,
}

impl RiskRating {
    pub fn label(&self) -> &'static str {
        match self {
            RiskRating::Low => "Low",
            RiskRating::Medium => "Medium",
            RiskRating::High => "High",
            RiskRating::Critical => "Critical",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            RiskRating::Low => "var(--green)",
            RiskRating::Medium => "var(--orange)",
            RiskRating::High => "var(--red)",
            RiskRating::Critical => "#8b0000",
        }
    }
}

impl Default for RiskRating {
    fn default() -> Self {
        RiskRating::Low
    }
}

/// Supplier registration status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum SupplierStatus {
    /// Pending verification
    Pending,
    /// Active and verified
    Active,
    /// Temporarily suspended
    Suspended,
    /// Blacklisted
    Blacklisted,
    /// Registration expired
    Expired,
    /// Inactive - no recent activity
    Inactive,
}

impl SupplierStatus {
    pub fn label(&self) -> &'static str {
        match self {
            SupplierStatus::Pending => "Pending",
            SupplierStatus::Active => "Active",
            SupplierStatus::Suspended => "Suspended",
            SupplierStatus::Blacklisted => "Blacklisted",
            SupplierStatus::Expired => "Expired",
            SupplierStatus::Inactive => "Inactive",
        }
    }
}

impl Default for SupplierStatus {
    fn default() -> Self {
        SupplierStatus::Pending
    }
}

/// Supplier category
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupplierCategory {
    pub code: String,
    pub name: String,
    pub description: String,
}

/// Risk indicator for supplier assessment
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RiskIndicator {
    pub id: String,
    pub category: String,
    pub indicator: String,
    pub severity: RiskRating,
    pub description: String,
    pub detected_at: String,
    pub status: RiskIndicatorStatus,
}

/// Risk indicator status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum RiskIndicatorStatus {
    Open,
    UnderReview,
    Mitigated,
    Accepted,
    Closed,
}

impl RiskIndicatorStatus {
    pub fn label(&self) -> &'static str {
        match self {
            RiskIndicatorStatus::Open => "Open",
            RiskIndicatorStatus::UnderReview => "Under Review",
            RiskIndicatorStatus::Mitigated => "Mitigated",
            RiskIndicatorStatus::Accepted => "Accepted",
            RiskIndicatorStatus::Closed => "Closed",
        }
    }
}

/// Performance score breakdown
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PerformanceScore {
    pub overall: f64,
    pub quality: f64,
    pub delivery: f64,
    pub price: f64,
    pub responsiveness: f64,
    pub compliance: f64,
}

impl Default for PerformanceScore {
    fn default() -> Self {
        Self {
            overall: 0.0,
            quality: 0.0,
            delivery: 0.0,
            price: 0.0,
            responsiveness: 0.0,
            compliance: 0.0,
        }
    }
}

/// Performance trend data point
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PerformanceTrend {
    pub period: String,
    pub score: f64,
}

/// Contract history with supplier
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupplierContract {
    pub id: String,
    pub reference: String,
    pub description: String,
    pub value: f64,
    pub start_date: String,
    pub end_date: String,
    pub status: String,
}

/// Main Supplier entity
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Supplier {
    pub id: String,
    pub name: String,
    pub trading_name: Option<String>,
    pub registration_number: String, // CIPC registration
    pub tax_number: Option<String>,  // SARS tax number
    pub vat_number: Option<String>,

    // B-BBEE Information
    pub bbbee_level: BbbeeLevel,
    pub bbbee_certificate_number: Option<String>,
    pub bbbee_expiry_date: Option<String>,
    pub bbbee_verification_agency: Option<String>,

    // Categories
    pub categories: Vec<SupplierCategory>,

    // Performance
    pub performance_score: PerformanceScore,
    pub performance_history: Vec<PerformanceTrend>,

    // Risk
    pub risk_rating: RiskRating,
    pub risk_indicators: Vec<RiskIndicator>,
    pub risk_score: f64,

    // Contact
    pub contact_person: String,
    pub email: String,
    pub phone: String,
    pub physical_address: String,
    pub postal_address: Option<String>,
    pub province: String,
    pub city: String,

    // Banking details
    pub bank_name: Option<String>,
    pub bank_account_number: Option<String>,
    pub bank_branch_code: Option<String>,

    // Status
    pub status: SupplierStatus,
    pub registered_at: String,
    pub verified_at: Option<String>,
    pub last_active: String,

    // Contracts
    pub active_contracts: u32,
    pub total_contract_value: f64,
    pub contracts: Vec<SupplierContract>,

    // CSD (Central Supplier Database) reference
    pub csd_number: Option<String>,
    pub csd_verified: bool,
}

impl Default for Supplier {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            trading_name: None,
            registration_number: String::new(),
            tax_number: None,
            vat_number: None,
            bbbee_level: BbbeeLevel::default(),
            bbbee_certificate_number: None,
            bbbee_expiry_date: None,
            bbbee_verification_agency: None,
            categories: Vec::new(),
            performance_score: PerformanceScore::default(),
            performance_history: Vec::new(),
            risk_rating: RiskRating::default(),
            risk_indicators: Vec::new(),
            risk_score: 0.0,
            contact_person: String::new(),
            email: String::new(),
            phone: String::new(),
            physical_address: String::new(),
            postal_address: None,
            province: String::new(),
            city: String::new(),
            bank_name: None,
            bank_account_number: None,
            bank_branch_code: None,
            status: SupplierStatus::default(),
            registered_at: String::new(),
            verified_at: None,
            last_active: String::new(),
            active_contracts: 0,
            total_contract_value: 0.0,
            contracts: Vec::new(),
            csd_number: None,
            csd_verified: false,
        }
    }
}

/// Filter criteria for supplier list
#[derive(Clone, Debug, Default)]
pub struct SupplierFilter {
    pub search_query: Option<String>,
    pub status: Option<SupplierStatus>,
    pub bbbee_level: Option<BbbeeLevel>,
    pub risk_rating: Option<RiskRating>,
    pub category: Option<String>,
    pub province: Option<String>,
    pub min_performance_score: Option<f64>,
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

/// Supplier summary for KPIs
#[derive(Clone, Debug, Default)]
pub struct SupplierKpis {
    pub total_suppliers: u32,
    pub active_suppliers: u32,
    pub pending_verification: u32,
    pub bbbee_compliant_percent: f64,
    pub avg_performance_score: f64,
    pub high_risk_count: u32,
    pub expiring_certificates: u32,
}
