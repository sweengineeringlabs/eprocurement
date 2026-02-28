//! Tender publication page for e-Tender portal

use components::prelude::*;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, panel_with_footer,
    status_badge, StatusType,
    notice_bar, NoticeType,
    tag, TagType,
};
use crate::shared::forms::{
    text_input, date_picker, textarea, checkbox, form_group,
};
use crate::util::format::format_currency;
use super::store::TendersStore;
use super::types::{TenderStatus, TenderType};
use super::service;

/// Tender publication page
#[component]
pub fn tender_publication(tender_id: String) -> View {
    let store = use_context::<TendersStore>();

    // Form fields
    let publish_date = signal(String::new());
    let closing_date = signal(String::new());
    let briefing_date = signal(String::new());
    let briefing_required = signal(false);
    let briefing_mandatory = signal(false);
    let portal_notes = signal(String::new());

    // Portal options
    let publish_etender = signal(true);
    let publish_internal = signal(true);
    let notify_suppliers = signal(true);

    // State
    let publishing = signal(false);
    let error = signal::<Option<String>>(None);
    let success = signal::<Option<String>>(None);

    // Load tender on mount
    effect({
        let store = store.clone();
        let tender_id = tender_id.clone();
        move || {
            let store = store.clone();
            let tender_id = tender_id.clone();
            spawn(async move {
                service::get_tender(&store, &tender_id).await;
            });
        }
    });

    let tender = store.selected.clone();

    // Check if tender can be published
    let can_publish = tender.get().map_or(false, |t| {
        t.status == TenderStatus::Approved
    });

    let is_published = tender.get().map_or(false, |t| {
        t.status == TenderStatus::Published || t.status == TenderStatus::Open
    });

    // Handle publish
    let handle_publish: Callback<()> = Callback::new({
        let store = store.clone();
        let tender_id = tender_id.clone();
        let publish_date = publish_date.clone();
        let closing_date = closing_date.clone();
        let briefing_date = briefing_date.clone();
        let briefing_required = briefing_required.clone();
        let publishing = publishing.clone();
        let error = error.clone();
        let success = success.clone();

        move |_| {
            let store = store.clone();
            let tender_id = tender_id.clone();
            let publish_date_val = publish_date.get();
            let closing_date_val = closing_date.get();
            let briefing_date_val = if briefing_required.get() {
                Some(briefing_date.get())
            } else {
                None
            };
            let publishing = publishing.clone();
            let error = error.clone();
            let success = success.clone();

            // Validation
            if publish_date_val.is_empty() {
                error.set(Some("Please select a publication date".to_string()));
                return;
            }
            if closing_date_val.is_empty() {
                error.set(Some("Please select a closing date".to_string()));
                return;
            }

            spawn(async move {
                publishing.set(true);
                error.set(None);
                success.set(None);

                match service::publish_tender(
                    &store,
                    &tender_id,
                    &publish_date_val,
                    &closing_date_val,
                    briefing_date_val.as_deref(),
                ).await {
                    Ok(portal_ref) => {
                        success.set(Some(format!(
                            "Tender successfully published to e-Tender portal. Reference: {}",
                            portal_ref
                        )));
                    }
                    Err(e) => {
                        error.set(Some(e));
                    }
                }
                publishing.set(false);
            });
        }
    });

    // Icons
    let icon_external = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="14" height="14"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/></svg>"#;

    view! {
        style {
            r#"
            .tender-publication { display: flex; flex-direction: column; gap: var(--space-4); }
            .tender-summary {
                display: grid;
                grid-template-columns: repeat(4, 1fr);
                gap: 16px;
                margin-bottom: 20px;
            }
            .summary-item label {
                font-size: 11px;
                color: var(--text-muted);
                display: block;
                margin-bottom: 4px;
            }
            .summary-item span {
                font-size: 14px;
                color: var(--text);
                font-weight: 500;
            }
            .portal-options {
                display: grid;
                grid-template-columns: repeat(2, 1fr);
                gap: 16px;
                margin-bottom: 20px;
            }
            .portal-option {
                display: flex;
                align-items: flex-start;
                gap: 12px;
                padding: 16px;
                background: var(--bg);
                border-radius: var(--radius);
                border: 1px solid var(--border);
            }
            .portal-option.selected {
                border-color: var(--blue);
                background: var(--blue-light);
            }
            .portal-option-icon {
                width: 40px;
                height: 40px;
                background: var(--surface);
                border-radius: var(--radius);
                display: flex;
                align-items: center;
                justify-content: center;
                color: var(--blue);
            }
            .portal-option-content h4 {
                font-size: 14px;
                font-weight: 600;
                margin-bottom: 4px;
            }
            .portal-option-content p {
                font-size: 12px;
                color: var(--text-muted);
            }
            .published-info {
                padding: 20px;
                background: var(--green-light);
                border-radius: var(--radius);
                margin-bottom: 20px;
            }
            .published-info h3 {
                color: var(--green);
                font-size: 16px;
                margin-bottom: 12px;
            }
            .published-info .portal-link {
                display: inline-flex;
                align-items: center;
                gap: 6px;
                color: var(--blue);
                text-decoration: none;
                font-size: 13px;
            }
            .published-info .portal-link:hover {
                text-decoration: underline;
            }
            .dates-grid {
                display: grid;
                grid-template-columns: repeat(3, 1fr);
                gap: 16px;
            }
            .briefing-options {
                margin-top: 16px;
                padding: 16px;
                background: var(--bg);
                border-radius: var(--radius);
            }
            @media (max-width: 768px) {
                .tender-summary, .portal-options, .dates-grid {
                    grid-template-columns: 1fr;
                }
            }
            "#
        }

        <div class="tender-publication" data-testid="tender-publication">
            {page_header(
                "Publish Tender".to_string(),
                Some("Publish tender to e-Tender portal for bidding".to_string()),
                vec![
                    view! { <a href={format!("#/tenders/{}", tender_id)} class="btn btn-secondary">"Back to Tender"</a> },
                ]
            )}

            if let Some(err) = error.get() {
                {notice_bar(err, NoticeType::Error, None)}
            }

            if let Some(msg) = success.get() {
                {notice_bar(msg, NoticeType::Success, None)}
            }

            // Tender summary
            if let Some(t) = tender.get() {
                {panel(
                    "Tender Summary".to_string(),
                    vec![
                        match t.tender_type {
                            TenderType::Rfq => tag("RFQ".to_string(), TagType::Rfq),
                            TenderType::Rfp => tag("RFP".to_string(), TagType::Rfp),
                            TenderType::Rft => tag("RFT".to_string(), TagType::Tender),
                        },
                        match t.status {
                            TenderStatus::Approved => status_badge(StatusType::Approved),
                            TenderStatus::Published => status_badge(StatusType::Published),
                            TenderStatus::Open => status_badge(StatusType::Open),
                            _ => status_badge(StatusType::Draft),
                        },
                    ],
                    vec![
                        view! {
                            <div class="tender-summary">
                                <div class="summary-item">
                                    <label>"Reference"</label>
                                    <span>{t.reference_number.clone()}</span>
                                </div>
                                <div class="summary-item">
                                    <label>"Title"</label>
                                    <span>{t.title.clone()}</span>
                                </div>
                                <div class="summary-item">
                                    <label>"Estimated Value"</label>
                                    <span>{format_currency(t.estimated_value)}</span>
                                </div>
                                <div class="summary-item">
                                    <label>"Category"</label>
                                    <span>{t.category.clone()}</span>
                                </div>
                            </div>
                        }
                    ]
                )}

                // If already published, show portal info
                if is_published {
                    <div class="published-info">
                        <h3>"Tender Published"</h3>
                        <div class="tender-summary">
                            <div class="summary-item">
                                <label>"Portal Reference"</label>
                                <span>{t.portal_reference.clone().unwrap_or_default()}</span>
                            </div>
                            <div class="summary-item">
                                <label>"Publication Date"</label>
                                <span>{t.publish_date.clone().unwrap_or_default()}</span>
                            </div>
                            <div class="summary-item">
                                <label>"Closing Date"</label>
                                <span>{t.closing_date.clone().unwrap_or_default()}</span>
                            </div>
                            <div class="summary-item">
                                <label>"Portal Link"</label>
                                <a href={t.portal_url.clone().unwrap_or_default()} target="_blank" class="portal-link">
                                    "View on e-Tender Portal"
                                    <span inner_html={icon_external}></span>
                                </a>
                            </div>
                        </div>
                    </div>
                }

                // If can publish, show publication form
                if can_publish {
                    {panel_with_footer(
                        "Publication Settings".to_string(),
                        vec![],
                        vec![
                            // Portal options
                            view! {
                                <div class="portal-options">
                                    <div class={if publish_etender.get() { "portal-option selected" } else { "portal-option" }}>
                                        <div class="portal-option-icon">
                                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="20" height="20">
                                                <circle cx="12" cy="12" r="10"/>
                                                <path d="M2 12h20"/>
                                                <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/>
                                            </svg>
                                        </div>
                                        <div class="portal-option-content">
                                            <h4>"e-Tender Portal"</h4>
                                            <p>"Publish to the national e-Tender portal (etenders.gov.za)"</p>
                                            {checkbox("Publish to e-Tender".to_string(), publish_etender.clone(), false)}
                                        </div>
                                    </div>
                                    <div class={if publish_internal.get() { "portal-option selected" } else { "portal-option" }}>
                                        <div class="portal-option-icon">
                                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="20" height="20">
                                                <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>
                                                <polyline points="9 22 9 12 15 12 15 22"/>
                                            </svg>
                                        </div>
                                        <div class="portal-option-content">
                                            <h4>"Internal Portal"</h4>
                                            <p>"Also publish to internal supplier portal"</p>
                                            {checkbox("Publish internally".to_string(), publish_internal.clone(), false)}
                                        </div>
                                    </div>
                                </div>
                            },

                            // Publication dates
                            view! {
                                {form_group(
                                    Some("Publication Dates".to_string()),
                                    3,
                                    vec![
                                        date_picker(
                                            "Publication Date".to_string(),
                                            publish_date.clone(),
                                            true,
                                            false,
                                            Some("2025-01-01".to_string()),
                                            None,
                                            None,
                                        ),
                                        date_picker(
                                            "Closing Date".to_string(),
                                            closing_date.clone(),
                                            true,
                                            false,
                                            Some("2025-01-01".to_string()),
                                            None,
                                            None,
                                        ),
                                        view! {
                                            <div class="form-group">
                                                <label>"Minimum Bidding Period"</label>
                                                <span style="font-size: 13px; color: var(--text-muted);">
                                                    "21 days (per Treasury Regulations)"
                                                </span>
                                            </div>
                                        },
                                    ]
                                )}
                            },

                            // Briefing session
                            view! {
                                <div class="briefing-options">
                                    {checkbox("Include Briefing Session".to_string(), briefing_required.clone(), false)}
                                    if briefing_required.get() {
                                        <div style="margin-top: 16px;">
                                            {form_group(
                                                None,
                                                2,
                                                vec![
                                                    date_picker(
                                                        "Briefing Date".to_string(),
                                                        briefing_date.clone(),
                                                        true,
                                                        false,
                                                        None,
                                                        None,
                                                        None,
                                                    ),
                                                    checkbox("Attendance Mandatory".to_string(), briefing_mandatory.clone(), false),
                                                ]
                                            )}
                                        </div>
                                    }
                                </div>
                            },

                            // Notifications
                            view! {
                                {form_group(
                                    Some("Notifications".to_string()),
                                    1,
                                    vec![
                                        checkbox("Notify registered suppliers in this category".to_string(), notify_suppliers.clone(), false),
                                    ]
                                )}
                            },

                            // Portal notes
                            view! {
                                {form_group(
                                    Some("Portal Notes".to_string()),
                                    1,
                                    vec![
                                        textarea(
                                            "Additional Information".to_string(),
                                            portal_notes.clone(),
                                            Some("Any additional notes to display on the portal".to_string()),
                                            false,
                                            false,
                                            Some(3),
                                            None,
                                            None,
                                        ),
                                    ]
                                )}
                            },

                            // Warning notice
                            {notice_bar(
                                "Once published, the tender will be visible to all suppliers. Ensure all documents are finalized before publishing.".to_string(),
                                NoticeType::Warning,
                                None,
                            )}
                        ],
                        vec![
                            view! { <a href="#/tenders" class="btn btn-secondary">"Cancel"</a> },
                            view! {
                                <button class="btn btn-primary" on:click={handle_publish} disabled={publishing.get()}>
                                    if publishing.get() { "Publishing..." } else { "Publish to Portal" }
                                </button>
                            },
                        ]
                    )}
                }

                // If not approved, show warning
                if !can_publish && !is_published {
                    {notice_bar(
                        "This tender must be approved before it can be published. Current status: ".to_string() + t.status.label(),
                        NoticeType::Warning,
                        None,
                    )}
                }
            }
        </div>
    }
}
