//! Requisition create/edit form

use components::prelude::*;
use wasm_bindgen::JsCast;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, panel_with_footer,
    stepper, StepperItem, StepStatus,
};
use crate::shared::forms::{
    form_group, text_input, textarea, select, SelectOption,
    currency_input, file_upload, UploadedFile,
};
use crate::util::format::format_currency;
use super::store::RequisitionsStore;
use super::types::{Requisition, LineItem, Priority};
use super::service;

/// Requisition form steps
#[derive(Clone, Copy, PartialEq)]
pub enum FormStep {
    Details = 1,
    LineItems = 2,
    Attachments = 3,
    Review = 4,
}

impl FormStep {
    fn from_u32(step: u32) -> Self {
        match step {
            1 => FormStep::Details,
            2 => FormStep::LineItems,
            3 => FormStep::Attachments,
            4 => FormStep::Review,
            _ => FormStep::Details,
        }
    }
}

/// Requisition form component
#[component]
pub fn requisition_form() -> View {
    let store = use_context::<RequisitionsStore>();

    // Form step state
    let current_step = store.form_step.clone();

    // Form data signals
    let description = signal(String::new());
    let justification = signal(String::new());
    let department = signal(String::new());
    let cost_center = signal(String::new());
    let priority = signal("medium".to_string());
    let required_by = signal(String::new());
    let delivery_address = signal(String::new());
    let notes = signal(String::new());

    // Line items state
    let line_items: Signal<Vec<LineItem>> = signal(vec![LineItem::new()]);

    // Attachments state
    let attachments: Signal<Vec<UploadedFile>> = signal(Vec::new());

    // Validation errors
    let errors: Signal<Vec<(String, String)>> = signal(Vec::new());

    // Build stepper items
    let step = current_step.get();
    let stepper_items = vec![
        StepperItem {
            number: 1,
            label: "Details".to_string(),
            status: if step > 1 { StepStatus::Completed } else if step == 1 { StepStatus::Active } else { StepStatus::Pending },
        },
        StepperItem {
            number: 2,
            label: "Line Items".to_string(),
            status: if step > 2 { StepStatus::Completed } else if step == 2 { StepStatus::Active } else { StepStatus::Pending },
        },
        StepperItem {
            number: 3,
            label: "Attachments".to_string(),
            status: if step > 3 { StepStatus::Completed } else if step == 3 { StepStatus::Active } else { StepStatus::Pending },
        },
        StepperItem {
            number: 4,
            label: "Review".to_string(),
            status: if step == 4 { StepStatus::Active } else { StepStatus::Pending },
        },
    ];

    // Step navigation handlers
    let on_step_click = {
        let current_step = current_step.clone();
        Callback::new(move |step: u32| {
            // Allow going back, or forward if current step is valid
            if step <= current_step.get() {
                current_step.set(step);
            }
        })
    };

    let on_next = {
        let current_step = current_step.clone();
        Callback::<()>::new(move |_| {
            let step = current_step.get();
            if step < 4 {
                current_step.set(step + 1);
            }
        })
    };

    let on_prev = {
        let current_step = current_step.clone();
        Callback::<()>::new(move |_| {
            let step = current_step.get();
            if step > 1 {
                current_step.set(step - 1);
            }
        })
    };

    // Add line item handler
    let on_add_line_item = {
        let line_items = line_items.clone();
        Callback::<()>::new(move |_| {
            let mut items = line_items.get();
            items.push(LineItem::new());
            line_items.set(items);
        })
    };

    // Remove line item handler
    let on_remove_line_item = {
        let line_items = line_items.clone();
        Callback::new(move |idx: usize| {
            let mut items = line_items.get();
            if items.len() > 1 {
                items.remove(idx);
                line_items.set(items);
            }
        })
    };

    // File remove handler
    let on_file_remove = {
        let attachments = attachments.clone();
        Callback::new(move |idx: usize| {
            let mut files = attachments.get();
            files.remove(idx);
            attachments.set(files);
        })
    };

    // Save draft handler
    let on_save_draft = {
        let store = store.clone();
        let description = description.clone();
        let justification = justification.clone();
        let department = department.clone();
        let cost_center = cost_center.clone();
        let priority = priority.clone();
        let required_by = required_by.clone();
        let delivery_address = delivery_address.clone();
        let notes = notes.clone();
        let line_items = line_items.clone();
        Callback::<()>::new(move |_| {
            let mut requisition = Requisition::default();
            requisition.description = description.get();
            requisition.justification = justification.get();
            requisition.department = department.get();
            requisition.cost_center = cost_center.get();
            requisition.priority = match priority.get().as_str() {
                "low" => Priority::Low,
                "high" => Priority::High,
                "urgent" => Priority::Urgent,
                _ => Priority::Medium,
            };
            requisition.required_by = if required_by.get().is_empty() { None } else { Some(required_by.get()) };
            requisition.delivery_address = delivery_address.get();
            requisition.notes = if notes.get().is_empty() { None } else { Some(notes.get()) };
            requisition.line_items = line_items.get();
            requisition.requester = "Current User".to_string();
            requisition.requester_email = "user@gov.za".to_string();

            let store = store.clone();
            spawn(async move {
                match service::create_requisition(&store, requisition).await {
                    Ok(req) => {
                        web_sys::console::log_1(&format!("Requisition {} saved as draft", req.id).into());
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Error saving requisition: {}", e).into());
                    }
                }
            });
        })
    };

    // Submit handler
    let on_submit = {
        let on_save_draft = on_save_draft.clone();
        Callback::<()>::new(move |_| {
            // First save, then submit
            on_save_draft.call(());
            web_sys::console::log_1(&"Requisition submitted for approval".into());
        })
    };

    // Department options
    let department_options = vec![
        SelectOption { value: "Information Technology".to_string(), label: "Information Technology".to_string() },
        SelectOption { value: "Administration".to_string(), label: "Administration".to_string() },
        SelectOption { value: "Facilities".to_string(), label: "Facilities".to_string() },
        SelectOption { value: "Transport".to_string(), label: "Transport".to_string() },
        SelectOption { value: "Human Resources".to_string(), label: "Human Resources".to_string() },
        SelectOption { value: "Health Services".to_string(), label: "Health Services".to_string() },
        SelectOption { value: "Finance".to_string(), label: "Finance".to_string() },
    ];

    let priority_options = vec![
        SelectOption { value: "low".to_string(), label: "Low".to_string() },
        SelectOption { value: "medium".to_string(), label: "Medium".to_string() },
        SelectOption { value: "high".to_string(), label: "High".to_string() },
        SelectOption { value: "urgent".to_string(), label: "Urgent".to_string() },
    ];

    // Calculate total
    let total: f64 = line_items.get().iter().map(|li| li.total).sum();

    view! {
        style {
            r#"
            .requisition-form { display: flex; flex-direction: column; gap: var(--space-4); }
            .form-actions {
                display: flex;
                justify-content: space-between;
                gap: 12px;
                padding-top: 16px;
                border-top: 1px solid var(--border);
                margin-top: 16px;
            }
            .form-actions-left {
                display: flex;
                gap: 12px;
            }
            .form-actions-right {
                display: flex;
                gap: 12px;
            }
            .line-items-table {
                width: 100%;
                border-collapse: collapse;
            }
            .line-items-table th,
            .line-items-table td {
                padding: 12px;
                text-align: left;
                border-bottom: 1px solid var(--border);
            }
            .line-items-table th {
                background: var(--bg);
                font-size: 12px;
                font-weight: 500;
                color: var(--text-muted);
                text-transform: uppercase;
            }
            .line-items-table input,
            .line-items-table select {
                width: 100%;
                padding: 8px;
                border: 1px solid var(--border);
                border-radius: var(--radius-sm);
                font-size: 13px;
            }
            .line-items-table .qty-input {
                width: 80px;
            }
            .line-items-table .price-input {
                width: 120px;
                text-align: right;
                font-family: IBM Plex Mono, monospace;
            }
            .line-items-table .total-cell {
                font-family: IBM Plex Mono, monospace;
                font-weight: 500;
                text-align: right;
            }
            .line-item-actions {
                display: flex;
                gap: 4px;
            }
            .remove-btn {
                width: 28px;
                height: 28px;
                border: none;
                background: transparent;
                color: var(--text-muted);
                cursor: pointer;
                border-radius: var(--radius-sm);
                font-size: 16px;
            }
            .remove-btn:hover {
                color: var(--red);
                background: var(--red-light);
            }
            .add-line-item {
                margin-top: 12px;
            }
            .line-items-total {
                display: flex;
                justify-content: flex-end;
                padding: 16px;
                background: var(--bg);
                border-radius: var(--radius);
                margin-top: 16px;
            }
            .line-items-total-label {
                font-weight: 500;
                margin-right: 24px;
            }
            .line-items-total-value {
                font-family: IBM Plex Mono, monospace;
                font-size: 18px;
                font-weight: 600;
                color: var(--navy);
            }
            .review-section {
                margin-bottom: 24px;
            }
            .review-section-title {
                font-weight: 600;
                font-size: 14px;
                color: var(--navy);
                margin-bottom: 12px;
                padding-bottom: 8px;
                border-bottom: 1px solid var(--border);
            }
            .review-grid {
                display: grid;
                grid-template-columns: repeat(2, 1fr);
                gap: 16px;
            }
            .review-item {
                display: flex;
                flex-direction: column;
                gap: 4px;
            }
            .review-item-label {
                font-size: 12px;
                color: var(--text-muted);
            }
            .review-item-value {
                font-size: 14px;
                font-weight: 500;
            }
            .review-item-value.amount {
                font-family: IBM Plex Mono, monospace;
            }
            "#
        }

        <div class="requisition-form" data-testid="requisition-form">
            {page_header(
                "New Requisition".to_string(),
                Some("Create a new purchase requisition".to_string()),
                vec![
                    view! { <a href="/requisitions" class="btn btn-secondary">"Cancel"</a> },
                ]
            )}

            // Stepper
            {stepper(stepper_items, Some(on_step_click))}

            // Step 1: Details
            if current_step.get() == 1 {
                {panel_with_footer(
                    "Requisition Details".to_string(),
                    vec![],
                    vec![
                        form_group(
                            Some("Basic Information".to_string()),
                            2,
                            vec![
                                text_input(
                                    "Description".to_string(),
                                    description.clone(),
                                    Some("Brief description of the requisition".to_string()),
                                    true,
                                    false,
                                    None,
                                    None,
                                    None,
                                ),
                                select(
                                    "Priority".to_string(),
                                    priority.clone(),
                                    priority_options,
                                    None,
                                    true,
                                    false,
                                    None,
                                ),
                            ]
                        ),
                        form_group(
                            None,
                            1,
                            vec![
                                textarea(
                                    "Justification".to_string(),
                                    justification.clone(),
                                    Some("Explain why this requisition is needed...".to_string()),
                                    true,
                                    false,
                                    Some(4),
                                    None,
                                    Some("Provide detailed justification for the procurement".to_string()),
                                ),
                            ]
                        ),
                        form_group(
                            Some("Department & Budget".to_string()),
                            2,
                            vec![
                                select(
                                    "Department".to_string(),
                                    department.clone(),
                                    department_options,
                                    Some("Select department".to_string()),
                                    true,
                                    false,
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
                        ),
                        form_group(
                            Some("Delivery Information".to_string()),
                            2,
                            vec![
                                text_input(
                                    "Required By".to_string(),
                                    required_by.clone(),
                                    Some("YYYY-MM-DD".to_string()),
                                    false,
                                    false,
                                    None,
                                    None,
                                    Some("date".to_string()),
                                ),
                                text_input(
                                    "Delivery Address".to_string(),
                                    delivery_address.clone(),
                                    Some("Delivery location".to_string()),
                                    true,
                                    false,
                                    None,
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
                                    "Additional Notes".to_string(),
                                    notes.clone(),
                                    Some("Any additional information...".to_string()),
                                    false,
                                    false,
                                    Some(3),
                                    None,
                                    None,
                                ),
                            ]
                        ),
                    ],
                    vec![
                        view! { <button class="btn btn-secondary" on:click={on_save_draft.clone()}>"Save Draft"</button> },
                        view! { <button class="btn btn-primary" on:click={on_next.clone()}>"Continue to Line Items"</button> },
                    ]
                )}
            }

            // Step 2: Line Items
            if current_step.get() == 2 {
                {panel_with_footer(
                    "Line Items".to_string(),
                    vec![],
                    vec![
                        view! {
                            <table class="line-items-table">
                                <thead>
                                    <tr>
                                        <th style="width: 30%;">"Description"</th>
                                        <th style="width: 15%;">"Category"</th>
                                        <th style="width: 10%;">"Qty"</th>
                                        <th style="width: 10%;">"Unit"</th>
                                        <th style="width: 15%;">"Unit Price"</th>
                                        <th style="width: 15%;">"Total"</th>
                                        <th style="width: 5%;"></th>
                                    </tr>
                                </thead>
                                <tbody>
                                    for (idx, item) in line_items.get().iter().enumerate() {
                                        {line_item_row(idx, item.clone(), line_items.clone(), on_remove_line_item.clone())}
                                    }
                                </tbody>
                            </table>
                        },
                        view! {
                            <div class="add-line-item">
                                <button class="btn btn-secondary" on:click={on_add_line_item.clone()}>
                                    "+ Add Line Item"
                                </button>
                            </div>
                        },
                        view! {
                            <div class="line-items-total">
                                <span class="line-items-total-label">"Total Amount:"</span>
                                <span class="line-items-total-value">{format_currency(total)}</span>
                            </div>
                        },
                    ],
                    vec![
                        view! { <button class="btn btn-secondary" on:click={on_prev.clone()}>"Back"</button> },
                        view! { <button class="btn btn-secondary" on:click={on_save_draft.clone()}>"Save Draft"</button> },
                        view! { <button class="btn btn-primary" on:click={on_next.clone()}>"Continue to Attachments"</button> },
                    ]
                )}
            }

            // Step 3: Attachments
            if current_step.get() == 3 {
                {panel_with_footer(
                    "Attachments".to_string(),
                    vec![],
                    vec![
                        view! {
                            <p style="color: var(--text-muted); margin-bottom: 16px;">
                                "Upload supporting documents such as specifications, quotes, or budget approvals."
                            </p>
                        },
                        file_upload(
                            "Supporting Documents".to_string(),
                            attachments.clone(),
                            Some(".pdf,.doc,.docx,.xls,.xlsx,.jpg,.png".to_string()),
                            true,
                            false,
                            Some("PDF, Word, Excel, or images. Max 10MB per file.".to_string()),
                            on_file_remove.clone(),
                        ),
                    ],
                    vec![
                        view! { <button class="btn btn-secondary" on:click={on_prev.clone()}>"Back"</button> },
                        view! { <button class="btn btn-secondary" on:click={on_save_draft.clone()}>"Save Draft"</button> },
                        view! { <button class="btn btn-primary" on:click={on_next.clone()}>"Continue to Review"</button> },
                    ]
                )}
            }

            // Step 4: Review
            if current_step.get() == 4 {
                {panel_with_footer(
                    "Review & Submit".to_string(),
                    vec![],
                    vec![
                        view! {
                            <div class="review-section">
                                <div class="review-section-title">"Requisition Details"</div>
                                <div class="review-grid">
                                    <div class="review-item">
                                        <span class="review-item-label">"Description"</span>
                                        <span class="review-item-value">{description.get()}</span>
                                    </div>
                                    <div class="review-item">
                                        <span class="review-item-label">"Priority"</span>
                                        <span class="review-item-value">{priority.get()}</span>
                                    </div>
                                    <div class="review-item">
                                        <span class="review-item-label">"Department"</span>
                                        <span class="review-item-value">{department.get()}</span>
                                    </div>
                                    <div class="review-item">
                                        <span class="review-item-label">"Cost Center"</span>
                                        <span class="review-item-value">{cost_center.get()}</span>
                                    </div>
                                    <div class="review-item">
                                        <span class="review-item-label">"Required By"</span>
                                        <span class="review-item-value">{required_by.get()}</span>
                                    </div>
                                    <div class="review-item">
                                        <span class="review-item-label">"Delivery Address"</span>
                                        <span class="review-item-value">{delivery_address.get()}</span>
                                    </div>
                                </div>
                            </div>
                        },
                        view! {
                            <div class="review-section">
                                <div class="review-section-title">"Line Items Summary"</div>
                                <table class="line-items-table">
                                    <thead>
                                        <tr>
                                            <th>"Description"</th>
                                            <th>"Qty"</th>
                                            <th>"Unit Price"</th>
                                            <th>"Total"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        for item in line_items.get().iter() {
                                            <tr>
                                                <td>{item.description.clone()}</td>
                                                <td>{format!("{} {}", item.quantity, item.unit)}</td>
                                                <td class="price-input">{format_currency(item.unit_price)}</td>
                                                <td class="total-cell">{format_currency(item.total)}</td>
                                            </tr>
                                        }
                                    </tbody>
                                </table>
                                <div class="line-items-total">
                                    <span class="line-items-total-label">"Total Amount:"</span>
                                    <span class="line-items-total-value">{format_currency(total)}</span>
                                </div>
                            </div>
                        },
                        view! {
                            <div class="review-section">
                                <div class="review-section-title">"Attachments"</div>
                                if attachments.get().is_empty() {
                                    <p style="color: var(--text-muted);">"No attachments added"</p>
                                } else {
                                    <ul>
                                        for file in attachments.get().iter() {
                                            <li>{file.name.clone()}</li>
                                        }
                                    </ul>
                                }
                            </div>
                        },
                        view! {
                            <div class="review-section">
                                <div class="review-section-title">"Justification"</div>
                                <p>{justification.get()}</p>
                            </div>
                        },
                    ],
                    vec![
                        view! { <button class="btn btn-secondary" on:click={on_prev.clone()}>"Back"</button> },
                        view! { <button class="btn btn-secondary" on:click={on_save_draft.clone()}>"Save Draft"</button> },
                        view! { <button class="btn btn-primary" on:click={on_submit.clone()}>"Submit for Approval"</button> },
                    ]
                )}
            }
        </div>
    }
}

/// Render a single line item row
fn line_item_row(
    idx: usize,
    item: LineItem,
    line_items: Signal<Vec<LineItem>>,
    on_remove: Callback<usize>,
) -> View {
    let on_description_change = {
        let line_items = line_items.clone();
        Callback::new(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let mut items = line_items.get();
            if idx < items.len() {
                items[idx].description = input.value();
                line_items.set(items);
            }
        })
    };

    let on_category_change = {
        let line_items = line_items.clone();
        Callback::new(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            let mut items = line_items.get();
            if idx < items.len() {
                items[idx].category = select.value();
                line_items.set(items);
            }
        })
    };

    let on_qty_change = {
        let line_items = line_items.clone();
        Callback::new(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let mut items = line_items.get();
            if idx < items.len() {
                items[idx].quantity = input.value().parse().unwrap_or(1);
                items[idx].calculate_total();
                line_items.set(items);
            }
        })
    };

    let on_unit_change = {
        let line_items = line_items.clone();
        Callback::new(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            let mut items = line_items.get();
            if idx < items.len() {
                items[idx].unit = select.value();
                line_items.set(items);
            }
        })
    };

    let on_price_change = {
        let line_items = line_items.clone();
        Callback::new(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let mut items = line_items.get();
            if idx < items.len() {
                items[idx].unit_price = input.value().replace(",", "").parse().unwrap_or(0.0);
                items[idx].calculate_total();
                line_items.set(items);
            }
        })
    };

    let handle_remove = {
        let on_remove = on_remove.clone();
        Callback::<()>::new(move |_| {
            on_remove.call(idx);
        })
    };

    view! {
        <tr>
            <td>
                <input
                    type="text"
                    value={item.description.clone()}
                    placeholder="Item description"
                    on:input={on_description_change}
                />
            </td>
            <td>
                <select value={item.category.clone()} on:change={on_category_change}>
                    <option value="">"Select..."</option>
                    <option value="IT Equipment">"IT Equipment"</option>
                    <option value="Software">"Software"</option>
                    <option value="Office Supplies">"Office Supplies"</option>
                    <option value="Furniture">"Furniture"</option>
                    <option value="Services">"Services"</option>
                    <option value="Fleet Services">"Fleet Services"</option>
                    <option value="Training">"Training"</option>
                    <option value="Medical">"Medical"</option>
                    <option value="Other">"Other"</option>
                </select>
            </td>
            <td>
                <input
                    type="number"
                    class="qty-input"
                    value={item.quantity.to_string()}
                    min="1"
                    on:input={on_qty_change}
                />
            </td>
            <td>
                <select value={item.unit.clone()} on:change={on_unit_change}>
                    <option value="Each">"Each"</option>
                    <option value="Box">"Box"</option>
                    <option value="Pack">"Pack"</option>
                    <option value="License">"License"</option>
                    <option value="Month">"Month"</option>
                    <option value="Service">"Service"</option>
                    <option value="Tyre">"Tyre"</option>
                    <option value="Participant">"Participant"</option>
                    <option value="Kit">"Kit"</option>
                    <option value="Container">"Container"</option>
                </select>
            </td>
            <td>
                <input
                    type="text"
                    class="price-input"
                    value={format!("{:.2}", item.unit_price)}
                    on:input={on_price_change}
                />
            </td>
            <td class="total-cell">
                {format_currency(item.total)}
            </td>
            <td>
                <div class="line-item-actions">
                    <button class="remove-btn" on:click={handle_remove} title="Remove">
                        "×"
                    </button>
                </div>
            </td>
        </tr>
    }
}
