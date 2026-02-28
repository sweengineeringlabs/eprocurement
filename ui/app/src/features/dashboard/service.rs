//! Dashboard service - API calls

use super::store::{DashboardStore, load_mock_data};

/// Load dashboard data
pub async fn load_dashboard(store: &DashboardStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    // For now, load mock data
    load_mock_data(store);

    store.loading.set(false);
}
