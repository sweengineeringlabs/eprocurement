//! Evaluation store

use components::prelude::*;
use super::types::{
    TenderEvaluation, Bid, EvaluationCriterion, CommitteeMember,
    CommitteeNote, CriterionScore, EvaluationStatus, CriterionCategory,
    ScoringMethod,
};

/// Evaluation state store
#[derive(Clone)]
pub struct EvaluationStore {
    pub evaluations: Signal<Vec<TenderEvaluation>>,
    pub selected: Signal<Option<TenderEvaluation>>,
    pub selected_bid: Signal<Option<Bid>>,
    pub scoring_in_progress: Signal<bool>,
    pub current_scores: Signal<Vec<CriterionScore>>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
    pub filter_status: Signal<Option<EvaluationStatus>>,
}

impl EvaluationStore {
    pub fn new() -> Self {
        Self {
            evaluations: signal(Vec::new()),
            selected: signal(None),
            selected_bid: signal(None),
            scoring_in_progress: signal(false),
            current_scores: signal(Vec::new()),
            loading: signal(false),
            error: signal(None),
            filter_status: signal(None),
        }
    }

    /// Select an evaluation for scoring
    pub fn select_evaluation(&self, evaluation_id: &str) {
        let evaluations = self.evaluations.get();
        if let Some(eval) = evaluations.iter().find(|e| e.id == evaluation_id) {
            self.selected.set(Some(eval.clone()));
            self.scoring_in_progress.set(false);
            self.current_scores.set(Vec::new());
        }
    }

    /// Select a bid for detailed scoring
    pub fn select_bid(&self, bid_id: &str) {
        if let Some(eval) = self.selected.get().as_ref() {
            if let Some(bid) = eval.bids.iter().find(|b| b.id == bid_id) {
                self.selected_bid.set(Some(bid.clone()));
                self.current_scores.set(bid.scores.clone());
            }
        }
    }

    /// Start scoring session
    pub fn start_scoring(&self) {
        self.scoring_in_progress.set(true);
    }

    /// Update a criterion score
    pub fn update_score(&self, criterion_id: &str, score: u32, comment: Option<String>) {
        let mut scores = self.current_scores.get().clone();

        if let Some(existing) = scores.iter_mut().find(|s| s.criterion_id == criterion_id) {
            existing.score = score;
            existing.comment = comment;
        } else {
            scores.push(CriterionScore {
                criterion_id: criterion_id.to_string(),
                score,
                comment,
                scored_by: "current_user".to_string(),
                scored_at: "2025-02-27T10:00:00Z".to_string(),
            });
        }

        self.current_scores.set(scores);
    }

    /// Clear selection
    pub fn clear_selection(&self) {
        self.selected.set(None);
        self.selected_bid.set(None);
        self.scoring_in_progress.set(false);
        self.current_scores.set(Vec::new());
    }
}

/// Load mock evaluation data for demo
pub fn load_mock_data(store: &EvaluationStore) {
    let mock_criteria = vec![
        EvaluationCriterion {
            id: "crit_001".to_string(),
            name: "Technical Approach".to_string(),
            description: "Assessment of proposed methodology and technical solution".to_string(),
            weight: 25.0,
            max_score: 10,
            category: CriterionCategory::Technical,
        },
        EvaluationCriterion {
            id: "crit_002".to_string(),
            name: "Team Experience".to_string(),
            description: "Qualifications and experience of proposed team".to_string(),
            weight: 20.0,
            max_score: 10,
            category: CriterionCategory::Experience,
        },
        EvaluationCriterion {
            id: "crit_003".to_string(),
            name: "Past Performance".to_string(),
            description: "Track record on similar projects".to_string(),
            weight: 15.0,
            max_score: 10,
            category: CriterionCategory::Experience,
        },
        EvaluationCriterion {
            id: "crit_004".to_string(),
            name: "B-BBEE Compliance".to_string(),
            description: "B-BBEE level and empowerment credentials".to_string(),
            weight: 20.0,
            max_score: 10,
            category: CriterionCategory::Bbbee,
        },
        EvaluationCriterion {
            id: "crit_005".to_string(),
            name: "Local Content".to_string(),
            description: "Local manufacturing and job creation".to_string(),
            weight: 10.0,
            max_score: 10,
            category: CriterionCategory::LocalContent,
        },
        EvaluationCriterion {
            id: "crit_006".to_string(),
            name: "Price Competitiveness".to_string(),
            description: "Value for money assessment".to_string(),
            weight: 10.0,
            max_score: 10,
            category: CriterionCategory::Financial,
        },
    ];

    let mock_committee = vec![
        CommitteeMember {
            id: "mem_001".to_string(),
            name: "Dr. Thandi Nkosi".to_string(),
            role: "Chairperson".to_string(),
            department: "Supply Chain Management".to_string(),
            has_scored: true,
            conflict_declared: false,
        },
        CommitteeMember {
            id: "mem_002".to_string(),
            name: "Mr. Johan van der Berg".to_string(),
            role: "Technical Expert".to_string(),
            department: "IT Infrastructure".to_string(),
            has_scored: true,
            conflict_declared: false,
        },
        CommitteeMember {
            id: "mem_003".to_string(),
            name: "Ms. Priya Naidoo".to_string(),
            role: "Finance Representative".to_string(),
            department: "Finance".to_string(),
            has_scored: false,
            conflict_declared: false,
        },
        CommitteeMember {
            id: "mem_004".to_string(),
            name: "Mr. Sipho Dlamini".to_string(),
            role: "End User Representative".to_string(),
            department: "Operations".to_string(),
            has_scored: false,
            conflict_declared: false,
        },
        CommitteeMember {
            id: "mem_005".to_string(),
            name: "Adv. Sarah Mokoena".to_string(),
            role: "Legal Advisor".to_string(),
            department: "Legal Services".to_string(),
            has_scored: true,
            conflict_declared: false,
        },
    ];

    let mock_evaluations = vec![
        TenderEvaluation {
            id: "eval_001".to_string(),
            tender_id: "tender_001".to_string(),
            tender_reference: "RFP-2025-0089".to_string(),
            tender_title: "IT Infrastructure Upgrade - Data Centre Modernization".to_string(),
            tender_value: 45_000_000.0,
            closing_date: "2025-02-15".to_string(),
            evaluation_start: Some("2025-02-18".to_string()),
            evaluation_deadline: "2025-03-05".to_string(),
            status: EvaluationStatus::InProgress,
            bids: vec![
                Bid {
                    id: "bid_001".to_string(),
                    supplier_id: "sup_001".to_string(),
                    supplier_name: "TechSolutions SA (Pty) Ltd".to_string(),
                    bbbee_level: 1,
                    submitted_at: "2025-02-14T16:45:00Z".to_string(),
                    total_price: 42_500_000.0,
                    technical_score: Some(82.5),
                    financial_score: Some(94.1),
                    total_score: Some(84.8),
                    scores: vec![
                        CriterionScore { criterion_id: "crit_001".to_string(), score: 8, comment: Some("Strong technical proposal".to_string()), scored_by: "mem_001".to_string(), scored_at: "2025-02-20T09:00:00Z".to_string() },
                        CriterionScore { criterion_id: "crit_002".to_string(), score: 9, comment: Some("Highly experienced team".to_string()), scored_by: "mem_001".to_string(), scored_at: "2025-02-20T09:15:00Z".to_string() },
                        CriterionScore { criterion_id: "crit_003".to_string(), score: 8, comment: None, scored_by: "mem_001".to_string(), scored_at: "2025-02-20T09:30:00Z".to_string() },
                        CriterionScore { criterion_id: "crit_004".to_string(), score: 10, comment: Some("Level 1 B-BBEE, 51% black owned".to_string()), scored_by: "mem_001".to_string(), scored_at: "2025-02-20T09:45:00Z".to_string() },
                        CriterionScore { criterion_id: "crit_005".to_string(), score: 7, comment: None, scored_by: "mem_001".to_string(), scored_at: "2025-02-20T10:00:00Z".to_string() },
                        CriterionScore { criterion_id: "crit_006".to_string(), score: 8, comment: Some("Competitive pricing".to_string()), scored_by: "mem_001".to_string(), scored_at: "2025-02-20T10:15:00Z".to_string() },
                    ],
                    rank: Some(1),
                    recommendation: Some("Recommended for award".to_string()),
                },
                Bid {
                    id: "bid_002".to_string(),
                    supplier_id: "sup_002".to_string(),
                    supplier_name: "DataCore Systems".to_string(),
                    bbbee_level: 2,
                    submitted_at: "2025-02-15T09:30:00Z".to_string(),
                    total_price: 39_800_000.0,
                    technical_score: Some(75.0),
                    financial_score: Some(100.0),
                    total_score: Some(80.0),
                    scores: vec![
                        CriterionScore { criterion_id: "crit_001".to_string(), score: 7, comment: Some("Adequate technical solution".to_string()), scored_by: "mem_001".to_string(), scored_at: "2025-02-21T09:00:00Z".to_string() },
                        CriterionScore { criterion_id: "crit_002".to_string(), score: 7, comment: None, scored_by: "mem_001".to_string(), scored_at: "2025-02-21T09:15:00Z".to_string() },
                        CriterionScore { criterion_id: "crit_003".to_string(), score: 8, comment: Some("Good track record".to_string()), scored_by: "mem_001".to_string(), scored_at: "2025-02-21T09:30:00Z".to_string() },
                        CriterionScore { criterion_id: "crit_004".to_string(), score: 8, comment: Some("Level 2 B-BBEE".to_string()), scored_by: "mem_001".to_string(), scored_at: "2025-02-21T09:45:00Z".to_string() },
                        CriterionScore { criterion_id: "crit_005".to_string(), score: 6, comment: None, scored_by: "mem_001".to_string(), scored_at: "2025-02-21T10:00:00Z".to_string() },
                        CriterionScore { criterion_id: "crit_006".to_string(), score: 10, comment: Some("Lowest price".to_string()), scored_by: "mem_001".to_string(), scored_at: "2025-02-21T10:15:00Z".to_string() },
                    ],
                    rank: Some(2),
                    recommendation: None,
                },
                Bid {
                    id: "bid_003".to_string(),
                    supplier_id: "sup_003".to_string(),
                    supplier_name: "Innovate IT Holdings".to_string(),
                    bbbee_level: 1,
                    submitted_at: "2025-02-14T11:00:00Z".to_string(),
                    total_price: 47_200_000.0,
                    technical_score: Some(88.0),
                    financial_score: Some(84.3),
                    total_score: Some(87.3),
                    scores: vec![
                        CriterionScore { criterion_id: "crit_001".to_string(), score: 9, comment: Some("Excellent technical approach".to_string()), scored_by: "mem_001".to_string(), scored_at: "2025-02-22T09:00:00Z".to_string() },
                        CriterionScore { criterion_id: "crit_002".to_string(), score: 9, comment: Some("Industry-leading expertise".to_string()), scored_by: "mem_001".to_string(), scored_at: "2025-02-22T09:15:00Z".to_string() },
                        CriterionScore { criterion_id: "crit_003".to_string(), score: 9, comment: Some("Outstanding references".to_string()), scored_by: "mem_001".to_string(), scored_at: "2025-02-22T09:30:00Z".to_string() },
                        CriterionScore { criterion_id: "crit_004".to_string(), score: 10, comment: Some("Level 1, 100% black owned".to_string()), scored_by: "mem_001".to_string(), scored_at: "2025-02-22T09:45:00Z".to_string() },
                        CriterionScore { criterion_id: "crit_005".to_string(), score: 8, comment: None, scored_by: "mem_001".to_string(), scored_at: "2025-02-22T10:00:00Z".to_string() },
                        CriterionScore { criterion_id: "crit_006".to_string(), score: 6, comment: Some("Higher price but justified".to_string()), scored_by: "mem_001".to_string(), scored_at: "2025-02-22T10:15:00Z".to_string() },
                    ],
                    rank: Some(1),
                    recommendation: Some("Highest functionality score".to_string()),
                },
                Bid {
                    id: "bid_004".to_string(),
                    supplier_id: "sup_004".to_string(),
                    supplier_name: "GlobalTech Partners".to_string(),
                    bbbee_level: 4,
                    submitted_at: "2025-02-15T14:20:00Z".to_string(),
                    total_price: 41_000_000.0,
                    technical_score: Some(68.0),
                    financial_score: Some(97.1),
                    total_score: Some(73.8),
                    scores: vec![
                        CriterionScore { criterion_id: "crit_001".to_string(), score: 7, comment: None, scored_by: "mem_001".to_string(), scored_at: "2025-02-23T09:00:00Z".to_string() },
                        CriterionScore { criterion_id: "crit_002".to_string(), score: 6, comment: Some("Limited local experience".to_string()), scored_by: "mem_001".to_string(), scored_at: "2025-02-23T09:15:00Z".to_string() },
                        CriterionScore { criterion_id: "crit_003".to_string(), score: 7, comment: None, scored_by: "mem_001".to_string(), scored_at: "2025-02-23T09:30:00Z".to_string() },
                        CriterionScore { criterion_id: "crit_004".to_string(), score: 5, comment: Some("Level 4 B-BBEE".to_string()), scored_by: "mem_001".to_string(), scored_at: "2025-02-23T09:45:00Z".to_string() },
                        CriterionScore { criterion_id: "crit_005".to_string(), score: 4, comment: Some("Low local content".to_string()), scored_by: "mem_001".to_string(), scored_at: "2025-02-23T10:00:00Z".to_string() },
                        CriterionScore { criterion_id: "crit_006".to_string(), score: 9, comment: None, scored_by: "mem_001".to_string(), scored_at: "2025-02-23T10:15:00Z".to_string() },
                    ],
                    rank: Some(4),
                    recommendation: Some("Does not meet minimum B-BBEE requirements".to_string()),
                },
            ],
            criteria: mock_criteria.clone(),
            committee_members: mock_committee.clone(),
            notes: vec![
                CommitteeNote {
                    id: "note_001".to_string(),
                    author_id: "mem_001".to_string(),
                    author_name: "Dr. Thandi Nkosi".to_string(),
                    content: "All committee members must complete scoring by 28 February. Please ensure conflict of interest declarations are up to date.".to_string(),
                    created_at: "2025-02-18T08:00:00Z".to_string(),
                    is_confidential: false,
                },
                CommitteeNote {
                    id: "note_002".to_string(),
                    author_id: "mem_002".to_string(),
                    author_name: "Mr. Johan van der Berg".to_string(),
                    content: "Technical clarifications received from TechSolutions SA and Innovate IT Holdings. Documents uploaded to tender portal.".to_string(),
                    created_at: "2025-02-20T14:30:00Z".to_string(),
                    is_confidential: false,
                },
            ],
            scoring_method: ScoringMethod::Functionality80Price20,
            min_technical_score: 70.0,
            price_weight: 20.0,
            functionality_weight: 80.0,
        },
        TenderEvaluation {
            id: "eval_002".to_string(),
            tender_id: "tender_002".to_string(),
            tender_reference: "RFP-2025-0076".to_string(),
            tender_title: "Security Services - Perimeter and Access Control".to_string(),
            tender_value: 18_500_000.0,
            closing_date: "2025-02-10".to_string(),
            evaluation_start: Some("2025-02-12".to_string()),
            evaluation_deadline: "2025-02-28".to_string(),
            status: EvaluationStatus::Completed,
            bids: vec![
                Bid {
                    id: "bid_005".to_string(),
                    supplier_id: "sup_005".to_string(),
                    supplier_name: "SecureGuard SA".to_string(),
                    bbbee_level: 1,
                    submitted_at: "2025-02-09T15:00:00Z".to_string(),
                    total_price: 17_800_000.0,
                    technical_score: Some(85.0),
                    financial_score: Some(96.2),
                    total_score: Some(87.2),
                    scores: vec![],
                    rank: Some(1),
                    recommendation: Some("Recommended for award".to_string()),
                },
                Bid {
                    id: "bid_006".to_string(),
                    supplier_id: "sup_006".to_string(),
                    supplier_name: "ProtectCo Services".to_string(),
                    bbbee_level: 2,
                    submitted_at: "2025-02-10T08:45:00Z".to_string(),
                    total_price: 19_200_000.0,
                    technical_score: Some(78.0),
                    financial_score: Some(89.1),
                    total_score: Some(80.2),
                    scores: vec![],
                    rank: Some(2),
                    recommendation: None,
                },
            ],
            criteria: mock_criteria.clone(),
            committee_members: mock_committee.clone(),
            notes: vec![],
            scoring_method: ScoringMethod::Functionality80Price20,
            min_technical_score: 70.0,
            price_weight: 20.0,
            functionality_weight: 80.0,
        },
        TenderEvaluation {
            id: "eval_003".to_string(),
            tender_id: "tender_003".to_string(),
            tender_reference: "RFP-2025-0092".to_string(),
            tender_title: "Fleet Management Services - Vehicle Leasing".to_string(),
            tender_value: 32_000_000.0,
            closing_date: "2025-02-20".to_string(),
            evaluation_start: None,
            evaluation_deadline: "2025-03-10".to_string(),
            status: EvaluationStatus::Pending,
            bids: vec![
                Bid {
                    id: "bid_007".to_string(),
                    supplier_id: "sup_007".to_string(),
                    supplier_name: "FleetMaster SA".to_string(),
                    bbbee_level: 2,
                    submitted_at: "2025-02-19T16:30:00Z".to_string(),
                    total_price: 30_500_000.0,
                    technical_score: None,
                    financial_score: None,
                    total_score: None,
                    scores: vec![],
                    rank: None,
                    recommendation: None,
                },
                Bid {
                    id: "bid_008".to_string(),
                    supplier_id: "sup_008".to_string(),
                    supplier_name: "AutoLease Holdings".to_string(),
                    bbbee_level: 3,
                    submitted_at: "2025-02-20T11:00:00Z".to_string(),
                    total_price: 29_800_000.0,
                    technical_score: None,
                    financial_score: None,
                    total_score: None,
                    scores: vec![],
                    rank: None,
                    recommendation: None,
                },
                Bid {
                    id: "bid_009".to_string(),
                    supplier_id: "sup_009".to_string(),
                    supplier_name: "DriveRight Solutions".to_string(),
                    bbbee_level: 1,
                    submitted_at: "2025-02-20T14:45:00Z".to_string(),
                    total_price: 31_200_000.0,
                    technical_score: None,
                    financial_score: None,
                    total_score: None,
                    scores: vec![],
                    rank: None,
                    recommendation: None,
                },
            ],
            criteria: mock_criteria.clone(),
            committee_members: mock_committee.clone(),
            notes: vec![],
            scoring_method: ScoringMethod::Functionality90Price10,
            min_technical_score: 75.0,
            price_weight: 10.0,
            functionality_weight: 90.0,
        },
        TenderEvaluation {
            id: "eval_004".to_string(),
            tender_id: "tender_004".to_string(),
            tender_reference: "RFP-2025-0065".to_string(),
            tender_title: "Professional Services - Internal Audit".to_string(),
            tender_value: 8_500_000.0,
            closing_date: "2025-01-31".to_string(),
            evaluation_start: Some("2025-02-03".to_string()),
            evaluation_deadline: "2025-02-20".to_string(),
            status: EvaluationStatus::Approved,
            bids: vec![
                Bid {
                    id: "bid_010".to_string(),
                    supplier_id: "sup_010".to_string(),
                    supplier_name: "AuditPro Consulting".to_string(),
                    bbbee_level: 1,
                    submitted_at: "2025-01-30T12:00:00Z".to_string(),
                    total_price: 8_200_000.0,
                    technical_score: Some(92.0),
                    financial_score: Some(96.4),
                    total_score: Some(92.9),
                    scores: vec![],
                    rank: Some(1),
                    recommendation: Some("Award approved by NBAC".to_string()),
                },
            ],
            criteria: mock_criteria.clone(),
            committee_members: mock_committee.clone(),
            notes: vec![],
            scoring_method: ScoringMethod::Functionality80Price20,
            min_technical_score: 80.0,
            price_weight: 20.0,
            functionality_weight: 80.0,
        },
        TenderEvaluation {
            id: "eval_005".to_string(),
            tender_id: "tender_005".to_string(),
            tender_reference: "RFP-2025-0088".to_string(),
            tender_title: "Office Furniture Supply and Installation".to_string(),
            tender_value: 5_200_000.0,
            closing_date: "2025-02-25".to_string(),
            evaluation_start: None,
            evaluation_deadline: "2025-03-15".to_string(),
            status: EvaluationStatus::Pending,
            bids: vec![
                Bid {
                    id: "bid_011".to_string(),
                    supplier_id: "sup_011".to_string(),
                    supplier_name: "OfficePro Furniture".to_string(),
                    bbbee_level: 1,
                    submitted_at: "2025-02-24T10:00:00Z".to_string(),
                    total_price: 4_950_000.0,
                    technical_score: None,
                    financial_score: None,
                    total_score: None,
                    scores: vec![],
                    rank: None,
                    recommendation: None,
                },
                Bid {
                    id: "bid_012".to_string(),
                    supplier_id: "sup_012".to_string(),
                    supplier_name: "WorkSpace Solutions".to_string(),
                    bbbee_level: 2,
                    submitted_at: "2025-02-25T09:30:00Z".to_string(),
                    total_price: 5_100_000.0,
                    technical_score: None,
                    financial_score: None,
                    total_score: None,
                    scores: vec![],
                    rank: None,
                    recommendation: None,
                },
            ],
            criteria: mock_criteria,
            committee_members: mock_committee,
            notes: vec![],
            scoring_method: ScoringMethod::PriceOnly,
            min_technical_score: 60.0,
            price_weight: 100.0,
            functionality_weight: 0.0,
        },
    ];

    store.evaluations.set(mock_evaluations);
}
