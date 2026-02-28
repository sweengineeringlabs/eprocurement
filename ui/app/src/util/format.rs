//! Formatting utilities

/// Format currency in South African Rand
pub fn format_currency(amount: f64) -> String {
    let abs = amount.abs();
    let sign = if amount < 0.0 { "-" } else { "" };

    if abs >= 1_000_000_000.0 {
        format!("{}R {:.1}B", sign, abs / 1_000_000_000.0)
    } else if abs >= 1_000_000.0 {
        format!("{}R {:.1}M", sign, abs / 1_000_000.0)
    } else if abs >= 1_000.0 {
        format!("{}R {}", sign, format_number(abs as i64))
    } else {
        format!("{}R {:.2}", sign, abs)
    }
}

/// Format currency with full precision
pub fn format_currency_full(amount: f64) -> String {
    let formatted = format!("{:.2}", amount.abs());
    let parts: Vec<&str> = formatted.split('.').collect();
    let integer_part = parts[0];
    let decimal_part = parts.get(1).unwrap_or(&"00");

    // Add thousand separators
    let with_separators: String = integer_part
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            if i > 0 && i % 3 == 0 {
                format!(",{}", c)
            } else {
                c.to_string()
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    let sign = if amount < 0.0 { "-" } else { "" };
    format!("{}R {}.{}", sign, with_separators, decimal_part)
}

/// Format percentage
pub fn format_percentage(value: f64, decimals: usize) -> String {
    format!("{:.precision$}%", value, precision = decimals)
}

/// Format date (ISO to display)
pub fn format_date(iso_date: &str) -> String {
    // Simple conversion from YYYY-MM-DD to DD MMM YYYY
    if iso_date.len() >= 10 {
        let parts: Vec<&str> = iso_date[..10].split('-').collect();
        if parts.len() == 3 {
            let month = match parts[1] {
                "01" => "Jan",
                "02" => "Feb",
                "03" => "Mar",
                "04" => "Apr",
                "05" => "May",
                "06" => "Jun",
                "07" => "Jul",
                "08" => "Aug",
                "09" => "Sep",
                "10" => "Oct",
                "11" => "Nov",
                "12" => "Dec",
                _ => parts[1],
            };
            return format!("{} {} {}", parts[2], month, parts[0]);
        }
    }
    iso_date.to_string()
}

/// Format date and time
pub fn format_datetime(iso_datetime: &str) -> String {
    if iso_datetime.len() >= 16 {
        let date = format_date(&iso_datetime[..10]);
        let time = &iso_datetime[11..16];
        format!("{} {}", date, time)
    } else {
        format_date(iso_datetime)
    }
}

/// Format relative time
pub fn format_relative_time(iso_datetime: &str) -> String {
    // Simplified - in production would use actual date comparison
    if iso_datetime.contains("T") {
        "Today".to_string()
    } else {
        format_date(iso_datetime)
    }
}

/// Format number with thousands separators
pub fn format_number(value: impl std::fmt::Display) -> String {
    let s = value.to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    for (i, c) in chars.iter().enumerate() {
        result.push(*c);
        let remaining = len - i - 1;
        if remaining > 0 && remaining % 3 == 0 && *c != '-' {
            result.push(',');
        }
    }
    result
}

/// Format file size
pub fn format_file_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

/// Truncate text with ellipsis
pub fn truncate(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len - 3])
    }
}
