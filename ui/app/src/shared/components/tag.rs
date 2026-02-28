//! Tag component for categorization

use components::prelude::*;

/// Tag type variants
#[derive(Clone, Copy, PartialEq, Default)]
pub enum TagType {
    #[default]
    Rfq,
    Rfp,
    Tender,
    Limited,
    Strategic,
    Tactical,
    Default,
    // Color-based variants
    Green,
    Orange,
    Red,
    Blue,
    Purple,
    Cyan,
    // Semantic variants
    Info,
    Warning,
    Danger,
    Success,
    Primary,
}

impl TagType {
    fn class(&self) -> &'static str {
        match self {
            TagType::Rfq => "tag-rfq",
            TagType::Rfp => "tag-rfp",
            TagType::Tender => "tag-tender",
            TagType::Limited => "tag-limited",
            TagType::Strategic => "tag-strategic",
            TagType::Tactical => "tag-tactical",
            TagType::Default => "",
            TagType::Green => "tag-green",
            TagType::Orange => "tag-orange",
            TagType::Red => "tag-red",
            TagType::Blue => "tag-blue",
            TagType::Purple => "tag-purple",
            TagType::Cyan => "tag-cyan",
            TagType::Info => "tag-info",
            TagType::Warning => "tag-warning",
            TagType::Danger => "tag-danger",
            TagType::Success => "tag-success",
            TagType::Primary => "tag-primary",
        }
    }
}

/// Tag component
#[component]
pub fn tag(label: String, tag_type: TagType) -> View {
    let class = format!("tag {}", tag_type.class());

    view! {
        style {
            r#"
            .tag {
                display: inline-block;
                padding: 2px 8px;
                border-radius: var(--radius-sm);
                font-size: 10px;
                font-weight: 600;
                text-transform: uppercase;
                background: var(--bg);
                color: var(--text-muted);
            }
            .tag-rfq { background: var(--blue-light); color: var(--blue); }
            .tag-rfp { background: var(--purple-light); color: var(--purple); }
            .tag-tender { background: var(--accent-light); color: var(--accent); }
            .tag-limited { background: var(--cyan-light); color: var(--cyan); }
            .tag-strategic { background: var(--purple-light); color: var(--purple); }
            .tag-tactical { background: var(--blue-light); color: var(--blue); }
            .tag-green { background: var(--green-light); color: var(--green); }
            .tag-orange { background: var(--orange-light); color: var(--orange); }
            .tag-red { background: var(--red-light); color: var(--red); }
            .tag-blue { background: var(--blue-light); color: var(--blue); }
            .tag-purple { background: var(--purple-light); color: var(--purple); }
            .tag-cyan { background: var(--cyan-light); color: var(--cyan); }
            .tag-info { background: var(--blue-light); color: var(--blue); }
            .tag-warning { background: var(--orange-light); color: var(--orange); }
            .tag-danger { background: var(--red-light); color: var(--red); }
            .tag-success { background: var(--green-light); color: var(--green); }
            .tag-primary { background: var(--accent-light); color: var(--accent); }
            "#
        }

        <span class={class}>{label}</span>
    }
}
