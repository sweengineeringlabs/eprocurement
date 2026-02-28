//! Purchase Order create/edit form

use components::prelude::*;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, panel_with_footer,
    stepper, StepperItem, StepStatus,
    notice_bar, NoticeType,
};
use crate::shared::forms::{
    text_input, textarea, select, SelectOption, date_picker, currency_input,
};
use crate::util::format::format_currency;
use super::types::{
    PurchaseOrder, PurchaseOrderStatus, LineItem, DeliveryAddress, Supplier,
};
use super::store::PurchaseOrdersStore;
use super::service;

/// Purchase Order form page (create/edit)
#[component]
pub fn po_form(po_id: Option<String>) -> View {
    let store = use_context::<PurchaseOrdersStore>();
    let is_edit = po_id.is_some();

    // Form step state
    let current_step = signal(1u32);

    // Supplier (Step 1)
    let supplier_id = signal(String::new());
    let supplier_name = signal(String::new());
    let supplier_contact = signal(String::new());
    let supplier_email = signal(String::new());
    let supplier_phone = signal(String::new());
    let contract_ref = signal(String::new());
    let requisition_ref = signal(String::new());

    // Line Items (Step 2)
    let line_items: Signal<Vec<LineItem>> = signal(Vec::new());

    // Delivery Address (Step 3)
    let address_line1 = signal(String::new());
    let address_line2 = signal(String::new());
    let city = signal(String::new());
    let province = signal(String::new());
    let postal_code = signal(String::new());
    let contact_person = signal(String::new());
    let contact_phone = signal(String::new());
    let contact_email = signal(String::new());
    let delivery_instructions = signal(String::new());
    let expected_delivery_date = signal(String::new());

    // Additional Info (Step 4)
    let payment_terms = signal("30 days from invoice".to_string());
    let notes = signal(String::new());
    let internal_notes = signal(String::new());

    // Form state
    let form_error = signal::<Option<String>>(None);
    let saving = store.saving.clone();

    // Load existing PO if editing
    if let Some(id) = &po_id {
        effect({
            let store = store.clone();
            let id = id.clone();
            let supplier_id = supplier_id.clone();
            let supplier_name = supplier_name.clone();
            let supplier_contact = supplier_contact.clone();
            let supplier_email = supplier_email.clone();
            let supplier_phone = supplier_phone.clone();
            let contract_ref = contract_ref.clone();
            let line_items = line_items.clone();
            let address_line1 = address_line1.clone();
            let city = city.clone();
            let province = province.clone();
            let postal_code = postal_code.clone();
            let contact_person = contact_person.clone();
            let contact_phone = contact_phone.clone();
            let contact_email = contact_email.clone();
            let delivery_instructions = delivery_instructions.clone();
            let expected_delivery_date = expected_delivery_date.clone();
            let payment_terms = payment_terms.clone();
            let notes = notes.clone();

            move || {
                let store = store.clone();
                let id = id.clone();
                let supplier_id = supplier_id.clone();
                let supplier_name = supplier_name.clone();
                let supplier_contact = supplier_contact.clone();
                let supplier_email = supplier_email.clone();
                let supplier_phone = supplier_phone.clone();
                let contract_ref = contract_ref.clone();
                let line_items = line_items.clone();
                let address_line1 = address_line1.clone();
                let city = city.clone();
                let province = province.clone();
                let postal_code = postal_code.clone();
                let contact_person = contact_person.clone();
                let contact_phone = contact_phone.clone();
                let contact_email = contact_email.clone();
                let delivery_instructions = delivery_instructions.clone();
                let expected_delivery_date = expected_delivery_date.clone();
                let payment_terms = payment_terms.clone();
                let notes = notes.clone();

                spawn(async move {
                    service::load_purchase_order(&store, &id).await;

                    if let Some(po) = store.selected.get() {
                        supplier_id.set(po.supplier.id);
                        supplier_name.set(po.supplier.name);
                        supplier_contact.set(po.supplier.contact_person);
                        supplier_email.set(po.supplier.contact_email);
                        supplier_phone.set(po.supplier.contact_phone);
                        contract_ref.set(po.contract_ref.unwrap_or_default());
                        line_items.set(po.line_items);
                        address_line1.set(po.delivery_address.address_line1);
                        city.set(po.delivery_address.city);
                        province.set(po.delivery_address.province);
                        postal_code.set(po.delivery_address.postal_code);
                        contact_person.set(po.delivery_address.contact_person);
                        contact_phone.set(po.delivery_address.contact_phone);
                        contact_email.set(po.delivery_address.contact_email);
                        delivery_instructions.set(po.delivery_address.delivery_instructions.unwrap_or_default());
                        expected_delivery_date.set(po.expected_delivery_date);
                        payment_terms.set(po.payment_terms);
                        notes.set(po.notes.unwrap_or_default());
                    }
                });
            }
        });
    }

    // Step navigation
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

    // Add line item
    let add_line_item: Callback<()> = Callback::new({
        let line_items = line_items.clone();
        move |_| {
            let mut items = line_items.get();
            items.push(LineItem {
                id: format!("LI-{:03}", items.len() + 1),
                ..Default::default()
            });
            line_items.set(items);
        }
    });

    // Remove line item
    let remove_line_item = {
        let line_items = line_items.clone();
        move |idx: usize| {
            let mut items = line_items.get();
            if idx < items.len() {
                items.remove(idx);
                line_items.set(items);
            }
        }
    };

    // Update line item field
    let update_line_item = {
        let line_items = line_items.clone();
        move |idx: usize, field: &str, value: String| {
            let mut items = line_items.get();
            if idx < items.len() {
                match field {
                    "item_code" => items[idx].item_code = value,
                    "description" => items[idx].description = value,
                    "quantity" => {
                        items[idx].quantity = value.parse().unwrap_or(1);
                        items[idx].calculate_totals();
                    }
                    "unit" => items[idx].unit = value,
                    "unit_price" => {
                        items[idx].unit_price = value.parse().unwrap_or(0.0);
                        items[idx].calculate_totals();
                    }
                    "delivery_date" => items[idx].delivery_date = value,
                    "notes" => items[idx].notes = if value.is_empty() { None } else { Some(value) },
                    _ => {}
                }
                line_items.set(items);
            }
        }
    };

    // Calculate totals
    let calculate_totals = {
        let line_items = line_items.clone();
        move || {
            let items = line_items.get();
            let subtotal: f64 = items.iter().map(|i| i.total_price).sum();
            let tax_total: f64 = items.iter().map(|i| i.tax_amount).sum();
            (subtotal, tax_total, subtotal + tax_total)
        }
    };

    // Save purchase order
    let handle_save: Callback<()> = Callback::new({
        let store = store.clone();
        let po_id = po_id.clone();
        let supplier_id = supplier_id.clone();
        let supplier_name = supplier_name.clone();
        let supplier_contact = supplier_contact.clone();
        let supplier_email = supplier_email.clone();
        let supplier_phone = supplier_phone.clone();
        let contract_ref = contract_ref.clone();
        let requisition_ref = requisition_ref.clone();
        let line_items = line_items.clone();
        let address_line1 = address_line1.clone();
        let address_line2 = address_line2.clone();
        let city = city.clone();
        let province = province.clone();
        let postal_code = postal_code.clone();
        let contact_person = contact_person.clone();
        let contact_phone = contact_phone.clone();
        let contact_email = contact_email.clone();
        let delivery_instructions = delivery_instructions.clone();
        let expected_delivery_date = expected_delivery_date.clone();
        let payment_terms = payment_terms.clone();
        let notes = notes.clone();
        let internal_notes = internal_notes.clone();
        let form_error = form_error.clone();
        let calculate_totals = calculate_totals.clone();

        move |_| {
            let store = store.clone();
            let po_id = po_id.clone();
            let form_error = form_error.clone();

            let (subtotal, tax_total, total) = calculate_totals();

            let po = PurchaseOrder {
                id: po_id.clone().unwrap_or_default(),
                po_number: po_id.clone().unwrap_or_default(),
                contract_ref: {
                    let cr = contract_ref.get();
                    if cr.is_empty() { None } else { Some(cr) }
                },
                requisition_ref: {
                    let rr = requisition_ref.get();
                    if rr.is_empty() { None } else { Some(rr) }
                },
                tender_ref: None,
                supplier: Supplier {
                    id: supplier_id.get(),
                    name: supplier_name.get(),
                    registration_number: String::new(),
                    tax_number: String::new(),
                    bbbee_level: 1,
                    contact_person: supplier_contact.get(),
                    contact_email: supplier_email.get(),
                    contact_phone: supplier_phone.get(),
                    address: String::new(),
                },
                line_items: line_items.get(),
                delivery_address: DeliveryAddress {
                    address_line1: address_line1.get(),
                    address_line2: {
                        let a2 = address_line2.get();
                        if a2.is_empty() { None } else { Some(a2) }
                    },
                    city: city.get(),
                    province: province.get(),
                    postal_code: postal_code.get(),
                    country: "South Africa".to_string(),
                    contact_person: contact_person.get(),
                    contact_phone: contact_phone.get(),
                    contact_email: contact_email.get(),
                    delivery_instructions: {
                        let di = delivery_instructions.get();
                        if di.is_empty() { None } else { Some(di) }
                    },
                },
                status: PurchaseOrderStatus::Draft,
                subtotal,
                tax_total,
                total_amount: total,
                currency: "ZAR".to_string(),
                payment_terms: payment_terms.get(),
                order_date: String::new(),
                expected_delivery_date: expected_delivery_date.get(),
                actual_delivery_date: None,
                notes: {
                    let n = notes.get();
                    if n.is_empty() { None } else { Some(n) }
                },
                internal_notes: {
                    let in_ = internal_notes.get();
                    if in_.is_empty() { None } else { Some(in_) }
                },
                attachments: Vec::new(),
                created_by: "Current User".to_string(),
                created_at: String::new(),
                updated_at: String::new(),
                approved_by: None,
                approved_at: None,
                sent_at: None,
                acknowledged_at: None,
            };

            spawn(async move {
                let result = if po_id.is_some() {
                    service::update_purchase_order(&store, po).await
                } else {
                    service::create_purchase_order(&store, po).await.map(|_| ())
                };

                match result {
                    Ok(_) => {
                        // Navigate to PO list
                        // router.push("/purchase-orders");
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
        let po_id = po_id.clone();
        let form_error = form_error.clone();

        move |_| {
            if let Some(id) = &po_id {
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
            label: "Supplier".to_string(),
            status: if step > 1 { StepStatus::Completed } else if step == 1 { StepStatus::Active } else { StepStatus::Pending },
        },
        StepperItem {
            number: 2,
            label: "Line Items".to_string(),
            status: if step > 2 { StepStatus::Completed } else if step == 2 { StepStatus::Active } else { StepStatus::Pending },
        },
        StepperItem {
            number: 3,
            label: "Delivery".to_string(),
            status: if step > 3 { StepStatus::Completed } else if step == 3 { StepStatus::Active } else { StepStatus::Pending },
        },
        StepperItem {
            number: 4,
            label: "Review".to_string(),
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
        ("SUP-006", "Gourmet Corporate Catering"),
        ("SUP-007", "Skills Development Academy"),
        ("SUP-008", "CleanCorp Services"),
    ];

    // Province options
    let province_options = vec![
        ("GP", "Gauteng"),
        ("WC", "Western Cape"),
        ("KZN", "KwaZulu-Natal"),
        ("EC", "Eastern Cape"),
        ("FS", "Free State"),
        ("MP", "Mpumalanga"),
        ("NW", "North West"),
        ("LP", "Limpopo"),
        ("NC", "Northern Cape"),
    ];

    let (subtotal, tax_total, total) = calculate_totals();

    view! {
        style {
            r#"
            .po-form { display: flex; flex-direction: column; gap: var(--space-6); }
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

            .line-items-list { display: flex; flex-direction: column; gap: 16px; }
            .line-item {
                padding: 16px;
                background: var(--bg);
                border-radius: var(--radius);
                border: 1px solid var(--border);
            }
            .line-item-header {
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin-bottom: 16px;
            }
            .line-item-header h4 {
                font-size: 14px;
                font-weight: 600;
            }
            .line-item-grid {
                display: grid;
                grid-template-columns: 1fr 2fr 100px 100px 120px 120px;
                gap: 12px;
                align-items: end;
            }
            .line-item-totals {
                display: flex;
                justify-content: flex-end;
                gap: 16px;
                margin-top: 12px;
                padding-top: 12px;
                border-top: 1px solid var(--border);
                font-size: 13px;
            }
            .line-item-totals .total-label { color: var(--text-muted); }
            .line-item-totals .total-value {
                font-weight: 600;
                font-family: IBM Plex Mono, monospace;
            }

            .add-line-item-btn {
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
            .add-line-item-btn:hover {
                border-color: var(--blue);
                color: var(--blue);
            }

            .totals-section {
                display: flex;
                flex-direction: column;
                gap: 8px;
                padding: 16px;
                background: var(--bg);
                border-radius: var(--radius);
                margin-top: 16px;
            }
            .totals-row {
                display: flex;
                justify-content: space-between;
                font-size: 13px;
            }
            .totals-row.total {
                font-size: 16px;
                font-weight: 600;
                padding-top: 8px;
                border-top: 1px solid var(--border);
            }
            .totals-row .label { color: var(--text-muted); }
            .totals-row .value {
                font-family: IBM Plex Mono, monospace;
                font-weight: 500;
            }

            .review-section { margin-bottom: 24px; }
            .review-section h4 {
                font-size: 14px;
                font-weight: 600;
                margin-bottom: 12px;
                color: var(--text-muted);
            }
            .review-grid {
                display: grid;
                grid-template-columns: 140px 1fr;
                gap: 8px 16px;
                font-size: 13px;
            }
            .review-grid .label { color: var(--text-muted); }
            .review-grid .value { font-weight: 500; }
            "#
        }

        <div class="po-form" data-testid="po-form">
            {page_header(
                if is_edit { "Edit Purchase Order".to_string() } else { "New Purchase Order".to_string() },
                Some(if is_edit { "Update purchase order details".to_string() } else { "Create a new purchase order".to_string() }),
                vec![
                    view! { <a href="/purchase-orders" class="btn btn-secondary">"Cancel"</a> },
                ]
            )}

            // Error notice
            if let Some(error) = form_error.get() {
                {notice_bar(error, NoticeType::Error, Some(Callback::<()>::new({
                    let form_error = form_error.clone();
                    move |_| form_error.set(None)
                })))}
            }

            // Stepper
            {stepper(steps, Some(Callback::<u32>::new({
                let current_step = current_step.clone();
                move |step| current_step.set(step)
            })))}

            // Step 1: Supplier
            if step == 1 {
                {panel_with_footer(
                    "Supplier Information".to_string(),
                    vec![],
                    vec![
                        view! {
                            <div class="form-grid">
                                {select(
                                    "Supplier".to_string(),
                                    supplier_id.clone(),
                                    supplier_options.iter().map(|(v, l)| {
                                        SelectOption { value: v.to_string(), label: l.to_string() }
                                    }).collect(),
                                    Some("Select supplier".to_string()),
                                    true, false, None
                                )}
                                {text_input(
                                    "Contact Person".to_string(),
                                    supplier_contact.clone(),
                                    Some("Supplier contact name".to_string()),
                                    false, false, None, None, None
                                )}
                                {text_input(
                                    "Contact Email".to_string(),
                                    supplier_email.clone(),
                                    Some("supplier@example.com".to_string()),
                                    false, false, None, None, Some("email".to_string())
                                )}
                                {text_input(
                                    "Contact Phone".to_string(),
                                    supplier_phone.clone(),
                                    Some("+27 11 555 1234".to_string()),
                                    false, false, None, None, Some("tel".to_string())
                                )}
                                {text_input(
                                    "Contract Reference".to_string(),
                                    contract_ref.clone(),
                                    Some("e.g., CTR-2025-0001".to_string()),
                                    false, false, None,
                                    Some("Link to existing contract (optional)".to_string()),
                                    None
                                )}
                                {text_input(
                                    "Requisition Reference".to_string(),
                                    requisition_ref.clone(),
                                    Some("e.g., REQ-2025-0001".to_string()),
                                    false, false, None,
                                    Some("Link to requisition (optional)".to_string()),
                                    None
                                )}
                            </div>
                        }
                    ],
                    vec![
                        view! { <div class="form-actions-left"></div> },
                        view! {
                            <div class="form-actions-right">
                                <button class="btn btn-primary" on:click={next_step.clone()}>"Next: Line Items"</button>
                            </div>
                        },
                    ]
                )}
            }

            // Step 2: Line Items
            if step == 2 {
                {panel_with_footer(
                    "Line Items".to_string(),
                    vec![],
                    vec![
                        view! {
                            <div class="line-items-list">
                                for (idx, item) in line_items.get().iter().enumerate() {
                                    <div class="line-item">
                                        <div class="line-item-header">
                                            <h4>{format!("Item #{}", idx + 1)}</h4>
                                            <button
                                                class="btn btn-sm btn-danger"
                                                on:click={Callback::<()>::new({
                                                    let remove_line_item = remove_line_item.clone();
                                                    move |_| remove_line_item(idx)
                                                })}
                                            >
                                                "Remove"
                                            </button>
                                        </div>
                                        <div class="line-item-grid">
                                            {text_input(
                                                "Item Code".to_string(),
                                                signal(item.item_code.clone()),
                                                Some("SKU".to_string()),
                                                false, false, None, None, None
                                            )}
                                            {text_input(
                                                "Description".to_string(),
                                                signal(item.description.clone()),
                                                Some("Item description".to_string()),
                                                true, false, None, None, None
                                            )}
                                            {text_input(
                                                "Qty".to_string(),
                                                signal(item.quantity.to_string()),
                                                Some("1".to_string()),
                                                true, false, None, None, Some("number".to_string())
                                            )}
                                            {text_input(
                                                "Unit".to_string(),
                                                signal(item.unit.clone()),
                                                Some("Each".to_string()),
                                                true, false, None, None, None
                                            )}
                                            {currency_input(
                                                "Unit Price".to_string(),
                                                signal(item.unit_price),
                                                true, false, None, None
                                            )}
                                            <div class="line-item-total">
                                                <label>"Total"</label>
                                                <span class="total-value">{format_currency(item.total_price)}</span>
                                            </div>
                                        </div>
                                        <div class="line-item-totals">
                                            <span class="total-label">"Tax (15%):"</span>
                                            <span class="total-value">{format_currency(item.tax_amount)}</span>
                                        </div>
                                    </div>
                                }

                                <button class="add-line-item-btn" on:click={add_line_item}>
                                    <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2">
                                        <line x1="12" y1="5" x2="12" y2="19"/>
                                        <line x1="5" y1="12" x2="19" y2="12"/>
                                    </svg>
                                    "Add Line Item"
                                </button>

                                // Order totals
                                <div class="totals-section">
                                    <div class="totals-row">
                                        <span class="label">"Subtotal:"</span>
                                        <span class="value">{format_currency(subtotal)}</span>
                                    </div>
                                    <div class="totals-row">
                                        <span class="label">"VAT (15%):"</span>
                                        <span class="value">{format_currency(tax_total)}</span>
                                    </div>
                                    <div class="totals-row total">
                                        <span class="label">"Total:"</span>
                                        <span class="value">{format_currency(total)}</span>
                                    </div>
                                </div>
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
                                <button class="btn btn-primary" on:click={next_step.clone()}>"Next: Delivery"</button>
                            </div>
                        },
                    ]
                )}
            }

            // Step 3: Delivery
            if step == 3 {
                {panel_with_footer(
                    "Delivery Information".to_string(),
                    vec![],
                    vec![
                        view! {
                            <div class="form-grid">
                                <div class="span-2">
                                    {text_input(
                                        "Delivery Address Line 1".to_string(),
                                        address_line1.clone(),
                                        Some("Street address".to_string()),
                                        true, false, None, None, None
                                    )}
                                </div>
                                <div class="span-2">
                                    {text_input(
                                        "Address Line 2".to_string(),
                                        address_line2.clone(),
                                        Some("Building, floor, etc. (optional)".to_string()),
                                        false, false, None, None, None
                                    )}
                                </div>
                                {text_input(
                                    "City".to_string(),
                                    city.clone(),
                                    Some("City".to_string()),
                                    true, false, None, None, None
                                )}
                                {select(
                                    "Province".to_string(),
                                    province.clone(),
                                    province_options.iter().map(|(v, l)| {
                                        SelectOption { value: v.to_string(), label: l.to_string() }
                                    }).collect(),
                                    Some("Select province".to_string()),
                                    true, false, None
                                )}
                                {text_input(
                                    "Postal Code".to_string(),
                                    postal_code.clone(),
                                    Some("0000".to_string()),
                                    true, false, None, None, None
                                )}
                                {date_picker(
                                    "Expected Delivery Date".to_string(),
                                    expected_delivery_date.clone(),
                                    true, false, None, None, None
                                )}
                                {text_input(
                                    "Contact Person".to_string(),
                                    contact_person.clone(),
                                    Some("Receiver's name".to_string()),
                                    true, false, None, None, None
                                )}
                                {text_input(
                                    "Contact Phone".to_string(),
                                    contact_phone.clone(),
                                    Some("+27 XX XXX XXXX".to_string()),
                                    true, false, None, None, Some("tel".to_string())
                                )}
                                {text_input(
                                    "Contact Email".to_string(),
                                    contact_email.clone(),
                                    Some("receiver@example.com".to_string()),
                                    false, false, None, None, Some("email".to_string())
                                )}
                                <div></div>
                                <div class="span-2">
                                    {textarea(
                                        "Delivery Instructions".to_string(),
                                        delivery_instructions.clone(),
                                        Some("Special delivery instructions, access requirements, etc.".to_string()),
                                        false, false, Some(3), None, None
                                    )}
                                </div>
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
                                <button class="btn btn-primary" on:click={next_step.clone()}>"Next: Review"</button>
                            </div>
                        },
                    ]
                )}
            }

            // Step 4: Review & Additional Info
            if step == 4 {
                {panel_with_footer(
                    "Review & Submit".to_string(),
                    vec![],
                    vec![
                        view! {
                            <div>
                                // Supplier summary
                                <div class="review-section">
                                    <h4>"Supplier"</h4>
                                    <div class="review-grid">
                                        <span class="label">"Supplier:"</span>
                                        <span class="value">{supplier_name.get()}</span>
                                        <span class="label">"Contact:"</span>
                                        <span class="value">{supplier_contact.get()}</span>
                                        <span class="label">"Contract:"</span>
                                        <span class="value">{if contract_ref.get().is_empty() { "-".to_string() } else { contract_ref.get() }}</span>
                                    </div>
                                </div>

                                // Line items summary
                                <div class="review-section">
                                    <h4>{format!("Line Items ({})", line_items.get().len())}</h4>
                                    <div class="review-grid">
                                        <span class="label">"Subtotal:"</span>
                                        <span class="value">{format_currency(subtotal)}</span>
                                        <span class="label">"VAT:"</span>
                                        <span class="value">{format_currency(tax_total)}</span>
                                        <span class="label">"Total:"</span>
                                        <span class="value" style="font-size: 16px;">{format_currency(total)}</span>
                                    </div>
                                </div>

                                // Delivery summary
                                <div class="review-section">
                                    <h4>"Delivery"</h4>
                                    <div class="review-grid">
                                        <span class="label">"Address:"</span>
                                        <span class="value">{format!("{}, {}, {}", address_line1.get(), city.get(), province.get())}</span>
                                        <span class="label">"Contact:"</span>
                                        <span class="value">{contact_person.get()}</span>
                                        <span class="label">"Expected Date:"</span>
                                        <span class="value">{expected_delivery_date.get()}</span>
                                    </div>
                                </div>

                                // Additional information
                                <div class="form-grid" style="margin-top: 24px;">
                                    {text_input(
                                        "Payment Terms".to_string(),
                                        payment_terms.clone(),
                                        Some("e.g., 30 days from invoice".to_string()),
                                        false, false, None, None, None
                                    )}
                                    <div></div>
                                    <div class="span-2">
                                        {textarea(
                                            "Notes (visible to supplier)".to_string(),
                                            notes.clone(),
                                            Some("Any notes or comments for the supplier".to_string()),
                                            false, false, Some(3), None, None
                                        )}
                                    </div>
                                    <div class="span-2">
                                        {textarea(
                                            "Internal Notes".to_string(),
                                            internal_notes.clone(),
                                            Some("Internal notes (not visible to supplier)".to_string()),
                                            false, false, Some(3), None, None
                                        )}
                                    </div>
                                </div>
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
                                        "Create Purchase Order"
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
