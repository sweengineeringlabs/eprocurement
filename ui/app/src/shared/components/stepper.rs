//! Stepper component for multi-step workflows

use components::prelude::*;

/// Step status
#[derive(Clone, Copy, PartialEq)]
pub enum StepStatus {
    Completed,
    Active,
    Pending,
}

/// Stepper item data
#[derive(Clone)]
pub struct StepperItem {
    pub number: u32,
    pub label: String,
    pub status: StepStatus,
}

/// Stepper component
#[component]
pub fn stepper(steps: Vec<StepperItem>, on_step_click: Option<Callback<u32>>) -> View {
    view! {
        style {
            r#"
            .stepper {
                display: flex;
                align-items: center;
                gap: 0;
                margin-bottom: 24px;
            }
            .stepper-item {
                display: flex;
                align-items: center;
                gap: 8px;
                flex: 1;
                position: relative;
                cursor: pointer;
            }
            .stepper-item:not(:last-child)::after {
                content: "";
                flex: 1;
                height: 2px;
                background: var(--border);
                margin: 0 12px;
            }
            .stepper-item.completed::after {
                background: var(--green);
            }
            .stepper-item.active::after {
                background: linear-gradient(90deg, var(--blue) 50%, var(--border) 50%);
            }
            .stepper-number {
                width: 32px;
                height: 32px;
                border-radius: 50%;
                background: var(--bg);
                border: 2px solid var(--border);
                display: flex;
                align-items: center;
                justify-content: center;
                font-size: 13px;
                font-weight: 600;
                color: var(--text-muted);
                flex-shrink: 0;
                transition: all 0.2s ease;
            }
            .stepper-item:hover .stepper-number {
                transform: scale(1.1);
            }
            .stepper-item.completed .stepper-number {
                background: var(--green);
                border-color: var(--green);
                color: #fff;
            }
            .stepper-item.active .stepper-number {
                background: var(--blue);
                border-color: var(--blue);
                color: #fff;
            }
            .stepper-label {
                font-size: 12px;
                font-weight: 500;
                color: var(--text-muted);
                white-space: nowrap;
            }
            .stepper-item.active .stepper-label,
            .stepper-item.completed .stepper-label {
                color: var(--text);
            }
            "#
        }

        <div class="stepper" data-testid="stepper">
            for step in steps.iter() {
                {stepper_item_view(step.clone(), on_step_click.clone())}
            }
        </div>
    }
}

fn stepper_item_view(step: StepperItem, on_click: Option<Callback<u32>>) -> View {
    let status_class = match step.status {
        StepStatus::Completed => "completed",
        StepStatus::Active => "active",
        StepStatus::Pending => "",
    };
    let class = format!("stepper-item {}", status_class);
    let number = step.number;

    let handle_click = Callback::<()>::new({
        let on_click = on_click.clone();
        move |_| {
            if let Some(cb) = &on_click {
                cb.call(number);
            }
        }
    });

    let display_content = if step.status == StepStatus::Completed {
        "✓".to_string()
    } else {
        step.number.to_string()
    };

    view! {
        <div class={class} on:click={handle_click}>
            <div class="stepper-number">{display_content}</div>
            <span class="stepper-label">{step.label}</span>
        </div>
    }
}
