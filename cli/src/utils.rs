pub fn format_bytes(bytes: u64) -> String {
    let (unit, size) = if bytes >> 40 > 0 {
        ("TB", bytes as f64 / (1u64 << 40) as f64)
    } else if bytes >> 30 > 0 {
        ("GB", bytes as f64 / (1u64 << 30) as f64)
    } else if bytes >> 20 > 0 {
        ("MB", bytes as f64 / (1u64 << 20) as f64)
    } else if bytes >> 10 > 0 {
        ("KB", bytes as f64 / (1u64 << 10) as f64)
    } else {
        ("B", bytes as f64)
    };

    format!("{:.2}{}", size, unit)
}

pub fn format_millis(millis: u128) -> String {
    let seconds = millis / 1000;
    let minutes = seconds / 60;

    let hours = minutes / 60;
    let minutes = minutes % 60;
    let seconds = seconds % 60;
    let ms = millis % 1000;

    let mut s = String::new();
    if hours > 0 {
        s.push_str(&format!("{}h ", hours));
    }
    if minutes > 0 {
        s.push_str(&format!("{}m ", minutes));
    }
    if seconds > 0 {
        s.push_str(&format!("{}s ", seconds));
    }
    if ms > 0 {
        s.push_str(&format!("{}ms ", ms));
    }

    if s.ends_with(" ") {
        s.pop();
    }

    s
}
