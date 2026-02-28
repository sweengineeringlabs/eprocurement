//! Documents service - API calls

use super::store::{DocumentsStore, load_mock_data, select_document, clear_selection};
use super::types::{
    Document, DocumentFolder, DocumentFilter, DocumentUploadRequest,
    DocumentType, DocumentCategory, DocumentVersion, format_file_size,
};

/// Load all documents
pub async fn load_documents(store: &DocumentsStore) {
    store.loading.set(true);
    store.error.set(None);

    // In production, this would call the API:
    // let response = api::get("/api/documents").await;
    // For now, load mock data
    load_mock_data(store);

    store.loading.set(false);
}

/// Load documents with filter
pub async fn load_filtered_documents(store: &DocumentsStore, filter: DocumentFilter) {
    store.loading.set(true);
    store.error.set(None);

    // Set the filter
    store.filter.set(filter);

    // Ensure documents are loaded
    if store.documents.get().is_empty() {
        load_mock_data(store);
    }

    store.loading.set(false);
}

/// Load document by ID
pub async fn load_document(store: &DocumentsStore, document_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // Ensure documents are loaded
    if store.documents.get().is_empty() {
        load_mock_data(store);
    }

    // Select the document
    select_document(store, document_id);

    store.loading.set(false);
}

/// Load documents for a specific entity
pub async fn load_entity_documents(
    store: &DocumentsStore,
    entity_type: &str,
    entity_id: &str,
) {
    store.loading.set(true);
    store.error.set(None);

    // Ensure documents are loaded
    if store.documents.get().is_empty() {
        load_mock_data(store);
    }

    // Set filter for specific entity
    let mut filter = DocumentFilter::default();
    filter.related_entity_type = Some(entity_type.to_string());
    filter.related_entity_id = Some(entity_id.to_string());
    store.filter.set(filter);

    store.loading.set(false);
}

/// Load documents in a folder
pub async fn load_folder_documents(store: &DocumentsStore, folder_id: &str) {
    store.loading.set(true);
    store.error.set(None);

    // Ensure documents are loaded
    if store.documents.get().is_empty() {
        load_mock_data(store);
    }

    // Find folder and navigate to it
    let folders = store.folders.get();
    if let Some(folder) = folders.iter().find(|f| f.id == folder_id) {
        store.navigate_to_folder(&folder.path);
    }

    store.loading.set(false);
}

/// Search documents
pub async fn search_documents(store: &DocumentsStore, query: &str) {
    store.loading.set(true);
    store.error.set(None);

    // Ensure documents are loaded
    if store.documents.get().is_empty() {
        load_mock_data(store);
    }

    // Set search filter
    store.set_search(if query.is_empty() { None } else { Some(query.to_string()) });

    store.loading.set(false);
}

/// Upload new document
pub async fn upload_document(
    store: &DocumentsStore,
    request: DocumentUploadRequest,
    file_name: &str,
    file_size: u64,
    _file_data: Vec<u8>,
) -> Result<Document, String> {
    store.loading.set(true);
    store.upload_progress.set(Some(0.0));
    store.error.set(None);

    // Simulate upload progress
    for progress in [0.2, 0.4, 0.6, 0.8, 1.0] {
        store.upload_progress.set(Some(progress));
        // In production, this would be actual upload progress
    }

    // In production, this would POST to the API
    // For now, create mock document
    let extension = file_name.split('.').last().unwrap_or("").to_string();
    let doc_type = DocumentType::from_extension(&extension);

    let new_doc = Document {
        id: format!("doc_{:03}", store.documents.get().len() + 1),
        name: request.name.clone(),
        file_name: file_name.to_string(),
        description: request.description,
        document_type: doc_type,
        category: request.category,
        version: 1,
        uploaded_by: "usr_current".to_string(),
        uploaded_by_name: "Current User".to_string(),
        size: file_size,
        tags: request.tags,
        file_url: format!("/documents/{}", file_name),
        thumbnail_url: None,
        mime_type: super::types::mime_type_from_extension(&extension).to_string(),
        extension,
        related_entity_type: request.related_entity_type,
        related_entity_id: request.related_entity_id,
        related_entity_name: None,
        folder_id: request.folder_id.clone(),
        folder_path: store.current_path.get(),
        versions: vec![],
        is_public: request.is_public,
        permissions: vec![],
        created_at: "2025-02-27".to_string(),
        updated_at: "2025-02-27".to_string(),
        last_accessed_at: None,
        download_count: 0,
        is_archived: false,
        is_locked: false,
        locked_by: None,
    };

    // Add to store
    let mut documents = store.documents.get();
    documents.insert(0, new_doc.clone());
    store.documents.set(documents);
    store.calculate_stats();

    store.upload_progress.set(None);
    store.loading.set(false);
    Ok(new_doc)
}

/// Update document metadata
pub async fn update_document(
    store: &DocumentsStore,
    document_id: &str,
    name: Option<String>,
    description: Option<String>,
    category: Option<DocumentCategory>,
    tags: Option<Vec<String>>,
) -> Result<Document, String> {
    store.loading.set(true);
    store.error.set(None);

    let mut documents = store.documents.get();
    if let Some(pos) = documents.iter().position(|d| d.id == document_id) {
        if let Some(n) = name {
            documents[pos].name = n;
        }
        if let Some(d) = description {
            documents[pos].description = Some(d);
        }
        if let Some(c) = category {
            documents[pos].category = c;
        }
        if let Some(t) = tags {
            documents[pos].tags = t;
        }
        documents[pos].updated_at = "2025-02-27".to_string();

        let updated_doc = documents[pos].clone();
        store.documents.set(documents);
        store.selected.set(Some(updated_doc.clone()));
        store.loading.set(false);
        Ok(updated_doc)
    } else {
        store.loading.set(false);
        store.error.set(Some("Document not found".to_string()));
        Err("Document not found".to_string())
    }
}

/// Upload new version of document
pub async fn upload_new_version(
    store: &DocumentsStore,
    document_id: &str,
    _file_data: Vec<u8>,
    file_size: u64,
    comment: Option<String>,
) -> Result<Document, String> {
    store.loading.set(true);
    store.error.set(None);

    let mut documents = store.documents.get();
    if let Some(pos) = documents.iter().position(|d| d.id == document_id) {
        let new_version = documents[pos].version + 1;

        // Create version entry
        let version_entry = DocumentVersion {
            version: documents[pos].version,
            uploaded_by: documents[pos].uploaded_by.clone(),
            uploaded_by_name: documents[pos].uploaded_by_name.clone(),
            uploaded_at: documents[pos].updated_at.clone(),
            size: documents[pos].size,
            file_url: documents[pos].file_url.clone(),
            comment: None,
        };

        documents[pos].versions.push(version_entry);
        documents[pos].version = new_version;
        documents[pos].size = file_size;
        documents[pos].updated_at = "2025-02-27".to_string();
        documents[pos].uploaded_by = "usr_current".to_string();
        documents[pos].uploaded_by_name = "Current User".to_string();

        let updated_doc = documents[pos].clone();
        store.documents.set(documents);
        store.selected.set(Some(updated_doc.clone()));
        store.loading.set(false);
        Ok(updated_doc)
    } else {
        store.loading.set(false);
        store.error.set(Some("Document not found".to_string()));
        Err("Document not found".to_string())
    }
}

/// Delete document
pub async fn delete_document(store: &DocumentsStore, document_id: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut documents = store.documents.get();
    if let Some(pos) = documents.iter().position(|d| d.id == document_id) {
        // Check if locked
        if documents[pos].is_locked {
            store.loading.set(false);
            store.error.set(Some("Cannot delete locked document".to_string()));
            return Err("Cannot delete locked document".to_string());
        }

        documents.remove(pos);
        store.documents.set(documents);
        clear_selection(store);
        store.calculate_stats();
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Document not found".to_string()));
        Err("Document not found".to_string())
    }
}

/// Archive document
pub async fn archive_document(store: &DocumentsStore, document_id: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut documents = store.documents.get();
    if let Some(pos) = documents.iter().position(|d| d.id == document_id) {
        documents[pos].is_archived = true;
        documents[pos].updated_at = "2025-02-27".to_string();
        store.documents.set(documents.clone());
        store.selected.set(Some(documents[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Document not found".to_string()));
        Err("Document not found".to_string())
    }
}

/// Restore archived document
pub async fn restore_document(store: &DocumentsStore, document_id: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut documents = store.documents.get();
    if let Some(pos) = documents.iter().position(|d| d.id == document_id) {
        documents[pos].is_archived = false;
        documents[pos].updated_at = "2025-02-27".to_string();
        store.documents.set(documents.clone());
        store.selected.set(Some(documents[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Document not found".to_string()));
        Err("Document not found".to_string())
    }
}

/// Lock document
pub async fn lock_document(store: &DocumentsStore, document_id: &str, reason: Option<String>) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut documents = store.documents.get();
    if let Some(pos) = documents.iter().position(|d| d.id == document_id) {
        documents[pos].is_locked = true;
        documents[pos].locked_by = Some(reason.unwrap_or_else(|| "Current User".to_string()));
        documents[pos].updated_at = "2025-02-27".to_string();
        store.documents.set(documents.clone());
        store.selected.set(Some(documents[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Document not found".to_string()));
        Err("Document not found".to_string())
    }
}

/// Unlock document
pub async fn unlock_document(store: &DocumentsStore, document_id: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut documents = store.documents.get();
    if let Some(pos) = documents.iter().position(|d| d.id == document_id) {
        documents[pos].is_locked = false;
        documents[pos].locked_by = None;
        documents[pos].updated_at = "2025-02-27".to_string();
        store.documents.set(documents.clone());
        store.selected.set(Some(documents[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Document not found".to_string()));
        Err("Document not found".to_string())
    }
}

/// Move document to folder
pub async fn move_document(
    store: &DocumentsStore,
    document_id: &str,
    folder_id: Option<String>,
) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let mut documents = store.documents.get();
    if let Some(pos) = documents.iter().position(|d| d.id == document_id) {
        let new_path = if let Some(ref fid) = folder_id {
            let folders = store.folders.get();
            folders.iter().find(|f| &f.id == fid)
                .map(|f| f.path.clone())
                .unwrap_or_else(|| "/".to_string())
        } else {
            "/".to_string()
        };

        documents[pos].folder_id = folder_id;
        documents[pos].folder_path = new_path;
        documents[pos].updated_at = "2025-02-27".to_string();
        store.documents.set(documents.clone());
        store.selected.set(Some(documents[pos].clone()));
        store.loading.set(false);
        Ok(())
    } else {
        store.loading.set(false);
        store.error.set(Some("Document not found".to_string()));
        Err("Document not found".to_string())
    }
}

/// Create new folder
pub async fn create_folder(
    store: &DocumentsStore,
    name: &str,
    parent_id: Option<String>,
) -> Result<DocumentFolder, String> {
    store.loading.set(true);
    store.error.set(None);

    let folders = store.folders.get();

    // Determine path
    let path = if let Some(ref pid) = parent_id {
        let parent = folders.iter().find(|f| &f.id == pid);
        if let Some(p) = parent {
            format!("{}/{}", p.path, name)
        } else {
            format!("/{}", name)
        }
    } else {
        format!("/{}", name)
    };

    // Check if folder already exists
    if folders.iter().any(|f| f.path == path) {
        store.loading.set(false);
        store.error.set(Some("Folder already exists".to_string()));
        return Err("Folder already exists".to_string());
    }

    let new_folder = DocumentFolder {
        id: format!("folder_{:03}", folders.len() + 1),
        name: name.to_string(),
        path,
        parent_id,
        icon: None,
        color: None,
        document_count: 0,
        created_at: "2025-02-27".to_string(),
        created_by: "usr_current".to_string(),
        is_system: false,
    };

    let mut updated_folders = folders;
    updated_folders.push(new_folder.clone());
    store.folders.set(updated_folders);

    store.loading.set(false);
    Ok(new_folder)
}

/// Delete folder
pub async fn delete_folder(store: &DocumentsStore, folder_id: &str) -> Result<(), String> {
    store.loading.set(true);
    store.error.set(None);

    let folders = store.folders.get();

    // Check if folder exists
    let folder = folders.iter().find(|f| f.id == folder_id);
    if folder.is_none() {
        store.loading.set(false);
        store.error.set(Some("Folder not found".to_string()));
        return Err("Folder not found".to_string());
    }

    let folder = folder.unwrap();

    // Check if system folder
    if folder.is_system {
        store.loading.set(false);
        store.error.set(Some("Cannot delete system folder".to_string()));
        return Err("Cannot delete system folder".to_string());
    }

    // Check if folder has documents
    let documents = store.documents.get();
    if documents.iter().any(|d| d.folder_id.as_ref() == Some(&folder_id.to_string())) {
        store.loading.set(false);
        store.error.set(Some("Cannot delete folder with documents".to_string()));
        return Err("Cannot delete folder with documents".to_string());
    }

    // Remove folder
    let updated_folders: Vec<DocumentFolder> = folders.into_iter()
        .filter(|f| f.id != folder_id)
        .collect();
    store.folders.set(updated_folders);

    store.loading.set(false);
    Ok(())
}

/// Download document (returns URL)
pub async fn download_document(store: &DocumentsStore, document_id: &str) -> Result<String, String> {
    let documents = store.documents.get();
    if let Some(doc) = documents.iter().find(|d| d.id == document_id) {
        // In production, might generate signed URL
        // For now, return the file URL
        Ok(doc.file_url.clone())
    } else {
        Err("Document not found".to_string())
    }
}

/// Get document preview URL
pub fn get_preview_url(document: &Document) -> Option<String> {
    if document.can_preview() {
        // In production, would generate preview URL based on type
        Some(format!("/api/documents/{}/preview", document.id))
    } else {
        None
    }
}

/// Format document timestamp for display
pub fn format_document_date(timestamp: &str) -> String {
    // Simple formatting - in production would use proper date library
    if timestamp.len() >= 10 {
        timestamp[0..10].to_string()
    } else {
        timestamp.to_string()
    }
}

/// Get relative time description
pub fn get_relative_time(timestamp: &str) -> String {
    // Simplified relative time
    if timestamp.starts_with("2025-02-27") {
        "Today".to_string()
    } else if timestamp.starts_with("2025-02-26") {
        "Yesterday".to_string()
    } else if timestamp.starts_with("2025-02") {
        "This month".to_string()
    } else if timestamp.starts_with("2025-01") {
        "Last month".to_string()
    } else {
        format_document_date(timestamp)
    }
}

/// Get document type icon SVG
pub fn get_document_type_icon(doc_type: &DocumentType) -> &'static str {
    match doc_type {
        DocumentType::Pdf => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><path d="M9 13h6"/><path d="M9 17h3"/></svg>"#,
        DocumentType::Word => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/><polyline points="10 9 9 9 8 9"/></svg>"#,
        DocumentType::Excel => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><rect x="8" y="12" width="8" height="6" rx="1"/></svg>"#,
        DocumentType::Image => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2"/><circle cx="8.5" cy="8.5" r="1.5"/><polyline points="21 15 16 10 5 21"/></svg>"#,
        DocumentType::Text => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/></svg>"#,
        DocumentType::Archive => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 8v13H3V8"/><path d="M23 3H1v5h22V3z"/><path d="M10 12h4"/></svg>"#,
        DocumentType::Other => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>"#,
    }
}

/// Get document type color
pub fn get_document_type_color(doc_type: &DocumentType) -> &'static str {
    match doc_type {
        DocumentType::Pdf => "#EF4444",
        DocumentType::Word => "#3B82F6",
        DocumentType::Excel => "#10B981",
        DocumentType::Image => "#8B5CF6",
        DocumentType::Text => "#6B7280",
        DocumentType::Archive => "#F59E0B",
        DocumentType::Other => "#9CA3AF",
    }
}
