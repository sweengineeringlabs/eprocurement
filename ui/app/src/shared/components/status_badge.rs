//! Status badge component

use components::prelude::*;

/// Status types with associated styling
#[derive(Clone, Copy, PartialEq, Default)]
pub enum StatusType {
    // Green statuses
    #[default]
    Approved,
    Active,
    Published,
    Complete,
    // Orange statuses
    Pending,
    Draft,
    InProgress,
    // Red statuses
    Rejected,
    Cancelled,
    Expired,
    Failed,
    // Blue statuses
    New,
    Submitted,
    Open,
    // Purple statuses
    Evaluation,
    Review,
    // Gray statuses
    OnHold,
    Scheduled,
}

impl StatusType {
    fn class(&self) -> &'static str {
        match self {
            StatusType::Approved => "status-approved",
            StatusType::Active => "status-active",
            StatusType::Published => "status-published",
            StatusType::Complete => "status-complete",
            StatusType::Pending => "status-pending",
            StatusType::Draft => "status-draft",
            StatusType::InProgress => "status-in-progress",
            StatusType::Rejected => "status-rejected",
            StatusType::Cancelled => "status-cancelled",
            StatusType::Expired => "status-expired",
            StatusType::Failed => "status-failed",
            StatusType::New => "status-new",
            StatusType::Submitted => "status-submitted",
            StatusType::Open => "status-open",
            StatusType::Evaluation => "status-evaluation",
            StatusType::Review => "status-review",
            StatusType::OnHold => "status-on-hold",
            StatusType::Scheduled => "status-scheduled",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            StatusType::Approved => "Approved",
            StatusType::Active => "Active",
            StatusType::Published => "Published",
            StatusType::Complete => "Complete",
            StatusType::Pending => "Pending",
            StatusType::Draft => "Draft",
            StatusType::InProgress => "In Progress",
            StatusType::Rejected => "Rejected",
            StatusType::Cancelled => "Cancelled",
            StatusType::Expired => "Expired",
            StatusType::Failed => "Failed",
            StatusType::New => "New",
            StatusType::Submitted => "Submitted",
            StatusType::Open => "Open",
            StatusType::Evaluation => "Evaluation",
            StatusType::Review => "Review",
            StatusType::OnHold => "On Hold",
            StatusType::Scheduled => "Scheduled",
        }
    }
}

/// Status badge component
#[component]
pub fn status_badge(status: StatusType) -> View {
    let class = format!("status {}", status.class());
    let label = status.label();

    view! {
        style {
            r#"
            .status {
                display: inline-flex;
                align-items: center;
                gap: 6px;
                padding: 4px 10px;
                border-radius: 12px;
                font-size: 11px;
                font-weight: 500;
            }
            .status::before {
                content: "";
                width: 6px;
                height: 6px;
                border-radius: 50%;
            }
            .status-approved, .status-active, .status-published, .status-complete {
                background: var(--green-light);
                color: var(--green);
            }
            .status-approved::before, .status-active::before, .status-published::before, .status-complete::before {
                background: var(--green);
            }
            .status-pending, .status-draft, .status-in-progress {
                background: var(--orange-light);
                color: var(--orange);
            }
            .status-pending::before, .status-draft::before, .status-in-progress::before {
                background: var(--orange);
            }
            .status-rejected, .status-cancelled, .status-expired, .status-failed {
                background: var(--red-light);
                color: var(--red);
            }
            .status-rejected::before, .status-cancelled::before, .status-expired::before, .status-failed::before {
                background: var(--red);
            }
            .status-new, .status-submitted, .status-open {
                background: var(--blue-light);
                color: var(--blue);
            }
            .status-new::before, .status-submitted::before, .status-open::before {
                background: var(--blue);
            }
            .status-evaluation, .status-review {
                background: var(--purple-light);
                color: var(--purple);
            }
            .status-evaluation::before, .status-review::before {
                background: var(--purple);
            }
            .status-on-hold, .status-scheduled {
                background: var(--gray-light, #f0f0f0);
                color: var(--text-muted);
            }
            .status-on-hold::before, .status-scheduled::before {
                background: var(--text-muted);
            }
            "#
        }

        <span class={class}>{label}</span>
    }
}
