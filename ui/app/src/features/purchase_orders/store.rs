//! Purchase Orders store

use components::prelude::*;
use super::types::{
    PurchaseOrder, PurchaseOrderSummary, PurchaseOrderFilter, PurchaseOrderStatus,
    LineItem, DeliveryAddress, Supplier,
};

/// Purchase Orders state store
#[derive(Clone)]
pub struct PurchaseOrdersStore {
    pub purchase_orders: Signal<Vec<PurchaseOrderSummary>>,
    pub selected: Signal<Option<PurchaseOrder>>,
    pub filter: Signal<PurchaseOrderFilter>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
    pub saving: Signal<bool>,
}

impl PurchaseOrdersStore {
    pub fn new() -> Self {
        Self {
            purchase_orders: signal(Vec::new()),
            selected: signal(None),
            filter: signal(PurchaseOrderFilter::default()),
            loading: signal(false),
            error: signal(None),
            saving: signal(false),
        }
    }

    /// Update filter status
    pub fn set_filter_status(&self, status: Option<PurchaseOrderStatus>) {
        let mut filter = self.filter.get();
        filter.status = status;
        self.filter.set(filter);
    }

    /// Update filter supplier
    pub fn set_filter_supplier(&self, supplier_id: Option<String>) {
        let mut filter = self.filter.get();
        filter.supplier_id = supplier_id;
        self.filter.set(filter);
    }

    /// Update filter contract reference
    pub fn set_filter_contract(&self, contract_ref: Option<String>) {
        let mut filter = self.filter.get();
        filter.contract_ref = contract_ref;
        self.filter.set(filter);
    }

    /// Update filter date range
    pub fn set_filter_date_range(&self, date_from: Option<String>, date_to: Option<String>) {
        let mut filter = self.filter.get();
        filter.date_from = date_from;
        filter.date_to = date_to;
        self.filter.set(filter);
    }

    /// Update filter search
    pub fn set_filter_search(&self, search: Option<String>) {
        let mut filter = self.filter.get();
        filter.search = search;
        self.filter.set(filter);
    }

    /// Update filter for pending delivery
    pub fn set_filter_pending_delivery(&self, pending: Option<bool>) {
        let mut filter = self.filter.get();
        filter.pending_delivery = pending;
        self.filter.set(filter);
    }

    /// Clear all filters
    pub fn clear_filters(&self) {
        self.filter.set(PurchaseOrderFilter::default());
    }

    /// Get filtered purchase orders
    pub fn get_filtered_purchase_orders(&self) -> Vec<PurchaseOrderSummary> {
        let purchase_orders = self.purchase_orders.get();
        let filter = self.filter.get();

        purchase_orders
            .iter()
            .filter(|po| {
                // Filter by status
                if let Some(status) = &filter.status {
                    if po.status != *status {
                        return false;
                    }
                }

                // Filter by supplier
                if let Some(supplier_id) = &filter.supplier_id {
                    if !supplier_id.is_empty() && !po.supplier_name.to_lowercase().contains(&supplier_id.to_lowercase()) {
                        return false;
                    }
                }

                // Filter by contract reference
                if let Some(contract_ref) = &filter.contract_ref {
                    if !contract_ref.is_empty() {
                        match &po.contract_ref {
                            Some(ref cr) if cr.to_lowercase().contains(&contract_ref.to_lowercase()) => {}
                            _ => return false,
                        }
                    }
                }

                // Filter by search
                if let Some(search) = &filter.search {
                    if !search.is_empty() {
                        let search_lower = search.to_lowercase();
                        if !po.po_number.to_lowercase().contains(&search_lower)
                            && !po.supplier_name.to_lowercase().contains(&search_lower)
                            && !po.id.to_lowercase().contains(&search_lower)
                            && !po.contract_ref.as_ref().map(|c| c.to_lowercase().contains(&search_lower)).unwrap_or(false)
                        {
                            return false;
                        }
                    }
                }

                // Filter by pending delivery
                if let Some(pending) = filter.pending_delivery {
                    if pending {
                        let is_pending = matches!(
                            po.status,
                            PurchaseOrderStatus::Sent
                                | PurchaseOrderStatus::Acknowledged
                                | PurchaseOrderStatus::PartiallyDelivered
                        );
                        if !is_pending {
                            return false;
                        }
                    }
                }

                true
            })
            .cloned()
            .collect()
    }

    /// Get count by status
    pub fn get_count_by_status(&self, status: PurchaseOrderStatus) -> usize {
        self.purchase_orders
            .get()
            .iter()
            .filter(|po| po.status == status)
            .count()
    }

    /// Get total value of all POs
    pub fn get_total_value(&self) -> f64 {
        self.purchase_orders
            .get()
            .iter()
            .map(|po| po.total_amount)
            .sum()
    }
}

/// Load mock purchase orders data for demo
pub fn load_mock_purchase_orders(store: &PurchaseOrdersStore) {
    let purchase_orders = vec![
        PurchaseOrderSummary {
            id: "PO-2025-0456".to_string(),
            po_number: "PO-2025-0456".to_string(),
            contract_ref: Some("CTR-2025-0234".to_string()),
            supplier_name: "TechSolutions SA (Pty) Ltd".to_string(),
            supplier_bbbee_level: 1,
            total_amount: 1_250_000.0,
            currency: "ZAR".to_string(),
            status: PurchaseOrderStatus::Delivered,
            order_date: "2025-02-01".to_string(),
            expected_delivery_date: "2025-02-15".to_string(),
            delivery_progress: 100.0,
            line_item_count: 5,
        },
        PurchaseOrderSummary {
            id: "PO-2025-0455".to_string(),
            po_number: "PO-2025-0455".to_string(),
            contract_ref: Some("CTR-2025-0198".to_string()),
            supplier_name: "Office Pro Distributors".to_string(),
            supplier_bbbee_level: 2,
            total_amount: 85_500.0,
            currency: "ZAR".to_string(),
            status: PurchaseOrderStatus::Acknowledged,
            order_date: "2025-02-20".to_string(),
            expected_delivery_date: "2025-03-05".to_string(),
            delivery_progress: 0.0,
            line_item_count: 12,
        },
        PurchaseOrderSummary {
            id: "PO-2025-0454".to_string(),
            po_number: "PO-2025-0454".to_string(),
            contract_ref: None,
            supplier_name: "SecureGuard Holdings".to_string(),
            supplier_bbbee_level: 1,
            total_amount: 350_000.0,
            currency: "ZAR".to_string(),
            status: PurchaseOrderStatus::Sent,
            order_date: "2025-02-22".to_string(),
            expected_delivery_date: "2025-03-01".to_string(),
            delivery_progress: 0.0,
            line_item_count: 3,
        },
        PurchaseOrderSummary {
            id: "PO-2025-0453".to_string(),
            po_number: "PO-2025-0453".to_string(),
            contract_ref: Some("CTR-2025-0089".to_string()),
            supplier_name: "AutoCare Fleet Management".to_string(),
            supplier_bbbee_level: 3,
            total_amount: 125_800.0,
            currency: "ZAR".to_string(),
            status: PurchaseOrderStatus::PartiallyDelivered,
            order_date: "2025-02-10".to_string(),
            expected_delivery_date: "2025-02-28".to_string(),
            delivery_progress: 60.0,
            line_item_count: 8,
        },
        PurchaseOrderSummary {
            id: "PO-2025-0452".to_string(),
            po_number: "PO-2025-0452".to_string(),
            contract_ref: None,
            supplier_name: "Gourmet Corporate Catering".to_string(),
            supplier_bbbee_level: 2,
            total_amount: 45_000.0,
            currency: "ZAR".to_string(),
            status: PurchaseOrderStatus::Invoiced,
            order_date: "2025-02-05".to_string(),
            expected_delivery_date: "2025-02-10".to_string(),
            delivery_progress: 100.0,
            line_item_count: 1,
        },
        PurchaseOrderSummary {
            id: "PO-2025-0451".to_string(),
            po_number: "PO-2025-0451".to_string(),
            contract_ref: Some("CTR-2025-0301".to_string()),
            supplier_name: "CloudFirst SA".to_string(),
            supplier_bbbee_level: 1,
            total_amount: 620_000.0,
            currency: "ZAR".to_string(),
            status: PurchaseOrderStatus::PendingApproval,
            order_date: "2025-02-25".to_string(),
            expected_delivery_date: "2025-03-15".to_string(),
            delivery_progress: 0.0,
            line_item_count: 4,
        },
        PurchaseOrderSummary {
            id: "PO-2025-0450".to_string(),
            po_number: "PO-2025-0450".to_string(),
            contract_ref: None,
            supplier_name: "Skills Development Academy".to_string(),
            supplier_bbbee_level: 1,
            total_amount: 180_000.0,
            currency: "ZAR".to_string(),
            status: PurchaseOrderStatus::Draft,
            order_date: "2025-02-26".to_string(),
            expected_delivery_date: "2025-04-01".to_string(),
            delivery_progress: 0.0,
            line_item_count: 2,
        },
        PurchaseOrderSummary {
            id: "PO-2025-0449".to_string(),
            po_number: "PO-2025-0449".to_string(),
            contract_ref: Some("CTR-2024-0123".to_string()),
            supplier_name: "CleanCorp Services".to_string(),
            supplier_bbbee_level: 2,
            total_amount: 175_000.0,
            currency: "ZAR".to_string(),
            status: PurchaseOrderStatus::Closed,
            order_date: "2025-01-15".to_string(),
            expected_delivery_date: "2025-02-01".to_string(),
            delivery_progress: 100.0,
            line_item_count: 6,
        },
    ];

    store.purchase_orders.set(purchase_orders);
}

/// Load mock purchase order details
pub fn get_mock_purchase_order(id: &str) -> Option<PurchaseOrder> {
    match id {
        "PO-2025-0456" => Some(PurchaseOrder {
            id: "PO-2025-0456".to_string(),
            po_number: "PO-2025-0456".to_string(),
            contract_ref: Some("CTR-2025-0234".to_string()),
            requisition_ref: Some("REQ-2025-0089".to_string()),
            tender_ref: Some("TND-2024-0089".to_string()),
            supplier: Supplier {
                id: "SUP-001".to_string(),
                name: "TechSolutions SA (Pty) Ltd".to_string(),
                registration_number: "2015/123456/07".to_string(),
                tax_number: "4123456789".to_string(),
                bbbee_level: 1,
                contact_person: "James Ndlovu".to_string(),
                contact_email: "james.ndlovu@techsolutions.co.za".to_string(),
                contact_phone: "+27 11 555 1234".to_string(),
                address: "123 Tech Park, Sandton, Gauteng, 2196".to_string(),
            },
            line_items: vec![
                LineItem {
                    id: "LI-001".to_string(),
                    item_code: "HW-SVR-001".to_string(),
                    description: "Dell PowerEdge R750 Server".to_string(),
                    quantity: 5,
                    unit: "Each".to_string(),
                    unit_price: 185_000.0,
                    total_price: 925_000.0,
                    tax_rate: 15.0,
                    tax_amount: 138_750.0,
                    delivery_date: "2025-02-15".to_string(),
                    delivered_quantity: 5,
                    notes: None,
                },
                LineItem {
                    id: "LI-002".to_string(),
                    item_code: "HW-STO-001".to_string(),
                    description: "NetApp Storage Array".to_string(),
                    quantity: 2,
                    unit: "Each".to_string(),
                    unit_price: 95_000.0,
                    total_price: 190_000.0,
                    tax_rate: 15.0,
                    tax_amount: 28_500.0,
                    delivery_date: "2025-02-15".to_string(),
                    delivered_quantity: 2,
                    notes: None,
                },
                LineItem {
                    id: "LI-003".to_string(),
                    item_code: "SW-LIC-001".to_string(),
                    description: "VMware vSphere Enterprise Plus License".to_string(),
                    quantity: 10,
                    unit: "License".to_string(),
                    unit_price: 8_500.0,
                    total_price: 85_000.0,
                    tax_rate: 15.0,
                    tax_amount: 12_750.0,
                    delivery_date: "2025-02-10".to_string(),
                    delivered_quantity: 10,
                    notes: Some("License keys delivered via email".to_string()),
                },
                LineItem {
                    id: "LI-004".to_string(),
                    item_code: "SVC-INS-001".to_string(),
                    description: "Installation and Configuration Services".to_string(),
                    quantity: 40,
                    unit: "Hours".to_string(),
                    unit_price: 1_250.0,
                    total_price: 50_000.0,
                    tax_rate: 15.0,
                    tax_amount: 7_500.0,
                    delivery_date: "2025-02-28".to_string(),
                    delivered_quantity: 40,
                    notes: None,
                },
            ],
            delivery_address: DeliveryAddress {
                address_line1: "Government Data Centre".to_string(),
                address_line2: Some("Block B, Level 3".to_string()),
                city: "Pretoria".to_string(),
                province: "Gauteng".to_string(),
                postal_code: "0001".to_string(),
                country: "South Africa".to_string(),
                contact_person: "Thabo Molefe".to_string(),
                contact_phone: "+27 12 555 9876".to_string(),
                contact_email: "thabo.molefe@gov.za".to_string(),
                delivery_instructions: Some("Delivery must be during office hours (08:00-16:00). Security clearance required.".to_string()),
            },
            status: PurchaseOrderStatus::Delivered,
            subtotal: 1_250_000.0,
            tax_total: 187_500.0,
            total_amount: 1_437_500.0,
            currency: "ZAR".to_string(),
            payment_terms: "30 days from invoice".to_string(),
            order_date: "2025-02-01".to_string(),
            expected_delivery_date: "2025-02-15".to_string(),
            actual_delivery_date: Some("2025-02-14".to_string()),
            notes: Some("Urgent order for data centre upgrade project".to_string()),
            internal_notes: Some("Part of digital transformation initiative - priority delivery".to_string()),
            attachments: vec![
                "technical_specifications.pdf".to_string(),
                "delivery_schedule.xlsx".to_string(),
                "supplier_quote.pdf".to_string(),
            ],
            created_by: "Sarah Johnson".to_string(),
            created_at: "2025-01-28T09:30:00Z".to_string(),
            updated_at: "2025-02-14T16:45:00Z".to_string(),
            approved_by: Some("Michael Chen".to_string()),
            approved_at: Some("2025-01-30T11:00:00Z".to_string()),
            sent_at: Some("2025-02-01T08:00:00Z".to_string()),
            acknowledged_at: Some("2025-02-01T10:30:00Z".to_string()),
        }),
        "PO-2025-0453" => Some(PurchaseOrder {
            id: "PO-2025-0453".to_string(),
            po_number: "PO-2025-0453".to_string(),
            contract_ref: Some("CTR-2025-0089".to_string()),
            requisition_ref: Some("REQ-2025-0056".to_string()),
            tender_ref: None,
            supplier: Supplier {
                id: "SUP-004".to_string(),
                name: "AutoCare Fleet Management".to_string(),
                registration_number: "2018/789012/07".to_string(),
                tax_number: "4567890123".to_string(),
                bbbee_level: 3,
                contact_person: "Pieter van der Berg".to_string(),
                contact_email: "pieter@autocare.co.za".to_string(),
                contact_phone: "+27 11 555 4567".to_string(),
                address: "456 Industrial Road, Midrand, Gauteng, 1685".to_string(),
            },
            line_items: vec![
                LineItem {
                    id: "LI-001".to_string(),
                    item_code: "SVC-FSC-001".to_string(),
                    description: "Full Vehicle Service - Sedan".to_string(),
                    quantity: 10,
                    unit: "Service".to_string(),
                    unit_price: 4_500.0,
                    total_price: 45_000.0,
                    tax_rate: 15.0,
                    tax_amount: 6_750.0,
                    delivery_date: "2025-02-28".to_string(),
                    delivered_quantity: 6,
                    notes: None,
                },
                LineItem {
                    id: "LI-002".to_string(),
                    item_code: "SVC-FSC-002".to_string(),
                    description: "Full Vehicle Service - SUV".to_string(),
                    quantity: 5,
                    unit: "Service".to_string(),
                    unit_price: 6_200.0,
                    total_price: 31_000.0,
                    tax_rate: 15.0,
                    tax_amount: 4_650.0,
                    delivery_date: "2025-02-28".to_string(),
                    delivered_quantity: 3,
                    notes: None,
                },
                LineItem {
                    id: "LI-003".to_string(),
                    item_code: "PRT-TYR-001".to_string(),
                    description: "Replacement Tyres (Set of 4)".to_string(),
                    quantity: 8,
                    unit: "Set".to_string(),
                    unit_price: 4_800.0,
                    total_price: 38_400.0,
                    tax_rate: 15.0,
                    tax_amount: 5_760.0,
                    delivery_date: "2025-02-20".to_string(),
                    delivered_quantity: 8,
                    notes: Some("Continental tyres as specified".to_string()),
                },
            ],
            delivery_address: DeliveryAddress {
                address_line1: "Government Fleet Depot".to_string(),
                address_line2: Some("Workshop Area".to_string()),
                city: "Centurion".to_string(),
                province: "Gauteng".to_string(),
                postal_code: "0157".to_string(),
                country: "South Africa".to_string(),
                contact_person: "David Mthembu".to_string(),
                contact_phone: "+27 12 555 3456".to_string(),
                contact_email: "david.mthembu@gov.za".to_string(),
                delivery_instructions: Some("Vehicles to be collected from various departments. Schedule to be coordinated with fleet manager.".to_string()),
            },
            status: PurchaseOrderStatus::PartiallyDelivered,
            subtotal: 114_400.0,
            tax_total: 17_160.0,
            total_amount: 131_560.0,
            currency: "ZAR".to_string(),
            payment_terms: "30 days from invoice".to_string(),
            order_date: "2025-02-10".to_string(),
            expected_delivery_date: "2025-02-28".to_string(),
            actual_delivery_date: None,
            notes: Some("Quarterly fleet maintenance order".to_string()),
            internal_notes: None,
            attachments: vec![
                "fleet_service_schedule.pdf".to_string(),
                "vehicle_list.xlsx".to_string(),
            ],
            created_by: "Linda Nkosi".to_string(),
            created_at: "2025-02-08T14:00:00Z".to_string(),
            updated_at: "2025-02-20T11:30:00Z".to_string(),
            approved_by: Some("Robert Williams".to_string()),
            approved_at: Some("2025-02-09T09:15:00Z".to_string()),
            sent_at: Some("2025-02-10T08:00:00Z".to_string()),
            acknowledged_at: Some("2025-02-10T09:45:00Z".to_string()),
        }),
        _ => None,
    }
}
