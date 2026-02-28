//! Tender create/edit form with multi-step wizard

use components::prelude::*;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, panel_with_footer,
    stepper, StepperItem, StepStatus,
    notice_bar, NoticeType,
};
use crate::shared::forms::{
    text_input, textarea, select, SelectOption,
    currency_input, date_picker, file_upload, UploadedFile,
    form_group, checkbox,
};
use crate::util::format::format_currency;
use super::store::TendersStore;
use super::types::{Tender, TenderType, EvaluationCriterion, TenderDocument};
use super::service;

/// Form steps
#[derive(Clone, Copy, PartialEq)]
enum FormStep {
    BasicInfo,
    Scope,
    Criteria,
    Documents,
    Review,
}

impl FormStep {
    fn index(&self) -> u32 {
        match self {
            FormStep::BasicInfo => 1,
            FormStep::Scope => 2,
            FormStep::Criteria => 3,
            FormStep::Documents => 4,
            FormStep::Review => 5,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            FormStep::BasicInfo => "Basic Info",
            FormStep::Scope => "Scope",
            FormStep::Criteria => "Criteria",
            FormStep::Documents => "Documents",
            FormStep::Review => "Review",
        }
    }
}

/// Tender form component
#[component]
pub fn tender_form(tender_id: Option<String>) -> View {
    let store = use_context::<TendersStore>();
    let is_edit = tender_id.is_some();

    // Current step
    let current_step = signal(FormStep::BasicInfo);

    // Form fields - Basic Info
    let title = signal(String::new());
    let description = signal(String::new());
    let tender_type = signal("rfq".to_string());
    let estimated_value = signal(0.0);
    let category = signal(String::new());
    let department = signal(String::new());

    // Form fields - Scope
    let scope_of_work = signal(String::new());
    let technical_requirements = signal(String::new());
    let delivery_location = signal(String::new());
    let contract_duration = signal(String::new());
    let cost_center = signal(String::new());

    // Form fields - Criteria
    let price_weight = signal(80.0);
    let bbbee_weight = signal(20.0);
    let functionality_threshold = signal(70.0);
    let criteria: Signal<Vec<EvaluationCriterion>> = signal(Vec::new());

    // Form fields - Documents
    let documents: Signal<Vec<UploadedFile>> = signal(Vec::new());

    // Mandatory requirements
    let req_bbbee = signal(true);
    let req_tax = signal(true);
    let req_cipc = signal(false);
    let req_experience = signal(false);

    // Form state
    let saving = signal(false);
    let error = signal::<Option<String>>(None);

    // Load existing tender if editing
    effect({
        let store = store.clone();
        let tender_id = tender_id.clone();
        let title = title.clone();
        let description = description.clone();
        let tender_type = tender_type.clone();
        let estimated_value = estimated_value.clone();
        let category = category.clone();
        let department = department.clone();
        let scope_of_work = scope_of_work.clone();
        let technical_requirements = technical_requirements.clone();
        let delivery_location = delivery_location.clone();
        let contract_duration = contract_duration.clone();
        let cost_center = cost_center.clone();
        let price_weight = price_weight.clone();
        let bbbee_weight = bbbee_weight.clone();
        let functionality_threshold = functionality_threshold.clone();
        let criteria = criteria.clone();

        move || {
            if let Some(ref id) = tender_id {
                let store = store.clone();
                let id = id.clone();
                let title = title.clone();
                let description = description.clone();
                let tender_type = tender_type.clone();
                let estimated_value = estimated_value.clone();
                let category = category.clone();
                let department = department.clone();
                let scope_of_work = scope_of_work.clone();
                let technical_requirements = technical_requirements.clone();
                let delivery_location = delivery_location.clone();
                let contract_duration = contract_duration.clone();
                let cost_center = cost_center.clone();
                let price_weight = price_weight.clone();
                let bbbee_weight = bbbee_weight.clone();
                let functionality_threshold = functionality_threshold.clone();
                let criteria = criteria.clone();

                spawn(async move {
                    service::get_tender(&store, &id).await;
                    if let Some(tender) = store.selected.get() {
                        title.set(tender.title);
                        description.set(tender.description);
                        tender_type.set(match tender.tender_type {
                            TenderType::Rfq => "rfq".to_string(),
                            TenderType::Rfp => "rfp".to_string(),
                            TenderType::Rft => "rft".to_string(),
                        });
                        estimated_value.set(tender.estimated_value);
                        category.set(tender.category);
                        department.set(tender.department);
                        scope_of_work.set(tender.scope_of_work);
                        technical_requirements.set(tender.technical_requirements);
                        delivery_location.set(tender.delivery_location);
                        contract_duration.set(tender.contract_duration);
                        cost_center.set(tender.cost_center);
                        price_weight.set(tender.price_weight);
                        bbbee_weight.set(tender.bbbee_weight);
                        functionality_threshold.set(tender.functionality_threshold);
                        criteria.set(tender.evaluation_criteria);
                    }
                });
            }
        }
    });

    // Build stepper items
    let steps = vec![
        StepperItem {
            number: 1,
            label: "Basic Info".to_string(),
            status: if current_step.get().index() > 1 { StepStatus::Completed }
                   else if current_step.get() == FormStep::BasicInfo { StepStatus::Active }
                   else { StepStatus::Pending },
        },
        StepperItem {
            number: 2,
            label: "Scope".to_string(),
            status: if current_step.get().index() > 2 { StepStatus::Completed }
                   else if current_step.get() == FormStep::Scope { StepStatus::Active }
                   else { StepStatus::Pending },
        },
        StepperItem {
            number: 3,
            label: "Criteria".to_string(),
            status: if current_step.get().index() > 3 { StepStatus::Completed }
                   else if current_step.get() == FormStep::Criteria { StepStatus::Active }
                   else { StepStatus::Pending },
        },
        StepperItem {
            number: 4,
            label: "Documents".to_string(),
            status: if current_step.get().index() > 4 { StepStatus::Completed }
                   else if current_step.get() == FormStep::Documents { StepStatus::Active }
                   else { StepStatus::Pending },
        },
        StepperItem {
            number: 5,
            label: "Review".to_string(),
            status: if current_step.get() == FormStep::Review { StepStatus::Active }
                   else { StepStatus::Pending },
        },
    ];

    // Step navigation
    let handle_step_click = Callback::<u32>::new({
        let current_step = current_step.clone();
        move |step: u32| {
            let new_step = match step {
                1 => FormStep::BasicInfo,
                2 => FormStep::Scope,
                3 => FormStep::Criteria,
                4 => FormStep::Documents,
                5 => FormStep::Review,
                _ => FormStep::BasicInfo,
            };
            current_step.set(new_step);
        }
    });

    let handle_next: Callback<()> = Callback::new({
        let current_step = current_step.clone();
        move |_| {
            let new_step = match current_step.get() {
                FormStep::BasicInfo => FormStep::Scope,
                FormStep::Scope => FormStep::Criteria,
                FormStep::Criteria => FormStep::Documents,
                FormStep::Documents => FormStep::Review,
                FormStep::Review => FormStep::Review,
            };
            current_step.set(new_step);
        }
    });

    let handle_back: Callback<()> = Callback::new({
        let current_step = current_step.clone();
        move |_| {
            let new_step = match current_step.get() {
                FormStep::BasicInfo => FormStep::BasicInfo,
                FormStep::Scope => FormStep::BasicInfo,
                FormStep::Criteria => FormStep::Scope,
                FormStep::Documents => FormStep::Criteria,
                FormStep::Review => FormStep::Documents,
            };
            current_step.set(new_step);
        }
    });

    // Save handlers
    let handle_save_draft: Callback<()> = Callback::new({
        let store = store.clone();
        let saving = saving.clone();
        let error = error.clone();
        let title = title.clone();
        let description = description.clone();
        let tender_type = tender_type.clone();
        let estimated_value = estimated_value.clone();
        let category = category.clone();
        let department = department.clone();
        let scope_of_work = scope_of_work.clone();
        let technical_requirements = technical_requirements.clone();
        let delivery_location = delivery_location.clone();
        let contract_duration = contract_duration.clone();
        let cost_center = cost_center.clone();
        let price_weight = price_weight.clone();
        let bbbee_weight = bbbee_weight.clone();
        let functionality_threshold = functionality_threshold.clone();
        let criteria = criteria.clone();
        let req_bbbee = req_bbbee.clone();
        let req_tax = req_tax.clone();
        let req_cipc = req_cipc.clone();
        let req_experience = req_experience.clone();
        let tender_id = tender_id.clone();

        move |_| {
            let store = store.clone();
            let saving = saving.clone();
            let error = error.clone();
            let tender_id = tender_id.clone();

            // Build mandatory requirements
            let mut mandatory_requirements = Vec::new();
            if req_bbbee.get() { mandatory_requirements.push("Valid B-BBEE certificate".to_string()); }
            if req_tax.get() { mandatory_requirements.push("Tax clearance certificate".to_string()); }
            if req_cipc.get() { mandatory_requirements.push("CIPC registration".to_string()); }
            if req_experience.get() { mandatory_requirements.push("Minimum 3 years experience".to_string()); }

            let tender = Tender {
                id: tender_id.clone().unwrap_or_default(),
                reference_number: String::new(),
                title: title.get(),
                description: description.get(),
                tender_type: match tender_type.get().as_str() {
                    "rfp" => TenderType::Rfp,
                    "rft" => TenderType::Rft,
                    _ => TenderType::Rfq,
                },
                estimated_value: estimated_value.get(),
                category: category.get(),
                department: department.get(),
                scope_of_work: scope_of_work.get(),
                technical_requirements: technical_requirements.get(),
                delivery_location: delivery_location.get(),
                contract_duration: contract_duration.get(),
                cost_center: cost_center.get(),
                price_weight: price_weight.get(),
                bbbee_weight: bbbee_weight.get(),
                functionality_threshold: functionality_threshold.get(),
                evaluation_criteria: criteria.get(),
                mandatory_requirements,
                ..Default::default()
            };

            spawn(async move {
                saving.set(true);
                error.set(None);

                let result = if tender_id.is_some() {
                    service::update_tender(&store, tender).await
                } else {
                    service::create_tender(&store, tender).await
                };

                match result {
                    Ok(_) => {
                        // Navigate back to list
                        web_sys::window()
                            .unwrap()
                            .location()
                            .set_href("#/tenders")
                            .ok();
                    }
                    Err(e) => {
                        error.set(Some(e));
                    }
                }
                saving.set(false);
            });
        }
    });

    let handle_submit: Callback<()> = Callback::new({
        let store = store.clone();
        let saving = saving.clone();
        let error = error.clone();
        let tender_id = tender_id.clone();

        move |_| {
            let store = store.clone();
            let saving = saving.clone();
            let error = error.clone();
            let tender_id = tender_id.clone();

            if let Some(id) = tender_id {
                spawn(async move {
                    saving.set(true);
                    error.set(None);

                    match service::submit_for_approval(&store, &id).await {
                        Ok(_) => {
                            web_sys::window()
                                .unwrap()
                                .location()
                                .set_href("#/tenders")
                                .ok();
                        }
                        Err(e) => {
                            error.set(Some(e));
                        }
                    }
                    saving.set(false);
                });
            }
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

    // Add criterion
    let handle_add_criterion: Callback<()> = Callback::new({
        let criteria = criteria.clone();
        move |_| {
            let mut crits = criteria.get();
            let idx = crits.len() + 1;
            crits.push(EvaluationCriterion {
                id: format!("EC-{:03}", idx),
                name: String::new(),
                description: String::new(),
                weight: 0.0,
                max_score: 100,
            });
            criteria.set(crits);
        }
    });

    // Category options
    let category_options = vec![
        SelectOption { value: "".to_string(), label: "Select Category".to_string() },
        SelectOption { value: "it".to_string(), label: "Information Technology".to_string() },
        SelectOption { value: "facilities".to_string(), label: "Facilities Management".to_string() },
        SelectOption { value: "security".to_string(), label: "Security Services".to_string() },
        SelectOption { value: "professional".to_string(), label: "Professional Services".to_string() },
        SelectOption { value: "fleet".to_string(), label: "Fleet Services".to_string() },
        SelectOption { value: "furniture".to_string(), label: "Furniture & Fittings".to_string() },
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

    // Tender type options
    let type_options = vec![
        SelectOption { value: "rfq".to_string(), label: "RFQ - Request for Quotation".to_string() },
        SelectOption { value: "rfp".to_string(), label: "RFP - Request for Proposal".to_string() },
        SelectOption { value: "rft".to_string(), label: "RFT - Request for Tender".to_string() },
    ];

    let page_title = if is_edit { "Edit Tender" } else { "New Tender" };
    let page_subtitle = if is_edit {
        "Update tender details"
    } else {
        "Create a new procurement tender"
    };

    view! {
        style {
            r#"
            .tender-form { display: flex; flex-direction: column; gap: var(--space-4); }
            .step-content { min-height: 400px; }
            .form-actions {
                display: flex;
                justify-content: space-between;
                padding-top: 16px;
                border-top: 1px solid var(--border);
                margin-top: 16px;
            }
            .form-actions-left, .form-actions-right {
                display: flex;
                gap: 12px;
            }
            .review-section {
                margin-bottom: 24px;
            }
            .review-section h4 {
                font-size: 14px;
                font-weight: 600;
                color: var(--navy);
                margin-bottom: 12px;
                padding-bottom: 8px;
                border-bottom: 1px solid var(--border);
            }
            .review-grid {
                display: grid;
                grid-template-columns: repeat(2, 1fr);
                gap: 12px;
            }
            .review-item label {
                font-size: 11px;
                color: var(--text-muted);
                display: block;
                margin-bottom: 4px;
            }
            .review-item span {
                font-size: 13px;
                color: var(--text);
            }
            .criteria-list {
                display: flex;
                flex-direction: column;
                gap: 12px;
            }
            .criterion-row {
                display: grid;
                grid-template-columns: 2fr 3fr 80px 60px;
                gap: 12px;
                padding: 12px;
                background: var(--bg);
                border-radius: var(--radius);
            }
            .criterion-row input {
                padding: 8px 10px;
                border: 1px solid var(--border);
                border-radius: var(--radius-sm);
                font-size: 13px;
            }
            .weight-summary {
                margin-top: 16px;
                padding: 12px;
                background: var(--blue-light);
                border-radius: var(--radius);
                font-size: 13px;
            }
            "#
        }

        <div class="tender-form" data-testid="tender-form">
            {page_header(
                page_title.to_string(),
                Some(page_subtitle.to_string()),
                vec![
                    view! { <a href="#/tenders" class="btn btn-secondary">"Cancel"</a> },
                ]
            )}

            // Error notice
            if let Some(err) = error.get() {
                {notice_bar(err, NoticeType::Error, None)}
            }

            // Stepper
            {stepper(steps, Some(handle_step_click))}

            // Step content
            {panel(
                current_step.get().label().to_string(),
                vec![],
                vec![
                    view! {
                        <div class="step-content">
                            // Basic Info Step
                            if current_step.get() == FormStep::BasicInfo {
                                <div>
                                    {form_group(
                                        Some("Tender Details".to_string()),
                                        2,
                                        vec![
                                            text_input(
                                                "Tender Title".to_string(),
                                                title.clone(),
                                                Some("Enter a descriptive title".to_string()),
                                                true,
                                                false,
                                                None,
                                                None,
                                                None,
                                            ),
                                            select(
                                                "Tender Type".to_string(),
                                                tender_type.clone(),
                                                type_options.clone(),
                                                None,
                                                true,
                                                false,
                                                None,
                                            ),
                                        ]
                                    )}
                                    {form_group(
                                        None,
                                        1,
                                        vec![
                                            textarea(
                                                "Description".to_string(),
                                                description.clone(),
                                                Some("Brief description of the procurement".to_string()),
                                                true,
                                                false,
                                                Some(4),
                                                None,
                                                None,
                                            ),
                                        ]
                                    )}
                                    {form_group(
                                        Some("Classification".to_string()),
                                        2,
                                        vec![
                                            select(
                                                "Category".to_string(),
                                                category.clone(),
                                                category_options.clone(),
                                                None,
                                                true,
                                                false,
                                                None,
                                            ),
                                            select(
                                                "Department".to_string(),
                                                department.clone(),
                                                department_options.clone(),
                                                None,
                                                true,
                                                false,
                                                None,
                                            ),
                                        ]
                                    )}
                                    {form_group(
                                        Some("Value".to_string()),
                                        2,
                                        vec![
                                            currency_input(
                                                "Estimated Value".to_string(),
                                                estimated_value.clone(),
                                                true,
                                                false,
                                                None,
                                                Some("Estimated contract value in ZAR".to_string()),
                                            ),
                                        ]
                                    )}
                                </div>
                            }

                            // Scope Step
                            if current_step.get() == FormStep::Scope {
                                <div>
                                    {form_group(
                                        Some("Scope of Work".to_string()),
                                        1,
                                        vec![
                                            textarea(
                                                "Scope Description".to_string(),
                                                scope_of_work.clone(),
                                                Some("Detailed description of work to be performed".to_string()),
                                                true,
                                                false,
                                                Some(6),
                                                None,
                                                None,
                                            ),
                                        ]
                                    )}
                                    {form_group(
                                        Some("Technical Requirements".to_string()),
                                        1,
                                        vec![
                                            textarea(
                                                "Technical Specifications".to_string(),
                                                technical_requirements.clone(),
                                                Some("Technical requirements and specifications".to_string()),
                                                false,
                                                false,
                                                Some(4),
                                                None,
                                                None,
                                            ),
                                        ]
                                    )}
                                    {form_group(
                                        Some("Delivery Details".to_string()),
                                        2,
                                        vec![
                                            text_input(
                                                "Delivery Location".to_string(),
                                                delivery_location.clone(),
                                                Some("e.g., Head Office, Johannesburg".to_string()),
                                                true,
                                                false,
                                                None,
                                                None,
                                                None,
                                            ),
                                            text_input(
                                                "Contract Duration".to_string(),
                                                contract_duration.clone(),
                                                Some("e.g., 36 months".to_string()),
                                                true,
                                                false,
                                                None,
                                                None,
                                                None,
                                            ),
                                            text_input(
                                                "Cost Center".to_string(),
                                                cost_center.clone(),
                                                Some("e.g., CC-IT-001".to_string()),
                                                true,
                                                false,
                                                None,
                                                None,
                                                None,
                                            ),
                                        ]
                                    )}
                                    {form_group(
                                        Some("Mandatory Requirements".to_string()),
                                        2,
                                        vec![
                                            checkbox("Valid B-BBEE certificate".to_string(), req_bbbee.clone(), false),
                                            checkbox("Tax clearance certificate".to_string(), req_tax.clone(), false),
                                            checkbox("CIPC registration".to_string(), req_cipc.clone(), false),
                                            checkbox("Minimum 3 years experience".to_string(), req_experience.clone(), false),
                                        ]
                                    )}
                                </div>
                            }

                            // Criteria Step
                            if current_step.get() == FormStep::Criteria {
                                <div>
                                    {form_group(
                                        Some("Scoring Weights".to_string()),
                                        3,
                                        vec![
                                            currency_input(
                                                "Price Weight (%)".to_string(),
                                                price_weight.clone(),
                                                true,
                                                false,
                                                None,
                                                None,
                                            ),
                                            currency_input(
                                                "B-BBEE Weight (%)".to_string(),
                                                bbbee_weight.clone(),
                                                true,
                                                false,
                                                None,
                                                None,
                                            ),
                                            currency_input(
                                                "Functionality Threshold (%)".to_string(),
                                                functionality_threshold.clone(),
                                                true,
                                                false,
                                                None,
                                                Some("Minimum score to qualify".to_string()),
                                            ),
                                        ]
                                    )}

                                    <div class="form-section">
                                        <div class="form-section-title">
                                            "Evaluation Criteria"
                                            <button class="btn btn-sm btn-secondary" style="float: right;" on:click={handle_add_criterion}>
                                                "+ Add Criterion"
                                            </button>
                                        </div>
                                        <div class="criteria-list">
                                            <div class="criterion-row" style="background: var(--surface); font-weight: 600; font-size: 12px;">
                                                <span>"Criterion Name"</span>
                                                <span>"Description"</span>
                                                <span>"Weight %"</span>
                                                <span>"Max"</span>
                                            </div>
                                            for (idx, criterion) in criteria.get().iter().enumerate() {
                                                <div class="criterion-row">
                                                    <input type="text" value={criterion.name.clone()} placeholder="e.g., Technical Capability" />
                                                    <input type="text" value={criterion.description.clone()} placeholder="Description" />
                                                    <input type="number" value={criterion.weight.to_string()} placeholder="%" />
                                                    <input type="number" value={criterion.max_score.to_string()} placeholder="100" />
                                                </div>
                                            }
                                        </div>
                                        <div class="weight-summary">
                                            "Total Functionality Weight: "
                                            <strong>{criteria.get().iter().map(|c| c.weight).sum::<f64>()}"%"</strong>
                                            " (must equal 100%)"
                                        </div>
                                    </div>
                                </div>
                            }

                            // Documents Step
                            if current_step.get() == FormStep::Documents {
                                <div>
                                    {file_upload(
                                        "Tender Documents".to_string(),
                                        documents.clone(),
                                        Some(".pdf,.doc,.docx,.xls,.xlsx".to_string()),
                                        true,
                                        false,
                                        Some("Upload specifications, terms, and other tender documents (PDF, Word, Excel)".to_string()),
                                        handle_file_remove.clone(),
                                    )}

                                    {notice_bar(
                                        "Recommended documents: Technical Specifications, Terms and Conditions, Evaluation Criteria, Pricing Schedule, Company Profile Template".to_string(),
                                        NoticeType::Info,
                                        None,
                                    )}
                                </div>
                            }

                            // Review Step
                            if current_step.get() == FormStep::Review {
                                <div>
                                    <div class="review-section">
                                        <h4>"Basic Information"</h4>
                                        <div class="review-grid">
                                            <div class="review-item">
                                                <label>"Title"</label>
                                                <span>{title.get()}</span>
                                            </div>
                                            <div class="review-item">
                                                <label>"Type"</label>
                                                <span>{tender_type.get().to_uppercase()}</span>
                                            </div>
                                            <div class="review-item">
                                                <label>"Category"</label>
                                                <span>{category.get()}</span>
                                            </div>
                                            <div class="review-item">
                                                <label>"Department"</label>
                                                <span>{department.get()}</span>
                                            </div>
                                            <div class="review-item">
                                                <label>"Estimated Value"</label>
                                                <span>{format_currency(estimated_value.get())}</span>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="review-section">
                                        <h4>"Scope"</h4>
                                        <div class="review-grid">
                                            <div class="review-item">
                                                <label>"Delivery Location"</label>
                                                <span>{delivery_location.get()}</span>
                                            </div>
                                            <div class="review-item">
                                                <label>"Contract Duration"</label>
                                                <span>{contract_duration.get()}</span>
                                            </div>
                                            <div class="review-item">
                                                <label>"Cost Center"</label>
                                                <span>{cost_center.get()}</span>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="review-section">
                                        <h4>"Evaluation Criteria"</h4>
                                        <div class="review-grid">
                                            <div class="review-item">
                                                <label>"Price Weight"</label>
                                                <span>{price_weight.get()}"%"</span>
                                            </div>
                                            <div class="review-item">
                                                <label>"B-BBEE Weight"</label>
                                                <span>{bbbee_weight.get()}"%"</span>
                                            </div>
                                            <div class="review-item">
                                                <label>"Functionality Threshold"</label>
                                                <span>{functionality_threshold.get()}"%"</span>
                                            </div>
                                            <div class="review-item">
                                                <label>"Criteria Count"</label>
                                                <span>{criteria.get().len()}" criteria defined"</span>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="review-section">
                                        <h4>"Documents"</h4>
                                        <div class="review-item">
                                            <label>"Uploaded Documents"</label>
                                            <span>{documents.get().len()}" document(s)"</span>
                                        </div>
                                    </div>

                                    {notice_bar(
                                        "Please review all information before saving. Once submitted for approval, changes will require re-approval.".to_string(),
                                        NoticeType::Warning,
                                        None,
                                    )}
                                </div>
                            }
                        </div>
                    },

                    // Form actions
                    view! {
                        <div class="form-actions">
                            <div class="form-actions-left">
                                if current_step.get() != FormStep::BasicInfo {
                                    <button class="btn btn-secondary" on:click={handle_back}>
                                        "Back"
                                    </button>
                                }
                            </div>
                            <div class="form-actions-right">
                                <button class="btn btn-secondary" on:click={handle_save_draft.clone()} disabled={saving.get()}>
                                    if saving.get() { "Saving..." } else { "Save Draft" }
                                </button>
                                if current_step.get() == FormStep::Review {
                                    if is_edit {
                                        <button class="btn btn-primary" on:click={handle_submit} disabled={saving.get()}>
                                            "Submit for Approval"
                                        </button>
                                    }
                                } else {
                                    <button class="btn btn-primary" on:click={handle_next}>
                                        "Next"
                                    </button>
                                }
                            </div>
                        </div>
                    }
                ]
            )}
        </div>
    }
}
