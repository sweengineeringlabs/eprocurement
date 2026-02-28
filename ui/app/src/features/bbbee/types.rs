//! B-BBEE domain types for South African Broad-Based Black Economic Empowerment compliance
//!
//! This module contains types for tracking B-BBEE spend targets, compliance metrics,
//! supplier classifications, and scorecard elements per the B-BBEE Codes of Good Practice.

use serde::{Deserialize, Serialize};

/// B-BBEE certification levels (1-8 + Non-Compliant)
/// Recognition levels per the Amended Codes of Good Practice
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

    /// B-BBEE procurement recognition level percentage
    /// Per the Amended Generic Codes of Good Practice
    pub fn recognition_level(&self) -> f64 {
        match self {
            BbbeeLevel::Level1 => 135.0,  // 135% recognition
            BbbeeLevel::Level2 => 125.0,  // 125% recognition
            BbbeeLevel::Level3 => 110.0,  // 110% recognition
            BbbeeLevel::Level4 => 100.0,  // 100% recognition
            BbbeeLevel::Level5 => 80.0,   // 80% recognition
            BbbeeLevel::Level6 => 60.0,   // 60% recognition
            BbbeeLevel::Level7 => 50.0,   // 50% recognition
            BbbeeLevel::Level8 => 10.0,   // 10% recognition
            BbbeeLevel::NonCompliant => 0.0,
        }
    }

    /// Color coding for display
    pub fn color(&self) -> &'static str {
        match self {
            BbbeeLevel::Level1 => "var(--green)",
            BbbeeLevel::Level2 => "var(--green)",
            BbbeeLevel::Level3 => "var(--blue)",
            BbbeeLevel::Level4 => "var(--blue)",
            BbbeeLevel::Level5 => "var(--orange)",
            BbbeeLevel::Level6 => "var(--orange)",
            BbbeeLevel::Level7 => "var(--red)",
            BbbeeLevel::Level8 => "var(--red)",
            BbbeeLevel::NonCompliant => "var(--text-muted)",
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

/// Supplier ownership classification per B-BBEE codes
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum OwnershipClassification {
    /// 51%+ Black ownership
    BlackOwned,
    /// 30%+ Black Women ownership
    BlackWomenOwned,
    /// Exempted Micro Enterprise (EME) - turnover < R10m
    EME,
    /// Qualifying Small Enterprise (QSE) - turnover R10m - R50m
    QSE,
    /// Generic Enterprise - turnover > R50m
    Generic,
    /// Youth-owned enterprise (51%+ ownership by persons under 35)
    YouthOwned,
    /// Persons with disabilities ownership
    DisabilityOwned,
    /// Military veteran owned
    MilitaryVeteranOwned,
    /// Rural or township based enterprise
    RuralTownship,
    /// Designated group supplier
    DesignatedGroup,
}

impl OwnershipClassification {
    pub fn label(&self) -> &'static str {
        match self {
            OwnershipClassification::BlackOwned => "Black Owned (51%+)",
            OwnershipClassification::BlackWomenOwned => "Black Women Owned (30%+)",
            OwnershipClassification::EME => "Exempted Micro Enterprise",
            OwnershipClassification::QSE => "Qualifying Small Enterprise",
            OwnershipClassification::Generic => "Generic Enterprise",
            OwnershipClassification::YouthOwned => "Youth Owned",
            OwnershipClassification::DisabilityOwned => "Disability Owned",
            OwnershipClassification::MilitaryVeteranOwned => "Military Veteran Owned",
            OwnershipClassification::RuralTownship => "Rural/Township Based",
            OwnershipClassification::DesignatedGroup => "Designated Group",
        }
    }

    /// Bonus points multiplier for preferential procurement
    pub fn bonus_multiplier(&self) -> f64 {
        match self {
            OwnershipClassification::BlackOwned => 1.0,
            OwnershipClassification::BlackWomenOwned => 1.1, // Additional recognition
            OwnershipClassification::EME => 1.0,
            OwnershipClassification::QSE => 1.0,
            OwnershipClassification::Generic => 1.0,
            OwnershipClassification::YouthOwned => 1.0,
            OwnershipClassification::DisabilityOwned => 1.0,
            OwnershipClassification::MilitaryVeteranOwned => 1.0,
            OwnershipClassification::RuralTownship => 1.0,
            OwnershipClassification::DesignatedGroup => 1.1,
        }
    }
}

/// B-BBEE spend target for a specific category or overall
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpendTarget {
    pub id: String,
    /// Target name (e.g., "Overall B-BBEE", "Black Owned", "EME/QSE")
    pub name: String,
    /// Description of the target
    pub description: String,
    /// Target percentage of total procurement spend
    pub target_percentage: f64,
    /// Actual percentage achieved
    pub actual_percentage: f64,
    /// Target spend amount in Rand
    pub target_amount: f64,
    /// Actual spend amount in Rand
    pub actual_amount: f64,
    /// Total measurable procurement spend
    pub total_spend: f64,
    /// Category (e.g., "Overall", "Goods", "Services", "Works")
    pub category: String,
    /// Financial year (e.g., "FY 2025/26")
    pub financial_year: String,
    /// Sub-targets for designated groups
    pub sub_targets: Vec<SubTarget>,
    /// Regulatory reference
    pub regulation: String,
    /// Status indicator
    pub status: TargetStatus,
    /// Variance from target
    pub variance: f64,
    /// Trend compared to previous period
    pub trend: Trend,
}

impl Default for SpendTarget {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            description: String::new(),
            target_percentage: 0.0,
            actual_percentage: 0.0,
            target_amount: 0.0,
            actual_amount: 0.0,
            total_spend: 0.0,
            category: "Overall".to_string(),
            financial_year: "FY 2025/26".to_string(),
            sub_targets: Vec::new(),
            regulation: String::new(),
            status: TargetStatus::OnTrack,
            variance: 0.0,
            trend: Trend::Stable,
        }
    }
}

/// Sub-target for designated group spending
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubTarget {
    pub name: String,
    pub target_percentage: f64,
    pub actual_percentage: f64,
    pub actual_amount: f64,
    pub classification: Option<OwnershipClassification>,
}

/// Target achievement status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum TargetStatus {
    /// Exceeding target
    Exceeding,
    /// On track to meet target
    OnTrack,
    /// Below target but within acceptable variance
    AtRisk,
    /// Significantly below target
    BelowTarget,
    /// Not applicable
    NotApplicable,
}

impl TargetStatus {
    pub fn label(&self) -> &'static str {
        match self {
            TargetStatus::Exceeding => "Exceeding",
            TargetStatus::OnTrack => "On Track",
            TargetStatus::AtRisk => "At Risk",
            TargetStatus::BelowTarget => "Below Target",
            TargetStatus::NotApplicable => "N/A",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            TargetStatus::Exceeding => "var(--green)",
            TargetStatus::OnTrack => "var(--blue)",
            TargetStatus::AtRisk => "var(--orange)",
            TargetStatus::BelowTarget => "var(--red)",
            TargetStatus::NotApplicable => "var(--text-muted)",
        }
    }
}

/// Trend indicator
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Trend {
    Increasing,
    Stable,
    Decreasing,
}

impl Trend {
    pub fn label(&self) -> &'static str {
        match self {
            Trend::Increasing => "Increasing",
            Trend::Stable => "Stable",
            Trend::Decreasing => "Decreasing",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Trend::Increasing => "arrow-up",
            Trend::Stable => "minus",
            Trend::Decreasing => "arrow-down",
        }
    }
}

/// B-BBEE level distribution breakdown
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LevelBreakdown {
    /// B-BBEE level
    pub level: BbbeeLevel,
    /// Number of suppliers at this level
    pub supplier_count: u32,
    /// Total spend with suppliers at this level
    pub spend_amount: f64,
    /// Percentage of total spend
    pub spend_percentage: f64,
    /// Recognized spend (after applying recognition level)
    pub recognized_spend: f64,
    /// Number of active contracts
    pub contract_count: u32,
    /// Average supplier performance score
    pub avg_performance: f64,
}

impl Default for LevelBreakdown {
    fn default() -> Self {
        Self {
            level: BbbeeLevel::NonCompliant,
            supplier_count: 0,
            spend_amount: 0.0,
            spend_percentage: 0.0,
            recognized_spend: 0.0,
            contract_count: 0,
            avg_performance: 0.0,
        }
    }
}

/// B-BBEE compliance metric
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComplianceMetric {
    pub id: String,
    /// Metric name
    pub name: String,
    /// Description
    pub description: String,
    /// Scorecard element (e.g., "Preferential Procurement", "Enterprise Development")
    pub scorecard_element: ScorecardElement,
    /// Weighting points (out of total scorecard)
    pub weighting_points: f64,
    /// Points achieved
    pub points_achieved: f64,
    /// Compliance percentage
    pub compliance_percentage: f64,
    /// Target value
    pub target_value: f64,
    /// Actual value
    pub actual_value: f64,
    /// Unit of measurement
    pub unit: String,
    /// Status
    pub status: ComplianceStatus,
    /// Last assessment date
    pub last_assessed: String,
    /// Next review date
    pub next_review: String,
    /// Evidence documents count
    pub evidence_count: u32,
    /// Notes/comments
    pub notes: String,
}

impl Default for ComplianceMetric {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            description: String::new(),
            scorecard_element: ScorecardElement::PreferentialProcurement,
            weighting_points: 0.0,
            points_achieved: 0.0,
            compliance_percentage: 0.0,
            target_value: 0.0,
            actual_value: 0.0,
            unit: "%".to_string(),
            status: ComplianceStatus::Compliant,
            last_assessed: String::new(),
            next_review: String::new(),
            evidence_count: 0,
            notes: String::new(),
        }
    }
}

/// B-BBEE Scorecard elements per Generic Codes
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ScorecardElement {
    /// Ownership element (25 points)
    Ownership,
    /// Management Control (19 points)
    ManagementControl,
    /// Skills Development (20 points)
    SkillsDevelopment,
    /// Enterprise and Supplier Development (40 points)
    EnterpriseSupplierDevelopment,
    /// Preferential Procurement (sub-element of ESD)
    PreferentialProcurement,
    /// Enterprise Development (sub-element of ESD)
    EnterpriseDevelopment,
    /// Supplier Development (sub-element of ESD)
    SupplierDevelopment,
    /// Socio-Economic Development (5 points)
    SocioEconomicDevelopment,
}

impl ScorecardElement {
    pub fn label(&self) -> &'static str {
        match self {
            ScorecardElement::Ownership => "Ownership",
            ScorecardElement::ManagementControl => "Management Control",
            ScorecardElement::SkillsDevelopment => "Skills Development",
            ScorecardElement::EnterpriseSupplierDevelopment => "Enterprise & Supplier Development",
            ScorecardElement::PreferentialProcurement => "Preferential Procurement",
            ScorecardElement::EnterpriseDevelopment => "Enterprise Development",
            ScorecardElement::SupplierDevelopment => "Supplier Development",
            ScorecardElement::SocioEconomicDevelopment => "Socio-Economic Development",
        }
    }

    /// Maximum weighting points per Generic Codes
    pub fn max_points(&self) -> f64 {
        match self {
            ScorecardElement::Ownership => 25.0,
            ScorecardElement::ManagementControl => 19.0,
            ScorecardElement::SkillsDevelopment => 20.0,
            ScorecardElement::EnterpriseSupplierDevelopment => 40.0,
            ScorecardElement::PreferentialProcurement => 25.0, // Sub-element
            ScorecardElement::EnterpriseDevelopment => 5.0,    // Sub-element
            ScorecardElement::SupplierDevelopment => 10.0,     // Sub-element
            ScorecardElement::SocioEconomicDevelopment => 5.0,
        }
    }
}

/// Compliance status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Compliant,
    PartiallyCompliant,
    NonCompliant,
    PendingReview,
    NotApplicable,
}

impl ComplianceStatus {
    pub fn label(&self) -> &'static str {
        match self {
            ComplianceStatus::Compliant => "Compliant",
            ComplianceStatus::PartiallyCompliant => "Partially Compliant",
            ComplianceStatus::NonCompliant => "Non-Compliant",
            ComplianceStatus::PendingReview => "Pending Review",
            ComplianceStatus::NotApplicable => "N/A",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            ComplianceStatus::Compliant => "var(--green)",
            ComplianceStatus::PartiallyCompliant => "var(--orange)",
            ComplianceStatus::NonCompliant => "var(--red)",
            ComplianceStatus::PendingReview => "var(--blue)",
            ComplianceStatus::NotApplicable => "var(--text-muted)",
        }
    }
}

/// Supplier B-BBEE classification summary
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupplierClassification {
    pub supplier_id: String,
    pub supplier_name: String,
    /// B-BBEE level
    pub bbbee_level: BbbeeLevel,
    /// Certificate number
    pub certificate_number: Option<String>,
    /// Certificate expiry date
    pub certificate_expiry: Option<String>,
    /// Verification agency (SANAS accredited)
    pub verification_agency: Option<String>,
    /// Ownership classifications
    pub ownership_classifications: Vec<OwnershipClassification>,
    /// Black ownership percentage
    pub black_ownership_percent: f64,
    /// Black women ownership percentage
    pub black_women_ownership_percent: f64,
    /// Is EME (turnover < R10m)
    pub is_eme: bool,
    /// Is QSE (turnover R10m - R50m)
    pub is_qse: bool,
    /// Enterprise size category
    pub enterprise_size: EnterpriseSize,
    /// Annual turnover
    pub annual_turnover: Option<f64>,
    /// Total spend with this supplier
    pub total_spend: f64,
    /// Recognition level percentage
    pub recognition_level: f64,
    /// Recognized spend value
    pub recognized_spend: f64,
    /// Province
    pub province: String,
    /// Is township/rural based
    pub is_township_rural: bool,
    /// Verification status
    pub verification_status: VerificationStatus,
    /// Last verification date
    pub last_verified: Option<String>,
}

impl Default for SupplierClassification {
    fn default() -> Self {
        Self {
            supplier_id: String::new(),
            supplier_name: String::new(),
            bbbee_level: BbbeeLevel::NonCompliant,
            certificate_number: None,
            certificate_expiry: None,
            verification_agency: None,
            ownership_classifications: Vec::new(),
            black_ownership_percent: 0.0,
            black_women_ownership_percent: 0.0,
            is_eme: false,
            is_qse: false,
            enterprise_size: EnterpriseSize::Generic,
            annual_turnover: None,
            total_spend: 0.0,
            recognition_level: 0.0,
            recognized_spend: 0.0,
            province: String::new(),
            is_township_rural: false,
            verification_status: VerificationStatus::Unverified,
            last_verified: None,
        }
    }
}

/// Enterprise size classification
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum EnterpriseSize {
    /// Exempted Micro Enterprise - turnover <= R10 million
    EME,
    /// Qualifying Small Enterprise - turnover > R10m and <= R50m
    QSE,
    /// Generic Enterprise - turnover > R50 million
    Generic,
}

impl EnterpriseSize {
    pub fn label(&self) -> &'static str {
        match self {
            EnterpriseSize::EME => "EME (< R10m)",
            EnterpriseSize::QSE => "QSE (R10m - R50m)",
            EnterpriseSize::Generic => "Generic (> R50m)",
        }
    }

    pub fn from_turnover(turnover: f64) -> Self {
        if turnover <= 10_000_000.0 {
            EnterpriseSize::EME
        } else if turnover <= 50_000_000.0 {
            EnterpriseSize::QSE
        } else {
            EnterpriseSize::Generic
        }
    }
}

impl Default for EnterpriseSize {
    fn default() -> Self {
        EnterpriseSize::Generic
    }
}

/// B-BBEE certificate verification status
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum VerificationStatus {
    /// Verified with valid certificate
    Verified,
    /// Certificate expired
    Expired,
    /// Pending verification
    Pending,
    /// Unverified / no certificate
    Unverified,
    /// Sworn affidavit (for EME/QSE)
    SwornAffidavit,
}

impl VerificationStatus {
    pub fn label(&self) -> &'static str {
        match self {
            VerificationStatus::Verified => "Verified",
            VerificationStatus::Expired => "Expired",
            VerificationStatus::Pending => "Pending",
            VerificationStatus::Unverified => "Unverified",
            VerificationStatus::SwornAffidavit => "Sworn Affidavit",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            VerificationStatus::Verified => "var(--green)",
            VerificationStatus::Expired => "var(--red)",
            VerificationStatus::Pending => "var(--orange)",
            VerificationStatus::Unverified => "var(--text-muted)",
            VerificationStatus::SwornAffidavit => "var(--blue)",
        }
    }
}

/// B-BBEE Goals Summary KPIs
#[derive(Clone, Debug, Default)]
pub struct BbbeeKpis {
    /// Overall B-BBEE spend percentage
    pub overall_bbbee_percent: f64,
    /// Target B-BBEE spend percentage
    pub target_bbbee_percent: f64,
    /// Total measurable procurement spend
    pub total_mpsp: f64,
    /// Total B-BBEE compliant spend
    pub total_bbbee_spend: f64,
    /// Total recognized spend
    pub total_recognized_spend: f64,
    /// Level 1-2 supplier spend percentage
    pub level_1_2_percent: f64,
    /// Level 1-4 supplier spend percentage
    pub level_1_4_percent: f64,
    /// Black owned spend percentage
    pub black_owned_percent: f64,
    /// Black women owned spend percentage
    pub black_women_owned_percent: f64,
    /// EME/QSE spend percentage
    pub eme_qse_percent: f64,
    /// Designated group spend percentage
    pub designated_group_percent: f64,
    /// Number of B-BBEE compliant suppliers
    pub compliant_supplier_count: u32,
    /// Number of non-compliant suppliers
    pub non_compliant_supplier_count: u32,
    /// Number of expiring certificates (within 90 days)
    pub expiring_certificates: u32,
    /// Scorecard points achieved
    pub scorecard_points: f64,
    /// Maximum scorecard points
    pub max_scorecard_points: f64,
    /// Projected B-BBEE level
    pub projected_level: BbbeeLevel,
}

/// Monthly B-BBEE spend trend data point
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BbbeeTrendPoint {
    pub period: String,
    pub total_spend: f64,
    pub bbbee_spend: f64,
    pub bbbee_percent: f64,
    pub target_percent: f64,
}

/// Filter criteria for B-BBEE data
#[derive(Clone, Debug, Default)]
pub struct BbbeeFilter {
    pub financial_year: Option<String>,
    pub category: Option<String>,
    pub bbbee_level: Option<BbbeeLevel>,
    pub ownership_classification: Option<OwnershipClassification>,
    pub province: Option<String>,
    pub enterprise_size: Option<EnterpriseSize>,
    pub verification_status: Option<VerificationStatus>,
    pub search_query: Option<String>,
}

/// Designated group spending summary
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DesignatedGroupSpend {
    pub classification: OwnershipClassification,
    pub supplier_count: u32,
    pub spend_amount: f64,
    pub spend_percentage: f64,
    pub target_percentage: f64,
    pub variance: f64,
}

/// Provincial B-BBEE distribution
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProvincialDistribution {
    pub province: String,
    pub supplier_count: u32,
    pub total_spend: f64,
    pub bbbee_spend: f64,
    pub bbbee_percentage: f64,
    pub township_rural_spend: f64,
}
