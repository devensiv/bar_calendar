use chrono::Duration;

pub fn format_duration(duration: Duration) -> String {
    let sec = duration.num_seconds();
    let min = sec / 60i64;
    match min {
        0..=120 => format!("{} min", min),
        121..=2880 => format!("{} h", min / 60),
        _ => format!("{} days", min / 60 / 24),
    }
}
