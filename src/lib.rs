use chrono::{Duration, Utc};
use duration::format_duration;
use entry::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

mod duration;
mod entry;

pub fn next_calendar_event(configfile: PathBuf) -> Result<String, Box<dyn Error>> {
    let now = Utc::now();
    let reader = BufReader::new(File::open(configfile)?);
    for line in reader.lines() {
        let line = line?;

        // Ignore comments
        if line.starts_with("#") {
            continue;
        }

        // create event from that line
        let event = Entry::from_string(&line, &now)?;

        // format output
        let duration = event.date.signed_duration_since(now);
        if duration > Duration::seconds(0) {
            let state = match duration.num_minutes() {
                0..=5 => "Critical".to_string(),
                6..=15 => "Warning".to_string(),
                16..=60 => "Good".to_string(),
                61..=120 => "Info".to_string(),
                _ => "Idle".to_string(),
            };
            return Ok(format!(
                "{{\"icon\": \"calendar\", \"state\": \"{}\", \"text\": \"{} in {}\"}}",
                state,
                event.note,
                format_duration(duration),
            ));
        }
    }
    Ok("{\"icon\": \"calendar\", \"state\": \"Critical\", \"text\": \"None\"}".to_string())
}
