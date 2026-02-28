//! Contract create/edit form

use components::prelude::*;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, panel_with_footer,
    stepper, StepperItem, StepStatus,
    notice_bar, NoticeType,
};
use crate::shared::forms::{
    text_input, textarea, select, date_picker, currency_input,
};
use crate::util::format::format_currency;
use super::types::{Contract, ContractStatus, ContractTerms, ContractSla, ContractDeliverable};
use super::store::ContractsStore;
use super::service;

/// Contract form page (create/edit)
#[component]
pub fn contract_form(contract_id: Option<String>) -> View {
    let store = use_context::<ContractsStore>();
    let is_edit = contract_id.is_some();

    // Form step state
    let current_step = signal(1u32);

    // Contract Details (Step 1)
    let title = signal(String::new());
    let description = signal(String::new());
    let contract_type = signal("Services".to_string());
    let reference_number = signal(String::new());
    let supplier_id = signal(String::new());
    let supplier_name = signal(String::new());
    let value = signal(0.0f64);
    let start_date = signal(String::new());
    let end_date = signal(String::new());
    let tender_id = signal(String::new());

    // Terms (Step 2)
    let payment_terms = signal("30 days from invoice".to_string());
    let warranty_months = signal(12u32);
    let notice_days = signal(30u32);
    let renewal_terms = signal(String::new());
    let termination_clause = signal(String::new());
    let dispute_resolution = signal("Arbitration".to_string());
    let governing_law = signal("South African Law".to_string());

    // SLA (Step 3)
    let sla_enabled = signal(false);
    let response_time = signal(4u32);
    let resolution_time = signal(24u32);
    let availability = signal(99.5f64);
    let penalty_clause = signal(String::new());
    let escalation_procedure = signal(String::new());

    // Deliverables (Step 4)
    let deliverables: Signal<Vec<ContractDeliverable>> = signal(Vec::new());

    // Form state
    let form_error = signal::<Option<String>>(None);
    let saving = store.saving.clone();

    // Load existing contract if editing
    if let Some(id) = &contract_id {
        effect({
            let store = store.clone();
            let id = id.clone();
            let title = title.clone();
            let description = description.clone();
            let contract_type = contract_type.clone();
            let reference_number = reference_number.clone();
            let supplier_id = supplier_id.clone();
            let supplier_name = supplier_name.clone();
            let value = value.clone();
            let start_date = start_date.clone();
            let end_date = end_date.clone();
            let payment_terms = payment_terms.clone();
            let sla_enabled = sla_enabled.clone();

            move || {
                let store = store.clone();
                let id = id.clone();
                let title = title.clone();
                let description = description.clone();
                let contract_type = contract_type.clone();
                let reference_number = reference_number.clone();
                let supplier_id = supplier_id.clone();
                let supplier_name = supplier_name.clone();
                let value = value.clone();
                let start_date = start_date.clone();
                let end_date = end_date.clone();
                let payment_terms = payment_terms.clone();
                let sla_enabled = sla_enabled.clone();

                spawn(async move {
                    service::load_contract(&store, &id).await;

                    if let Some(contract) = store.selected.get() {
                        title.set(contract.title);
                        description.set(contract.description);
                        contract_type.set(contract.contract_type);
                        reference_number.set(contract.reference_number);
                        supplier_id.set(contract.supplier_id);
                        supplier_name.set(contract.supplier_name);
                        value.set(contract.value);
                        start_date.set(contract.start_date);
                        end_date.set(contract.end_date);
                        payment_terms.set(contract.terms.payment_terms);
                        sla_enabled.set(contract.sla.is_some());
                    }
                });
            }
        });
    }

    // Step navigation
    let go_to_step = Callback::new({
        let current_step = current_step.clone();
        move |step: u32| {
            current_step.set(step);
        }
    });

    let next_step: Callback<()> = Callback::new({
        let current_step = current_step.clone();
        move |_| {
            let step = current_step.get();
            if step < 4 {
                current_step.set(step + 1);
            }
        }
    });

    let prev_step: Callback<()> = Callback::new({
        let current_step = current_step.clone();
        move |_| {
            let step = current_step.get();
            if step > 1 {
                current_step.set(step - 1);
            }
        }
    });

    // Add deliverable
    let add_deliverable: Callback<()> = Callback::new({
        let deliverables = deliverables.clone();
        move |_| {
            let mut items = deliverables.get();
            items.push(ContractDeliverable {
                id: format!("DEL-{:03}", items.len() + 1),
                ..Default::default()
            });
            deliverables.set(items);
        }
    });

    // Remove deliverable
    let remove_deliverable = {
        let deliverables = deliverables.clone();
        move |idx: usize| {
            let mut items = deliverables.get();
            if idx < items.len() {
                items.remove(idx);
                deliverables.set(items);
            }
        }
    };

    // Save contract
    let handle_save: Callback<()> = Callback::new({
        let store = store.clone();
        let contract_id = contract_id.clone();
        let title = title.clone();
        let description = description.clone();
        let contract_type = contract_type.clone();
        let reference_number = reference_number.clone();
        let supplier_id = supplier_id.clone();
        let supplier_name = supplier_name.clone();
        let value = value.clone();
        let start_date = start_date.clone();
        let end_date = end_date.clone();
        let payment_terms = payment_terms.clone();
        let warranty_months = warranty_months.clone();
        let notice_days = notice_days.clone();
        let renewal_terms = renewal_terms.clone();
        let termination_clause = termination_clause.clone();
        let dispute_resolution = dispute_resolution.clone();
        let governing_law = governing_law.clone();
        let sla_enabled = sla_enabled.clone();
        let response_time = response_time.clone();
        let resolution_time = resolution_time.clone();
        let availability = availability.clone();
        let penalty_clause = penalty_clause.clone();
        let escalation_procedure = escalation_procedure.clone();
        let deliverables = deliverables.clone();
        let form_error = form_error.clone();

        move |_| {
            let store = store.clone();
            let contract_id = contract_id.clone();
            let form_error = form_error.clone();

            let contract = Contract {
                id: contract_id.clone().unwrap_or_default(),
                title: title.get(),
                description: description.get(),
                supplier_id: supplier_id.get(),
                supplier_name: supplier_name.get(),
                supplier_bbbee_level: 1,
                value: value.get(),
                start_date: start_date.get(),
                end_date: end_date.get(),
                status: ContractStatus::Draft,
                contract_type: contract_type.get(),
                reference_number: reference_number.get(),
                tender_id: None,
                purchase_order_id: None,
                terms: ContractTerms {
                    payment_terms: payment_terms.get(),
                    warranty_period_months: warranty_months.get(),
                    notice_period_days: notice_days.get(),
                    renewal_terms: renewal_terms.get(),
                    termination_clause: termination_clause.get(),
                    dispute_resolution: dispute_resolution.get(),
                    governing_law: governing_law.get(),
                    special_conditions: Vec::new(),
                },
                sla: if sla_enabled.get() {
                    Some(ContractSla {
                        response_time_hours: response_time.get(),
                        resolution_time_hours: resolution_time.get(),
                        availability_percent: availability.get(),
                        penalty_clause: penalty_clause.get(),
                        escalation_procedure: escalation_procedure.get(),
                    })
                } else {
                    None
                },
                deliverables: deliverables.get(),
                milestones: Vec::new(),
                documents: Vec::new(),
                created_by: "Current User".to_string(),
                created_at: "2025-02-27T10:00:00Z".to_string(),
                updated_at: "2025-02-27T10:00:00Z".to_string(),
                approved_by: None,
                approved_at: None,
            };

            spawn(async move {
                let result = if contract_id.is_some() {
                    service::update_contract(&store, contract).await
                } else {
                    service::create_contract(&store, contract).await.map(|_| ())
                };

                match result {
                    Ok(_) => {
                        // Navigate to contract list
                        // router.push("/contracts");
                    }
                    Err(e) => {
                        form_error.set(Some(e));
                    }
                }
            });
        }
    });

    // Submit for approval
    let handle_submit_for_approval: Callback<()> = Callback::new({
        let store = store.clone();
        let contract_id = contract_id.clone();
        let form_error = form_error.clone();

        move |_| {
            if let Some(id) = &contract_id {
                let store = store.clone();
                let id = id.clone();
                let form_error = form_error.clone();

                spawn(async move {
                    match service::submit_for_approval(&store, &id).await {
                        Ok(_) => {
                            // Navigate or show success
                        }
                        Err(e) => {
                            form_error.set(Some(e));
                        }
                    }
                });
            }
        }
    });

    // Stepper items
    let step = current_step.get();
    let steps = vec![
        StepperItem {
            number: 1,
            label: "Contract Details".to_string(),
            status: if step > 1 { StepStatus::Completed } else if step == 1 { StepStatus::Active } else { StepStatus::Pending },
        },
        StepperItem {
            number: 2,
            label: "Terms".to_string(),
            status: if step > 2 { StepStatus::Completed } else if step == 2 { StepStatus::Active } else { StepStatus::Pending },
        },
        StepperItem {
            number: 3,
            label: "SLA".to_string(),
            status: if step > 3 { StepStatus::Completed } else if step == 3 { StepStatus::Active } else { StepStatus::Pending },
        },
        StepperItem {
            number: 4,
            label: "Deliverables".to_string(),
            status: if step == 4 { StepStatus::Active } else { StepStatus::Pending },
        },
    ];

    // Supplier options (mock)
    let supplier_options = vec![
        ("SUP-001", "TechSolutions SA (Pty) Ltd"),
        ("SUP-002", "Office Pro Distributors"),
        ("SUP-003", "SecureGuard Holdings"),
        ("SUP-004", "AutoCare Fleet Management"),
        ("SUP-005", "CloudFirst SA"),
    ];

    view! {
        style {
            r#"
            .contract-form { display: flex; flex-direction: column; gap: var(--space-6); }
            .form-grid {
                display: grid;
                grid-template-columns: repeat(2, 1fr);
                gap: 20px;
            }
            .form-grid .span-2 { grid-column: span 2; }
            .form-actions {
                display: flex;
                justify-content: space-between;
                padding-top: 20px;
                border-top: 1px solid var(--border);
            }
            .form-actions-left { display: flex; gap: 12px; }
            .form-actions-right { display: flex; gap: 12px; }
            .sla-toggle {
                display: flex;
                align-items: center;
                gap: 12px;
                padding: 16px;
                background: var(--bg);
                border-radius: var(--radius);
                margin-bottom: 20px;
            }
            .sla-toggle label {
                display: flex;
                align-items: center;
                gap: 8px;
                cursor: pointer;
                font-weight: 500;
            }
            .deliverables-list { display: flex; flex-direction: column; gap: 16px; }
            .deliverable-item {
                padding: 16px;
                background: var(--bg);
                border-radius: var(--radius);
                border: 1px solid var(--border);
            }
            .deliverable-header {
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin-bottom: 16px;
            }
            .deliverable-header h4 {
                font-size: 14px;
                font-weight: 600;
            }
            .deliverable-grid {
                display: grid;
                grid-template-columns: repeat(4, 1fr);
                gap: 16px;
            }
            .add-deliverable-btn {
                display: flex;
                align-items: center;
                justify-content: center;
                gap: 8px;
                padding: 12px;
                border: 2px dashed var(--border);
                border-radius: var(--radius);
                background: transparent;
                cursor: pointer;
                color: var(--text-muted);
                font-size: 13px;
                transition: all 0.15s;
            }
            .add-deliverable-btn:hover {
                border-color: var(--blue);
                color: var(--blue);
            }
            "#
        }

        <div class="contract-form" data-testid="contract-form">
            {page_header(
                if is_edit { "Edit Contract".to_string() } else { "New Contract".to_string() },
                Some(if is_edit { "Update contract details".to_string() } else { "Create a new supplier contract".to_string() }),
                vec![
                    view! { <a href="/contracts" class="btn btn-secondary">"Cancel"</a> },
                ]
            )}

            // Error notice
            if let Some(error) = form_error.get() {
                {notice_bar(error, NoticeType::Error, Some(Callback::new({
                    let form_error = form_error.clone();
                    move |_| form_error.set(None)
                })))}
            }

            // Stepper
            {stepper(steps, Some(Callback::new({
                let current_step = current_step.clone();
                move |step| current_step.set(step)
            })))}

            // Step 1: Contract Details
            if step == 1 {
                {panel_with_footer(
                    "Contract Details".to_string(),
                    vec![],
                    vec![
                        view! {
                            <div class="form-grid">
                                {text_input(
                                    "Contract Title".to_string(),
                                    title.clone(),
                                    Some("Enter contract title".to_string()),
                                    true, false, None, None, None
                                )}
                                {select(
                                    "Contract Type".to_string(),
                                    contract_type.clone(),
                                    vec![
                                        crate::shared::forms::select::SelectOption { value: "Goods".to_string(), label: "Goods".to_string() },
                                        crate::shared::forms::select::SelectOption { value: "Services".to_string(), label: "Services".to_string() },
                                        crate::shared::forms::select::SelectOption { value: "Works".to_string(), label: "Works".to_string() },
                                        crate::shared::forms::select::SelectOption { value: "Framework".to_string(), label: "Framework Agreement".to_string() },
                                    ],
                                    Some("Select type".to_string()),
                                    true, false, None
                                )}
                                <div class="span-2">
                                    {textarea(
                                        "Description".to_string(),
                                        description.clone(),
                                        Some("Describe the contract scope and objectives".to_string()),
                                        true, false, Some(4), None, None
                                    )}
                                </div>
                                {text_input(
                                    "Reference Number".to_string(),
                                    reference_number.clone(),
                                    Some("e.g., PFMA/2025/IT/001".to_string()),
                                    false, false, None, None, None
                                )}
                                {text_input(
                                    "Linked Tender ID".to_string(),
                                    tender_id.clone(),
                                    Some("e.g., TND-2025-0001".to_string()),
                                    false, false, None, None, None
                                )}
                                {select(
                                    "Supplier".to_string(),
                                    supplier_id.clone(),
                                    supplier_options.iter().map(|(v, l)| {
                                        crate::shared::forms::select::SelectOption { value: v.to_string(), label: l.to_string() }
                                    }).collect(),
                                    Some("Select supplier".to_string()),
                                    true, false, None
                                )}
                                {currency_input(
                                    "Contract Value".to_string(),
                                    value.clone(),
                                    true, false, None, None
                                )}
                                {date_picker(
                                    "Start Date".to_string(),
                                    start_date.clone(),
                                    true, false, None, None, None
                                )}
                                {date_picker(
                                    "End Date".to_string(),
                                    end_date.clone(),
                                    true, false, None, None, None
                                )}
                            </div>
                        }
                    ],
                    vec![
                        view! { <div class="form-actions-left"></div> },
                        view! {
                            <div class="form-actions-right">
                                <button class="btn btn-primary" on:click={next_step.clone()}>"Next: Terms"</button>
                            </div>
                        },
                    ]
                )}
            }

            // Step 2: Terms
            if step == 2 {
                {panel_with_footer(
                    "Contract Terms".to_string(),
                    vec![],
                    vec![
                        view! {
                            <div class="form-grid">
                                {text_input(
                                    "Payment Terms".to_string(),
                                    payment_terms.clone(),
                                    Some("e.g., 30 days from invoice".to_string()),
                                    true, false, None, None, None
                                )}
                                {text_input(
                                    "Warranty Period (months)".to_string(),
                                    signal(warranty_months.get().to_string()),
                                    Some("e.g., 12".to_string()),
                                    false, false, None, None, Some("number".to_string())
                                )}
                                {text_input(
                                    "Notice Period (days)".to_string(),
                                    signal(notice_days.get().to_string()),
                                    Some("e.g., 30".to_string()),
                                    false, false, None, None, Some("number".to_string())
                                )}
                                {text_input(
                                    "Dispute Resolution".to_string(),
                                    dispute_resolution.clone(),
                                    Some("e.g., Arbitration".to_string()),
                                    false, false, None, None, None
                                )}
                                <div class="span-2">
                                    {textarea(
                                        "Renewal Terms".to_string(),
                                        renewal_terms.clone(),
                                        Some("Describe renewal conditions".to_string()),
                                        false, false, Some(3), None, None
                                    )}
                                </div>
                                <div class="span-2">
                                    {textarea(
                                        "Termination Clause".to_string(),
                                        termination_clause.clone(),
                                        Some("Describe termination conditions".to_string()),
                                        false, false, Some(3), None, None
                                    )}
                                </div>
                                {text_input(
                                    "Governing Law".to_string(),
                                    governing_law.clone(),
                                    Some("e.g., South African Law".to_string()),
                                    false, false, None, None, None
                                )}
                            </div>
                        }
                    ],
                    vec![
                        view! {
                            <div class="form-actions-left">
                                <button class="btn btn-secondary" on:click={prev_step.clone()}>"Back"</button>
                            </div>
                        },
                        view! {
                            <div class="form-actions-right">
                                <button class="btn btn-primary" on:click={next_step.clone()}>"Next: SLA"</button>
                            </div>
                        },
                    ]
                )}
            }

            // Step 3: SLA
            if step == 3 {
                {panel_with_footer(
                    "Service Level Agreement".to_string(),
                    vec![],
                    vec![
                        view! {
                            <div>
                                <div class="sla-toggle">
                                    <label>
                                        <input
                                            type="checkbox"
                                            checked={sla_enabled.get()}
                                            on:change={Callback::new({
                                                let sla_enabled = sla_enabled.clone();
                                                move |_| sla_enabled.set(!sla_enabled.get())
                                            })}
                                        />
                                        "Include SLA Terms"
                                    </label>
                                    <span style="color: var(--text-muted); font-size: 12px;">
                                        "Enable this to define service level requirements and penalties"
                                    </span>
                                </div>

                                if sla_enabled.get() {
                                    <div class="form-grid">
                                        {text_input(
                                            "Response Time (hours)".to_string(),
                                            signal(response_time.get().to_string()),
                                            Some("e.g., 4".to_string()),
                                            true, false, None,
                                            Some("Maximum time to acknowledge an issue".to_string()),
                                            Some("number".to_string())
                                        )}
                                        {text_input(
                                            "Resolution Time (hours)".to_string(),
                                            signal(resolution_time.get().to_string()),
                                            Some("e.g., 24".to_string()),
                                            true, false, None,
                                            Some("Maximum time to resolve an issue".to_string()),
                                            Some("number".to_string())
                                        )}
                                        {text_input(
                                            "Availability (%)".to_string(),
                                            signal(format!("{:.1}", availability.get())),
                                            Some("e.g., 99.9".to_string()),
                                            true, false, None,
                                            Some("Required uptime percentage".to_string()),
                                            Some("number".to_string())
                                        )}
                                        <div></div>
                                        <div class="span-2">
                                            {textarea(
                                                "Penalty Clause".to_string(),
                                                penalty_clause.clone(),
                                                Some("Describe penalties for SLA breaches".to_string()),
                                                false, false, Some(3), None, None
                                            )}
                                        </div>
                                        <div class="span-2">
                                            {textarea(
                                                "Escalation Procedure".to_string(),
                                                escalation_procedure.clone(),
                                                Some("Describe escalation levels and contacts".to_string()),
                                                false, false, Some(3), None, None
                                            )}
                                        </div>
                                    </div>
                                }
                            </div>
                        }
                    ],
                    vec![
                        view! {
                            <div class="form-actions-left">
                                <button class="btn btn-secondary" on:click={prev_step.clone()}>"Back"</button>
                            </div>
                        },
                        view! {
                            <div class="form-actions-right">
                                <button class="btn btn-primary" on:click={next_step.clone()}>"Next: Deliverables"</button>
                            </div>
                        },
                    ]
                )}
            }

            // Step 4: Deliverables
            if step == 4 {
                {panel_with_footer(
                    "Deliverables".to_string(),
                    vec![],
                    vec![
                        view! {
                            <div class="deliverables-list">
                                for (idx, deliverable) in deliverables.get().iter().enumerate() {
                                    <div class="deliverable-item">
                                        <div class="deliverable-header">
                                            <h4>{format!("Deliverable #{}", idx + 1)}</h4>
                                            <button
                                                class="btn btn-sm btn-danger"
                                                on:click={Callback::new({
                                                    let remove_deliverable = remove_deliverable.clone();
                                                    move |_| remove_deliverable(idx)
                                                })}
                                            >
                                                "Remove"
                                            </button>
                                        </div>
                                        <div class="deliverable-grid">
                                            {text_input(
                                                "Description".to_string(),
                                                signal(deliverable.description.clone()),
                                                Some("Item description".to_string()),
                                                true, false, None, None, None
                                            )}
                                            {text_input(
                                                "Quantity".to_string(),
                                                signal(deliverable.quantity.to_string()),
                                                Some("1".to_string()),
                                                true, false, None, None, Some("number".to_string())
                                            )}
                                            {text_input(
                                                "Unit".to_string(),
                                                signal(deliverable.unit.clone()),
                                                Some("e.g., Each, Months".to_string()),
                                                true, false, None, None, None
                                            )}
                                            {currency_input(
                                                "Unit Price".to_string(),
                                                signal(deliverable.unit_price),
                                                true, false, None, None
                                            )}
                                        </div>
                                    </div>
                                }

                                <button class="add-deliverable-btn" on:click={add_deliverable}>
                                    <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2">
                                        <line x1="12" y1="5" x2="12" y2="19"/>
                                        <line x1="5" y1="12" x2="19" y2="12"/>
                                    </svg>
                                    "Add Deliverable"
                                </button>
                            </div>
                        }
                    ],
                    vec![
                        view! {
                            <div class="form-actions-left">
                                <button class="btn btn-secondary" on:click={prev_step.clone()}>"Back"</button>
                            </div>
                        },
                        view! {
                            <div class="form-actions-right">
                                <button class="btn btn-secondary" on:click={handle_save.clone()} disabled={saving.get()}>
                                    {if saving.get() { "Saving..." } else { "Save Draft" }}
                                </button>
                                if is_edit {
                                    <button class="btn btn-primary" on:click={handle_submit_for_approval} disabled={saving.get()}>
                                        "Submit for Approval"
                                    </button>
                                } else {
                                    <button class="btn btn-primary" on:click={handle_save.clone()} disabled={saving.get()}>
                                        "Create Contract"
                                    </button>
                                }
                            </div>
                        },
                    ]
                )}
            }
        </div>
    }
}
