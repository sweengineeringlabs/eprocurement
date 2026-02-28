//! Mobile store

use components::prelude::*;
use super::types::{
    AppFeature, MobileUser, MobileUserFilter, MobileAppConfig, MobileAppStats,
    NotificationPreference, NotificationCategory, NotificationPriority,
    DeviceType, PreviewMode, PreviewScreen,
};

/// Mobile state store
#[derive(Clone)]
pub struct MobileStore {
    // App Configuration
    pub config: Signal<MobileAppConfig>,
    pub config_draft: Signal<MobileAppConfig>,

    // Mobile Users
    pub users: Signal<Vec<MobileUser>>,
    pub selected_user: Signal<Option<MobileUser>>,
    pub user_filter: Signal<MobileUserFilter>,

    // Statistics
    pub stats: Signal<MobileAppStats>,

    // Preview State
    pub preview_mode: Signal<PreviewMode>,
    pub preview_screen: Signal<PreviewScreen>,
    pub preview_dark_mode: Signal<bool>,

    // UI State
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
    pub saving: Signal<bool>,
    pub active_tab: Signal<String>,
    pub config_dirty: Signal<bool>,
}

impl MobileStore {
    pub fn new() -> Self {
        Self {
            config: signal(MobileAppConfig::default()),
            config_draft: signal(MobileAppConfig::default()),
            users: signal(Vec::new()),
            selected_user: signal(None),
            user_filter: signal(MobileUserFilter::default()),
            stats: signal(MobileAppStats::default()),
            preview_mode: signal(PreviewMode::IPhone),
            preview_screen: signal(PreviewScreen::Dashboard),
            preview_dark_mode: signal(false),
            loading: signal(false),
            error: signal(None),
            saving: signal(false),
            active_tab: signal("preview".to_string()),
            config_dirty: signal(false),
        }
    }

    /// Set active tab
    pub fn set_active_tab(&self, tab: &str) {
        self.active_tab.set(tab.to_string());
    }

    /// Set preview mode
    pub fn set_preview_mode(&self, mode: PreviewMode) {
        self.preview_mode.set(mode);
    }

    /// Set preview screen
    pub fn set_preview_screen(&self, screen: PreviewScreen) {
        self.preview_screen.set(screen);
    }

    /// Toggle dark mode preview
    pub fn toggle_preview_dark_mode(&self) {
        let current = self.preview_dark_mode.get();
        self.preview_dark_mode.set(!current);
    }

    /// Update config draft and mark as dirty
    pub fn update_config_draft(&self, config: MobileAppConfig) {
        self.config_draft.set(config);
        self.config_dirty.set(true);
    }

    /// Reset config draft to current config
    pub fn reset_config_draft(&self) {
        self.config_draft.set(self.config.get());
        self.config_dirty.set(false);
    }

    /// Toggle feature in config draft
    pub fn toggle_feature(&self, feature: AppFeature) {
        let mut config = self.config_draft.get();
        if config.features_enabled.contains(&feature) {
            config.features_enabled.retain(|f| *f != feature);
        } else {
            config.features_enabled.push(feature);
        }
        self.config_draft.set(config);
        self.config_dirty.set(true);
    }

    /// Set primary color in config draft
    pub fn set_primary_color(&self, color: String) {
        let mut config = self.config_draft.get();
        config.primary_color = color;
        self.config_draft.set(config);
        self.config_dirty.set(true);
    }

    /// Set secondary color in config draft
    pub fn set_secondary_color(&self, color: String) {
        let mut config = self.config_draft.get();
        config.secondary_color = color;
        self.config_draft.set(config);
        self.config_dirty.set(true);
    }

    /// Set app name in config draft
    pub fn set_app_name(&self, name: String) {
        let mut config = self.config_draft.get();
        config.app_name = name;
        self.config_draft.set(config);
        self.config_dirty.set(true);
    }

    /// Set require biometric in config draft
    pub fn set_require_biometric(&self, require: bool) {
        let mut config = self.config_draft.get();
        config.require_biometric = require;
        self.config_draft.set(config);
        self.config_dirty.set(true);
    }

    /// Set offline mode enabled in config draft
    pub fn set_offline_mode_enabled(&self, enabled: bool) {
        let mut config = self.config_draft.get();
        config.offline_mode_enabled = enabled;
        self.config_draft.set(config);
        self.config_dirty.set(true);
    }

    /// Set session timeout in config draft
    pub fn set_session_timeout(&self, minutes: u32) {
        let mut config = self.config_draft.get();
        config.session_timeout_minutes = minutes;
        self.config_draft.set(config);
        self.config_dirty.set(true);
    }

    // User filter methods
    pub fn set_user_device_filter(&self, device_type: Option<DeviceType>) {
        let mut filter = self.user_filter.get();
        filter.device_type = device_type;
        self.user_filter.set(filter);
    }

    pub fn set_user_active_filter(&self, is_active: Option<bool>) {
        let mut filter = self.user_filter.get();
        filter.is_active = is_active;
        self.user_filter.set(filter);
    }

    pub fn set_user_search(&self, search: Option<String>) {
        let mut filter = self.user_filter.get();
        filter.search_query = search;
        self.user_filter.set(filter);
    }

    pub fn clear_user_filters(&self) {
        self.user_filter.set(MobileUserFilter::default());
    }

    /// Get filtered users
    pub fn get_filtered_users(&self) -> Vec<MobileUser> {
        let users = self.users.get();
        let filter = self.user_filter.get();

        users
            .iter()
            .filter(|u| {
                // Filter by device type
                if let Some(device_type) = filter.device_type {
                    if u.device_type != device_type {
                        return false;
                    }
                }

                // Filter by active status
                if let Some(is_active) = filter.is_active {
                    if u.is_active != is_active {
                        return false;
                    }
                }

                // Filter by push enabled
                if let Some(has_push) = filter.has_push_enabled {
                    let user_has_push = u.push_token.is_some();
                    if user_has_push != has_push {
                        return false;
                    }
                }

                // Filter by app version
                if let Some(ref version) = filter.app_version {
                    if !version.is_empty() && u.app_version != *version {
                        return false;
                    }
                }

                // Filter by search query
                if let Some(ref search) = filter.search_query {
                    if !search.is_empty() {
                        let search_lower = search.to_lowercase();
                        if !u.supplier_name.to_lowercase().contains(&search_lower)
                            && !u.email.to_lowercase().contains(&search_lower)
                            && !u.phone.contains(&search_lower)
                        {
                            return false;
                        }
                    }
                }

                true
            })
            .cloned()
            .collect()
    }

    /// Get users by device type counts
    pub fn get_device_type_counts(&self) -> (u32, u32) {
        let users = self.users.get();
        let ios = users.iter().filter(|u| u.device_type == DeviceType::IOS).count() as u32;
        let android = users.iter().filter(|u| u.device_type == DeviceType::Android).count() as u32;
        (ios, android)
    }

    /// Get active user count
    pub fn get_active_user_count(&self) -> u32 {
        self.users.get().iter().filter(|u| u.is_active).count() as u32
    }
}

/// Load mock data for mobile store
pub fn load_mock_mobile_data(store: &MobileStore) {
    // Load config
    store.config.set(MobileAppConfig::default());
    store.config_draft.set(MobileAppConfig::default());

    // Load stats
    store.stats.set(MobileAppStats::default());

    // Load mock users
    store.users.set(vec![
        MobileUser {
            id: "MOB-001".to_string(),
            supplier_id: "SUP-2025-0001".to_string(),
            supplier_name: "TechSolutions SA (Pty) Ltd".to_string(),
            email: "john.smith@techsolutions.co.za".to_string(),
            phone: "+27 82 123 4567".to_string(),
            device_id: Some("iPhone14-ABCD1234".to_string()),
            device_type: DeviceType::IOS,
            device_model: Some("iPhone 14 Pro".to_string()),
            os_version: Some("iOS 17.3".to_string()),
            app_version: "1.2.0".to_string(),
            push_token: Some("apns-token-abc123".to_string()),
            registered_at: "2024-06-15T10:30:00Z".to_string(),
            last_active_at: "2025-02-27T08:45:00Z".to_string(),
            is_active: true,
            features_enabled: vec![
                AppFeature::TenderNotifications,
                AppFeature::BidStatusAlerts,
                AppFeature::ContractReminders,
                AppFeature::DocumentScanning,
                AppFeature::BiometricAuth,
            ],
            notification_preferences: NotificationPreference::default(),
        },
        MobileUser {
            id: "MOB-002".to_string(),
            supplier_id: "SUP-2025-0002".to_string(),
            supplier_name: "GreenBuild Construction".to_string(),
            email: "thabo.ndlovu@greenbuild.co.za".to_string(),
            phone: "+27 83 987 6543".to_string(),
            device_id: Some("Samsung-S24-XYZ789".to_string()),
            device_type: DeviceType::Android,
            device_model: Some("Samsung Galaxy S24".to_string()),
            os_version: Some("Android 14".to_string()),
            app_version: "1.2.0".to_string(),
            push_token: Some("fcm-token-xyz789".to_string()),
            registered_at: "2024-08-20T14:15:00Z".to_string(),
            last_active_at: "2025-02-26T16:30:00Z".to_string(),
            is_active: true,
            features_enabled: vec![
                AppFeature::TenderNotifications,
                AppFeature::BidStatusAlerts,
                AppFeature::DeliveryConfirmation,
                AppFeature::QrCodeScanning,
            ],
            notification_preferences: NotificationPreference {
                push_enabled: true,
                email_enabled: true,
                sms_enabled: true,
                quiet_hours_start: Some("20:00".to_string()),
                quiet_hours_end: Some("06:00".to_string()),
                categories: vec![
                    NotificationCategory::Tenders,
                    NotificationCategory::Bids,
                    NotificationCategory::Contracts,
                ],
                min_priority: NotificationPriority::Low,
                daily_digest: false,
                digest_time: None,
            },
        },
        MobileUser {
            id: "MOB-003".to_string(),
            supplier_id: "SUP-2025-0003".to_string(),
            supplier_name: "SecureGuard Services".to_string(),
            email: "sarah.mokoena@secureguard.co.za".to_string(),
            phone: "+27 84 555 1234".to_string(),
            device_id: Some("Pixel-8-DEF456".to_string()),
            device_type: DeviceType::Android,
            device_model: Some("Google Pixel 8".to_string()),
            os_version: Some("Android 14".to_string()),
            app_version: "1.1.5".to_string(),
            push_token: Some("fcm-token-def456".to_string()),
            registered_at: "2024-09-10T09:00:00Z".to_string(),
            last_active_at: "2025-02-25T11:20:00Z".to_string(),
            is_active: true,
            features_enabled: vec![
                AppFeature::TenderNotifications,
                AppFeature::BidStatusAlerts,
                AppFeature::BriefingCheckIn,
            ],
            notification_preferences: NotificationPreference::default(),
        },
        MobileUser {
            id: "MOB-004".to_string(),
            supplier_id: "SUP-2025-0004".to_string(),
            supplier_name: "CleanPro Facilities".to_string(),
            email: "nomsa.dlamini@cleanpro.co.za".to_string(),
            phone: "+27 81 777 8888".to_string(),
            device_id: Some("iPhone13-GHI789".to_string()),
            device_type: DeviceType::IOS,
            device_model: Some("iPhone 13".to_string()),
            os_version: Some("iOS 17.2".to_string()),
            app_version: "1.2.0".to_string(),
            push_token: Some("apns-token-ghi789".to_string()),
            registered_at: "2024-07-25T11:45:00Z".to_string(),
            last_active_at: "2025-02-27T07:00:00Z".to_string(),
            is_active: true,
            features_enabled: vec![
                AppFeature::TenderNotifications,
                AppFeature::ContractReminders,
                AppFeature::MobileInvoicing,
                AppFeature::DeliveryConfirmation,
            ],
            notification_preferences: NotificationPreference {
                push_enabled: true,
                email_enabled: false,
                sms_enabled: false,
                quiet_hours_start: None,
                quiet_hours_end: None,
                categories: NotificationCategory::all(),
                min_priority: NotificationPriority::High,
                daily_digest: true,
                digest_time: Some("07:00".to_string()),
            },
        },
        MobileUser {
            id: "MOB-005".to_string(),
            supplier_id: "SUP-2025-0005".to_string(),
            supplier_name: "IT Networks Africa".to_string(),
            email: "peter.van.wyk@itnetworks.co.za".to_string(),
            phone: "+27 82 111 2222".to_string(),
            device_id: None,
            device_type: DeviceType::Android,
            device_model: None,
            os_version: None,
            app_version: "1.0.0".to_string(),
            push_token: None,
            registered_at: "2024-11-01T08:30:00Z".to_string(),
            last_active_at: "2024-12-15T10:00:00Z".to_string(),
            is_active: false,
            features_enabled: vec![AppFeature::TenderNotifications],
            notification_preferences: NotificationPreference::default(),
        },
        MobileUser {
            id: "MOB-006".to_string(),
            supplier_id: "SUP-2025-0006".to_string(),
            supplier_name: "Agri-Fresh Suppliers".to_string(),
            email: "themba.khumalo@agrifresh.co.za".to_string(),
            phone: "+27 83 333 4444".to_string(),
            device_id: Some("iPad-Pro-JKL012".to_string()),
            device_type: DeviceType::IOS,
            device_model: Some("iPad Pro 12.9\"".to_string()),
            os_version: Some("iPadOS 17.3".to_string()),
            app_version: "1.2.0".to_string(),
            push_token: Some("apns-token-jkl012".to_string()),
            registered_at: "2024-10-05T13:20:00Z".to_string(),
            last_active_at: "2025-02-26T09:15:00Z".to_string(),
            is_active: true,
            features_enabled: AppFeature::all(),
            notification_preferences: NotificationPreference::default(),
        },
    ]);
}

/// Get mock user by ID
pub fn get_mock_user(id: &str) -> Option<MobileUser> {
    let users = vec![
        MobileUser {
            id: "MOB-001".to_string(),
            supplier_id: "SUP-2025-0001".to_string(),
            supplier_name: "TechSolutions SA (Pty) Ltd".to_string(),
            email: "john.smith@techsolutions.co.za".to_string(),
            phone: "+27 82 123 4567".to_string(),
            device_id: Some("iPhone14-ABCD1234".to_string()),
            device_type: DeviceType::IOS,
            device_model: Some("iPhone 14 Pro".to_string()),
            os_version: Some("iOS 17.3".to_string()),
            app_version: "1.2.0".to_string(),
            push_token: Some("apns-token-abc123".to_string()),
            registered_at: "2024-06-15T10:30:00Z".to_string(),
            last_active_at: "2025-02-27T08:45:00Z".to_string(),
            is_active: true,
            features_enabled: vec![
                AppFeature::TenderNotifications,
                AppFeature::BidStatusAlerts,
                AppFeature::ContractReminders,
                AppFeature::DocumentScanning,
                AppFeature::BiometricAuth,
            ],
            notification_preferences: NotificationPreference::default(),
        },
    ];

    users.into_iter().find(|u| u.id == id)
}
