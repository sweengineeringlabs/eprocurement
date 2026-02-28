//! Sourcing Plan create/edit form

use components::prelude::*;
use wasm_bindgen::JsCast;
use crate::shared::layout::page_header;
use crate::shared::components::{
    panel, panel_with_footer,
    stepper, StepperItem, StepStatus,
};
use crate::shared::forms::{
    form_group, text_input, textarea, select, SelectOption,
    currency_input, date_picker,
};
use crate::util::format::format_currency;
use super::store::SourcingPlanStore;
use super::types::{
    SourcingPlan, ProcurementCategory, CategoryPriority,
    TimelineMilestone, MilestoneStatus, BudgetAllocation,
};
use super::service;

/// Sourcing plan form steps
#[derive(Clone, Copy, PartialEq)]
pub enum FormStep {
    BasicInfo = 1,
    Categories = 2,
    Budget = 3,
    Timeline = 4,
    Review = 5,
}

impl FormStep {
    fn from_u32(step: u32) -> Self {
        match step {
            1 => FormStep::BasicInfo,
            2 => FormStep::Categories,
            3 => FormStep::Budget,
            4 => FormStep::Timeline,
            5 => FormStep::Review,
            _ => FormStep::BasicInfo,
        }
    }
}

/// Sourcing plan form component
#[component]
pub fn sourcing_form() -> View {
    let store = use_context::<SourcingPlanStore>();

    // Form step state
    let current_step = store.form_step.clone();

    // Basic info signals
    let title = signal(String::new());
    let description = signal(String::new());
    let fiscal_year = signal("2025/26".to_string());
    let department = signal(String::new());
    let start_date = signal(String::new());
    let end_date = signal(String::new());
    let notes = signal(String::new());

    // Strategic objectives
    let objectives: Signal<Vec<String>> = signal(vec![String::new()]);

    // Categories state
    let categories: Signal<Vec<ProcurementCategory>> = signal(vec![ProcurementCategory::new()]);

    // Budget state
    let total_budget = signal(0.0f64);
    let budget_currency = signal("ZAR".to_string());

    // Timeline milestones
    let milestones: Signal<Vec<TimelineMilestone>> = signal(vec![TimelineMilestone::new()]);

    // Validation errors
    let errors: Signal<Vec<(String, String)>> = signal(Vec::new());

    // Build stepper items
    let step = current_step.get();
    let stepper_items = vec![
        StepperItem {
            number: 1,
            label: "Basic Info".to_string(),
            status: if step > 1 { StepStatus::Completed } else if step == 1 { StepStatus::Active } else { StepStatus::Pending },
        },
        StepperItem {
            number: 2,
            label: "Categories".to_string(),
            status: if step > 2 { StepStatus::Completed } else if step == 2 { StepStatus::Active } else { StepStatus::Pending },
        },
        StepperItem {
            number: 3,
            label: "Budget".to_string(),
            status: if step > 3 { StepStatus::Completed } else if step == 3 { StepStatus::Active } else { StepStatus::Pending },
        },
        StepperItem {
            number: 4,
            label: "Timeline".to_string(),
            status: if step > 4 { StepStatus::Completed } else if step == 4 { StepStatus::Active } else { StepStatus::Pending },
        },
        StepperItem {
            number: 5,
            label: "Review".to_string(),
            status: if step == 5 { StepStatus::Active } else { StepStatus::Pending },
        },
    ];

    // Step navigation handlers
    let on_step_click = {
        let current_step = current_step.clone();
        Callback::new(move |step: u32| {
            if step <= current_step.get() {
                current_step.set(step);
            }
        })
    };

    let on_next = {
        let current_step = current_step.clone();
        Callback::<()>::new(move |_| {
            let step = current_step.get();
            if step < 5 {
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

    // Add category handler
    let on_add_category = {
        let categories = categories.clone();
        Callback::<()>::new(move |_| {
            let mut items = categories.get();
            items.push(ProcurementCategory::new());
            categories.set(items);
        })
    };

    // Remove category handler
    let on_remove_category = {
        let categories = categories.clone();
        Callback::new(move |idx: usize| {
            let mut items = categories.get();
            if items.len() > 1 {
                items.remove(idx);
                categories.set(items);
            }
        })
    };

    // Add milestone handler
    let on_add_milestone = {
        let milestones = milestones.clone();
        Callback::<()>::new(move |_| {
            let mut items = milestones.get();
            items.push(TimelineMilestone::new());
            milestones.set(items);
        })
    };

    // Remove milestone handler
    let on_remove_milestone = {
        let milestones = milestones.clone();
        Callback::new(move |idx: usize| {
            let mut items = milestones.get();
            if items.len() > 1 {
                items.remove(idx);
                milestones.set(items);
            }
        })
    };

    // Add objective handler
    let on_add_objective = {
        let objectives = objectives.clone();
        Callback::<()>::new(move |_| {
            let mut items = objectives.get();
            items.push(String::new());
            objectives.set(items);
        })
    };

    // Remove objective handler
    let on_remove_objective = {
        let objectives = objectives.clone();
        Callback::new(move |idx: usize| {
            let mut items = objectives.get();
            if items.len() > 1 {
                items.remove(idx);
                objectives.set(items);
            }
        })
    };

    // Save draft handler
    let on_save_draft = {
        let store = store.clone();
        let title = title.clone();
        let description = description.clone();
        let fiscal_year = fiscal_year.clone();
        let department = department.clone();
        let start_date = start_date.clone();
        let end_date = end_date.clone();
        let notes = notes.clone();
        let objectives = objectives.clone();
        let categories = categories.clone();
        let total_budget = total_budget.clone();
        let milestones = milestones.clone();
        Callback::<()>::new(move |_| {
            let mut plan = SourcingPlan::default();
            plan.title = title.get();
            plan.description = description.get();
            plan.fiscal_year = fiscal_year.get();
            plan.department = department.get();
            plan.start_date = start_date.get();
            plan.end_date = end_date.get();
            plan.notes = if notes.get().is_empty() { None } else { Some(notes.get()) };
            plan.strategic_objectives = objectives.get().into_iter()
                .filter(|o| !o.is_empty())
                .collect();
            plan.categories = categories.get();
            plan.budget = BudgetAllocation {
                total_budget: total_budget.get(),
                allocated_amount: categories.get().iter().map(|c| c.allocated_budget).sum(),
                committed_amount: 0.0,
                spent_amount: 0.0,
                currency: "ZAR".to_string(),
                fiscal_year_start: start_date.get(),
                fiscal_year_end: end_date.get(),
            };
            plan.timeline = milestones.get();
            plan.owner = "Current User".to_string();
            plan.owner_email = "user@gov.za".to_string();

            let store = store.clone();
            spawn(async move {
                match service::create_sourcing_plan(&store, plan).await {
                    Ok(p) => {
                        web_sys::console::log_1(&format!("Sourcing plan {} saved as draft", p.id).into());
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Error saving sourcing plan: {}", e).into());
                    }
                }
            });
        })
    };

    // Submit for review handler
    let on_submit = {
        let on_save_draft = on_save_draft.clone();
        Callback::<()>::new(move |_| {
            on_save_draft.call(());
            web_sys::console::log_1(&"Sourcing plan submitted for review".into());
        })
    };

    // Department options
    let department_options = vec![
        SelectOption { value: "Procurement Division".to_string(), label: "Procurement Division".to_string() },
        SelectOption { value: "Information Technology".to_string(), label: "Information Technology".to_string() },
        SelectOption { value: "Facilities".to_string(), label: "Facilities".to_string() },
        SelectOption { value: "Transport".to_string(), label: "Transport".to_string() },
        SelectOption { value: "Human Resources".to_string(), label: "Human Resources".to_string() },
        SelectOption { value: "Finance".to_string(), label: "Finance".to_string() },
        SelectOption { value: "Health Services".to_string(), label: "Health Services".to_string() },
    ];

    let fiscal_year_options = vec![
        SelectOption { value: "2025/26".to_string(), label: "2025/26".to_string() },
        SelectOption { value: "2026/27".to_string(), label: "2026/27".to_string() },
        SelectOption { value: "2027/28".to_string(), label: "2027/28".to_string() },
    ];

    let priority_options = vec![
        SelectOption { value: "low".to_string(), label: "Low".to_string() },
        SelectOption { value: "medium".to_string(), label: "Medium".to_string() },
        SelectOption { value: "high".to_string(), label: "High".to_string() },
        SelectOption { value: "critical".to_string(), label: "Critical".to_string() },
    ];

    // Calculate totals
    let categories_total: f64 = categories.get().iter().map(|c| c.allocated_budget).sum();
    let budget_remaining = total_budget.get() - categories_total;

    view! {
        style {
            r#"
            .sourcing-form { display: flex; flex-direction: column; gap: var(--space-4); }
            .form-actions {
                display: flex;
                justify-content: space-between;
                gap: 12px;
                padding-top: 16px;
                border-top: 1px solid var(--border);
                margin-top: 16px;
            }
            .categories-table,
            .milestones-table {
                width: 100%;
                border-collapse: collapse;
            }
            .categories-table th,
            .categories-table td,
            .milestones-table th,
            .milestones-table td {
                padding: 12px;
                text-align: left;
                border-bottom: 1px solid var(--border);
            }
            .categories-table th,
            .milestones-table th {
                background: var(--bg);
                font-size: 12px;
                font-weight: 500;
                color: var(--text-muted);
                text-transform: uppercase;
            }
            .categories-table input,
            .categories-table select,
            .milestones-table input,
            .milestones-table select {
                width: 100%;
                padding: 8px;
                border: 1px solid var(--border);
                border-radius: var(--radius-sm);
                font-size: 13px;
            }
            .budget-input {
                text-align: right;
                font-family: IBM Plex Mono, monospace;
            }
            .percentage-input {
                width: 80px;
                min-width: 80px;
                text-align: right;
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
            .add-btn {
                margin-top: 12px;
            }
            .budget-summary {
                display: flex;
                justify-content: flex-end;
                gap: 32px;
                padding: 16px;
                background: var(--bg);
                border-radius: var(--radius);
                margin-top: 16px;
            }
            .budget-item {
                display: flex;
                flex-direction: column;
                align-items: flex-end;
            }
            .budget-item-label {
                font-size: 12px;
                color: var(--text-muted);
            }
            .budget-item-value {
                font-family: IBM Plex Mono, monospace;
                font-size: 18px;
                font-weight: 600;
            }
            .budget-item-value.positive {
                color: var(--green);
            }
            .budget-item-value.negative {
                color: var(--red);
            }
            .objectives-list {
                display: flex;
                flex-direction: column;
                gap: 8px;
            }
            .objective-row {
                display: flex;
                gap: 8px;
                align-items: center;
            }
            .objective-row input {
                flex: 1;
                padding: 8px 12px;
                border: 1px solid var(--border);
                border-radius: var(--radius-sm);
                font-size: 13px;
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
            .review-objectives {
                list-style: disc;
                padding-left: 20px;
            }
            .review-objectives li {
                margin-bottom: 4px;
            }
            "#
        }

        <div class="sourcing-form" data-testid="sourcing-form">
            {page_header(
                "New Sourcing Plan".to_string(),
                Some("Create an annual procurement sourcing plan".to_string()),
                vec![
                    view! { <a href="/sourcing-plans" class="btn btn-secondary">"Cancel"</a> },
                ]
            )}

            // Stepper
            {stepper(stepper_items, Some(on_step_click))}

            // Step 1: Basic Info
            if current_step.get() == 1 {
                {panel_with_footer(
                    "Basic Information".to_string(),
                    vec![],
                    vec![
                        form_group(
                            Some("Plan Details".to_string()),
                            2,
                            vec![
                                text_input(
                                    "Plan Title".to_string(),
                                    title.clone(),
                                    Some("e.g., Annual Procurement Plan FY 2025/26".to_string()),
                                    true,
                                    false,
                                    None,
                                    None,
                                    None,
                                ),
                                select(
                                    "Fiscal Year".to_string(),
                                    fiscal_year.clone(),
                                    fiscal_year_options,
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
                                    "Description".to_string(),
                                    description.clone(),
                                    Some("Describe the scope and purpose of this sourcing plan...".to_string()),
                                    true,
                                    false,
                                    Some(4),
                                    None,
                                    Some("Provide a detailed description of the procurement plan".to_string()),
                                ),
                            ]
                        ),
                        form_group(
                            Some("Department & Timeline".to_string()),
                            2,
                            vec![
                                select(
                                    "Department".to_string(),
                                    department.clone(),
                                    department_options.clone(),
                                    Some("Select department".to_string()),
                                    true,
                                    false,
                                    None,
                                ),
                                text_input(
                                    "Start Date".to_string(),
                                    start_date.clone(),
                                    Some("YYYY-MM-DD".to_string()),
                                    true,
                                    false,
                                    None,
                                    None,
                                    Some("date".to_string()),
                                ),
                            ]
                        ),
                        form_group(
                            None,
                            2,
                            vec![
                                text_input(
                                    "End Date".to_string(),
                                    end_date.clone(),
                                    Some("YYYY-MM-DD".to_string()),
                                    true,
                                    false,
                                    None,
                                    None,
                                    Some("date".to_string()),
                                ),
                                view! { <div></div> },
                            ]
                        ),
                        view! {
                            <div class="form-section">
                                <label class="form-label">"Strategic Objectives"</label>
                                <div class="objectives-list">
                                    for (idx, obj) in objectives.get().iter().enumerate() {
                                        {objective_row(idx, obj.clone(), objectives.clone(), on_remove_objective.clone())}
                                    }
                                </div>
                                <button class="btn btn-secondary add-btn" on:click={on_add_objective.clone()}>
                                    "+ Add Objective"
                                </button>
                            </div>
                        },
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
                        view! { <button class="btn btn-primary" on:click={on_next.clone()}>"Continue to Categories"</button> },
                    ]
                )}
            }

            // Step 2: Categories
            if current_step.get() == 2 {
                {panel_with_footer(
                    "Procurement Categories".to_string(),
                    vec![],
                    vec![
                        view! {
                            <p style="color: var(--text-muted); margin-bottom: 16px;">
                                "Define the procurement categories and their budget allocations for this plan."
                            </p>
                        },
                        view! {
                            <table class="categories-table">
                                <thead>
                                    <tr>
                                        <th style="width: 20%;">"Category Name"</th>
                                        <th style="width: 10%;">"Code"</th>
                                        <th style="width: 18%;">"Allocated Budget"</th>
                                        <th style="width: 10%;">"Tenders"</th>
                                        <th style="width: 10%;">"Priority"</th>
                                        <th style="width: 10%;">"B-BBEE %"</th>
                                        <th style="width: 10%;">"Local %"</th>
                                        <th style="width: 5%;"></th>
                                    </tr>
                                </thead>
                                <tbody>
                                    for (idx, cat) in categories.get().iter().enumerate() {
                                        {category_row(idx, cat.clone(), categories.clone(), on_remove_category.clone())}
                                    }
                                </tbody>
                            </table>
                        },
                        view! {
                            <button class="btn btn-secondary add-btn" on:click={on_add_category.clone()}>
                                "+ Add Category"
                            </button>
                        },
                        view! {
                            <div class="budget-summary">
                                <div class="budget-item">
                                    <span class="budget-item-label">"Total Allocated"</span>
                                    <span class="budget-item-value">{format_currency(categories_total)}</span>
                                </div>
                            </div>
                        },
                    ],
                    vec![
                        view! { <button class="btn btn-secondary" on:click={on_prev.clone()}>"Back"</button> },
                        view! { <button class="btn btn-secondary" on:click={on_save_draft.clone()}>"Save Draft"</button> },
                        view! { <button class="btn btn-primary" on:click={on_next.clone()}>"Continue to Budget"</button> },
                    ]
                )}
            }

            // Step 3: Budget
            if current_step.get() == 3 {
                {panel_with_footer(
                    "Budget Allocation".to_string(),
                    vec![],
                    vec![
                        form_group(
                            Some("Total Budget".to_string()),
                            2,
                            vec![
                                currency_input(
                                    "Total Budget Amount".to_string(),
                                    total_budget.clone(),
                                    Some("Total budget for this sourcing plan".to_string()),
                                    true,
                                    false,
                                    Some("ZAR".to_string()),
                                    None,
                                ),
                                view! { <div></div> },
                            ]
                        ),
                        view! {
                            <div class="budget-summary">
                                <div class="budget-item">
                                    <span class="budget-item-label">"Total Budget"</span>
                                    <span class="budget-item-value">{format_currency(total_budget.get())}</span>
                                </div>
                                <div class="budget-item">
                                    <span class="budget-item-label">"Allocated to Categories"</span>
                                    <span class="budget-item-value">{format_currency(categories_total)}</span>
                                </div>
                                <div class="budget-item">
                                    <span class="budget-item-label">"Remaining / Unallocated"</span>
                                    <span class={format!("budget-item-value {}", if budget_remaining >= 0.0 { "positive" } else { "negative" })}>
                                        {format_currency(budget_remaining)}
                                    </span>
                                </div>
                            </div>
                        },
                        if budget_remaining < 0.0 {
                            view! {
                                <div class="alert alert-warning" style="margin-top: 16px;">
                                    "Warning: Category allocations exceed total budget by "
                                    <strong>{format_currency(-budget_remaining)}</strong>
                                </div>
                            }
                        } else {
                            view! { <div></div> }
                        },
                    ],
                    vec![
                        view! { <button class="btn btn-secondary" on:click={on_prev.clone()}>"Back"</button> },
                        view! { <button class="btn btn-secondary" on:click={on_save_draft.clone()}>"Save Draft"</button> },
                        view! { <button class="btn btn-primary" on:click={on_next.clone()}>"Continue to Timeline"</button> },
                    ]
                )}
            }

            // Step 4: Timeline
            if current_step.get() == 4 {
                {panel_with_footer(
                    "Timeline & Milestones".to_string(),
                    vec![],
                    vec![
                        view! {
                            <p style="color: var(--text-muted); margin-bottom: 16px;">
                                "Define key milestones and target dates for the procurement plan execution."
                            </p>
                        },
                        view! {
                            <table class="milestones-table">
                                <thead>
                                    <tr>
                                        <th style="width: 25%;">"Milestone Name"</th>
                                        <th style="width: 30%;">"Description"</th>
                                        <th style="width: 15%;">"Target Date"</th>
                                        <th style="width: 20%;">"Responsible Party"</th>
                                        <th style="width: 5%;"></th>
                                    </tr>
                                </thead>
                                <tbody>
                                    for (idx, ms) in milestones.get().iter().enumerate() {
                                        {milestone_row(idx, ms.clone(), milestones.clone(), on_remove_milestone.clone())}
                                    }
                                </tbody>
                            </table>
                        },
                        view! {
                            <button class="btn btn-secondary add-btn" on:click={on_add_milestone.clone()}>
                                "+ Add Milestone"
                            </button>
                        },
                    ],
                    vec![
                        view! { <button class="btn btn-secondary" on:click={on_prev.clone()}>"Back"</button> },
                        view! { <button class="btn btn-secondary" on:click={on_save_draft.clone()}>"Save Draft"</button> },
                        view! { <button class="btn btn-primary" on:click={on_next.clone()}>"Continue to Review"</button> },
                    ]
                )}
            }

            // Step 5: Review
            if current_step.get() == 5 {
                {panel_with_footer(
                    "Review & Submit".to_string(),
                    vec![],
                    vec![
                        view! {
                            <div class="review-section">
                                <div class="review-section-title">"Plan Details"</div>
                                <div class="review-grid">
                                    <div class="review-item">
                                        <span class="review-item-label">"Title"</span>
                                        <span class="review-item-value">{title.get()}</span>
                                    </div>
                                    <div class="review-item">
                                        <span class="review-item-label">"Fiscal Year"</span>
                                        <span class="review-item-value">{fiscal_year.get()}</span>
                                    </div>
                                    <div class="review-item">
                                        <span class="review-item-label">"Department"</span>
                                        <span class="review-item-value">{department.get()}</span>
                                    </div>
                                    <div class="review-item">
                                        <span class="review-item-label">"Period"</span>
                                        <span class="review-item-value">{format!("{} to {}", start_date.get(), end_date.get())}</span>
                                    </div>
                                </div>
                            </div>
                        },
                        view! {
                            <div class="review-section">
                                <div class="review-section-title">"Strategic Objectives"</div>
                                <ul class="review-objectives">
                                    for obj in objectives.get().iter().filter(|o| !o.is_empty()) {
                                        <li>{obj.clone()}</li>
                                    }
                                </ul>
                            </div>
                        },
                        view! {
                            <div class="review-section">
                                <div class="review-section-title">"Budget Summary"</div>
                                <div class="review-grid">
                                    <div class="review-item">
                                        <span class="review-item-label">"Total Budget"</span>
                                        <span class="review-item-value amount">{format_currency(total_budget.get())}</span>
                                    </div>
                                    <div class="review-item">
                                        <span class="review-item-label">"Allocated"</span>
                                        <span class="review-item-value amount">{format_currency(categories_total)}</span>
                                    </div>
                                </div>
                            </div>
                        },
                        view! {
                            <div class="review-section">
                                <div class="review-section-title">"Categories ({} total)"</div>
                                <table class="categories-table">
                                    <thead>
                                        <tr>
                                            <th>"Category"</th>
                                            <th>"Budget"</th>
                                            <th>"Tenders"</th>
                                            <th>"Priority"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        for cat in categories.get().iter() {
                                            <tr>
                                                <td>{cat.name.clone()}</td>
                                                <td class="budget-input">{format_currency(cat.allocated_budget)}</td>
                                                <td>{cat.planned_tenders.to_string()}</td>
                                                <td>{cat.priority.label()}</td>
                                            </tr>
                                        }
                                    </tbody>
                                </table>
                            </div>
                        },
                        view! {
                            <div class="review-section">
                                <div class="review-section-title">"Timeline ({} milestones)"</div>
                                <table class="milestones-table">
                                    <thead>
                                        <tr>
                                            <th>"Milestone"</th>
                                            <th>"Target Date"</th>
                                            <th>"Responsible"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        for ms in milestones.get().iter() {
                                            <tr>
                                                <td>{ms.name.clone()}</td>
                                                <td>{ms.planned_date.clone()}</td>
                                                <td>{ms.responsible_party.clone().unwrap_or_default()}</td>
                                            </tr>
                                        }
                                    </tbody>
                                </table>
                            </div>
                        },
                    ],
                    vec![
                        view! { <button class="btn btn-secondary" on:click={on_prev.clone()}>"Back"</button> },
                        view! { <button class="btn btn-secondary" on:click={on_save_draft.clone()}>"Save Draft"</button> },
                        view! { <button class="btn btn-primary" on:click={on_submit.clone()}>"Submit for Review"</button> },
                    ]
                )}
            }
        </div>
    }
}

/// Render a single category row
fn category_row(
    idx: usize,
    cat: ProcurementCategory,
    categories: Signal<Vec<ProcurementCategory>>,
    on_remove: Callback<usize>,
) -> View {
    let on_name_change = {
        let categories = categories.clone();
        Callback::new(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let mut items = categories.get();
            if idx < items.len() {
                items[idx].name = input.value();
                categories.set(items);
            }
        })
    };

    let on_code_change = {
        let categories = categories.clone();
        Callback::new(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let mut items = categories.get();
            if idx < items.len() {
                items[idx].code = input.value();
                categories.set(items);
            }
        })
    };

    let on_budget_change = {
        let categories = categories.clone();
        Callback::new(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let mut items = categories.get();
            if idx < items.len() {
                items[idx].allocated_budget = input.value().replace(",", "").parse().unwrap_or(0.0);
                categories.set(items);
            }
        })
    };

    let on_tenders_change = {
        let categories = categories.clone();
        Callback::new(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let mut items = categories.get();
            if idx < items.len() {
                items[idx].planned_tenders = input.value().parse().unwrap_or(0);
                categories.set(items);
            }
        })
    };

    let on_priority_change = {
        let categories = categories.clone();
        Callback::new(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let select: web_sys::HtmlSelectElement = target.dyn_into().unwrap();
            let mut items = categories.get();
            if idx < items.len() {
                items[idx].priority = match select.value().as_str() {
                    "low" => CategoryPriority::Low,
                    "high" => CategoryPriority::High,
                    "critical" => CategoryPriority::Critical,
                    _ => CategoryPriority::Medium,
                };
                categories.set(items);
            }
        })
    };

    let on_bbbee_change = {
        let categories = categories.clone();
        Callback::new(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let mut items = categories.get();
            if idx < items.len() {
                let val: f64 = input.value().parse().unwrap_or(0.0);
                items[idx].bbbee_target = if val > 0.0 { Some(val) } else { None };
                categories.set(items);
            }
        })
    };

    let on_local_change = {
        let categories = categories.clone();
        Callback::new(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let mut items = categories.get();
            if idx < items.len() {
                let val: f64 = input.value().parse().unwrap_or(0.0);
                items[idx].local_content_target = if val > 0.0 { Some(val) } else { None };
                categories.set(items);
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
                    value={cat.name.clone()}
                    placeholder="Category name"
                    on:input={on_name_change}
                />
            </td>
            <td>
                <input
                    type="text"
                    value={cat.code.clone()}
                    placeholder="Code"
                    on:input={on_code_change}
                />
            </td>
            <td>
                <input
                    type="text"
                    class="budget-input"
                    value={format!("{:.2}", cat.allocated_budget)}
                    on:input={on_budget_change}
                />
            </td>
            <td>
                <input
                    type="number"
                    value={cat.planned_tenders.to_string()}
                    min="0"
                    on:input={on_tenders_change}
                />
            </td>
            <td>
                <select value={cat.priority.as_str()} on:change={on_priority_change}>
                    <option value="low">"Low"</option>
                    <option value="medium">"Medium"</option>
                    <option value="high">"High"</option>
                    <option value="critical">"Critical"</option>
                </select>
            </td>
            <td>
                <input
                    type="number"
                    class="percentage-input"
                    value={cat.bbbee_target.map(|v| format!("{:.0}", v)).unwrap_or_default()}
                    placeholder="-"
                    min="0"
                    max="100"
                    on:input={on_bbbee_change}
                />
            </td>
            <td>
                <input
                    type="number"
                    class="percentage-input"
                    value={cat.local_content_target.map(|v| format!("{:.0}", v)).unwrap_or_default()}
                    placeholder="-"
                    min="0"
                    max="100"
                    on:input={on_local_change}
                />
            </td>
            <td>
                <button class="remove-btn" on:click={handle_remove} title="Remove">
                    "x"
                </button>
            </td>
        </tr>
    }
}

/// Render a single milestone row
fn milestone_row(
    idx: usize,
    ms: TimelineMilestone,
    milestones: Signal<Vec<TimelineMilestone>>,
    on_remove: Callback<usize>,
) -> View {
    let on_name_change = {
        let milestones = milestones.clone();
        Callback::new(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let mut items = milestones.get();
            if idx < items.len() {
                items[idx].name = input.value();
                milestones.set(items);
            }
        })
    };

    let on_desc_change = {
        let milestones = milestones.clone();
        Callback::new(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let mut items = milestones.get();
            if idx < items.len() {
                items[idx].description = if input.value().is_empty() { None } else { Some(input.value()) };
                milestones.set(items);
            }
        })
    };

    let on_date_change = {
        let milestones = milestones.clone();
        Callback::new(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let mut items = milestones.get();
            if idx < items.len() {
                items[idx].planned_date = input.value();
                milestones.set(items);
            }
        })
    };

    let on_responsible_change = {
        let milestones = milestones.clone();
        Callback::new(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let mut items = milestones.get();
            if idx < items.len() {
                items[idx].responsible_party = if input.value().is_empty() { None } else { Some(input.value()) };
                milestones.set(items);
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
                    value={ms.name.clone()}
                    placeholder="Milestone name"
                    on:input={on_name_change}
                />
            </td>
            <td>
                <input
                    type="text"
                    value={ms.description.clone().unwrap_or_default()}
                    placeholder="Description"
                    on:input={on_desc_change}
                />
            </td>
            <td>
                <input
                    type="date"
                    value={ms.planned_date.clone()}
                    on:input={on_date_change}
                />
            </td>
            <td>
                <input
                    type="text"
                    value={ms.responsible_party.clone().unwrap_or_default()}
                    placeholder="Responsible party"
                    on:input={on_responsible_change}
                />
            </td>
            <td>
                <button class="remove-btn" on:click={handle_remove} title="Remove">
                    "x"
                </button>
            </td>
        </tr>
    }
}

/// Render a single objective row
fn objective_row(
    idx: usize,
    obj: String,
    objectives: Signal<Vec<String>>,
    on_remove: Callback<usize>,
) -> View {
    let on_change = {
        let objectives = objectives.clone();
        Callback::new(move |e: web_sys::Event| {
            let target = e.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let mut items = objectives.get();
            if idx < items.len() {
                items[idx] = input.value();
                objectives.set(items);
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
        <div class="objective-row">
            <input
                type="text"
                value={obj}
                placeholder="Enter strategic objective..."
                on:input={on_change}
            />
            <button class="remove-btn" on:click={handle_remove} title="Remove">
                "x"
            </button>
        </div>
    }
}
