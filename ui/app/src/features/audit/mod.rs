//! Audit feature module
//!
//! Provides audit trail tracking and compliance logging for eProcurement system.
//! Tracks all user actions, system changes, and entity modifications for
//! regulatory compliance and governance requirements.

pub mod types;
pub mod store;
pub mod service;
pub mod audit_trail;

// Re-export commonly used items
pub use types::{
    AuditEntry,
    AuditEntityType,
    AuditActionType,
    AuditFilter,
    AuditStats,
    FieldChange,
    ExportFormat,
};
pub use store::AuditStore;
pub use audit_trail::{audit_trail, audit_entry_detail};
