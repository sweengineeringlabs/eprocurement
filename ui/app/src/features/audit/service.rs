//! Audit service - API calls

use super::store::{AuditStore, load_mock_data};
use super::types::{AuditEntry, AuditFilter, AuditExportRequest, ExportFormat};

/// Load all audit entries
pub async fn load_audit_entries(store: &AuditStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API
    // For now, load mock data
    load_mock_data(store);

    store.loading.set(false);
}

/// Load audit entries with filter
pub async fn load_filtered_audit_entries(store: &AuditStore, filter: AuditFilter) {
    store.loading.set(true);
    store.error.set(None);

    // Set the filter
    store.filter.set(filter);

    // Ensure entries are loaded
    if store.entries.get().is_empty() {
        load_mock_data(store);
    }

    store.loading.set(false);
}

/// Load audit entry by ID
pub async fn load_audit_entry(store: &AuditStore, entry_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // Ensure entries are loaded
    if store.entries.get().is_empty() {
        load_mock_data(store);
    }

    // Select the entry
    store.select_entry(entry_id);

    store.loading.set(false);
}

/// Load audit entries for a specific entity
pub async fn load_entity_audit_trail(
    store: &AuditStore,
    entity_type: &str,
    entity_id: &str,
) {
    store.loading.set(true);
    store.error.set(None);

    // Ensure entries are loaded
    if store.entries.get().is_empty() {
        load_mock_data(store);
    }

    // Set filter for specific entity
    let mut filter = AuditFilter::default();
    filter.entity_type = Some(super::types::AuditEntityType::from_str(entity_type));
    filter.entity_id = Some(entity_id.to_string());
    store.filter.set(filter);

    store.loading.set(false);
}

/// Load audit entries for a specific user
pub async fn load_user_audit_trail(store: &AuditStore, user_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // Ensure entries are loaded
    if store.entries.get().is_empty() {
        load_mock_data(store);
    }

    // Set filter for specific user
    let mut filter = AuditFilter::default();
    filter.user_id = Some(user_id.to_string());
    store.filter.set(filter);

    store.loading.set(false);
}

/// Search audit entries
pub async fn search_audit_entries(store: &AuditStore, query: &str) {
    store.loading.set(true);
    store.error.set(None);

    // Ensure entries are loaded
    if store.entries.get().is_empty() {
        load_mock_data(store);
    }

    // Set search filter
    let mut filter = store.filter.get().clone();
    filter.search = if query.is_empty() { None } else { Some(query.to_string()) };
    store.filter.set(filter);

    store.loading.set(false);
}

/// Export audit entries
pub async fn export_audit_entries(
    store: &AuditStore,
    request: AuditExportRequest,
) -> Result<String, String> {
    store.loading.set(true);
    store.error.set(None);

    // Get filtered entries
    let entries = store.get_filtered_entries();

    if entries.is_empty() {
        store.loading.set(false);
        return Err("No entries to export".to_string());
    }

    // In production, this would call the API to generate the export
    // For now, return a mock download URL
    let format_ext = match request.format {
        ExportFormat::Csv => "csv",
        ExportFormat::Excel => "xlsx",
        ExportFormat::Pdf => "pdf",
        ExportFormat::Json => "json",
    };

    let download_url = format!(
        "/api/audit/export/audit_trail_{}.{}",
        "2025-02-27",
        format_ext
    );

    store.loading.set(false);
    Ok(download_url)
}

/// Get audit statistics
pub async fn load_audit_stats(store: &AuditStore) {
    store.loading.set(true);

    // Ensure entries are loaded
    if store.entries.get().is_empty() {
        load_mock_data(store);
    }

    // Calculate stats from loaded entries
    store.calculate_stats();

    store.loading.set(false);
}

/// Create audit entry (internal use)
pub async fn create_audit_entry(
    store: &AuditStore,
    entry: AuditEntry,
) -> Result<String, String> {
    // In production, this would POST to the API
    // For now, add to local store
    let mut entries = store.entries.get().clone();
    let entry_id = entry.id.clone();
    entries.insert(0, entry); // Add to beginning (most recent first)
    store.entries.set(entries);
    store.calculate_stats();

    Ok(entry_id)
}

/// Refresh audit data
pub async fn refresh_audit_data(store: &AuditStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, would refetch from API
    // For now, just recalculate stats
    store.calculate_stats();

    store.loading.set(false);
}

/// Format timestamp for display
pub fn format_audit_timestamp(timestamp: &str) -> String {
    // In production, would use proper date formatting
    // For now, simple formatting
    if timestamp.len() >= 16 {
        let date = &timestamp[0..10];
        let time = &timestamp[11..16];
        format!("{} {}", date, time)
    } else {
        timestamp.to_string()
    }
}

/// Get relative time description
pub fn get_relative_time(timestamp: &str) -> String {
    // Simplified relative time - in production would use proper date comparison
    if timestamp.starts_with("2025-02-27") {
        let hour: u32 = timestamp[11..13].parse().unwrap_or(0);
        let current_hour = 12; // Mock current hour
        let diff = current_hour.saturating_sub(hour);
        if diff == 0 {
            "Just now".to_string()
        } else if diff == 1 {
            "1 hour ago".to_string()
        } else {
            format!("{} hours ago", diff)
        }
    } else if timestamp.starts_with("2025-02-26") {
        "Yesterday".to_string()
    } else {
        "2 days ago".to_string()
    }
}

/// Get action icon class
pub fn get_action_icon_class(action: &super::types::AuditActionType) -> &'static str {
    use super::types::AuditActionType;
    match action {
        AuditActionType::Create => "icon-create",
        AuditActionType::Update => "icon-update",
        AuditActionType::Delete => "icon-delete",
        AuditActionType::View => "icon-view",
        AuditActionType::Approve => "icon-approve",
        AuditActionType::Reject => "icon-reject",
        AuditActionType::Submit => "icon-submit",
        AuditActionType::Cancel => "icon-cancel",
        AuditActionType::Login => "icon-login",
        AuditActionType::Logout => "icon-logout",
        AuditActionType::Export => "icon-export",
        AuditActionType::Import => "icon-import",
    }
}

/// Get action color class
pub fn get_action_color_class(action: &super::types::AuditActionType) -> &'static str {
    use super::types::AuditActionType;
    match action {
        AuditActionType::Create => "action-create",
        AuditActionType::Update => "action-update",
        AuditActionType::Delete => "action-delete",
        AuditActionType::Approve => "action-approve",
        AuditActionType::Reject => "action-reject",
        AuditActionType::Cancel => "action-cancel",
        AuditActionType::Login | AuditActionType::Logout => "action-auth",
        _ => "action-default",
    }
}
