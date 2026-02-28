//! Catalogue domain types

use serde::{Deserialize, Serialize};

/// Catalogue item status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CatalogueItemStatus {
    /// Active and available for ordering
    Active,
    /// Temporarily unavailable
    Inactive,
    /// Discontinued
    Discontinued,
    /// Pending approval
    PendingApproval,
}

impl CatalogueItemStatus {
    pub fn label(&self) -> &'static str {
        match self {
            CatalogueItemStatus::Active => "Active",
            CatalogueItemStatus::Inactive => "Inactive",
            CatalogueItemStatus::Discontinued => "Discontinued",
            CatalogueItemStatus::PendingApproval => "Pending Approval",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "active" => CatalogueItemStatus::Active,
            "inactive" => CatalogueItemStatus::Inactive,
            "discontinued" => CatalogueItemStatus::Discontinued,
            "pending_approval" | "pending" => CatalogueItemStatus::PendingApproval,
            _ => CatalogueItemStatus::Active,
        }
    }
}

impl Default for CatalogueItemStatus {
    fn default() -> Self {
        CatalogueItemStatus::Active
    }
}

/// Catalogue category
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CatalogueCategory {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: String,
    pub parent_id: Option<String>,
    pub icon: Option<String>,
    pub item_count: u32,
}

impl Default for CatalogueCategory {
    fn default() -> Self {
        Self {
            id: String::new(),
            code: String::new(),
            name: String::new(),
            description: String::new(),
            parent_id: None,
            icon: None,
            item_count: 0,
        }
    }
}

/// Unit of measure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnitOfMeasure {
    pub code: String,
    pub name: String,
    pub symbol: String,
}

impl Default for UnitOfMeasure {
    fn default() -> Self {
        Self {
            code: "EA".to_string(),
            name: "Each".to_string(),
            symbol: "ea".to_string(),
        }
    }
}

/// Item specification key-value pair
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemSpecification {
    pub key: String,
    pub value: String,
    pub unit: Option<String>,
}

/// Supplier information for catalogue item
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CatalogueSupplier {
    pub id: String,
    pub name: String,
    pub bbbee_level: u8,
    pub lead_time_days: u32,
    pub minimum_order_quantity: u32,
    pub is_preferred: bool,
}

impl Default for CatalogueSupplier {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            bbbee_level: 0,
            lead_time_days: 7,
            minimum_order_quantity: 1,
            is_preferred: false,
        }
    }
}

/// Price tier for volume discounts
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceTier {
    pub min_quantity: u32,
    pub max_quantity: Option<u32>,
    pub unit_price: f64,
}

/// Main Catalogue Item entity
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CatalogueItem {
    pub id: String,
    pub item_code: String,
    pub name: String,
    pub description: String,

    // Category
    pub category_id: String,
    pub category_name: String,
    pub category_code: String,

    // Pricing
    pub unit_price: f64,
    pub currency: String,
    pub price_tiers: Vec<PriceTier>,
    pub vat_inclusive: bool,
    pub vat_rate: f64,

    // Unit of measure
    pub unit_of_measure: UnitOfMeasure,

    // Supplier
    pub supplier: CatalogueSupplier,
    pub alternate_suppliers: Vec<CatalogueSupplier>,

    // Specifications
    pub specifications: Vec<ItemSpecification>,
    pub brand: Option<String>,
    pub manufacturer: Option<String>,
    pub model_number: Option<String>,

    // Media
    pub image_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub datasheet_url: Option<String>,

    // Stock
    pub in_stock: bool,
    pub stock_quantity: Option<u32>,
    pub reorder_level: Option<u32>,

    // Status
    pub status: CatalogueItemStatus,
    pub created_at: String,
    pub updated_at: String,

    // Search/filter helpers
    pub tags: Vec<String>,
    pub featured: bool,
}

impl Default for CatalogueItem {
    fn default() -> Self {
        Self {
            id: String::new(),
            item_code: String::new(),
            name: String::new(),
            description: String::new(),
            category_id: String::new(),
            category_name: String::new(),
            category_code: String::new(),
            unit_price: 0.0,
            currency: "ZAR".to_string(),
            price_tiers: Vec::new(),
            vat_inclusive: true,
            vat_rate: 15.0,
            unit_of_measure: UnitOfMeasure::default(),
            supplier: CatalogueSupplier::default(),
            alternate_suppliers: Vec::new(),
            specifications: Vec::new(),
            brand: None,
            manufacturer: None,
            model_number: None,
            image_url: None,
            thumbnail_url: None,
            datasheet_url: None,
            in_stock: true,
            stock_quantity: None,
            reorder_level: None,
            status: CatalogueItemStatus::default(),
            created_at: String::new(),
            updated_at: String::new(),
            tags: Vec::new(),
            featured: false,
        }
    }
}

/// Filter criteria for catalogue items
#[derive(Clone, Debug, Default)]
pub struct CatalogueFilter {
    pub search_query: Option<String>,
    pub category_id: Option<String>,
    pub supplier_id: Option<String>,
    pub status: Option<CatalogueItemStatus>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub in_stock_only: bool,
    pub featured_only: bool,
    pub bbbee_level: Option<u8>,
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
            page_size: 12,
            total_items: 0,
            total_pages: 0,
        }
    }
}

/// Catalogue summary KPIs
#[derive(Clone, Debug, Default)]
pub struct CatalogueKpis {
    pub total_items: u32,
    pub active_items: u32,
    pub categories_count: u32,
    pub suppliers_count: u32,
    pub out_of_stock: u32,
    pub pending_approval: u32,
    pub avg_price: f64,
}

/// Sort options for catalogue
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CatalogueSortBy {
    NameAsc,
    NameDesc,
    PriceAsc,
    PriceDesc,
    CategoryAsc,
    RecentlyAdded,
    MostPopular,
}

impl Default for CatalogueSortBy {
    fn default() -> Self {
        CatalogueSortBy::NameAsc
    }
}

impl CatalogueSortBy {
    pub fn label(&self) -> &'static str {
        match self {
            CatalogueSortBy::NameAsc => "Name (A-Z)",
            CatalogueSortBy::NameDesc => "Name (Z-A)",
            CatalogueSortBy::PriceAsc => "Price (Low to High)",
            CatalogueSortBy::PriceDesc => "Price (High to Low)",
            CatalogueSortBy::CategoryAsc => "Category",
            CatalogueSortBy::RecentlyAdded => "Recently Added",
            CatalogueSortBy::MostPopular => "Most Popular",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "name_asc" => CatalogueSortBy::NameAsc,
            "name_desc" => CatalogueSortBy::NameDesc,
            "price_asc" => CatalogueSortBy::PriceAsc,
            "price_desc" => CatalogueSortBy::PriceDesc,
            "category" => CatalogueSortBy::CategoryAsc,
            "recent" => CatalogueSortBy::RecentlyAdded,
            "popular" => CatalogueSortBy::MostPopular,
            _ => CatalogueSortBy::NameAsc,
        }
    }
}
