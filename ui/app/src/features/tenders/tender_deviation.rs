//! Tender deviation request form for single-source/emergency procurement

use components::prelude::*;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, panel_with_footer,
    notice_bar, NoticeType,
    tag, TagType,
};
use crate::shared::forms::{
    text_input, textarea, select, SelectOption,
    currency_input, date_picker, file_upload, UploadedFile,
    form_group, checkbox, radio_group, RadioOption,
};
use crate::util::format::format_currency;
use super::store::TendersStore;
use super::types::{Tender, TenderType, TenderStatus, DeviationType};
use super::service;

/// Deviation request form
#[component]
pub fn tender_deviation() -> View {
    let store = use_context::<TendersStore>();

    // Form fields - Basic info
    let title = signal(String::new());
    let description = signal(String::new());
    let deviation_type = signal("single_source".to_string());
    let estimated_value = signal(0.0);
    let category = signal(String::new());
    let department = signal(String::new());

    // Justification
    let justification = signal(String::new());
    let market_analysis = signal(String::new());
    let supplier_name = signal(String::new());
    let supplier_reason = signal(String::new());

    // Emergency specific
    let is_emergency = signal(false);
    let emergency_date = signal(String::new());
    let emergency_reason = signal(String::new());

    // Approval requirements
    let cfo_approval = signal(false);
    let accounting_officer = signal(false);
    let treasury_approval = signal(false);

    // Supporting documents
    let documents: Signal<Vec<UploadedFile>> = signal(Vec::new());

    // State
    let submitting = signal(false);
    let error = signal::<Option<String>>(None);
    let success = signal::<Option<String>>(None);

    // Deviation type options
    let deviation_options = vec![
        RadioOption {
            value: "single_source".to_string(),
            label: "Single Source - Only one supplier can provide the goods/services".to_string(),
        },
        RadioOption {
            value: "emergency".to_string(),
            label: "Emergency - Urgent procurement due to unforeseen circumstances".to_string(),
        },
        RadioOption {
            value: "sole_supplier".to_string(),
            label: "Sole Supplier - Supplier has exclusive rights (patents, licenses)".to_string(),
        },
        RadioOption {
            value: "strategic".to_string(),
            label: "Strategic Partnership - Existing strategic relationship".to_string(),
        },
    ];

    // Category options
    let category_options = vec![
        SelectOption { value: "".to_string(), label: "Select Category".to_string() },
        SelectOption { value: "it".to_string(), label: "Information Technology".to_string() },
        SelectOption { value: "facilities".to_string(), label: "Facilities Management".to_string() },
        SelectOption { value: "security".to_string(), label: "Security Services".to_string() },
        SelectOption { value: "professional".to_string(), label: "Professional Services".to_string() },
        SelectOption { value: "fleet".to_string(), label: "Fleet Services".to_string() },
        SelectOption { value: "other".to_string(), label: "Other".to_string() },
    ];

    // Department options
    let department_options = vec![
        SelectOption { value: "".to_string(), label: "Select Department".to_string() },
        SelectOption { value: "corporate".to_string(), label: "Corporate Services".to_string() },
        SelectOption { value: "finance".to_string(), label: "Finance".to_string() },
        SelectOption { value: "operations".to_string(), label: "Operations".to_string() },
        SelectOption { value: "hr".to_string(), label: "Human Resources".to_string() },
        SelectOption { value: "it".to_string(), label: "IT".to_string() },
    ];

    // Update emergency flag when deviation type changes
    effect({
        let deviation_type = deviation_type.clone();
        let is_emergency = is_emergency.clone();
        move || {
            is_emergency.set(deviation_type.get() == "emergency");
        }
    });

    // Handle file remove
    let handle_file_remove = Callback::new({
        let documents = documents.clone();
        move |idx: usize| {
            let mut docs = documents.get();
            if idx < docs.len() {
                docs.remove(idx);
                documents.set(docs);
            }
        }
    });

    // Handle submit
    let handle_submit = Callback::<()>::new({
        let store = store.clone();
        let title = title.clone();
        let description = description.clone();
        let deviation_type = deviation_type.clone();
        let estimated_value = estimated_value.clone();
        let category = category.clone();
        let department = department.clone();
        let justification = justification.clone();
        let supplier_name = supplier_name.clone();
        let submitting = submitting.clone();
        let error = error.clone();
        let success = success.clone();

        move |_| {
            let store = store.clone();
            let submitting = submitting.clone();
            let error = error.clone();
            let success = success.clone();

            // Validation
            if title.get().is_empty() {
                error.set(Some("Please enter a title".to_string()));
                return;
            }
            if justification.get().is_empty() {
                error.set(Some("Please provide justification".to_string()));
                return;
            }
            if supplier_name.get().is_empty() {
                error.set(Some("Please enter supplier name".to_string()));
                return;
            }

            // Build tender with deviation
            let dev_type = match deviation_type.get().as_str() {
                "emergency" => Some(DeviationType::Emergency),
                "sole_supplier" => Some(DeviationType::SoleSupplier),
                "strategic" => Some(DeviationType::Strategic),
                _ => Some(DeviationType::SingleSource),
            };

            let tender = Tender {
                title: title.get(),
                description: description.get(),
                tender_type: TenderType::Rfq, // Deviations typically use simplified process
                estimated_value: estimated_value.get(),
                category: category.get(),
                department: department.get(),
                deviation_type: dev_type,
                deviation_justification: Some(justification.get()),
                ..Default::default()
            };

            spawn(async move {
                submitting.set(true);
                error.set(None);
                success.set(None);

                match service::create_tender(&store, tender).await {
                    Ok(t) => {
                        success.set(Some(format!(
                            "Deviation request {} created successfully. It will be routed for approval.",
                            t.reference_number
                        )));
                    }
                    Err(e) => {
                        error.set(Some(e));
                    }
                }
                submitting.set(false);
            });
        }
    });

    // Handle save draft
    let handle_save_draft = Callback::<()>::new({
        let submitting = submitting.clone();
        move |_| {
            // Save as draft logic
            submitting.set(false);
        }
    });

    // Value threshold for approval requirements
    let value = estimated_value.get();
    let requires_cfo = value > 500_000.0;
    let requires_ao = value > 2_000_000.0;
    let requires_treasury = value > 10_000_000.0;

    view! {
        style {
            r#"
            .tender-deviation { display: flex; flex-direction: column; gap: var(--space-4); }
            .deviation-type-info {
                padding: 16px;
                background: var(--bg);
                border-radius: var(--radius);
                margin-bottom: 20px;
                border-left: 4px solid var(--orange);
            }
            .deviation-type-info h4 {
                font-size: 14px;
                font-weight: 600;
                color: var(--orange);
                margin-bottom: 8px;
            }
            .deviation-type-info p {
                font-size: 13px;
                color: var(--text-muted);
            }
            .approval-requirements {
                display: flex;
                flex-direction: column;
                gap: 12px;
                padding: 16px;
                background: var(--bg);
                border-radius: var(--radius);
            }
            .approval-item {
                display: flex;
                align-items: center;
                gap: 12px;
                padding: 12px;
                background: var(--surface);
                border-radius: var(--radius);
            }
            .approval-item.required {
                border-left: 3px solid var(--orange);
            }
            .approval-item.not-required {
                opacity: 0.5;
            }
            .approval-icon {
                width: 32px;
                height: 32px;
                border-radius: 50%;
                display: flex;
                align-items: center;
                justify-content: center;
                font-size: 12px;
                font-weight: 600;
            }
            .approval-icon.pending {
                background: var(--orange-light);
                color: var(--orange);
            }
            .approval-icon.not-needed {
                background: var(--bg);
                color: var(--text-muted);
            }
            .approval-content h4 {
                font-size: 13px;
                font-weight: 600;
                margin-bottom: 2px;
            }
            .approval-content p {
                font-size: 11px;
                color: var(--text-muted);
            }
            .supplier-section {
                padding: 20px;
                background: var(--blue-light);
                border-radius: var(--radius);
                margin-bottom: 20px;
            }
            .supplier-section h4 {
                font-size: 14px;
                font-weight: 600;
                color: var(--blue);
                margin-bottom: 16px;
            }
            .emergency-section {
                padding: 20px;
                background: var(--red-light);
                border-radius: var(--radius);
                margin-bottom: 20px;
            }
            .emergency-section h4 {
                font-size: 14px;
                font-weight: 600;
                color: var(--red);
                margin-bottom: 16px;
            }
            "#
        }

        <div class="tender-deviation" data-testid="tender-deviation">
            {page_header(
                "Deviation Request".to_string(),
                Some("Request deviation from competitive bidding process".to_string()),
                vec![
                    view! { <a href="#/tenders" class="btn btn-secondary">"Cancel"</a> },
                ]
            )}

            if let Some(err) = error.get() {
                {notice_bar(err, NoticeType::Error, None)}
            }

            if let Some(msg) = success.get() {
                {notice_bar(msg, NoticeType::Success, None)}
            }

            // Warning notice
            {notice_bar(
                "Deviations from competitive bidding must be justified in terms of Treasury Regulation 16A6.4 and require appropriate approval based on value.".to_string(),
                NoticeType::Warning,
                None,
            )}

            // Deviation type selection
            {panel(
                "Deviation Type".to_string(),
                vec![tag("Requires Justification".to_string(), TagType::Limited)],
                vec![
                    radio_group(
                        "Select Deviation Type".to_string(),
                        "deviation_type".to_string(),
                        deviation_type.clone(),
                        deviation_options,
                        false,
                        false,
                        None,
                    ),

                    // Type-specific info
                    view! {
                        <div class="deviation-type-info">
                            if deviation_type.get() == "single_source" {
                                <h4>"Single Source Procurement"</h4>
                                <p>"Applicable when only one supplier can provide the specific goods or services required. Must demonstrate that no reasonable alternative exists."</p>
                            }
                            if deviation_type.get() == "emergency" {
                                <h4>"Emergency Procurement"</h4>
                                <p>"Applicable for urgent requirements due to unforeseen circumstances. Must demonstrate immediate need and inability to follow normal process."</p>
                            }
                            if deviation_type.get() == "sole_supplier" {
                                <h4>"Sole Supplier"</h4>
                                <p>"Applicable when supplier has exclusive rights through patents, licenses, or proprietary technology. Must provide evidence of exclusivity."</p>
                            }
                            if deviation_type.get() == "strategic" {
                                <h4>"Strategic Partnership"</h4>
                                <p>"Applicable for existing strategic relationships that provide significant value. Must demonstrate strategic importance and value."</p>
                            }
                        </div>
                    }
                ]
            )}

            // Basic information
            {panel(
                "Procurement Details".to_string(),
                vec![],
                vec![
                    form_group(
                        Some("Basic Information".to_string()),
                        2,
                        vec![
                            text_input(
                                "Title".to_string(),
                                title.clone(),
                                Some("Brief description of procurement".to_string()),
                                true,
                                false,
                                None,
                                None,
                                None,
                            ),
                            currency_input(
                                "Estimated Value".to_string(),
                                estimated_value.clone(),
                                true,
                                false,
                                None,
                                None,
                            ),
                        ]
                    ),
                    form_group(
                        None,
                        1,
                        vec![
                            textarea(
                                "Description".to_string(),
                                description.clone(),
                                Some("Detailed description of goods/services required".to_string()),
                                true,
                                false,
                                Some(4),
                                None,
                                None,
                            ),
                        ]
                    ),
                    form_group(
                        Some("Classification".to_string()),
                        2,
                        vec![
                            select(
                                "Category".to_string(),
                                category.clone(),
                                category_options,
                                None,
                                true,
                                false,
                                None,
                            ),
                            select(
                                "Department".to_string(),
                                department.clone(),
                                department_options,
                                None,
                                true,
                                false,
                                None,
                            ),
                        ]
                    ),
                ]
            )}

            // Supplier information
            {panel(
                "Proposed Supplier".to_string(),
                vec![],
                vec![
                    view! {
                        <div class="supplier-section">
                            <h4>"Supplier Details"</h4>
                            {form_group(
                                None,
                                2,
                                vec![
                                    text_input(
                                        "Supplier Name".to_string(),
                                        supplier_name.clone(),
                                        Some("Name of proposed supplier".to_string()),
                                        true,
                                        false,
                                        None,
                                        None,
                                        None,
                                    ),
                                ]
                            )}
                            {form_group(
                                None,
                                1,
                                vec![
                                    textarea(
                                        "Why This Supplier".to_string(),
                                        supplier_reason.clone(),
                                        Some("Explain why this specific supplier is required".to_string()),
                                        true,
                                        false,
                                        Some(3),
                                        None,
                                        None,
                                    ),
                                ]
                            )}
                        </div>
                    }
                ]
            )}

            // Emergency section (if applicable)
            if is_emergency.get() {
                {panel(
                    "Emergency Details".to_string(),
                    vec![tag("Emergency".to_string(), TagType::Limited)],
                    vec![
                        view! {
                            <div class="emergency-section">
                                <h4>"Emergency Circumstances"</h4>
                                {form_group(
                                    None,
                                    2,
                                    vec![
                                        date_picker(
                                            "Date Emergency Arose".to_string(),
                                            emergency_date.clone(),
                                            true,
                                            false,
                                            None,
                                            None,
                                            None,
                                        ),
                                    ]
                                )}
                                {form_group(
                                    None,
                                    1,
                                    vec![
                                        textarea(
                                            "Emergency Circumstances".to_string(),
                                            emergency_reason.clone(),
                                            Some("Describe the emergency and why normal procurement cannot be followed".to_string()),
                                            true,
                                            false,
                                            Some(4),
                                            None,
                                            None,
                                        ),
                                    ]
                                )}
                            </div>
                        }
                    ]
                )}
            }

            // Justification
            {panel(
                "Justification".to_string(),
                vec![],
                vec![
                    form_group(
                        Some("Deviation Justification".to_string()),
                        1,
                        vec![
                            textarea(
                                "Full Justification".to_string(),
                                justification.clone(),
                                Some("Provide detailed justification for deviating from competitive bidding".to_string()),
                                true,
                                false,
                                Some(6),
                                None,
                                Some("Include: reasons why competitive bidding is not possible, value for money considerations, risk assessment".to_string()),
                            ),
                        ]
                    ),
                    form_group(
                        Some("Market Analysis".to_string()),
                        1,
                        vec![
                            textarea(
                                "Market Research".to_string(),
                                market_analysis.clone(),
                                Some("Describe market research conducted".to_string()),
                                false,
                                false,
                                Some(4),
                                None,
                                Some("What alternatives were considered? Why were they not suitable?".to_string()),
                            ),
                        ]
                    ),
                ]
            )}

            // Supporting documents
            {panel(
                "Supporting Documents".to_string(),
                vec![],
                vec![
                    file_upload(
                        "Upload Supporting Documents".to_string(),
                        documents.clone(),
                        Some(".pdf,.doc,.docx,.xls,.xlsx".to_string()),
                        true,
                        false,
                        Some("Upload quotations, market research, supplier documentation".to_string()),
                        handle_file_remove,
                    ),
                    notice_bar(
                        "Required documents: Supplier quotation, Market research evidence, Supplier B-BBEE certificate, Tax clearance".to_string(),
                        NoticeType::Info,
                        None,
                    ),
                ]
            )}

            // Approval requirements
            {panel(
                "Approval Requirements".to_string(),
                vec![],
                vec![
                    view! {
                        <div class="approval-requirements">
                            <div class={if requires_cfo { "approval-item required" } else { "approval-item not-required" }}>
                                <div class={if requires_cfo { "approval-icon pending" } else { "approval-icon not-needed" }}>
                                    if requires_cfo { "1" } else { "-" }
                                </div>
                                <div class="approval-content">
                                    <h4>"CFO Approval"</h4>
                                    <p>
                                        if requires_cfo {
                                            "Required for values above R500,000"
                                        } else {
                                            "Not required (value below R500,000)"
                                        }
                                    </p>
                                </div>
                            </div>

                            <div class={if requires_ao { "approval-item required" } else { "approval-item not-required" }}>
                                <div class={if requires_ao { "approval-icon pending" } else { "approval-icon not-needed" }}>
                                    if requires_ao { "2" } else { "-" }
                                </div>
                                <div class="approval-content">
                                    <h4>"Accounting Officer Approval"</h4>
                                    <p>
                                        if requires_ao {
                                            "Required for values above R2,000,000"
                                        } else {
                                            "Not required (value below R2,000,000)"
                                        }
                                    </p>
                                </div>
                            </div>

                            <div class={if requires_treasury { "approval-item required" } else { "approval-item not-required" }}>
                                <div class={if requires_treasury { "approval-icon pending" } else { "approval-icon not-needed" }}>
                                    if requires_treasury { "3" } else { "-" }
                                </div>
                                <div class="approval-content">
                                    <h4>"National Treasury Approval"</h4>
                                    <p>
                                        if requires_treasury {
                                            "Required for values above R10,000,000"
                                        } else {
                                            "Not required (value below R10,000,000)"
                                        }
                                    </p>
                                </div>
                            </div>
                        </div>
                    },

                    view! {
                        <div style="margin-top: 16px;">
                            <p style="font-size: 12px; color: var(--text-muted);">
                                "Based on estimated value of "
                                <strong>{format_currency(value)}</strong>
                                ", the following approvals will be required before proceeding."
                            </p>
                        </div>
                    }
                ]
            )}

            // Form actions
            {panel_with_footer(
                "Submit Request".to_string(),
                vec![],
                vec![
                    notice_bar(
                        "By submitting this request, you confirm that you have conducted appropriate market research and that deviation from competitive bidding is justified.".to_string(),
                        NoticeType::Warning,
                        None,
                    ),
                ],
                vec![
                    view! { <button class="btn btn-secondary" on:click={handle_save_draft}>"Save Draft"</button> },
                    view! {
                        <button class="btn btn-primary" on:click={handle_submit} disabled={submitting.get()}>
                            if submitting.get() { "Submitting..." } else { "Submit for Approval" }
                        </button>
                    },
                ]
            )}
        </div>
    }
}
