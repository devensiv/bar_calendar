use chrono::{DateTime, Utc};
use std::error::Error;

pub struct Entry {
    pub note: String,
    pub description: String,
    pub date: DateTime<Utc>,
}

impl Entry {
    pub fn from_string(data: &str, now: &DateTime<Utc>) -> Result<Entry, Box<dyn Error>> {
        let mut parts = data.split_whitespace();
        let mut date_string = String::new();

        // Time zone with %:z (e.g. +02:00)
        if let Some(entry) = parts.next() {
            match entry {
                "*" => date_string.push_str(&format!("{}", now.format("%z"))),
                _ => date_string.push_str(entry),
            }
        }
        date_string.push(' ');

        // Year with %Y (e.g. 2001)
        if let Some(entry) = parts.next() {
            match entry {
                "*" => date_string.push_str(&format!("{}", now.format("%Y"))),
                _ => date_string.push_str(entry),
            }
        }
        date_string.push(' ');

        // parse mode (month mode or week mode)
        let weekformat = match parts.next() {
            Some(mode) => match mode {
                "m" => false,
                "w" => true,
                _ => Err(format!("{} is invalid as mode indicator", mode))?,
            },
            None => Err("No mode indicator")?,
        };

        // month mode
        if !weekformat {
            // Month with %m (01..=12)
            if let Some(entry) = parts.next() {
                match entry {
                    "*" => date_string.push_str(&format!("{}", now.format("%b"))),
                    _ => date_string.push_str(entry),
                }
            }
            date_string.push(' ');

            // Day with %d (01..=31)
            if let Some(entry) = parts.next() {
                match entry {
                    "*" => date_string.push_str(&format!("{}", now.format("%d"))),
                    _ => date_string.push_str(entry),
                }
            }
            date_string.push(' ');
        }
        // week mode
        else {
            // Day again but with %a (e.g. Sun/Mon)
            if let Some(entry) = parts.next() {
                match entry {
                    "*" => date_string.push_str(&format!("{}", now.format("%a"))),
                    _ => date_string.push_str(entry),
                }
            }
            date_string.push(' ');

            // Week with %W (1..=52)
            if let Some(entry) = parts.next() {
                match entry {
                    "*" => date_string.push_str(&format!("{}", now.format("%W"))),
                    _ => date_string.push_str(entry),
                }
            }
            date_string.push(' ');
        }

        // Time with %R (e.g. 16:35)
        if let Some(entry) = parts.next() {
            match entry {
                "*" => date_string.push_str(&format!("{}", now.format("%R"))),
                _ => date_string.push_str(entry),
            }
        }

        let date = if weekformat {
            DateTime::parse_from_str(&date_string, "%z %Y %a %W %R")
        } else {
            DateTime::parse_from_str(&date_string, "%z %Y %b %d %R")
        }?
        .with_timezone(&Utc);

        Ok(Entry {
            note: parts.next().unwrap_or("No Note").to_string(),
            description: parts.next().unwrap_or("No description").to_string(),
            date,
        })
    }
}
