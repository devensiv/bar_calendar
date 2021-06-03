use std::env::Args;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Options {
    pub json: bool,
    pub filepath: PathBuf,
}

impl Options {
    /// create an instance of Options from the command line arguments
    pub fn new(args: Args) -> Options {
        let mut options = Options {
            json: true,
            filepath: dirs::config_dir().unwrap().join("bar_calendar/config.conf"),
        };

        for arg in args.skip(1) {
            Options::parse_arg(&arg, &mut options);
        }

        options
    }

    /// parse a single argument altering the current options from their default state
    fn parse_arg(argument: &str, options: &mut Options) {
        match argument {
            "--json" => options.json = true,
            "--no-json" => options.json = false,
            _ => options.filepath = PathBuf::from(argument),
        }
    }
}
