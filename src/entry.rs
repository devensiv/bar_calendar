use chrono::{DateTime, Datelike, Duration, Utc};
use std::error::Error;

type DateMod = fn(&DateTime<Utc>, &DateTime<Utc>) -> DateTime<Utc>;

#[derive(Clone)]
pub struct Entry {
    pub note: String,
    pub date: DateTime<Utc>,
    pub duration: Duration,
    mod_stack: Vec<DateMod>,
}

impl Entry {
    /// Tries to create a version of this Entry with a date laying in the future of the given now value using the available wildcards
    /// returns true if modifying the internal state to future worked
    pub fn resolve_wildcards(&mut self, now: DateTime<Utc>, duration: &mut Duration) -> bool {
        while !self.mod_stack.is_empty() {
            let time_mod = self.mod_stack.pop().unwrap();
            let new_date = time_mod(&self.date, &now);

            *duration = new_date.signed_duration_since(now);
            if duration > &mut Duration::seconds(0) {
                self.date = new_date;
                return true;
            }
        }
        false
    }

    pub fn from_string(data: &str, now: &DateTime<Utc>) -> Result<Entry, Box<dyn Error>> {
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

        // parse mode (month_mode or week_mode)
        let weekformat = match parts.next() {
            Some(mode) => match mode {
                "m" => false,
                "w" => true,
                _ => Err(format!("{} is invalid as mode indicator", mode))?,
            },
            None => Err("No mode indicator")?,
        };

        // month_mode
        if !weekformat {
            // Month with %b (Jun..=Dec)
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
                            let ndate = *date;
                            ndate.checked_add_signed(Duration::days(1)).unwrap()
                        });
                    }
                    _ => date_string.push_str(entry),
                }
            }
            date_string.push(' ');
        }
        // week_mode
        else {
            // Week with %W (1..=52)
            if let Some(entry) = parts.next() {
                match entry {
                    "*" => {
                        date_string.push_str(&format!("{}", now.format("%W")));
                        wildcard_offset.push(|date: &DateTime<Utc>, _: &DateTime<Utc>| {
                            let ndate = *date;
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
                            let ndate = *date;
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
            let mut time = entry.split(':');
            let hours = if let Some(hours) = time.next() {
                match hours {
                    "*" => {
                        wildcard_offset.push(|date: &DateTime<Utc>, _: &DateTime<Utc>| {
                            let ndate = *date;
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
                            let ndate = *date;
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
            DateTime::parse_from_str(&date_string, "%:z %Y %W %a %R")
        } else {
            DateTime::parse_from_str(&date_string, "%:z %Y %b %d %R")
        }
        .map_err(|_| {
            format!(
                "Error parsing {} in {}",
                date_string,
                if weekformat { "weekmode" } else { "monthmode" }
            )
        })?
        .with_timezone(&Utc);

        let note = parts.next();

        let duration = match parts.next() {
            Some(pattern) => {
                let mut chars = pattern.chars();
                let indicator = chars.next();
                let num = chars.collect::<String>().parse::<i64>()?;
                match indicator {
                    Some('s') => Duration::seconds(num),
                    Some('m') => Duration::minutes(num),
                    Some('h') => Duration::hours(num),
                    Some('d') => Duration::days(num),
                    _ => Err("Invalid duration indicator")?,
                }
            }
            None => Duration::seconds(0),
        };

        Ok(Entry {
            note: note.unwrap_or("No Note").to_string(),
            date,
            mod_stack: wildcard_offset,
            duration,
        })
    }
}
