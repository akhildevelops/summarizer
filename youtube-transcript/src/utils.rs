use std::time::Duration;

pub(crate) fn to_human_readable(duration: &Duration) -> String {
    let dur = duration.as_nanos();
    match dur {
        0u128..=999 => format!("{}ns", dur),
        1000u128..=999_999 => format!("{}μs", dur / 1000),
        1000_000u128..=999_999_999 => format!("{}ms", dur / 1000_000),
        1000_000_000u128..=59_999_999_999 => format!("{}s", dur / 1000_000_000),
        60_000_000_000u128..=3599_999_999_999 => {
            format!(
                "{}m {}s",
                dur as f64 / 60_000_000_000.0,
                dur % 60_000_000_000
            )
        }
        _ => format!(
            "{}h {}m",
            dur / 3600_000_000_000,
            (dur % 3600_000_000_000) / 60_000_000_000,
        ),
    }
}
