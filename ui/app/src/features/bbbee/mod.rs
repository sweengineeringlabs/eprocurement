//! B-BBEE feature module
//!
//! Broad-Based Black Economic Empowerment compliance tracking and reporting
//! per South African B-BBEE Act and Generic Codes of Good Practice.
//!
//! This module provides:
//! - Spend target tracking (overall B-BBEE, Black Owned, BWO, EME/QSE, designated groups)
//! - B-BBEE level distribution analysis
//! - Supplier classification management
//! - Compliance metrics and scorecard tracking
//! - DTI reporting support

pub mod types;
pub mod store;
pub mod service;
pub mod bbbee_goals;
