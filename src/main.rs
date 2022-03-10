use bar_calendar::*;
use options::Options;
use std::env;
use std::error::Error;

mod options;
pub static VERSION: &str = "0.1";

fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::new(env::args());

    if options.version {
        println!("bar_calendar {}", VERSION);
        println!("Source Code: https://github.com/devensiv/bar_calendar");
        return Ok(());
    }

    let calendar_text = next_calendar_event(options.filepath)?;

    println!("{}", calendar_text.format(options.json));
    Ok(())
}
