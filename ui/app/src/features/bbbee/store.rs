//! B-BBEE store - state management for B-BBEE compliance tracking

use components::prelude::*;
use super::types::{
    SpendTarget, SubTarget, LevelBreakdown, ComplianceMetric, SupplierClassification,
    BbbeeKpis, BbbeeFilter, BbbeeTrendPoint, DesignatedGroupSpend, ProvincialDistribution,
    BbbeeLevel, OwnershipClassification, ScorecardElement, TargetStatus, Trend,
    ComplianceStatus, EnterpriseSize, VerificationStatus,
};

/// B-BBEE state store
#[derive(Clone)]
pub struct BbbeeStore {
    /// Spend targets (overall and by category)
    pub spend_targets: Signal<Vec<SpendTarget>>,
    /// Level distribution breakdown
    pub level_breakdown: Signal<Vec<LevelBreakdown>>,
    /// Compliance metrics
    pub compliance_metrics: Signal<Vec<ComplianceMetric>>,
    /// Supplier classifications
    pub supplier_classifications: Signal<Vec<SupplierClassification>>,
    /// Summary KPIs
    pub kpis: Signal<BbbeeKpis>,
    /// Monthly trend data
    pub trend_data: Signal<Vec<BbbeeTrendPoint>>,
    /// Designated group spend breakdown
    pub designated_group_spend: Signal<Vec<DesignatedGroupSpend>>,
    /// Provincial distribution
    pub provincial_distribution: Signal<Vec<ProvincialDistribution>>,
    /// Current filter
    pub filter: Signal<BbbeeFilter>,
    /// Loading state
    pub loading: Signal<bool>,
    /// Error state
    pub error: Signal<Option<String>>,
    /// Selected view/tab
    pub selected_view: Signal<String>,
}

impl BbbeeStore {
    pub fn new() -> Self {
        Self {
            spend_targets: signal(Vec::new()),
            level_breakdown: signal(Vec::new()),
            compliance_metrics: signal(Vec::new()),
            supplier_classifications: signal(Vec::new()),
            kpis: signal(BbbeeKpis::default()),
            trend_data: signal(Vec::new()),
            designated_group_spend: signal(Vec::new()),
            provincial_distribution: signal(Vec::new()),
            filter: signal(BbbeeFilter::default()),
            loading: signal(false),
            error: signal(None),
            selected_view: signal("overview".to_string()),
        }
    }
}

/// Load mock B-BBEE data for demo
pub fn load_mock_data(store: &BbbeeStore) {
    // Total measurable procurement spend
    let total_mpsp = 847_000_000.0;

    // Spend targets
    let spend_targets = vec![
        SpendTarget {
            id: "TGT-001".to_string(),
            name: "Overall B-BBEE Spend".to_string(),
            description: "Total procurement spend with B-BBEE compliant suppliers".to_string(),
            target_percentage: 80.0,
            actual_percentage: 78.4,
            target_amount: 677_600_000.0,
            actual_amount: 664_048_000.0,
            total_spend: total_mpsp,
            category: "Overall".to_string(),
            financial_year: "FY 2025/26".to_string(),
            sub_targets: vec![
                SubTarget {
                    name: "Level 1-2 Suppliers".to_string(),
                    target_percentage: 40.0,
                    actual_percentage: 42.0,
                    actual_amount: 355_740_000.0,
                    classification: None,
                },
                SubTarget {
                    name: "Level 3-4 Suppliers".to_string(),
                    target_percentage: 25.0,
                    actual_percentage: 23.5,
                    actual_amount: 199_045_000.0,
                    classification: None,
                },
            ],
            regulation: "PPPFA & B-BBEE Act".to_string(),
            status: TargetStatus::AtRisk,
            variance: -1.6,
            trend: Trend::Increasing,
        },
        SpendTarget {
            id: "TGT-002".to_string(),
            name: "Black Owned Enterprises".to_string(),
            description: "Spend with 51%+ Black owned enterprises".to_string(),
            target_percentage: 40.0,
            actual_percentage: 38.5,
            target_amount: 338_800_000.0,
            actual_amount: 326_095_000.0,
            total_spend: total_mpsp,
            category: "Ownership".to_string(),
            financial_year: "FY 2025/26".to_string(),
            sub_targets: Vec::new(),
            regulation: "Generic Codes Code 400".to_string(),
            status: TargetStatus::AtRisk,
            variance: -1.5,
            trend: Trend::Increasing,
        },
        SpendTarget {
            id: "TGT-003".to_string(),
            name: "Black Women Owned Enterprises".to_string(),
            description: "Spend with 30%+ Black Women owned enterprises".to_string(),
            target_percentage: 12.0,
            actual_percentage: 14.2,
            target_amount: 101_640_000.0,
            actual_amount: 120_274_000.0,
            total_spend: total_mpsp,
            category: "Ownership".to_string(),
            financial_year: "FY 2025/26".to_string(),
            sub_targets: Vec::new(),
            regulation: "Generic Codes Code 400".to_string(),
            status: TargetStatus::Exceeding,
            variance: 2.2,
            trend: Trend::Increasing,
        },
        SpendTarget {
            id: "TGT-004".to_string(),
            name: "EME/QSE Spend".to_string(),
            description: "Spend with Exempted Micro Enterprises and Qualifying Small Enterprises".to_string(),
            target_percentage: 30.0,
            actual_percentage: 28.8,
            target_amount: 254_100_000.0,
            actual_amount: 243_936_000.0,
            total_spend: total_mpsp,
            category: "Enterprise Size".to_string(),
            financial_year: "FY 2025/26".to_string(),
            sub_targets: vec![
                SubTarget {
                    name: "EME Spend".to_string(),
                    target_percentage: 15.0,
                    actual_percentage: 12.5,
                    actual_amount: 105_875_000.0,
                    classification: Some(OwnershipClassification::EME),
                },
                SubTarget {
                    name: "QSE Spend".to_string(),
                    target_percentage: 15.0,
                    actual_percentage: 16.3,
                    actual_amount: 138_061_000.0,
                    classification: Some(OwnershipClassification::QSE),
                },
            ],
            regulation: "Generic Codes Code 400".to_string(),
            status: TargetStatus::AtRisk,
            variance: -1.2,
            trend: Trend::Stable,
        },
        SpendTarget {
            id: "TGT-005".to_string(),
            name: "Designated Group Suppliers".to_string(),
            description: "Spend with youth, disabled, military veteran, and rural/township suppliers".to_string(),
            target_percentage: 2.0,
            actual_percentage: 2.8,
            target_amount: 16_940_000.0,
            actual_amount: 23_716_000.0,
            total_spend: total_mpsp,
            category: "Designated Groups".to_string(),
            financial_year: "FY 2025/26".to_string(),
            sub_targets: vec![
                SubTarget {
                    name: "Youth Owned".to_string(),
                    target_percentage: 0.5,
                    actual_percentage: 0.8,
                    actual_amount: 6_776_000.0,
                    classification: Some(OwnershipClassification::YouthOwned),
                },
                SubTarget {
                    name: "Township/Rural".to_string(),
                    target_percentage: 1.0,
                    actual_percentage: 1.5,
                    actual_amount: 12_705_000.0,
                    classification: Some(OwnershipClassification::RuralTownship),
                },
                SubTarget {
                    name: "Disability Owned".to_string(),
                    target_percentage: 0.3,
                    actual_percentage: 0.3,
                    actual_amount: 2_541_000.0,
                    classification: Some(OwnershipClassification::DisabilityOwned),
                },
                SubTarget {
                    name: "Military Veteran".to_string(),
                    target_percentage: 0.2,
                    actual_percentage: 0.2,
                    actual_amount: 1_694_000.0,
                    classification: Some(OwnershipClassification::MilitaryVeteranOwned),
                },
            ],
            regulation: "Generic Codes Code 400".to_string(),
            status: TargetStatus::Exceeding,
            variance: 0.8,
            trend: Trend::Increasing,
        },
    ];

    // Level breakdown
    let level_breakdown = vec![
        LevelBreakdown {
            level: BbbeeLevel::Level1,
            supplier_count: 485,
            spend_amount: 356_000_000.0,
            spend_percentage: 42.0,
            recognized_spend: 480_600_000.0, // 135% recognition
            contract_count: 156,
            avg_performance: 91.2,
        },
        LevelBreakdown {
            level: BbbeeLevel::Level2,
            supplier_count: 312,
            spend_amount: 237_000_000.0,
            spend_percentage: 28.0,
            recognized_spend: 296_250_000.0, // 125% recognition
            contract_count: 98,
            avg_performance: 88.5,
        },
        LevelBreakdown {
            level: BbbeeLevel::Level3,
            supplier_count: 124,
            spend_amount: 95_000_000.0,
            spend_percentage: 11.2,
            recognized_spend: 104_500_000.0, // 110% recognition
            contract_count: 45,
            avg_performance: 85.3,
        },
        LevelBreakdown {
            level: BbbeeLevel::Level4,
            supplier_count: 87,
            spend_amount: 57_500_000.0,
            spend_percentage: 6.8,
            recognized_spend: 57_500_000.0, // 100% recognition
            contract_count: 32,
            avg_performance: 82.1,
        },
        LevelBreakdown {
            level: BbbeeLevel::Level5,
            supplier_count: 45,
            spend_amount: 28_000_000.0,
            spend_percentage: 3.3,
            recognized_spend: 22_400_000.0, // 80% recognition
            contract_count: 18,
            avg_performance: 78.5,
        },
        LevelBreakdown {
            level: BbbeeLevel::Level6,
            supplier_count: 32,
            spend_amount: 15_000_000.0,
            spend_percentage: 1.8,
            recognized_spend: 9_000_000.0, // 60% recognition
            contract_count: 12,
            avg_performance: 75.2,
        },
        LevelBreakdown {
            level: BbbeeLevel::Level7,
            supplier_count: 18,
            spend_amount: 5_800_000.0,
            spend_percentage: 0.7,
            recognized_spend: 2_900_000.0, // 50% recognition
            contract_count: 8,
            avg_performance: 72.8,
        },
        LevelBreakdown {
            level: BbbeeLevel::Level8,
            supplier_count: 12,
            spend_amount: 2_000_000.0,
            spend_percentage: 0.2,
            recognized_spend: 200_000.0, // 10% recognition
            contract_count: 4,
            avg_performance: 70.5,
        },
        LevelBreakdown {
            level: BbbeeLevel::NonCompliant,
            supplier_count: 78,
            spend_amount: 50_700_000.0,
            spend_percentage: 6.0,
            recognized_spend: 0.0, // 0% recognition
            contract_count: 28,
            avg_performance: 68.2,
        },
    ];

    // Compliance metrics
    let compliance_metrics = vec![
        ComplianceMetric {
            id: "CM-001".to_string(),
            name: "B-BBEE Procurement Spend".to_string(),
            description: "Procurement spend from B-BBEE compliant suppliers as percentage of TMPS".to_string(),
            scorecard_element: ScorecardElement::PreferentialProcurement,
            weighting_points: 25.0,
            points_achieved: 22.8,
            compliance_percentage: 91.2,
            target_value: 80.0,
            actual_value: 78.4,
            unit: "%".to_string(),
            status: ComplianceStatus::PartiallyCompliant,
            last_assessed: "2026-01-15".to_string(),
            next_review: "2026-04-15".to_string(),
            evidence_count: 45,
            notes: "Slightly below target. Action plan in place to increase EME/QSE spend.".to_string(),
        },
        ComplianceMetric {
            id: "CM-002".to_string(),
            name: "Supplier Development Contributions".to_string(),
            description: "Enterprise development contributions as percentage of NPAT".to_string(),
            scorecard_element: ScorecardElement::SupplierDevelopment,
            weighting_points: 10.0,
            points_achieved: 9.5,
            compliance_percentage: 95.0,
            target_value: 2.0,
            actual_value: 1.9,
            unit: "%".to_string(),
            status: ComplianceStatus::Compliant,
            last_assessed: "2026-01-15".to_string(),
            next_review: "2026-04-15".to_string(),
            evidence_count: 28,
            notes: "On track with supplier development program.".to_string(),
        },
        ComplianceMetric {
            id: "CM-003".to_string(),
            name: "Enterprise Development".to_string(),
            description: "Qualifying contributions to enterprise development beneficiaries".to_string(),
            scorecard_element: ScorecardElement::EnterpriseDevelopment,
            weighting_points: 5.0,
            points_achieved: 5.0,
            compliance_percentage: 100.0,
            target_value: 1.0,
            actual_value: 1.2,
            unit: "%".to_string(),
            status: ComplianceStatus::Compliant,
            last_assessed: "2026-01-15".to_string(),
            next_review: "2026-04-15".to_string(),
            evidence_count: 15,
            notes: "Exceeding target through incubation program.".to_string(),
        },
        ComplianceMetric {
            id: "CM-004".to_string(),
            name: "Black Owned Procurement".to_string(),
            description: "Procurement from 51%+ Black owned suppliers".to_string(),
            scorecard_element: ScorecardElement::PreferentialProcurement,
            weighting_points: 9.0,
            points_achieved: 8.1,
            compliance_percentage: 90.0,
            target_value: 40.0,
            actual_value: 38.5,
            unit: "%".to_string(),
            status: ComplianceStatus::PartiallyCompliant,
            last_assessed: "2026-01-15".to_string(),
            next_review: "2026-04-15".to_string(),
            evidence_count: 32,
            notes: "Implementing enhanced Black owned supplier sourcing program.".to_string(),
        },
        ComplianceMetric {
            id: "CM-005".to_string(),
            name: "Black Women Owned Procurement".to_string(),
            description: "Procurement from 30%+ Black Women owned suppliers".to_string(),
            scorecard_element: ScorecardElement::PreferentialProcurement,
            weighting_points: 4.0,
            points_achieved: 4.0,
            compliance_percentage: 100.0,
            target_value: 12.0,
            actual_value: 14.2,
            unit: "%".to_string(),
            status: ComplianceStatus::Compliant,
            last_assessed: "2026-01-15".to_string(),
            next_review: "2026-04-15".to_string(),
            evidence_count: 22,
            notes: "Exceeding target. Women-owned business program successful.".to_string(),
        },
        ComplianceMetric {
            id: "CM-006".to_string(),
            name: "EME/QSE Procurement".to_string(),
            description: "Procurement from EME and QSE suppliers".to_string(),
            scorecard_element: ScorecardElement::PreferentialProcurement,
            weighting_points: 5.0,
            points_achieved: 4.5,
            compliance_percentage: 90.0,
            target_value: 30.0,
            actual_value: 28.8,
            unit: "%".to_string(),
            status: ComplianceStatus::PartiallyCompliant,
            last_assessed: "2026-01-15".to_string(),
            next_review: "2026-04-15".to_string(),
            evidence_count: 38,
            notes: "Small enterprise development initiatives underway.".to_string(),
        },
        ComplianceMetric {
            id: "CM-007".to_string(),
            name: "Designated Group Procurement".to_string(),
            description: "Procurement from youth, disabled, military veteran and rural suppliers".to_string(),
            scorecard_element: ScorecardElement::PreferentialProcurement,
            weighting_points: 2.0,
            points_achieved: 2.0,
            compliance_percentage: 100.0,
            target_value: 2.0,
            actual_value: 2.8,
            unit: "%".to_string(),
            status: ComplianceStatus::Compliant,
            last_assessed: "2026-01-15".to_string(),
            next_review: "2026-04-15".to_string(),
            evidence_count: 18,
            notes: "Exceeding target through dedicated outreach programs.".to_string(),
        },
    ];

    // Supplier classifications (sample top suppliers)
    let supplier_classifications = vec![
        SupplierClassification {
            supplier_id: "SUP-001".to_string(),
            supplier_name: "TechSolutions SA (Pty) Ltd".to_string(),
            bbbee_level: BbbeeLevel::Level1,
            certificate_number: Some("SANAS-2025-12345".to_string()),
            certificate_expiry: Some("2026-08-15".to_string()),
            verification_agency: Some("Empowerdex".to_string()),
            ownership_classifications: vec![OwnershipClassification::BlackOwned, OwnershipClassification::YouthOwned],
            black_ownership_percent: 65.0,
            black_women_ownership_percent: 30.0,
            is_eme: false,
            is_qse: false,
            enterprise_size: EnterpriseSize::Generic,
            annual_turnover: Some(120_000_000.0),
            total_spend: 45_000_000.0,
            recognition_level: 135.0,
            recognized_spend: 60_750_000.0,
            province: "Gauteng".to_string(),
            is_township_rural: false,
            verification_status: VerificationStatus::Verified,
            last_verified: Some("2025-08-15".to_string()),
        },
        SupplierClassification {
            supplier_id: "SUP-002".to_string(),
            supplier_name: "Ubuntu Construction Group".to_string(),
            bbbee_level: BbbeeLevel::Level2,
            certificate_number: Some("SANAS-2025-22341".to_string()),
            certificate_expiry: Some("2026-05-20".to_string()),
            verification_agency: Some("AQRate".to_string()),
            ownership_classifications: vec![OwnershipClassification::BlackOwned],
            black_ownership_percent: 75.0,
            black_women_ownership_percent: 15.0,
            is_eme: false,
            is_qse: false,
            enterprise_size: EnterpriseSize::Generic,
            annual_turnover: Some(85_000_000.0),
            total_spend: 28_500_000.0,
            recognition_level: 125.0,
            recognized_spend: 35_625_000.0,
            province: "Gauteng".to_string(),
            is_township_rural: false,
            verification_status: VerificationStatus::Verified,
            last_verified: Some("2025-05-20".to_string()),
        },
        SupplierClassification {
            supplier_id: "SUP-003".to_string(),
            supplier_name: "Cape Catering Services CC".to_string(),
            bbbee_level: BbbeeLevel::Level1,
            certificate_number: Some("SANAS-2025-33456".to_string()),
            certificate_expiry: Some("2026-11-10".to_string()),
            verification_agency: Some("Empowerdex".to_string()),
            ownership_classifications: vec![OwnershipClassification::BlackWomenOwned, OwnershipClassification::QSE],
            black_ownership_percent: 100.0,
            black_women_ownership_percent: 60.0,
            is_eme: false,
            is_qse: true,
            enterprise_size: EnterpriseSize::QSE,
            annual_turnover: Some(25_000_000.0),
            total_spend: 8_200_000.0,
            recognition_level: 135.0,
            recognized_spend: 11_070_000.0,
            province: "Western Cape".to_string(),
            is_township_rural: false,
            verification_status: VerificationStatus::Verified,
            last_verified: Some("2025-11-10".to_string()),
        },
        SupplierClassification {
            supplier_id: "SUP-004".to_string(),
            supplier_name: "SecureGuard Holdings (Pty) Ltd".to_string(),
            bbbee_level: BbbeeLevel::Level1,
            certificate_number: Some("SANAS-2025-44567".to_string()),
            certificate_expiry: Some("2026-03-25".to_string()),
            verification_agency: Some("AQRate".to_string()),
            ownership_classifications: vec![OwnershipClassification::BlackOwned, OwnershipClassification::MilitaryVeteranOwned],
            black_ownership_percent: 80.0,
            black_women_ownership_percent: 20.0,
            is_eme: false,
            is_qse: false,
            enterprise_size: EnterpriseSize::Generic,
            annual_turnover: Some(95_000_000.0),
            total_spend: 35_000_000.0,
            recognition_level: 135.0,
            recognized_spend: 47_250_000.0,
            province: "KwaZulu-Natal".to_string(),
            is_township_rural: false,
            verification_status: VerificationStatus::Verified,
            last_verified: Some("2025-03-25".to_string()),
        },
        SupplierClassification {
            supplier_id: "SUP-005".to_string(),
            supplier_name: "GreenTech Environmental Solutions".to_string(),
            bbbee_level: BbbeeLevel::Level1,
            certificate_number: Some("SANAS-2025-55678".to_string()),
            certificate_expiry: Some("2026-07-18".to_string()),
            verification_agency: Some("BEE Matrix".to_string()),
            ownership_classifications: vec![OwnershipClassification::BlackWomenOwned],
            black_ownership_percent: 55.0,
            black_women_ownership_percent: 55.0,
            is_eme: false,
            is_qse: true,
            enterprise_size: EnterpriseSize::QSE,
            annual_turnover: Some(35_000_000.0),
            total_spend: 15_500_000.0,
            recognition_level: 135.0,
            recognized_spend: 20_925_000.0,
            province: "Gauteng".to_string(),
            is_township_rural: false,
            verification_status: VerificationStatus::Verified,
            last_verified: Some("2025-07-18".to_string()),
        },
        SupplierClassification {
            supplier_id: "SUP-006".to_string(),
            supplier_name: "Soweto Tech Hub".to_string(),
            bbbee_level: BbbeeLevel::Level1,
            certificate_number: None,
            certificate_expiry: None,
            verification_agency: None,
            ownership_classifications: vec![OwnershipClassification::BlackOwned, OwnershipClassification::EME, OwnershipClassification::YouthOwned, OwnershipClassification::RuralTownship],
            black_ownership_percent: 100.0,
            black_women_ownership_percent: 40.0,
            is_eme: true,
            is_qse: false,
            enterprise_size: EnterpriseSize::EME,
            annual_turnover: Some(6_500_000.0),
            total_spend: 2_800_000.0,
            recognition_level: 135.0,
            recognized_spend: 3_780_000.0,
            province: "Gauteng".to_string(),
            is_township_rural: true,
            verification_status: VerificationStatus::SwornAffidavit,
            last_verified: Some("2025-12-01".to_string()),
        },
        SupplierClassification {
            supplier_id: "SUP-007".to_string(),
            supplier_name: "Mpumalanga Transport Logistics".to_string(),
            bbbee_level: BbbeeLevel::Level2,
            certificate_number: Some("SANAS-2025-77890".to_string()),
            certificate_expiry: Some("2026-09-30".to_string()),
            verification_agency: Some("Empowerdex".to_string()),
            ownership_classifications: vec![OwnershipClassification::BlackOwned],
            black_ownership_percent: 70.0,
            black_women_ownership_percent: 25.0,
            is_eme: false,
            is_qse: false,
            enterprise_size: EnterpriseSize::Generic,
            annual_turnover: Some(68_000_000.0),
            total_spend: 18_000_000.0,
            recognition_level: 125.0,
            recognized_spend: 22_500_000.0,
            province: "Mpumalanga".to_string(),
            is_township_rural: false,
            verification_status: VerificationStatus::Verified,
            last_verified: Some("2025-09-30".to_string()),
        },
        SupplierClassification {
            supplier_id: "SUP-008".to_string(),
            supplier_name: "Eastern Cape Medical Supplies".to_string(),
            bbbee_level: BbbeeLevel::Level4,
            certificate_number: Some("SANAS-2025-88901".to_string()),
            certificate_expiry: Some("2026-04-12".to_string()),
            verification_agency: Some("AQRate".to_string()),
            ownership_classifications: vec![OwnershipClassification::BlackOwned],
            black_ownership_percent: 52.0,
            black_women_ownership_percent: 10.0,
            is_eme: false,
            is_qse: true,
            enterprise_size: EnterpriseSize::QSE,
            annual_turnover: Some(42_000_000.0),
            total_spend: 6_800_000.0,
            recognition_level: 100.0,
            recognized_spend: 6_800_000.0,
            province: "Eastern Cape".to_string(),
            is_township_rural: false,
            verification_status: VerificationStatus::Verified,
            last_verified: Some("2025-04-12".to_string()),
        },
    ];

    // Monthly trend data for FY 2025/26
    let trend_data = vec![
        BbbeeTrendPoint { period: "Apr".to_string(), total_spend: 68_500_000.0, bbbee_spend: 52_985_000.0, bbbee_percent: 77.3, target_percent: 80.0 },
        BbbeeTrendPoint { period: "May".to_string(), total_spend: 72_300_000.0, bbbee_spend: 56_394_000.0, bbbee_percent: 78.0, target_percent: 80.0 },
        BbbeeTrendPoint { period: "Jun".to_string(), total_spend: 85_200_000.0, bbbee_spend: 66_456_000.0, bbbee_percent: 78.0, target_percent: 80.0 },
        BbbeeTrendPoint { period: "Jul".to_string(), total_spend: 78_900_000.0, bbbee_spend: 61_542_000.0, bbbee_percent: 78.0, target_percent: 80.0 },
        BbbeeTrendPoint { period: "Aug".to_string(), total_spend: 81_400_000.0, bbbee_spend: 63_492_000.0, bbbee_percent: 78.0, target_percent: 80.0 },
        BbbeeTrendPoint { period: "Sep".to_string(), total_spend: 92_100_000.0, bbbee_spend: 72_039_000.0, bbbee_percent: 78.2, target_percent: 80.0 },
        BbbeeTrendPoint { period: "Oct".to_string(), total_spend: 88_700_000.0, bbbee_spend: 69_586_000.0, bbbee_percent: 78.4, target_percent: 80.0 },
        BbbeeTrendPoint { period: "Nov".to_string(), total_spend: 95_300_000.0, bbbee_spend: 75_137_000.0, bbbee_percent: 78.8, target_percent: 80.0 },
        BbbeeTrendPoint { period: "Dec".to_string(), total_spend: 76_200_000.0, bbbee_spend: 60_198_000.0, bbbee_percent: 79.0, target_percent: 80.0 },
        BbbeeTrendPoint { period: "Jan".to_string(), total_spend: 108_400_000.0, bbbee_spend: 85_659_200.0, bbbee_percent: 79.0, target_percent: 80.0 },
    ];

    // Designated group spend breakdown
    let designated_group_spend = vec![
        DesignatedGroupSpend {
            classification: OwnershipClassification::BlackOwned,
            supplier_count: 542,
            spend_amount: 326_095_000.0,
            spend_percentage: 38.5,
            target_percentage: 40.0,
            variance: -1.5,
        },
        DesignatedGroupSpend {
            classification: OwnershipClassification::BlackWomenOwned,
            supplier_count: 245,
            spend_amount: 120_274_000.0,
            spend_percentage: 14.2,
            target_percentage: 12.0,
            variance: 2.2,
        },
        DesignatedGroupSpend {
            classification: OwnershipClassification::EME,
            supplier_count: 312,
            spend_amount: 105_875_000.0,
            spend_percentage: 12.5,
            target_percentage: 15.0,
            variance: -2.5,
        },
        DesignatedGroupSpend {
            classification: OwnershipClassification::QSE,
            supplier_count: 187,
            spend_amount: 138_061_000.0,
            spend_percentage: 16.3,
            target_percentage: 15.0,
            variance: 1.3,
        },
        DesignatedGroupSpend {
            classification: OwnershipClassification::YouthOwned,
            supplier_count: 68,
            spend_amount: 6_776_000.0,
            spend_percentage: 0.8,
            target_percentage: 0.5,
            variance: 0.3,
        },
        DesignatedGroupSpend {
            classification: OwnershipClassification::RuralTownship,
            supplier_count: 95,
            spend_amount: 12_705_000.0,
            spend_percentage: 1.5,
            target_percentage: 1.0,
            variance: 0.5,
        },
        DesignatedGroupSpend {
            classification: OwnershipClassification::DisabilityOwned,
            supplier_count: 18,
            spend_amount: 2_541_000.0,
            spend_percentage: 0.3,
            target_percentage: 0.3,
            variance: 0.0,
        },
        DesignatedGroupSpend {
            classification: OwnershipClassification::MilitaryVeteranOwned,
            supplier_count: 12,
            spend_amount: 1_694_000.0,
            spend_percentage: 0.2,
            target_percentage: 0.2,
            variance: 0.0,
        },
    ];

    // Provincial distribution
    let provincial_distribution = vec![
        ProvincialDistribution { province: "Gauteng".to_string(), supplier_count: 542, total_spend: 380_000_000.0, bbbee_spend: 304_000_000.0, bbbee_percentage: 80.0, township_rural_spend: 8_500_000.0 },
        ProvincialDistribution { province: "Western Cape".to_string(), supplier_count: 234, total_spend: 165_000_000.0, bbbee_spend: 128_700_000.0, bbbee_percentage: 78.0, township_rural_spend: 2_200_000.0 },
        ProvincialDistribution { province: "KwaZulu-Natal".to_string(), supplier_count: 187, total_spend: 120_000_000.0, bbbee_spend: 94_800_000.0, bbbee_percentage: 79.0, township_rural_spend: 1_800_000.0 },
        ProvincialDistribution { province: "Eastern Cape".to_string(), supplier_count: 98, total_spend: 68_000_000.0, bbbee_spend: 52_360_000.0, bbbee_percentage: 77.0, township_rural_spend: 1_200_000.0 },
        ProvincialDistribution { province: "Mpumalanga".to_string(), supplier_count: 67, total_spend: 42_000_000.0, bbbee_spend: 33_600_000.0, bbbee_percentage: 80.0, township_rural_spend: 950_000.0 },
        ProvincialDistribution { province: "Free State".to_string(), supplier_count: 54, total_spend: 28_000_000.0, bbbee_spend: 21_840_000.0, bbbee_percentage: 78.0, township_rural_spend: 650_000.0 },
        ProvincialDistribution { province: "Limpopo".to_string(), supplier_count: 48, total_spend: 22_000_000.0, bbbee_spend: 17_160_000.0, bbbee_percentage: 78.0, township_rural_spend: 850_000.0 },
        ProvincialDistribution { province: "North West".to_string(), supplier_count: 35, total_spend: 15_000_000.0, bbbee_spend: 11_550_000.0, bbbee_percentage: 77.0, township_rural_spend: 450_000.0 },
        ProvincialDistribution { province: "Northern Cape".to_string(), supplier_count: 19, total_spend: 7_000_000.0, bbbee_spend: 5_320_000.0, bbbee_percentage: 76.0, township_rural_spend: 200_000.0 },
    ];

    // Summary KPIs
    let kpis = BbbeeKpis {
        overall_bbbee_percent: 78.4,
        target_bbbee_percent: 80.0,
        total_mpsp: total_mpsp,
        total_bbbee_spend: 664_048_000.0,
        total_recognized_spend: 973_350_000.0,
        level_1_2_percent: 70.0,
        level_1_4_percent: 88.0,
        black_owned_percent: 38.5,
        black_women_owned_percent: 14.2,
        eme_qse_percent: 28.8,
        designated_group_percent: 2.8,
        compliant_supplier_count: 1115,
        non_compliant_supplier_count: 78,
        expiring_certificates: 23,
        scorecard_points: 53.9,
        max_scorecard_points: 60.0,
        projected_level: BbbeeLevel::Level2,
    };

    // Set all store values
    store.spend_targets.set(spend_targets);
    store.level_breakdown.set(level_breakdown);
    store.compliance_metrics.set(compliance_metrics);
    store.supplier_classifications.set(supplier_classifications);
    store.kpis.set(kpis);
    store.trend_data.set(trend_data);
    store.designated_group_spend.set(designated_group_spend);
    store.provincial_distribution.set(provincial_distribution);
}

/// Update filter
pub fn set_filter(store: &BbbeeStore, filter: BbbeeFilter) {
    store.filter.set(filter);
}

/// Update selected view/tab
pub fn set_view(store: &BbbeeStore, view: String) {
    store.selected_view.set(view);
}

/// Clear all filters
pub fn clear_filters(store: &BbbeeStore) {
    store.filter.set(BbbeeFilter::default());
}

/// Filter suppliers by B-BBEE level
pub fn filter_by_level(store: &BbbeeStore, level: Option<BbbeeLevel>) {
    let mut filter = store.filter.get();
    filter.bbbee_level = level;
    store.filter.set(filter);
}

/// Filter suppliers by ownership classification
pub fn filter_by_ownership(store: &BbbeeStore, classification: Option<OwnershipClassification>) {
    let mut filter = store.filter.get();
    filter.ownership_classification = classification;
    store.filter.set(filter);
}

/// Filter suppliers by enterprise size
pub fn filter_by_enterprise_size(store: &BbbeeStore, size: Option<EnterpriseSize>) {
    let mut filter = store.filter.get();
    filter.enterprise_size = size;
    store.filter.set(filter);
}
