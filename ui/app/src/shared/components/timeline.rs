//! Timeline component for activity history

use components::prelude::*;

/// Timeline item status
#[derive(Clone, Copy, PartialEq)]
pub enum TimelineStatus {
    Completed,
    Complete,  // Alias for Completed
    Pending,
    Active,
    Error,
}

/// Timeline item data
#[derive(Clone)]
pub struct TimelineItem {
    pub date: String,
    pub title: String,
    pub description: String,
    pub status: TimelineStatus,
}

/// Timeline component
#[component]
pub fn timeline(items: Vec<TimelineItem>, on_item_click: Option<Callback<usize>>) -> View {
    view! {
        style {
            r#"
            .timeline {
                position: relative;
                padding-left: 24px;
            }
            .timeline::before {
                content: "";
                position: absolute;
                left: 6px;
                top: 0;
                bottom: 0;
                width: 2px;
                background: var(--border);
            }
            .timeline-item {
                position: relative;
                padding-bottom: 20px;
                cursor: pointer;
            }
            .timeline-item:hover {
                opacity: 0.8;
            }
            .timeline-item::before {
                content: "";
                position: absolute;
                left: -24px;
                top: 4px;
                width: 14px;
                height: 14px;
                border-radius: 50%;
                background: var(--blue);
                border: 3px solid var(--surface);
            }
            .timeline-item.completed::before { background: var(--green); }
            .timeline-item.pending::before { background: var(--orange); }
            .timeline-item.active::before { background: var(--blue); }
            .timeline-item.error::before { background: var(--red); }
            .timeline-date {
                font-size: 11px;
                color: var(--text-muted);
                margin-bottom: 4px;
            }
            .timeline-title {
                font-weight: 500;
                font-size: 13px;
            }
            .timeline-desc {
                font-size: 12px;
                color: var(--text-muted);
                margin-top: 4px;
            }
            "#
        }

        <div class="timeline" data-testid="timeline">
            for (idx, item) in items.iter().enumerate() {
                {timeline_item_view(item.clone(), idx, on_item_click.clone())}
            }
        </div>
    }
}

fn timeline_item_view(item: TimelineItem, idx: usize, on_click: Option<Callback<usize>>) -> View {
    let status_class = match item.status {
        TimelineStatus::Completed | TimelineStatus::Complete => "completed",
        TimelineStatus::Pending => "pending",
        TimelineStatus::Active => "active",
        TimelineStatus::Error => "error",
    };
    let class = format!("timeline-item {}", status_class);

    let handle_click = Callback::<()>::new({
        let on_click = on_click.clone();
        move |_| {
            if let Some(cb) = &on_click {
                cb.call(idx);
            }
        }
    });

    view! {
        <div class={class} on:click={handle_click}>
            <div class="timeline-date">{item.date}</div>
            <div class="timeline-title">{item.title}</div>
            <div class="timeline-desc">{item.description}</div>
        </div>
    }
}
