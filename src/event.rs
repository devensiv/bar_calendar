use crate::duration::format_duration;
use chrono::Duration;

#[derive(Debug)]
pub struct Event {
    pub text: String,
    pub state: String,
    pub time_until: Duration,
}

impl Event {
    pub fn format(&self, json: bool) -> String {
        if json {
            format!(
                "{{\"icon\": \"calendar\", \"state\": \"{}\", \"text\": \"{} in {}\"}}",
                self.state,
                self.text,
                format_duration(self.time_until),
            )
        } else {
            format!("{} in {}", self.text, format_duration(self.time_until))
        }
    }
}
