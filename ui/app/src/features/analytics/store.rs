//! Analytics store

use components::prelude::*;
use super::types::{
    SpendAnalytics, AnalyticsFilter, AnalyticsKpis, TimePeriod,
    TrendData, TrendDataPoint, CategoryBreakdown, SupplierMetrics,
    BbbeeMetrics, BbbeeLevelCount, ProvinceDistribution,
};

/// Analytics state store
#[derive(Clone)]
pub struct AnalyticsStore {
    pub analytics: Signal<SpendAnalytics>,
    pub filter: Signal<AnalyticsFilter>,
    pub kpis: Signal<AnalyticsKpis>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
    pub selected_view: Signal<String>,
}

impl AnalyticsStore {
    pub fn new() -> Self {
        Self {
            analytics: signal(SpendAnalytics::default()),
            filter: signal(AnalyticsFilter::default()),
            kpis: signal(AnalyticsKpis::default()),
            loading: signal(false),
            error: signal(None),
            selected_view: signal("overview".to_string()),
        }
    }
}

/// Load mock analytics data for demo
pub fn load_mock_data(store: &AnalyticsStore) {
    // Monthly spend data for FY 2025/26 (April to current)
    let monthly_spend = vec![
        TrendDataPoint { period: "Apr".to_string(), value: 68_500_000.0, budget: Some(75_000_000.0) },
        TrendDataPoint { period: "May".to_string(), value: 72_300_000.0, budget: Some(75_000_000.0) },
        TrendDataPoint { period: "Jun".to_string(), value: 85_200_000.0, budget: Some(80_000_000.0) },
        TrendDataPoint { period: "Jul".to_string(), value: 78_900_000.0, budget: Some(80_000_000.0) },
        TrendDataPoint { period: "Aug".to_string(), value: 81_400_000.0, budget: Some(85_000_000.0) },
        TrendDataPoint { period: "Sep".to_string(), value: 92_100_000.0, budget: Some(90_000_000.0) },
        TrendDataPoint { period: "Oct".to_string(), value: 88_700_000.0, budget: Some(85_000_000.0) },
        TrendDataPoint { period: "Nov".to_string(), value: 95_300_000.0, budget: Some(90_000_000.0) },
        TrendDataPoint { period: "Dec".to_string(), value: 76_200_000.0, budget: Some(80_000_000.0) },
        TrendDataPoint { period: "Jan".to_string(), value: 108_400_000.0, budget: Some(100_000_000.0) },
    ];

    // Cumulative spend
    let mut cumulative = 0.0;
    let cumulative_spend: Vec<TrendDataPoint> = monthly_spend.iter().map(|m| {
        cumulative += m.value;
        TrendDataPoint {
            period: m.period.clone(),
            value: cumulative,
            budget: None,
        }
    }).collect();

    // Year-over-year comparison
    let year_over_year = vec![
        TrendDataPoint { period: "FY 23/24".to_string(), value: 890_000_000.0, budget: None },
        TrendDataPoint { period: "FY 24/25".to_string(), value: 945_000_000.0, budget: None },
        TrendDataPoint { period: "FY 25/26 (YTD)".to_string(), value: 847_000_000.0, budget: None },
    ];

    let trends = TrendData {
        monthly_spend,
        cumulative_spend,
        year_over_year,
    };

    // Category breakdown
    let category_breakdown = vec![
        CategoryBreakdown {
            code: "IT".to_string(),
            name: "Information Technology".to_string(),
            spend: 250_000_000.0,
            budget: 280_000_000.0,
            variance: 30_000_000.0,
            percentage: 29.5,
            contract_count: 45,
            supplier_count: 28,
            color: "var(--blue)".to_string(),
        },
        CategoryBreakdown {
            code: "PRO".to_string(),
            name: "Professional Services".to_string(),
            spend: 200_000_000.0,
            budget: 220_000_000.0,
            variance: 20_000_000.0,
            percentage: 23.6,
            contract_count: 38,
            supplier_count: 42,
            color: "var(--purple)".to_string(),
        },
        CategoryBreakdown {
            code: "FAC".to_string(),
            name: "Facilities Management".to_string(),
            spend: 180_000_000.0,
            budget: 175_000_000.0,
            variance: -5_000_000.0,
            percentage: 21.3,
            contract_count: 22,
            supplier_count: 18,
            color: "var(--green)".to_string(),
        },
        CategoryBreakdown {
            code: "SEC".to_string(),
            name: "Security Services".to_string(),
            spend: 120_000_000.0,
            budget: 130_000_000.0,
            variance: 10_000_000.0,
            percentage: 14.2,
            contract_count: 15,
            supplier_count: 12,
            color: "var(--orange)".to_string(),
        },
        CategoryBreakdown {
            code: "FLT".to_string(),
            name: "Fleet Management".to_string(),
            spend: 97_000_000.0,
            budget: 95_000_000.0,
            variance: -2_000_000.0,
            percentage: 11.4,
            contract_count: 8,
            supplier_count: 6,
            color: "var(--cyan)".to_string(),
        },
    ];

    // Province distribution
    let province_distribution = vec![
        ProvinceDistribution { province: "Gauteng".to_string(), spend: 380_000_000.0, percentage: 44.9, supplier_count: 542 },
        ProvinceDistribution { province: "Western Cape".to_string(), spend: 165_000_000.0, percentage: 19.5, supplier_count: 234 },
        ProvinceDistribution { province: "KwaZulu-Natal".to_string(), spend: 120_000_000.0, percentage: 14.2, supplier_count: 187 },
        ProvinceDistribution { province: "Eastern Cape".to_string(), spend: 68_000_000.0, percentage: 8.0, supplier_count: 98 },
        ProvinceDistribution { province: "Mpumalanga".to_string(), spend: 42_000_000.0, percentage: 5.0, supplier_count: 67 },
        ProvinceDistribution { province: "Free State".to_string(), spend: 28_000_000.0, percentage: 3.3, supplier_count: 54 },
        ProvinceDistribution { province: "Limpopo".to_string(), spend: 22_000_000.0, percentage: 2.6, supplier_count: 48 },
        ProvinceDistribution { province: "North West".to_string(), spend: 15_000_000.0, percentage: 1.8, supplier_count: 35 },
        ProvinceDistribution { province: "Northern Cape".to_string(), spend: 7_000_000.0, percentage: 0.8, supplier_count: 19 },
    ];

    // Top suppliers by spend
    let top_suppliers = vec![
        SupplierMetrics {
            supplier_id: "SUP-001".to_string(),
            supplier_name: "TechSolutions SA (Pty) Ltd".to_string(),
            total_spend: 45_000_000.0,
            contract_count: 3,
            average_delivery_time: 4.2,
            quality_score: 95.0,
            compliance_score: 96.0,
            overall_rating: 92.5,
            bbbee_level: 1,
            province: "Gauteng".to_string(),
        },
        SupplierMetrics {
            supplier_id: "SUP-004".to_string(),
            supplier_name: "SecureGuard Holdings (Pty) Ltd".to_string(),
            total_spend: 35_000_000.0,
            contract_count: 5,
            average_delivery_time: 2.8,
            quality_score: 90.0,
            compliance_score: 91.0,
            overall_rating: 88.5,
            bbbee_level: 1,
            province: "KwaZulu-Natal".to_string(),
        },
        SupplierMetrics {
            supplier_id: "SUP-002".to_string(),
            supplier_name: "Ubuntu Construction Group".to_string(),
            total_spend: 28_500_000.0,
            contract_count: 2,
            average_delivery_time: 12.5,
            quality_score: 88.0,
            compliance_score: 92.0,
            overall_rating: 85.0,
            bbbee_level: 2,
            province: "Gauteng".to_string(),
        },
        SupplierMetrics {
            supplier_id: "SUP-012".to_string(),
            supplier_name: "North West Mining Equipment".to_string(),
            total_spend: 22_000_000.0,
            contract_count: 2,
            average_delivery_time: 8.3,
            quality_score: 83.0,
            compliance_score: 83.0,
            overall_rating: 81.0,
            bbbee_level: 3,
            province: "North West".to_string(),
        },
        SupplierMetrics {
            supplier_id: "SUP-007".to_string(),
            supplier_name: "Mpumalanga Transport Logistics".to_string(),
            total_spend: 18_000_000.0,
            contract_count: 3,
            average_delivery_time: 3.5,
            quality_score: 85.0,
            compliance_score: 89.0,
            overall_rating: 86.5,
            bbbee_level: 2,
            province: "Mpumalanga".to_string(),
        },
        SupplierMetrics {
            supplier_id: "SUP-005".to_string(),
            supplier_name: "GreenTech Environmental Solutions".to_string(),
            total_spend: 15_500_000.0,
            contract_count: 2,
            average_delivery_time: 5.2,
            quality_score: 93.0,
            compliance_score: 94.0,
            overall_rating: 91.0,
            bbbee_level: 1,
            province: "Gauteng".to_string(),
        },
        SupplierMetrics {
            supplier_id: "SUP-003".to_string(),
            supplier_name: "Cape Catering Services CC".to_string(),
            total_spend: 8_200_000.0,
            contract_count: 4,
            average_delivery_time: 1.5,
            quality_score: 96.0,
            compliance_score: 96.0,
            overall_rating: 94.0,
            bbbee_level: 1,
            province: "Western Cape".to_string(),
        },
        SupplierMetrics {
            supplier_id: "SUP-008".to_string(),
            supplier_name: "Eastern Cape Medical Supplies".to_string(),
            total_spend: 6_800_000.0,
            contract_count: 1,
            average_delivery_time: 9.8,
            quality_score: 78.0,
            compliance_score: 72.0,
            overall_rating: 72.0,
            bbbee_level: 4,
            province: "Eastern Cape".to_string(),
        },
    ];

    // B-BBEE metrics
    let total_spend = 847_000_000.0;
    let bbbee_metrics = BbbeeMetrics {
        level_1_spend: 356_000_000.0,
        level_1_percent: 42.0,
        level_2_spend: 237_000_000.0,
        level_2_percent: 28.0,
        level_3_4_spend: 152_500_000.0,
        level_3_4_percent: 18.0,
        level_5_8_spend: 50_800_000.0,
        level_5_8_percent: 6.0,
        non_compliant_spend: 50_700_000.0,
        non_compliant_percent: 6.0,
        total_bbbee_spend: 745_500_000.0,
        bbbee_target: 80.0,
        bbbee_actual: 78.4,
        suppliers_by_level: vec![
            BbbeeLevelCount { level: "Level 1".to_string(), count: 485, spend: 356_000_000.0 },
            BbbeeLevelCount { level: "Level 2".to_string(), count: 312, spend: 237_000_000.0 },
            BbbeeLevelCount { level: "Level 3".to_string(), count: 187, spend: 95_000_000.0 },
            BbbeeLevelCount { level: "Level 4".to_string(), count: 124, spend: 57_500_000.0 },
            BbbeeLevelCount { level: "Level 5-8".to_string(), count: 98, spend: 50_800_000.0 },
            BbbeeLevelCount { level: "Non-Compliant".to_string(), count: 78, spend: 50_700_000.0 },
        ],
    };

    // Complete analytics data
    let analytics = SpendAnalytics {
        total_spend,
        budget_allocated: 1_000_000_000.0,
        budget_variance: 153_000_000.0,
        active_contracts: 456,
        active_suppliers: 1284,
        pending_payments: 45_600_000.0,
        savings_achieved: 32_500_000.0,
        savings_percentage: 3.8,
        trends,
        category_breakdown,
        province_distribution,
        top_suppliers,
        bbbee_metrics,
    };

    // KPIs
    let kpis = AnalyticsKpis {
        ytd_spend: 847_000_000.0,
        budget_utilization: 84.7,
        cost_savings: 32_500_000.0,
        bbbee_compliance: 78.4,
        avg_procurement_cycle: 42,
        supplier_performance: 86.5,
        pending_approvals: 34,
        compliance_score: 94.2,
    };

    store.analytics.set(analytics);
    store.kpis.set(kpis);
}

/// Update time period filter
pub fn set_time_period(store: &AnalyticsStore, period: TimePeriod) {
    let mut filter = store.filter.get();
    filter.time_period = period;
    store.filter.set(filter);
    // In production, this would trigger a data reload
}

/// Update category filter
pub fn set_category_filter(store: &AnalyticsStore, category: Option<String>) {
    let mut filter = store.filter.get();
    filter.category = category;
    store.filter.set(filter);
}

/// Update province filter
pub fn set_province_filter(store: &AnalyticsStore, province: Option<String>) {
    let mut filter = store.filter.get();
    filter.province = province;
    store.filter.set(filter);
}

/// Set the selected view/tab
pub fn set_view(store: &AnalyticsStore, view: String) {
    store.selected_view.set(view);
}

/// Clear all filters
pub fn clear_filters(store: &AnalyticsStore) {
    store.filter.set(AnalyticsFilter::default());
}
