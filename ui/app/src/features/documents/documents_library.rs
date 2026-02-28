//! Document library view - Browse documents with folder structure, search, upload, preview

use components::prelude::*;
use wasm_bindgen::JsCast;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel,
    tag, TagType,
    pagination,
    kpi_card, KpiColor,
    empty_state,
    modal, ModalSize,
};
use crate::shared::forms::filter_bar;
use crate::util::format::format_number;
use super::store::DocumentsStore;
use super::types::{
    Document, DocumentFolder, DocumentType, DocumentCategory,
    DocumentSortBy, DocumentViewMode, format_file_size,
};
use super::service;

/// Document library page
#[component]
pub fn documents_library() -> View {
    let store = use_context::<DocumentsStore>();

    // Local state
    let search_query = signal(String::new());
    let show_upload_modal = signal(false);
    let show_preview_modal = signal(false);
    let upload_name = signal(String::new());
    let upload_category = signal("general".to_string());
    let upload_tags = signal(String::new());
    let selected_type_filter = signal(String::new());
    let selected_category_filter = signal(String::new());

    // Load data on mount
    effect({
        let store = store.clone();
        move || {
            let store = store.clone();
            spawn(async move {
                service::load_documents(&store).await;
            });
        }
    });

    let stats = store.stats.clone();
    let loading = store.loading.clone();
    let view_mode = store.view_mode.clone();
    let current_path = store.current_path.clone();
    let folders = store.folders.clone();
    let preview_document = store.preview_document.clone();

    // Handle search
    let handle_search = Callback::new({
        let store = store.clone();
        let search_query = search_query.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let value = input.value();
            search_query.set(value.clone());
            store.set_search(if value.is_empty() { None } else { Some(value) });
        }
    });

    // Handle type filter
    let handle_type_change = Callback::new({
        let store = store.clone();
        let selected_type_filter = selected_type_filter.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            let value = select.value();
            selected_type_filter.set(value.clone());
            let doc_type = if value.is_empty() {
                None
            } else {
                Some(DocumentType::from_extension(&value))
            };
            store.set_type_filter(doc_type);
        }
    });

    // Handle category filter
    let handle_category_change = Callback::new({
        let store = store.clone();
        let selected_category_filter = selected_category_filter.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            let value = select.value();
            selected_category_filter.set(value.clone());
            let category = if value.is_empty() {
                None
            } else {
                Some(DocumentCategory::from_str(&value))
            };
            store.set_category_filter(category);
        }
    });

    // Handle sort change
    let handle_sort_change = Callback::new({
        let store = store.clone();
        move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            let value = select.value();
            store.set_sort(DocumentSortBy::from_str(&value));
        }
    });

    // Handle view mode toggle
    let handle_toggle_grid: Callback<()> = Callback::new({
        let store = store.clone();
        move |_| {
            store.set_view_mode(DocumentViewMode::Grid);
        }
    });

    let handle_toggle_list: Callback<()> = Callback::new({
        let store = store.clone();
        move |_| {
            store.set_view_mode(DocumentViewMode::List);
        }
    });

    // Clear filters
    let handle_clear_filters: Callback<()> = Callback::new({
        let store = store.clone();
        let search_query = search_query.clone();
        let selected_type_filter = selected_type_filter.clone();
        let selected_category_filter = selected_category_filter.clone();
        move |_| {
            search_query.set(String::new());
            selected_type_filter.set(String::new());
            selected_category_filter.set(String::new());
            store.clear_filters();
        }
    });

    // Handle folder navigation
    let handle_folder_click = Callback::new({
        let store = store.clone();
        move |folder_path: String| {
            store.navigate_to_folder(&folder_path);
        }
    });

    // Handle navigate up
    let handle_navigate_up: Callback<()> = Callback::new({
        let store = store.clone();
        move |_| {
            store.navigate_up();
        }
    });

    // Handle document click
    let handle_document_click = Callback::new({
        let store = store.clone();
        move |document_id: String| {
            store.select_document(&document_id);
            // Navigate to document detail
            web_sys::window()
                .unwrap()
                .location()
                .set_href(&format!("#/documents/{}", document_id))
                .ok();
        }
    });

    // Handle document preview
    let handle_preview = Callback::new({
        let store = store.clone();
        let show_preview_modal = show_preview_modal.clone();
        move |document_id: String| {
            store.set_preview(&document_id);
            show_preview_modal.set(true);
        }
    });

    // Close preview modal
    let handle_close_preview: Callback<()> = Callback::new({
        let store = store.clone();
        let show_preview_modal = show_preview_modal.clone();
        move |_| {
            store.clear_preview();
            show_preview_modal.set(false);
        }
    });

    // Handle upload modal
    let handle_open_upload: Callback<()> = Callback::new({
        let show_upload_modal = show_upload_modal.clone();
        move |_| {
            show_upload_modal.set(true);
        }
    });

    let handle_close_upload: Callback<()> = Callback::new({
        let show_upload_modal = show_upload_modal.clone();
        move |_| {
            show_upload_modal.set(false);
        }
    });

    // Handle page change
    let handle_page_change = Callback::new({
        let store = store.clone();
        move |page: u32| {
            store.set_page(page as usize);
        }
    });

    // Get filtered documents and breadcrumbs
    let filtered_documents = store.get_paginated_documents();
    let current_folders = store.get_current_folders();
    let breadcrumbs = store.get_breadcrumbs();
    let stats_data = stats.get();
    let total_pages = store.total_pages();
    let current_page = store.page.get();

    // Icons
    let icon_file = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>"#;
    let icon_folder = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>"#;
    let icon_upload = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/></svg>"#;
    let icon_storage = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><ellipse cx="12" cy="5" rx="9" ry="3"/><path d="M21 12c0 1.66-4 3-9 3s-9-1.34-9-3"/><path d="M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5"/></svg>"#;
    let icon_calendar = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="4" width="18" height="18" rx="2" ry="2"/><line x1="16" y1="2" x2="16" y2="6"/><line x1="8" y1="2" x2="8" y2="6"/><line x1="3" y1="10" x2="21" y2="10"/></svg>"#;

    view! {
        style {
            r#"
            .documents-page { display: flex; flex-direction: column; gap: var(--space-4); }

            .kpi-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; margin-bottom: 8px; }
            @media (max-width: 1200px) { .kpi-grid { grid-template-columns: repeat(2, 1fr); } }
            @media (max-width: 768px) { .kpi-grid { grid-template-columns: 1fr; } }

            .documents-layout { display: grid; grid-template-columns: 240px 1fr; gap: 24px; }
            @media (max-width: 992px) { .documents-layout { grid-template-columns: 1fr; } }

            .folders-sidebar {
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius-lg);
                padding: 16px;
                height: fit-content;
                position: sticky;
                top: 16px;
            }
            .folders-title {
                font-size: 14px;
                font-weight: 600;
                color: var(--text);
                margin-bottom: 12px;
                padding-bottom: 8px;
                border-bottom: 1px solid var(--border);
            }
            .folder-list { display: flex; flex-direction: column; gap: 4px; }
            .folder-item {
                display: flex;
                align-items: center;
                gap: 8px;
                padding: 8px 12px;
                border-radius: var(--radius-sm);
                cursor: pointer;
                font-size: 13px;
                color: var(--text-muted);
                transition: all 0.15s;
            }
            .folder-item:hover { background: var(--bg); color: var(--text); }
            .folder-item.active { background: var(--blue-bg); color: var(--blue); font-weight: 500; }
            .folder-item svg { width: 16px; height: 16px; }
            .folder-count {
                margin-left: auto;
                font-size: 11px;
                background: var(--bg);
                padding: 2px 6px;
                border-radius: 10px;
            }
            .folder-item.active .folder-count { background: var(--blue); color: white; }

            .breadcrumbs {
                display: flex;
                align-items: center;
                gap: 8px;
                font-size: 13px;
                color: var(--text-muted);
                margin-bottom: 16px;
            }
            .breadcrumb-item {
                cursor: pointer;
                transition: color 0.15s;
            }
            .breadcrumb-item:hover { color: var(--blue); }
            .breadcrumb-item.current { color: var(--text); font-weight: 500; cursor: default; }
            .breadcrumb-separator { color: var(--border); }

            .filter-group { display: flex; align-items: center; gap: 8px; }
            .filter-group label { font-size: 12px; font-weight: 500; color: var(--text-muted); }
            .filter-group select,
            .filter-group input {
                padding: 6px 10px;
                border: 1px solid var(--border);
                border-radius: var(--radius-sm);
                font-size: 12px;
                background: var(--surface);
            }
            .filter-group select:focus,
            .filter-group input:focus {
                outline: none;
                border-color: var(--blue);
            }
            .filter-spacer { flex: 1; }
            .search-input { min-width: 220px; }

            .view-toggle {
                display: flex;
                gap: 4px;
                background: var(--bg);
                padding: 4px;
                border-radius: var(--radius-sm);
            }
            .view-toggle button {
                padding: 4px 12px;
                border: none;
                background: transparent;
                border-radius: var(--radius-sm);
                cursor: pointer;
                font-size: 12px;
                color: var(--text-muted);
            }
            .view-toggle button.active {
                background: var(--surface);
                color: var(--text);
                box-shadow: var(--shadow-sm);
            }

            .documents-content { flex: 1; }

            .folder-grid {
                display: grid;
                grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
                gap: 12px;
                margin-bottom: 24px;
            }
            .folder-card {
                display: flex;
                align-items: center;
                gap: 12px;
                padding: 12px 16px;
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius-md);
                cursor: pointer;
                transition: all 0.15s;
            }
            .folder-card:hover {
                border-color: var(--blue);
                background: var(--blue-bg);
            }
            .folder-card svg { width: 24px; height: 24px; color: var(--blue); }
            .folder-card-info { min-width: 0; }
            .folder-card-name {
                font-weight: 500;
                color: var(--text);
                white-space: nowrap;
                overflow: hidden;
                text-overflow: ellipsis;
            }
            .folder-card-count { font-size: 11px; color: var(--text-muted); }

            .document-grid {
                display: grid;
                grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
                gap: 16px;
            }
            .document-card {
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius-lg);
                overflow: hidden;
                cursor: pointer;
                transition: transform 0.2s, box-shadow 0.2s;
            }
            .document-card:hover {
                transform: translateY(-2px);
                box-shadow: var(--shadow);
            }
            .document-card.locked { opacity: 0.7; }
            .document-thumbnail {
                height: 120px;
                background: var(--bg);
                display: flex;
                align-items: center;
                justify-content: center;
                position: relative;
            }
            .document-thumbnail svg { width: 48px; height: 48px; }
            .document-type-pdf { color: #EF4444; }
            .document-type-word { color: #3B82F6; }
            .document-type-excel { color: #10B981; }
            .document-type-image { color: #8B5CF6; }
            .document-type-archive { color: #F59E0B; }
            .document-type-other { color: #6B7280; }
            .document-badges {
                position: absolute;
                top: 8px;
                right: 8px;
                display: flex;
                flex-direction: column;
                gap: 4px;
            }
            .document-badge {
                font-size: 9px;
                font-weight: 600;
                padding: 2px 6px;
                border-radius: 4px;
                text-transform: uppercase;
            }
            .document-badge.locked { background: var(--red); color: white; }
            .document-badge.archived { background: var(--text-muted); color: white; }
            .document-badge.version { background: var(--blue); color: white; }
            .document-content { padding: 12px; }
            .document-name {
                font-size: 13px;
                font-weight: 500;
                color: var(--text);
                margin-bottom: 4px;
                max-height: 2.6em;
                overflow: hidden;
                text-overflow: ellipsis;
            }
            .document-meta {
                font-size: 11px;
                color: var(--text-muted);
                display: flex;
                justify-content: space-between;
            }
            .document-actions {
                display: flex;
                gap: 4px;
                padding: 8px 12px;
                border-top: 1px solid var(--border);
                background: var(--bg);
            }
            .document-actions button {
                flex: 1;
                padding: 4px 8px;
                border: none;
                background: transparent;
                color: var(--text-muted);
                font-size: 11px;
                cursor: pointer;
                border-radius: var(--radius-sm);
                transition: all 0.15s;
            }
            .document-actions button:hover {
                background: var(--surface);
                color: var(--blue);
            }

            .document-list { display: flex; flex-direction: column; gap: 8px; }
            .document-row {
                display: grid;
                grid-template-columns: 40px 1fr 100px 100px 120px 80px;
                align-items: center;
                gap: 16px;
                padding: 12px 16px;
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: var(--radius-md);
                cursor: pointer;
                transition: background 0.15s;
            }
            .document-row:hover { background: var(--bg); }
            .document-row-icon svg { width: 24px; height: 24px; }
            .document-row-info { min-width: 0; }
            .document-row-name {
                font-weight: 500;
                color: var(--text);
                white-space: nowrap;
                overflow: hidden;
                text-overflow: ellipsis;
            }
            .document-row-file { font-size: 11px; color: var(--text-muted); }
            .document-row-category { font-size: 12px; color: var(--text-muted); }
            .document-row-size {
                font-size: 12px;
                font-family: IBM Plex Mono, monospace;
                color: var(--text-muted);
                text-align: right;
            }
            .document-row-date { font-size: 12px; color: var(--text-muted); }
            .document-row-actions button {
                padding: 4px 8px;
                border: 1px solid var(--border);
                background: var(--surface);
                border-radius: var(--radius-sm);
                font-size: 11px;
                cursor: pointer;
            }
            .document-row-actions button:hover {
                border-color: var(--blue);
                color: var(--blue);
            }

            .loading-state {
                display: flex;
                align-items: center;
                justify-content: center;
                padding: 60px;
                color: var(--text-muted);
            }

            .preview-modal-content {
                width: 90vw;
                max-width: 1000px;
                height: 80vh;
            }
            .preview-frame {
                width: 100%;
                height: 100%;
                border: none;
                background: var(--bg);
            }
            .preview-placeholder {
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                height: 100%;
                color: var(--text-muted);
                gap: 16px;
            }
            .preview-placeholder svg { width: 64px; height: 64px; }

            .upload-modal-content { width: 500px; }
            .upload-dropzone {
                border: 2px dashed var(--border);
                border-radius: var(--radius-lg);
                padding: 40px;
                text-align: center;
                cursor: pointer;
                transition: all 0.15s;
                margin-bottom: 24px;
            }
            .upload-dropzone:hover {
                border-color: var(--blue);
                background: var(--blue-bg);
            }
            .upload-dropzone svg { width: 48px; height: 48px; color: var(--text-muted); margin-bottom: 16px; }
            .upload-dropzone-text { color: var(--text); font-weight: 500; margin-bottom: 4px; }
            .upload-dropzone-hint { font-size: 12px; color: var(--text-muted); }
            .upload-form { display: flex; flex-direction: column; gap: 16px; }
            .upload-form-group { display: flex; flex-direction: column; gap: 4px; }
            .upload-form-group label { font-size: 12px; font-weight: 500; color: var(--text-muted); }
            .upload-form-group input,
            .upload-form-group select,
            .upload-form-group textarea {
                padding: 8px 12px;
                border: 1px solid var(--border);
                border-radius: var(--radius-sm);
                font-size: 14px;
            }
            .upload-form-group input:focus,
            .upload-form-group select:focus,
            .upload-form-group textarea:focus {
                outline: none;
                border-color: var(--blue);
            }
            .upload-actions { display: flex; justify-content: flex-end; gap: 8px; margin-top: 16px; }
            "#
        }

        <div class="documents-page" data-testid="documents-library">
            {page_header(
                "Document Library".to_string(),
                Some("Manage and organize procurement documents".to_string()),
                vec![
                    view! {
                        <button class="btn btn-primary" on:click={handle_open_upload}>
                            <span inner_html={icon_upload}></span>
                            " Upload Document"
                        </button>
                    },
                ]
            )}

            // KPI summary
            <div class="kpi-grid">
                {kpi_card(
                    "Total Documents".to_string(),
                    format_number(stats_data.total_documents),
                    KpiColor::Blue,
                    icon_file.to_string(),
                    None,
                    None
                )}
                {kpi_card(
                    "Total Storage".to_string(),
                    format_file_size(stats_data.total_size),
                    KpiColor::Green,
                    icon_storage.to_string(),
                    None,
                    None
                )}
                {kpi_card(
                    "Folders".to_string(),
                    stats_data.folder_count.to_string(),
                    KpiColor::Accent,
                    icon_folder.to_string(),
                    None,
                    None
                )}
                {kpi_card(
                    "This Month".to_string(),
                    stats_data.documents_this_month.to_string(),
                    KpiColor::Orange,
                    icon_calendar.to_string(),
                    None,
                    None
                )}
            </div>

            // Filter bar
            {filter_bar(vec![
                view! {
                    <div class="filter-group">
                        <label>"Type"</label>
                        <select on:change={handle_type_change}>
                            <option value="">"All Types"</option>
                            <option value="pdf">"PDF"</option>
                            <option value="docx">"Word"</option>
                            <option value="xlsx">"Excel"</option>
                            <option value="png">"Image"</option>
                            <option value="zip">"Archive"</option>
                        </select>
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"Category"</label>
                        <select on:change={handle_category_change}>
                            <option value="">"All Categories"</option>
                            <option value="tender">"Tender"</option>
                            <option value="contract">"Contract"</option>
                            <option value="bid">"Bid"</option>
                            <option value="compliance">"Compliance"</option>
                            <option value="report">"Report"</option>
                            <option value="template">"Template"</option>
                        </select>
                    </div>
                },
                view! {
                    <div class="filter-group">
                        <label>"Sort By"</label>
                        <select on:change={handle_sort_change}>
                            <option value="date_desc">"Date (Newest)"</option>
                            <option value="date_asc">"Date (Oldest)"</option>
                            <option value="name_asc">"Name (A-Z)"</option>
                            <option value="name_desc">"Name (Z-A)"</option>
                            <option value="size_desc">"Size (Largest)"</option>
                            <option value="size_asc">"Size (Smallest)"</option>
                        </select>
                    </div>
                },
                view! { <div class="filter-spacer"></div> },
                view! {
                    <div class="filter-group">
                        <input
                            type="text"
                            class="search-input"
                            placeholder="Search documents..."
                            value={search_query.get()}
                            on:input={handle_search}
                        />
                    </div>
                },
                view! {
                    <div class="view-toggle">
                        <button
                            class={if matches!(view_mode.get(), DocumentViewMode::Grid) { "active" } else { "" }}
                            on:click={handle_toggle_grid}
                        >"Grid"</button>
                        <button
                            class={if matches!(view_mode.get(), DocumentViewMode::List) { "active" } else { "" }}
                            on:click={handle_toggle_list}
                        >"List"</button>
                    </div>
                },
                view! {
                    <button class="btn btn-sm btn-secondary" on:click={handle_clear_filters}>"Clear"</button>
                },
            ])}

            // Main layout with sidebar
            <div class="documents-layout">
                // Folders sidebar
                <div class="folders-sidebar">
                    <div class="folders-title">"Folders"</div>
                    <div class="folder-list">
                        <div
                            class={if current_path.get() == "/" { "folder-item active" } else { "folder-item" }}
                            on:click={Callback::<web_sys::MouseEvent>::new({
                                let handle_folder_click = handle_folder_click.clone();
                                move |_| handle_folder_click.call("/".to_string())
                            })}
                        >
                            <span inner_html={icon_folder}></span>
                            <span>"All Documents"</span>
                        </div>
                        for folder in folders.get().iter() {
                            {folder_sidebar_item(folder.clone(), current_path.get(), handle_folder_click.clone())}
                        }
                    </div>
                </div>

                // Documents content
                <div class="documents-content">
                    // Breadcrumbs
                    <div class="breadcrumbs">
                        for (i, (name, path)) in breadcrumbs.iter().enumerate() {
                            if i > 0 {
                                <span class="breadcrumb-separator">"/"</span>
                            }
                            if i == breadcrumbs.len() - 1 {
                                <span class="breadcrumb-item current">{name.clone()}</span>
                            } else {
                                <span
                                    class="breadcrumb-item"
                                    on:click={Callback::<web_sys::MouseEvent>::new({
                                        let handle_folder_click = handle_folder_click.clone();
                                        let path = path.clone();
                                        move |_| handle_folder_click.call(path.clone())
                                    })}
                                >{name.clone()}</span>
                            }
                        }
                    </div>

                    {panel(
                        format!("Documents ({} found)", filtered_documents.len()),
                        vec![],
                        vec![
                            if loading.get() {
                                view! { <div class="loading-state">"Loading documents..."</div> }
                            } else if filtered_documents.is_empty() && current_folders.is_empty() {
                                {empty_state(
                                    "No documents found".to_string(),
                                    Some("Upload documents or adjust your filters".to_string()),
                                    None,
                                    None
                                )}
                            } else {
                                view! {
                                    <div>
                                        // Folders grid
                                        if !current_folders.is_empty() {
                                            <div class="folder-grid">
                                                for folder in current_folders.iter() {
                                                    {folder_card(folder.clone(), handle_folder_click.clone())}
                                                }
                                            </div>
                                        }

                                        // Documents grid or list
                                        if matches!(view_mode.get(), DocumentViewMode::List) {
                                            <div class="document-list">
                                                for doc in filtered_documents.iter() {
                                                    {document_row(doc.clone(), handle_document_click.clone(), handle_preview.clone())}
                                                }
                                            </div>
                                        } else {
                                            <div class="document-grid">
                                                for doc in filtered_documents.iter() {
                                                    {document_card(doc.clone(), handle_document_click.clone(), handle_preview.clone())}
                                                }
                                            </div>
                                        }
                                    </div>
                                }
                            },
                            view! {
                                <div style="margin-top: 16px;">
                                    {pagination(current_page as u32, total_pages as u32, handle_page_change)}
                                </div>
                            }
                        ]
                    )}
                </div>
            </div>

            // Preview modal
            if show_preview_modal.get() {
                {modal(
                    "Document Preview".to_string(),
                    ModalSize::Large,
                    show_preview_modal.clone(),
                    handle_close_preview.clone(),
                    vec![view! {
                        <div class="preview-modal-content">
                            if let Some(doc) = preview_document.get() {
                                if doc.can_preview() {
                                    if doc.document_type == DocumentType::Image {
                                        <img src={doc.file_url.clone()} alt={doc.name.clone()} style="max-width: 100%; max-height: 100%; object-fit: contain;" />
                                    } else {
                                        <div class="preview-placeholder">
                                            <span inner_html={service::get_document_type_icon(&doc.document_type)}></span>
                                            <div>"Preview: " {doc.name.clone()}</div>
                                            <a href={doc.file_url.clone()} target="_blank" class="btn btn-primary">"Open in New Tab"</a>
                                        </div>
                                    }
                                } else {
                                    <div class="preview-placeholder">
                                        <span inner_html={icon_file}></span>
                                        <div>"Preview not available for this file type"</div>
                                        <a href={doc.file_url.clone()} download={doc.file_name.clone()} class="btn btn-primary">"Download"</a>
                                    </div>
                                }
                            }
                        </div>
                    }],
                    vec![]
                )}
            }

            // Upload modal
            if show_upload_modal.get() {
                {modal(
                    "Upload Document".to_string(),
                    ModalSize::Medium,
                    show_upload_modal.clone(),
                    handle_close_upload.clone(),
                    vec![view! {
                        <div class="upload-modal-content">
                            <div class="upload-dropzone">
                                <span inner_html={icon_upload}></span>
                                <div class="upload-dropzone-text">"Drop files here or click to browse"</div>
                                <div class="upload-dropzone-hint">"Maximum file size: 50MB"</div>
                            </div>

                            <div class="upload-form">
                                <div class="upload-form-group">
                                    <label>"Document Name"</label>
                                    <input
                                        type="text"
                                        placeholder="Enter document name"
                                        value={upload_name.get()}
                                        on:input={Callback::new({
                                            let upload_name = upload_name.clone();
                                            move |e: web_sys::Event| {
                                                let target = e.target().unwrap();
                                                let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
                                                upload_name.set(input.value());
                                            }
                                        })}
                                    />
                                </div>

                                <div class="upload-form-group">
                                    <label>"Category"</label>
                                    <select
                                        value={upload_category.get()}
                                        on:change={Callback::new({
                                            let upload_category = upload_category.clone();
                                            move |e: web_sys::Event| {
                                                let target = e.target().unwrap();
                                                let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
                                                upload_category.set(select.value());
                                            }
                                        })}
                                    >
                                        <option value="general">"General"</option>
                                        <option value="tender">"Tender"</option>
                                        <option value="contract">"Contract"</option>
                                        <option value="bid">"Bid"</option>
                                        <option value="compliance">"Compliance"</option>
                                        <option value="report">"Report"</option>
                                        <option value="template">"Template"</option>
                                    </select>
                                </div>

                                <div class="upload-form-group">
                                    <label>"Tags (comma-separated)"</label>
                                    <input
                                        type="text"
                                        placeholder="e.g. important, q1, project-x"
                                        value={upload_tags.get()}
                                        on:input={Callback::new({
                                            let upload_tags = upload_tags.clone();
                                            move |e: web_sys::Event| {
                                                let target = e.target().unwrap();
                                                let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
                                                upload_tags.set(input.value());
                                            }
                                        })}
                                    />
                                </div>

                                <div class="upload-actions">
                                    <button class="btn btn-secondary" on:click={handle_close_upload.clone()}>"Cancel"</button>
                                    <button class="btn btn-primary">"Upload"</button>
                                </div>
                            </div>
                        </div>
                    }],
                    vec![]
                )}
            }
        </div>
    }
}

/// Folder sidebar item
fn folder_sidebar_item(
    folder: DocumentFolder,
    current_path: String,
    on_click: Callback<String>,
) -> View {
    let folder_path = folder.path.clone();
    let is_active = current_path == folder_path;
    let class_name = if is_active { "folder-item active" } else { "folder-item" };

    let icon_folder = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>"#;

    view! {
        <div
            class={class_name}
            on:click={Callback::<web_sys::MouseEvent>::new({
                let folder_path = folder_path.clone();
                move |_| on_click.call(folder_path.clone())
            })}
        >
            <span inner_html={icon_folder}></span>
            <span>{folder.name.clone()}</span>
            <span class="folder-count">{folder.document_count.to_string()}</span>
        </div>
    }
}

/// Folder card for grid view
fn folder_card(folder: DocumentFolder, on_click: Callback<String>) -> View {
    let folder_path = folder.path.clone();

    let icon_folder = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>"#;

    let handle_click = Callback::unit({
        let on_click = on_click.clone();
        let folder_path = folder_path.clone();
        move || on_click.call(folder_path.clone())
    });

    view! {
        <div
            class="folder-card"
            on:click={handle_click}
        >
            <span inner_html={icon_folder}></span>
            <div class="folder-card-info">
                <div class="folder-card-name">{folder.name.clone()}</div>
                <div class="folder-card-count">{format!("{} documents", folder.document_count)}</div>
            </div>
        </div>
    }
}

/// Document card for grid view
fn document_card(
    doc: Document,
    on_click: Callback<String>,
    on_preview: Callback<String>,
) -> View {
    let doc_id = doc.id.clone();
    let doc_id_preview = doc.id.clone();
    let type_class = match doc.document_type {
        DocumentType::Pdf => "document-type-pdf",
        DocumentType::Word => "document-type-word",
        DocumentType::Excel => "document-type-excel",
        DocumentType::Image => "document-type-image",
        DocumentType::Archive => "document-type-archive",
        _ => "document-type-other",
    };

    let card_class = if doc.is_locked { "document-card locked" } else { "document-card" };

    view! {
        <div class={card_class}>
            <div
                class="document-thumbnail"
                on:click={Callback::<web_sys::MouseEvent>::new({
                    let doc_id = doc_id.clone();
                    move |_| on_click.call(doc_id.clone())
                })}
            >
                <span class={type_class} inner_html={service::get_document_type_icon(&doc.document_type)}></span>
                <div class="document-badges">
                    if doc.is_locked {
                        <span class="document-badge locked">"Locked"</span>
                    }
                    if doc.is_archived {
                        <span class="document-badge archived">"Archived"</span>
                    }
                    if doc.version > 1 {
                        <span class="document-badge version">{format!("v{}", doc.version)}</span>
                    }
                </div>
            </div>
            <div class="document-content">
                <div class="document-name">{doc.name.clone()}</div>
                <div class="document-meta">
                    <span>{doc.category.label()}</span>
                    <span>{format_file_size(doc.size)}</span>
                </div>
            </div>
            <div class="document-actions">
                <button on:click={Callback::new({
                    move |e: web_sys::MouseEvent| {
                        e.stop_propagation();
                        on_preview.call(doc_id_preview.clone())
                    }
                })}>"Preview"</button>
                <button>"Download"</button>
            </div>
        </div>
    }
}

/// Document row for list view
fn document_row(
    doc: Document,
    on_click: Callback<String>,
    on_preview: Callback<String>,
) -> View {
    let doc_id = doc.id.clone();
    let doc_id_preview = doc.id.clone();
    let type_class = match doc.document_type {
        DocumentType::Pdf => "document-type-pdf",
        DocumentType::Word => "document-type-word",
        DocumentType::Excel => "document-type-excel",
        DocumentType::Image => "document-type-image",
        DocumentType::Archive => "document-type-archive",
        _ => "document-type-other",
    };

    view! {
        <div
            class="document-row"
            on:click={Callback::<web_sys::MouseEvent>::new({
                let doc_id = doc_id.clone();
                move |_| on_click.call(doc_id.clone())
            })}
        >
            <div class={format!("document-row-icon {}", type_class)}>
                <span inner_html={service::get_document_type_icon(&doc.document_type)}></span>
            </div>
            <div class="document-row-info">
                <div class="document-row-name">
                    {doc.name.clone()}
                    if doc.is_locked {
                        " "
                        {tag("Locked".to_string(), TagType::Red)}
                    }
                </div>
                <div class="document-row-file">{doc.file_name.clone()}</div>
            </div>
            <div class="document-row-category">{doc.category.label()}</div>
            <div class="document-row-size">{format_file_size(doc.size)}</div>
            <div class="document-row-date">{service::format_document_date(&doc.updated_at)}</div>
            <div class="document-row-actions">
                <button on:click={Callback::new({
                    move |e: web_sys::MouseEvent| {
                        e.stop_propagation();
                        on_preview.call(doc_id_preview.clone())
                    }
                })}>"Preview"</button>
            </div>
        </div>
    }
}
