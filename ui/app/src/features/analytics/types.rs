//! Analytics domain types

use serde::{Deserialize, Serialize};

/// Time period for analytics filtering
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum TimePeriod {
    /// Current month
    ThisMonth,
    /// Current quarter
    ThisQuarter,
    /// Current financial year (April to March)
    ThisYear,
    /// Last 12 months
    Last12Months,
    /// Custom date range
    Custom,
}

impl TimePeriod {
    pub fn label(&self) -> &'static str {
        match self {
            TimePeriod::ThisMonth => "This Month",
            TimePeriod::ThisQuarter => "This Quarter",
            TimePeriod::ThisYear => "FY 2025/26",
            TimePeriod::Last12Months => "Last 12 Months",
            TimePeriod::Custom => "Custom Range",
        }
    }
}

impl Default for TimePeriod {
    fn default() -> Self {
        TimePeriod::ThisYear
    }
}

/// Spend trend data point
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrendDataPoint {
    pub period: String,
    pub value: f64,
    pub budget: Option<f64>,
}

/// Overall spend trends
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrendData {
    pub monthly_spend: Vec<TrendDataPoint>,
    pub cumulative_spend: Vec<TrendDataPoint>,
    pub year_over_year: Vec<TrendDataPoint>,
}

impl Default for TrendData {
    fn default() -> Self {
        Self {
            monthly_spend: Vec::new(),
            cumulative_spend: Vec::new(),
            year_over_year: Vec::new(),
        }
    }
}

/// Spend category breakdown
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CategoryBreakdown {
    pub code: String,
    pub name: String,
    pub spend: f64,
    pub budget: f64,
    pub variance: f64,
    pub percentage: f64,
    pub contract_count: u32,
    pub supplier_count: u32,
    pub color: String,
}

/// Supplier performance metrics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupplierMetrics {
    pub supplier_id: String,
    pub supplier_name: String,
    pub total_spend: f64,
    pub contract_count: u32,
    pub average_delivery_time: f64,
    pub quality_score: f64,
    pub compliance_score: f64,
    pub overall_rating: f64,
    pub bbbee_level: u8,
    pub province: String,
}

/// B-BBEE metrics for spend analysis
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BbbeeMetrics {
    pub level_1_spend: f64,
    pub level_1_percent: f64,
    pub level_2_spend: f64,
    pub level_2_percent: f64,
    pub level_3_4_spend: f64,
    pub level_3_4_percent: f64,
    pub level_5_8_spend: f64,
    pub level_5_8_percent: f64,
    pub non_compliant_spend: f64,
    pub non_compliant_percent: f64,
    pub total_bbbee_spend: f64,
    pub bbbee_target: f64,
    pub bbbee_actual: f64,
    pub suppliers_by_level: Vec<BbbeeLevelCount>,
}

/// Count of suppliers by B-BBEE level
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BbbeeLevelCount {
    pub level: String,
    pub count: u32,
    pub spend: f64,
}

impl Default for BbbeeMetrics {
    fn default() -> Self {
        Self {
            level_1_spend: 0.0,
            level_1_percent: 0.0,
            level_2_spend: 0.0,
            level_2_percent: 0.0,
            level_3_4_spend: 0.0,
            level_3_4_percent: 0.0,
            level_5_8_spend: 0.0,
            level_5_8_percent: 0.0,
            non_compliant_spend: 0.0,
            non_compliant_percent: 0.0,
            total_bbbee_spend: 0.0,
            bbbee_target: 80.0,
            bbbee_actual: 0.0,
            suppliers_by_level: Vec::new(),
        }
    }
}

/// Provincial distribution data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProvinceDistribution {
    pub province: String,
    pub spend: f64,
    pub percentage: f64,
    pub supplier_count: u32,
}

/// Main spend analytics data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpendAnalytics {
    // Summary KPIs
    pub total_spend: f64,
    pub budget_allocated: f64,
    pub budget_variance: f64,
    pub active_contracts: u32,
    pub active_suppliers: u32,
    pub pending_payments: f64,
    pub savings_achieved: f64,
    pub savings_percentage: f64,

    // Trends
    pub trends: TrendData,

    // Breakdowns
    pub category_breakdown: Vec<CategoryBreakdown>,
    pub province_distribution: Vec<ProvinceDistribution>,

    // Supplier metrics
    pub top_suppliers: Vec<SupplierMetrics>,

    // B-BBEE
    pub bbbee_metrics: BbbeeMetrics,
}

impl Default for SpendAnalytics {
    fn default() -> Self {
        Self {
            total_spend: 0.0,
            budget_allocated: 0.0,
            budget_variance: 0.0,
            active_contracts: 0,
            active_suppliers: 0,
            pending_payments: 0.0,
            savings_achieved: 0.0,
            savings_percentage: 0.0,
            trends: TrendData::default(),
            category_breakdown: Vec::new(),
            province_distribution: Vec::new(),
            top_suppliers: Vec::new(),
            bbbee_metrics: BbbeeMetrics::default(),
        }
    }
}

/// Analytics filter criteria
#[derive(Clone, Debug, Default)]
pub struct AnalyticsFilter {
    pub time_period: TimePeriod,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub category: Option<String>,
    pub province: Option<String>,
    pub bbbee_level: Option<u8>,
    pub department: Option<String>,
}

/// Analytics summary KPIs
#[derive(Clone, Debug, Default)]
pub struct AnalyticsKpis {
    pub ytd_spend: f64,
    pub budget_utilization: f64,
    pub cost_savings: f64,
    pub bbbee_compliance: f64,
    pub avg_procurement_cycle: u32,
    pub supplier_performance: f64,
    pub pending_approvals: u32,
    pub compliance_score: f64,
}
