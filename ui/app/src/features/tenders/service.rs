//! Tenders service - API calls

use super::store::{TendersStore, load_mock_data, select_tender, clear_selection};
use super::types::{Tender, TenderFilter, TenderStatus};

/// Load tenders list
pub async fn load_tenders(store: &TendersStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API:
    // let response = api::get("/api/tenders", &store.filter.get()).await;
    // For now, load mock data
    load_mock_data(store);

    store.loading.set(false);
}

/// Load tenders with filter
pub async fn load_tenders_filtered(store: &TendersStore, filter: TenderFilter) {
    store.filter.set(filter);
    load_tenders(store).await;
}

/// Get single tender by ID
pub async fn get_tender(store: &TendersStore, tender_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::get(&format!("/api/tenders/{}", tender_id)).await;
    // For now, find in mock data
    select_tender(store, tender_id);

    store.loading.set(false);
}

/// Create new tender
pub async fn create_tender(store: &TendersStore, tender: Tender) -> Result<Tender, String> {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::post("/api/tenders", &tender).await;

    // Generate a new ID for mock
    let mut new_tender = tender;
    let count = store.tenders.get().len() + 1;
    new_tender.id = format!("TND-2025-{:04}", 100 + count);
    new_tender.reference_number = format!("{}-2025-{:04}",
        new_tender.tender_type.label(),
        100 + count
    );
    new_tender.status = TenderStatus::Draft;
    new_tender.created_at = "2025-02-15".to_string();
    new_tender.last_modified_at = "2025-02-15".to_string();

    // Add to list
    let mut tenders = store.tenders.get();
    tenders.push(new_tender.clone());
    store.tenders.set(tenders);

    store.loading.set(false);
    Ok(new_tender)
}

/// Update existing tender
pub async fn update_tender(store: &TendersStore, tender: Tender) -> Result<Tender, String> {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::put(&format!("/api/tenders/{}", tender.id), &tender).await;

    // Update in mock list
    let mut tenders = store.tenders.get();
    if let Some(pos) = tenders.iter().position(|t| t.id == tender.id) {
        tenders[pos] = tender.clone();
        store.tenders.set(tenders);
        store.selected.set(Some(tender.clone()));
        store.loading.set(false);
        Ok(tender)
    } else {
        store.loading.set(false);
        store.error.set(Some("Tender not found".to_string()));
        Err("Tender not found".to_string())
    }
}

/// Submit tender for approval
pub async fn submit_for_approval(store: &TendersStore, tender_id: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::post(&format!("/api/tenders/{}/submit", tender_id), &{}).await;

    // Update status in mock
    let mut tenders = store.tenders.get();
    if let Some(pos) = tenders.iter().position(|t| t.id == tender_id) {
        tenders[pos].status = TenderStatus::PendingApproval;
        tenders[pos].last_modified_at = "2025-02-15".to_string();
        store.tenders.set(tenders.clone());
        store.selected.set(Some(tenders[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Tender not found".to_string()));
        Err("Tender not found".to_string())
    }
}

/// Approve tender
pub async fn approve_tender(store: &TendersStore, tender_id: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut tenders = store.tenders.get();
    if let Some(pos) = tenders.iter().position(|t| t.id == tender_id) {
        tenders[pos].status = TenderStatus::Approved;
        tenders[pos].last_modified_at = "2025-02-15".to_string();
        store.tenders.set(tenders.clone());
        store.selected.set(Some(tenders[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Tender not found".to_string()));
        Err("Tender not found".to_string())
    }
}

/// Publish tender to e-Tender portal
pub async fn publish_tender(
    store: &TendersStore,
    tender_id: &str,
    publish_date: &str,
    closing_date: &str,
    briefing_date: Option<&str>,
) -> Result<String, String> {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::post(&format!("/api/tenders/{}/publish", tender_id), &PublishRequest {
    //     publish_date,
    //     closing_date,
    //     briefing_date,
    // }).await;

    // Update status and dates in mock
    let mut tenders = store.tenders.get();
    if let Some(pos) = tenders.iter().position(|t| t.id == tender_id) {
        tenders[pos].status = TenderStatus::Published;
        tenders[pos].publish_date = Some(publish_date.to_string());
        tenders[pos].closing_date = Some(closing_date.to_string());
        tenders[pos].briefing_date = briefing_date.map(|s| s.to_string());

        // Generate portal reference
        let portal_ref = format!("eTender-{}", tender_id.replace("TND-", ""));
        tenders[pos].portal_reference = Some(portal_ref.clone());
        tenders[pos].portal_url = Some(format!("https://etenders.gov.za/tender/{}", portal_ref));
        tenders[pos].last_modified_at = "2025-02-15".to_string();

        store.tenders.set(tenders.clone());
        store.selected.set(Some(tenders[pos].clone()));
        store.loading.set(false);
        Ok(portal_ref)
    } else {
        store.loading.set(false);
        store.error.set(Some("Tender not found".to_string()));
        Err("Tender not found".to_string())
    }
}

/// Cancel tender
pub async fn cancel_tender(store: &TendersStore, tender_id: &str, reason: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut tenders = store.tenders.get();
    if let Some(pos) = tenders.iter().position(|t| t.id == tender_id) {
        tenders[pos].status = TenderStatus::Cancelled;
        tenders[pos].last_modified_at = "2025-02-15".to_string();
        store.tenders.set(tenders.clone());
        store.selected.set(Some(tenders[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Tender not found".to_string()));
        Err("Tender not found".to_string())
    }
}

/// Delete tender (draft only)
pub async fn delete_tender(store: &TendersStore, tender_id: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let tenders = store.tenders.get();
    if let Some(tender) = tenders.iter().find(|t| t.id == tender_id) {
        if tender.status != TenderStatus::Draft {
            store.loading.set(false);
            store.error.set(Some("Only draft tenders can be deleted".to_string()));
            return Err("Only draft tenders can be deleted".to_string());
        }
    }

    let new_tenders: Vec<_> = tenders.into_iter()
        .filter(|t| t.id != tender_id)
        .collect();
    store.tenders.set(new_tenders);
    clear_selection(store);

    store.loading.set(false);
    Ok(())
}
