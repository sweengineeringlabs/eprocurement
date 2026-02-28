//! Catalogue store

use components::prelude::*;
use super::types::{
    CatalogueItem, CatalogueCategory, CatalogueFilter, CatalogueItemStatus,
    CatalogueKpis, PaginationState, CatalogueSortBy, CatalogueSupplier,
    ItemSpecification, UnitOfMeasure, PriceTier,
};

/// Catalogue state store
#[derive(Clone)]
pub struct CatalogueStore {
    pub items: Signal<Vec<CatalogueItem>>,
    pub categories: Signal<Vec<CatalogueCategory>>,
    pub selected: Signal<Option<CatalogueItem>>,
    pub filter: Signal<CatalogueFilter>,
    pub sort_by: Signal<CatalogueSortBy>,
    pub pagination: Signal<PaginationState>,
    pub kpis: Signal<CatalogueKpis>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
    pub view_mode: Signal<String>, // "grid" or "list"
}

impl CatalogueStore {
    pub fn new() -> Self {
        Self {
            items: signal(Vec::new()),
            categories: signal(Vec::new()),
            selected: signal(None),
            filter: signal(CatalogueFilter::default()),
            sort_by: signal(CatalogueSortBy::default()),
            pagination: signal(PaginationState::default()),
            kpis: signal(CatalogueKpis::default()),
            loading: signal(false),
            error: signal(None),
            view_mode: signal("grid".to_string()),
        }
    }

    /// Get filtered and sorted items
    pub fn get_filtered_items(&self) -> Vec<CatalogueItem> {
        let items = self.items.get();
        let filter = self.filter.get();
        let sort_by = self.sort_by.get();

        let mut filtered: Vec<CatalogueItem> = items.iter()
            .filter(|item| {
                // Search query
                let search_match = if let Some(ref query) = filter.search_query {
                    let q = query.to_lowercase();
                    item.name.to_lowercase().contains(&q) ||
                    item.item_code.to_lowercase().contains(&q) ||
                    item.description.to_lowercase().contains(&q) ||
                    item.category_name.to_lowercase().contains(&q) ||
                    item.tags.iter().any(|t| t.to_lowercase().contains(&q))
                } else {
                    true
                };

                // Category filter
                let category_match = if let Some(ref cat_id) = filter.category_id {
                    &item.category_id == cat_id
                } else {
                    true
                };

                // Supplier filter
                let supplier_match = if let Some(ref sup_id) = filter.supplier_id {
                    &item.supplier.id == sup_id
                } else {
                    true
                };

                // Status filter
                let status_match = if let Some(status) = filter.status {
                    item.status == status
                } else {
                    true
                };

                // Price range
                let price_min_match = if let Some(min) = filter.min_price {
                    item.unit_price >= min
                } else {
                    true
                };

                let price_max_match = if let Some(max) = filter.max_price {
                    item.unit_price <= max
                } else {
                    true
                };

                // Stock filter
                let stock_match = if filter.in_stock_only {
                    item.in_stock
                } else {
                    true
                };

                // Featured filter
                let featured_match = if filter.featured_only {
                    item.featured
                } else {
                    true
                };

                // B-BBEE level filter
                let bbbee_match = if let Some(level) = filter.bbbee_level {
                    item.supplier.bbbee_level <= level
                } else {
                    true
                };

                search_match && category_match && supplier_match && status_match &&
                price_min_match && price_max_match && stock_match && featured_match && bbbee_match
            })
            .cloned()
            .collect();

        // Sort
        match sort_by {
            CatalogueSortBy::NameAsc => filtered.sort_by(|a, b| a.name.cmp(&b.name)),
            CatalogueSortBy::NameDesc => filtered.sort_by(|a, b| b.name.cmp(&a.name)),
            CatalogueSortBy::PriceAsc => filtered.sort_by(|a, b| a.unit_price.partial_cmp(&b.unit_price).unwrap()),
            CatalogueSortBy::PriceDesc => filtered.sort_by(|a, b| b.unit_price.partial_cmp(&a.unit_price).unwrap()),
            CatalogueSortBy::CategoryAsc => filtered.sort_by(|a, b| a.category_name.cmp(&b.category_name)),
            CatalogueSortBy::RecentlyAdded => filtered.sort_by(|a, b| b.created_at.cmp(&a.created_at)),
            CatalogueSortBy::MostPopular => filtered.sort_by(|a, b| b.featured.cmp(&a.featured)),
        }

        filtered
    }

    /// Set filter search query
    pub fn set_search(&self, query: Option<String>) {
        let mut filter = self.filter.get();
        filter.search_query = query;
        self.filter.set(filter);
    }

    /// Set filter category
    pub fn set_category(&self, category_id: Option<String>) {
        let mut filter = self.filter.get();
        filter.category_id = category_id;
        self.filter.set(filter);
    }

    /// Set filter status
    pub fn set_status(&self, status: Option<CatalogueItemStatus>) {
        let mut filter = self.filter.get();
        filter.status = status;
        self.filter.set(filter);
    }

    /// Set in stock only filter
    pub fn set_in_stock_only(&self, value: bool) {
        let mut filter = self.filter.get();
        filter.in_stock_only = value;
        self.filter.set(filter);
    }

    /// Clear all filters
    pub fn clear_filters(&self) {
        self.filter.set(CatalogueFilter::default());
    }

    /// Get count by status
    pub fn get_count_by_status(&self, status: CatalogueItemStatus) -> usize {
        self.items.get().iter().filter(|i| i.status == status).count()
    }
}

/// Load mock catalogue data
pub fn load_mock_data(store: &CatalogueStore) {
    let categories = vec![
        CatalogueCategory {
            id: "CAT-001".to_string(),
            code: "IT".to_string(),
            name: "IT Equipment".to_string(),
            description: "Computers, peripherals, and IT accessories".to_string(),
            parent_id: None,
            icon: Some("laptop".to_string()),
            item_count: 45,
        },
        CatalogueCategory {
            id: "CAT-002".to_string(),
            code: "OFFICE".to_string(),
            name: "Office Supplies".to_string(),
            description: "Stationery, paper, and office consumables".to_string(),
            parent_id: None,
            icon: Some("clipboard".to_string()),
            item_count: 120,
        },
        CatalogueCategory {
            id: "CAT-003".to_string(),
            code: "FURN".to_string(),
            name: "Furniture".to_string(),
            description: "Office furniture and fittings".to_string(),
            parent_id: None,
            icon: Some("chair".to_string()),
            item_count: 35,
        },
        CatalogueCategory {
            id: "CAT-004".to_string(),
            code: "CLEAN".to_string(),
            name: "Cleaning Supplies".to_string(),
            description: "Cleaning materials and equipment".to_string(),
            parent_id: None,
            icon: Some("spray".to_string()),
            item_count: 28,
        },
        CatalogueCategory {
            id: "CAT-005".to_string(),
            code: "SAFETY".to_string(),
            name: "Safety & PPE".to_string(),
            description: "Personal protective equipment and safety gear".to_string(),
            parent_id: None,
            icon: Some("shield".to_string()),
            item_count: 42,
        },
        CatalogueCategory {
            id: "CAT-006".to_string(),
            code: "ELEC".to_string(),
            name: "Electrical".to_string(),
            description: "Electrical supplies and equipment".to_string(),
            parent_id: None,
            icon: Some("zap".to_string()),
            item_count: 18,
        },
    ];

    let mock_items = vec![
        CatalogueItem {
            id: "ITEM-001".to_string(),
            item_code: "IT-LAP-001".to_string(),
            name: "Dell Latitude 5540 Laptop".to_string(),
            description: "15.6\" Business Laptop with Intel Core i7, 16GB RAM, 512GB SSD".to_string(),
            category_id: "CAT-001".to_string(),
            category_name: "IT Equipment".to_string(),
            category_code: "IT".to_string(),
            unit_price: 24999.00,
            currency: "ZAR".to_string(),
            price_tiers: vec![
                PriceTier { min_quantity: 1, max_quantity: Some(9), unit_price: 24999.00 },
                PriceTier { min_quantity: 10, max_quantity: Some(49), unit_price: 23999.00 },
                PriceTier { min_quantity: 50, max_quantity: None, unit_price: 22999.00 },
            ],
            vat_inclusive: true,
            vat_rate: 15.0,
            unit_of_measure: UnitOfMeasure { code: "EA".to_string(), name: "Each".to_string(), symbol: "ea".to_string() },
            supplier: CatalogueSupplier {
                id: "SUP-001".to_string(),
                name: "TechSolutions SA (Pty) Ltd".to_string(),
                bbbee_level: 1,
                lead_time_days: 5,
                minimum_order_quantity: 1,
                is_preferred: true,
            },
            alternate_suppliers: vec![],
            specifications: vec![
                ItemSpecification { key: "Processor".to_string(), value: "Intel Core i7-1365U".to_string(), unit: None },
                ItemSpecification { key: "RAM".to_string(), value: "16".to_string(), unit: Some("GB".to_string()) },
                ItemSpecification { key: "Storage".to_string(), value: "512".to_string(), unit: Some("GB SSD".to_string()) },
                ItemSpecification { key: "Display".to_string(), value: "15.6\" FHD".to_string(), unit: None },
                ItemSpecification { key: "Warranty".to_string(), value: "3".to_string(), unit: Some("Years".to_string()) },
            ],
            brand: Some("Dell".to_string()),
            manufacturer: Some("Dell Technologies".to_string()),
            model_number: Some("Latitude 5540".to_string()),
            image_url: Some("/images/catalogue/laptop.jpg".to_string()),
            thumbnail_url: Some("/images/catalogue/laptop-thumb.jpg".to_string()),
            datasheet_url: Some("/docs/dell-latitude-5540.pdf".to_string()),
            in_stock: true,
            stock_quantity: Some(25),
            reorder_level: Some(10),
            status: CatalogueItemStatus::Active,
            created_at: "2024-06-15".to_string(),
            updated_at: "2025-02-10".to_string(),
            tags: vec!["laptop".to_string(), "computer".to_string(), "dell".to_string(), "business".to_string()],
            featured: true,
        },
        CatalogueItem {
            id: "ITEM-002".to_string(),
            item_code: "IT-MON-001".to_string(),
            name: "Samsung 27\" 4K Monitor".to_string(),
            description: "27\" UHD IPS Monitor with USB-C connectivity, 60Hz".to_string(),
            category_id: "CAT-001".to_string(),
            category_name: "IT Equipment".to_string(),
            category_code: "IT".to_string(),
            unit_price: 7499.00,
            currency: "ZAR".to_string(),
            price_tiers: vec![],
            vat_inclusive: true,
            vat_rate: 15.0,
            unit_of_measure: UnitOfMeasure { code: "EA".to_string(), name: "Each".to_string(), symbol: "ea".to_string() },
            supplier: CatalogueSupplier {
                id: "SUP-001".to_string(),
                name: "TechSolutions SA (Pty) Ltd".to_string(),
                bbbee_level: 1,
                lead_time_days: 3,
                minimum_order_quantity: 1,
                is_preferred: true,
            },
            alternate_suppliers: vec![],
            specifications: vec![
                ItemSpecification { key: "Resolution".to_string(), value: "3840 x 2160".to_string(), unit: None },
                ItemSpecification { key: "Panel".to_string(), value: "IPS".to_string(), unit: None },
                ItemSpecification { key: "Refresh Rate".to_string(), value: "60".to_string(), unit: Some("Hz".to_string()) },
            ],
            brand: Some("Samsung".to_string()),
            manufacturer: Some("Samsung Electronics".to_string()),
            model_number: Some("S27A804".to_string()),
            image_url: Some("/images/catalogue/monitor.jpg".to_string()),
            thumbnail_url: Some("/images/catalogue/monitor-thumb.jpg".to_string()),
            datasheet_url: None,
            in_stock: true,
            stock_quantity: Some(42),
            reorder_level: Some(15),
            status: CatalogueItemStatus::Active,
            created_at: "2024-07-01".to_string(),
            updated_at: "2025-02-08".to_string(),
            tags: vec!["monitor".to_string(), "display".to_string(), "4k".to_string(), "samsung".to_string()],
            featured: true,
        },
        CatalogueItem {
            id: "ITEM-003".to_string(),
            item_code: "OFF-PAP-001".to_string(),
            name: "A4 Copy Paper (5 Ream Box)".to_string(),
            description: "Premium white A4 copy paper, 80gsm, 500 sheets per ream".to_string(),
            category_id: "CAT-002".to_string(),
            category_name: "Office Supplies".to_string(),
            category_code: "OFFICE".to_string(),
            unit_price: 389.00,
            currency: "ZAR".to_string(),
            price_tiers: vec![
                PriceTier { min_quantity: 1, max_quantity: Some(9), unit_price: 389.00 },
                PriceTier { min_quantity: 10, max_quantity: Some(49), unit_price: 369.00 },
                PriceTier { min_quantity: 50, max_quantity: None, unit_price: 349.00 },
            ],
            vat_inclusive: true,
            vat_rate: 15.0,
            unit_of_measure: UnitOfMeasure { code: "BOX".to_string(), name: "Box".to_string(), symbol: "box".to_string() },
            supplier: CatalogueSupplier {
                id: "SUP-002".to_string(),
                name: "Office Essentials SA".to_string(),
                bbbee_level: 2,
                lead_time_days: 2,
                minimum_order_quantity: 1,
                is_preferred: true,
            },
            alternate_suppliers: vec![],
            specifications: vec![
                ItemSpecification { key: "Size".to_string(), value: "A4".to_string(), unit: None },
                ItemSpecification { key: "Weight".to_string(), value: "80".to_string(), unit: Some("gsm".to_string()) },
                ItemSpecification { key: "Sheets/Ream".to_string(), value: "500".to_string(), unit: None },
                ItemSpecification { key: "Reams/Box".to_string(), value: "5".to_string(), unit: None },
            ],
            brand: Some("Typek".to_string()),
            manufacturer: Some("Sappi".to_string()),
            model_number: None,
            image_url: Some("/images/catalogue/paper.jpg".to_string()),
            thumbnail_url: Some("/images/catalogue/paper-thumb.jpg".to_string()),
            datasheet_url: None,
            in_stock: true,
            stock_quantity: Some(500),
            reorder_level: Some(100),
            status: CatalogueItemStatus::Active,
            created_at: "2024-01-10".to_string(),
            updated_at: "2025-02-12".to_string(),
            tags: vec!["paper".to_string(), "stationery".to_string(), "a4".to_string(), "copy paper".to_string()],
            featured: false,
        },
        CatalogueItem {
            id: "ITEM-004".to_string(),
            item_code: "FURN-DSK-001".to_string(),
            name: "Executive Office Desk".to_string(),
            description: "L-shaped executive desk with cable management, 1800mm x 1600mm".to_string(),
            category_id: "CAT-003".to_string(),
            category_name: "Furniture".to_string(),
            category_code: "FURN".to_string(),
            unit_price: 8999.00,
            currency: "ZAR".to_string(),
            price_tiers: vec![],
            vat_inclusive: true,
            vat_rate: 15.0,
            unit_of_measure: UnitOfMeasure { code: "EA".to_string(), name: "Each".to_string(), symbol: "ea".to_string() },
            supplier: CatalogueSupplier {
                id: "SUP-010".to_string(),
                name: "Free State Furniture Factory".to_string(),
                bbbee_level: 2,
                lead_time_days: 14,
                minimum_order_quantity: 1,
                is_preferred: true,
            },
            alternate_suppliers: vec![],
            specifications: vec![
                ItemSpecification { key: "Width".to_string(), value: "1800".to_string(), unit: Some("mm".to_string()) },
                ItemSpecification { key: "Depth".to_string(), value: "1600".to_string(), unit: Some("mm".to_string()) },
                ItemSpecification { key: "Height".to_string(), value: "750".to_string(), unit: Some("mm".to_string()) },
                ItemSpecification { key: "Material".to_string(), value: "Melamine".to_string(), unit: None },
                ItemSpecification { key: "Color".to_string(), value: "White Oak".to_string(), unit: None },
            ],
            brand: Some("FS Furniture".to_string()),
            manufacturer: Some("Free State Furniture Factory".to_string()),
            model_number: Some("EXD-1800L".to_string()),
            image_url: Some("/images/catalogue/desk.jpg".to_string()),
            thumbnail_url: Some("/images/catalogue/desk-thumb.jpg".to_string()),
            datasheet_url: None,
            in_stock: true,
            stock_quantity: Some(8),
            reorder_level: Some(5),
            status: CatalogueItemStatus::Active,
            created_at: "2024-03-20".to_string(),
            updated_at: "2025-01-15".to_string(),
            tags: vec!["desk".to_string(), "furniture".to_string(), "office".to_string(), "executive".to_string()],
            featured: true,
        },
        CatalogueItem {
            id: "ITEM-005".to_string(),
            item_code: "FURN-CHR-001".to_string(),
            name: "Ergonomic Office Chair".to_string(),
            description: "High-back ergonomic mesh chair with lumbar support and adjustable armrests".to_string(),
            category_id: "CAT-003".to_string(),
            category_name: "Furniture".to_string(),
            category_code: "FURN".to_string(),
            unit_price: 4599.00,
            currency: "ZAR".to_string(),
            price_tiers: vec![
                PriceTier { min_quantity: 1, max_quantity: Some(9), unit_price: 4599.00 },
                PriceTier { min_quantity: 10, max_quantity: Some(24), unit_price: 4399.00 },
                PriceTier { min_quantity: 25, max_quantity: None, unit_price: 4199.00 },
            ],
            vat_inclusive: true,
            vat_rate: 15.0,
            unit_of_measure: UnitOfMeasure { code: "EA".to_string(), name: "Each".to_string(), symbol: "ea".to_string() },
            supplier: CatalogueSupplier {
                id: "SUP-010".to_string(),
                name: "Free State Furniture Factory".to_string(),
                bbbee_level: 2,
                lead_time_days: 7,
                minimum_order_quantity: 1,
                is_preferred: true,
            },
            alternate_suppliers: vec![],
            specifications: vec![
                ItemSpecification { key: "Max Weight".to_string(), value: "120".to_string(), unit: Some("kg".to_string()) },
                ItemSpecification { key: "Seat Height".to_string(), value: "440-540".to_string(), unit: Some("mm".to_string()) },
                ItemSpecification { key: "Material".to_string(), value: "Mesh/Fabric".to_string(), unit: None },
                ItemSpecification { key: "Warranty".to_string(), value: "5".to_string(), unit: Some("Years".to_string()) },
            ],
            brand: Some("FS Furniture".to_string()),
            manufacturer: Some("Free State Furniture Factory".to_string()),
            model_number: Some("ERG-PRO-500".to_string()),
            image_url: Some("/images/catalogue/chair.jpg".to_string()),
            thumbnail_url: Some("/images/catalogue/chair-thumb.jpg".to_string()),
            datasheet_url: None,
            in_stock: true,
            stock_quantity: Some(35),
            reorder_level: Some(15),
            status: CatalogueItemStatus::Active,
            created_at: "2024-02-01".to_string(),
            updated_at: "2025-02-05".to_string(),
            tags: vec!["chair".to_string(), "furniture".to_string(), "ergonomic".to_string(), "office".to_string()],
            featured: true,
        },
        CatalogueItem {
            id: "ITEM-006".to_string(),
            item_code: "CLEAN-DET-001".to_string(),
            name: "Multi-Surface Cleaner (5L)".to_string(),
            description: "Concentrated multi-surface cleaning solution, biodegradable".to_string(),
            category_id: "CAT-004".to_string(),
            category_name: "Cleaning Supplies".to_string(),
            category_code: "CLEAN".to_string(),
            unit_price: 189.00,
            currency: "ZAR".to_string(),
            price_tiers: vec![],
            vat_inclusive: true,
            vat_rate: 15.0,
            unit_of_measure: UnitOfMeasure { code: "BTL".to_string(), name: "Bottle".to_string(), symbol: "btl".to_string() },
            supplier: CatalogueSupplier {
                id: "SUP-009".to_string(),
                name: "Limpopo Cleaning Services".to_string(),
                bbbee_level: 1,
                lead_time_days: 3,
                minimum_order_quantity: 4,
                is_preferred: true,
            },
            alternate_suppliers: vec![],
            specifications: vec![
                ItemSpecification { key: "Volume".to_string(), value: "5".to_string(), unit: Some("Litres".to_string()) },
                ItemSpecification { key: "Dilution".to_string(), value: "1:20".to_string(), unit: None },
                ItemSpecification { key: "Type".to_string(), value: "Biodegradable".to_string(), unit: None },
            ],
            brand: Some("EcoClean".to_string()),
            manufacturer: Some("Limpopo Cleaning Services".to_string()),
            model_number: None,
            image_url: Some("/images/catalogue/cleaner.jpg".to_string()),
            thumbnail_url: Some("/images/catalogue/cleaner-thumb.jpg".to_string()),
            datasheet_url: None,
            in_stock: true,
            stock_quantity: Some(120),
            reorder_level: Some(40),
            status: CatalogueItemStatus::Active,
            created_at: "2024-04-10".to_string(),
            updated_at: "2025-01-20".to_string(),
            tags: vec!["cleaner".to_string(), "cleaning".to_string(), "detergent".to_string(), "eco".to_string()],
            featured: false,
        },
        CatalogueItem {
            id: "ITEM-007".to_string(),
            item_code: "SAFE-HEL-001".to_string(),
            name: "Safety Hard Hat".to_string(),
            description: "SABS approved safety hard hat with ratchet adjustment".to_string(),
            category_id: "CAT-005".to_string(),
            category_name: "Safety & PPE".to_string(),
            category_code: "SAFETY".to_string(),
            unit_price: 149.00,
            currency: "ZAR".to_string(),
            price_tiers: vec![
                PriceTier { min_quantity: 1, max_quantity: Some(19), unit_price: 149.00 },
                PriceTier { min_quantity: 20, max_quantity: Some(99), unit_price: 139.00 },
                PriceTier { min_quantity: 100, max_quantity: None, unit_price: 129.00 },
            ],
            vat_inclusive: true,
            vat_rate: 15.0,
            unit_of_measure: UnitOfMeasure { code: "EA".to_string(), name: "Each".to_string(), symbol: "ea".to_string() },
            supplier: CatalogueSupplier {
                id: "SUP-004".to_string(),
                name: "SecureGuard Holdings (Pty) Ltd".to_string(),
                bbbee_level: 1,
                lead_time_days: 5,
                minimum_order_quantity: 1,
                is_preferred: true,
            },
            alternate_suppliers: vec![],
            specifications: vec![
                ItemSpecification { key: "Standard".to_string(), value: "SANS 1397".to_string(), unit: None },
                ItemSpecification { key: "Material".to_string(), value: "HDPE".to_string(), unit: None },
                ItemSpecification { key: "Colors".to_string(), value: "White, Yellow, Blue, Red".to_string(), unit: None },
            ],
            brand: Some("SafetyFirst".to_string()),
            manufacturer: Some("SecureGuard Holdings".to_string()),
            model_number: Some("HH-100".to_string()),
            image_url: Some("/images/catalogue/hardhat.jpg".to_string()),
            thumbnail_url: Some("/images/catalogue/hardhat-thumb.jpg".to_string()),
            datasheet_url: None,
            in_stock: true,
            stock_quantity: Some(250),
            reorder_level: Some(50),
            status: CatalogueItemStatus::Active,
            created_at: "2024-05-15".to_string(),
            updated_at: "2025-02-01".to_string(),
            tags: vec!["ppe".to_string(), "safety".to_string(), "hard hat".to_string(), "helmet".to_string()],
            featured: false,
        },
        CatalogueItem {
            id: "ITEM-008".to_string(),
            item_code: "IT-PRN-001".to_string(),
            name: "HP LaserJet Pro MFP".to_string(),
            description: "Multifunction laser printer with scan, copy, and wireless capability".to_string(),
            category_id: "CAT-001".to_string(),
            category_name: "IT Equipment".to_string(),
            category_code: "IT".to_string(),
            unit_price: 6999.00,
            currency: "ZAR".to_string(),
            price_tiers: vec![],
            vat_inclusive: true,
            vat_rate: 15.0,
            unit_of_measure: UnitOfMeasure { code: "EA".to_string(), name: "Each".to_string(), symbol: "ea".to_string() },
            supplier: CatalogueSupplier {
                id: "SUP-001".to_string(),
                name: "TechSolutions SA (Pty) Ltd".to_string(),
                bbbee_level: 1,
                lead_time_days: 5,
                minimum_order_quantity: 1,
                is_preferred: true,
            },
            alternate_suppliers: vec![],
            specifications: vec![
                ItemSpecification { key: "Print Speed".to_string(), value: "28".to_string(), unit: Some("ppm".to_string()) },
                ItemSpecification { key: "Duplex".to_string(), value: "Automatic".to_string(), unit: None },
                ItemSpecification { key: "Connectivity".to_string(), value: "USB, Ethernet, WiFi".to_string(), unit: None },
            ],
            brand: Some("HP".to_string()),
            manufacturer: Some("Hewlett-Packard".to_string()),
            model_number: Some("M428fdw".to_string()),
            image_url: Some("/images/catalogue/printer.jpg".to_string()),
            thumbnail_url: Some("/images/catalogue/printer-thumb.jpg".to_string()),
            datasheet_url: None,
            in_stock: false,
            stock_quantity: Some(0),
            reorder_level: Some(5),
            status: CatalogueItemStatus::Active,
            created_at: "2024-08-01".to_string(),
            updated_at: "2025-02-10".to_string(),
            tags: vec!["printer".to_string(), "laser".to_string(), "mfp".to_string(), "hp".to_string()],
            featured: false,
        },
        CatalogueItem {
            id: "ITEM-009".to_string(),
            item_code: "OFF-PEN-001".to_string(),
            name: "Ballpoint Pens (Box of 50)".to_string(),
            description: "Medium point ballpoint pens, black ink".to_string(),
            category_id: "CAT-002".to_string(),
            category_name: "Office Supplies".to_string(),
            category_code: "OFFICE".to_string(),
            unit_price: 89.00,
            currency: "ZAR".to_string(),
            price_tiers: vec![],
            vat_inclusive: true,
            vat_rate: 15.0,
            unit_of_measure: UnitOfMeasure { code: "BOX".to_string(), name: "Box".to_string(), symbol: "box".to_string() },
            supplier: CatalogueSupplier {
                id: "SUP-002".to_string(),
                name: "Office Essentials SA".to_string(),
                bbbee_level: 2,
                lead_time_days: 2,
                minimum_order_quantity: 1,
                is_preferred: true,
            },
            alternate_suppliers: vec![],
            specifications: vec![
                ItemSpecification { key: "Ink Color".to_string(), value: "Black".to_string(), unit: None },
                ItemSpecification { key: "Point Size".to_string(), value: "Medium (1.0mm)".to_string(), unit: None },
                ItemSpecification { key: "Quantity".to_string(), value: "50".to_string(), unit: Some("pens/box".to_string()) },
            ],
            brand: Some("BIC".to_string()),
            manufacturer: Some("BIC".to_string()),
            model_number: Some("Cristal".to_string()),
            image_url: Some("/images/catalogue/pens.jpg".to_string()),
            thumbnail_url: Some("/images/catalogue/pens-thumb.jpg".to_string()),
            datasheet_url: None,
            in_stock: true,
            stock_quantity: Some(200),
            reorder_level: Some(50),
            status: CatalogueItemStatus::Active,
            created_at: "2024-01-05".to_string(),
            updated_at: "2025-01-28".to_string(),
            tags: vec!["pens".to_string(), "stationery".to_string(), "ballpoint".to_string()],
            featured: false,
        },
        CatalogueItem {
            id: "ITEM-010".to_string(),
            item_code: "ELEC-CAB-001".to_string(),
            name: "CAT6 Network Cable (305m)".to_string(),
            description: "CAT6 UTP network cable, 305m box".to_string(),
            category_id: "CAT-006".to_string(),
            category_name: "Electrical".to_string(),
            category_code: "ELEC".to_string(),
            unit_price: 2499.00,
            currency: "ZAR".to_string(),
            price_tiers: vec![],
            vat_inclusive: true,
            vat_rate: 15.0,
            unit_of_measure: UnitOfMeasure { code: "BOX".to_string(), name: "Box".to_string(), symbol: "box".to_string() },
            supplier: CatalogueSupplier {
                id: "SUP-001".to_string(),
                name: "TechSolutions SA (Pty) Ltd".to_string(),
                bbbee_level: 1,
                lead_time_days: 3,
                minimum_order_quantity: 1,
                is_preferred: true,
            },
            alternate_suppliers: vec![],
            specifications: vec![
                ItemSpecification { key: "Category".to_string(), value: "CAT6".to_string(), unit: None },
                ItemSpecification { key: "Type".to_string(), value: "UTP".to_string(), unit: None },
                ItemSpecification { key: "Length".to_string(), value: "305".to_string(), unit: Some("m".to_string()) },
            ],
            brand: Some("Linkbasic".to_string()),
            manufacturer: Some("Linkbasic".to_string()),
            model_number: Some("CAT6-305".to_string()),
            image_url: Some("/images/catalogue/cable.jpg".to_string()),
            thumbnail_url: Some("/images/catalogue/cable-thumb.jpg".to_string()),
            datasheet_url: None,
            in_stock: true,
            stock_quantity: Some(18),
            reorder_level: Some(10),
            status: CatalogueItemStatus::Active,
            created_at: "2024-09-10".to_string(),
            updated_at: "2025-02-05".to_string(),
            tags: vec!["cable".to_string(), "network".to_string(), "cat6".to_string(), "ethernet".to_string()],
            featured: false,
        },
        CatalogueItem {
            id: "ITEM-011".to_string(),
            item_code: "SAFE-GLV-001".to_string(),
            name: "Nitrile Gloves (Box of 100)".to_string(),
            description: "Disposable nitrile examination gloves, powder-free".to_string(),
            category_id: "CAT-005".to_string(),
            category_name: "Safety & PPE".to_string(),
            category_code: "SAFETY".to_string(),
            unit_price: 199.00,
            currency: "ZAR".to_string(),
            price_tiers: vec![
                PriceTier { min_quantity: 1, max_quantity: Some(9), unit_price: 199.00 },
                PriceTier { min_quantity: 10, max_quantity: Some(49), unit_price: 189.00 },
                PriceTier { min_quantity: 50, max_quantity: None, unit_price: 179.00 },
            ],
            vat_inclusive: true,
            vat_rate: 15.0,
            unit_of_measure: UnitOfMeasure { code: "BOX".to_string(), name: "Box".to_string(), symbol: "box".to_string() },
            supplier: CatalogueSupplier {
                id: "SUP-008".to_string(),
                name: "Eastern Cape Medical Supplies".to_string(),
                bbbee_level: 4,
                lead_time_days: 5,
                minimum_order_quantity: 1,
                is_preferred: false,
            },
            alternate_suppliers: vec![],
            specifications: vec![
                ItemSpecification { key: "Material".to_string(), value: "Nitrile".to_string(), unit: None },
                ItemSpecification { key: "Powder".to_string(), value: "Powder-free".to_string(), unit: None },
                ItemSpecification { key: "Sizes".to_string(), value: "S, M, L, XL".to_string(), unit: None },
                ItemSpecification { key: "Quantity".to_string(), value: "100".to_string(), unit: Some("gloves/box".to_string()) },
            ],
            brand: Some("MedSafe".to_string()),
            manufacturer: Some("EC Medical Supplies".to_string()),
            model_number: None,
            image_url: Some("/images/catalogue/gloves.jpg".to_string()),
            thumbnail_url: Some("/images/catalogue/gloves-thumb.jpg".to_string()),
            datasheet_url: None,
            in_stock: true,
            stock_quantity: Some(500),
            reorder_level: Some(100),
            status: CatalogueItemStatus::Active,
            created_at: "2024-06-01".to_string(),
            updated_at: "2025-02-08".to_string(),
            tags: vec!["gloves".to_string(), "ppe".to_string(), "nitrile".to_string(), "disposable".to_string()],
            featured: false,
        },
        CatalogueItem {
            id: "ITEM-012".to_string(),
            item_code: "IT-KEY-001".to_string(),
            name: "Wireless Keyboard and Mouse Combo".to_string(),
            description: "Wireless keyboard and mouse set with USB receiver".to_string(),
            category_id: "CAT-001".to_string(),
            category_name: "IT Equipment".to_string(),
            category_code: "IT".to_string(),
            unit_price: 699.00,
            currency: "ZAR".to_string(),
            price_tiers: vec![],
            vat_inclusive: true,
            vat_rate: 15.0,
            unit_of_measure: UnitOfMeasure { code: "SET".to_string(), name: "Set".to_string(), symbol: "set".to_string() },
            supplier: CatalogueSupplier {
                id: "SUP-001".to_string(),
                name: "TechSolutions SA (Pty) Ltd".to_string(),
                bbbee_level: 1,
                lead_time_days: 3,
                minimum_order_quantity: 1,
                is_preferred: true,
            },
            alternate_suppliers: vec![],
            specifications: vec![
                ItemSpecification { key: "Connectivity".to_string(), value: "2.4GHz Wireless".to_string(), unit: None },
                ItemSpecification { key: "Battery Life".to_string(), value: "12".to_string(), unit: Some("months".to_string()) },
                ItemSpecification { key: "Keyboard Layout".to_string(), value: "QWERTY".to_string(), unit: None },
            ],
            brand: Some("Logitech".to_string()),
            manufacturer: Some("Logitech".to_string()),
            model_number: Some("MK270".to_string()),
            image_url: Some("/images/catalogue/keyboard.jpg".to_string()),
            thumbnail_url: Some("/images/catalogue/keyboard-thumb.jpg".to_string()),
            datasheet_url: None,
            in_stock: true,
            stock_quantity: Some(65),
            reorder_level: Some(20),
            status: CatalogueItemStatus::Active,
            created_at: "2024-07-15".to_string(),
            updated_at: "2025-01-30".to_string(),
            tags: vec!["keyboard".to_string(), "mouse".to_string(), "wireless".to_string(), "logitech".to_string()],
            featured: false,
        },
    ];

    store.items.set(mock_items.clone());
    store.categories.set(categories);

    // Calculate KPIs
    let total = mock_items.len() as u32;
    let active = mock_items.iter().filter(|i| i.status == CatalogueItemStatus::Active).count() as u32;
    let out_of_stock = mock_items.iter().filter(|i| !i.in_stock).count() as u32;
    let pending = mock_items.iter().filter(|i| i.status == CatalogueItemStatus::PendingApproval).count() as u32;
    let avg_price: f64 = mock_items.iter().map(|i| i.unit_price).sum::<f64>() / total as f64;

    let unique_suppliers: std::collections::HashSet<_> = mock_items.iter().map(|i| &i.supplier.id).collect();

    store.kpis.set(CatalogueKpis {
        total_items: total,
        active_items: active,
        categories_count: 6,
        suppliers_count: unique_suppliers.len() as u32,
        out_of_stock,
        pending_approval: pending,
        avg_price,
    });

    store.pagination.set(PaginationState {
        current_page: 1,
        page_size: 12,
        total_items: total,
        total_pages: ((total as f32) / 12.0).ceil() as u32,
    });
}

/// Select an item by ID
pub fn select_item(store: &CatalogueStore, item_id: &str) {
    let item = store.items.get().iter()
        .find(|i| i.id == item_id)
        .cloned();
    store.selected.set(item);
}

/// Clear selection
pub fn clear_selection(store: &CatalogueStore) {
    store.selected.set(None);
}
