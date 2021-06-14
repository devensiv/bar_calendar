use chrono::{Duration, Utc};
use entry::*;
use event::Event;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

mod duration;
mod entry;
mod event;

pub fn next_calendar_event(configfile: PathBuf) -> Result<Event, Box<dyn Error>> {
    let now = Utc::now();
    let reader = BufReader::new(File::open(configfile)?);
    let mut closest = (Duration::max_value(), None);
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
        if (duration > Duration::seconds(0)) & (duration < closest.0) {
            closest = (duration, Some(event));
        }
    }

    if let Some(event) = closest.1 {
        let state = match closest.0.num_seconds() {
            0..=300 => "Critical".to_string(),
            301..=900 => "Warning".to_string(),
            901..=3600 => "Good".to_string(),
            3601..=7200 => "Info".to_string(),
            _ => "Idle".to_string(),
        };
        return Ok(Event {
            text: event.note,
            time_until: closest.0,
            state: state,
        });
    }

    Ok(Event {
        text: "No Events".to_string(),
        state: "Info".to_string(),
        time_until: Duration::seconds(0),
    })
}
