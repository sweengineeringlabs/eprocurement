//! Audit trail list page with filters

use components::prelude::*;
use wasm_bindgen::JsCast;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, data_table, DataTableColumn, DataTableRow,
    status_badge, StatusType,
    tag, TagType,
    empty_state,
};
use crate::shared::forms::filter_bar;
use crate::util::format::format_date;
use super::store::AuditStore;
use super::types::{AuditEntityType, AuditActionType, AuditEntry};
use super::service;

/// Audit trail list page
#[component]
pub fn audit_trail() -> View {
    let store = use_context::<AuditStore>();

    // Load data on mount
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                service::load_audit_entries(&store).await;
            });
        }
    });

    // Filter signals
    let entity_type_filter = signal(String::new());
    let action_type_filter = signal(String::new());
    let user_filter = signal(String::new());
    let date_from_filter = signal(String::new());
    let date_to_filter = signal(String::new());
    let search_filter = signal(String::new());

    // Handle entity type filter change
    let handle_entity_type_change = Callback::new({
        let store = store.clone();
        let entity_type_filter = entity_type_filter.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            let value = select.value();
            entity_type_filter.set(value.clone());

            let entity_type = if value.is_empty() {
                None
            } else {
                Some(AuditEntityType::from_str(&value))
            };
            store.set_filter_entity_type(entity_type);
        }
    });

    // Handle action type filter change
    let handle_action_type_change = Callback::new({
        let store = store.clone();
        let action_type_filter = action_type_filter.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            let value = select.value();
            action_type_filter.set(value.clone());

            let action_type = if value.is_empty() {
                None
            } else {
                Some(AuditActionType::from_str(&value))
            };
            store.set_filter_action_type(action_type);
        }
    });

    // Handle user filter change
    let handle_user_change = Callback::new({
        let store = store.clone();
        let user_filter = user_filter.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let value = input.value();
            user_filter.set(value.clone());
            store.set_filter_user(if value.is_empty() { None } else { Some(value) });
        }
    });

    // Handle date from filter change
    let handle_date_from_change = Callback::new({
        let store = store.clone();
        let date_from_filter = date_from_filter.clone();
        let date_to_filter = date_to_filter.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let value = input.value();
            date_from_filter.set(value.clone());
            store.set_filter_date_range(
                if value.is_empty() { None } else { Some(value) },
                if date_to_filter.get().is_empty() { None } else { Some(date_to_filter.get()) },
            );
        }
    });

    // Handle date to filter change
    let handle_date_to_change = Callback::new({
        let store = store.clone();
        let date_from_filter = date_from_filter.clone();
        let date_to_filter = date_to_filter.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let value = input.value();
            date_to_filter.set(value.clone());
            store.set_filter_date_range(
                if date_from_filter.get().is_empty() { None } else { Some(date_from_filter.get()) },
                if value.is_empty() { None } else { Some(value) },
            );
        }
    });

    // Handle search filter change
    let handle_search_change = Callback::new({
        let store = store.clone();
        let search_filter = search_filter.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let value = input.value();
            search_filter.set(value.clone());
            store.set_filter_search(if value.is_empty() { None } else { Some(value) });
        }
    });

    // Handle clear filters
    let handle_clear_filters = Callback::new({
        let store = store.clone();
        let entity_type_filter = entity_type_filter.clone();
        let action_type_filter = action_type_filter.clone();
        let user_filter = user_filter.clone();
        let date_from_filter = date_from_filter.clone();
        let date_to_filter = date_to_filter.clone();
        let search_filter = search_filter.clone();
        move |_| {
            entity_type_filter.set(String::new());
            action_type_filter.set(String::new());
            user_filter.set(String::new());
            date_from_filter.set(String::new());
            date_to_filter.set(String::new());
            search_filter.set(String::new());
            store.clear_filters();
        }
    });

    // Handle export
    let handle_export = Callback::new({
        let store = store.clone();
        move |_| {
            let store = store.clone();
            spawn(async move {
                let request = super::types::AuditExportRequest {
                    filter: store.filter.get().clone(),
                    format: super::types::ExportFormat::Csv,
                    include_changes: true,
                };
                match service::export_audit_entries(&store, request).await {
                    Ok(url) => {
                        // In production, would trigger download
                        web_sys::console::log_1(&format!("Export URL: {}", url).into());
                    }
                    Err(e) => {
                        store.error.set(Some(e));
                    }
                }
            });
        }
    });

    // Get filtered entries
    let filtered_entries = {
        let store = store.clone();
        move || store.get_filtered_entries()
    };

    // Stats
    let stats = store.stats.clone();
    let loading = store.loading.clone();

    // Table columns
    let columns = vec![
        DataTableColumn {
            key: "timestamp".to_string(),
            label: "Timestamp".to_string(),
            width: Some("160px".to_string()),
            align: None,
            cell_class: Some("timestamp-cell".to_string()),
        },
        DataTableColumn {
            key: "user".to_string(),
            label: "User".to_string(),
            width: Some("180px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "action".to_string(),
            label: "Action".to_string(),
            width: Some("100px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "entity".to_string(),
            label: "Entity".to_string(),
            width: Some("120px".to_string()),
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "details".to_string(),
            label: "Details".to_string(),
            width: None,
            align: None,
            cell_class: None,
        },
        DataTableColumn {
            key: "ip".to_string(),
            label: "IP Address".to_string(),
            width: Some("130px".to_string()),
            align: None,
            cell_class: Some("ip-cell".to_string()),
        },
        DataTableColumn {
            key: "actions".to_string(),
            label: "".to_string(),
            width: Some("80px".to_string()),
            align: Some("right".to_string()),
            cell_class: None,
        },
    ];

    // Transform entries to table rows
    let rows = move || -> Vec<DataTableRow> {
        filtered_entries().iter().map(|entry| {
            let action_badge = get_action_badge(&entry.action);
            let entity_tag = get_entity_tag(&entry.entity_type);
            let timestamp_formatted = service::format_audit_timestamp(&entry.timestamp);
            let relative_time = service::get_relative_time(&entry.timestamp);

            let details_text = entry.description.clone()
                .unwrap_or_else(|| entry.summary());

            let changes_indicator = if entry.has_changes() {
                view! {
                    <span class="changes-indicator" title={format!("{} field(s) changed", entry.changes.len())}>
                        {format!("{} changes", entry.changes.len())}
                    </span>
                }
            } else {
                view! { <span></span> }
            };

            DataTableRow {
                id: entry.id.clone(),
                cells: vec![
                    view! {
                        <div class="timestamp-cell">
                            <span class="timestamp-full">{timestamp_formatted}</span>
                            <span class="timestamp-relative">{relative_time}</span>
                        </div>
                    },
                    view! {
                        <div class="user-cell">
                            <span class="user-name">{entry.user_name.clone()}</span>
                            <span class="user-email">{entry.user_email.clone()}</span>
                        </div>
                    },
                    action_badge,
                    entity_tag,
                    view! {
                        <div class="details-cell">
                            <span class="details-text">{details_text}</span>
                            <span class="entity-id">{entry.entity_id.clone()}</span>
                            {changes_indicator}
                        </div>
                    },
                    view! { <span class="ip-cell">{entry.ip_address.clone()}</span> },
                    view! {
                        <div class="row-actions">
                            <button class="btn btn-sm btn-secondary" title="View Details">"View"</button>
                        </div>
                    },
                ],
            }
        }).collect()
    };

    // Handle row click
    let handle_row_click = Callback::new({
        let store = store.clone();
        move |entry_id: String| {
            store.select_entry(&entry_id);
            // In production, would show detail modal or navigate
        }
    });

    // Icons for stats
    let icon_activity = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="22 12 18 12 15 21 9 3 6 12 2 12"/></svg>"#;
    let icon_users = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>"#;
    let icon_clock = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>"#;
    let icon_shield = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>"#;

    view! {
        style {
            r#"
            .audit-trail-page {
                display: flex;
                flex-direction: column;
                gap: var(--space-6);
            }
            .stats-cards {
                display: grid;
                grid-template-columns: repeat(4, 1fr);
                gap: 16px;
            }
            @media (max-width: 1024px) {
                .stats-cards { grid-template-columns: repeat(2, 1fr); }
            }
            @media (max-width: 600px) {
                .stats-cards { grid-template-columns: 1fr; }
            }
            .stat-card {
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius-lg);
                padding: 20px;
                display: flex;
                align-items: center;
                gap: 16px;
            }
            .stat-icon {
                width: 48px;
                height: 48px;
                border-radius: var(--radius-md);
                display: flex;
                align-items: center;
                justify-content: center;
            }
            .stat-icon svg {
                width: 24px;
                height: 24px;
            }
            .stat-icon.blue { background: var(--blue-light); color: var(--blue); }
            .stat-icon.green { background: var(--green-light); color: var(--green); }
            .stat-icon.orange { background: var(--orange-light); color: var(--orange); }
            .stat-icon.purple { background: var(--purple-light); color: var(--purple); }
            .stat-content h3 {
                font-size: 24px;
                font-weight: 700;
                color: var(--navy);
                margin-bottom: 4px;
            }
            .stat-content p {
                font-size: 13px;
                color: var(--text-muted);
            }

            .filter-row {
                display: flex;
                flex-wrap: wrap;
                gap: 12px;
                align-items: flex-end;
            }
            .filter-group {
                display: flex;
                flex-direction: column;
                gap: 4px;
            }
            .filter-group label {
                font-size: 11px;
                font-weight: 500;
                color: var(--text-muted);
                text-transform: uppercase;
            }
            .filter-group select,
            .filter-group input {
                padding: 8px 12px;
                border: 1px solid var(--border);
                border-radius: var(--radius-md);
                font-size: 13px;
                background: var(--surface);
                min-width: 150px;
            }
            .filter-group input[type="date"] {
                min-width: 140px;
            }
            .filter-group input[type="text"] {
                min-width: 200px;
            }
            .filter-group select:focus,
            .filter-group input:focus {
                outline: none;
                border-color: var(--blue);
            }
            .filter-spacer { flex: 1; }
            .filter-actions {
                display: flex;
                gap: 8px;
            }

            .timestamp-cell {
                display: flex;
                flex-direction: column;
                gap: 2px;
            }
            .timestamp-full {
                font-family: IBM Plex Mono, monospace;
                font-size: 12px;
                color: var(--text);
            }
            .timestamp-relative {
                font-size: 11px;
                color: var(--text-muted);
            }

            .user-cell {
                display: flex;
                flex-direction: column;
                gap: 2px;
            }
            .user-name {
                font-weight: 500;
                font-size: 13px;
            }
            .user-email {
                font-size: 11px;
                color: var(--text-muted);
            }

            .details-cell {
                display: flex;
                flex-direction: column;
                gap: 4px;
            }
            .details-text {
                font-size: 13px;
                max-width: 300px;
                white-space: nowrap;
                overflow: hidden;
                text-overflow: ellipsis;
            }
            .entity-id {
                font-family: IBM Plex Mono, monospace;
                font-size: 11px;
                color: var(--text-muted);
            }
            .changes-indicator {
                display: inline-flex;
                align-items: center;
                gap: 4px;
                font-size: 10px;
                color: var(--blue);
                background: var(--blue-light);
                padding: 2px 6px;
                border-radius: var(--radius-sm);
                width: fit-content;
            }

            .ip-cell {
                font-family: IBM Plex Mono, monospace;
                font-size: 11px;
                color: var(--text-muted);
            }

            .action-badge {
                display: inline-flex;
                align-items: center;
                gap: 4px;
                padding: 4px 8px;
                border-radius: var(--radius-sm);
                font-size: 11px;
                font-weight: 500;
            }
            .action-badge.create { background: var(--green-light); color: var(--green); }
            .action-badge.update { background: var(--blue-light); color: var(--blue); }
            .action-badge.delete { background: var(--red-light); color: var(--red); }
            .action-badge.approve { background: var(--green-light); color: var(--green); }
            .action-badge.reject { background: var(--red-light); color: var(--red); }
            .action-badge.view { background: var(--bg); color: var(--text-muted); }
            .action-badge.submit { background: var(--purple-light); color: var(--purple); }
            .action-badge.cancel { background: var(--orange-light); color: var(--orange); }
            .action-badge.login { background: var(--blue-light); color: var(--blue); }
            .action-badge.logout { background: var(--bg); color: var(--text-muted); }
            .action-badge.export { background: var(--purple-light); color: var(--purple); }
            .action-badge.import { background: var(--purple-light); color: var(--purple); }

            .entity-tag {
                display: inline-flex;
                padding: 4px 8px;
                border-radius: var(--radius-sm);
                font-size: 11px;
                font-weight: 500;
                background: var(--bg);
                color: var(--text);
            }

            .row-actions { display: flex; gap: 8px; justify-content: flex-end; }
            .loading-state {
                padding: 40px;
                text-align: center;
                color: var(--text-muted);
            }
            "#
        }

        <div class="audit-trail-page" data-testid="audit-trail">
            {page_header(
                "Audit Trail".to_string(),
                Some("Track all system activities and changes".to_string()),
                vec![
                    view! { <button class="btn btn-secondary" on:click={handle_export}>"Export"</button> },
                ]
            )}

            // Stats cards
            <div class="stats-cards">
                <div class="stat-card">
                    <div class="stat-icon blue" inner_html={icon_activity}></div>
                    <div class="stat-content">
                        <h3>{move || stats.get().total_entries.to_string()}</h3>
                        <p>"Total Entries"</p>
                    </div>
                </div>
                <div class="stat-card">
                    <div class="stat-icon green" inner_html={icon_clock}></div>
                    <div class="stat-content">
                        <h3>{move || stats.get().entries_today.to_string()}</h3>
                        <p>"Today's Activity"</p>
                    </div>
                </div>
                <div class="stat-card">
                    <div class="stat-icon orange" inner_html={icon_users}></div>
                    <div class="stat-content">
                        <h3>{move || stats.get().unique_users.to_string()}</h3>
                        <p>"Active Users"</p>
                    </div>
                </div>
                <div class="stat-card">
                    <div class="stat-icon purple" inner_html={icon_shield}></div>
                    <div class="stat-content">
                        <h3>{move || stats.get().logins.to_string()}</h3>
                        <p>"Login Events"</p>
                    </div>
                </div>
            </div>

            // Filters
            {filter_bar(vec![
                view! {
                    <div class="filter-group">
                        <label>"Entity Type"</label>
                        <select on:change={handle_entity_type_change}>
                            <option value="">"All Entities"</option>
                            <option value="requisition">"Requisition"</option>
                            <option value="tender">"Tender"</option>
                            <option value="bid">"Bid"</option>
                            <option value="evaluation">"Evaluation"</option>
                            <option value="contract">"Contract"</option>
                            <option value="purchase_order">"Purchase Order"</option>
                            <option value="goods_receipt">"Goods Receipt"</option>
                            <option value="supplier">"Supplier"</option>
                            <option value="user">"User"</option>
                            <option value="system">"System"</option>
                        </select>
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"Action Type"</label>
                        <select on:change={handle_action_type_change}>
                            <option value="">"All Actions"</option>
                            <option value="create">"Create"</option>
                            <option value="update">"Update"</option>
                            <option value="delete">"Delete"</option>
                            <option value="view">"View"</option>
                            <option value="approve">"Approve"</option>
                            <option value="reject">"Reject"</option>
                            <option value="submit">"Submit"</option>
                            <option value="cancel">"Cancel"</option>
                            <option value="login">"Login"</option>
                            <option value="logout">"Logout"</option>
                            <option value="export">"Export"</option>
                            <option value="import">"Import"</option>
                        </select>
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"User"</label>
                        <input
                            type="text"
                            placeholder="Search user..."
                            value={user_filter.get()}
                            on:input={handle_user_change}
                        />
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"Date From"</label>
                        <input
                            type="date"
                            value={date_from_filter.get()}
                            on:change={handle_date_from_change}
                        />
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"Date To"</label>
                        <input
                            type="date"
                            value={date_to_filter.get()}
                            on:change={handle_date_to_change}
                        />
                    </div>
                },
                view! { <div class="filter-spacer"></div> },
                view! {
                    <div class="filter-group">
                        <label>"Search"</label>
                        <input
                            type="text"
                            placeholder="Search audit logs..."
                            value={search_filter.get()}
                            on:input={handle_search_change}
                        />
                    </div>
                },
                view! {
                    <div class="filter-actions">
                        <button class="btn btn-sm btn-secondary" on:click={handle_clear_filters}>"Clear Filters"</button>
                    </div>
                },
            ])}

            // Audit entries table
            if loading.get() {
                <div class="loading-state">"Loading audit entries..."</div>
            } else if rows().is_empty() {
                {empty_state(
                    "No audit entries found".to_string(),
                    Some("Try adjusting your filters to see more results".to_string()),
                    None
                )}
            } else {
                {panel(
                    format!("Audit Entries ({})", rows().len()),
                    vec![],
                    vec![data_table(columns.clone(), rows(), Some(handle_row_click))]
                )}
            }
        </div>
    }
}

/// Get action badge view
fn get_action_badge(action: &AuditActionType) -> View {
    let class = match action {
        AuditActionType::Create => "action-badge create",
        AuditActionType::Update => "action-badge update",
        AuditActionType::Delete => "action-badge delete",
        AuditActionType::View => "action-badge view",
        AuditActionType::Approve => "action-badge approve",
        AuditActionType::Reject => "action-badge reject",
        AuditActionType::Submit => "action-badge submit",
        AuditActionType::Cancel => "action-badge cancel",
        AuditActionType::Login => "action-badge login",
        AuditActionType::Logout => "action-badge logout",
        AuditActionType::Export => "action-badge export",
        AuditActionType::Import => "action-badge import",
    };

    view! {
        <span class={class}>{action.as_str()}</span>
    }
}

/// Get entity tag view
fn get_entity_tag(entity_type: &AuditEntityType) -> View {
    view! {
        <span class="entity-tag">{entity_type.as_str()}</span>
    }
}

/// Audit entry detail modal component
#[component]
pub fn audit_entry_detail(entry: AuditEntry) -> View {
    view! {
        style {
            r#"
            .audit-detail {
                padding: 20px;
            }
            .audit-detail-header {
                margin-bottom: 20px;
            }
            .audit-detail-header h3 {
                font-size: 18px;
                font-weight: 600;
                margin-bottom: 8px;
            }
            .audit-detail-meta {
                display: flex;
                gap: 16px;
                flex-wrap: wrap;
                font-size: 13px;
                color: var(--text-muted);
            }
            .audit-detail-section {
                margin-bottom: 20px;
            }
            .audit-detail-section h4 {
                font-size: 14px;
                font-weight: 600;
                margin-bottom: 12px;
                color: var(--navy);
            }
            .audit-detail-grid {
                display: grid;
                grid-template-columns: 120px 1fr;
                gap: 8px 16px;
            }
            .audit-detail-grid dt {
                font-size: 12px;
                color: var(--text-muted);
            }
            .audit-detail-grid dd {
                font-size: 13px;
                color: var(--text);
            }
            .changes-list {
                border: 1px solid var(--border);
                border-radius: var(--radius-md);
                overflow: hidden;
            }
            .change-item {
                display: grid;
                grid-template-columns: 150px 1fr 1fr;
                gap: 16px;
                padding: 12px 16px;
                border-bottom: 1px solid var(--border);
                font-size: 13px;
            }
            .change-item:last-child {
                border-bottom: none;
            }
            .change-item .field-name {
                font-weight: 500;
                color: var(--navy);
            }
            .change-item .old-value {
                color: var(--red);
                text-decoration: line-through;
            }
            .change-item .new-value {
                color: var(--green);
            }
            "#
        }

        <div class="audit-detail">
            <div class="audit-detail-header">
                <h3>{entry.summary()}</h3>
                <div class="audit-detail-meta">
                    <span>{service::format_audit_timestamp(&entry.timestamp)}</span>
                    <span>{format!("IP: {}", entry.ip_address)}</span>
                </div>
            </div>

            <div class="audit-detail-section">
                <h4>"User Information"</h4>
                <dl class="audit-detail-grid">
                    <dt>"Name"</dt>
                    <dd>{entry.user_name.clone()}</dd>
                    <dt>"Email"</dt>
                    <dd>{entry.user_email.clone()}</dd>
                    <dt>"User ID"</dt>
                    <dd>{entry.user_id.clone()}</dd>
                </dl>
            </div>

            <div class="audit-detail-section">
                <h4>"Action Details"</h4>
                <dl class="audit-detail-grid">
                    <dt>"Action"</dt>
                    <dd>{entry.action.as_str()}</dd>
                    <dt>"Entity Type"</dt>
                    <dd>{entry.entity_type.as_str()}</dd>
                    <dt>"Entity ID"</dt>
                    <dd>{entry.entity_id.clone()}</dd>
                    <dt>"Entity Name"</dt>
                    <dd>{entry.entity_name.clone().unwrap_or_else(|| "-".to_string())}</dd>
                </dl>
            </div>

            if !entry.changes.is_empty() {
                <div class="audit-detail-section">
                    <h4>"Changes"</h4>
                    <div class="changes-list">
                        {entry.changes.iter().map(|change| {
                            view! {
                                <div class="change-item">
                                    <span class="field-name">{change.field_name.clone()}</span>
                                    <span class="old-value">{change.old_value.clone().unwrap_or_else(|| "(empty)".to_string())}</span>
                                    <span class="new-value">{change.new_value.clone().unwrap_or_else(|| "(empty)".to_string())}</span>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            }

            if let Some(description) = entry.description.as_ref() {
                <div class="audit-detail-section">
                    <h4>"Description"</h4>
                    <p>{description.clone()}</p>
                </div>
            }
        </div>
    }
}
