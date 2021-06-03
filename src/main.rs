use bar_calendar::*;
use options::Options;
use std::env;
use std::error::Error;

mod options;

fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::new(env::args());
    let calendar_text;
    calendar_text = next_calendar_event(options.filepath)?;

    println!("{}", calendar_text);
    Ok(())
}
