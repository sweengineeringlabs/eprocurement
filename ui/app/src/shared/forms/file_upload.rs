//! File upload component

use components::prelude::*;

/// Uploaded file info
#[derive(Clone)]
pub struct UploadedFile {
    pub name: String,
    pub size: u64,
}

/// File upload component with drop zone
#[component]
pub fn file_upload(
    label: String,
    files: Signal<Vec<UploadedFile>>,
    accept: Option<String>,
    multiple: bool,
    required: bool,
    hint: Option<String>,
    on_remove: Callback<usize>,
) -> View {
    view! {
        style {
            r#"
            .form-group {
                display: flex;
                flex-direction: column;
                gap: 6px;
            }
            .form-group > label {
                font-size: 12px;
                font-weight: 500;
                color: var(--text-muted);
            }
            .required-mark {
                color: var(--red);
            }
            .drop-zone {
                border: 2px dashed var(--border);
                border-radius: var(--radius-lg);
                padding: 40px;
                text-align: center;
                transition: all 0.15s;
                cursor: pointer;
            }
            .drop-zone:hover {
                border-color: var(--blue);
                background: var(--blue-light);
            }
            .drop-zone-icon {
                width: 48px;
                height: 48px;
                margin: 0 auto 16px;
                color: var(--text-muted);
            }
            .drop-zone-icon svg {
                width: 100%;
                height: 100%;
            }
            .drop-zone-text {
                font-size: 14px;
                color: var(--text);
                margin-bottom: 4px;
            }
            .drop-zone-hint {
                font-size: 12px;
                color: var(--text-muted);
            }
            .file-list {
                display: flex;
                flex-direction: column;
                gap: 8px;
                margin-top: 16px;
            }
            .file-item {
                display: flex;
                align-items: center;
                gap: 12px;
                padding: 12px;
                background: var(--bg);
                border-radius: var(--radius);
            }
            .file-icon {
                width: 32px;
                height: 32px;
                background: var(--surface);
                border-radius: var(--radius-sm);
                display: flex;
                align-items: center;
                justify-content: center;
                color: var(--blue);
            }
            .file-icon svg {
                width: 16px;
                height: 16px;
            }
            .file-info {
                flex: 1;
            }
            .file-name {
                font-size: 13px;
                font-weight: 500;
            }
            .file-size {
                font-size: 11px;
                color: var(--text-muted);
            }
            .file-remove {
                width: 24px;
                height: 24px;
                border: none;
                background: transparent;
                color: var(--text-muted);
                cursor: pointer;
                border-radius: var(--radius-sm);
                font-size: 16px;
            }
            .file-remove:hover {
                color: var(--red);
                background: var(--red-light);
            }
            "#
        }

        <div class="form-group">
            <label>
                {label}
                if required {
                    <span class="required-mark">" *"</span>
                }
            </label>
            <div class="drop-zone">
                <div class="drop-zone-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                        <polyline points="17 8 12 3 7 8"/>
                        <line x1="12" y1="3" x2="12" y2="15"/>
                    </svg>
                </div>
                <div class="drop-zone-text">"Drop files here or click to browse"</div>
                if let Some(h) = hint {
                    <div class="drop-zone-hint">{h}</div>
                }
                <input
                    type="file"
                    accept={accept.unwrap_or_default()}
                    multiple={multiple}
                    style="display: none;"
                />
            </div>
            if !files.get().is_empty() {
                <div class="file-list">
                    for (idx, file) in files.get().iter().enumerate() {
                        {file_item(file.clone(), idx, on_remove.clone())}
                    }
                </div>
            }
        </div>
    }
}

fn file_item(file: UploadedFile, idx: usize, on_remove: Callback<usize>) -> View {
    let size_str = format_file_size(file.size);

    let handle_remove = Callback::<()>::new({
        let on_remove = on_remove.clone();
        move |_| {
            on_remove.call(idx);
        }
    });

    view! {
        <div class="file-item">
            <div class="file-icon">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                    <polyline points="14 2 14 8 20 8"/>
                </svg>
            </div>
            <div class="file-info">
                <div class="file-name">{file.name}</div>
                <div class="file-size">{size_str}</div>
            </div>
            <button class="file-remove" on:click={handle_remove}>"×"</button>
        </div>
    }
}

fn format_file_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}
