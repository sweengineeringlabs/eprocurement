//! AI Chat interface component

use components::prelude::*;
use crate::shared::components::{
    panel, status_badge, StatusType, btn_primary, btn_secondary, ButtonSize,
    empty_state,
};
use super::store::{
    AiAssistantStore, init_store, toggle_chat, close_chat, open_chat,
    send_user_message, start_new_conversation, select_conversation,
    get_suggestions_by_category,
};
use super::service;
use super::types::{
    MessageRole, Suggestion, SuggestionCategory, ConversationContext,
    ResponseType,
};

/// AI Chat panel component - floating chat interface
#[component]
pub fn ai_chat_panel() -> View {
    let store = use_context::<AiAssistantStore>();

    // Initialize store on mount
    effect({
        let store = store.clone();
        move || {
            init_store(&store);
        }
    });

    let is_open = store.is_open.clone();
    let is_loading = store.is_loading.clone();
    let conversation = store.current_conversation.clone();
    let input_message = store.input_message.clone();
    let error = store.error.clone();

    // Handle send message
    let handle_send = {
        let store = store.clone();
        let input = store.input_message.clone();
        move || {
            let message = input.get();
            if !message.trim().is_empty() {
                let store = store.clone();
                let msg = message.clone();
                send_user_message(&store, msg.clone());
                spawn(async move {
                    service::send_message(&store, msg).await;
                });
            }
        }
    };

    // Handle suggestion click
    let handle_suggestion = {
        let store = store.clone();
        move |prompt: String| {
            let store = store.clone();
            send_user_message(&store, prompt.clone());
            spawn(async move {
                service::send_message(&store, prompt).await;
            });
        }
    };

    // Handle new conversation
    let handle_new_conversation = {
        let store = store.clone();
        move || {
            start_new_conversation(&store, ConversationContext::General);
        }
    };

    // Handle close
    let handle_close = {
        let store = store.clone();
        move || {
            close_chat(&store);
        }
    };

    // Icons
    let icon_ai = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="3"/><path d="M12 2v4m0 12v4M2 12h4m12 0h4m-3.5-6.5L17 7m-10 10-1.5 1.5M19.5 17.5 17 17M7 7 5.5 5.5"/></svg>"#;
    let icon_send = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="22" y1="2" x2="11" y2="13"/><polygon points="22 2 15 22 11 13 2 9 22 2"/></svg>"#;
    let icon_close = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>"#;
    let icon_new = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>"#;
    let icon_history = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>"#;

    view! {
        style {
            r#"
            .ai-chat-fab {
                position: fixed;
                bottom: 24px;
                right: 24px;
                width: 56px;
                height: 56px;
                border-radius: 50%;
                background: var(--accent);
                color: white;
                border: none;
                cursor: pointer;
                box-shadow: 0 4px 12px #00000026;
                display: flex;
                align-items: center;
                justify-content: center;
                transition: transform 0.2s, box-shadow 0.2s;
                z-index: 1000;
            }
            .ai-chat-fab:hover {
                transform: scale(1.05);
                box-shadow: 0 6px 16px #00000033;
            }
            .ai-chat-fab svg {
                width: 24px;
                height: 24px;
            }

            .ai-chat-panel {
                position: fixed;
                bottom: 24px;
                right: 24px;
                width: 420px;
                height: 600px;
                max-height: calc(100vh - 48px);
                background: var(--surface);
                border-radius: 12px;
                box-shadow: 0 8px 32px #00000026;
                display: flex;
                flex-direction: column;
                z-index: 1001;
                overflow: hidden;
            }

            .ai-chat-header {
                display: flex;
                align-items: center;
                justify-content: space-between;
                padding: 16px;
                background: var(--accent);
                color: white;
            }
            .ai-chat-header-title {
                display: flex;
                align-items: center;
                gap: 8px;
                font-weight: 600;
            }
            .ai-chat-header-title svg {
                width: 20px;
                height: 20px;
            }
            .ai-chat-header-actions {
                display: flex;
                gap: 8px;
            }
            .ai-chat-header-btn {
                background: #ffffff33;
                border: none;
                color: white;
                width: 32px;
                height: 32px;
                border-radius: 6px;
                cursor: pointer;
                display: flex;
                align-items: center;
                justify-content: center;
                transition: background 0.2s;
            }
            .ai-chat-header-btn:hover {
                background: #ffffff4D;
            }
            .ai-chat-header-btn svg {
                width: 16px;
                height: 16px;
            }

            .ai-chat-body {
                flex: 1;
                overflow-y: auto;
                padding: 16px;
                display: flex;
                flex-direction: column;
                gap: 16px;
            }

            .ai-chat-messages {
                display: flex;
                flex-direction: column;
                gap: 12px;
            }

            .ai-message {
                display: flex;
                flex-direction: column;
                gap: 4px;
                max-width: 90%;
            }
            .ai-message.user {
                align-self: flex-end;
            }
            .ai-message.assistant {
                align-self: flex-start;
            }

            .ai-message-bubble {
                padding: 12px 16px;
                border-radius: 12px;
                font-size: 14px;
                line-height: 1.5;
            }
            .ai-message.user .ai-message-bubble {
                background: var(--accent);
                color: white;
                border-bottom-right-radius: 4px;
            }
            .ai-message.assistant .ai-message-bubble {
                background: var(--background);
                border: 1px solid var(--border);
                border-bottom-left-radius: 4px;
            }

            .ai-message-meta {
                font-size: 11px;
                color: var(--text-muted);
                padding: 0 4px;
            }
            .ai-message.user .ai-message-meta {
                text-align: right;
            }

            .ai-message-content {
                white-space: pre-wrap;
            }
            .ai-message-content strong {
                font-weight: 600;
            }
            .ai-message-content code {
                background: var(--surface);
                padding: 2px 6px;
                border-radius: 4px;
                font-family: monospace;
                font-size: 13px;
            }

            .ai-message-sources {
                margin-top: 8px;
                padding-top: 8px;
                border-top: 1px solid var(--border);
                font-size: 12px;
                color: var(--text-muted);
            }
            .ai-message-sources-title {
                font-weight: 500;
                margin-bottom: 4px;
            }
            .ai-message-sources-list {
                display: flex;
                flex-wrap: wrap;
                gap: 4px;
            }
            .ai-message-source-tag {
                background: var(--surface);
                padding: 2px 8px;
                border-radius: 4px;
                font-size: 11px;
            }

            .ai-chat-welcome {
                text-align: center;
                padding: 24px 16px;
            }
            .ai-chat-welcome-icon {
                width: 48px;
                height: 48px;
                margin: 0 auto 16px;
                color: var(--accent);
            }
            .ai-chat-welcome h3 {
                margin: 0 0 8px;
                font-size: 18px;
                font-weight: 600;
            }
            .ai-chat-welcome p {
                margin: 0;
                color: var(--text-muted);
                font-size: 14px;
            }

            .ai-suggestions {
                display: flex;
                flex-direction: column;
                gap: 8px;
            }
            .ai-suggestions-title {
                font-size: 12px;
                font-weight: 500;
                color: var(--text-muted);
                text-transform: uppercase;
                letter-spacing: 0.5px;
            }
            .ai-suggestions-grid {
                display: grid;
                grid-template-columns: repeat(2, 1fr);
                gap: 8px;
            }
            .ai-suggestion-btn {
                display: flex;
                align-items: center;
                gap: 8px;
                padding: 10px 12px;
                background: var(--background);
                border: 1px solid var(--border);
                border-radius: 8px;
                cursor: pointer;
                transition: all 0.2s;
                text-align: left;
                font-size: 13px;
            }
            .ai-suggestion-btn:hover {
                background: var(--surface);
                border-color: var(--accent);
            }
            .ai-suggestion-btn svg {
                width: 16px;
                height: 16px;
                color: var(--accent);
                flex-shrink: 0;
            }

            .ai-chat-footer {
                padding: 16px;
                border-top: 1px solid var(--border);
                background: var(--surface);
            }
            .ai-chat-input-container {
                display: flex;
                gap: 8px;
            }
            .ai-chat-input {
                flex: 1;
                padding: 12px 16px;
                border: 1px solid var(--border);
                border-radius: 8px;
                font-size: 14px;
                resize: none;
                min-height: 44px;
                max-height: 120px;
                font-family: inherit;
            }
            .ai-chat-input:focus {
                outline: none;
                border-color: var(--accent);
            }
            .ai-chat-send-btn {
                width: 44px;
                height: 44px;
                border-radius: 8px;
                background: var(--accent);
                color: white;
                border: none;
                cursor: pointer;
                display: flex;
                align-items: center;
                justify-content: center;
                transition: background 0.2s;
            }
            .ai-chat-send-btn:hover:not(:disabled) {
                background: var(--accent-dark);
            }
            .ai-chat-send-btn:disabled {
                opacity: 0.5;
                cursor: not-allowed;
            }
            .ai-chat-send-btn svg {
                width: 18px;
                height: 18px;
            }

            .ai-chat-loading {
                display: flex;
                align-items: center;
                gap: 8px;
                padding: 12px 16px;
                background: var(--background);
                border-radius: 12px;
                align-self: flex-start;
            }
            .ai-chat-loading-dots {
                display: flex;
                gap: 4px;
            }
            .ai-chat-loading-dot {
                width: 8px;
                height: 8px;
                background: var(--accent);
                border-radius: 50%;
                animation: ai-dot-pulse 1.4s ease-in-out infinite;
            }
            .ai-chat-loading-dot:nth-child(2) {
                animation-delay: 0.2s;
            }
            .ai-chat-loading-dot:nth-child(3) {
                animation-delay: 0.4s;
            }
            @keyframes ai-dot-pulse {
                0%, 80%, 100% {
                    transform: scale(0.6);
                    opacity: 0.5;
                }
                40% {
                    transform: scale(1);
                    opacity: 1;
                }
            }

            .ai-chat-error {
                padding: 12px 16px;
                background: var(--red-light);
                color: var(--red);
                border-radius: 8px;
                font-size: 13px;
            }

            @media (max-width: 480px) {
                .ai-chat-panel {
                    width: calc(100vw - 32px);
                    height: calc(100vh - 32px);
                    bottom: 16px;
                    right: 16px;
                }
            }
            "#
        }

        // Floating action button (when chat is closed)
        {if !is_open.get() {
            let store = store.clone();
            view! {
                <button
                    class="ai-chat-fab"
                    on:click={move || toggle_chat(&store)}
                    title="AI Assistant"
                    data-testid="ai-chat-fab"
                >
                    <span inner_html={icon_ai}></span>
                </button>
            }
        } else {
            view! { <span></span> }
        }}

        // Chat panel (when open)
        {if is_open.get() {
            let messages = conversation.get().messages;
            let suggestions = store.suggestions.get();
            let is_empty = messages.is_empty();

            view! {
                <div class="ai-chat-panel" data-testid="ai-chat-panel">
                    // Header
                    <div class="ai-chat-header">
                        <div class="ai-chat-header-title">
                            <span inner_html={icon_ai}></span>
                            <span>"AI Procurement Assistant"</span>
                        </div>
                        <div class="ai-chat-header-actions">
                            <button
                                class="ai-chat-header-btn"
                                on:click={
                                    let handle = handle_new_conversation.clone();
                                    move || handle()
                                }
                                title="New Conversation"
                            >
                                <span inner_html={icon_new}></span>
                            </button>
                            <button
                                class="ai-chat-header-btn"
                                on:click={
                                    let handle = handle_close.clone();
                                    move || handle()
                                }
                                title="Close"
                            >
                                <span inner_html={icon_close}></span>
                            </button>
                        </div>
                    </div>

                    // Body
                    <div class="ai-chat-body">
                        {if is_empty {
                            // Welcome state with suggestions
                            view! {
                                <div class="ai-chat-welcome">
                                    <div class="ai-chat-welcome-icon" inner_html={icon_ai}></div>
                                    <h3>"How can I help you today?"</h3>
                                    <p>"I can assist with tender analysis, compliance checks, bid evaluation, and procurement guidance."</p>
                                </div>

                                // Quick action suggestions
                                <div class="ai-suggestions">
                                    <div class="ai-suggestions-title">"Quick Actions"</div>
                                    <div class="ai-suggestions-grid">
                                        {suggestions.iter().take(6).map(|suggestion| {
                                            let prompt = suggestion.prompt.clone();
                                            let label = suggestion.label.clone();
                                            let handle = handle_suggestion.clone();
                                            view! {
                                                <button
                                                    class="ai-suggestion-btn"
                                                    on:click={move || handle(prompt.clone())}
                                                >
                                                    <span>{label.clone()}</span>
                                                </button>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            }
                        } else {
                            // Message history
                            view! {
                                <div class="ai-chat-messages">
                                    {messages.iter().map(|msg| {
                                        let role_class = match msg.role {
                                            MessageRole::User => "user",
                                            MessageRole::Assistant => "assistant",
                                            MessageRole::System => "system",
                                        };
                                        let sources = msg.metadata.as_ref().map(|m| m.sources.clone()).unwrap_or_default();

                                        view! {
                                            <div class=format!("ai-message {}", role_class)>
                                                <div class="ai-message-bubble">
                                                    <div class="ai-message-content">{msg.content.clone()}</div>
                                                    {if !sources.is_empty() {
                                                        view! {
                                                            <div class="ai-message-sources">
                                                                <div class="ai-message-sources-title">"Sources:"</div>
                                                                <div class="ai-message-sources-list">
                                                                    {sources.iter().map(|s| {
                                                                        view! {
                                                                            <span class="ai-message-source-tag">{s.clone()}</span>
                                                                        }
                                                                    }).collect::<Vec<_>>()}
                                                                </div>
                                                            </div>
                                                        }
                                                    } else {
                                                        view! { <span></span> }
                                                    }}
                                                </div>
                                                <div class="ai-message-meta">{msg.timestamp.clone()}</div>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}

                                    // Loading indicator
                                    {if is_loading.get() {
                                        view! {
                                            <div class="ai-chat-loading">
                                                <div class="ai-chat-loading-dots">
                                                    <div class="ai-chat-loading-dot"></div>
                                                    <div class="ai-chat-loading-dot"></div>
                                                    <div class="ai-chat-loading-dot"></div>
                                                </div>
                                                <span>"Thinking..."</span>
                                            </div>
                                        }
                                    } else {
                                        view! { <span></span> }
                                    }}

                                    // Error message
                                    {if let Some(err) = error.get() {
                                        view! {
                                            <div class="ai-chat-error">{err}</div>
                                        }
                                    } else {
                                        view! { <span></span> }
                                    }}
                                </div>
                            }
                        }}
                    </div>

                    // Footer with input
                    <div class="ai-chat-footer">
                        <div class="ai-chat-input-container">
                            <textarea
                                class="ai-chat-input"
                                placeholder="Ask me anything about procurement..."
                                prop:value={input_message.get()}
                                on:input={
                                    let input = store.input_message.clone();
                                    move |e: web_sys::Event| {
                                        let target = e.target().unwrap();
                                        let textarea: web_sys::HtmlTextAreaElement = target.unchecked_into();
                                        input.set(textarea.value());
                                    }
                                }
                                on:keydown={
                                    let handle = handle_send.clone();
                                    move |e: web_sys::KeyboardEvent| {
                                        if e.key() == "Enter" && !e.shift_key() {
                                            e.prevent_default();
                                            handle();
                                        }
                                    }
                                }
                                data-testid="ai-chat-input"
                            ></textarea>
                            <button
                                class="ai-chat-send-btn"
                                on:click={
                                    let handle = handle_send.clone();
                                    move || handle()
                                }
                                disabled=is_loading.get() || input_message.get().trim().is_empty()
                                data-testid="ai-chat-send"
                            >
                                <span inner_html={icon_send}></span>
                            </button>
                        </div>
                    </div>
                </div>
            }
        } else {
            view! { <span></span> }
        }}
    }
}

/// AI Chat trigger button - for embedding in other components
#[component]
pub fn ai_chat_trigger(
    label: String,
    context: ConversationContext,
    initial_prompt: Option<String>,
) -> View {
    let store = use_context::<AiAssistantStore>();

    let handle_click = {
        let store = store.clone();
        let ctx = context.clone();
        let prompt = initial_prompt.clone();
        move || {
            start_new_conversation(&store, ctx.clone());
            open_chat(&store);
            if let Some(p) = prompt.clone() {
                let store = store.clone();
                send_user_message(&store, p.clone());
                spawn(async move {
                    service::send_message(&store, p).await;
                });
            }
        }
    };

    let icon_ai = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="3"/><path d="M12 2v4m0 12v4M2 12h4m12 0h4m-3.5-6.5L17 7m-10 10-1.5 1.5M19.5 17.5 17 17M7 7 5.5 5.5"/></svg>"#;

    view! {
        style {
            r#"
            .ai-trigger-btn {
                display: inline-flex;
                align-items: center;
                gap: 8px;
                padding: 8px 16px;
                background: var(--accent-light);
                color: var(--accent);
                border: 1px solid var(--accent);
                border-radius: 6px;
                cursor: pointer;
                font-size: 14px;
                font-weight: 500;
                transition: all 0.2s;
            }
            .ai-trigger-btn:hover {
                background: var(--accent);
                color: white;
            }
            .ai-trigger-btn svg {
                width: 16px;
                height: 16px;
            }
            "#
        }

        <button
            class="ai-trigger-btn"
            on:click={move || handle_click()}
            data-testid="ai-trigger-btn"
        >
            <span inner_html={icon_ai}></span>
            <span>{label}</span>
        </button>
    }
}

/// Quick analysis button for embedding in tender/contract views
#[component]
pub fn ai_quick_analysis(
    entity_type: String,
    entity_id: String,
    entity_title: String,
) -> View {
    let store = use_context::<AiAssistantStore>();

    let context = match entity_type.as_str() {
        "tender" => ConversationContext::Tender {
            tender_id: entity_id.clone(),
            tender_title: entity_title.clone(),
        },
        "contract" => ConversationContext::Contract {
            contract_id: entity_id.clone(),
            contract_title: entity_title.clone(),
        },
        "supplier" => ConversationContext::Supplier {
            supplier_id: entity_id.clone(),
            supplier_name: entity_title.clone(),
        },
        _ => ConversationContext::General,
    };

    let prompt = format!("Analyze {} {} for completeness and compliance.", entity_type, entity_id);

    view! {
        {ai_chat_trigger(
            format!("AI Analysis"),
            context,
            Some(prompt),
        )}
    }
}

/// AI suggestions panel - for embedding in forms or views
#[component]
pub fn ai_suggestions_panel(category: SuggestionCategory) -> View {
    let store = use_context::<AiAssistantStore>();
    let suggestions = get_suggestions_by_category(&store, category);

    let handle_suggestion = {
        let store = store.clone();
        move |prompt: String| {
            let store = store.clone();
            start_new_conversation(&store, ConversationContext::General);
            open_chat(&store);
            send_user_message(&store, prompt.clone());
            spawn(async move {
                service::send_message(&store, prompt).await;
            });
        }
    };

    view! {
        style {
            r#"
            .ai-suggestions-panel {
                padding: 16px;
                background: var(--background);
                border: 1px solid var(--border);
                border-radius: 8px;
            }
            .ai-suggestions-panel-header {
                display: flex;
                align-items: center;
                gap: 8px;
                margin-bottom: 12px;
            }
            .ai-suggestions-panel-header svg {
                width: 20px;
                height: 20px;
                color: var(--accent);
            }
            .ai-suggestions-panel-title {
                font-weight: 600;
                font-size: 14px;
            }
            .ai-suggestions-panel-list {
                display: flex;
                flex-direction: column;
                gap: 8px;
            }
            .ai-suggestions-panel-item {
                display: flex;
                align-items: center;
                justify-content: space-between;
                padding: 10px 12px;
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: 6px;
                cursor: pointer;
                transition: all 0.2s;
            }
            .ai-suggestions-panel-item:hover {
                border-color: var(--accent);
                background: var(--accent-light);
            }
            .ai-suggestions-panel-item-label {
                font-size: 13px;
            }
            .ai-suggestions-panel-item-arrow {
                color: var(--text-muted);
            }
            "#
        }

        <div class="ai-suggestions-panel">
            <div class="ai-suggestions-panel-header">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <circle cx="12" cy="12" r="3"/>
                    <path d="M12 2v4m0 12v4M2 12h4m12 0h4"/>
                </svg>
                <span class="ai-suggestions-panel-title">{format!("AI {} Suggestions", category.label())}</span>
            </div>
            <div class="ai-suggestions-panel-list">
                {suggestions.iter().map(|s| {
                    let prompt = s.prompt.clone();
                    let label = s.label.clone();
                    let handle = handle_suggestion.clone();
                    view! {
                        <div
                            class="ai-suggestions-panel-item"
                            on:click={move || handle(prompt.clone())}
                        >
                            <span class="ai-suggestions-panel-item-label">{label}</span>
                            <span class="ai-suggestions-panel-item-arrow">">"</span>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
