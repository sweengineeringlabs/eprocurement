//! Audit store

use components::prelude::*;
use super::types::{
    AuditEntry, AuditFilter, AuditStats, AuditEntityType, AuditActionType, FieldChange,
};

/// Audit state store
#[derive(Clone)]
pub struct AuditStore {
    pub entries: Signal<Vec<AuditEntry>>,
    pub selected: Signal<Option<AuditEntry>>,
    pub filter: Signal<AuditFilter>,
    pub stats: Signal<AuditStats>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
    pub page: Signal<usize>,
    pub page_size: Signal<usize>,
    pub total_count: Signal<usize>,
}

impl AuditStore {
    pub fn new() -> Self {
        Self {
            entries: signal(Vec::new()),
            selected: signal(None),
            filter: signal(AuditFilter::default()),
            stats: signal(AuditStats::default()),
            loading: signal(false),
            error: signal(None),
            page: signal(1),
            page_size: signal(50),
            total_count: signal(0),
        }
    }

    /// Get filtered audit entries based on current filter
    pub fn get_filtered_entries(&self) -> Vec<AuditEntry> {
        let entries = self.entries.get();
        let filter = self.filter.get();

        entries.into_iter().filter(|entry| {
            // Filter by entity type
            if let Some(ref entity_type) = filter.entity_type {
                if &entry.entity_type != entity_type {
                    return false;
                }
            }

            // Filter by action type
            if let Some(ref action_type) = filter.action_type {
                if &entry.action != action_type {
                    return false;
                }
            }

            // Filter by user ID
            if let Some(ref user_id) = filter.user_id {
                if &entry.user_id != user_id {
                    return false;
                }
            }

            // Filter by user search (name or email)
            if let Some(ref search) = filter.user_search {
                let search_lower = search.to_lowercase();
                if !entry.user_name.to_lowercase().contains(&search_lower)
                    && !entry.user_email.to_lowercase().contains(&search_lower)
                {
                    return false;
                }
            }

            // Filter by entity ID
            if let Some(ref entity_id) = filter.entity_id {
                if &entry.entity_id != entity_id {
                    return false;
                }
            }

            // Filter by date range (from)
            if let Some(ref date_from) = filter.date_from {
                if entry.timestamp < *date_from {
                    return false;
                }
            }

            // Filter by date range (to)
            if let Some(ref date_to) = filter.date_to {
                // Add a day to include the end date
                let end_date = format!("{}T23:59:59Z", date_to);
                if entry.timestamp > end_date {
                    return false;
                }
            }

            // Filter by IP address
            if let Some(ref ip) = filter.ip_address {
                if !entry.ip_address.contains(ip) {
                    return false;
                }
            }

            // General search filter
            if let Some(ref search) = filter.search {
                let search_lower = search.to_lowercase();
                let matches = entry.user_name.to_lowercase().contains(&search_lower)
                    || entry.user_email.to_lowercase().contains(&search_lower)
                    || entry.entity_id.to_lowercase().contains(&search_lower)
                    || entry.entity_name.as_ref().map(|n| n.to_lowercase().contains(&search_lower)).unwrap_or(false)
                    || entry.description.as_ref().map(|d| d.to_lowercase().contains(&search_lower)).unwrap_or(false)
                    || entry.ip_address.contains(&search_lower);
                if !matches {
                    return false;
                }
            }

            true
        }).collect()
    }

    /// Select an audit entry for detail view
    pub fn select_entry(&self, entry_id: &str) {
        let entries = self.entries.get();
        if let Some(entry) = entries.iter().find(|e| e.id == entry_id) {
            self.selected.set(Some(entry.clone()));
        }
    }

    /// Clear selected entry
    pub fn clear_selection(&self) {
        self.selected.set(None);
    }

    /// Set entity type filter
    pub fn set_filter_entity_type(&self, entity_type: Option<AuditEntityType>) {
        let mut filter = self.filter.get().clone();
        filter.entity_type = entity_type;
        self.filter.set(filter);
    }

    /// Set action type filter
    pub fn set_filter_action_type(&self, action_type: Option<AuditActionType>) {
        let mut filter = self.filter.get().clone();
        filter.action_type = action_type;
        self.filter.set(filter);
    }

    /// Set user search filter
    pub fn set_filter_user(&self, user_search: Option<String>) {
        let mut filter = self.filter.get().clone();
        filter.user_search = user_search;
        self.filter.set(filter);
    }

    /// Set date range filter
    pub fn set_filter_date_range(&self, date_from: Option<String>, date_to: Option<String>) {
        let mut filter = self.filter.get().clone();
        filter.date_from = date_from;
        filter.date_to = date_to;
        self.filter.set(filter);
    }

    /// Set search filter
    pub fn set_filter_search(&self, search: Option<String>) {
        let mut filter = self.filter.get().clone();
        filter.search = search;
        self.filter.set(filter);
    }

    /// Clear all filters
    pub fn clear_filters(&self) {
        self.filter.set(AuditFilter::default());
    }

    /// Calculate statistics from current entries
    pub fn calculate_stats(&self) {
        let entries = self.entries.get();
        let today = "2025-02-27"; // Would use actual current date in production

        let mut unique_users: Vec<String> = entries.iter().map(|e| e.user_id.clone()).collect();
        unique_users.sort();
        unique_users.dedup();

        let stats = AuditStats {
            total_entries: entries.len(),
            entries_today: entries.iter().filter(|e| e.timestamp.starts_with(today)).count(),
            unique_users: unique_users.len(),
            creates: entries.iter().filter(|e| e.action == AuditActionType::Create).count(),
            updates: entries.iter().filter(|e| e.action == AuditActionType::Update).count(),
            deletes: entries.iter().filter(|e| e.action == AuditActionType::Delete).count(),
            logins: entries.iter().filter(|e| e.action == AuditActionType::Login).count(),
        };

        self.stats.set(stats);
    }

    /// Set page for pagination
    pub fn set_page(&self, page: usize) {
        self.page.set(page);
    }

    /// Get paginated entries
    pub fn get_paginated_entries(&self) -> Vec<AuditEntry> {
        let filtered = self.get_filtered_entries();
        let page = self.page.get();
        let page_size = self.page_size.get();

        let start = (page - 1) * page_size;
        let end = std::cmp::min(start + page_size, filtered.len());

        if start >= filtered.len() {
            return Vec::new();
        }

        filtered[start..end].to_vec()
    }

    /// Get total pages
    pub fn total_pages(&self) -> usize {
        let filtered_count = self.get_filtered_entries().len();
        let page_size = self.page_size.get();
        (filtered_count + page_size - 1) / page_size
    }
}

/// Load mock audit data for demo
pub fn load_mock_data(store: &AuditStore) {
    let mock_entries = vec![
        AuditEntry {
            id: "aud_001".to_string(),
            timestamp: "2025-02-27T09:15:00Z".to_string(),
            user_id: "usr_001".to_string(),
            user_name: "Dr. Thandi Nkosi".to_string(),
            user_email: "thandi.nkosi@gov.za".to_string(),
            action: AuditActionType::Approve,
            entity_type: AuditEntityType::Tender,
            entity_id: "TND-2025-0089".to_string(),
            entity_name: Some("IT Infrastructure Upgrade".to_string()),
            changes: vec![
                FieldChange::new("status", Some("Pending Approval"), Some("Published")),
            ],
            ip_address: "196.25.100.45".to_string(),
            user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)".to_string()),
            session_id: Some("sess_abc123".to_string()),
            description: Some("Approved tender for publication".to_string()),
            metadata: None,
        },
        AuditEntry {
            id: "aud_002".to_string(),
            timestamp: "2025-02-27T08:45:00Z".to_string(),
            user_id: "usr_002".to_string(),
            user_name: "Mr. Johan van der Berg".to_string(),
            user_email: "johan.vdb@gov.za".to_string(),
            action: AuditActionType::Update,
            entity_type: AuditEntityType::Contract,
            entity_id: "CON-2025-0042".to_string(),
            entity_name: Some("Security Services Contract".to_string()),
            changes: vec![
                FieldChange::new("end_date", Some("2025-06-30"), Some("2025-12-31")),
                FieldChange::new("value", Some("15000000.00"), Some("18500000.00")),
            ],
            ip_address: "196.25.100.82".to_string(),
            user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)".to_string()),
            session_id: Some("sess_def456".to_string()),
            description: Some("Extended contract term and increased value".to_string()),
            metadata: None,
        },
        AuditEntry {
            id: "aud_003".to_string(),
            timestamp: "2025-02-27T08:30:00Z".to_string(),
            user_id: "usr_003".to_string(),
            user_name: "Ms. Priya Naidoo".to_string(),
            user_email: "priya.naidoo@gov.za".to_string(),
            action: AuditActionType::Create,
            entity_type: AuditEntityType::Requisition,
            entity_id: "REQ-2025-0156".to_string(),
            entity_name: Some("Office Equipment Procurement".to_string()),
            changes: vec![],
            ip_address: "196.25.100.93".to_string(),
            user_agent: Some("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)".to_string()),
            session_id: Some("sess_ghi789".to_string()),
            description: Some("Created new requisition for office equipment".to_string()),
            metadata: None,
        },
        AuditEntry {
            id: "aud_004".to_string(),
            timestamp: "2025-02-27T08:00:00Z".to_string(),
            user_id: "usr_001".to_string(),
            user_name: "Dr. Thandi Nkosi".to_string(),
            user_email: "thandi.nkosi@gov.za".to_string(),
            action: AuditActionType::Login,
            entity_type: AuditEntityType::User,
            entity_id: "usr_001".to_string(),
            entity_name: Some("Dr. Thandi Nkosi".to_string()),
            changes: vec![],
            ip_address: "196.25.100.45".to_string(),
            user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)".to_string()),
            session_id: Some("sess_abc123".to_string()),
            description: Some("User logged in successfully".to_string()),
            metadata: None,
        },
        AuditEntry {
            id: "aud_005".to_string(),
            timestamp: "2025-02-26T16:30:00Z".to_string(),
            user_id: "usr_004".to_string(),
            user_name: "Mr. Sipho Dlamini".to_string(),
            user_email: "sipho.dlamini@gov.za".to_string(),
            action: AuditActionType::Submit,
            entity_type: AuditEntityType::Evaluation,
            entity_id: "EVL-2025-0034".to_string(),
            entity_name: Some("Bid Evaluation - Fleet Management".to_string()),
            changes: vec![
                FieldChange::new("status", Some("In Progress"), Some("Submitted")),
            ],
            ip_address: "196.25.100.67".to_string(),
            user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)".to_string()),
            session_id: Some("sess_jkl012".to_string()),
            description: Some("Submitted evaluation scores for review".to_string()),
            metadata: None,
        },
        AuditEntry {
            id: "aud_006".to_string(),
            timestamp: "2025-02-26T15:45:00Z".to_string(),
            user_id: "usr_005".to_string(),
            user_name: "Adv. Sarah Mokoena".to_string(),
            user_email: "sarah.mokoena@gov.za".to_string(),
            action: AuditActionType::View,
            entity_type: AuditEntityType::Bid,
            entity_id: "BID-2025-0089-003".to_string(),
            entity_name: Some("Innovate IT Holdings Bid".to_string()),
            changes: vec![],
            ip_address: "196.25.100.55".to_string(),
            user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)".to_string()),
            session_id: Some("sess_mno345".to_string()),
            description: Some("Viewed bid details for legal review".to_string()),
            metadata: None,
        },
        AuditEntry {
            id: "aud_007".to_string(),
            timestamp: "2025-02-26T14:20:00Z".to_string(),
            user_id: "usr_002".to_string(),
            user_name: "Mr. Johan van der Berg".to_string(),
            user_email: "johan.vdb@gov.za".to_string(),
            action: AuditActionType::Export,
            entity_type: AuditEntityType::Contract,
            entity_id: "CON-2025-0038".to_string(),
            entity_name: Some("IT Support Services Contract".to_string()),
            changes: vec![],
            ip_address: "196.25.100.82".to_string(),
            user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)".to_string()),
            session_id: Some("sess_def456".to_string()),
            description: Some("Exported contract to PDF format".to_string()),
            metadata: Some("{\"format\": \"pdf\", \"size\": \"2.4MB\"}".to_string()),
        },
        AuditEntry {
            id: "aud_008".to_string(),
            timestamp: "2025-02-26T11:30:00Z".to_string(),
            user_id: "usr_006".to_string(),
            user_name: "System Administrator".to_string(),
            user_email: "admin@gov.za".to_string(),
            action: AuditActionType::Update,
            entity_type: AuditEntityType::Supplier,
            entity_id: "SUP-001".to_string(),
            entity_name: Some("TechSolutions SA (Pty) Ltd".to_string()),
            changes: vec![
                FieldChange::new("bbbee_level", Some("2"), Some("1")),
                FieldChange::new("bbbee_certificate_expiry", Some("2024-12-31"), Some("2025-12-31")),
            ],
            ip_address: "196.25.100.10".to_string(),
            user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)".to_string()),
            session_id: Some("sess_pqr678".to_string()),
            description: Some("Updated supplier B-BBEE certification".to_string()),
            metadata: None,
        },
        AuditEntry {
            id: "aud_009".to_string(),
            timestamp: "2025-02-26T10:15:00Z".to_string(),
            user_id: "usr_003".to_string(),
            user_name: "Ms. Priya Naidoo".to_string(),
            user_email: "priya.naidoo@gov.za".to_string(),
            action: AuditActionType::Cancel,
            entity_type: AuditEntityType::PurchaseOrder,
            entity_id: "PO-2025-0234".to_string(),
            entity_name: Some("Stationery Order Q1".to_string()),
            changes: vec![
                FieldChange::new("status", Some("Pending"), Some("Cancelled")),
            ],
            ip_address: "196.25.100.93".to_string(),
            user_agent: Some("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)".to_string()),
            session_id: Some("sess_stu901".to_string()),
            description: Some("Cancelled purchase order due to budget constraints".to_string()),
            metadata: None,
        },
        AuditEntry {
            id: "aud_010".to_string(),
            timestamp: "2025-02-26T09:00:00Z".to_string(),
            user_id: "usr_007".to_string(),
            user_name: "Mr. David Moyo".to_string(),
            user_email: "david.moyo@gov.za".to_string(),
            action: AuditActionType::Create,
            entity_type: AuditEntityType::GoodsReceipt,
            entity_id: "GR-2025-0089".to_string(),
            entity_name: Some("IT Equipment Delivery".to_string()),
            changes: vec![],
            ip_address: "196.25.100.112".to_string(),
            user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)".to_string()),
            session_id: Some("sess_vwx234".to_string()),
            description: Some("Recorded goods receipt for PO-2025-0198".to_string()),
            metadata: None,
        },
        AuditEntry {
            id: "aud_011".to_string(),
            timestamp: "2025-02-25T17:00:00Z".to_string(),
            user_id: "usr_001".to_string(),
            user_name: "Dr. Thandi Nkosi".to_string(),
            user_email: "thandi.nkosi@gov.za".to_string(),
            action: AuditActionType::Logout,
            entity_type: AuditEntityType::User,
            entity_id: "usr_001".to_string(),
            entity_name: Some("Dr. Thandi Nkosi".to_string()),
            changes: vec![],
            ip_address: "196.25.100.45".to_string(),
            user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)".to_string()),
            session_id: Some("sess_abc122".to_string()),
            description: Some("User logged out".to_string()),
            metadata: None,
        },
        AuditEntry {
            id: "aud_012".to_string(),
            timestamp: "2025-02-25T14:30:00Z".to_string(),
            user_id: "usr_004".to_string(),
            user_name: "Mr. Sipho Dlamini".to_string(),
            user_email: "sipho.dlamini@gov.za".to_string(),
            action: AuditActionType::Reject,
            entity_type: AuditEntityType::Requisition,
            entity_id: "REQ-2025-0148".to_string(),
            entity_name: Some("Marketing Materials".to_string()),
            changes: vec![
                FieldChange::new("status", Some("Pending Approval"), Some("Rejected")),
            ],
            ip_address: "196.25.100.67".to_string(),
            user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)".to_string()),
            session_id: Some("sess_jkl011".to_string()),
            description: Some("Rejected requisition - insufficient budget allocation".to_string()),
            metadata: None,
        },
        AuditEntry {
            id: "aud_013".to_string(),
            timestamp: "2025-02-25T11:45:00Z".to_string(),
            user_id: "usr_002".to_string(),
            user_name: "Mr. Johan van der Berg".to_string(),
            user_email: "johan.vdb@gov.za".to_string(),
            action: AuditActionType::Update,
            entity_type: AuditEntityType::Tender,
            entity_id: "TND-2025-0076".to_string(),
            entity_name: Some("Security Services Tender".to_string()),
            changes: vec![
                FieldChange::new("closing_date", Some("2025-02-10"), Some("2025-02-15")),
                FieldChange::new("clarification_deadline", Some("2025-02-05"), Some("2025-02-10")),
            ],
            ip_address: "196.25.100.82".to_string(),
            user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)".to_string()),
            session_id: Some("sess_def455".to_string()),
            description: Some("Extended tender closing date".to_string()),
            metadata: None,
        },
        AuditEntry {
            id: "aud_014".to_string(),
            timestamp: "2025-02-25T09:20:00Z".to_string(),
            user_id: "usr_008".to_string(),
            user_name: "Ms. Nomvula Khumalo".to_string(),
            user_email: "nomvula.khumalo@gov.za".to_string(),
            action: AuditActionType::Import,
            entity_type: AuditEntityType::Supplier,
            entity_id: "batch_import_001".to_string(),
            entity_name: Some("Supplier Bulk Import".to_string()),
            changes: vec![],
            ip_address: "196.25.100.78".to_string(),
            user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)".to_string()),
            session_id: Some("sess_yza567".to_string()),
            description: Some("Imported 45 new suppliers from CSV".to_string()),
            metadata: Some("{\"records_imported\": 45, \"records_failed\": 2}".to_string()),
        },
        AuditEntry {
            id: "aud_015".to_string(),
            timestamp: "2025-02-24T16:00:00Z".to_string(),
            user_id: "usr_006".to_string(),
            user_name: "System Administrator".to_string(),
            user_email: "admin@gov.za".to_string(),
            action: AuditActionType::Delete,
            entity_type: AuditEntityType::User,
            entity_id: "usr_temp_001".to_string(),
            entity_name: Some("Temporary User Account".to_string()),
            changes: vec![],
            ip_address: "196.25.100.10".to_string(),
            user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)".to_string()),
            session_id: Some("sess_pqr677".to_string()),
            description: Some("Deleted expired temporary user account".to_string()),
            metadata: None,
        },
    ];

    store.entries.set(mock_entries);
    store.calculate_stats();
    store.total_count.set(store.entries.get().len());
}
