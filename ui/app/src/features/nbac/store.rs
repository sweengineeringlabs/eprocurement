//! NBAC store

use components::prelude::*;
use super::types::{
    ReviewItem, CommitteeMember, Decision, Meeting, Agenda, AgendaItem,
    ReviewStatus, ReviewCategory, Priority, RiskRating, MeetingStatus,
    MeetingType, MemberRole, DecisionType, VoteType, Vote, MeetingAttendee,
    ComplianceCheckResult, ReviewDocument, DocumentType, AgendaItemStatus,
    NbacKpis, ReviewFilter,
};

/// NBAC state store
#[derive(Clone)]
pub struct NbacStore {
    pub reviews: Signal<Vec<ReviewItem>>,
    pub selected_review: Signal<Option<ReviewItem>>,
    pub committee_members: Signal<Vec<CommitteeMember>>,
    pub meetings: Signal<Vec<Meeting>>,
    pub selected_meeting: Signal<Option<Meeting>>,
    pub decisions: Signal<Vec<Decision>>,
    pub kpis: Signal<NbacKpis>,
    pub filter: Signal<ReviewFilter>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
}

impl NbacStore {
    pub fn new() -> Self {
        Self {
            reviews: signal(Vec::new()),
            selected_review: signal(None),
            committee_members: signal(Vec::new()),
            meetings: signal(Vec::new()),
            selected_meeting: signal(None),
            decisions: signal(Vec::new()),
            kpis: signal(NbacKpis::default()),
            filter: signal(ReviewFilter::default()),
            loading: signal(false),
            error: signal(None),
        }
    }

    /// Select a review for detailed view
    pub fn select_review(&self, review_id: &str) {
        let reviews = self.reviews.get();
        if let Some(review) = reviews.iter().find(|r| r.id == review_id) {
            self.selected_review.set(Some(review.clone()));
        }
    }

    /// Clear review selection
    pub fn clear_review_selection(&self) {
        self.selected_review.set(None);
    }

    /// Select a meeting for detailed view
    pub fn select_meeting(&self, meeting_id: &str) {
        let meetings = self.meetings.get();
        if let Some(meeting) = meetings.iter().find(|m| m.id == meeting_id) {
            self.selected_meeting.set(Some(meeting.clone()));
        }
    }

    /// Clear meeting selection
    pub fn clear_meeting_selection(&self) {
        self.selected_meeting.set(None);
    }

    /// Update filter
    pub fn set_status_filter(&self, status: Option<ReviewStatus>) {
        let mut filter = self.filter.get().clone();
        filter.status = status;
        self.filter.set(filter);
    }

    /// Get filtered reviews
    pub fn filtered_reviews(&self) -> Vec<ReviewItem> {
        let reviews = self.reviews.get();
        let filter = self.filter.get();

        reviews.into_iter().filter(|review| {
            // Status filter
            if let Some(ref status) = filter.status {
                if &review.status != status {
                    return false;
                }
            }

            // Category filter
            if let Some(ref category) = filter.category {
                if &review.category != category {
                    return false;
                }
            }

            // Priority filter
            if let Some(ref priority) = filter.priority {
                if &review.priority != priority {
                    return false;
                }
            }

            // Search query
            if let Some(ref query) = filter.search_query {
                let query_lower = query.to_lowercase();
                if !review.tender_title.to_lowercase().contains(&query_lower)
                    && !review.tender_reference.to_lowercase().contains(&query_lower)
                    && !review.reference_number.to_lowercase().contains(&query_lower)
                {
                    return false;
                }
            }

            true
        }).collect()
    }

    /// Get upcoming meetings
    pub fn upcoming_meetings(&self) -> Vec<Meeting> {
        self.meetings.get().into_iter()
            .filter(|m| matches!(m.status, MeetingStatus::Scheduled))
            .collect()
    }

    /// Get recent decisions
    pub fn recent_decisions(&self) -> Vec<Decision> {
        let mut decisions = self.decisions.get();
        decisions.sort_by(|a, b| b.decision_date.cmp(&a.decision_date));
        decisions.into_iter().take(10).collect()
    }
}

/// Load mock data for demo
pub fn load_mock_data(store: &NbacStore) {
    // Committee members
    let mock_members = vec![
        CommitteeMember {
            id: "nbac_001".to_string(),
            name: "Dr. Nomvula Mokonyane".to_string(),
            title: "Chief Procurement Officer".to_string(),
            role: MemberRole::Chairperson,
            department: "Supply Chain Management".to_string(),
            email: "n.mokonyane@gov.za".to_string(),
            phone: "+27 12 345 6789".to_string(),
            is_active: true,
            appointment_date: "2023-04-01".to_string(),
            term_end_date: Some("2026-03-31".to_string()),
            conflict_of_interest: Vec::new(),
            attendance_rate: 95.0,
        },
        CommitteeMember {
            id: "nbac_002".to_string(),
            name: "Mr. Thabo Molefe".to_string(),
            title: "Chief Financial Officer".to_string(),
            role: MemberRole::DeputyChairperson,
            department: "Finance".to_string(),
            email: "t.molefe@gov.za".to_string(),
            phone: "+27 12 345 6790".to_string(),
            is_active: true,
            appointment_date: "2023-04-01".to_string(),
            term_end_date: Some("2026-03-31".to_string()),
            conflict_of_interest: Vec::new(),
            attendance_rate: 92.0,
        },
        CommitteeMember {
            id: "nbac_003".to_string(),
            name: "Adv. Lindiwe Sisulu".to_string(),
            title: "General Counsel".to_string(),
            role: MemberRole::LegalAdvisor,
            department: "Legal Services".to_string(),
            email: "l.sisulu@gov.za".to_string(),
            phone: "+27 12 345 6791".to_string(),
            is_active: true,
            appointment_date: "2023-04-01".to_string(),
            term_end_date: Some("2026-03-31".to_string()),
            conflict_of_interest: Vec::new(),
            attendance_rate: 88.0,
        },
        CommitteeMember {
            id: "nbac_004".to_string(),
            name: "Mr. Johan Pretorius".to_string(),
            title: "Director: IT".to_string(),
            role: MemberRole::TechnicalAdvisor,
            department: "Information Technology".to_string(),
            email: "j.pretorius@gov.za".to_string(),
            phone: "+27 12 345 6792".to_string(),
            is_active: true,
            appointment_date: "2023-04-01".to_string(),
            term_end_date: Some("2026-03-31".to_string()),
            conflict_of_interest: Vec::new(),
            attendance_rate: 90.0,
        },
        CommitteeMember {
            id: "nbac_005".to_string(),
            name: "Ms. Priya Naidoo".to_string(),
            title: "Director: SCM".to_string(),
            role: MemberRole::VotingMember,
            department: "Supply Chain Management".to_string(),
            email: "p.naidoo@gov.za".to_string(),
            phone: "+27 12 345 6793".to_string(),
            is_active: true,
            appointment_date: "2023-04-01".to_string(),
            term_end_date: Some("2026-03-31".to_string()),
            conflict_of_interest: Vec::new(),
            attendance_rate: 94.0,
        },
        CommitteeMember {
            id: "nbac_006".to_string(),
            name: "Mr. Sipho Dlamini".to_string(),
            title: "Deputy Director: Finance".to_string(),
            role: MemberRole::FinanceRepresentative,
            department: "Finance".to_string(),
            email: "s.dlamini@gov.za".to_string(),
            phone: "+27 12 345 6794".to_string(),
            is_active: true,
            appointment_date: "2023-04-01".to_string(),
            term_end_date: Some("2026-03-31".to_string()),
            conflict_of_interest: Vec::new(),
            attendance_rate: 96.0,
        },
        CommitteeMember {
            id: "nbac_007".to_string(),
            name: "Ms. Sarah van Wyk".to_string(),
            title: "Committee Secretary".to_string(),
            role: MemberRole::Secretary,
            department: "Supply Chain Management".to_string(),
            email: "s.vanwyk@gov.za".to_string(),
            phone: "+27 12 345 6795".to_string(),
            is_active: true,
            appointment_date: "2023-04-01".to_string(),
            term_end_date: None,
            conflict_of_interest: Vec::new(),
            attendance_rate: 100.0,
        },
    ];

    // Review items
    let mock_reviews = vec![
        ReviewItem {
            id: "rev_001".to_string(),
            reference_number: "NBAC/2025/0089".to_string(),
            tender_id: "tender_001".to_string(),
            tender_reference: "RFP-2025-0089".to_string(),
            tender_title: "IT Infrastructure Upgrade - Data Centre Modernization".to_string(),
            tender_value: 45_000_000.0,
            procurement_method: "Open Tender".to_string(),
            department: "Information Technology".to_string(),
            category: ReviewCategory::BidAward,
            status: ReviewStatus::Scheduled,
            priority: Priority::High,
            submitted_by: "Dr. Thandi Nkosi".to_string(),
            submitted_date: "2025-02-20".to_string(),
            scheduled_meeting_id: Some("mtg_001".to_string()),
            scheduled_meeting_date: Some("2025-03-05".to_string()),
            evaluation_summary: "Four bids received. Technical evaluation completed with Innovate IT Holdings scoring highest (87.3%). Recommended for award based on 80/20 scoring methodology.".to_string(),
            recommended_supplier_id: Some("sup_003".to_string()),
            recommended_supplier_name: Some("Innovate IT Holdings".to_string()),
            recommended_value: Some(47_200_000.0),
            recommendation: "Award contract to Innovate IT Holdings based on highest combined score".to_string(),
            bbbee_level: Some(1),
            local_content_percent: Some(65.0),
            compliance_checks: vec![
                ComplianceCheckResult { check_name: "PFMA Compliance".to_string(), passed: true, details: None },
                ComplianceCheckResult { check_name: "Treasury Regulations".to_string(), passed: true, details: None },
                ComplianceCheckResult { check_name: "B-BBEE Verification".to_string(), passed: true, details: Some("Level 1 verified".to_string()) },
                ComplianceCheckResult { check_name: "Tax Clearance".to_string(), passed: true, details: None },
                ComplianceCheckResult { check_name: "CIPC Registration".to_string(), passed: true, details: None },
            ],
            risk_rating: RiskRating::Medium,
            documents: vec![
                ReviewDocument {
                    id: "doc_001".to_string(),
                    name: "Bid Evaluation Report".to_string(),
                    document_type: DocumentType::EvaluationReport,
                    file_path: "/documents/rev_001/evaluation_report.pdf".to_string(),
                    uploaded_by: "Dr. Thandi Nkosi".to_string(),
                    uploaded_at: "2025-02-20T10:00:00Z".to_string(),
                    is_mandatory: true,
                },
                ReviewDocument {
                    id: "doc_002".to_string(),
                    name: "Technical Assessment".to_string(),
                    document_type: DocumentType::TechnicalReport,
                    file_path: "/documents/rev_001/technical_assessment.pdf".to_string(),
                    uploaded_by: "Mr. Johan Pretorius".to_string(),
                    uploaded_at: "2025-02-19T14:30:00Z".to_string(),
                    is_mandatory: true,
                },
            ],
            decision: None,
            notes: Vec::new(),
        },
        ReviewItem {
            id: "rev_002".to_string(),
            reference_number: "NBAC/2025/0076".to_string(),
            tender_id: "tender_002".to_string(),
            tender_reference: "RFP-2025-0076".to_string(),
            tender_title: "Security Services - Perimeter and Access Control".to_string(),
            tender_value: 18_500_000.0,
            procurement_method: "Open Tender".to_string(),
            department: "Facilities Management".to_string(),
            category: ReviewCategory::BidAward,
            status: ReviewStatus::Pending,
            priority: Priority::Medium,
            submitted_by: "Ms. Priya Naidoo".to_string(),
            submitted_date: "2025-02-22".to_string(),
            scheduled_meeting_id: None,
            scheduled_meeting_date: None,
            evaluation_summary: "Two bids received. SecureGuard SA recommended based on superior technical proposal and competitive pricing.".to_string(),
            recommended_supplier_id: Some("sup_005".to_string()),
            recommended_supplier_name: Some("SecureGuard SA".to_string()),
            recommended_value: Some(17_800_000.0),
            recommendation: "Award to SecureGuard SA".to_string(),
            bbbee_level: Some(1),
            local_content_percent: Some(85.0),
            compliance_checks: vec![
                ComplianceCheckResult { check_name: "PFMA Compliance".to_string(), passed: true, details: None },
                ComplianceCheckResult { check_name: "B-BBEE Verification".to_string(), passed: true, details: None },
                ComplianceCheckResult { check_name: "Tax Clearance".to_string(), passed: true, details: None },
            ],
            risk_rating: RiskRating::Low,
            documents: Vec::new(),
            decision: None,
            notes: Vec::new(),
        },
        ReviewItem {
            id: "rev_003".to_string(),
            reference_number: "NBAC/2025/0092".to_string(),
            tender_id: "tender_006".to_string(),
            tender_reference: "RFP-2025-0092".to_string(),
            tender_title: "Emergency Generator Installation - Head Office".to_string(),
            tender_value: 8_500_000.0,
            procurement_method: "Emergency Procurement".to_string(),
            department: "Facilities Management".to_string(),
            category: ReviewCategory::EmergencyProcurement,
            status: ReviewStatus::Pending,
            priority: Priority::Critical,
            submitted_by: "Mr. Thabo Molefe".to_string(),
            submitted_date: "2025-02-25".to_string(),
            scheduled_meeting_id: None,
            scheduled_meeting_date: None,
            evaluation_summary: "Emergency procurement required due to load shedding impact on critical operations. Single quote obtained from pre-qualified supplier.".to_string(),
            recommended_supplier_id: Some("sup_015".to_string()),
            recommended_supplier_name: Some("PowerGen Solutions".to_string()),
            recommended_value: Some(8_500_000.0),
            recommendation: "Approve emergency procurement".to_string(),
            bbbee_level: Some(2),
            local_content_percent: Some(70.0),
            compliance_checks: vec![
                ComplianceCheckResult { check_name: "Emergency Justification".to_string(), passed: true, details: Some("Business impact documented".to_string()) },
                ComplianceCheckResult { check_name: "Accounting Officer Approval".to_string(), passed: true, details: None },
            ],
            risk_rating: RiskRating::High,
            documents: Vec::new(),
            decision: None,
            notes: Vec::new(),
        },
        ReviewItem {
            id: "rev_004".to_string(),
            reference_number: "NBAC/2025/0065".to_string(),
            tender_id: "tender_004".to_string(),
            tender_reference: "RFP-2025-0065".to_string(),
            tender_title: "Professional Services - Internal Audit".to_string(),
            tender_value: 8_500_000.0,
            procurement_method: "Open Tender".to_string(),
            department: "Internal Audit".to_string(),
            category: ReviewCategory::BidAward,
            status: ReviewStatus::Approved,
            priority: Priority::Medium,
            submitted_by: "Ms. Priya Naidoo".to_string(),
            submitted_date: "2025-02-10".to_string(),
            scheduled_meeting_id: Some("mtg_002".to_string()),
            scheduled_meeting_date: Some("2025-02-15".to_string()),
            evaluation_summary: "AuditPro Consulting demonstrated exceptional technical capability and experience in public sector auditing.".to_string(),
            recommended_supplier_id: Some("sup_010".to_string()),
            recommended_supplier_name: Some("AuditPro Consulting".to_string()),
            recommended_value: Some(8_200_000.0),
            recommendation: "Award to AuditPro Consulting".to_string(),
            bbbee_level: Some(1),
            local_content_percent: Some(90.0),
            compliance_checks: vec![
                ComplianceCheckResult { check_name: "PFMA Compliance".to_string(), passed: true, details: None },
                ComplianceCheckResult { check_name: "B-BBEE Verification".to_string(), passed: true, details: None },
            ],
            risk_rating: RiskRating::Low,
            documents: Vec::new(),
            decision: Some(Decision {
                id: "dec_001".to_string(),
                review_id: "rev_004".to_string(),
                decision_type: DecisionType::Award,
                decision_date: "2025-02-15".to_string(),
                meeting_id: "mtg_002".to_string(),
                resolution_number: "NBAC/RES/2025/023".to_string(),
                summary: "Contract awarded to AuditPro Consulting".to_string(),
                rationale: "Highest technical score with competitive pricing and excellent B-BBEE credentials".to_string(),
                conditions: vec!["Subject to contract negotiation".to_string(), "Valid for 3 years with option to extend".to_string()],
                votes: vec![
                    Vote { member_id: "nbac_001".to_string(), member_name: "Dr. Nomvula Mokonyane".to_string(), vote: VoteType::For, reason: None, voted_at: "2025-02-15T10:30:00Z".to_string() },
                    Vote { member_id: "nbac_002".to_string(), member_name: "Mr. Thabo Molefe".to_string(), vote: VoteType::For, reason: None, voted_at: "2025-02-15T10:30:00Z".to_string() },
                    Vote { member_id: "nbac_005".to_string(), member_name: "Ms. Priya Naidoo".to_string(), vote: VoteType::Recused, reason: Some("Conflict of interest".to_string()), voted_at: "2025-02-15T10:30:00Z".to_string() },
                ],
                votes_for: 5,
                votes_against: 0,
                votes_abstain: 0,
                is_unanimous: true,
                effective_date: "2025-02-15".to_string(),
                approved_value: Some(8_200_000.0),
                awarded_supplier_id: Some("sup_010".to_string()),
                awarded_supplier_name: Some("AuditPro Consulting".to_string()),
                attachments: Vec::new(),
                recorded_by: "Ms. Sarah van Wyk".to_string(),
                confirmed_by: Some("Dr. Nomvula Mokonyane".to_string()),
                confirmed_at: Some("2025-02-16T08:00:00Z".to_string()),
            }),
            notes: Vec::new(),
        },
        ReviewItem {
            id: "rev_005".to_string(),
            reference_number: "NBAC/2025/0088".to_string(),
            tender_id: "ctr_012".to_string(),
            tender_reference: "CTR-2024-0156".to_string(),
            tender_title: "Cleaning Services Contract - 20% Value Increase".to_string(),
            tender_value: 12_000_000.0,
            procurement_method: "Contract Variation".to_string(),
            department: "Facilities Management".to_string(),
            category: ReviewCategory::ContractVariation,
            status: ReviewStatus::InReview,
            priority: Priority::High,
            submitted_by: "Mr. Sipho Dlamini".to_string(),
            submitted_date: "2025-02-24".to_string(),
            scheduled_meeting_id: Some("mtg_001".to_string()),
            scheduled_meeting_date: Some("2025-03-05".to_string()),
            evaluation_summary: "Request for 20% contract value increase due to expanded scope including new building. Variation within Treasury limits.".to_string(),
            recommended_supplier_id: None,
            recommended_supplier_name: Some("CleanCorp Services (existing contractor)".to_string()),
            recommended_value: Some(14_400_000.0),
            recommendation: "Approve variation subject to Treasury approval".to_string(),
            bbbee_level: Some(2),
            local_content_percent: Some(95.0),
            compliance_checks: vec![
                ComplianceCheckResult { check_name: "Variation within 20% limit".to_string(), passed: true, details: None },
                ComplianceCheckResult { check_name: "Contract still valid".to_string(), passed: true, details: None },
            ],
            risk_rating: RiskRating::Medium,
            documents: Vec::new(),
            decision: None,
            notes: Vec::new(),
        },
    ];

    // Meetings
    let mock_meetings = vec![
        Meeting {
            id: "mtg_001".to_string(),
            meeting_number: "NBAC/MTG/2025/008".to_string(),
            date: "2025-03-05".to_string(),
            time: "10:00".to_string(),
            venue: "Boardroom A, Head Office".to_string(),
            meeting_type: MeetingType::Ordinary,
            status: MeetingStatus::Scheduled,
            chairperson_id: "nbac_001".to_string(),
            chairperson_name: "Dr. Nomvula Mokonyane".to_string(),
            secretary_id: "nbac_007".to_string(),
            secretary_name: "Ms. Sarah van Wyk".to_string(),
            attendees: mock_members.iter().map(|m| MeetingAttendee {
                member_id: m.id.clone(),
                member_name: m.name.clone(),
                role: m.role,
                attended: false,
                arrival_time: None,
                departure_time: None,
                recusals: Vec::new(),
            }).collect(),
            apologies: Vec::new(),
            quorum_required: 5,
            quorum_present: 0,
            has_quorum: false,
            agenda: Agenda {
                id: "agd_001".to_string(),
                meeting_id: "mtg_001".to_string(),
                items: vec![
                    AgendaItem {
                        id: "agi_001".to_string(),
                        sequence: 1,
                        review_id: "rev_001".to_string(),
                        title: "IT Infrastructure Upgrade - Data Centre Modernization".to_string(),
                        presenter: "Dr. Thandi Nkosi".to_string(),
                        estimated_duration_minutes: 30,
                        status: AgendaItemStatus::Pending,
                        outcome: None,
                        resolution_number: None,
                    },
                    AgendaItem {
                        id: "agi_002".to_string(),
                        sequence: 2,
                        review_id: "rev_005".to_string(),
                        title: "Cleaning Services Contract Variation".to_string(),
                        presenter: "Mr. Sipho Dlamini".to_string(),
                        estimated_duration_minutes: 20,
                        status: AgendaItemStatus::Pending,
                        outcome: None,
                        resolution_number: None,
                    },
                ],
                previous_minutes_review: true,
                matters_arising: vec!["Follow-up on Fleet Management tender evaluation".to_string()],
                any_other_business: Vec::new(),
                next_meeting_date: Some("2025-03-19".to_string()),
                finalized: true,
                finalized_at: Some("2025-02-28T16:00:00Z".to_string()),
            },
            minutes_url: None,
            recording_url: None,
            created_at: "2025-02-20T09:00:00Z".to_string(),
            updated_at: "2025-02-28T16:00:00Z".to_string(),
        },
        Meeting {
            id: "mtg_002".to_string(),
            meeting_number: "NBAC/MTG/2025/007".to_string(),
            date: "2025-02-15".to_string(),
            time: "10:00".to_string(),
            venue: "Boardroom A, Head Office".to_string(),
            meeting_type: MeetingType::Ordinary,
            status: MeetingStatus::Completed,
            chairperson_id: "nbac_001".to_string(),
            chairperson_name: "Dr. Nomvula Mokonyane".to_string(),
            secretary_id: "nbac_007".to_string(),
            secretary_name: "Ms. Sarah van Wyk".to_string(),
            attendees: mock_members.iter().map(|m| MeetingAttendee {
                member_id: m.id.clone(),
                member_name: m.name.clone(),
                role: m.role,
                attended: true,
                arrival_time: Some("09:55".to_string()),
                departure_time: Some("12:30".to_string()),
                recusals: Vec::new(),
            }).collect(),
            apologies: Vec::new(),
            quorum_required: 5,
            quorum_present: 7,
            has_quorum: true,
            agenda: Agenda::default(),
            minutes_url: Some("/documents/meetings/mtg_002/minutes.pdf".to_string()),
            recording_url: None,
            created_at: "2025-02-01T09:00:00Z".to_string(),
            updated_at: "2025-02-16T09:00:00Z".to_string(),
        },
        Meeting {
            id: "mtg_003".to_string(),
            meeting_number: "NBAC/MTG/2025/009".to_string(),
            date: "2025-03-19".to_string(),
            time: "10:00".to_string(),
            venue: "Boardroom A, Head Office".to_string(),
            meeting_type: MeetingType::Ordinary,
            status: MeetingStatus::Scheduled,
            chairperson_id: "nbac_001".to_string(),
            chairperson_name: "Dr. Nomvula Mokonyane".to_string(),
            secretary_id: "nbac_007".to_string(),
            secretary_name: "Ms. Sarah van Wyk".to_string(),
            attendees: Vec::new(),
            apologies: Vec::new(),
            quorum_required: 5,
            quorum_present: 0,
            has_quorum: false,
            agenda: Agenda::default(),
            minutes_url: None,
            recording_url: None,
            created_at: "2025-02-28T16:00:00Z".to_string(),
            updated_at: "2025-02-28T16:00:00Z".to_string(),
        },
    ];

    // Decisions history
    let mock_decisions = vec![
        Decision {
            id: "dec_001".to_string(),
            review_id: "rev_004".to_string(),
            decision_type: DecisionType::Award,
            decision_date: "2025-02-15".to_string(),
            meeting_id: "mtg_002".to_string(),
            resolution_number: "NBAC/RES/2025/023".to_string(),
            summary: "Internal Audit Services - Award to AuditPro Consulting".to_string(),
            rationale: "Highest technical score with competitive pricing".to_string(),
            conditions: Vec::new(),
            votes: Vec::new(),
            votes_for: 5,
            votes_against: 0,
            votes_abstain: 1,
            is_unanimous: false,
            effective_date: "2025-02-15".to_string(),
            approved_value: Some(8_200_000.0),
            awarded_supplier_id: Some("sup_010".to_string()),
            awarded_supplier_name: Some("AuditPro Consulting".to_string()),
            attachments: Vec::new(),
            recorded_by: "Ms. Sarah van Wyk".to_string(),
            confirmed_by: Some("Dr. Nomvula Mokonyane".to_string()),
            confirmed_at: Some("2025-02-16T08:00:00Z".to_string()),
        },
        Decision {
            id: "dec_002".to_string(),
            review_id: "rev_010".to_string(),
            decision_type: DecisionType::Award,
            decision_date: "2025-02-15".to_string(),
            meeting_id: "mtg_002".to_string(),
            resolution_number: "NBAC/RES/2025/022".to_string(),
            summary: "Office Supplies - Award to OfficeMax SA".to_string(),
            rationale: "Lowest price with acceptable quality".to_string(),
            conditions: Vec::new(),
            votes: Vec::new(),
            votes_for: 6,
            votes_against: 0,
            votes_abstain: 0,
            is_unanimous: true,
            effective_date: "2025-02-15".to_string(),
            approved_value: Some(2_500_000.0),
            awarded_supplier_id: Some("sup_020".to_string()),
            awarded_supplier_name: Some("OfficeMax SA".to_string()),
            attachments: Vec::new(),
            recorded_by: "Ms. Sarah van Wyk".to_string(),
            confirmed_by: Some("Dr. Nomvula Mokonyane".to_string()),
            confirmed_at: Some("2025-02-16T08:00:00Z".to_string()),
        },
        Decision {
            id: "dec_003".to_string(),
            review_id: "rev_011".to_string(),
            decision_type: DecisionType::Defer,
            decision_date: "2025-02-01".to_string(),
            meeting_id: "mtg_001".to_string(),
            resolution_number: "NBAC/RES/2025/020".to_string(),
            summary: "Vehicle Fleet Expansion - Deferred pending budget confirmation".to_string(),
            rationale: "Budget allocation needs confirmation from Finance".to_string(),
            conditions: vec!["Re-submit once budget confirmed".to_string()],
            votes: Vec::new(),
            votes_for: 4,
            votes_against: 2,
            votes_abstain: 0,
            is_unanimous: false,
            effective_date: "2025-02-01".to_string(),
            approved_value: None,
            awarded_supplier_id: None,
            awarded_supplier_name: None,
            attachments: Vec::new(),
            recorded_by: "Ms. Sarah van Wyk".to_string(),
            confirmed_by: Some("Dr. Nomvula Mokonyane".to_string()),
            confirmed_at: Some("2025-02-02T08:00:00Z".to_string()),
        },
        Decision {
            id: "dec_004".to_string(),
            review_id: "rev_012".to_string(),
            decision_type: DecisionType::Extension,
            decision_date: "2025-01-18".to_string(),
            meeting_id: "mtg_000".to_string(),
            resolution_number: "NBAC/RES/2025/015".to_string(),
            summary: "Security Services Contract - 12 month extension approved".to_string(),
            rationale: "Continuity of service during re-tender process".to_string(),
            conditions: vec!["No price increase".to_string(), "Re-tender must commence by Q2".to_string()],
            votes: Vec::new(),
            votes_for: 6,
            votes_against: 0,
            votes_abstain: 0,
            is_unanimous: true,
            effective_date: "2025-02-01".to_string(),
            approved_value: Some(15_000_000.0),
            awarded_supplier_id: Some("sup_005".to_string()),
            awarded_supplier_name: Some("SecureGuard SA".to_string()),
            attachments: Vec::new(),
            recorded_by: "Ms. Sarah van Wyk".to_string(),
            confirmed_by: Some("Dr. Nomvula Mokonyane".to_string()),
            confirmed_at: Some("2025-01-19T08:00:00Z".to_string()),
        },
    ];

    // KPIs
    let kpis = NbacKpis {
        pending_reviews: 3,
        scheduled_reviews: 2,
        decisions_this_month: 4,
        decisions_ytd: 23,
        total_value_approved_ytd: 156_500_000.0,
        average_turnaround_days: 12.5,
        approval_rate: 87.5,
        upcoming_meetings: 2,
        overdue_reviews: 1,
    };

    store.committee_members.set(mock_members);
    store.reviews.set(mock_reviews);
    store.meetings.set(mock_meetings);
    store.decisions.set(mock_decisions);
    store.kpis.set(kpis);
}
