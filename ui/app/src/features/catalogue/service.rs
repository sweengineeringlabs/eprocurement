//! Catalogue service - API calls

use super::store::{CatalogueStore, load_mock_data, select_item, clear_selection};
use super::types::{CatalogueItem, CatalogueCategory, CatalogueFilter, CatalogueItemStatus};

/// Load catalogue items
pub async fn load_catalogue(store: &CatalogueStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API:
    // let response = api::get("/api/catalogue/items", &store.filter.get()).await;
    // For now, load mock data
    load_mock_data(store);

    store.loading.set(false);
}

/// Load catalogue items with filter
pub async fn load_catalogue_filtered(store: &CatalogueStore, filter: CatalogueFilter) {
    store.filter.set(filter);
    load_catalogue(store).await;
}

/// Get single catalogue item by ID
pub async fn get_item(store: &CatalogueStore, item_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::get(&format!("/api/catalogue/items/{}", item_id)).await;
    // For now, find in mock data
    select_item(store, item_id);

    store.loading.set(false);
}

/// Load categories
pub async fn load_categories(store: &CatalogueStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::get("/api/catalogue/categories").await;
    // Categories are loaded with mock data

    store.loading.set(false);
}

/// Create new catalogue item
pub async fn create_item(store: &CatalogueStore, item: CatalogueItem) -> Result<CatalogueItem, String> {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::post("/api/catalogue/items", &item).await;

    // Generate a new ID for mock
    let mut new_item = item;
    let count = store.items.get().len() + 1;
    new_item.id = format!("ITEM-{:03}", count);
    new_item.status = CatalogueItemStatus::PendingApproval;
    new_item.created_at = "2025-02-15".to_string();
    new_item.updated_at = "2025-02-15".to_string();

    // Add to list
    let mut items = store.items.get();
    items.push(new_item.clone());
    store.items.set(items);

    store.loading.set(false);
    Ok(new_item)
}

/// Update existing catalogue item
pub async fn update_item(store: &CatalogueStore, item: CatalogueItem) -> Result<CatalogueItem, String> {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::put(&format!("/api/catalogue/items/{}", item.id), &item).await;

    // Update in mock list
    let mut items = store.items.get();
    if let Some(pos) = items.iter().position(|i| i.id == item.id) {
        let mut updated_item = item.clone();
        updated_item.updated_at = "2025-02-15".to_string();
        items[pos] = updated_item.clone();
        store.items.set(items);
        store.selected.set(Some(updated_item.clone()));
        store.loading.set(false);
        Ok(updated_item)
    } else {
        store.loading.set(false);
        store.error.set(Some("Item not found".to_string()));
        Err("Item not found".to_string())
    }
}

/// Delete catalogue item
pub async fn delete_item(store: &CatalogueStore, item_id: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::delete(&format!("/api/catalogue/items/{}", item_id)).await;

    let mut items = store.items.get();
    if let Some(pos) = items.iter().position(|i| i.id == item_id) {
        items.remove(pos);
        store.items.set(items);
        clear_selection(store);
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Item not found".to_string()));
        Err("Item not found".to_string())
    }
}

/// Activate catalogue item
pub async fn activate_item(store: &CatalogueStore, item_id: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::post(&format!("/api/catalogue/items/{}/activate", item_id), &{}).await;

    let mut items = store.items.get();
    if let Some(pos) = items.iter().position(|i| i.id == item_id) {
        items[pos].status = CatalogueItemStatus::Active;
        items[pos].updated_at = "2025-02-15".to_string();
        store.items.set(items.clone());
        store.selected.set(Some(items[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Item not found".to_string()));
        Err("Item not found".to_string())
    }
}

/// Deactivate catalogue item
pub async fn deactivate_item(store: &CatalogueStore, item_id: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut items = store.items.get();
    if let Some(pos) = items.iter().position(|i| i.id == item_id) {
        items[pos].status = CatalogueItemStatus::Inactive;
        items[pos].updated_at = "2025-02-15".to_string();
        store.items.set(items.clone());
        store.selected.set(Some(items[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Item not found".to_string()));
        Err("Item not found".to_string())
    }
}

/// Discontinue catalogue item
pub async fn discontinue_item(store: &CatalogueStore, item_id: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut items = store.items.get();
    if let Some(pos) = items.iter().position(|i| i.id == item_id) {
        items[pos].status = CatalogueItemStatus::Discontinued;
        items[pos].updated_at = "2025-02-15".to_string();
        store.items.set(items.clone());
        store.selected.set(Some(items[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Item not found".to_string()));
        Err("Item not found".to_string())
    }
}

/// Approve pending catalogue item
pub async fn approve_item(store: &CatalogueStore, item_id: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut items = store.items.get();
    if let Some(pos) = items.iter().position(|i| i.id == item_id) {
        if items[pos].status != CatalogueItemStatus::PendingApproval {
            store.loading.set(false);
            store.error.set(Some("Item is not pending approval".to_string()));
            return Err("Item is not pending approval".to_string());
        }
        items[pos].status = CatalogueItemStatus::Active;
        items[pos].updated_at = "2025-02-15".to_string();
        store.items.set(items.clone());
        store.selected.set(Some(items[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Item not found".to_string()));
        Err("Item not found".to_string())
    }
}

/// Update item stock status
pub async fn update_stock(store: &CatalogueStore, item_id: &str, in_stock: bool, quantity: Option<u32>) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut items = store.items.get();
    if let Some(pos) = items.iter().position(|i| i.id == item_id) {
        items[pos].in_stock = in_stock;
        items[pos].stock_quantity = quantity;
        items[pos].updated_at = "2025-02-15".to_string();
        store.items.set(items.clone());
        store.selected.set(Some(items[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Item not found".to_string()));
        Err("Item not found".to_string())
    }
}

/// Update item price
pub async fn update_price(store: &CatalogueStore, item_id: &str, new_price: f64) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut items = store.items.get();
    if let Some(pos) = items.iter().position(|i| i.id == item_id) {
        items[pos].unit_price = new_price;
        items[pos].updated_at = "2025-02-15".to_string();
        store.items.set(items.clone());
        store.selected.set(Some(items[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Item not found".to_string()));
        Err("Item not found".to_string())
    }
}

/// Search catalogue items
pub async fn search_catalogue(store: &CatalogueStore, query: &str) {
    let filter = CatalogueFilter {
        search_query: Some(query.to_string()),
        ..Default::default()
    };
    load_catalogue_filtered(store, filter).await;
}

/// Get items by category
pub async fn get_items_by_category(store: &CatalogueStore, category_id: &str) {
    let filter = CatalogueFilter {
        category_id: Some(category_id.to_string()),
        ..Default::default()
    };
    load_catalogue_filtered(store, filter).await;
}

/// Get featured items
pub async fn get_featured_items(store: &CatalogueStore) {
    let filter = CatalogueFilter {
        featured_only: true,
        ..Default::default()
    };
    load_catalogue_filtered(store, filter).await;
}

/// Get out of stock items
pub async fn get_out_of_stock_items(store: &CatalogueStore) {
    let filter = CatalogueFilter {
        in_stock_only: false,
        ..Default::default()
    };
    load_catalogue_filtered(store, filter).await;
}

/// Create new category
pub async fn create_category(store: &CatalogueStore, category: CatalogueCategory) -> Result<CatalogueCategory, String> {
    store.loading.set(true);
    store.error.set(None);

    // In production:
    // let response = api::post("/api/catalogue/categories", &category).await;

    let mut new_category = category;
    let count = store.categories.get().len() + 1;
    new_category.id = format!("CAT-{:03}", count);
    new_category.item_count = 0;

    let mut categories = store.categories.get();
    categories.push(new_category.clone());
    store.categories.set(categories);

    store.loading.set(false);
    Ok(new_category)
}

/// Update category
pub async fn update_category(store: &CatalogueStore, category: CatalogueCategory) -> Result<CatalogueCategory, String> {
    store.loading.set(true);
    store.error.set(None);

    let mut categories = store.categories.get();
    if let Some(pos) = categories.iter().position(|c| c.id == category.id) {
        categories[pos] = category.clone();
        store.categories.set(categories);
        store.loading.set(false);
        Ok(category)
    } else {
        store.loading.set(false);
        store.error.set(Some("Category not found".to_string()));
        Err("Category not found".to_string())
    }
}

/// Delete category
pub async fn delete_category(store: &CatalogueStore, category_id: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    // Check if category has items
    let has_items = store.items.get().iter().any(|i| i.category_id == category_id);
    if has_items {
        store.loading.set(false);
        store.error.set(Some("Cannot delete category with items".to_string()));
        return Err("Cannot delete category with items".to_string());
    }

    let mut categories = store.categories.get();
    if let Some(pos) = categories.iter().position(|c| c.id == category_id) {
        categories.remove(pos);
        store.categories.set(categories);
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Category not found".to_string()));
        Err("Category not found".to_string())
    }
}

/// Import items from CSV (placeholder)
pub async fn import_items_csv(_store: &CatalogueStore, _csv_data: &str) -> Result<u32, String> {
    // In production, this would parse CSV and create items
    Ok(0)
}

/// Export items to CSV (placeholder)
pub async fn export_items_csv(_store: &CatalogueStore) -> Result<String, String> {
    // In production, this would generate CSV from items
    Ok(String::new())
}
