//! Mobile domain types

use serde::{Deserialize, Serialize};

/// Mobile app feature that can be enabled/disabled
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum AppFeature {
    /// Push notifications for tender updates
    TenderNotifications,
    /// Push notifications for bid status changes
    BidStatusAlerts,
    /// Push notifications for contract milestones
    ContractReminders,
    /// Offline bid preparation
    OfflineBidPrep,
    /// Document scanning and upload
    DocumentScanning,
    /// Biometric authentication
    BiometricAuth,
    /// Location-based briefing check-in
    BriefingCheckIn,
    /// Real-time auction participation
    LiveAuction,
    /// Digital signature for contracts
    DigitalSignature,
    /// Invoice submission via mobile
    MobileInvoicing,
    /// Delivery confirmation with photos
    DeliveryConfirmation,
    /// QR code scanning for goods receipt
    QrCodeScanning,
}

impl AppFeature {
    pub fn label(&self) -> &'static str {
        match self {
            AppFeature::TenderNotifications => "Tender Notifications",
            AppFeature::BidStatusAlerts => "Bid Status Alerts",
            AppFeature::ContractReminders => "Contract Reminders",
            AppFeature::OfflineBidPrep => "Offline Bid Preparation",
            AppFeature::DocumentScanning => "Document Scanning",
            AppFeature::BiometricAuth => "Biometric Authentication",
            AppFeature::BriefingCheckIn => "Briefing Check-In",
            AppFeature::LiveAuction => "Live Auction",
            AppFeature::DigitalSignature => "Digital Signature",
            AppFeature::MobileInvoicing => "Mobile Invoicing",
            AppFeature::DeliveryConfirmation => "Delivery Confirmation",
            AppFeature::QrCodeScanning => "QR Code Scanning",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            AppFeature::TenderNotifications => "Receive push notifications when new tenders matching your profile are published",
            AppFeature::BidStatusAlerts => "Get instant alerts when your bid status changes during evaluation",
            AppFeature::ContractReminders => "Reminders for upcoming contract milestones, renewals, and deliverables",
            AppFeature::OfflineBidPrep => "Prepare bid documents offline and sync when connected",
            AppFeature::DocumentScanning => "Scan and upload documents directly from your device camera",
            AppFeature::BiometricAuth => "Use fingerprint or face recognition for secure authentication",
            AppFeature::BriefingCheckIn => "Check in to mandatory briefing sessions using GPS location",
            AppFeature::LiveAuction => "Participate in reverse auctions in real-time from your mobile device",
            AppFeature::DigitalSignature => "Sign contracts electronically using your mobile device",
            AppFeature::MobileInvoicing => "Submit and track invoices directly from the mobile app",
            AppFeature::DeliveryConfirmation => "Confirm deliveries with photo evidence and timestamps",
            AppFeature::QrCodeScanning => "Scan QR codes on purchase orders for quick goods receipt",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            AppFeature::TenderNotifications => "bell",
            AppFeature::BidStatusAlerts => "activity",
            AppFeature::ContractReminders => "calendar",
            AppFeature::OfflineBidPrep => "cloud-off",
            AppFeature::DocumentScanning => "camera",
            AppFeature::BiometricAuth => "fingerprint",
            AppFeature::BriefingCheckIn => "map-pin",
            AppFeature::LiveAuction => "trending-down",
            AppFeature::DigitalSignature => "edit-3",
            AppFeature::MobileInvoicing => "file-text",
            AppFeature::DeliveryConfirmation => "check-circle",
            AppFeature::QrCodeScanning => "maximize",
        }
    }

    pub fn category(&self) -> &'static str {
        match self {
            AppFeature::TenderNotifications
            | AppFeature::BidStatusAlerts
            | AppFeature::ContractReminders => "Notifications",
            AppFeature::OfflineBidPrep
            | AppFeature::DocumentScanning => "Bid Management",
            AppFeature::BiometricAuth
            | AppFeature::DigitalSignature => "Security",
            AppFeature::BriefingCheckIn
            | AppFeature::LiveAuction => "Events",
            AppFeature::MobileInvoicing
            | AppFeature::DeliveryConfirmation
            | AppFeature::QrCodeScanning => "Operations",
        }
    }

    /// Get all features
    pub fn all() -> Vec<AppFeature> {
        vec![
            AppFeature::TenderNotifications,
            AppFeature::BidStatusAlerts,
            AppFeature::ContractReminders,
            AppFeature::OfflineBidPrep,
            AppFeature::DocumentScanning,
            AppFeature::BiometricAuth,
            AppFeature::BriefingCheckIn,
            AppFeature::LiveAuction,
            AppFeature::DigitalSignature,
            AppFeature::MobileInvoicing,
            AppFeature::DeliveryConfirmation,
            AppFeature::QrCodeScanning,
        ]
    }
}

/// Mobile user registered for the app
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MobileUser {
    pub id: String,
    pub supplier_id: String,
    pub supplier_name: String,
    pub email: String,
    pub phone: String,
    pub device_id: Option<String>,
    pub device_type: DeviceType,
    pub device_model: Option<String>,
    pub os_version: Option<String>,
    pub app_version: String,
    pub push_token: Option<String>,
    pub registered_at: String,
    pub last_active_at: String,
    pub is_active: bool,
    pub features_enabled: Vec<AppFeature>,
    pub notification_preferences: NotificationPreference,
}

impl Default for MobileUser {
    fn default() -> Self {
        Self {
            id: String::new(),
            supplier_id: String::new(),
            supplier_name: String::new(),
            email: String::new(),
            phone: String::new(),
            device_id: None,
            device_type: DeviceType::Android,
            device_model: None,
            os_version: None,
            app_version: "1.0.0".to_string(),
            push_token: None,
            registered_at: String::new(),
            last_active_at: String::new(),
            is_active: true,
            features_enabled: vec![
                AppFeature::TenderNotifications,
                AppFeature::BidStatusAlerts,
                AppFeature::ContractReminders,
            ],
            notification_preferences: NotificationPreference::default(),
        }
    }
}

/// Device type for mobile app
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum DeviceType {
    /// iOS device (iPhone/iPad)
    IOS,
    /// Android device
    Android,
    /// Unknown device type
    Unknown,
}

impl DeviceType {
    pub fn label(&self) -> &'static str {
        match self {
            DeviceType::IOS => "iOS",
            DeviceType::Android => "Android",
            DeviceType::Unknown => "Unknown",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            DeviceType::IOS => "smartphone",
            DeviceType::Android => "smartphone",
            DeviceType::Unknown => "help-circle",
        }
    }
}

/// Notification preference settings
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationPreference {
    /// Enable push notifications
    pub push_enabled: bool,
    /// Enable email notifications
    pub email_enabled: bool,
    /// Enable SMS notifications
    pub sms_enabled: bool,
    /// Quiet hours start (24h format, e.g., "22:00")
    pub quiet_hours_start: Option<String>,
    /// Quiet hours end (24h format, e.g., "07:00")
    pub quiet_hours_end: Option<String>,
    /// Categories to receive notifications for
    pub categories: Vec<NotificationCategory>,
    /// Minimum priority level for push notifications
    pub min_priority: NotificationPriority,
    /// Receive daily digest instead of individual notifications
    pub daily_digest: bool,
    /// Preferred digest time (24h format, e.g., "08:00")
    pub digest_time: Option<String>,
}

impl Default for NotificationPreference {
    fn default() -> Self {
        Self {
            push_enabled: true,
            email_enabled: true,
            sms_enabled: false,
            quiet_hours_start: Some("22:00".to_string()),
            quiet_hours_end: Some("07:00".to_string()),
            categories: vec![
                NotificationCategory::Tenders,
                NotificationCategory::Bids,
                NotificationCategory::Contracts,
                NotificationCategory::Payments,
            ],
            min_priority: NotificationPriority::Medium,
            daily_digest: false,
            digest_time: Some("08:00".to_string()),
        }
    }
}

/// Notification category
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum NotificationCategory {
    /// New tender opportunities
    Tenders,
    /// Bid submission and evaluation updates
    Bids,
    /// Contract lifecycle events
    Contracts,
    /// Payment and invoice updates
    Payments,
    /// Document requests and uploads
    Documents,
    /// System announcements
    System,
    /// Auction events
    Auctions,
}

impl NotificationCategory {
    pub fn label(&self) -> &'static str {
        match self {
            NotificationCategory::Tenders => "Tenders",
            NotificationCategory::Bids => "Bids",
            NotificationCategory::Contracts => "Contracts",
            NotificationCategory::Payments => "Payments",
            NotificationCategory::Documents => "Documents",
            NotificationCategory::System => "System",
            NotificationCategory::Auctions => "Auctions",
        }
    }

    pub fn all() -> Vec<NotificationCategory> {
        vec![
            NotificationCategory::Tenders,
            NotificationCategory::Bids,
            NotificationCategory::Contracts,
            NotificationCategory::Payments,
            NotificationCategory::Documents,
            NotificationCategory::System,
            NotificationCategory::Auctions,
        ]
    }
}

/// Notification priority level
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum NotificationPriority {
    /// Low priority - informational
    Low,
    /// Medium priority - requires attention
    Medium,
    /// High priority - urgent action needed
    High,
    /// Critical - immediate action required
    Critical,
}

impl NotificationPriority {
    pub fn label(&self) -> &'static str {
        match self {
            NotificationPriority::Low => "Low",
            NotificationPriority::Medium => "Medium",
            NotificationPriority::High => "High",
            NotificationPriority::Critical => "Critical",
        }
    }

    pub fn all() -> Vec<NotificationPriority> {
        vec![
            NotificationPriority::Low,
            NotificationPriority::Medium,
            NotificationPriority::High,
            NotificationPriority::Critical,
        ]
    }
}

/// Mobile app configuration for the organization
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MobileAppConfig {
    pub id: String,
    pub organization_name: String,
    pub app_name: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub logo_url: Option<String>,
    pub splash_screen_url: Option<String>,
    pub features_enabled: Vec<AppFeature>,
    pub require_biometric: bool,
    pub session_timeout_minutes: u32,
    pub offline_mode_enabled: bool,
    pub max_offline_days: u32,
    pub min_app_version: String,
    pub app_store_url: Option<String>,
    pub play_store_url: Option<String>,
    pub support_email: String,
    pub support_phone: String,
    pub terms_url: Option<String>,
    pub privacy_url: Option<String>,
    pub updated_at: String,
}

impl Default for MobileAppConfig {
    fn default() -> Self {
        Self {
            id: "config-001".to_string(),
            organization_name: "Government Department".to_string(),
            app_name: "eProcurement Supplier".to_string(),
            primary_color: "#1e3a5f".to_string(),
            secondary_color: "#2563eb".to_string(),
            logo_url: None,
            splash_screen_url: None,
            features_enabled: AppFeature::all(),
            require_biometric: false,
            session_timeout_minutes: 30,
            offline_mode_enabled: true,
            max_offline_days: 7,
            min_app_version: "1.0.0".to_string(),
            app_store_url: None,
            play_store_url: None,
            support_email: "support@eprocurement.gov.za".to_string(),
            support_phone: "+27 12 000 0000".to_string(),
            terms_url: None,
            privacy_url: None,
            updated_at: String::new(),
        }
    }
}

/// Mobile app usage statistics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MobileAppStats {
    pub total_users: u32,
    pub active_users_30d: u32,
    pub ios_users: u32,
    pub android_users: u32,
    pub push_enabled_users: u32,
    pub avg_session_duration_mins: f64,
    pub bids_submitted_mobile: u32,
    pub documents_scanned: u32,
    pub briefing_checkins: u32,
    pub auction_participations: u32,
    pub invoices_submitted: u32,
    pub deliveries_confirmed: u32,
}

impl Default for MobileAppStats {
    fn default() -> Self {
        Self {
            total_users: 856,
            active_users_30d: 423,
            ios_users: 312,
            android_users: 544,
            push_enabled_users: 678,
            avg_session_duration_mins: 8.5,
            bids_submitted_mobile: 156,
            documents_scanned: 892,
            briefing_checkins: 234,
            auction_participations: 45,
            invoices_submitted: 312,
            deliveries_confirmed: 178,
        }
    }
}

/// Filter criteria for mobile users
#[derive(Clone, Debug, Default)]
pub struct MobileUserFilter {
    pub device_type: Option<DeviceType>,
    pub is_active: Option<bool>,
    pub has_push_enabled: Option<bool>,
    pub search_query: Option<String>,
    pub app_version: Option<String>,
}

/// Mobile app preview mode
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum PreviewMode {
    /// iPhone preview
    IPhone,
    /// Android phone preview
    AndroidPhone,
    /// iPad preview
    IPad,
    /// Android tablet preview
    AndroidTablet,
}

impl PreviewMode {
    pub fn label(&self) -> &'static str {
        match self {
            PreviewMode::IPhone => "iPhone",
            PreviewMode::AndroidPhone => "Android Phone",
            PreviewMode::IPad => "iPad",
            PreviewMode::AndroidTablet => "Android Tablet",
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            PreviewMode::IPhone => (375, 812),
            PreviewMode::AndroidPhone => (360, 800),
            PreviewMode::IPad => (768, 1024),
            PreviewMode::AndroidTablet => (800, 1280),
        }
    }

    pub fn all() -> Vec<PreviewMode> {
        vec![
            PreviewMode::IPhone,
            PreviewMode::AndroidPhone,
            PreviewMode::IPad,
            PreviewMode::AndroidTablet,
        ]
    }
}

/// Mobile app screen to preview
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum PreviewScreen {
    /// Login screen
    Login,
    /// Dashboard/Home screen
    Dashboard,
    /// Tender opportunities list
    Opportunities,
    /// Tender detail view
    TenderDetail,
    /// Bid submission form
    BidSubmission,
    /// My submissions list
    MySubmissions,
    /// Contracts list
    Contracts,
    /// Contract detail view
    ContractDetail,
    /// Notifications center
    Notifications,
    /// Settings screen
    Settings,
    /// Profile screen
    Profile,
    /// Document scanner
    DocumentScanner,
}

impl PreviewScreen {
    pub fn label(&self) -> &'static str {
        match self {
            PreviewScreen::Login => "Login",
            PreviewScreen::Dashboard => "Dashboard",
            PreviewScreen::Opportunities => "Opportunities",
            PreviewScreen::TenderDetail => "Tender Detail",
            PreviewScreen::BidSubmission => "Bid Submission",
            PreviewScreen::MySubmissions => "My Submissions",
            PreviewScreen::Contracts => "Contracts",
            PreviewScreen::ContractDetail => "Contract Detail",
            PreviewScreen::Notifications => "Notifications",
            PreviewScreen::Settings => "Settings",
            PreviewScreen::Profile => "Profile",
            PreviewScreen::DocumentScanner => "Document Scanner",
        }
    }

    pub fn all() -> Vec<PreviewScreen> {
        vec![
            PreviewScreen::Login,
            PreviewScreen::Dashboard,
            PreviewScreen::Opportunities,
            PreviewScreen::TenderDetail,
            PreviewScreen::BidSubmission,
            PreviewScreen::MySubmissions,
            PreviewScreen::Contracts,
            PreviewScreen::ContractDetail,
            PreviewScreen::Notifications,
            PreviewScreen::Settings,
            PreviewScreen::Profile,
            PreviewScreen::DocumentScanner,
        ]
    }
}
