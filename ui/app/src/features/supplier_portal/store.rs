//! Supplier Portal store

use components::prelude::*;
use super::types::{
    TenderOpportunity, BidSubmission, ContractAward, PortalDocument,
    PortalKpi, PortalNotification, OpportunityStatus, BidSubmissionStatus,
    ContractAwardStatus, OpportunityFilter, BidSubmissionFilter, ContractAwardFilter,
    PortalDocumentType,
};

/// Supplier Portal state store
#[derive(Clone)]
pub struct SupplierPortalStore {
    // KPIs
    pub kpis: Signal<PortalKpi>,

    // Opportunities
    pub opportunities: Signal<Vec<TenderOpportunity>>,
    pub selected_opportunity: Signal<Option<TenderOpportunity>>,
    pub opportunity_filter: Signal<OpportunityFilter>,

    // Bid Submissions
    pub submissions: Signal<Vec<BidSubmission>>,
    pub selected_submission: Signal<Option<BidSubmission>>,
    pub submission_filter: Signal<BidSubmissionFilter>,

    // Contract Awards
    pub awards: Signal<Vec<ContractAward>>,
    pub selected_award: Signal<Option<ContractAward>>,
    pub award_filter: Signal<ContractAwardFilter>,

    // Documents
    pub documents: Signal<Vec<PortalDocument>>,
    pub pending_uploads: Signal<Vec<PortalDocument>>,

    // Notifications
    pub notifications: Signal<Vec<PortalNotification>>,
    pub unread_count: Signal<u32>,

    // UI State
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
    pub saving: Signal<bool>,
    pub active_tab: Signal<String>,
}

impl SupplierPortalStore {
    pub fn new() -> Self {
        Self {
            kpis: signal(PortalKpi::default()),
            opportunities: signal(Vec::new()),
            selected_opportunity: signal(None),
            opportunity_filter: signal(OpportunityFilter::default()),
            submissions: signal(Vec::new()),
            selected_submission: signal(None),
            submission_filter: signal(BidSubmissionFilter::default()),
            awards: signal(Vec::new()),
            selected_award: signal(None),
            award_filter: signal(ContractAwardFilter::default()),
            documents: signal(Vec::new()),
            pending_uploads: signal(Vec::new()),
            notifications: signal(Vec::new()),
            unread_count: signal(0),
            loading: signal(false),
            error: signal(None),
            saving: signal(false),
            active_tab: signal("opportunities".to_string()),
        }
    }

    // Opportunity filter methods
    pub fn set_opportunity_category(&self, category: Option<String>) {
        let mut filter = self.opportunity_filter.get();
        filter.category = category;
        self.opportunity_filter.set(filter);
    }

    pub fn set_opportunity_status(&self, status: Option<OpportunityStatus>) {
        let mut filter = self.opportunity_filter.get();
        filter.status = status;
        self.opportunity_filter.set(filter);
    }

    pub fn set_opportunity_search(&self, search: Option<String>) {
        let mut filter = self.opportunity_filter.get();
        filter.search_query = search;
        self.opportunity_filter.set(filter);
    }

    pub fn clear_opportunity_filters(&self) {
        self.opportunity_filter.set(OpportunityFilter::default());
    }

    /// Get filtered opportunities
    pub fn get_filtered_opportunities(&self) -> Vec<TenderOpportunity> {
        let opportunities = self.opportunities.get();
        let filter = self.opportunity_filter.get();

        opportunities
            .iter()
            .filter(|o| {
                // Filter by category
                if let Some(ref category) = filter.category {
                    if !category.is_empty() && o.category != *category {
                        return false;
                    }
                }

                // Filter by status
                if let Some(status) = filter.status {
                    if o.status != status {
                        return false;
                    }
                }

                // Filter by tender type
                if let Some(ref tender_type) = filter.tender_type {
                    if !tender_type.is_empty() && o.tender_type != *tender_type {
                        return false;
                    }
                }

                // Filter by value range
                if let Some(min_value) = filter.min_value {
                    if o.estimated_value < min_value {
                        return false;
                    }
                }

                if let Some(max_value) = filter.max_value {
                    if o.estimated_value > max_value {
                        return false;
                    }
                }

                // Filter by closing within days
                if let Some(days) = filter.closing_within_days {
                    if o.days_remaining > days as i32 || o.days_remaining < 0 {
                        return false;
                    }
                }

                // Filter by search query
                if let Some(ref search) = filter.search_query {
                    if !search.is_empty() {
                        let search_lower = search.to_lowercase();
                        if !o.title.to_lowercase().contains(&search_lower)
                            && !o.reference_number.to_lowercase().contains(&search_lower)
                            && !o.description.to_lowercase().contains(&search_lower)
                        {
                            return false;
                        }
                    }
                }

                true
            })
            .cloned()
            .collect()
    }

    // Submission filter methods
    pub fn set_submission_status(&self, status: Option<BidSubmissionStatus>) {
        let mut filter = self.submission_filter.get();
        filter.status = status;
        self.submission_filter.set(filter);
    }

    pub fn set_submission_search(&self, search: Option<String>) {
        let mut filter = self.submission_filter.get();
        filter.search_query = search;
        self.submission_filter.set(filter);
    }

    pub fn clear_submission_filters(&self) {
        self.submission_filter.set(BidSubmissionFilter::default());
    }

    /// Get filtered submissions
    pub fn get_filtered_submissions(&self) -> Vec<BidSubmission> {
        let submissions = self.submissions.get();
        let filter = self.submission_filter.get();

        submissions
            .iter()
            .filter(|s| {
                // Filter by status
                if let Some(status) = filter.status {
                    if s.status != status {
                        return false;
                    }
                }

                // Filter by search query
                if let Some(ref search) = filter.search_query {
                    if !search.is_empty() {
                        let search_lower = search.to_lowercase();
                        if !s.tender_title.to_lowercase().contains(&search_lower)
                            && !s.tender_reference.to_lowercase().contains(&search_lower)
                        {
                            return false;
                        }
                    }
                }

                true
            })
            .cloned()
            .collect()
    }

    // Award filter methods
    pub fn set_award_status(&self, status: Option<ContractAwardStatus>) {
        let mut filter = self.award_filter.get();
        filter.status = status;
        self.award_filter.set(filter);
    }

    pub fn set_award_search(&self, search: Option<String>) {
        let mut filter = self.award_filter.get();
        filter.search_query = search;
        self.award_filter.set(filter);
    }

    pub fn clear_award_filters(&self) {
        self.award_filter.set(ContractAwardFilter::default());
    }

    /// Get filtered awards
    pub fn get_filtered_awards(&self) -> Vec<ContractAward> {
        let awards = self.awards.get();
        let filter = self.award_filter.get();

        awards
            .iter()
            .filter(|a| {
                // Filter by status
                if let Some(status) = filter.status {
                    if a.status != status {
                        return false;
                    }
                }

                // Filter by search query
                if let Some(ref search) = filter.search_query {
                    if !search.is_empty() {
                        let search_lower = search.to_lowercase();
                        if !a.title.to_lowercase().contains(&search_lower)
                            && !a.contract_number.to_lowercase().contains(&search_lower)
                            && !a.tender_reference.to_lowercase().contains(&search_lower)
                        {
                            return false;
                        }
                    }
                }

                true
            })
            .cloned()
            .collect()
    }

    /// Set active tab
    pub fn set_active_tab(&self, tab: &str) {
        self.active_tab.set(tab.to_string());
    }

    /// Mark notification as read
    pub fn mark_notification_read(&self, notification_id: &str) {
        let mut notifications = self.notifications.get();
        if let Some(notification) = notifications.iter_mut().find(|n| n.id == notification_id) {
            if !notification.read {
                notification.read = true;
                let count = self.unread_count.get();
                if count > 0 {
                    self.unread_count.set(count - 1);
                }
            }
        }
        self.notifications.set(notifications);
    }

    /// Mark all notifications as read
    pub fn mark_all_notifications_read(&self) {
        let mut notifications = self.notifications.get();
        for notification in notifications.iter_mut() {
            notification.read = true;
        }
        self.notifications.set(notifications);
        self.unread_count.set(0);
    }
}

/// Load mock data for supplier portal
pub fn load_mock_portal_data(store: &SupplierPortalStore) {
    // Load opportunities
    store.opportunities.set(vec![
        TenderOpportunity {
            id: "TND-2025-0089".to_string(),
            reference_number: "RFP/2025/IT/089".to_string(),
            title: "IT Infrastructure Upgrade - Data Center Modernization".to_string(),
            description: "Supply and installation of server infrastructure, storage solutions, and network equipment for data center upgrade project.".to_string(),
            category: "IT".to_string(),
            estimated_value: 15_000_000.0,
            currency: "ZAR".to_string(),
            publish_date: "2025-02-15".to_string(),
            closing_date: "2025-03-15".to_string(),
            briefing_date: Some("2025-02-25".to_string()),
            department: "Information Technology".to_string(),
            delivery_location: "Johannesburg Head Office".to_string(),
            contract_duration: "24 months".to_string(),
            status: OpportunityStatus::Open,
            days_remaining: 16,
            document_count: 5,
            mandatory_briefing: true,
            tender_type: "RFP".to_string(),
        },
        TenderOpportunity {
            id: "TND-2025-0092".to_string(),
            reference_number: "RFQ/2025/FAC/092".to_string(),
            title: "Office Furniture Supply - Regional Offices".to_string(),
            description: "Supply and delivery of office furniture including desks, chairs, and storage units for 5 regional offices.".to_string(),
            category: "Facilities".to_string(),
            estimated_value: 2_500_000.0,
            currency: "ZAR".to_string(),
            publish_date: "2025-02-20".to_string(),
            closing_date: "2025-03-05".to_string(),
            briefing_date: None,
            department: "Facilities Management".to_string(),
            delivery_location: "Multiple Locations".to_string(),
            contract_duration: "3 months".to_string(),
            status: OpportunityStatus::ClosingSoon,
            days_remaining: 6,
            document_count: 3,
            mandatory_briefing: false,
            tender_type: "RFQ".to_string(),
        },
        TenderOpportunity {
            id: "TND-2025-0095".to_string(),
            reference_number: "RFT/2025/SEC/095".to_string(),
            title: "Security Services - National Coverage".to_string(),
            description: "Provision of physical security services including access control, CCTV monitoring, and armed response for all premises nationally.".to_string(),
            category: "Security".to_string(),
            estimated_value: 25_000_000.0,
            currency: "ZAR".to_string(),
            publish_date: "2025-02-18".to_string(),
            closing_date: "2025-03-25".to_string(),
            briefing_date: Some("2025-03-01".to_string()),
            department: "Corporate Services".to_string(),
            delivery_location: "National".to_string(),
            contract_duration: "36 months".to_string(),
            status: OpportunityStatus::Open,
            days_remaining: 26,
            document_count: 8,
            mandatory_briefing: true,
            tender_type: "RFT".to_string(),
        },
        TenderOpportunity {
            id: "TND-2025-0087".to_string(),
            reference_number: "RFP/2025/TRAIN/087".to_string(),
            title: "Staff Training Program - Leadership Development".to_string(),
            description: "Design and delivery of leadership development training program for senior management.".to_string(),
            category: "Training".to_string(),
            estimated_value: 1_800_000.0,
            currency: "ZAR".to_string(),
            publish_date: "2025-02-10".to_string(),
            closing_date: "2025-02-28".to_string(),
            briefing_date: None,
            department: "Human Resources".to_string(),
            delivery_location: "Head Office".to_string(),
            contract_duration: "12 months".to_string(),
            status: OpportunityStatus::BidSubmitted,
            days_remaining: 1,
            document_count: 4,
            mandatory_briefing: false,
            tender_type: "RFP".to_string(),
        },
    ]);

    // Load bid submissions
    store.submissions.set(vec![
        BidSubmission {
            id: "BID-2025-0234".to_string(),
            tender_id: "TND-2025-0087".to_string(),
            tender_reference: "RFP/2025/TRAIN/087".to_string(),
            tender_title: "Staff Training Program - Leadership Development".to_string(),
            submitted_at: Some("2025-02-25T14:30:00Z".to_string()),
            total_price: 1_650_000.0,
            currency: "ZAR".to_string(),
            status: BidSubmissionStatus::UnderEvaluation,
            technical_compliance: Some(85.0),
            price_score: None,
            total_score: None,
            rank: None,
            documents_uploaded: 6,
            documents_required: 6,
            notes: Some("Technical evaluation in progress".to_string()),
            created_at: "2025-02-20T09:00:00Z".to_string(),
            updated_at: "2025-02-25T14:30:00Z".to_string(),
        },
        BidSubmission {
            id: "BID-2025-0198".to_string(),
            tender_id: "TND-2025-0078".to_string(),
            tender_reference: "RFQ/2025/IT/078".to_string(),
            tender_title: "Network Equipment Supply".to_string(),
            submitted_at: Some("2025-02-15T11:45:00Z".to_string()),
            total_price: 3_200_000.0,
            currency: "ZAR".to_string(),
            status: BidSubmissionStatus::Shortlisted,
            technical_compliance: Some(92.0),
            price_score: Some(88.5),
            total_score: Some(89.8),
            rank: Some(2),
            documents_uploaded: 5,
            documents_required: 5,
            notes: Some("Shortlisted - awaiting final evaluation".to_string()),
            created_at: "2025-02-10T08:00:00Z".to_string(),
            updated_at: "2025-02-20T16:00:00Z".to_string(),
        },
        BidSubmission {
            id: "BID-2025-0156".to_string(),
            tender_id: "TND-2025-0065".to_string(),
            tender_reference: "RFT/2025/FAC/065".to_string(),
            tender_title: "Cleaning Services - Head Office".to_string(),
            submitted_at: Some("2025-02-01T09:00:00Z".to_string()),
            total_price: 1_200_000.0,
            currency: "ZAR".to_string(),
            status: BidSubmissionStatus::Awarded,
            technical_compliance: Some(95.0),
            price_score: Some(92.0),
            total_score: Some(93.2),
            rank: Some(1),
            documents_uploaded: 7,
            documents_required: 7,
            notes: Some("Contract awarded - awaiting signature".to_string()),
            created_at: "2025-01-25T10:00:00Z".to_string(),
            updated_at: "2025-02-20T14:00:00Z".to_string(),
        },
        BidSubmission {
            id: "BID-2025-0089".to_string(),
            tender_id: "TND-2025-0045".to_string(),
            tender_reference: "RFP/2025/IT/045".to_string(),
            tender_title: "Software Development Services".to_string(),
            submitted_at: Some("2025-01-20T15:00:00Z".to_string()),
            total_price: 8_500_000.0,
            currency: "ZAR".to_string(),
            status: BidSubmissionStatus::Unsuccessful,
            technical_compliance: Some(78.0),
            price_score: Some(75.0),
            total_score: Some(76.8),
            rank: Some(4),
            documents_uploaded: 8,
            documents_required: 8,
            notes: Some("Did not meet minimum technical threshold".to_string()),
            created_at: "2025-01-10T09:00:00Z".to_string(),
            updated_at: "2025-02-10T11:00:00Z".to_string(),
        },
        BidSubmission {
            id: "BID-2025-0301".to_string(),
            tender_id: "TND-2025-0089".to_string(),
            tender_reference: "RFP/2025/IT/089".to_string(),
            tender_title: "IT Infrastructure Upgrade - Data Center Modernization".to_string(),
            submitted_at: None,
            total_price: 0.0,
            currency: "ZAR".to_string(),
            status: BidSubmissionStatus::Draft,
            technical_compliance: None,
            price_score: None,
            total_score: None,
            rank: None,
            documents_uploaded: 2,
            documents_required: 8,
            notes: None,
            created_at: "2025-02-26T10:00:00Z".to_string(),
            updated_at: "2025-02-26T10:00:00Z".to_string(),
        },
    ]);

    // Load contract awards
    store.awards.set(vec![
        ContractAward {
            id: "CTR-2025-0156".to_string(),
            contract_number: "PFMA/2025/FAC/0156".to_string(),
            tender_reference: "RFT/2025/FAC/065".to_string(),
            title: "Cleaning Services - Head Office".to_string(),
            description: "Provision of daily cleaning services for head office premises including consumables supply.".to_string(),
            value: 1_200_000.0,
            currency: "ZAR".to_string(),
            start_date: "2025-03-01".to_string(),
            end_date: "2026-02-28".to_string(),
            status: ContractAwardStatus::AwaitingSignature,
            award_date: "2025-02-20".to_string(),
            buyer_name: "Government Department X".to_string(),
            buyer_department: "Facilities Management".to_string(),
            payment_terms: "Monthly in arrears, 30 days".to_string(),
            next_milestone: Some("Contract Signing".to_string()),
            next_milestone_date: Some("2025-02-28".to_string()),
            total_invoiced: 0.0,
            total_paid: 0.0,
            documents: Vec::new(),
        },
        ContractAward {
            id: "CTR-2024-0892".to_string(),
            contract_number: "PFMA/2024/IT/0892".to_string(),
            tender_reference: "RFP/2024/IT/156".to_string(),
            title: "IT Support Services".to_string(),
            description: "Provision of IT support services including help desk, network support, and hardware maintenance.".to_string(),
            value: 8_500_000.0,
            currency: "ZAR".to_string(),
            start_date: "2024-07-01".to_string(),
            end_date: "2025-06-30".to_string(),
            status: ContractAwardStatus::Active,
            award_date: "2024-06-15".to_string(),
            buyer_name: "Government Department X".to_string(),
            buyer_department: "Information Technology".to_string(),
            payment_terms: "Monthly in arrears, 30 days".to_string(),
            next_milestone: Some("Q3 Review".to_string()),
            next_milestone_date: Some("2025-03-31".to_string()),
            total_invoiced: 5_666_666.0,
            total_paid: 4_958_333.0,
            documents: Vec::new(),
        },
        ContractAward {
            id: "CTR-2024-0567".to_string(),
            contract_number: "PFMA/2024/SEC/0567".to_string(),
            tender_reference: "RFT/2024/SEC/089".to_string(),
            title: "Security Services - Regional Offices".to_string(),
            description: "Provision of physical security services for 3 regional offices.".to_string(),
            value: 4_200_000.0,
            currency: "ZAR".to_string(),
            start_date: "2024-04-01".to_string(),
            end_date: "2025-03-31".to_string(),
            status: ContractAwardStatus::Active,
            award_date: "2024-03-15".to_string(),
            buyer_name: "Government Department X".to_string(),
            buyer_department: "Corporate Services".to_string(),
            payment_terms: "Monthly in arrears, 30 days".to_string(),
            next_milestone: Some("Contract Renewal Decision".to_string()),
            next_milestone_date: Some("2025-02-28".to_string()),
            total_invoiced: 3_850_000.0,
            total_paid: 3_850_000.0,
            documents: Vec::new(),
        },
        ContractAward {
            id: "CTR-2023-0234".to_string(),
            contract_number: "PFMA/2023/FAC/0234".to_string(),
            tender_reference: "RFQ/2023/FAC/034".to_string(),
            title: "Office Supplies Framework Agreement".to_string(),
            description: "Supply of general office supplies and stationery on an as-needed basis.".to_string(),
            value: 1_500_000.0,
            currency: "ZAR".to_string(),
            start_date: "2023-07-01".to_string(),
            end_date: "2024-06-30".to_string(),
            status: ContractAwardStatus::Completed,
            award_date: "2023-06-15".to_string(),
            buyer_name: "Government Department X".to_string(),
            buyer_department: "Administration".to_string(),
            payment_terms: "30 days from invoice".to_string(),
            next_milestone: None,
            next_milestone_date: None,
            total_invoiced: 1_450_000.0,
            total_paid: 1_450_000.0,
            documents: Vec::new(),
        },
    ]);

    // Load notifications
    store.notifications.set(vec![
        PortalNotification {
            id: "NOT-001".to_string(),
            title: "New Tender Opportunity".to_string(),
            message: "RFP/2025/IT/089 - IT Infrastructure Upgrade is now open for bidding.".to_string(),
            notification_type: "opportunity".to_string(),
            reference_id: Some("TND-2025-0089".to_string()),
            created_at: "2025-02-26T09:00:00Z".to_string(),
            read: false,
            priority: "high".to_string(),
        },
        PortalNotification {
            id: "NOT-002".to_string(),
            title: "Bid Shortlisted".to_string(),
            message: "Your bid for Network Equipment Supply has been shortlisted for final evaluation.".to_string(),
            notification_type: "bid".to_string(),
            reference_id: Some("BID-2025-0198".to_string()),
            created_at: "2025-02-25T14:00:00Z".to_string(),
            read: false,
            priority: "high".to_string(),
        },
        PortalNotification {
            id: "NOT-003".to_string(),
            title: "Contract Awarded".to_string(),
            message: "Congratulations! You have been awarded the Cleaning Services contract.".to_string(),
            notification_type: "contract".to_string(),
            reference_id: Some("CTR-2025-0156".to_string()),
            created_at: "2025-02-20T16:00:00Z".to_string(),
            read: true,
            priority: "high".to_string(),
        },
        PortalNotification {
            id: "NOT-004".to_string(),
            title: "Document Required".to_string(),
            message: "Please upload updated B-BBEE certificate for your profile.".to_string(),
            notification_type: "document".to_string(),
            reference_id: None,
            created_at: "2025-02-18T10:00:00Z".to_string(),
            read: true,
            priority: "medium".to_string(),
        },
        PortalNotification {
            id: "NOT-005".to_string(),
            title: "Tender Closing Soon".to_string(),
            message: "RFQ/2025/FAC/092 - Office Furniture Supply closes in 6 days.".to_string(),
            notification_type: "opportunity".to_string(),
            reference_id: Some("TND-2025-0092".to_string()),
            created_at: "2025-02-26T08:00:00Z".to_string(),
            read: false,
            priority: "medium".to_string(),
        },
    ]);

    // Set unread count
    let unread = store.notifications.get().iter().filter(|n| !n.read).count() as u32;
    store.unread_count.set(unread);
}

/// Get mock opportunity details
pub fn get_mock_opportunity(id: &str) -> Option<TenderOpportunity> {
    let opportunities = vec![
        TenderOpportunity {
            id: "TND-2025-0089".to_string(),
            reference_number: "RFP/2025/IT/089".to_string(),
            title: "IT Infrastructure Upgrade - Data Center Modernization".to_string(),
            description: "Supply and installation of server infrastructure, storage solutions, and network equipment for data center upgrade project.".to_string(),
            category: "IT".to_string(),
            estimated_value: 15_000_000.0,
            currency: "ZAR".to_string(),
            publish_date: "2025-02-15".to_string(),
            closing_date: "2025-03-15".to_string(),
            briefing_date: Some("2025-02-25".to_string()),
            department: "Information Technology".to_string(),
            delivery_location: "Johannesburg Head Office".to_string(),
            contract_duration: "24 months".to_string(),
            status: OpportunityStatus::Open,
            days_remaining: 16,
            document_count: 5,
            mandatory_briefing: true,
            tender_type: "RFP".to_string(),
        },
    ];

    opportunities.into_iter().find(|o| o.id == id)
}

/// Get mock submission details
pub fn get_mock_submission(id: &str) -> Option<BidSubmission> {
    let submissions = vec![
        BidSubmission {
            id: "BID-2025-0234".to_string(),
            tender_id: "TND-2025-0087".to_string(),
            tender_reference: "RFP/2025/TRAIN/087".to_string(),
            tender_title: "Staff Training Program - Leadership Development".to_string(),
            submitted_at: Some("2025-02-25T14:30:00Z".to_string()),
            total_price: 1_650_000.0,
            currency: "ZAR".to_string(),
            status: BidSubmissionStatus::UnderEvaluation,
            technical_compliance: Some(85.0),
            price_score: None,
            total_score: None,
            rank: None,
            documents_uploaded: 6,
            documents_required: 6,
            notes: Some("Technical evaluation in progress".to_string()),
            created_at: "2025-02-20T09:00:00Z".to_string(),
            updated_at: "2025-02-25T14:30:00Z".to_string(),
        },
    ];

    submissions.into_iter().find(|s| s.id == id)
}

/// Get mock award details
pub fn get_mock_award(id: &str) -> Option<ContractAward> {
    let awards = vec![
        ContractAward {
            id: "CTR-2025-0156".to_string(),
            contract_number: "PFMA/2025/FAC/0156".to_string(),
            tender_reference: "RFT/2025/FAC/065".to_string(),
            title: "Cleaning Services - Head Office".to_string(),
            description: "Provision of daily cleaning services for head office premises including consumables supply.".to_string(),
            value: 1_200_000.0,
            currency: "ZAR".to_string(),
            start_date: "2025-03-01".to_string(),
            end_date: "2026-02-28".to_string(),
            status: ContractAwardStatus::AwaitingSignature,
            award_date: "2025-02-20".to_string(),
            buyer_name: "Government Department X".to_string(),
            buyer_department: "Facilities Management".to_string(),
            payment_terms: "Monthly in arrears, 30 days".to_string(),
            next_milestone: Some("Contract Signing".to_string()),
            next_milestone_date: Some("2025-02-28".to_string()),
            total_invoiced: 0.0,
            total_paid: 0.0,
            documents: Vec::new(),
        },
    ];

    awards.into_iter().find(|a| a.id == id)
}
