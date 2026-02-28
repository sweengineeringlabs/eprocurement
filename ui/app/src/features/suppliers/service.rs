//! Suppliers service - API calls

use super::store::{SuppliersStore, load_mock_data, select_supplier, clear_selection};
use super::types::{Supplier, SupplierFilter, SupplierStatus, BbbeeLevel, RiskRating};

/// Load suppliers list
pub async fn load_suppliers(store: &SuppliersStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API:
    // let response = api::get("/api/suppliers", &store.filter.get()).await;
    // For now, load mock data
    load_mock_data(store);

    store.loading.set(false);
}

/// Load suppliers with filter
pub async fn load_suppliers_filtered(store: &SuppliersStore, filter: SupplierFilter) {
    store.filter.set(filter);
    load_suppliers(store).await;
}

/// Get single supplier by ID
pub async fn get_supplier(store: &SuppliersStore, supplier_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::get(&format!("/api/suppliers/{}", supplier_id)).await;
    // For now, find in mock data
    select_supplier(store, supplier_id);

    store.loading.set(false);
}

/// Create new supplier
pub async fn create_supplier(store: &SuppliersStore, supplier: Supplier) -> Result<Supplier, String> {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::post("/api/suppliers", &supplier).await;

    // Generate a new ID for mock
    let mut new_supplier = supplier;
    let count = store.suppliers.get().len() + 1;
    new_supplier.id = format!("SUP-{:03}", count);
    new_supplier.status = SupplierStatus::Pending;
    new_supplier.registered_at = "2025-02-15".to_string();

    // Add to list
    let mut suppliers = store.suppliers.get();
    suppliers.push(new_supplier.clone());
    store.suppliers.set(suppliers);

    store.loading.set(false);
    Ok(new_supplier)
}

/// Update existing supplier
pub async fn update_supplier(store: &SuppliersStore, supplier: Supplier) -> Result<Supplier, String> {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::put(&format!("/api/suppliers/{}", supplier.id), &supplier).await;

    // Update in mock list
    let mut suppliers = store.suppliers.get();
    if let Some(pos) = suppliers.iter().position(|s| s.id == supplier.id) {
        suppliers[pos] = supplier.clone();
        store.suppliers.set(suppliers);
        store.selected.set(Some(supplier.clone()));
        store.loading.set(false);
        Ok(supplier)
    } else {
        store.loading.set(false);
        store.error.set(Some("Supplier not found".to_string()));
        Err("Supplier not found".to_string())
    }
}

/// Verify supplier (approve pending registration)
pub async fn verify_supplier(store: &SuppliersStore, supplier_id: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::post(&format!("/api/suppliers/{}/verify", supplier_id), &{}).await;

    let mut suppliers = store.suppliers.get();
    if let Some(pos) = suppliers.iter().position(|s| s.id == supplier_id) {
        suppliers[pos].status = SupplierStatus::Active;
        suppliers[pos].verified_at = Some("2025-02-15".to_string());
        store.suppliers.set(suppliers.clone());
        store.selected.set(Some(suppliers[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Supplier not found".to_string()));
        Err("Supplier not found".to_string())
    }
}

/// Suspend supplier
pub async fn suspend_supplier(store: &SuppliersStore, supplier_id: &str, reason: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::post(&format!("/api/suppliers/{}/suspend", supplier_id), &{ reason }).await;

    let mut suppliers = store.suppliers.get();
    if let Some(pos) = suppliers.iter().position(|s| s.id == supplier_id) {
        suppliers[pos].status = SupplierStatus::Suspended;
        store.suppliers.set(suppliers.clone());
        store.selected.set(Some(suppliers[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Supplier not found".to_string()));
        Err("Supplier not found".to_string())
    }
}

/// Reactivate suspended supplier
pub async fn reactivate_supplier(store: &SuppliersStore, supplier_id: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut suppliers = store.suppliers.get();
    if let Some(pos) = suppliers.iter().position(|s| s.id == supplier_id) {
        if suppliers[pos].status != SupplierStatus::Suspended {
            store.loading.set(false);
            store.error.set(Some("Supplier is not suspended".to_string()));
            return Err("Supplier is not suspended".to_string());
        }
        suppliers[pos].status = SupplierStatus::Active;
        store.suppliers.set(suppliers.clone());
        store.selected.set(Some(suppliers[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Supplier not found".to_string()));
        Err("Supplier not found".to_string())
    }
}

/// Blacklist supplier
pub async fn blacklist_supplier(store: &SuppliersStore, supplier_id: &str, reason: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut suppliers = store.suppliers.get();
    if let Some(pos) = suppliers.iter().position(|s| s.id == supplier_id) {
        suppliers[pos].status = SupplierStatus::Blacklisted;
        store.suppliers.set(suppliers.clone());
        store.selected.set(Some(suppliers[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Supplier not found".to_string()));
        Err("Supplier not found".to_string())
    }
}

/// Update supplier performance score
pub async fn update_performance(
    store: &SuppliersStore,
    supplier_id: &str,
    quality: f64,
    delivery: f64,
    price: f64,
    responsiveness: f64,
    compliance: f64,
) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut suppliers = store.suppliers.get();
    if let Some(pos) = suppliers.iter().position(|s| s.id == supplier_id) {
        let overall = (quality + delivery + price + responsiveness + compliance) / 5.0;
        suppliers[pos].performance_score.quality = quality;
        suppliers[pos].performance_score.delivery = delivery;
        suppliers[pos].performance_score.price = price;
        suppliers[pos].performance_score.responsiveness = responsiveness;
        suppliers[pos].performance_score.compliance = compliance;
        suppliers[pos].performance_score.overall = overall;
        store.suppliers.set(suppliers.clone());
        store.selected.set(Some(suppliers[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Supplier not found".to_string()));
        Err("Supplier not found".to_string())
    }
}

/// Update supplier risk rating
pub async fn update_risk_rating(
    store: &SuppliersStore,
    supplier_id: &str,
    risk_rating: RiskRating,
    risk_score: f64,
) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut suppliers = store.suppliers.get();
    if let Some(pos) = suppliers.iter().position(|s| s.id == supplier_id) {
        suppliers[pos].risk_rating = risk_rating;
        suppliers[pos].risk_score = risk_score;
        store.suppliers.set(suppliers.clone());
        store.selected.set(Some(suppliers[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Supplier not found".to_string()));
        Err("Supplier not found".to_string())
    }
}

/// Verify supplier against CSD (Central Supplier Database)
pub async fn verify_csd(store: &SuppliersStore, supplier_id: &str) -> Result<bool, String> {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the CSD API:
    // let response = api::get(&format!("/api/csd/verify/{}", supplier_id)).await;

    let mut suppliers = store.suppliers.get();
    if let Some(pos) = suppliers.iter().position(|s| s.id == supplier_id) {
        // Mock: assume verification passes if CSD number exists
        let verified = suppliers[pos].csd_number.is_some();
        suppliers[pos].csd_verified = verified;
        store.suppliers.set(suppliers.clone());
        store.selected.set(Some(suppliers[pos].clone()));
        store.loading.set(false);
        Ok(verified)
    } else {
        store.loading.set(false);
        store.error.set(Some("Supplier not found".to_string()));
        Err("Supplier not found".to_string())
    }
}

/// Search suppliers by query
pub async fn search_suppliers(store: &SuppliersStore, query: &str) {
    let filter = SupplierFilter {
        search_query: Some(query.to_string()),
        ..Default::default()
    };
    load_suppliers_filtered(store, filter).await;
}

/// Get suppliers by risk rating
pub async fn get_high_risk_suppliers(store: &SuppliersStore) {
    let filter = SupplierFilter {
        risk_rating: Some(RiskRating::High),
        ..Default::default()
    };
    load_suppliers_filtered(store, filter).await;
}

/// Get suppliers with expiring B-BBEE certificates
pub async fn get_expiring_certificates(store: &SuppliersStore) {
    // In production, this would filter by expiry date
    // For now, load all and let UI filter
    load_suppliers(store).await;
}
