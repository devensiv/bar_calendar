use chrono::{DateTime, Datelike, Duration, Utc};
use std::error::Error;

#[derive(Debug)]
pub struct Entry {
    pub note: String,
    pub description: String,
    pub date: DateTime<Utc>,
}

impl Entry {
    pub fn from_string(data: &str, now: &DateTime<Utc>) -> Result<Entry, Box<dyn Error>> {
        type DateMod = fn(&DateTime<Utc>, &DateTime<Utc>) -> DateTime<Utc>;
        let mut parts = data.split_whitespace();
        let mut date_string = String::new();
        let mut wildcard_offset = Vec::<DateMod>::new();

        // Time zone with %:z (e.g. +02:00)
        if let Some(entry) = parts.next() {
            date_string.push_str(entry);
        }
        date_string.push(' ');

        // Year with %Y (e.g. 2001)
        if let Some(entry) = parts.next() {
            match entry {
                "*" => {
                    date_string.push_str(&format!("{}", now.format("%Y")));
                    wildcard_offset.push(|date: &DateTime<Utc>, now: &DateTime<Utc>| {
                        date.with_year(now.year() + 1).unwrap()
                    });
                }
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
                    "*" => {
                        date_string.push_str(&format!("{}", now.format("%b")));
                        wildcard_offset.push(|date: &DateTime<Utc>, now: &DateTime<Utc>| {
                            if now.month() == 12 {
                                date.with_year(now.year() + 1)
                                    .unwrap()
                                    .with_month(1)
                                    .unwrap()
                            } else {
                                date.with_month(now.month() + 1).unwrap()
                            }
                        });
                    }
                    _ => date_string.push_str(entry),
                }
            }
            date_string.push(' ');

            // Day with %d (01..=31)
            if let Some(entry) = parts.next() {
                match entry {
                    "*" => {
                        date_string.push_str(&format!("{}", now.format("%d")));
                        wildcard_offset.push(|date: &DateTime<Utc>, _: &DateTime<Utc>| {
                            let ndate = date.clone();
                            ndate.checked_add_signed(Duration::days(1)).unwrap()
                        });
                    }
                    _ => date_string.push_str(entry),
                }
            }
            date_string.push(' ');
        }
        // week mode
        else {
            // Week with %W (1..=52)
            if let Some(entry) = parts.next() {
                match entry {
                    "*" => {
                        date_string.push_str(&format!("{}", now.format("%W")));
                        wildcard_offset.push(|date: &DateTime<Utc>, _: &DateTime<Utc>| {
                            let ndate = date.clone();
                            ndate.checked_add_signed(Duration::weeks(1)).unwrap()
                        });
                    }
                    _ => date_string.push_str(entry),
                }
            }
            date_string.push(' ');

            // Day again but with %a (e.g. Sun/Mon)
            if let Some(entry) = parts.next() {
                match entry {
                    "*" => {
                        date_string.push_str(&format!("{}", now.format("%a")));
                        wildcard_offset.push(|date: &DateTime<Utc>, _: &DateTime<Utc>| {
                            let ndate = date.clone();
                            ndate.checked_add_signed(Duration::days(1)).unwrap()
                        });
                    }
                    _ => date_string.push_str(entry),
                }
            }
            date_string.push(' ');
        }

        // Time with %R (e.g. 16:35)
        if let Some(entry) = parts.next() {
            let mut time = entry.split(":");
            let hours = if let Some(hours) = time.next() {
                match hours {
                    "*" => {
                        wildcard_offset.push(|date: &DateTime<Utc>, _: &DateTime<Utc>| {
                            let ndate = date.clone();
                            ndate.checked_add_signed(Duration::hours(1)).unwrap()
                        });
                        format!("{}", now.format("%H"))
                    }
                    _ => hours.to_string(),
                }
            } else {
                Err("Error parsing the hours")?
            };

            let minutes = if let Some(minutes) = time.next() {
                match minutes {
                    "*" => {
                        wildcard_offset.push(|date: &DateTime<Utc>, _: &DateTime<Utc>| {
                            let ndate = date.clone();
                            ndate.checked_add_signed(Duration::minutes(1)).unwrap()
                        });
                        format!("{}", now.format("%M"))
                    }
                    _ => minutes.to_string(),
                }
            } else {
                Err("Error parsing the minutes")?
            };
        
            date_string.push_str(&format!("{}:{}", hours, minutes));
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
