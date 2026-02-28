//! Mobile service - API calls

use super::store::{MobileStore, load_mock_mobile_data, get_mock_user};
use super::types::{
    AppFeature, MobileUser, MobileAppConfig, MobileAppStats,
    NotificationPreference, DeviceType,
};

/// Load all mobile data
pub async fn load_mobile_data(store: &MobileStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    // For now, load mock data
    load_mock_mobile_data(store);

    store.loading.set(false);
}

/// Load mobile app configuration
pub async fn load_config(store: &MobileStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    // Mock data is loaded via load_mobile_data
    if store.config.get().id.is_empty() {
        load_mock_mobile_data(store);
    }

    store.loading.set(false);
}

/// Save mobile app configuration
pub async fn save_config(store: &MobileStore) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    let config = store.config_draft.get();

    // Validate
    if config.app_name.trim().is_empty() {
        store.saving.set(false);
        return Err("App name is required".to_string());
    }

    if config.support_email.trim().is_empty() {
        store.saving.set(false);
        return Err("Support email is required".to_string());
    }

    if config.session_timeout_minutes < 5 {
        store.saving.set(false);
        return Err("Session timeout must be at least 5 minutes".to_string());
    }

    if config.session_timeout_minutes > 480 {
        store.saving.set(false);
        return Err("Session timeout cannot exceed 8 hours".to_string());
    }

    // In production, this would call the API
    // For now, just update the config
    let mut updated_config = config.clone();
    updated_config.updated_at = chrono_now();
    store.config.set(updated_config);
    store.config_dirty.set(false);

    store.saving.set(false);
    Ok(())
}

/// Load mobile users
pub async fn load_users(store: &MobileStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API with filters
    if store.users.get().is_empty() {
        load_mock_mobile_data(store);
    }

    store.loading.set(false);
}

/// Load single user details
pub async fn load_user(store: &MobileStore, id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    if let Some(user) = get_mock_user(id) {
        store.selected_user.set(Some(user));
    } else {
        store.error.set(Some(format!("User {} not found", id)));
    }

    store.loading.set(false);
}

/// Load mobile app statistics
pub async fn load_stats(store: &MobileStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    store.stats.set(MobileAppStats::default());

    store.loading.set(false);
}

/// Update user notification preferences
pub async fn update_user_preferences(
    store: &MobileStore,
    user_id: &str,
    preferences: NotificationPreference,
) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    // Find user
    let mut users = store.users.get();
    let idx = users.iter().position(|u| u.id == user_id);

    if idx.is_none() {
        store.saving.set(false);
        return Err("User not found".to_string());
    }

    let idx = idx.unwrap();

    // Update preferences
    users[idx].notification_preferences = preferences;
    store.users.set(users.clone());

    // Update selected user if same
    if let Some(selected) = store.selected_user.get() {
        if selected.id == user_id {
            store.selected_user.set(Some(users[idx].clone()));
        }
    }

    store.saving.set(false);
    Ok(())
}

/// Enable or disable features for a user
pub async fn update_user_features(
    store: &MobileStore,
    user_id: &str,
    features: Vec<AppFeature>,
) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    // Find user
    let mut users = store.users.get();
    let idx = users.iter().position(|u| u.id == user_id);

    if idx.is_none() {
        store.saving.set(false);
        return Err("User not found".to_string());
    }

    let idx = idx.unwrap();

    // Update features
    users[idx].features_enabled = features;
    store.users.set(users.clone());

    // Update selected user if same
    if let Some(selected) = store.selected_user.get() {
        if selected.id == user_id {
            store.selected_user.set(Some(users[idx].clone()));
        }
    }

    store.saving.set(false);
    Ok(())
}

/// Deactivate a mobile user
pub async fn deactivate_user(store: &MobileStore, user_id: &str) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    // Find user
    let mut users = store.users.get();
    let idx = users.iter().position(|u| u.id == user_id);

    if idx.is_none() {
        store.saving.set(false);
        return Err("User not found".to_string());
    }

    let idx = idx.unwrap();

    // Deactivate
    users[idx].is_active = false;
    store.users.set(users.clone());

    // Update selected user if same
    if let Some(selected) = store.selected_user.get() {
        if selected.id == user_id {
            store.selected_user.set(Some(users[idx].clone()));
        }
    }

    store.saving.set(false);
    Ok(())
}

/// Reactivate a mobile user
pub async fn reactivate_user(store: &MobileStore, user_id: &str) -> Result<(), String> {
    store.saving.set(true);
    store.error.set(None);

    // Find user
    let mut users = store.users.get();
    let idx = users.iter().position(|u| u.id == user_id);

    if idx.is_none() {
        store.saving.set(false);
        return Err("User not found".to_string());
    }

    let idx = idx.unwrap();

    // Reactivate
    users[idx].is_active = true;
    store.users.set(users.clone());

    // Update selected user if same
    if let Some(selected) = store.selected_user.get() {
        if selected.id == user_id {
            store.selected_user.set(Some(users[idx].clone()));
        }
    }

    store.saving.set(false);
    Ok(())
}

/// Send push notification to user
pub async fn send_push_notification(
    _user_id: &str,
    _title: &str,
    _message: &str,
) -> Result<(), String> {
    // In production, this would call the push notification service
    Ok(())
}

/// Send push notification to all users
pub async fn send_broadcast_notification(
    _title: &str,
    _message: &str,
    _device_type: Option<DeviceType>,
) -> Result<u32, String> {
    // In production, this would call the push notification service
    // Returns the number of users notified
    Ok(100)
}

/// Generate app download QR code
pub async fn generate_download_qr(_platform: DeviceType) -> Result<String, String> {
    // In production, this would generate a QR code image URL
    Ok("data:image/png;base64,placeholder".to_string())
}

/// Export mobile users to CSV
pub async fn export_users_csv(store: &MobileStore) -> Result<String, String> {
    let users = store.get_filtered_users();

    let mut csv = String::from("ID,Supplier,Email,Phone,Device,Version,Active,Last Active\n");

    for user in users {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{}\n",
            user.id,
            user.supplier_name,
            user.email,
            user.phone,
            user.device_type.label(),
            user.app_version,
            if user.is_active { "Yes" } else { "No" },
            user.last_active_at,
        ));
    }

    Ok(csv)
}

/// Refresh mobile data
pub async fn refresh_mobile_data(store: &MobileStore) {
    load_mobile_data(store).await;
}

// Helper functions
fn chrono_now() -> String {
    // In production, use chrono crate
    "2025-02-27T10:00:00Z".to_string()
}
