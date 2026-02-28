//! Document domain types

use serde::{Deserialize, Serialize};

/// Document type classification
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum DocumentType {
    /// PDF document
    Pdf,
    /// Word document
    Word,
    /// Excel spreadsheet
    Excel,
    /// Image file
    Image,
    /// Text file
    Text,
    /// Archive/compressed file
    Archive,
    /// Other file type
    Other,
}

impl DocumentType {
    pub fn label(&self) -> &'static str {
        match self {
            DocumentType::Pdf => "PDF",
            DocumentType::Word => "Word",
            DocumentType::Excel => "Excel",
            DocumentType::Image => "Image",
            DocumentType::Text => "Text",
            DocumentType::Archive => "Archive",
            DocumentType::Other => "Other",
        }
    }

    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "pdf" => DocumentType::Pdf,
            "doc" | "docx" | "odt" | "rtf" => DocumentType::Word,
            "xls" | "xlsx" | "ods" | "csv" => DocumentType::Excel,
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "webp" => DocumentType::Image,
            "txt" | "md" | "json" | "xml" => DocumentType::Text,
            "zip" | "rar" | "7z" | "tar" | "gz" => DocumentType::Archive,
            _ => DocumentType::Other,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            DocumentType::Pdf => "file-pdf",
            DocumentType::Word => "file-word",
            DocumentType::Excel => "file-excel",
            DocumentType::Image => "file-image",
            DocumentType::Text => "file-text",
            DocumentType::Archive => "file-archive",
            DocumentType::Other => "file",
        }
    }

    /// Get all document types for filter dropdown
    pub fn all() -> Vec<DocumentType> {
        vec![
            DocumentType::Pdf,
            DocumentType::Word,
            DocumentType::Excel,
            DocumentType::Image,
            DocumentType::Text,
            DocumentType::Archive,
            DocumentType::Other,
        ]
    }
}

impl Default for DocumentType {
    fn default() -> Self {
        DocumentType::Other
    }
}

/// Document category for organizational grouping
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum DocumentCategory {
    /// Tender documents
    Tender,
    /// Contract documents
    Contract,
    /// Bid submissions
    Bid,
    /// Requisition documents
    Requisition,
    /// Supplier documents
    Supplier,
    /// Compliance/regulatory documents
    Compliance,
    /// Financial documents
    Financial,
    /// Reports
    Report,
    /// Templates
    Template,
    /// General/uncategorized
    General,
}

impl DocumentCategory {
    pub fn label(&self) -> &'static str {
        match self {
            DocumentCategory::Tender => "Tender",
            DocumentCategory::Contract => "Contract",
            DocumentCategory::Bid => "Bid",
            DocumentCategory::Requisition => "Requisition",
            DocumentCategory::Supplier => "Supplier",
            DocumentCategory::Compliance => "Compliance",
            DocumentCategory::Financial => "Financial",
            DocumentCategory::Report => "Report",
            DocumentCategory::Template => "Template",
            DocumentCategory::General => "General",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "tender" => DocumentCategory::Tender,
            "contract" => DocumentCategory::Contract,
            "bid" => DocumentCategory::Bid,
            "requisition" => DocumentCategory::Requisition,
            "supplier" => DocumentCategory::Supplier,
            "compliance" => DocumentCategory::Compliance,
            "financial" => DocumentCategory::Financial,
            "report" => DocumentCategory::Report,
            "template" => DocumentCategory::Template,
            "general" | _ => DocumentCategory::General,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            DocumentCategory::Tender => "briefcase",
            DocumentCategory::Contract => "file-signature",
            DocumentCategory::Bid => "file-invoice",
            DocumentCategory::Requisition => "clipboard-list",
            DocumentCategory::Supplier => "building",
            DocumentCategory::Compliance => "shield-check",
            DocumentCategory::Financial => "coins",
            DocumentCategory::Report => "chart-bar",
            DocumentCategory::Template => "file-alt",
            DocumentCategory::General => "folder",
        }
    }

    /// Get all categories for filter dropdown
    pub fn all() -> Vec<DocumentCategory> {
        vec![
            DocumentCategory::Tender,
            DocumentCategory::Contract,
            DocumentCategory::Bid,
            DocumentCategory::Requisition,
            DocumentCategory::Supplier,
            DocumentCategory::Compliance,
            DocumentCategory::Financial,
            DocumentCategory::Report,
            DocumentCategory::Template,
            DocumentCategory::General,
        ]
    }
}

impl Default for DocumentCategory {
    fn default() -> Self {
        DocumentCategory::General
    }
}

/// Document version history entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocumentVersion {
    pub version: u32,
    pub uploaded_by: String,
    pub uploaded_by_name: String,
    pub uploaded_at: String,
    pub size: u64,
    pub file_url: String,
    pub comment: Option<String>,
}

impl Default for DocumentVersion {
    fn default() -> Self {
        Self {
            version: 1,
            uploaded_by: String::new(),
            uploaded_by_name: String::new(),
            uploaded_at: String::new(),
            size: 0,
            file_url: String::new(),
            comment: None,
        }
    }
}

/// Document access permission
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocumentPermission {
    pub user_id: String,
    pub user_name: String,
    pub can_view: bool,
    pub can_edit: bool,
    pub can_delete: bool,
    pub can_share: bool,
}

/// Main Document entity
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub name: String,
    pub file_name: String,
    pub description: Option<String>,
    pub document_type: DocumentType,
    pub category: DocumentCategory,
    pub version: u32,
    pub uploaded_by: String,
    pub uploaded_by_name: String,
    pub size: u64,
    pub tags: Vec<String>,
    pub file_url: String,
    pub thumbnail_url: Option<String>,
    pub mime_type: String,
    pub extension: String,

    // Related entity (optional)
    pub related_entity_type: Option<String>,
    pub related_entity_id: Option<String>,
    pub related_entity_name: Option<String>,

    // Folder path
    pub folder_id: Option<String>,
    pub folder_path: String,

    // Version history
    pub versions: Vec<DocumentVersion>,

    // Permissions
    pub is_public: bool,
    pub permissions: Vec<DocumentPermission>,

    // Metadata
    pub created_at: String,
    pub updated_at: String,
    pub last_accessed_at: Option<String>,
    pub download_count: u32,
    pub is_archived: bool,
    pub is_locked: bool,
    pub locked_by: Option<String>,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            file_name: String::new(),
            description: None,
            document_type: DocumentType::Other,
            category: DocumentCategory::General,
            version: 1,
            uploaded_by: String::new(),
            uploaded_by_name: String::new(),
            size: 0,
            tags: Vec::new(),
            file_url: String::new(),
            thumbnail_url: None,
            mime_type: String::new(),
            extension: String::new(),
            related_entity_type: None,
            related_entity_id: None,
            related_entity_name: None,
            folder_id: None,
            folder_path: "/".to_string(),
            versions: Vec::new(),
            is_public: false,
            permissions: Vec::new(),
            created_at: String::new(),
            updated_at: String::new(),
            last_accessed_at: None,
            download_count: 0,
            is_archived: false,
            is_locked: false,
            locked_by: None,
        }
    }
}

impl Document {
    /// Get formatted file size
    pub fn formatted_size(&self) -> String {
        format_file_size(self.size)
    }

    /// Check if document can be previewed
    pub fn can_preview(&self) -> bool {
        matches!(
            self.document_type,
            DocumentType::Pdf | DocumentType::Image | DocumentType::Text
        )
    }

    /// Get version count
    pub fn version_count(&self) -> usize {
        self.versions.len().max(1)
    }
}

/// Document folder for organization
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocumentFolder {
    pub id: String,
    pub name: String,
    pub path: String,
    pub parent_id: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub document_count: u32,
    pub created_at: String,
    pub created_by: String,
    pub is_system: bool,
}

impl Default for DocumentFolder {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            path: "/".to_string(),
            parent_id: None,
            icon: None,
            color: None,
            document_count: 0,
            created_at: String::new(),
            created_by: String::new(),
            is_system: false,
        }
    }
}

/// Filter criteria for documents
#[derive(Clone, Debug, Default)]
pub struct DocumentFilter {
    pub search_query: Option<String>,
    pub document_type: Option<DocumentType>,
    pub category: Option<DocumentCategory>,
    pub folder_id: Option<String>,
    pub uploaded_by: Option<String>,
    pub tags: Vec<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub include_archived: bool,
    pub related_entity_type: Option<String>,
    pub related_entity_id: Option<String>,
}

impl DocumentFilter {
    pub fn is_empty(&self) -> bool {
        self.search_query.is_none()
            && self.document_type.is_none()
            && self.category.is_none()
            && self.folder_id.is_none()
            && self.uploaded_by.is_none()
            && self.tags.is_empty()
            && self.date_from.is_none()
            && self.date_to.is_none()
            && !self.include_archived
            && self.related_entity_type.is_none()
            && self.related_entity_id.is_none()
    }

    pub fn clear(&mut self) {
        *self = DocumentFilter::default();
    }
}

/// Document library statistics
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DocumentStats {
    pub total_documents: u32,
    pub total_size: u64,
    pub documents_this_month: u32,
    pub folder_count: u32,
    pub by_type: Vec<(DocumentType, u32)>,
    pub by_category: Vec<(DocumentCategory, u32)>,
    pub recent_uploads: Vec<Document>,
}

/// Sort options for documents
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DocumentSortBy {
    NameAsc,
    NameDesc,
    DateAsc,
    DateDesc,
    SizeAsc,
    SizeDesc,
    TypeAsc,
    TypeDesc,
}

impl Default for DocumentSortBy {
    fn default() -> Self {
        DocumentSortBy::DateDesc
    }
}

impl DocumentSortBy {
    pub fn label(&self) -> &'static str {
        match self {
            DocumentSortBy::NameAsc => "Name (A-Z)",
            DocumentSortBy::NameDesc => "Name (Z-A)",
            DocumentSortBy::DateAsc => "Date (Oldest)",
            DocumentSortBy::DateDesc => "Date (Newest)",
            DocumentSortBy::SizeAsc => "Size (Smallest)",
            DocumentSortBy::SizeDesc => "Size (Largest)",
            DocumentSortBy::TypeAsc => "Type (A-Z)",
            DocumentSortBy::TypeDesc => "Type (Z-A)",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "name_asc" => DocumentSortBy::NameAsc,
            "name_desc" => DocumentSortBy::NameDesc,
            "date_asc" => DocumentSortBy::DateAsc,
            "date_desc" => DocumentSortBy::DateDesc,
            "size_asc" => DocumentSortBy::SizeAsc,
            "size_desc" => DocumentSortBy::SizeDesc,
            "type_asc" => DocumentSortBy::TypeAsc,
            "type_desc" => DocumentSortBy::TypeDesc,
            _ => DocumentSortBy::DateDesc,
        }
    }
}

/// Upload request for new document
#[derive(Clone, Debug)]
pub struct DocumentUploadRequest {
    pub name: String,
    pub description: Option<String>,
    pub category: DocumentCategory,
    pub folder_id: Option<String>,
    pub tags: Vec<String>,
    pub related_entity_type: Option<String>,
    pub related_entity_id: Option<String>,
    pub is_public: bool,
}

impl Default for DocumentUploadRequest {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: None,
            category: DocumentCategory::General,
            folder_id: None,
            tags: Vec::new(),
            related_entity_type: None,
            related_entity_id: None,
            is_public: false,
        }
    }
}

/// View mode for document library
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DocumentViewMode {
    Grid,
    List,
    Details,
}

impl Default for DocumentViewMode {
    fn default() -> Self {
        DocumentViewMode::Grid
    }
}

/// Format file size to human readable format
pub fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Get MIME type from extension
pub fn mime_type_from_extension(ext: &str) -> &'static str {
    match ext.to_lowercase().as_str() {
        "pdf" => "application/pdf",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "txt" => "text/plain",
        "csv" => "text/csv",
        "json" => "application/json",
        "xml" => "application/xml",
        "zip" => "application/zip",
        _ => "application/octet-stream",
    }
}
