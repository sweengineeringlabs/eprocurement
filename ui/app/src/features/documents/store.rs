//! Documents store

use components::prelude::*;
use super::types::{
    Document, DocumentFolder, DocumentFilter, DocumentStats, DocumentType,
    DocumentCategory, DocumentSortBy, DocumentViewMode, DocumentVersion,
    format_file_size,
};

/// Documents state store
#[derive(Clone)]
pub struct DocumentsStore {
    pub documents: Signal<Vec<Document>>,
    pub folders: Signal<Vec<DocumentFolder>>,
    pub selected: Signal<Option<Document>>,
    pub selected_folder: Signal<Option<DocumentFolder>>,
    pub filter: Signal<DocumentFilter>,
    pub sort_by: Signal<DocumentSortBy>,
    pub stats: Signal<DocumentStats>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
    pub view_mode: Signal<DocumentViewMode>,
    pub current_path: Signal<String>,
    pub page: Signal<usize>,
    pub page_size: Signal<usize>,
    pub total_count: Signal<usize>,
    pub upload_progress: Signal<Option<f32>>,
    pub preview_document: Signal<Option<Document>>,
}

impl DocumentsStore {
    pub fn new() -> Self {
        Self {
            documents: signal(Vec::new()),
            folders: signal(Vec::new()),
            selected: signal(None),
            selected_folder: signal(None),
            filter: signal(DocumentFilter::default()),
            sort_by: signal(DocumentSortBy::default()),
            stats: signal(DocumentStats::default()),
            loading: signal(false),
            error: signal(None),
            view_mode: signal(DocumentViewMode::Grid),
            current_path: signal("/".to_string()),
            page: signal(1),
            page_size: signal(24),
            total_count: signal(0),
            upload_progress: signal(None),
            preview_document: signal(None),
        }
    }

    /// Get filtered and sorted documents
    pub fn get_filtered_documents(&self) -> Vec<Document> {
        let documents = self.documents.get();
        let filter = self.filter.get();
        let sort_by = self.sort_by.get();
        let current_path = self.current_path.get();

        let mut filtered: Vec<Document> = documents.iter()
            .filter(|doc| {
                // Filter by current folder path
                if !doc.folder_path.starts_with(&current_path) {
                    return false;
                }

                // Archived filter
                if !filter.include_archived && doc.is_archived {
                    return false;
                }

                // Search query
                if let Some(ref query) = filter.search_query {
                    let q = query.to_lowercase();
                    let matches = doc.name.to_lowercase().contains(&q)
                        || doc.file_name.to_lowercase().contains(&q)
                        || doc.description.as_ref().map(|d| d.to_lowercase().contains(&q)).unwrap_or(false)
                        || doc.tags.iter().any(|t| t.to_lowercase().contains(&q));
                    if !matches {
                        return false;
                    }
                }

                // Document type filter
                if let Some(doc_type) = filter.document_type {
                    if doc.document_type != doc_type {
                        return false;
                    }
                }

                // Category filter
                if let Some(category) = filter.category {
                    if doc.category != category {
                        return false;
                    }
                }

                // Folder filter
                if let Some(ref folder_id) = filter.folder_id {
                    if doc.folder_id.as_ref() != Some(folder_id) {
                        return false;
                    }
                }

                // Uploaded by filter
                if let Some(ref uploaded_by) = filter.uploaded_by {
                    if &doc.uploaded_by != uploaded_by {
                        return false;
                    }
                }

                // Tags filter
                if !filter.tags.is_empty() {
                    let has_tag = filter.tags.iter().any(|t| doc.tags.contains(t));
                    if !has_tag {
                        return false;
                    }
                }

                // Date range filter
                if let Some(ref date_from) = filter.date_from {
                    if doc.created_at < *date_from {
                        return false;
                    }
                }

                if let Some(ref date_to) = filter.date_to {
                    if doc.created_at > *date_to {
                        return false;
                    }
                }

                // Related entity filter
                if let Some(ref entity_type) = filter.related_entity_type {
                    if doc.related_entity_type.as_ref() != Some(entity_type) {
                        return false;
                    }
                }

                if let Some(ref entity_id) = filter.related_entity_id {
                    if doc.related_entity_id.as_ref() != Some(entity_id) {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect();

        // Sort documents
        match sort_by {
            DocumentSortBy::NameAsc => filtered.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
            DocumentSortBy::NameDesc => filtered.sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase())),
            DocumentSortBy::DateAsc => filtered.sort_by(|a, b| a.created_at.cmp(&b.created_at)),
            DocumentSortBy::DateDesc => filtered.sort_by(|a, b| b.created_at.cmp(&a.created_at)),
            DocumentSortBy::SizeAsc => filtered.sort_by(|a, b| a.size.cmp(&b.size)),
            DocumentSortBy::SizeDesc => filtered.sort_by(|a, b| b.size.cmp(&a.size)),
            DocumentSortBy::TypeAsc => filtered.sort_by(|a, b| a.document_type.label().cmp(b.document_type.label())),
            DocumentSortBy::TypeDesc => filtered.sort_by(|a, b| b.document_type.label().cmp(a.document_type.label())),
        }

        filtered
    }

    /// Get folders at current path
    pub fn get_current_folders(&self) -> Vec<DocumentFolder> {
        let folders = self.folders.get();
        let current_path = self.current_path.get();

        folders.iter()
            .filter(|folder| {
                if current_path == "/" {
                    folder.parent_id.is_none()
                } else {
                    folder.path.starts_with(&current_path) && folder.path != current_path
                }
            })
            .cloned()
            .collect()
    }

    /// Select a document
    pub fn select_document(&self, document_id: &str) {
        let documents = self.documents.get();
        if let Some(doc) = documents.iter().find(|d| d.id == document_id) {
            self.selected.set(Some(doc.clone()));
        }
    }

    /// Clear document selection
    pub fn clear_selection(&self) {
        self.selected.set(None);
    }

    /// Set preview document
    pub fn set_preview(&self, document_id: &str) {
        let documents = self.documents.get();
        if let Some(doc) = documents.iter().find(|d| d.id == document_id) {
            self.preview_document.set(Some(doc.clone()));
        }
    }

    /// Clear preview
    pub fn clear_preview(&self) {
        self.preview_document.set(None);
    }

    /// Navigate to folder
    pub fn navigate_to_folder(&self, folder_path: &str) {
        self.current_path.set(folder_path.to_string());
        self.selected_folder.set(
            self.folders.get().iter()
                .find(|f| f.path == folder_path)
                .cloned()
        );
    }

    /// Navigate up one level
    pub fn navigate_up(&self) {
        let current = self.current_path.get();
        if current != "/" {
            let parts: Vec<&str> = current.trim_end_matches('/').split('/').collect();
            if parts.len() > 1 {
                let parent = parts[..parts.len() - 1].join("/");
                self.current_path.set(if parent.is_empty() { "/".to_string() } else { parent });
            } else {
                self.current_path.set("/".to_string());
            }
            self.selected_folder.set(None);
        }
    }

    /// Set search filter
    pub fn set_search(&self, query: Option<String>) {
        let mut filter = self.filter.get();
        filter.search_query = query;
        self.filter.set(filter);
    }

    /// Set document type filter
    pub fn set_type_filter(&self, doc_type: Option<DocumentType>) {
        let mut filter = self.filter.get();
        filter.document_type = doc_type;
        self.filter.set(filter);
    }

    /// Set category filter
    pub fn set_category_filter(&self, category: Option<DocumentCategory>) {
        let mut filter = self.filter.get();
        filter.category = category;
        self.filter.set(filter);
    }

    /// Clear all filters
    pub fn clear_filters(&self) {
        self.filter.set(DocumentFilter::default());
    }

    /// Set view mode
    pub fn set_view_mode(&self, mode: DocumentViewMode) {
        self.view_mode.set(mode);
    }

    /// Set sort order
    pub fn set_sort(&self, sort_by: DocumentSortBy) {
        self.sort_by.set(sort_by);
    }

    /// Set page
    pub fn set_page(&self, page: usize) {
        self.page.set(page);
    }

    /// Get paginated documents
    pub fn get_paginated_documents(&self) -> Vec<Document> {
        let filtered = self.get_filtered_documents();
        let page = self.page.get();
        let page_size = self.page_size.get();

        let start = (page - 1) * page_size;
        let end = std::cmp::min(start + page_size, filtered.len());

        if start >= filtered.len() {
            return Vec::new();
        }

        filtered[start..end].to_vec()
    }

    /// Get total pages
    pub fn total_pages(&self) -> usize {
        let filtered_count = self.get_filtered_documents().len();
        let page_size = self.page_size.get();
        ((filtered_count + page_size - 1) / page_size).max(1)
    }

    /// Calculate statistics
    pub fn calculate_stats(&self) {
        let documents = self.documents.get();
        let folders = self.folders.get();

        let total_size: u64 = documents.iter().map(|d| d.size).sum();
        let documents_this_month = documents.iter()
            .filter(|d| d.created_at.starts_with("2025-02"))
            .count() as u32;

        // Count by type
        let mut by_type: Vec<(DocumentType, u32)> = Vec::new();
        for doc_type in DocumentType::all() {
            let count = documents.iter().filter(|d| d.document_type == doc_type).count() as u32;
            if count > 0 {
                by_type.push((doc_type, count));
            }
        }

        // Count by category
        let mut by_category: Vec<(DocumentCategory, u32)> = Vec::new();
        for category in DocumentCategory::all() {
            let count = documents.iter().filter(|d| d.category == category).count() as u32;
            if count > 0 {
                by_category.push((category, count));
            }
        }

        // Recent uploads (last 5)
        let mut recent: Vec<Document> = documents.clone();
        recent.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        let recent_uploads = recent.into_iter().take(5).collect();

        self.stats.set(DocumentStats {
            total_documents: documents.len() as u32,
            total_size,
            documents_this_month,
            folder_count: folders.len() as u32,
            by_type,
            by_category,
            recent_uploads,
        });
    }

    /// Get breadcrumb path segments
    pub fn get_breadcrumbs(&self) -> Vec<(String, String)> {
        let current_path = self.current_path.get();
        let mut breadcrumbs = vec![("Root".to_string(), "/".to_string())];

        if current_path != "/" {
            let parts: Vec<&str> = current_path.trim_matches('/').split('/').collect();
            let mut path_so_far = String::new();
            for part in parts {
                path_so_far.push('/');
                path_so_far.push_str(part);
                breadcrumbs.push((part.to_string(), path_so_far.clone()));
            }
        }

        breadcrumbs
    }
}

/// Load mock document data for demo
pub fn load_mock_data(store: &DocumentsStore) {
    // Create folders
    let folders = vec![
        DocumentFolder {
            id: "folder_001".to_string(),
            name: "Tenders".to_string(),
            path: "/Tenders".to_string(),
            parent_id: None,
            icon: Some("briefcase".to_string()),
            color: Some("#3B82F6".to_string()),
            document_count: 12,
            created_at: "2024-01-15".to_string(),
            created_by: "usr_001".to_string(),
            is_system: true,
        },
        DocumentFolder {
            id: "folder_002".to_string(),
            name: "Contracts".to_string(),
            path: "/Contracts".to_string(),
            parent_id: None,
            icon: Some("file-signature".to_string()),
            color: Some("#10B981".to_string()),
            document_count: 8,
            created_at: "2024-01-15".to_string(),
            created_by: "usr_001".to_string(),
            is_system: true,
        },
        DocumentFolder {
            id: "folder_003".to_string(),
            name: "Compliance".to_string(),
            path: "/Compliance".to_string(),
            parent_id: None,
            icon: Some("shield-check".to_string()),
            color: Some("#F59E0B".to_string()),
            document_count: 15,
            created_at: "2024-01-15".to_string(),
            created_by: "usr_001".to_string(),
            is_system: true,
        },
        DocumentFolder {
            id: "folder_004".to_string(),
            name: "Templates".to_string(),
            path: "/Templates".to_string(),
            parent_id: None,
            icon: Some("file-alt".to_string()),
            color: Some("#8B5CF6".to_string()),
            document_count: 6,
            created_at: "2024-01-15".to_string(),
            created_by: "usr_001".to_string(),
            is_system: true,
        },
        DocumentFolder {
            id: "folder_005".to_string(),
            name: "Reports".to_string(),
            path: "/Reports".to_string(),
            parent_id: None,
            icon: Some("chart-bar".to_string()),
            color: Some("#EF4444".to_string()),
            document_count: 9,
            created_at: "2024-02-01".to_string(),
            created_by: "usr_001".to_string(),
            is_system: true,
        },
    ];

    // Create documents
    let documents = vec![
        Document {
            id: "doc_001".to_string(),
            name: "IT Infrastructure Tender Document".to_string(),
            file_name: "TND-2025-0089-Specification.pdf".to_string(),
            description: Some("Complete specification document for IT Infrastructure Upgrade tender".to_string()),
            document_type: DocumentType::Pdf,
            category: DocumentCategory::Tender,
            version: 2,
            uploaded_by: "usr_001".to_string(),
            uploaded_by_name: "Dr. Thandi Nkosi".to_string(),
            size: 2_458_624,
            tags: vec!["tender".to_string(), "IT".to_string(), "infrastructure".to_string()],
            file_url: "/documents/TND-2025-0089-Specification.pdf".to_string(),
            thumbnail_url: Some("/thumbnails/TND-2025-0089.png".to_string()),
            mime_type: "application/pdf".to_string(),
            extension: "pdf".to_string(),
            related_entity_type: Some("Tender".to_string()),
            related_entity_id: Some("TND-2025-0089".to_string()),
            related_entity_name: Some("IT Infrastructure Upgrade".to_string()),
            folder_id: Some("folder_001".to_string()),
            folder_path: "/Tenders".to_string(),
            versions: vec![
                DocumentVersion {
                    version: 1,
                    uploaded_by: "usr_001".to_string(),
                    uploaded_by_name: "Dr. Thandi Nkosi".to_string(),
                    uploaded_at: "2025-02-01".to_string(),
                    size: 2_345_678,
                    file_url: "/documents/TND-2025-0089-Specification-v1.pdf".to_string(),
                    comment: Some("Initial version".to_string()),
                },
                DocumentVersion {
                    version: 2,
                    uploaded_by: "usr_001".to_string(),
                    uploaded_by_name: "Dr. Thandi Nkosi".to_string(),
                    uploaded_at: "2025-02-15".to_string(),
                    size: 2_458_624,
                    file_url: "/documents/TND-2025-0089-Specification.pdf".to_string(),
                    comment: Some("Updated requirements section".to_string()),
                },
            ],
            is_public: false,
            permissions: Vec::new(),
            created_at: "2025-02-01".to_string(),
            updated_at: "2025-02-15".to_string(),
            last_accessed_at: Some("2025-02-27".to_string()),
            download_count: 45,
            is_archived: false,
            is_locked: false,
            locked_by: None,
        },
        Document {
            id: "doc_002".to_string(),
            name: "Security Services Contract".to_string(),
            file_name: "CON-2025-0042-Agreement.pdf".to_string(),
            description: Some("Signed contract agreement for security services".to_string()),
            document_type: DocumentType::Pdf,
            category: DocumentCategory::Contract,
            version: 1,
            uploaded_by: "usr_002".to_string(),
            uploaded_by_name: "Mr. Johan van der Berg".to_string(),
            size: 1_856_234,
            tags: vec!["contract".to_string(), "security".to_string(), "signed".to_string()],
            file_url: "/documents/CON-2025-0042-Agreement.pdf".to_string(),
            thumbnail_url: Some("/thumbnails/CON-2025-0042.png".to_string()),
            mime_type: "application/pdf".to_string(),
            extension: "pdf".to_string(),
            related_entity_type: Some("Contract".to_string()),
            related_entity_id: Some("CON-2025-0042".to_string()),
            related_entity_name: Some("Security Services Contract".to_string()),
            folder_id: Some("folder_002".to_string()),
            folder_path: "/Contracts".to_string(),
            versions: vec![],
            is_public: false,
            permissions: Vec::new(),
            created_at: "2025-02-10".to_string(),
            updated_at: "2025-02-10".to_string(),
            last_accessed_at: Some("2025-02-26".to_string()),
            download_count: 12,
            is_archived: false,
            is_locked: true,
            locked_by: Some("System - Contract Signed".to_string()),
        },
        Document {
            id: "doc_003".to_string(),
            name: "B-BBEE Certificate - TechSolutions".to_string(),
            file_name: "SUP-001-BBBEE-Certificate.pdf".to_string(),
            description: Some("B-BBEE Level 1 certificate for TechSolutions SA".to_string()),
            document_type: DocumentType::Pdf,
            category: DocumentCategory::Compliance,
            version: 1,
            uploaded_by: "usr_006".to_string(),
            uploaded_by_name: "System Administrator".to_string(),
            size: 524_288,
            tags: vec!["bbbee".to_string(), "compliance".to_string(), "supplier".to_string(), "certificate".to_string()],
            file_url: "/documents/SUP-001-BBBEE-Certificate.pdf".to_string(),
            thumbnail_url: None,
            mime_type: "application/pdf".to_string(),
            extension: "pdf".to_string(),
            related_entity_type: Some("Supplier".to_string()),
            related_entity_id: Some("SUP-001".to_string()),
            related_entity_name: Some("TechSolutions SA (Pty) Ltd".to_string()),
            folder_id: Some("folder_003".to_string()),
            folder_path: "/Compliance".to_string(),
            versions: vec![],
            is_public: false,
            permissions: Vec::new(),
            created_at: "2025-01-15".to_string(),
            updated_at: "2025-01-15".to_string(),
            last_accessed_at: Some("2025-02-26".to_string()),
            download_count: 8,
            is_archived: false,
            is_locked: false,
            locked_by: None,
        },
        Document {
            id: "doc_004".to_string(),
            name: "Requisition Template".to_string(),
            file_name: "Requisition-Template-v3.docx".to_string(),
            description: Some("Standard requisition form template for all departments".to_string()),
            document_type: DocumentType::Word,
            category: DocumentCategory::Template,
            version: 3,
            uploaded_by: "usr_003".to_string(),
            uploaded_by_name: "Ms. Priya Naidoo".to_string(),
            size: 156_789,
            tags: vec!["template".to_string(), "requisition".to_string(), "form".to_string()],
            file_url: "/documents/Requisition-Template-v3.docx".to_string(),
            thumbnail_url: None,
            mime_type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string(),
            extension: "docx".to_string(),
            related_entity_type: None,
            related_entity_id: None,
            related_entity_name: None,
            folder_id: Some("folder_004".to_string()),
            folder_path: "/Templates".to_string(),
            versions: vec![],
            is_public: true,
            permissions: Vec::new(),
            created_at: "2024-06-01".to_string(),
            updated_at: "2025-01-20".to_string(),
            last_accessed_at: Some("2025-02-27".to_string()),
            download_count: 234,
            is_archived: false,
            is_locked: false,
            locked_by: None,
        },
        Document {
            id: "doc_005".to_string(),
            name: "Q4 2024 Procurement Report".to_string(),
            file_name: "Q4-2024-Procurement-Report.xlsx".to_string(),
            description: Some("Quarterly procurement summary report for Q4 2024".to_string()),
            document_type: DocumentType::Excel,
            category: DocumentCategory::Report,
            version: 1,
            uploaded_by: "usr_004".to_string(),
            uploaded_by_name: "Mr. Sipho Dlamini".to_string(),
            size: 892_456,
            tags: vec!["report".to_string(), "quarterly".to_string(), "procurement".to_string(), "2024".to_string()],
            file_url: "/documents/Q4-2024-Procurement-Report.xlsx".to_string(),
            thumbnail_url: None,
            mime_type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string(),
            extension: "xlsx".to_string(),
            related_entity_type: None,
            related_entity_id: None,
            related_entity_name: None,
            folder_id: Some("folder_005".to_string()),
            folder_path: "/Reports".to_string(),
            versions: vec![],
            is_public: false,
            permissions: Vec::new(),
            created_at: "2025-01-10".to_string(),
            updated_at: "2025-01-10".to_string(),
            last_accessed_at: Some("2025-02-20".to_string()),
            download_count: 56,
            is_archived: false,
            is_locked: false,
            locked_by: None,
        },
        Document {
            id: "doc_006".to_string(),
            name: "Bid Evaluation Scorecard".to_string(),
            file_name: "TND-2025-0089-Evaluation-Scorecard.xlsx".to_string(),
            description: Some("Bid evaluation scoring template for IT Infrastructure tender".to_string()),
            document_type: DocumentType::Excel,
            category: DocumentCategory::Tender,
            version: 1,
            uploaded_by: "usr_004".to_string(),
            uploaded_by_name: "Mr. Sipho Dlamini".to_string(),
            size: 245_678,
            tags: vec!["evaluation".to_string(), "scorecard".to_string(), "tender".to_string()],
            file_url: "/documents/TND-2025-0089-Evaluation-Scorecard.xlsx".to_string(),
            thumbnail_url: None,
            mime_type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string(),
            extension: "xlsx".to_string(),
            related_entity_type: Some("Tender".to_string()),
            related_entity_id: Some("TND-2025-0089".to_string()),
            related_entity_name: Some("IT Infrastructure Upgrade".to_string()),
            folder_id: Some("folder_001".to_string()),
            folder_path: "/Tenders".to_string(),
            versions: vec![],
            is_public: false,
            permissions: Vec::new(),
            created_at: "2025-02-20".to_string(),
            updated_at: "2025-02-20".to_string(),
            last_accessed_at: Some("2025-02-27".to_string()),
            download_count: 15,
            is_archived: false,
            is_locked: false,
            locked_by: None,
        },
        Document {
            id: "doc_007".to_string(),
            name: "Company Registration - SecureGuard".to_string(),
            file_name: "SUP-004-Company-Registration.pdf".to_string(),
            description: Some("CIPC company registration certificate for SecureGuard Holdings".to_string()),
            document_type: DocumentType::Pdf,
            category: DocumentCategory::Compliance,
            version: 1,
            uploaded_by: "usr_006".to_string(),
            uploaded_by_name: "System Administrator".to_string(),
            size: 412_345,
            tags: vec!["cipc".to_string(), "registration".to_string(), "supplier".to_string(), "compliance".to_string()],
            file_url: "/documents/SUP-004-Company-Registration.pdf".to_string(),
            thumbnail_url: None,
            mime_type: "application/pdf".to_string(),
            extension: "pdf".to_string(),
            related_entity_type: Some("Supplier".to_string()),
            related_entity_id: Some("SUP-004".to_string()),
            related_entity_name: Some("SecureGuard Holdings (Pty) Ltd".to_string()),
            folder_id: Some("folder_003".to_string()),
            folder_path: "/Compliance".to_string(),
            versions: vec![],
            is_public: false,
            permissions: Vec::new(),
            created_at: "2024-11-20".to_string(),
            updated_at: "2024-11-20".to_string(),
            last_accessed_at: Some("2025-02-15".to_string()),
            download_count: 4,
            is_archived: false,
            is_locked: false,
            locked_by: None,
        },
        Document {
            id: "doc_008".to_string(),
            name: "Tender Invitation Template".to_string(),
            file_name: "Tender-Invitation-Template.docx".to_string(),
            description: Some("Standard tender invitation letter template".to_string()),
            document_type: DocumentType::Word,
            category: DocumentCategory::Template,
            version: 2,
            uploaded_by: "usr_001".to_string(),
            uploaded_by_name: "Dr. Thandi Nkosi".to_string(),
            size: 89_456,
            tags: vec!["template".to_string(), "tender".to_string(), "invitation".to_string()],
            file_url: "/documents/Tender-Invitation-Template.docx".to_string(),
            thumbnail_url: None,
            mime_type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string(),
            extension: "docx".to_string(),
            related_entity_type: None,
            related_entity_id: None,
            related_entity_name: None,
            folder_id: Some("folder_004".to_string()),
            folder_path: "/Templates".to_string(),
            versions: vec![],
            is_public: true,
            permissions: Vec::new(),
            created_at: "2024-03-15".to_string(),
            updated_at: "2024-09-10".to_string(),
            last_accessed_at: Some("2025-02-25".to_string()),
            download_count: 189,
            is_archived: false,
            is_locked: false,
            locked_by: None,
        },
        Document {
            id: "doc_009".to_string(),
            name: "Site Inspection Photos".to_string(),
            file_name: "TND-2025-0076-Site-Photos.zip".to_string(),
            description: Some("Site inspection photographs for Security Services tender".to_string()),
            document_type: DocumentType::Archive,
            category: DocumentCategory::Tender,
            version: 1,
            uploaded_by: "usr_002".to_string(),
            uploaded_by_name: "Mr. Johan van der Berg".to_string(),
            size: 15_678_234,
            tags: vec!["photos".to_string(), "site inspection".to_string(), "tender".to_string()],
            file_url: "/documents/TND-2025-0076-Site-Photos.zip".to_string(),
            thumbnail_url: None,
            mime_type: "application/zip".to_string(),
            extension: "zip".to_string(),
            related_entity_type: Some("Tender".to_string()),
            related_entity_id: Some("TND-2025-0076".to_string()),
            related_entity_name: Some("Security Services Tender".to_string()),
            folder_id: Some("folder_001".to_string()),
            folder_path: "/Tenders".to_string(),
            versions: vec![],
            is_public: false,
            permissions: Vec::new(),
            created_at: "2025-02-05".to_string(),
            updated_at: "2025-02-05".to_string(),
            last_accessed_at: Some("2025-02-22".to_string()),
            download_count: 23,
            is_archived: false,
            is_locked: false,
            locked_by: None,
        },
        Document {
            id: "doc_010".to_string(),
            name: "Monthly Spend Analysis".to_string(),
            file_name: "Feb-2025-Spend-Analysis.xlsx".to_string(),
            description: Some("Monthly procurement spend analysis for February 2025".to_string()),
            document_type: DocumentType::Excel,
            category: DocumentCategory::Report,
            version: 1,
            uploaded_by: "usr_004".to_string(),
            uploaded_by_name: "Mr. Sipho Dlamini".to_string(),
            size: 1_234_567,
            tags: vec!["report".to_string(), "spend".to_string(), "analysis".to_string(), "monthly".to_string()],
            file_url: "/documents/Feb-2025-Spend-Analysis.xlsx".to_string(),
            thumbnail_url: None,
            mime_type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string(),
            extension: "xlsx".to_string(),
            related_entity_type: None,
            related_entity_id: None,
            related_entity_name: None,
            folder_id: Some("folder_005".to_string()),
            folder_path: "/Reports".to_string(),
            versions: vec![],
            is_public: false,
            permissions: Vec::new(),
            created_at: "2025-02-26".to_string(),
            updated_at: "2025-02-26".to_string(),
            last_accessed_at: Some("2025-02-27".to_string()),
            download_count: 8,
            is_archived: false,
            is_locked: false,
            locked_by: None,
        },
    ];

    store.documents.set(documents);
    store.folders.set(folders);
    store.calculate_stats();
    store.total_count.set(store.documents.get().len());
}

/// Select document by ID
pub fn select_document(store: &DocumentsStore, document_id: &str) {
    store.select_document(document_id);
}

/// Clear document selection
pub fn clear_selection(store: &DocumentsStore) {
    store.clear_selection();
}
