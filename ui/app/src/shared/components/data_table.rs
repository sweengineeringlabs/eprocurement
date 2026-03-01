//! Data table component

use components::prelude::*;

/// Column definition for data table
#[derive(Clone)]
pub struct DataTableColumn {
    pub key: String,
    pub label: String,
    pub width: Option<String>,
    pub align: Option<String>, // "left", "center", "right"
    pub cell_class: Option<String>,
}

/// Row data for data table
#[derive(Clone)]
pub struct DataTableRow {
    pub id: String,
    pub cells: Vec<View>,
}

/// Data table component
#[component]
pub fn data_table(
    columns: Vec<DataTableColumn>,
    rows: Vec<DataTableRow>,
    on_row_click: Option<Callback<String>>,
) -> View {
    data_table_with_testid(columns, rows, on_row_click, None, None)
}

/// Data table component with custom testid
#[component]
pub fn data_table_with_testid(
    columns: Vec<DataTableColumn>,
    rows: Vec<DataTableRow>,
    on_row_click: Option<Callback<String>>,
    testid: Option<String>,
    row_testid_prefix: Option<String>,
) -> View {
    let table_testid = testid.unwrap_or_else(|| "data-table".to_string());
    let row_prefix = row_testid_prefix;
    // Pre-compute header views
    let header_views: Vec<View> = columns.iter().map(|col| {
        let width_style = col.width.as_ref().map(|w| format!("width: {}", w)).unwrap_or_default();
        let align_class = col.align.as_ref().map(|a| format!("text-{}", a)).unwrap_or_default();
        let label = col.label.clone();
        view! {
            <th style={width_style} class={align_class}>
                {label}
            </th>
        }
    }).collect();

    view! {
        style {
            r#"
            .data-table {
                width: 100%;
                border-collapse: collapse;
            }
            .data-table th,
            .data-table td {
                padding: 12px 16px;
                text-align: left;
                border-bottom: 1px solid var(--border);
            }
            .data-table th {
                background: var(--bg);
                font-weight: 600;
                font-size: 12px;
                text-transform: uppercase;
                letter-spacing: 0.5px;
                color: var(--text-muted);
            }
            .data-table tbody tr {
                cursor: pointer;
                transition: background 0.15s;
            }
            .data-table tbody tr:hover {
                background: var(--blue-light);
            }
            .data-table tbody tr:last-child td {
                border-bottom: none;
            }
            .data-table .id-cell {
                font-family: IBM Plex Mono, monospace;
                font-size: 12px;
                color: var(--blue);
            }
            .data-table .amount-cell {
                font-family: IBM Plex Mono, monospace;
                font-weight: 500;
            }
            .data-table .text-right {
                text-align: right;
            }
            .data-table .text-center {
                text-align: center;
            }
            "#
        }

        <table class="data-table" data-testid={table_testid}>
            <thead>
                <tr>
                    for header in header_views.iter() {
                        {header.clone()}
                    }
                </tr>
            </thead>
            <tbody>
                for row in rows.iter() {
                    {table_row_with_testid_prefix(row.clone(), on_row_click.clone(), row_prefix.as_deref())}
                }
            </tbody>
        </table>
    }
}

fn table_row_with_testid_prefix(row: DataTableRow, on_click: Option<Callback<String>>, testid_prefix: Option<&str>) -> View {
    let row_id = row.id.clone();
    let row_testid = testid_prefix.map(|p| format!("{}-{}", p, row_id)).unwrap_or_default();

    let handle_click = Callback::<()>::new({
        let on_click = on_click.clone();
        let row_id = row_id.clone();
        move |_| {
            if let Some(cb) = &on_click {
                cb.call(row_id.clone());
            }
        }
    });

    view! {
        <tr on:click={handle_click} data-row-id={row_id.clone()} data-testid={row_testid}>
            for cell in row.cells.iter() {
                <td>{cell.clone()}</td>
            }
        </tr>
    }
}
