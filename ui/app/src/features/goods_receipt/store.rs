//! Goods Receipt store

use components::prelude::*;
use super::types::{
    GoodsReceipt, GoodsReceiptSummary, GoodsReceiptFilter, GoodsReceiptStatus,
    InspectionStatus, ReceivedItem, GoodsReceiptSupplier, POReference,
};

/// Goods Receipt state store
#[derive(Clone)]
pub struct GoodsReceiptStore {
    pub receipts: Signal<Vec<GoodsReceiptSummary>>,
    pub selected: Signal<Option<GoodsReceipt>>,
    pub filter: Signal<GoodsReceiptFilter>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
    pub saving: Signal<bool>,
}

impl GoodsReceiptStore {
    pub fn new() -> Self {
        Self {
            receipts: signal(Vec::new()),
            selected: signal(None),
            filter: signal(GoodsReceiptFilter::default()),
            loading: signal(false),
            error: signal(None),
            saving: signal(false),
        }
    }

    /// Update filter status
    pub fn set_filter_status(&self, status: Option<GoodsReceiptStatus>) {
        let mut filter = self.filter.get();
        filter.status = status;
        self.filter.set(filter);
    }

    /// Update filter inspection status
    pub fn set_filter_inspection(&self, status: Option<InspectionStatus>) {
        let mut filter = self.filter.get();
        filter.inspection_status = status;
        self.filter.set(filter);
    }

    /// Update filter supplier
    pub fn set_filter_supplier(&self, supplier_id: Option<String>) {
        let mut filter = self.filter.get();
        filter.supplier_id = supplier_id;
        self.filter.set(filter);
    }

    /// Update filter search
    pub fn set_filter_search(&self, search: Option<String>) {
        let mut filter = self.filter.get();
        filter.search = search;
        self.filter.set(filter);
    }

    /// Update filter date range
    pub fn set_filter_date_range(&self, from: Option<String>, to: Option<String>) {
        let mut filter = self.filter.get();
        filter.date_from = from;
        filter.date_to = to;
        self.filter.set(filter);
    }

    /// Update filter received by
    pub fn set_filter_received_by(&self, received_by: Option<String>) {
        let mut filter = self.filter.get();
        filter.received_by = received_by;
        self.filter.set(filter);
    }

    /// Clear all filters
    pub fn clear_filters(&self) {
        self.filter.set(GoodsReceiptFilter::default());
    }

    /// Get filtered receipts
    pub fn get_filtered_receipts(&self) -> Vec<GoodsReceiptSummary> {
        let receipts = self.receipts.get();
        let filter = self.filter.get();

        receipts
            .iter()
            .filter(|r| {
                // Filter by status
                if let Some(status) = &filter.status {
                    if r.status != *status {
                        return false;
                    }
                }

                // Filter by inspection status
                if let Some(inspection) = &filter.inspection_status {
                    if r.inspection_status != *inspection {
                        return false;
                    }
                }

                // Filter by supplier
                if let Some(supplier_id) = &filter.supplier_id {
                    if !supplier_id.is_empty()
                        && !r.supplier_name.to_lowercase().contains(&supplier_id.to_lowercase())
                    {
                        return false;
                    }
                }

                // Filter by search
                if let Some(search) = &filter.search {
                    if !search.is_empty() {
                        let search_lower = search.to_lowercase();
                        if !r.id.to_lowercase().contains(&search_lower)
                            && !r.po_number.to_lowercase().contains(&search_lower)
                            && !r.supplier_name.to_lowercase().contains(&search_lower)
                            && !r.received_by.to_lowercase().contains(&search_lower)
                        {
                            return false;
                        }
                    }
                }

                // Filter by received by
                if let Some(received_by) = &filter.received_by {
                    if !received_by.is_empty()
                        && !r.received_by.to_lowercase().contains(&received_by.to_lowercase())
                    {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect()
    }

    /// Get pending receipts (awaiting processing)
    pub fn get_pending_receipts(&self) -> Vec<GoodsReceiptSummary> {
        self.receipts
            .get()
            .iter()
            .filter(|r| {
                r.status == GoodsReceiptStatus::Pending
                    || r.status == GoodsReceiptStatus::Draft
                    || r.status == GoodsReceiptStatus::PartiallyReceived
            })
            .cloned()
            .collect()
    }

    /// Get completed receipts
    pub fn get_completed_receipts(&self) -> Vec<GoodsReceiptSummary> {
        self.receipts
            .get()
            .iter()
            .filter(|r| r.status == GoodsReceiptStatus::Completed)
            .cloned()
            .collect()
    }
}

/// Load mock goods receipt data for demo
pub fn load_mock_receipts(store: &GoodsReceiptStore) {
    let receipts = vec![
        GoodsReceiptSummary {
            id: "GR-2025-0089".to_string(),
            po_number: "PO-2025-0234".to_string(),
            supplier_name: "TechSolutions SA (Pty) Ltd".to_string(),
            supplier_bbbee_level: 1,
            total_items: 5,
            total_value: 125_000.0,
            receipt_date: "2025-02-27".to_string(),
            status: GoodsReceiptStatus::Pending,
            inspection_status: InspectionStatus::Pending,
            received_by: "John Smith".to_string(),
            completion_percentage: 0.0,
        },
        GoodsReceiptSummary {
            id: "GR-2025-0088".to_string(),
            po_number: "PO-2025-0198".to_string(),
            supplier_name: "Office Pro Distributors".to_string(),
            supplier_bbbee_level: 2,
            total_items: 12,
            total_value: 45_600.0,
            receipt_date: "2025-02-26".to_string(),
            status: GoodsReceiptStatus::PartiallyReceived,
            inspection_status: InspectionStatus::InProgress,
            received_by: "Jane Doe".to_string(),
            completion_percentage: 75.0,
        },
        GoodsReceiptSummary {
            id: "GR-2025-0087".to_string(),
            po_number: "PO-2025-0156".to_string(),
            supplier_name: "Industrial Supplies Co".to_string(),
            supplier_bbbee_level: 1,
            total_items: 8,
            total_value: 89_200.0,
            receipt_date: "2025-02-25".to_string(),
            status: GoodsReceiptStatus::Completed,
            inspection_status: InspectionStatus::Passed,
            received_by: "Mike Wilson".to_string(),
            completion_percentage: 100.0,
        },
        GoodsReceiptSummary {
            id: "GR-2025-0086".to_string(),
            po_number: "PO-2025-0142".to_string(),
            supplier_name: "SafetyFirst Equipment".to_string(),
            supplier_bbbee_level: 2,
            total_items: 3,
            total_value: 156_800.0,
            receipt_date: "2025-02-24".to_string(),
            status: GoodsReceiptStatus::Completed,
            inspection_status: InspectionStatus::Passed,
            received_by: "Sarah Johnson".to_string(),
            completion_percentage: 100.0,
        },
        GoodsReceiptSummary {
            id: "GR-2025-0085".to_string(),
            po_number: "PO-2025-0128".to_string(),
            supplier_name: "Green Energy Solutions".to_string(),
            supplier_bbbee_level: 1,
            total_items: 15,
            total_value: 234_500.0,
            receipt_date: "2025-02-23".to_string(),
            status: GoodsReceiptStatus::Pending,
            inspection_status: InspectionStatus::Pending,
            received_by: "John Smith".to_string(),
            completion_percentage: 0.0,
        },
        GoodsReceiptSummary {
            id: "GR-2025-0084".to_string(),
            po_number: "PO-2025-0115".to_string(),
            supplier_name: "CleanCorp Services".to_string(),
            supplier_bbbee_level: 2,
            total_items: 20,
            total_value: 32_000.0,
            receipt_date: "2025-02-22".to_string(),
            status: GoodsReceiptStatus::Completed,
            inspection_status: InspectionStatus::Passed,
            received_by: "Jane Doe".to_string(),
            completion_percentage: 100.0,
        },
        GoodsReceiptSummary {
            id: "GR-2025-0083".to_string(),
            po_number: "PO-2025-0098".to_string(),
            supplier_name: "MedEquip Suppliers".to_string(),
            supplier_bbbee_level: 1,
            total_items: 6,
            total_value: 445_000.0,
            receipt_date: "2025-02-21".to_string(),
            status: GoodsReceiptStatus::PartiallyReceived,
            inspection_status: InspectionStatus::PartialPass,
            received_by: "Mike Wilson".to_string(),
            completion_percentage: 60.0,
        },
        GoodsReceiptSummary {
            id: "GR-2025-0082".to_string(),
            po_number: "PO-2025-0076".to_string(),
            supplier_name: "IT Hardware Direct".to_string(),
            supplier_bbbee_level: 3,
            total_items: 10,
            total_value: 178_900.0,
            receipt_date: "2025-02-20".to_string(),
            status: GoodsReceiptStatus::Rejected,
            inspection_status: InspectionStatus::Failed,
            received_by: "Sarah Johnson".to_string(),
            completion_percentage: 100.0,
        },
    ];

    store.receipts.set(receipts);
}

/// Load mock goods receipt details
pub fn get_mock_receipt(id: &str) -> Option<GoodsReceipt> {
    match id {
        "GR-2025-0089" => Some(GoodsReceipt {
            id: "GR-2025-0089".to_string(),
            po_reference: POReference {
                po_number: "PO-2025-0234".to_string(),
                po_date: "2025-02-20".to_string(),
                contract_id: Some("CTR-2025-0234".to_string()),
                delivery_date: "2025-02-27".to_string(),
                total_value: 125_000.0,
            },
            supplier: GoodsReceiptSupplier {
                id: "SUP-001".to_string(),
                name: "TechSolutions SA (Pty) Ltd".to_string(),
                contact_person: Some("David Nkosi".to_string()),
                contact_phone: Some("+27 11 234 5678".to_string()),
                bbbee_level: 1,
            },
            items_received: vec![
                ReceivedItem {
                    id: "ITEM-001".to_string(),
                    item_code: "LAP-DEL-001".to_string(),
                    description: "Dell Latitude 5540 Laptop".to_string(),
                    ordered_quantity: 10,
                    received_quantity: 0,
                    accepted_quantity: 0,
                    rejected_quantity: 0,
                    unit: "Each".to_string(),
                    unit_price: 18_500.0,
                    inspection_status: InspectionStatus::Pending,
                    inspection_notes: None,
                    batch_number: None,
                    serial_numbers: Vec::new(),
                    storage_location: None,
                },
                ReceivedItem {
                    id: "ITEM-002".to_string(),
                    item_code: "MON-DEL-001".to_string(),
                    description: "Dell 27\" Monitor P2722H".to_string(),
                    ordered_quantity: 10,
                    received_quantity: 0,
                    accepted_quantity: 0,
                    rejected_quantity: 0,
                    unit: "Each".to_string(),
                    unit_price: 5_200.0,
                    inspection_status: InspectionStatus::Pending,
                    inspection_notes: None,
                    batch_number: None,
                    serial_numbers: Vec::new(),
                    storage_location: None,
                },
                ReceivedItem {
                    id: "ITEM-003".to_string(),
                    item_code: "DOC-STN-001".to_string(),
                    description: "Dell Docking Station WD19".to_string(),
                    ordered_quantity: 10,
                    received_quantity: 0,
                    accepted_quantity: 0,
                    rejected_quantity: 0,
                    unit: "Each".to_string(),
                    unit_price: 3_800.0,
                    inspection_status: InspectionStatus::Pending,
                    inspection_notes: None,
                    batch_number: None,
                    serial_numbers: Vec::new(),
                    storage_location: None,
                },
                ReceivedItem {
                    id: "ITEM-004".to_string(),
                    item_code: "KEY-LOG-001".to_string(),
                    description: "Logitech MX Keys Keyboard".to_string(),
                    ordered_quantity: 10,
                    received_quantity: 0,
                    accepted_quantity: 0,
                    rejected_quantity: 0,
                    unit: "Each".to_string(),
                    unit_price: 1_850.0,
                    inspection_status: InspectionStatus::Pending,
                    inspection_notes: None,
                    batch_number: None,
                    serial_numbers: Vec::new(),
                    storage_location: None,
                },
                ReceivedItem {
                    id: "ITEM-005".to_string(),
                    item_code: "MOU-LOG-001".to_string(),
                    description: "Logitech MX Master 3 Mouse".to_string(),
                    ordered_quantity: 10,
                    received_quantity: 0,
                    accepted_quantity: 0,
                    rejected_quantity: 0,
                    unit: "Each".to_string(),
                    unit_price: 1_650.0,
                    inspection_status: InspectionStatus::Pending,
                    inspection_notes: None,
                    batch_number: None,
                    serial_numbers: Vec::new(),
                    storage_location: None,
                },
            ],
            inspection_status: InspectionStatus::Pending,
            received_by: "John Smith".to_string(),
            status: GoodsReceiptStatus::Pending,
            receipt_date: "2025-02-27".to_string(),
            delivery_note_number: Some("DN-2025-0456".to_string()),
            invoice_number: None,
            warehouse_location: "Main Warehouse - Bay A".to_string(),
            notes: Some("Delivery expected by 10:00 AM".to_string()),
            documents: vec!["delivery_note.pdf".to_string(), "packing_list.pdf".to_string()],
            created_at: "2025-02-27T08:00:00Z".to_string(),
            updated_at: "2025-02-27T08:00:00Z".to_string(),
            completed_at: None,
            completed_by: None,
        }),
        "GR-2025-0087" => Some(GoodsReceipt {
            id: "GR-2025-0087".to_string(),
            po_reference: POReference {
                po_number: "PO-2025-0156".to_string(),
                po_date: "2025-02-15".to_string(),
                contract_id: Some("CTR-2025-0156".to_string()),
                delivery_date: "2025-02-25".to_string(),
                total_value: 89_200.0,
            },
            supplier: GoodsReceiptSupplier {
                id: "SUP-003".to_string(),
                name: "Industrial Supplies Co".to_string(),
                contact_person: Some("Thabo Molefe".to_string()),
                contact_phone: Some("+27 12 345 6789".to_string()),
                bbbee_level: 1,
            },
            items_received: vec![
                ReceivedItem {
                    id: "ITEM-001".to_string(),
                    item_code: "PPE-HEL-001".to_string(),
                    description: "Safety Helmet - White".to_string(),
                    ordered_quantity: 100,
                    received_quantity: 100,
                    accepted_quantity: 100,
                    rejected_quantity: 0,
                    unit: "Each".to_string(),
                    unit_price: 185.0,
                    inspection_status: InspectionStatus::Passed,
                    inspection_notes: Some("All items meet SABS standards".to_string()),
                    batch_number: Some("BATCH-2025-001".to_string()),
                    serial_numbers: Vec::new(),
                    storage_location: Some("PPE Storage - Shelf B2".to_string()),
                },
                ReceivedItem {
                    id: "ITEM-002".to_string(),
                    item_code: "PPE-GLV-001".to_string(),
                    description: "Safety Gloves - Large".to_string(),
                    ordered_quantity: 200,
                    received_quantity: 200,
                    accepted_quantity: 200,
                    rejected_quantity: 0,
                    unit: "Pair".to_string(),
                    unit_price: 125.0,
                    inspection_status: InspectionStatus::Passed,
                    inspection_notes: Some("All items meet quality standards".to_string()),
                    batch_number: Some("BATCH-2025-002".to_string()),
                    serial_numbers: Vec::new(),
                    storage_location: Some("PPE Storage - Shelf B3".to_string()),
                },
            ],
            inspection_status: InspectionStatus::Passed,
            received_by: "Mike Wilson".to_string(),
            status: GoodsReceiptStatus::Completed,
            receipt_date: "2025-02-25".to_string(),
            delivery_note_number: Some("DN-2025-0412".to_string()),
            invoice_number: Some("INV-2025-0156".to_string()),
            warehouse_location: "Main Warehouse - Bay C".to_string(),
            notes: Some("All items received in good condition".to_string()),
            documents: vec![
                "delivery_note.pdf".to_string(),
                "invoice.pdf".to_string(),
                "quality_certificate.pdf".to_string(),
            ],
            created_at: "2025-02-25T09:30:00Z".to_string(),
            updated_at: "2025-02-25T14:45:00Z".to_string(),
            completed_at: Some("2025-02-25T14:45:00Z".to_string()),
            completed_by: Some("Mike Wilson".to_string()),
        }),
        _ => None,
    }
}
