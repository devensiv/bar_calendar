use bar_calendar::*;
use dirs;
use std::env;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let calendar_text;
    if let Some(filename) = args.next() {
        calendar_text = next_calendar_event(PathBuf::from(filename))?;
    } else {
        calendar_text =
            next_calendar_event(dirs::config_dir().unwrap().join("bar_calendar/config.conf"))?;
    }

    println!("{}", calendar_text);
    Ok(())
}
